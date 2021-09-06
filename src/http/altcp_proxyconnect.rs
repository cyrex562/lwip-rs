/*
 * @file
 * Application layered TCP connection API that executes a proxy-connect.
 *
 * This file provides a starting layer that executes a proxy-connect e.g. to
 * set up TLS connections through a http proxy.
 */

/*
 * Copyright (c) 2018 Simon Goldschmidt
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
 */

/* This string is passed in the HTTP header as "User-Agent: " */

// #define ALTCP_PROXYCONNECT_CLIENT_AGENT "lwIP/" LWIP_VERSION_STRING " (http://savannah.nongnu.org/projects/lwip)"

use crate::altcp_tls::altcp_tls_mbedtls::altcp_tls_wrap;
use crate::core::altcp::{
    altcp_abort, altcp_alloc, altcp_arg, altcp_close, altcp_connect, altcp_err, altcp_free,
    altcp_poll, altcp_recv, altcp_recved, altcp_sent, altcp_write,
};
use crate::core::altcp_h::AlTcpPcb;
use crate::core::altcp_tcp::altcp_tcp_new_ip_type;
use crate::core::def_h::None;
use crate::core::err_h::{LwipError, ERR_ABRT, ERR_ARG, ERR_CLSD, ERR_MEM, ERR_OK, ERR_VAL};
use crate::core::ip2::ipaddr_ntoa;
use crate::core::pbuf::pbuf_memfind;
use crate::core::pbuf_h::PacketBuffer;
use crate::core::tcpbase_h::TCP_WRITE_FLAG_COPY;
use crate::defines::LwipAddr;
use std::future::Future;

pub const ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED: u32 = 0x01;
pub const ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE: u32 = 0x02;

pub struct AlTcpProxyConnectState {
    pub outer_addr: LwipAddr,
    pub outer_port: u16,
    pub conf: altcp_proxyconnect_config,
    pub flags: u8,
}

/* Variable prototype, the actual declaration is at the end of this file
since it contains pointers to static functions declared here */
// extern const struct altcp_functions altcp_proxyconnect_functions;

/* memory management functions: */

pub fn altcp_proxyconnect_state_alloc() -> AlTcpProxyConnectState {
    // ret: &mut AlTcpProxyConnectState = mem_calloc(1, sizeof(altcp_proxyconnect_state_t));
    // return ret;
    return AlTcpProxyConnectState::new();
}

pub fn altcp_proxyconnect_state_free(state: &mut AlTcpProxyConnectState) {
    // LWIP_ASSERT("state != NULL", state != NULL);
    // mem_free(state);
    unimplemented!()
}

/* helper functions */

pub const PROXY_CONNECT: String = r#"CONNECT {}:{} HTTP/1.1\r\n"
  User-Agent: %s\r\n
  Proxy-Connection: keep-alive\r\n
  Connection: keep-alive\r\n
  \r\n"#
    .to_string();

pub fn PROXY_CONNECT_FORMAT(host: &String, port: u16, user_agent: &String) -> String {
    format!(PROXY_CONNECT, host, port, user_agent)
}

/* Format the http proxy connect request via snprintf */
pub fn altcp_proxyconnect_format_request(host: &String, port: u16) -> String {
    // return snprintf(buffer, bufsize, PROXY_CONNECT_FORMAT(host, port));
    PROXY_CONNECT_FORMAT(host, port, &"".to_string())
}

/* Create and send the http proxy connect request */
pub fn altcp_proxyconnect_send_request(conn: &mut AlTcpPcb) -> Result<(), LwipError> {
    let mut len2: usize;
    let mem_alloc_len: usize;
    let mut buffer: String;
    let host: String;
    let state = &conn.state;

    /* Use printf with zero length to get the required allocation size */
    /* add allocation size for IP address strings */

    len += 40; /* worst-case IPv6 address length */
    len += 16; /* worst-case IPv4 address length */

    /* Allocate a bufer for the request string */

    host = ipaddr_ntoa(&state.outer_addr);
    buffer = altcp_proxyconnect_format_request(&host, state.outer_port);
    if (len2 > 0) && (len2 <= len) && (len2 <= 0xFFFF) {
        let err: err_t = altcp_write(conn.inner_conn, buffer.as_bytes(), len, TCP_WRITE_FLAG_COPY);
        if err != ERR_OK {
            /* @todo: abort? */
            return err;
        }
    }
    return Ok(());
}

/* callback functions from inner/lower connection: */

/* Connected callback from lower connection (i.e. TCP).
 * Not really implemented/tested yet...
 */
pub fn altcp_proxyconnect_lower_connected(
    arg: &mut AlTcpPcb,
    inner_conn: &mut AlTcpPcb,
    err: err_t,
) -> Result<(), LwipError> {
    let conn: &mut AlTcpPcb = arg;
    if conn.state.is_some() {
        LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
        /* for LWIP_NOASSERT */
        /* upper connected is called when handshake is done */
        if err != ERR_OK {
            if conn.connected {
                if conn.connected(conn.arg.unwrap(), conn, err) == ERR_ABRT {
                    return Err(LwipError::new(ERR_ABRT, "abort"));
                }
                return Ok(());
            }
        }
        /* send proxy connect request here */
        return altcp_proxyconnect_send_request(conn);
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/* Recv callback from lower connection (i.e. TCP)
 * This one mainly differs between connection setup (wait for proxy OK string)
 * and application phase (data is passed on to the application).
 */
pub fn altcp_proxyconnect_lower_recv(
    arg: &mut AlTcpPcb,
    inner_conn: &mut AlTcpPcb,
    p: Option<&mut PacketBuffer>,
    err: err_t,
) -> Result<(), LwipError> {
    let conn: &mut AlTcpPcb = arg;
    LWIP_ASSERT("no err expected", err == ERR_OK);

    let state = &conn.state;
    LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    if state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE {
        /* application phase, just pass this through */
        if conn.recv.is_some() {
            return conn.recv.unwrap()(&mut conn.arg.unwrap(), conn, p.unwrap(), err);
        }
        pbuf_free(p);
        return Ok(());
    } else {
        /* setup phase */
        /* handle NULL pbuf (inner connection closed) */
        return if p.is_none() {
            if altcp_close(conn) != ERR_OK {
                altcp_abort(conn);
                return Err(LwipError::new(ERR_ABRT, "abort"));
            }
            Ok(())
        } else {
            /* @todo: parse setup phase rx data
            for now, we just wait for the end of the header... */
            let idx = pbuf_memfind(p.unwrap(), "\r\n\r\n".as_bytes(), 4, 0);
            altcp_recved(inner_conn, p.unwrap().tot_len);
            // pbuf_free(p);
            if idx != 0xFFFF {
                state.flags |= ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE;
                if conn.connected {
                    return conn.connected(&conn.arg, conn, ERR_OK);
                }
            }
            ERR_OK
        };
    }
}

/* Sent callback from lower connection (i.e. TCP)
 * This only informs the upper layer to try to send more, not about
 * the number of ACKed bytes.
 */
pub fn altcp_proxyconnect_lower_sent(
    arg: &mut AlTcpPcb,
    inner_conn: &mut AlTcpPcb,
    len: usize,
) -> Result<(), LwipError> {
    let conn: &mut AlTcpPcb = arg;

    if conn.state.is_some() {
        let state = conn.state.unwrap();
        LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
        /* for LWIP_NOASSERT */
        if !(state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE) {
            /* @todo: do something here? */
            return Ok(());
        }
        /* pass this on to upper sent */
        if conn.sent.is_some() {
            return conn.sent.unwrap()(&mut conn.arg.unwrap(), conn, len);
        }
    }
    return Ok(());
}

/* Poll callback from lower connection (i.e. TCP)
 * Just pass this on to the application.
 * @todo: retry sending?
 */
pub fn altcp_proxyconnect_lower_poll(
    arg: &mut AlTcpPcb,
    inner_conn: &mut AlTcpPcb,
) -> Result<(), LwipError> {
    let conn: &mut AlTcpPcb = arg;
    LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    /* for LWIP_NOASSERT */
    if conn.poll.is_some() {
        return conn.poll.unwrap()(&mut conn.arg.unwrap(), conn);
    }
    return Ok(());
}

pub fn altcp_proxyconnect_lower_err(arg: &mut AlTcpPcb, err: err_t) {
    let conn: &mut AlTcpPcb = arg;
    conn.inner_conn = None; /* already freed */
    if conn.err.is_some() {
        conn.err(&mut conn.arg, err);
    }
    altcp_free(conn);
}

/* setup functions */

pub fn altcp_proxyconnect_setup_callbacks(conn: &mut AlTcpPcb, inner_conn: &mut AlTcpPcb) {
    altcp_arg(inner_conn, Some(conn));
    inner_conn.recv = Some(altcp_proxyconnect_lower_recv);
    inner_conn.sent = Some(altcp_proxyconnect_lower_sent);
    inner_conn.err = Some(altcp_proxyconnect_lower_err);
    /* tcp_poll is set when interval is set by application */
    /* listen is set totally different :-) */
}

pub fn altcp_proxyconnect_setup(
    config: &mut altcp_proxyconnect_config,
    conn: &mut AlTcpPcb,
    inner_conn: &mut AlTcpPcb,
) -> Result<(), LwipError> {
    LWIP_ASSERT("invalid inner_conn", conn != inner_conn);

    /* allocate proxyconnect context */
    let mut state = altcp_proxyconnect_state_alloc();
    state.flags = 0;
    state.conf = config;
    altcp_proxyconnect_setup_callbacks(conn, inner_conn);
    conn.inner_conn = inner_conn;
    conn.fns = &altcp_proxyconnect_functions;
    conn.state = state;
    return Ok(());
}

/* Allocate a new altcp layer connecting through a proxy.
 * This function gets the inner pcb passed.
 *
 * @param config struct AltcpProxyconnectConfig that contains the proxy settings
 * @param inner_pcb pcb that makes the connection to the proxy (i.e. tcp pcb)
 */
pub fn altcp_proxyconnect_new(
    config: &mut altcp_proxyconnect_config,
    inner_pcb: &mut AlTcpPcb,
) -> Option<AlTcpPcb> {
    let mut ret = altcp_alloc();
    if altcp_proxyconnect_setup(config, &mut ret, inner_pcb).is_err() {
        altcp_free(&mut ret);
        return None;
    }
    return Some(ret);
}

/* Allocate a new altcp layer connecting through a proxy.
 * This function allocates the inner pcb as tcp pcb, resulting in a direct tcp
 * connection to the proxy.
 *
 * @param config struct AltcpProxyconnectConfig that contains the proxy settings
 * @param ip_type IP type of the connection (@ref LwipIpAddrType)
 */
pub fn altcp_proxyconnect_new_tcp(
    config: &mut altcp_proxyconnect_config,
    ip_type: u8,
) -> Option<AlTcpPcb> {
    /* inner pcb is tcp */
    let mut inner_pcb = altcp_tcp_new_ip_type(ip_type);
    altcp_proxyconnect_new(config, inner_pcb)
}

/* Allocator function to allocate a proxy connect altcp pcb connecting directly
 * via tcp to the proxy.
 *
 * The returned pcb is a chain: altcp_proxyconnect - altcp_tcp - tcp pcb
 *
 * This function is meant for use with @ref altcp_new.
 *
 * @param arg struct AltcpProxyconnectConfig that contains the proxy settings
 * @param ip_type IP type of the connection (@ref LwipIpAddrType)
 */
pub fn altcp_proxyconnect_alloc(arg: &mut Vec<u8>, ip_type: u8) -> Option<AlTcpPcb> {
    altcp_proxyconnect_new_tcp(arg, ip_type)
}

/* Allocator function to allocate a TLS connection through a proxy.
 *
 * The returned pcb is a chain: altcp_tls - altcp_proxyconnect - altcp_tcp - tcp pcb
 *
 * This function is meant for use with @ref altcp_new.
 *
 * @param arg struct AltcpProxyconnectTlsConfig that contains the proxy settings
 *        and tls settings
 * @param ip_type IP type of the connection (@ref LwipIpAddrType)
 */
pub fn altcp_proxyconnect_tls_alloc(arg: &mut Vec<u8>, ip_type: u8) -> Option<AlTcpPcb> {
    let cfg: &mut altcp_proxyconnect_tls_config = arg;
    let proxy_pcb: &mut AlTcpPcb;
    let tls_pcb: &mut AlTcpPcb;
    let proxy_pcb = altcp_proxyconnect_new_tcp(&mut cfg.proxy, ip_type);
    altcp_tls_wrap(cfg.tls_config, &mut proxy_pcb.unwrap())
}

/* "virtual" functions */
pub fn altcp_proxyconnect_set_poll(conn: &mut AlTcpPcb, interval: u8) {
    altcp_poll(
        conn.inner_conn,
        Some(altcp_proxyconnect_lower_poll),
        interval,
    )
}

pub fn altcp_proxyconnect_recved(conn: &mut AlTcpPcb, len: usize) {
    let mut state = conn.state;
    if !(state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE) {
        return;
    }
    altcp_recved(conn.inner_conn, len);
}

pub fn altcp_proxyconnect_connect(
    conn: &mut AlTcpPcb,
    ipaddr: &mut LwipAddr,
    port: u16,
    connected: altcp_connected_fn,
) -> Result<(), LwipError> {
    let mut state = conn.state;
    if state.flags & ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED {
        return Err(LwipError::new(ERR_VAL, "value error"));
    }
    state.flags |= ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED;

    conn.connected = connected;
    /* connect to our proxy instead, but store the requested address and port */
    ip_addr_copy(state.outer_addr, *ipaddr);
    state.outer_port = port;

    return altcp_connect(
        conn.inner_conn,
        &mut state.conf.proxy_addr,
        state.conf.proxy_port,
        altcp_proxyconnect_lower_connected,
    );
}

pub fn altcp_proxyconnect_listen(
    conn: &mut AlTcpPcb,
    backlog: u8,
    err: &mut err_t,
) -> Option<AlTcpPcb> {
    unimplemented!();
    None
}

pub fn altcp_proxyconnect_abort(conn: &mut AlTcpPcb) {
    if conn.inner_conn != None {
        altcp_abort(conn.inner_conn);
    }
    altcp_free(conn);
}

pub fn altcp_proxyconnect_close(conn: &mut AlTcpPcb) -> Result<(), LwipError> {
    if conn.inner_conn != None {
        let err = altcp_close(conn.inner_conn);
        if err.is_err() {
            /* closing inner conn failed, return the error */
            return err;
        }
    }
    /* no inner conn or closing it succeeded, deallocate myself */
    altcp_free(conn);
    return Ok(());
}

pub fn altcp_proxyconnect_write(
    conn: &mut AlTcpPcb,
    dataptr: &Vec<u8>,
    len: usize,
    apiflags: u8,
) -> Result<(), LwipError> {
    if conn.state.is_none() {
        return Err(LwipError::new(ERR_CLSD, "error closed"));
    }
    let state = conn.state.unwrap();
    if !(state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE) {
        /* @todo: which error? */
        return Err(LwipError::new(ERR_VAL, "error value"));
    }
    return altcp_write(conn.inner_conn, dataptr, len, apiflags);
}

pub fn altcp_proxyconnect_dealloc(conn: &mut AlTcpPcb) {
    /* clean up and free tls state */
    if (conn) {
        let state: &mut AlTcpProxyConnectState = conn.state;
        if (state) {
            altcp_proxyconnect_state_free(state);
            conn.state = None;
        }
    }
}

// const struct altcp_functions altcp_proxyconnect_functions = {
// altcp_proxyconnect_set_poll,
// altcp_proxyconnect_recved,
// altcp_default_bind,
// altcp_proxyconnect_connect,
// altcp_proxyconnect_listen,
// altcp_proxyconnect_abort,
// altcp_proxyconnect_close,
// altcp_default_shutdown,
// altcp_proxyconnect_write,
// altcp_default_output,
// altcp_default_mss,
// altcp_default_sndbuf,
// altcp_default_sndqueuelen,
// altcp_default_nagle_disable,
// altcp_default_nagle_enable,
// altcp_default_nagle_disabled,
// altcp_default_setprio,
// altcp_proxyconnect_dealloc,
// altcp_default_get_tcp_addrinfo,
// altcp_default_get_ip,
// altcp_default_get_port
//
// , altcp_default_dbg_get_tcp_state
//
// };
