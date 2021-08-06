/*
 * @file
 * Application layered TCP/TLS connection API (to be used from TCPIP thread)
 *
 * This file provides a TLS layer using mbedTLS
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 * Watch out:
 * - 'sent' is always called with len==0 to the upper layer. This is because keeping
 *   track of the ratio of application data and TLS overhead would be too much.
 *
 * Mandatory security-related configuration:
 * - define ALTCP_MBEDTLS_RNG_FN to mbedtls_entropy_func to use the standard mbedTLS
 *   entropy and ensure to add at least one strong entropy source to your mbedtls port
 *   (implement mbedtls_platform_entropy_poll or mbedtls_hardware_poll providing strong
 *   entropy)
 * - define ALTCP_MBEDTLS_ENTROPY_PTR and ALTCP_MBEDTLS_ENTROPY_LEN to something providing
 *   GOOD custom entropy
 *
 * Missing things / @todo:
 * - some unhandled/untested things migh be caught by LWIP_ASSERTs...
 */

/* @todo: which includes are really needed? */

// pub const ALTCP_MBEDTLS_ENTROPY_PTR:    NULL

use std::io::stdout;

use crate::altcp_tls::altcp_tls_mbedtls_mem::{
    altcp_mbedtls_alloc, altcp_mbedtls_alloc_config, altcp_mbedtls_free, altcp_mbedtls_free_config,
    altcp_mbedtls_mem_init,
};
use crate::altcp_tls::altcp_tls_mbedtls_structs::{
    altcp_mbedtls_state, mbedtls_ssl_context, ALTCP_MBEDTLS_FLAGS_APPLDATA_SENT,
    ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE, ALTCP_MBEDTLS_FLAGS_RX_CLOSED,
    ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED, ALTCP_MBEDTLS_FLAGS_UPPER_CALLED,
};
use crate::core::altcp::{
    altcp_abort, altcp_accept, altcp_alloc, altcp_arg, altcp_close, altcp_connect,
    altcp_default_sndbuf, altcp_err, altcp_free, altcp_listen_with_backlog_and_err, altcp_output,
    altcp_poll, altcp_recv, altcp_recved, altcp_sent, altcp_sndbuf,
};
use crate::core::altcp_h::altcp_pcb;
use crate::core::altcp_tls_mbedtls_opts_h::ALTCP_MBEDTLS_DEBUG;
use crate::core::debug_h::LWIP_DBG_LEVEL_SERIOUS;
use crate::core::def_h::NULL;
use crate::core::err_h::{
    LwipError, ERR_ABRT, ERR_ARG, ERR_CLSD, ERR_MEM, ERR_OK, ERR_STATE, ERR_VAL,
};
use crate::core::pbuf::{pbuf_alloc, pbuf_cat, pbuf_copy_partial, pbuf_realloc};
use crate::core::pbuf_h::{PacketBuffer, PBUF_POOL, PBUF_RAW};

pub const ALTCP_MBEDTLS_ENTROPY_LEN: u32 = 0;

/* Variable prototype, the actual declaration is at the end of this file
since it contains pointers to static functions declared here */
// extern const struct altcp_functions altcp_mbedtls_functions;

/* Our global mbedTLS configuration (server-specific, not connection-specific) */
pub struct altcp_tls_config {
    pub conf: mbedtls_ssl_config,
    pub entropy: mbedtls_entropy_context,
    pub ctr_drbg: mbedtls_ctr_drbg_context,
    pub cert: mbedtls_x509_crt,
    pub pkey: mbedtls_pk_context,
    pub ca: mbedtls_x509_crt,

    /* Inter-connection cache for fast connection startup */
    pub cache: mbedtls_ssl_cache_context,
}

// static err_t altcp_mbedtls_lower_recv(arg: &mut Vec<u8>, inner_conn: &mut altcp_pcb, p: &mut pbuf, err: err_t);
// static err_t altcp_mbedtls_setup(void *conf, conn: &mut altcp_pcb, inner_conn: &mut altcp_pcb);
// static err_t altcp_mbedtls_lower_recv_process(conn: &mut altcp_pcb, altcp_mbedtls_state *state);
// static err_t altcp_mbedtls_handle_rx_appldata(conn: &mut altcp_pcb, altcp_mbedtls_state *state);
// static altcp_mbedtls_bio_send: int(void *ctx,  unsigned char *dataptr, size: usize);

/* callback functions from inner/lower connection: */

/* Accept callback from lower connection (i.e. TCP)
 * Allocate one of our structures, assign it to the new connection's 'state' and
 * call the new connection's 'accepted' callback. If that succeeds, we wait
 * to receive connection setup handshake bytes from the client.
 */
pub fn altcp_mbedtls_lower_accept(
    arg: &mut altcp_pcb,
    accepted_conn: &mut altcp_pcb,
    err: err_t,
) -> Result<(), LwipError> {
    let listen_conn: &mut altcp_pcb = arg;
    if listen_conn.state.len() > 0 && listen_conn.accept.is_some() {
        let mut setup_err: err_t;
        let listen_state = &mut listen_conn.state;
        /* create a new altcp_conn to pass to the next 'accept' callback */
        let mut new_conn = altcp_alloc();
        // if new_conn == NULL {
        //     return ERR_MEM;
        // }
        setup_err = altcp_mbedtls_setup(&mut listen_state.conf, &mut new_conn, accepted_conn);
        if setup_err != ERR_OK {
            altcp_free(&mut new_conn);
            return setup_err;
        }
        return listen_conn.accept.unwrap()(&mut listen_conn.arg.unwrap(), &mut new_conn, err);
    }
    return Err(LwipError::new(
        ERR_ARG,
        &"invalid state and accept pointers in arg (altcp_pcb)".to_string(),
    ));
}

/* Connected callback from lower connection (i.e. TCP).
 * Not really implemented/tested yet...
 */
pub fn altcp_mbedtls_lower_connected(
    arg: &mut altcp_pcb,
    inner_conn: &mut altcp_pcb,
    err: err_t,
) -> Result<(), LwipError> {
    // LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    if conn.inner_conn != inner_conn {
        Err("pcb mismatch")
    }
    /* upper connected is called when handshake is done */
    if err != ERR_OK {
        if conn.connected {
            return conn.connected(conn.arg, conn, err);
        }
    }
    return altcp_mbedtls_lower_recv_process(conn, conn.state);
}

/* Call recved for possibly more than an u16 */
pub fn altcp_mbedtls_lower_recved(inner_conn: &mut altcp_pcb, recvd_cnt: int) {
    while recvd_cnt > 0 {
        let mut recvd_part: u16 = LWIP_MIN(recvd_cnt, 0xFFFF);
        altcp_recved(inner_conn, recvd_part);
        recvd_cnt -= recvd_part;
    }
}

/* Recv callback from lower connection (i.e. TCP)
 * This one mainly differs between connection setup/handshake (data is fed into mbedTLS only)
 * and application phase (data is decoded by mbedTLS and passed on to the application).
 */
pub fn altcp_mbedtls_lower_recv(
    arg: &mut altcp_pcb,
    inner_conn: &mut altcp_pcb,
    p: &mut pbuf,
    err: err_t,
) -> Result<(), LwipError> {
    altcp_mbedtls_state * state;
    let mut conn: &mut altcp_pcb = arg;

    LWIP_ASSERT("no err expected", err == ERR_OK);
    LWIP_UNUSED_ARG(err);

    if !conn {
        /* no connection given as arg? should not happen, but prevent pbuf/conn leaks */
        if p != NULL {
            pbuf_free(p);
        }
        altcp_close(inner_conn);
        return Err(LwipError::new(ERR_CLSD, "connection closed"));
    }
    state = conn.state;
    LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    if !state {
        /* already closed */
        if p != NULL {
            pbuf_free(p);
        }
        altcp_close(inner_conn);
        return Err(LwipError::new(ERR_CLSD, "connection closed"));
    }

    /* handle NULL pbuf (inner connection closed) */
    if p == NULL {
        /* remote host sent FIN, remember this (SSL state is destroyed
        when both sides are closed only!) */
        if (state.flags & (ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE | ALTCP_MBEDTLS_FLAGS_UPPER_CALLED))
            == (ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE | ALTCP_MBEDTLS_FLAGS_UPPER_CALLED)
        {
            /* need to notify upper layer (e.g. 'accept' called or 'connect' succeeded) */
            if (state.rx != NULL) || (state.rx_app != NULL) {
                state.flags |= ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED;
                /* this is a normal close (FIN) but we have unprocessed data, so delay the FIN */
                altcp_mbedtls_handle_rx_appldata(conn, state);
                return Ok(());
            }
            state.flags |= ALTCP_MBEDTLS_FLAGS_RX_CLOSED;
            if conn.recv {
                return conn.recv(conn.arg, conn, NULL, ERR_OK);
            }
        } else {
            /* before connection setup is done: call 'err' */
            if conn.err {
                conn.err(conn.arg, ERR_CLSD);
            }
            altcp_close(conn);
        }
        return Ok(());
    }

    /* If we come here, the connection is in good state (handshake phase or application data phase).
    Queue up the pbuf for processing as handshake data or application data. */
    if state.rx == NULL {
        state.rx = p;
    } else {
        LWIP_ASSERT("rx pbuf overflow", p.tot_len + p.len <= 0xFFFF);
        pbuf_cat(state.rx, p);
    }
    return altcp_mbedtls_lower_recv_process(conn, state);
}

pub fn altcp_mbedtls_lower_recv_process(
    conn: &mut altcp_pcb,
    state: &mut altcp_mbedtls_state,
) -> Result<(), LwipError> {
    if !(state.flags & ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE) {
        /* handle connection setup (handshake not done) */
        let mut ret: i32 = mbedtls_ssl_handshake(&state.ssl_context);
        /* try to send data... */
        altcp_output(conn.inner_conn);
        if state.bio_bytes_read {
            /* acknowledge all bytes read */
            altcp_mbedtls_lower_recved(conn.inner_conn, state.bio_bytes_read);
            state.bio_bytes_read = 0;
        }

        if ret == MBEDTLS_ERR_SSL_WANT_READ || ret == MBEDTLS_ERR_SSL_WANT_WRITE {
            /* handshake not done, wait for more recv calls */
            // LWIP_ASSERT("in this state, the rx chain should be empty", state.rx == NULL);
            if state.rx != NULL {
                return Err(LwipError::new(
                    ERR_STATE,
                    "in this state, the rx chain should be empty",
                ));
            }
            return Ok(());
        }
        if ret != 0 {
            // LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("mbedtls_ssl_handshake failed: %d\n", ret));
            /* handshake failed, connection has to be closed */

            if conn.err.is_some() {
                conn.err.unwrap()(&mut conn.arg.unwrap(), ERR_CLSD);
            }

            if altcp_close(conn) != ERR_OK {
                altcp_abort(conn);
            }
            return Ok(());
        }
        /* If we come here, handshake succeeded. */
        LWIP_ASSERT("state", state.bio_bytes_read == 0);
        LWIP_ASSERT("state", state.bio_bytes_appl == 0);
        state.flags |= ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE;
        /* issue "connect" callback" to upper connection (this can only happen for active open) */
        
        if conn.connected {
            let err: err_t;
            err = conn.connected(&mut conn.arg, conn, ERR_OK);
            if err != ERR_OK {
                return err;
            }
        }
        if state.rx == NULL {
            return Ok(());
        }
    }
    /* handle application data */
    return altcp_mbedtls_handle_rx_appldata(conn, state);
}

/* Pass queued decoded rx data to application */
pub fn altcp_mbedtls_pass_rx_data(
    conn: &mut altcp_pcb,
    state: &mut altcp_mbedtls_state,
) -> Result<(), LwipError> {
    let err: err_t;
    let mut buf: &mut PacketBuffer;
    LWIP_ASSERT("conn != NULL", conn != NULL);
    LWIP_ASSERT("state != NULL", state != NULL);
    buf = &mut state.rx_app;
    if state.rx_app.is_some() {
        state.rx_app = None;
        if conn.recv {
            let mut tot_len: u16 = buf.tot_len;
            /* this needs to be increased first because the 'recved' call may come nested */
            state.rx_passed_unrecved += tot_len;
            state.flags |= ALTCP_MBEDTLS_FLAGS_UPPER_CALLED;
            err = conn.recv(&mut conn.arg, conn, buf, ERR_OK);
            if err != ERR_OK {
                if err == ERR_ABRT {
                    return Err(LwipError::new(ERR_ABRT, "abort"));
                }
                /* not received, leave the pbuf(s) queued (and decrease 'unrecved' again) */
                LWIP_ASSERT("state == conn.state", state == conn.state);
                state.rx_app = buf;
                state.rx_passed_unrecved -= tot_len;
                LWIP_ASSERT(
                    "state.rx_passed_unrecved >= 0",
                    state.rx_passed_unrecved >= 0,
                );
                if state.rx_passed_unrecved < 0 {
                    state.rx_passed_unrecved = 0;
                }
                return err;
            }
        } else {
            pbuf_free(buf);
        }
    } else if (state.flags & (ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED | ALTCP_MBEDTLS_FLAGS_RX_CLOSED))
        == ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED
    {
        state.flags |= ALTCP_MBEDTLS_FLAGS_RX_CLOSED;
        if conn.recv {
            return conn.recv(&mut conn.arg, conn, NULL, ERR_OK);
        }
    }

    /* application may have close the connection */
    if conn.state != state {
        /* return error code to ensure altcp_mbedtls_handle_rx_appldata() exits the loop */
        return Err(LwipError(ERR_CLSD, "connection closed"));
    }
    return Ok(());
}

/* Helper function that processes rx application data stored in rx pbuf chain */
pub fn altcp_mbedtls_handle_rx_appldata(
    conn: &mut altcp_pcb,
    state: &mut altcp_mbedtls_state,
) -> Result<(), LwipError> {
    let mut ret: i32;
    LWIP_ASSERT("state != NULL", state != NULL);
    if !(state.flags & ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE) {
        /* handshake not done yet */
        return Err(LwipError::new(ERR_VAL, "handshake not done yet"));
    }
    loop {
        /* allocate a full-sized unchained PBUF_POOL: this is for RX! */
        let buf: &mut pbuf = pbuf_alloc(PBUF_RAW, PBUF_POOL_BUFSIZE, PBUF_POOL);
        if buf == NULL {
            /* We're short on pbufs, try again later from 'poll' or 'recv' callbacks.
            @todo: close on excessive allocation failures or leave this up to upper conn? */
            return Ok(());
        }

        /* decrypt application data, this pulls encrypted RX data off state.rx pbuf chain */
        ret = mbedtls_ssl_read(&state.ssl_context, buf.payload, PBUF_POOL_BUFSIZE);
        if ret < 0 {
            if ret == MBEDTLS_ERR_SSL_CLIENT_RECONNECT {
                /* client is initiating a new connection using the same source port -> close connection or make handshake */
                LWIP_DEBUGF(
                    ALTCP_MBEDTLS_DEBUG,
                    ("new connection on same source port\n"),
                );
                LWIP_ASSERT(
                    "TODO: new connection on same source port, close this connection",
                    0,
                );
            } else if (ret != MBEDTLS_ERR_SSL_WANT_READ) && (ret != MBEDTLS_ERR_SSL_WANT_WRITE) {
                if ret == MBEDTLS_ERR_SSL_PEER_CLOSE_NOTIFY {
                    LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("connection was closed gracefully\n"));
                } else if ret == MBEDTLS_ERR_NET_CONN_RESET {
                    LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("connection was reset by peer\n"));
                }
                pbuf_free(buf);
                return Ok(());
            } else {
                pbuf_free(buf);
                return Ok(());
            }
            pbuf_free(buf);
            altcp_abort(conn);
            return Err(LwipError::new(ERR_ABRT, "connection aborted"));
        } else {
            let err: err_t;
            if ret {
                LWIP_ASSERT("bogus receive length", ret <= PBUF_POOL_BUFSIZE);
                /* trim pool pbuf to actually decoded length */
                pbuf_realloc(buf, ret);

                state.bio_bytes_appl += ret;
                if (mbedtls_ssl_get_bytes_avail(&state.ssl_context) == 0) {
                    /* Record is done, now we know the share between application and protocol bytes
                    and can adjust the RX window by the protocol bytes.
                    The rest is 'recved' by the application calling our 'recved' fn. */
                    let overhead_bytes: int;
                    LWIP_ASSERT(
                        "bogus byte counts",
                        state.bio_bytes_read > state.bio_bytes_appl,
                    );
                    overhead_bytes = state.bio_bytes_read - state.bio_bytes_appl;
                    altcp_mbedtls_lower_recved(conn.inner_conn, overhead_bytes);
                    state.bio_bytes_read = 0;
                    state.bio_bytes_appl = 0;
                }

                if state.rx_app == NULL {
                    state.rx_app = buf;
                } else {
                    pbuf_cat(&mut state.rx_app.unwrap(), buf);
                }
            } else {
                pbuf_free(buf);
                buf = NULL;
            }
            err = altcp_mbedtls_pass_rx_data(conn, state);
            if err != ERR_OK {
                if err == ERR_ABRT {
                    /* recv callback needs to return this as the pcb is deallocated */
                    return Err(LwipError(ERR_ABRT, "abort"));
                }
                /* we hide all other errors as we retry feeding the pbuf to the app later */
                return Ok(());
            }
        }
        if !(ret > 0) {
            break;
        }
    }
    return Ok(());
}

/* Receive callback function called from mbedtls (set via mbedtls_ssl_set_bio)
 * This function mainly copies data from pbufs and frees the pbufs after copying.
 */
pub fn altcp_mbedtls_bio_recv(ctx: &mut altcp_pcb, buf: &mut Vec<u8>, len: usize) -> i32 {
    let conn: &mut altcp_pcb = ctx;
    altcp_mbedtls_state * state;
    let p: &mut pbuf;
    let ret: u16;
    let copy_len: u16;
    let err: err_t;

    if (conn == NULL) || (conn.state == NULL) {
        return MBEDTLS_ERR_NET_INVALID_CONTEXT;
    }
    state = conn.state;
    p = state.rx;

    /* @todo: return MBEDTLS_ERR_NET_CONN_RESET/MBEDTLS_ERR_NET_RECV_FAILED? */

    if ((p == NULL) || ((p.len == 0) && (p.next == NULL))) {
        if (p) {
            pbuf_free(p);
        }
        state.rx = NULL;
        if ((state.flags & (ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED | ALTCP_MBEDTLS_FLAGS_RX_CLOSED))
            == ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED)
        {
            /* close queued but not passed up yet */
            return 0;
        }
        return MBEDTLS_ERR_SSL_WANT_READ;
    }
    /* limit number of bytes again to copy from first pbuf in a chain only */
    copy_len = LWIP_MIN(len, p.len);
    /* copy the data */
    ret = pbuf_copy_partial(p, buf, copy_len, 0);
    LWIP_ASSERT("ret == copy_len", ret == copy_len);
    /* hide the copied bytes from the pbuf */
    err = pbuf_remove_header(p, ret);
    LWIP_ASSERT("error", err == ERR_OK);
    if p.len == 0 {
        /* the first pbuf has been fully read, free it */
        state.rx = p.next;
        p.next = NULL;
        pbuf_free(p);
    }

    state.bio_bytes_read += ret;
    return ret;
}

/* Sent callback from lower connection (i.e. TCP)
 * This only informs the upper layer to try to send more, not about
 * the number of ACKed bytes.
 */
pub fn altcp_mbedtls_lower_sent(
    arg: &mut altcp_pcb,
    inner_conn: &mut altcp_pcb,
    len: u16,
) -> Result<(), &str> {
    let conn: &mut altcp_pcb = arg;
    LWIP_UNUSED_ARG(inner_conn); /* for LWIP_NOASSERT */
    LWIP_UNUSED_ARG(len);
    if (conn) {
        altcp_mbedtls_state * state = conn.state;
        LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
        if (!state || !(state.flags & ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE)) {
            /* @todo: do something here? */
            return Ok(());
        }
        /* try to send more if we failed before */
        mbedtls_ssl_flush_output(&state.ssl_context);
        /* call upper sent with len==0 if the application already sent data */
        if ((state.flags & ALTCP_MBEDTLS_FLAGS_APPLDATA_SENT) && conn.sent) {
            return conn.sent(conn.arg, conn, 0);
        }
    }
    return Ok(());
}

/* Poll callback from lower connection (i.e. TCP)
 * Just pass this on to the application.
 * @todo: retry sending?
 */
pub fn altcp_mbedtls_lower_poll(
    arg: &mut altcp_pcb,
    inner_conn: &mut altcp_pcb,
) -> Result<(), LwipError> {
    let conn: &mut altcp_pcb = arg;
    LWIP_UNUSED_ARG(inner_conn); /* for LWIP_NOASSERT */
    if (conn) {
        LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
        /* check if there's unreceived rx data */
        if (conn.state) {
            altcp_mbedtls_state * state = conn.state;
            /* try to send more if we failed before */
            mbedtls_ssl_flush_output(&state.ssl_context);
            if altcp_mbedtls_handle_rx_appldata(conn, state) == ERR_ABRT {
                return Err(LwipError::new(ERR_ABRT, "abort"));
            }
        }
        if conn.poll {
            return conn.poll(conn.arg, conn);
        }
    }
    return Ok(());
}

pub fn altcp_mbedtls_lower_err(arg: &mut altcp_pcb, err: err_t) {
    let conn: &mut altcp_pcb = arg;
    if conn {
        conn.inner_conn = NULL; /* already freed */
        if conn.err {
            conn.err(conn.arg, err);
        }
        altcp_free(conn);
    }
}

/* setup functions */

pub fn altcp_mbedtls_remove_callbacks(inner_conn: &mut altcp_pcb) {
    altcp_arg(inner_conn, None);
    altcp_recv(inner_conn, None);
    altcp_sent(inner_conn, None);
    altcp_err(inner_conn, None);
    altcp_poll(inner_conn, None, inner_conn.pollinterval);
}

pub fn altcp_mbedtls_setup_callbacks(conn: &mut altcp_pcb, inner_conn: &mut altcp_pcb) {
    altcp_arg(inner_conn, Some(conn));
    altcp_recv(inner_conn, Some(altcp_mbedtls_lower_recv));
    altcp_sent(inner_conn, Some(altcp_mbedtls_lower_sent));
    altcp_err(inner_conn, Some(altcp_mbedtls_lower_err));
    /* tcp_poll is set when interval is set by application */
    /* listen is set totally different :-) */
}

pub fn altcp_mbedtls_setup(
    conf: &mut Vec<u8>,
    conn: &mut altcp_pcb,
    inner_conn: &mut altcp_pcb,
) -> Result<(), LwipError> {
    let ret: int;
    let config: &mut altcp_tls_config = conf;
    altcp_mbedtls_state * state;
    if !conf {
        return Err(LwipError::new(ERR_ARG, "invalid argument"));
    }
    LWIP_ASSERT("invalid inner_conn", conn != inner_conn);

    /* allocate mbedtls context */
    state = altcp_mbedtls_alloc(conf);
    if state == NULL {
        return Err(LwipError::new(ERR_MEM, "out of memory"));
    }
    /* initialize mbedtls context: */
    mbedtls_ssl_init(&state.ssl_context);
    ret = mbedtls_ssl_setup(&state.ssl_context, &config.conf);
    if ret != 0 {
        LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("mbedtls_ssl_setup failed\n"));
        /* @todo: convert 'ret' to err_t */
        altcp_mbedtls_free(conf, state);
        return Err(LwipError::new(ERR_MEM, "out of memory"));
    }
    /* tell mbedtls about our I/O functions */
    mbedtls_ssl_set_bio(
        &state.ssl_context,
        conn,
        altcp_mbedtls_bio_send,
        altcp_mbedtls_bio_recv,
        NULL,
    );

    altcp_mbedtls_setup_callbacks(conn, inner_conn);
    conn.inner_conn = inner_conn;
    conn.fns = altcp_mbedtls_functions.clone();
    conn.state = state;
    return Ok(());
}

pub fn altcp_tls_wrap(
    config: &mut altcp_tls_config,
    inner_pcb: &mut altcp_pcb,
) -> Option<altcp_pcb> {
    let ret: &mut altcp_pcb;
    if inner_pcb == NULL {
        return None;
    }
    ret = altcp_alloc();
    if ret != NULL {
        if altcp_mbedtls_setup(config, ret, inner_pcb) != ERR_OK {
            altcp_free(ret);
            return None;
        }
    }
    return ret;
}

pub fn altcp_tls_context(conn: Option<&mut altcp_pcb>) -> Option<mbedtls_ssl_context> {
    if conn.is_some() && conn.unwrap().state.is_some() {
        let state = conn.unwrap().state.unwrap();
        return Some(state.ssl_context.clone());
    }
    return None;
}

pub fn altcp_mbedtls_debug(
    ctx: &mut altcp_pcb,
    level: int,
    file: &String,
    line: i32,
    a_str: &String,
) {
    // LWIP_UNUSED_ARG(ctx);
    // LWIP_UNUSED_ARG(level);
    // LWIP_UNUSED_ARG(file);
    // LWIP_UNUSED_ARG(line);
    // LWIP_UNUSED_ARG(a_str);

    // LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("%s:%04d: %s", file, line, a_str));
}

/* ATTENTION: It is *really* important to *NOT* use this dummy RNG in production code!!!! */
pub fn dummy_rng(ctx: &mut altcp_pcb, buffer: &mut Vec<u8>, len: usize) -> i32 {
    let mut ctr: usize = 0;
    let mut i: usize;
    // LWIP_UNUSED_ARG(ctx);
    // TODO:
    // for (i = 0; i < len; i++) {
    //   buffer[i] = (unsigned char)++ctr;
    // }
    return 0;
}
// #define ALTCP_MBEDTLS_RNG_FN dummy_rng

/* Create new TLS configuration
 * ATTENTION: Server certificate and private key have to be added outside this function!
 */
pub fn altcp_tls_create_config(
    is_server: int,
    have_cert: int,
    have_pkey: int,
    have_ca: int,
) -> altcp_tls_config {
    let mut sz: usize;
    let mut ret: i32;
    let mut conf: altcp_tls_config = altcp_tls_config::new();
    let mut mem: mbedtls_x509_crt;

    if TCP_WND < MBEDTLS_SSL_MAX_CONTENT_LEN {
        LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                    ("altcp_tls: TCP_WND is smaller than the RX decryption buffer, connection RX might stall!\n"));
    }

    altcp_mbedtls_mem_init();

    // sz = sizeof(struct altcp_tls_config);
    sz = std::mem::size_of::<altcp_tls_config>();
    if have_cert {
        sz += sizeof(mbedtls_x509_crt);
    }
    if have_ca {
        sz += sizeof(mbedtls_x509_crt);
    }
    if have_pkey {
        sz += sizeof(mbedtls_pk_context);
    }

    // conf = altcp_mbedtls_alloc_config(sz);
    // if conf == NULL {
    //   return NULL;
    // }
    // TODO:
    // conf = altcp_tls_config::new();
    // mem = (conf + 1);
    // if have_cert {
    //     conf.cert = mem;
    //     mem ++;
    // }
    // if have_ca {
    //     conf.ca = mem;
    //     mem ++;
    // }
    // if have_pkey {
    //     conf.pkey = mem;
    // }

    mbedtls_ssl_config_init(&conf.conf);
    mbedtls_entropy_init(&conf.entropy);
    mbedtls_ctr_drbg_init(&conf.ctr_drbg);

    /* Seed the RNG */
    ret = mbedtls_ctr_drbg_seed(
        &conf.ctr_drbg,
        ALTCP_MBEDTLS_RNG_FN,
        &conf.entropy,
        ALTCP_MBEDTLS_ENTROPY_PTR,
        ALTCP_MBEDTLS_ENTROPY_LEN,
    );
    if ret != 0 {
        // LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("mbedtls_ctr_drbg_seed failed: %d\n", ret));
        altcp_mbedtls_free_config(&mut conf);
        return NULL;
    }

    /* Setup ssl context (@todo: what's different for a client here? -> might better be done on listen/connect) */
    let mut param = MBEDTLS_SSL_IS_CLIENT;
    if is_server {
        param = MBEDTLS_SSL_IS_SERVER;
    }
    ret = mbedtls_ssl_config_defaults(
        &conf.conf,
        param,
        MBEDTLS_SSL_TRANSPORT_STREAM,
        MBEDTLS_SSL_PRESET_DEFAULT,
    );
    if ret != 0 {
        // LWIP_DEBUGF(ALTCP_MBEDTLS_DEBUG, ("mbedtls_ssl_config_defaults failed: %d\n", ret));
        altcp_mbedtls_free_config(conf);
        return NULL;
    }
    mbedtls_ssl_conf_authmode(&conf.conf, MBEDTLS_SSL_VERIFY_OPTIONAL);

    mbedtls_ssl_conf_rng(&conf.conf, mbedtls_ctr_drbg_random, &conf.ctr_drbg);

    mbedtls_ssl_conf_dbg(&conf.conf, altcp_mbedtls_debug, stdout);

    mbedtls_ssl_conf_session_cache(
        &conf.conf,
        &conf.cache,
        mbedtls_ssl_cache_get,
        mbedtls_ssl_cache_set,
    );
    mbedtls_ssl_cache_set_timeout(&conf.cache, 30);
    mbedtls_ssl_cache_set_max_entries(&conf.cache, 30);

    return conf;
}

/* Create new TLS configuration
 * This is a suboptimal version that gets the encrypted private key and its password,
 * as well as the server certificate.
 */
pub fn altcp_tls_create_config_server_privkey_cert(
    privkey: &Vec<u8>,
    privkey_len: usize,
    privkey_pass: &Vec<u8>,
    privkey_pass_len: usize,
    cert: &Vec<u8>,
    cert_len: usize,
) -> altcp_tls_config {
    let ret: int;
    mbedtls_x509_crt * srvcert;
    mbedtls_pk_context * pkey;
    let conf: &mut altcp_tls_config = altcp_tls_create_config(1, 1, 1, 0);
    if conf == NULL {
        return NULL;
    }

    srvcert = conf.cert;
    mbedtls_x509_crt_init(srvcert);

    pkey = conf.pkey;
    mbedtls_pk_init(pkey);

    /* Load the certificates and private key */
    ret = mbedtls_x509_crt_parse(srvcert, cert, cert_len);
    if (ret != 0) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("mbedtls_x509_crt_parse failed: %d\n", ret),
        );
        altcp_mbedtls_free_config(conf);
        return NULL;
    }

    ret = mbedtls_pk_parse_key(pkey, privkey, privkey_len, privkey_pass, privkey_pass_len);
    if (ret != 0) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("mbedtls_pk_parse_public_key failed: %d\n", ret),
        );
        mbedtls_x509_crt_free(srvcert);
        altcp_mbedtls_free_config(conf);
        return NULL;
    }

    mbedtls_ssl_conf_ca_chain(&conf.conf, srvcert.next, NULL);
    ret = mbedtls_ssl_conf_own_cert(&conf.conf, srvcert, pkey);
    if (ret != 0) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("mbedtls_ssl_conf_own_cert failed: %d\n", ret),
        );
        mbedtls_x509_crt_free(srvcert);
        mbedtls_pk_free(pkey);
        altcp_mbedtls_free_config(conf);
        return NULL;
    }
    return conf;
}

pub fn altcp_tls_create_config_client_common(
    ca: &mut Vec<u8>,
    ca_len: usize,
    is_2wayauth: i32,
) -> altcp_tls_config {
    let ret: i32;
    let conf: &mut altcp_tls_config =
        altcp_tls_create_config(0, is_2wayauth, is_2wayauth, ca != NULL);
    if conf == NULL {
        return NULL;
    }

    /* Initialize the CA certificate if provided
     * CA certificate is optional (to save memory) but recommended for production environment
     * Without CA certificate, connection will be prone to man-in-the-middle attacks */
    if (ca) {
        mbedtls_x509_crt_init(conf.ca);
        ret = mbedtls_x509_crt_parse(conf.ca, ca, ca_len);
        if (ret != 0) {
            LWIP_DEBUGF(
                ALTCP_MBEDTLS_DEBUG,
                ("mbedtls_x509_crt_parse ca failed: %d 0x%x", ret, -1 * ret),
            );
            altcp_mbedtls_free_config(conf);
            return NULL;
        }

        mbedtls_ssl_conf_ca_chain(&conf.conf, conf.ca, NULL);
    }
    return conf;
}

pub fn altcp_tls_create_config_client(ca: &mut Vec<u8>, ca_len: usize) -> altcp_tls_config {
    return altcp_tls_create_config_client_common(ca, ca_len, 0);
}

pub fn altcp_tls_create_config_client_2wayauth(
    ca: &mut Vec<u8>,
    ca_len: usize,
    privkey: &mut Vec<u8>,
    privkey_len: usize,
    privkey_pass: &mut Vec<u8>,
    privkey_pass_len: usize,
    cert: &mut Vec<u8>,
    cert_len: usize,
) -> altcp_tls_config {
    let ret: int;
    let conf: &mut altcp_tls_config;

    if (!cert || !privkey) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("altcp_tls_create_config_client_2wayauth: certificate and priv key required"),
        );
        return NULL;
    }

    conf = altcp_tls_create_config_client_common(ca, ca_len, 1);
    if (conf == NULL) {
        return NULL;
    }

    /* Initialize the client certificate and corresponding private key */
    mbedtls_x509_crt_init(conf.cert);
    ret = mbedtls_x509_crt_parse(conf.cert, cert, cert_len);
    if (ret != 0) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("mbedtls_x509_crt_parse cert failed: %d 0x%x", ret, -1 * ret),
        );
        altcp_mbedtls_free_config(conf.cert);
        return NULL;
    }

    mbedtls_pk_init(conf.pkey);
    ret = mbedtls_pk_parse_key(
        conf.pkey,
        privkey,
        privkey_len,
        privkey_pass,
        privkey_pass_len,
    );
    if (ret != 0) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("mbedtls_pk_parse_key failed: %d 0x%x", ret, -1 * ret),
        );
        altcp_mbedtls_free_config(conf);
        return NULL;
    }

    ret = mbedtls_ssl_conf_own_cert(&conf.conf, conf.cert, conf.pkey);
    if (ret != 0) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            ("mbedtls_ssl_conf_own_cert failed: %d 0x%x", ret, -1 * ret),
        );
        altcp_mbedtls_free_config(conf);
        return NULL;
    }

    return conf;
}

pub fn altcp_tls_free_config(conf: &mut altcp_tls_config) {
    if (conf.pkey) {
        mbedtls_pk_free(conf.pkey);
    }
    if (conf.cert) {
        mbedtls_x509_crt_free(conf.cert);
    }
    if (conf.ca) {
        mbedtls_x509_crt_free(conf.ca);
    }
    altcp_mbedtls_free_config(conf);
}

/* "virtual" functions */
pub fn altcp_mbedtls_set_poll(conn: &mut altcp_pcb, interval: u8) {
    if (conn != NULL) {
        altcp_poll(conn.inner_conn, altcp_mbedtls_lower_poll, interval);
    }
}

pub fn altcp_mbedtls_recved(conn: &mut altcp_pcb, len: u16) {
    let lower_recved: u16;
    altcp_mbedtls_state * state;
    if (conn == NULL) {
        return;
    }
    state = conn.state;
    if (state == NULL) {
        return;
    }
    if (!(state.flags & ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE)) {
        return;
    }
    lower_recved = len;
    if (lower_recved > state.rx_passed_unrecved) {
        LWIP_DEBUGF(
            ALTCP_MBEDTLS_DEBUG,
            (
                "bogus recved count (len > state.rx_passed_unrecved / %d / %d)",
                len,
                state.rx_passed_unrecved,
            ),
        );
        lower_recved = state.rx_passed_unrecved;
    }
    state.rx_passed_unrecved -= lower_recved;

    altcp_recved(conn.inner_conn, lower_recved);
}

pub fn altcp_mbedtls_connect(
    conn: &mut altcp_pcb,
    ipaddr: &ip_addr_t,
    port: u16,
    connected: altcp_connected_fn,
) -> Result<(), LwipError> {
    if (conn == NULL) {
        return ERR_VAL;
    }
    conn.connected = connected;
    return altcp_connect(conn.inner_conn, ipaddr, port, altcp_mbedtls_lower_connected);
}

pub fn altcp_mbedtls_listen(conn: &mut altcp_pcb, backlog: u8, err: &mut err_t) -> altcp_pcb {
    let lpcb: &mut altcp_pcb;
    if (conn == NULL) {
        return NULL;
    }
    lpcb = altcp_listen_with_backlog_and_err(conn.inner_conn, backlog, err);
    if (lpcb != NULL) {
        conn.inner_conn = lpcb;
        altcp_accept(lpcb, altcp_mbedtls_lower_accept);
        return conn;
    }
    return NULL;
}

pub fn altcp_mbedtls_abort(conn: &mut altcp_pcb) {
    if (conn != NULL) {
        altcp_abort(conn.inner_conn);
    }
}

pub fn altcp_mbedtls_close(conn: &mut altcp_pcb) -> Result<(), &str> {
    let inner_conn: &mut altcp_pcb;
    if (conn == NULL) {
        return ERR_VAL;
    }
    inner_conn = conn.inner_conn;
    if (inner_conn) {
        let err: err_t;

        let oldpoll: altcp_poll_fn = inner_conn.poll;
        altcp_mbedtls_remove_callbacks(conn.inner_conn);
        err = altcp_close(conn.inner_conn);
        if (err != ERR_OK) {
            /* not closed, set up all callbacks again */
            altcp_mbedtls_setup_callbacks(conn, inner_conn);
            /* poll callback is not included in the above */
            altcp_poll(inner_conn, oldpoll, inner_conn.pollinterval);
            return err;
        }
        conn.inner_conn = NULL;
    }
    altcp_free(conn);
    return ERR_OK;
}

/* Allow caller of altcp_write() to limit to negotiated chunk size
 *  or remaining sndbuf space of inner_conn.
 */
pub fn altcp_mbedtls_sndbuf(conn: &mut altcp_pcb) -> u16 {
    if conn {
        altcp_mbedtls_state * state;
        state = conn.state;
        if !state || !(state.flags & ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE) {
            return 0;
        }
        if conn.inner_conn {
            let sndbuf: u16 = altcp_sndbuf(conn.inner_conn);
            /* Take care of record header, IV, AuthTag */
            let ssl_expan: int = mbedtls_ssl_get_record_expansion(&state.ssl_context);
            if ssl_expan > 0 {
                let ssl_added: usize = LWIP_MIN(ssl_expan, 0xFFFF);
                /* internal sndbuf smaller than our offset */
                if ssl_added < sndbuf {
                    let mut max_len: usize = 0xFFFF;
                    let ret: usize;

                    /* @todo: adjust ssl_added to real value related to negociated cipher */
                    let mut max_frag_len: usize = mbedtls_ssl_get_max_frag_len(&state.ssl_context);
                    max_len = LWIP_MIN(max_frag_len, max_len);

                    /* Adjust sndbuf of inner_conn with what added by SSL */
                    ret = LWIP_MIN(sndbuf - ssl_added, max_len);
                    LWIP_ASSERT("sndbuf overflow", ret <= 0xFFFF);
                    return ret;
                }
            }
        }
    }
    /* fallback: use sendbuf of the inner connection */
    return altcp_default_sndbuf(conn);
}

/* Write data to a TLS connection. Calls into mbedTLS, which in turn calls into
 * @ref altcp_mbedtls_bio_send() to send the encrypted data
 */
pub fn altcp_mbedtls_write(
    conn: &mut altcp_pcb,
    dataptr: &Vec<u8>,
    len: u16,
    apiflags: u8,
) -> Result<(), LwipError> {
    let ret: i32;
    let state: altcp_mbedtls_state;

    LWIP_UNUSED_ARG(apiflags);

    if (conn == NULL) {
        return ERR_VAL;
    }

    state = conn.state;
    if (state == NULL) {
        /* @todo: which error? */
        return ERR_CLSD;
    }
    if (!(state.flags & ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE)) {
        /* @todo: which error? */
        return ERR_VAL;
    }

    /* HACK: if thre is something left to send, try to flush it and only
    allow sending more if this succeeded (this is a hack because neither
    returning 0 nor MBEDTLS_ERR_SSL_WANT_WRITE worked for me) */
    if (state.ssl_context.out_left) {
        mbedtls_ssl_flush_output(&state.ssl_context);
        if (state.ssl_context.out_left) {
            return ERR_MEM;
        }
    }
    ret = mbedtls_ssl_write(&state.ssl_context, dataptr, len);
    /* try to send data... */
    altcp_output(conn.inner_conn);
    if (ret >= 0) {
        if (ret == len) {
            state.flags |= ALTCP_MBEDTLS_FLAGS_APPLDATA_SENT;
            return ERR_OK;
        } else {
            /* @todo/@fixme: assumption: either everything sent or error */
            LWIP_ASSERT("ret <= 0", 0);
            return ERR_MEM;
        }
    } else {
        if (ret == MBEDTLS_ERR_SSL_WANT_WRITE) {
            /* @todo: convert error to err_t */
            return ERR_MEM;
        }
        LWIP_ASSERT("unhandled error", 0);
        return ERR_VAL;
    }
}

/* Send callback function called from mbedtls (set via mbedtls_ssl_set_bio)
 * This function is either called during handshake or when sending application
 * data via @ref altcp_mbedtls_write (or altcp_write)
 */
pub fn altcp_mbedtls_bio_send(ctx: &mut Vec<u8>, dataptr: &mut Vec<u8>, state: usize) -> i32 {
    let conn: &mut altcp_pcb = ctx;
    let written: i32 = 0;
    let size_left: usize = size;
    let apiflags: u8 = TCP_WRITE_FLAG_COPY;

    LWIP_ASSERT("conn != NULL", conn != NULL);
    if ((conn == NULL) || (conn.inner_conn == NULL)) {
        return MBEDTLS_ERR_NET_INVALID_CONTEXT;
    }

    while (size_left) {
        let write_len: u16 = LWIP_MIN(size_left, 0xFFFF);
        let res = altcp_write(conn.inner_conn, dataptr, write_len, apiflags);
        if (err == ERR_OK) {
            written += write_len;
            size_left -= write_len;
        } else if (res == ERR_MEM) {
            if (written) {
                return written;
            }
            return 0; /* MBEDTLS_ERR_SSL_WANT_WRITE; */
        } else {
            LWIP_ASSERT("tls_write, tcp_write: err != ERR MEM", 0); /* @todo: return MBEDTLS_ERR_NET_CONN_RESET or MBEDTLS_ERR_NET_SEND_FAILED */
            return MBEDTLS_ERR_NET_SEND_FAILED;
        }
    }
    return written;
}

pub fn altcp_mbedtls_mss(conn: &mut altcp_pcb) -> u16 {
    if (conn == NULL) {
        return 0;
    }
    /* @todo: LWIP_MIN(mss, mbedtls_ssl_get_max_frag_len()) ? */
    return altcp_mss(conn.inner_conn);
}

pub fn altcp_mbedtls_dealloc(conn: &mut altcp_pcb) {
    /* clean up and free tls state */
    if (conn) {
        altcp_mbedtls_state * state = conn.state;
        if (state) {
            mbedtls_ssl_free(&state.ssl_context);
            state.flags = 0;
            if (state.rx) {
                /* free leftover (unhandled) rx pbufs */
                pbuf_free(state.rx);
                state.rx = NULL;
            }
            altcp_mbedtls_free(state.conf, state);
            conn.state = NULL;
        }
    }
}

// pub const altcp_mbedtls_functions: altcp_functions = {
// altcp_mbedtls_set_poll,
// altcp_mbedtls_recved,
// altcp_default_bind,
// altcp_mbedtls_connect,
// altcp_mbedtls_listen,
// altcp_mbedtls_abort,
// altcp_mbedtls_close,
// altcp_default_shutdown,
// altcp_mbedtls_write,
// altcp_default_output,
// altcp_mbedtls_mss,
// altcp_mbedtls_sndbuf,
// altcp_default_sndqueuelen,
// altcp_default_nagle_disable,
// altcp_default_nagle_enable,
// altcp_default_nagle_disabled,
// altcp_default_setprio,
// altcp_mbedtls_dealloc,
// altcp_default_get_tcp_addrinfo,
// altcp_default_get_ip,
// altcp_default_get_port
// , altcp_default_dbg_get_tcp_state

// };
