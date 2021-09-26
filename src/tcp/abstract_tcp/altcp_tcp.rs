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

// pub fn altcp_tcp_setup(conn: &mut AltcpPcb, tpcb: &mut TcpContext);

/* callback functions for TCP */
use crate::core::altcp::{altcp_nagle_disable, altcp_sndqueuelen, altcp_alloc};
use crate::core::altcp_h::AlTcpContext;
use lwip_rs::core::err_h::{LwipError, ERR_VAL};
use lwip_rs::core::tcp2::{set_tcp_accept_fn, set_tcp_poll_fn, tcp_connect, tcp_listen_with_backlog_and_err, tcp_setprio, tcp_shutdown, tcp_recv, tcp_recved, tcp_bind};
use lwip_rs::core::tcp2_h::TcpContext;
use lwip_rs::core::tcp_out::{tcp_output, tcp_write};
use lwip_rs::core::tcpbase_h::TcpState;
use lwip_rs::defines::LwipAddr;
use lwip_rs::core::pbuf_h::PacketBuffer;
use lwip_rs::core::pbuf::pbuf_free;

pub fn altcp_tcp_accept(arg: &mut Vec<u8>, new_tpcb: &mut TcpContext, err: err_t) -> err_t {
    // TODO: let listen_conn: &mut AltcpPcb = arg;

    if (listen_conn && listen_conn.accept) {
        /* create a new altcp_conn to pass to the next 'accept' callback */
        let new_conn: &mut AlTcpContext = altcp_alloc();
        if (new_conn == None) {
            return ERR_MEM;
        }
        altcp_tcp_setup(new_conn, new_tpcb);
        return listen_conn.accept(listen_conn.arg, new_conn, err);
    }
    return ERR_ARG;
}

pub fn altcp_tcp_connected(arg: &mut Vec<u8>, tpcb: &mut TcpContext, err: err_t) -> err_t {
    // let conn: &mut AltcpPcb = arg;
    if (conn) {
        ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb);
        if (conn.connected) {
            return conn.connected(conn.arg, conn, err);
        }
    }
    return Ok(());
}

pub fn altcp_tcp_recv(arg: &mut AlTcpContext, tcp_ctx: &mut TcpContext, pkt_buf: &mut PacketBuffer) -> err_t {
    ALTCP_TCP_ASSERT_CONN_PCB(conn, tcp_ctx);
    tcp_recv(tcp_ctx, altcp_tcp_recved)

    if (conn.recv) {
        return conn.recv(conn.arg, conn, pkt_buf, err);
    }
    pbuf_free(pkt_buf);
    return Ok(());
}

pub fn altcp_tcp_sent(arg: &mut Vec<u8>, tpcb: &mut TcpContext, len: usize) -> err_t {
    // TODO: conn: &mut AltcpPcb = arg;
    if (conn) {
        ALTCP_TCP_ASSERT_CONN_PCB(conn, tpcb);
        if (conn.sent) {
            return conn.sent(conn.arg, conn, len);
        }
    }
    return Ok(());
}

pub fn altcp_tcp_poll(arg: &mut Vec<u8>, tpcb: &mut TcpContext) -> err_t {
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

pub fn altcp_tcp_remove_callbacks(tpcb: &mut TcpContext) {
    tcp_arg(tpcb, None);
    tcp_recv(tpcb, None);
    tcp_sent(tpcb, None);
    tcp_err(tpcb, None);
    set_tcp_poll_fn(tpcb, None, tpcb.pollinterval);
}

pub fn altcp_tcp_setup_callbacks(conn: &mut AlTcpContext, tpcb: &mut TcpContext) {
    tcp_arg(tpcb, conn);
    tcp_recv(tpcb, altcp_tcp_recv);
    tcp_sent(tpcb, altcp_tcp_sent);
    tcp_err(tpcb, altcp_tcp_err);
    /* tcp_poll is set when interval is set by application */
    /* listen is set totally different :-) */
}

pub fn altcp_tcp_setup(conn: &mut AlTcpContext, tpcb: &mut TcpContext) {
    altcp_tcp_setup_callbacks(conn, tpcb);
    conn.state = tpcb;
    conn.fns = &altcp_tcp_functions;
}

pub fn altcp_tcp_new_ip_type(ip_type: u8) -> Result<AlTcpContext, LwipError> {
    /* Allocate the tcp pcb first to invoke the priority handling code
    if we're out of pcbs */
    let tpcb: &mut TcpContext = tcp_new_ip_type(ip_type);
    if (tpcb != None) {
        let ret: &mut AlTcpContext = altcp_alloc();
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
pub fn altcp_tcp_alloc(arg: &mut Vec<u8>, ip_type: u8) -> &mut AlTcpContext {
    return altcp_tcp_new_ip_type(ip_type);
}

pub fn altcp_tcp_wrap(tpcb: &mut TcpContext) -> &mut AlTcpContext {
    if (tpcb != None) {
        let ret: &mut AlTcpContext = altcp_alloc();
        if (ret != None) {
            altcp_tcp_setup(ret, tpcb);
            return ret;
        }
    }
    return None;
}

/* "virtual" functions calling into tcp */
pub fn altcp_tcp_set_poll(conn: &mut AlTcpContext, interval: u64) -> Result<(), LwipError> {
    set_tcp_poll_fn(&mut conn.tcp_ctx, altcp_tcp_poll, interval)
}

pub fn altcp_tcp_recved(conn: &mut AlTcpContext, len: usize) -> Result<(), LwipError> {
    tcp_recved(&mut conn.tcp_ctx, len)
}

pub fn altcp_tcp_bind(conn: &mut AlTcpContext, ipaddr: &mut LwipAddr, port: u16) -> Result<(), LwipError> {
    // ALTCP_TCP_ASSERT_CONN(conn);
    tcp_bind(&mut conn.tcp_ctx, ipaddr, port)
}

pub fn altcp_tcp_connect(
    conn: &mut AlTcpContext,
    ipaddr: &LwipAddr,
    port: u16,
    connected: altcp_connected_fn,
) -> err_t {
    let pcb: &mut TcpContext;
    if (conn == None) {
        return ERR_VAL;
    }
    ALTCP_TCP_ASSERT_CONN(conn);
    conn.connected = connected;
    pcb = conn.state;
    return tcp_connect(pcb, ipaddr, port, altcp_tcp_connected);
}

pub fn altcp_tcp_listen(conn: &mut AlTcpContext, backlog: u8) -> Result<(), LwipError> {
    let mut tcp_listen_ctx = match tcp_listen_with_backlog_and_err(&mut conn.tcp_ctx, backlog) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    conn.tcp_ctx = tcp_listen_ctx;
    set_tcp_accept_fn(&mut tcp_listen_ctx, altcp_tcp_accept);
    Ok(())
}

pub fn altcp_tcp_abort(conn: &mut AlTcpContext) {
    if conn != None {
        let pcb: &mut TcpContext = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        if (pcb) {
            tcp_abort(pcb);
        }
    }
}

pub fn altcp_tcp_close(conn: &mut AlTcpContext) -> err_t {
    let pcb: &mut TcpContext;
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
            set_tcp_poll_fn(pcb, oldpoll, pcb.pollinterval);
            return err;
        }
        conn.state = None; /* unsafe to reference pcb after tcp_close(). */
    }
    altcp_free(conn);
    return Ok(());
}

pub fn altcp_tcp_shutdown(conn: &mut AlTcpContext, shut_rx: bool, shut_tx: bool) -> Result<(), LwipError> {
    return tcp_shutdown(&mut conn.tcp_ctx, shut_rx, shut_tx);
}

pub fn altcp_tcp_write(
    conn: &mut AlTcpContext,
    dataptr: &mut Vec<u8>,
    len: usize,
    apiflags: u8,
) -> Result<(), LwipError> {
    tcp_write(&mut conn.tcp_ctx, dataptr, len, apiflags)
}

pub fn altcp_tcp_output(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    tcp_output(&mut conn.tcp_ctx)
}

pub fn altcp_tcp_mss(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    tcp_mss(&mut conn.tcp_ctx)
}

pub fn altcp_tcp_sndbuf(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    tcp_sndbuf(&mut conn.tcp_ctx)
}

pub fn altcp_tcp_sndqueuelen(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    altcp_sndqueuelen(conn)
}

pub fn altcp_tcp_nagle_disable(conn: &mut AlTcpContext) {
    altcp_nagle_disable(conn)
}

pub fn altcp_tcp_nagle_enable(conn: &mut AlTcpContext) {
    if (conn && conn.state) {
        let pcb: &mut TcpContext = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        tcp_nagle_enable(pcb);
    }
}

pub fn altcp_tcp_nagle_disabled(conn: &mut AlTcpContext) -> Result<bool, LwipError> {
    tcp_nagle_disabled(&mut conn.tcp_ctx)
}

pub fn altcp_tcp_setprio(conn: &mut AlTcpContext, prio: u8) -> Result<(), LwipError> {
    tcp_setprio(&mut conn.tcp_ctx, prio)
}

pub fn altcp_tcp_dealloc(conn: &mut AlTcpContext) {
    ALTCP_TCP_ASSERT_CONN(conn);
    /* no private state to clean up */
}

pub fn altcp_tcp_get_tcp_addrinfo(
    conn: &mut AlTcpContext,
    local: i32,
    addr: &mut LwipAddr,
    port: &mut u16,
) -> err_t {
    if (conn) {
        let pcb: &mut TcpContext = conn.state;
        ALTCP_TCP_ASSERT_CONN(conn);
        return tcp_tcp_get_tcp_addrinfo(pcb, local, addr, port);
    }
    return ERR_VAL;
}

pub fn altcp_tcp_get_ip(conn: &mut AlTcpContext, local: bool) -> Result<LwipAddr, LwipError> {
    return if local {
        Ok(conn.tcp_ctx.local_ip)
    } else {
        Ok(conn.tcp_ctx.local_ip)
    };
}

pub fn altcp_tcp_get_port(conn: &mut AlTcpContext, local: bool) -> u16 {
    return if local {
        conn.tcp_ctx.local_port
    } else {
        conn.tcp_ctx.remote_port
    };
}

pub fn altcp_tcp_dbg_get_tcp_state(conn: &mut AlTcpContext) -> TcpState {
    conn.tcp_ctx.state.clone()
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
