/*
 * @file
 * Application layered TCP connection API (to be used from TCPIP thread)\n
 * This interface mimics the tcp callback API to the application while preventing
 * direct linking (much like virtual functions).
 * This way, an application can make use of other application layer protocols
 * on top of TCP without knowing the details (e.g. TLS, proxy connection).
 *
 * This file contains the base implementation calling into tcp.
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
 */

// #define ALTCP_TCP_ASSERT_CONN(conn) loop { \
//   LWIP_ASSERT("conn.inner_conn == NULL", conn.inner_conn == NULL); \
//    /* for LWIP_NOASSERT */ } while(0)
// #define ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb) loop { \
//   LWIP_ASSERT("pcb mismatch", conn.state == tpcb); \
//    /* for LWIP_NOASSERT */ \
//   ALTCP_TCP_ASSERT_CONN(conn); } while(0)

/* Variable prototype, the actual declaration is at the end of this file
since it contains pointers to static functions declared here */
// extern const struct AltcpFunctions altcp_tcp_functions;

// pub fn altcp_tcp_setup(conn: &mut AltcpPcb, tpcb: &mut tcp_pcb);

/* callback functions for TCP */
use crate::core::altcp_h::AltcpFunctions;

pub fn altcp_tcp_accept(arg: &mut Vec<u8>, new_tpcb: &mut tcp_pcb, err: err_t) -> err_t {
    // TODO: let listen_conn: &mut AltcpPcb = arg;

    if (listen_conn && listen_conn.accept) {
        /* create a new altcp_conn to pass to the next 'accept' callback */
        let new_conn: &mut AlTcpPcb = altcp_alloc();
        if (new_conn == None) {
            return ERR_MEM;
        }
        altcp_tcp_setup(new_conn, new_tpcb);
        return listen_conn.accept(listen_conn.arg, new_conn, err);
    }
    return ERR_ARG;
}

pub fn altcp_tcp_connected(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb, err: err_t) -> err_t {
    // let conn: &mut AltcpPcb = arg;
    if (conn) {
        ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb);
        if (conn.connected) {
            return conn.connected(conn.arg, conn, err);
        }
    }
   return Ok(());
}

pub fn altcp_tcp_recv(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb, p: &mut pbuf, err: err_t) -> err_t {
    // let conn: &mut AltcpPcb = arg;
    if (conn) {
        ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb);
        if (conn.recv) {
            return conn.recv(conn.arg, conn, p, err);
        }
    }
    if (p != None) {
        /* prevent memory leaks */
        pbuf_free(p);
    }
   return Ok(());
}

pub fn altcp_tcp_sent(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb, len: usize) -> err_t {
    // TODO: conn: &mut AltcpPcb = arg;
    if (conn) {
        ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb);
        if (conn.sent) {
            return conn.sent(conn.arg, conn, len);
        }
    }
   return Ok(());
}

pub fn altcp_tcp_poll(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb) -> err_t {
    // TODO: conn: &mut AltcpPcb = arg;
    if (conn) {
        ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb);
        if (conn.poll) {
            return conn.poll(conn.arg, conn);
        }
    }
   return Ok(());
}

pub fn altcp_tcp_err(arg: &mut Vec<u8>, err: err_t) {
    // TODO: conn: &mut AltcpPcb = arg;
    if (conn) {
        conn.state = None; /* already freed */
        if (conn.err) {
            conn.err(conn.arg, err);
        }
        altcp_free(conn);
    }
}

/* setup functions */

pub fn altcp_tcp_remove_callbacks(tpcb: &mut tcp_pcb) {
    tcp_arg(tpcb, None);
    tcp_recv(tpcb, None);
    tcp_sent(tpcb, None);
    tcp_err(tpcb, None);
    tcp_poll(tpcb, None, tpcb.pollinterval);
}

pub fn altcp_tcp_setup_callbacks(conn: &mut AlTcpPcb, tpcb: &mut tcp_pcb) {
    tcp_arg(tpcb, conn);
    tcp_recv(tpcb, altcp_tcp_recv);
    tcp_sent(tpcb, altcp_tcp_sent);
    tcp_err(tpcb, altcp_tcp_err);
    /* tcp_poll is set when interval is set by application */
    /* listen is set totally different :-) */
}

pub fn altcp_tcp_setup(conn: &mut AlTcpPcb, tpcb: &mut tcp_pcb) {
    altcp_tcp_setup_callbacks(conn, tpcb);
    conn.state = tpcb;
    conn.fns = &altcp_tcp_functions;
}

pub fn altcp_tcp_new_ip_type(ip_type: u8) -> &mut AlTcpPcb {
    /* Allocate the tcp pcb first to invoke the priority handling code
    if we're out of pcbs */
    let tpcb: &mut tcp_pcb = tcp_new_ip_type(ip_type);
    if (tpcb != None) {
        let ret: &mut AlTcpPcb = altcp_alloc();
        if (ret != None) {
            altcp_tcp_setup(ret, tpcb);
            return ret;
        } else {
            /* AltcpPcb allocation failed -> free the tcp_pcb too */
            tcp_close(tpcb);
        }
    }
    return None;
}

/* altcp_tcp allocator function fitting to @ref AltcpAllocatorT / @ref altcp_new.
*
* arg pointer is not used for TCP.
*/
pub fn altcp_tcp_alloc(arg: &mut Vec<u8>, ip_type: u8) -> &mut AlTcpPcb {
    return altcp_tcp_new_ip_type(ip_type);
}

pub fn altcp_tcp_wrap(tpcb: &mut tcp_pcb) -> &mut AlTcpPcb {
    if (tpcb != None) {
        let ret: &mut AlTcpPcb = altcp_alloc();
        if (ret != None) {
            altcp_tcp_setup(ret, tpcb);
            return ret;
        }
    }
    return None;
}

/* "virtual" functions calling into tcp */
pub fn altcp_tcp_set_poll(conn: &mut AlTcpPcb, interval: u8) {
    if (conn != None) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        tcp_poll(pcb, altcp_tcp_poll, interval);
    }
}

pub fn altcp_tcp_recved(conn: &mut AlTcpPcb, len: usize) {
    if (conn != None) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        tcp_recved(pcb, len);
    }
}

pub fn altcp_tcp_bind(conn: &mut AlTcpPcb, ipaddr: &LwipAddr, port: u16) -> err_t {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_bind(pcb, ipaddr, port);
}

pub fn altcp_tcp_connect(
    conn: &mut AlTcpPcb,
    ipaddr: &LwipAddr,
    port: u16,
    connected: altcp_connected_fn,
) -> err_t {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    conn.connected = connected;
    pcb = conn.state;
    return tcp_connect(pcb, ipaddr, port, altcp_tcp_connected);
}

pub fn altcp_tcp_listen(conn: &mut AlTcpPcb, backlog: u8, err: &mut err_t) -> &mut AlTcpPcb {
    let pcb: &mut tcp_pcb;
    let lpcb: &mut tcp_pcb;
    if (conn == None) {
        return None;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    lpcb = tcp_listen_with_backlog_and_err(pcb, backlog, err);
    if (lpcb != None) {
        conn.state = lpcb;
        tcp_accept(lpcb, altcp_tcp_accept);
        return conn;
    }
    return None;
}

pub fn altcp_tcp_abort(conn: &mut AlTcpPcb) {
    if (conn != None) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        if (pcb) {
            tcp_abort(pcb);
        }
    }
}

pub fn altcp_tcp_close(conn: &mut AlTcpPcb) -> err_t {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    if (pcb) {
        let err: err_t;
        let oldpoll: tcp_poll_fn = pcb.poll;
        altcp_tcp_remove_callbacks(pcb);
        err = tcp_close(pcb);
        if (err != ERR_OK) {
            /* not closed, set up all callbacks again */
            altcp_tcp_setup_callbacks(conn, pcb);
            /* poll callback is not included in the above */
            tcp_poll(pcb, oldpoll, pcb.pollinterval);
            return err;
        }
        conn.state = None; /* unsafe to reference pcb after tcp_close(). */
    }
    altcp_free(conn);
   return Ok(());
}

pub fn altcp_tcp_shutdown(conn: &mut AlTcpPcb, shut_rx: i32, shut_tx: i32) -> err_t {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_shutdown(pcb, shut_rx, shut_tx);
}

pub fn altcp_tcp_write(conn: &mut AlTcpPcb, dataptr: &Vec<u8>, len: usize, apiflags: u8) -> err_t {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_write(pcb, dataptr, len, apiflags);
}

pub fn altcp_tcp_output(conn: &mut AlTcpPcb) -> err_t {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_output(pcb);
}

pub fn altcp_tcp_mss(conn: &mut AlTcpPcb) -> u16 {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return 0;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_mss(pcb);
}

pub fn altcp_tcp_sndbuf(conn: &mut AlTcpPcb) -> u16 {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return 0;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_sndbuf(pcb);
}

pub fn altcp_tcp_sndqueuelen(conn: &mut AlTcpPcb) -> u16 {
    let pcb: &mut tcp_pcb;
    if (conn == None) {
        return 0;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    pcb = conn.state;
    return tcp_sndqueuelen(pcb);
}

pub fn altcp_tcp_nagle_disable(conn: &mut AlTcpPcb) {
    if (conn && conn.state) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        tcp_nagle_disable(pcb);
    }
}

pub fn altcp_tcp_nagle_enable(conn: &mut AlTcpPcb) {
    if (conn && conn.state) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        tcp_nagle_enable(pcb);
    }
}

pub fn altcp_tcp_nagle_disabled(conn: &mut AlTcpPcb) -> i32 {
    if (conn && conn.state) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        return tcp_nagle_disabled(pcb);
    }
    return 0;
}

pub fn altcp_tcp_setprio(conn: &mut AlTcpPcb, prio: u8) {
    if (conn != None) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        tcp_setprio(pcb, prio);
    }
}

pub fn altcp_tcp_dealloc(conn: &mut AlTcpPcb) {
    ALTCP_TCP_ASSERT_CONN(conn);
    /* no private state to clean up */
}

pub fn altcp_tcp_get_tcp_addrinfo(
     conn: &mut AlTcpPcb,
    local: i32,
    addr: &mut LwipAddr,
    port: &mut u16,
) -> err_t {
    if (conn) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        return tcp_tcp_get_tcp_addrinfo(pcb, local, addr, port);
    }
    return ERR_VAL;
}

pub fn altcp_tcp_get_ip(conn: &mut AlTcpPcb, local: i32) -> &mut LwipAddr {
    if (conn) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        if (pcb) {
            if (local) {
                return &pcb.local_ip;
            } else {
                return &pcb.remote_ip;
            }
        }
    }
    return None;
}

pub fn altcp_tcp_get_port(conn: &mut AlTcpPcb, local: i32) -> u16 {
    if (conn) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        if (pcb) {
            if (local) {
                return pcb.local_port;
            } else {
                return pcb.remote_port;
            }
        }
    }
    return 0;
}

pub fn altcp_tcp_dbg_get_tcp_state(conn: &mut AlTcpPcb) -> tcp_state {
    if (conn) {
        let pcb: &mut tcp_pcb = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        if (pcb) {
            return pcb.state;
        }
    }
    return CLOSED;
}

pub const altcp_tcp_functions: AltcpFunctions = AltcpFunctions {
    set_poll: Some(altcp_tcp_set_poll),
    recved: Some(altcp_tcp_recved),
    bind: Some(altcp_tcp_bind),
    connect: Some(altcp_tcp_connect),
    listen: Some(altcp_tcp_listen),
    abort: Some(altcp_tcp_abort),
    close: Some(altcp_tcp_close),
    shutdown: Some(altcp_tcp_shutdown),
    write: Some(altcp_tcp_write),
    output: Some(altcp_tcp_output),
    mss: Some(altcp_tcp_mss),
    sndbuf: Some(altcp_tcp_sndbuf),
    sndqueuelen: Some(altcp_tcp_sndqueuelen),
    nagle_disable: Some(altcp_tcp_nagle_disable),
    nagle_enable: Some(altcp_tcp_nagle_enable),
    nagle_disabled: Some(altcp_tcp_nagle_disabled),
    setprio: Some(altcp_tcp_setprio),
    dealloc: Some(altcp_tcp_dealloc),
    addrinfo: Some(altcp_tcp_get_tcp_addrinfo),
    getip: Some(altcp_tcp_get_ip),
    getport: Some(altcp_tcp_get_port),
    dbg_get_tcp_state: Some(altcp_tcp_dbg_get_tcp_state),
};
