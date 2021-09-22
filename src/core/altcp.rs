use crate::core::altcp_h::AltcpAllocatorT;
use crate::core::altcp_tcp::{altcp_tcp_new_ip_type, altcp_tcp_mss, altcp_tcp_sndbuf, altcp_tcp_sndqueuelen, altcp_tcp_nagle_disable, altcp_tcp_setprio, altcp_tcp_get_tcp_addrinfo, altcp_tcp_get_ip, altcp_tcp_nagle_disabled, altcp_tcp_get_port, altcp_tcp_dbg_get_tcp_state, altcp_tcp_nagle_enable, altcp_tcp_output};
use crate::core::err_h::{LwipError, ERR_VAL};

use super::altcp_h::AlTcpContext;
use crate::core::ip_addr_h::LwipIpAddrType;
use crate::core::ip_addr_h::LwipIpAddrType::{IpaddrTypeV4, IpaddrTypeV6};
use crate::defines::LwipAddr;
use crate::core::tcpbase_h::{TcpWriteFlags, TcpState};

/*
 * @file
 * @defgroup altcp Application layered TCP Functions
 * @ingroup altcp_api
 *
 * This file contains the common functions for altcp to work.
 * For more details see @ref altcp_api.
 */

/*
 * @defgroup altcp_api Application layered TCP Introduction
 * @ingroup callbackstyle_api
 *
 * Overview
 * --------
 * altcp (application layered TCP connection API; to be used from TCPIP thread)
 * is an abstraction layer that prevents applications linking hard against the
 * @ref tcp.h functions while providing the same functionality. It is used to
 * e.g. add SSL/TLS (see LWIP_ALTCP_TLS) or proxy-connect support to an application
 * written for the tcp callback API without that application knowing the
 * protocol details.
 *
 * * This interface mimics the tcp callback API to the application while preventing
 *   direct linking (much like virtual functions).
 * * This way, an application can make use of other application layer protocols
 *   on top of TCP without knowing the details (e.g. TLS, proxy connection).
 * * This is achieved by simply including "lwip/altcp.h" instead of "lwip/tcp.h",
 *   replacing "struct tcp_pcb" with "struct AltcpPcb" and prefixing all functions
 *   with "altcp_" instead of "tcp_".
 *
 * With altcp support disabled (LWIP_ALTCP==0), applications written against the
 * altcp API can still be compiled but are directly linked against the tcp.h
 * callback API and then cannot use layered protocols. To minimize code changes
 * in this case, the use of altcp_allocators is strongly suggested.
 *
 * Usage
 * -----
 * To make use of this API from an existing tcp raw API application:
 * * Include "lwip/altcp.h" instead of "lwip/tcp.h"
 * * Replace "struct tcp_pcb" with "struct AltcpPcb"
 * * Prefix all called tcp API functions with "altcp_" instead of "tcp_" to link
 *   against the altcp functions
 * * @ref altcp_new (and @ref altcp_new_ip_type/@ref altcp_new_ip6) take
 *   an @ref AltcpAllocatorT as an argument, whereas the original tcp API
 *   functions take no arguments.
 * * An @ref AltcpAllocatorT allocator is an object that holds a pointer to an
 *   allocator object and a corresponding state (e.g. for TLS, the corresponding
 *   state may hold certificates or keys). This way, the application does not
 *   even need to know if it uses TLS or pure TCP, this is handled at runtime
 *   by passing a specific allocator.
 * * An application can alternatively bind hard to the altcp_tls API by calling
 *   @ref altcp_tls_new or @ref altcp_tls_wrap.
 * * The TLS layer is not directly implemented by lwIP, but a port to mbedTLS is
 *   provided.
 * * Another altcp layer is proxy-connect to use TLS behind a HTTP proxy (see
 *   @ref altcp_proxyconnect.h)
 *
 * AltcpAllocatorT
 * -----------------
 * An altcp allocator is created by the application by combining an allocator
 * callback function and a corresponding state, e.g.:\code{.c}
 * static const  cert: [u8;] = {0x2D, ... (see mbedTLS doc for how to create this)};
 * struct altcp_tls_config * conf = altcp_tls_create_config_client(cert, sizeof(cert));
 * AltcpAllocatorT tls_allocator = {
 *   altcp_tls_alloc, conf
 * };
 * \endcode
 *
 *
 * struct altcp_tls_config
 * -----------------------
 * The struct altcp_tls_config holds state that is needed to create new TLS client
 * or server connections (e.g. certificates and private keys).
 *
 * It is not defined by lwIP itself but by the TLS port (e.g. altcp_tls to mbedTLS
 * adaption). However, the parameters used to create it are defined in @ref
 * altcp_tls.h (see @ref altcp_tls_create_config_server_privkey_cert for servers
 * and @ref altcp_tls_create_config_client/@ref altcp_tls_create_config_client_2wayauth
 * for clients).
 *
 * For mbedTLS, ensure that certificates can be parsed by 'mbedtls_x509_crt_parse()' and
 * private keys can be parsed by 'mbedtls_pk_parse_key()'.
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
// extern const struct AltcpFunctions altcp_tcp_functions;

/*
 * For altcp layer implementations only: allocate a new struct AltcpPcb from the pool
 * and zero the memory
 */
pub fn altcp_alloc() -> AlTcpContext {
    // let ret: &mut AltcpPcb = memp_malloc(MEMP_ALTCP_PCB);
    // if (ret != NULL) {
    //   memset(ret, 0, mem::sizeof(AltcpPcb));
    // }
    return AlTcpContext::new();
}

/*
 * For altcp layer implementations only: return a struct AltcpPcb to the pool
 */
pub fn altcp_free(conn: &mut AlTcpContext) {
    // if (conn) {
    //     if (conn.fns && conn.fns.dealloc) {
    //         conn.fns.dealloc(conn);
    //     }
    //     memp_free(MEMP_ALTCP_PCB, conn);
    // }
    unimplemented!()
}

/*
 * @ingroup altcp
 * altcp_new_ip6: @ref altcp_new for IPv6
 */
pub fn altcp_new_ip6(allocator: &mut AltcpAllocatorT) -> AlTcpContext {
    return altcp_new_ip_type(allocator, IpaddrTypeV6);
}

/*
 * @ingroup altcp
 * altcp_new: @ref altcp_new for IPv4
 */
pub fn altcp_new(allocator: &mut AltcpAllocatorT) -> AlTcpContext {
    return altcp_new_ip_type(allocator, IpaddrTypeV4);
}

/*
 * @ingroup altcp
 * altcp_new_ip_type: called by applications to allocate a new pcb with the help of an
 * allocator function.
 *
 * @param allocator allocator function and argument
 * @param ip_type IP version of the pcb (@ref LwipIpAddrType)
 * @return a new AltcpPcb or NULL on error
 */
pub fn altcp_new_ip_type(allocator: &mut AltcpAllocatorT, ip_type: LwipIpAddrType) -> AlTcpContext {
    // let conn: &mut AltcpPcb;
    // if (allocator == NULL) {
    //     /* no allocator given, create a simple TCP connection */
    //     return altcp_tcp_new_ip_type(ip_type);
    // }
    // if (allocator.alloc == NULL) {
    //     /* illegal allocator */
    //     return NULL;
    // }
    // conn = allocator.alloc(allocator.arg, ip_type);
    // if (conn == NULL) {
    //     /* allocation failed */
    //     return NULL;
    // }
    // return conn;
    altcp_alloc()
}

/*
 * @ingroup altcp
 * @see tcp_arg()
 */
pub fn altcp_arg(conn: &mut AlTcpContext, arg: Option<&mut AlTcpContext>) {
    if arg.is_some() {
        conn.arg = Some(arg.unwrap().clone())
    } else {
        conn.arg = None
    }
}

/*
 * @ingroup altcp
 * @see tcp_accept()
 */
pub fn altcp_accept(conn: &mut AlTcpContext, accept: altcp_accept_fn) {
    conn.accept = accept;
}

/*
 * @ingroup altcp
 * @see tcp_recv()
 */
pub fn altcp_recv(conn: &mut AlTcpContext, recv: Option<altcp_recv_fn>) {
    conn.recv = recv;
}

/*
 * @ingroup altcp
 * @see tcp_sent()
 */
pub fn altcp_sent(conn: &mut AlTcpContext, sent: Option<altcp_sent_fn>) {
    conn.sent = sent;
}

/*
 * @ingroup altcp
 * @see tcp_poll()
 */
pub fn altcp_poll(conn: &mut AlTcpContext, poll: Option<altcp_poll_fn>, interval: u64) {
    conn.poll = poll;
    conn.pollinterval = interval;
    if conn.functions.set_poll.is_some() {
        conn.functions.set_poll(conn, interval);
    }
}

/*
 * @ingroup altcp
 * @see tcp_err()
 */
pub fn altcp_err(conn: &mut AlTcpContext, err: Option<altcp_err_fn>) {
    if (conn) {
        conn.err = err;
    }
}

/* Generic functions calling the "virtual" ones */

/*
 * @ingroup altcp
 * @see tcp_recved()
 */
pub fn altcp_recved(conn: &mut AlTcpContext, len: usize) {
    // if conn && conn.fns && conn.fns.recved {
    //     conn.fns.recved(conn, len);
    // }
    if conn.functions.recved.is_some() {
        let func = conn.functions.recved.unwrap();
        func(conn, len);
    }
}

/*
 * @ingroup altcp
 * @see tcp_bind()
 */
pub fn altcp_bind(conn: &mut AlTcpContext, ipaddr: &mut LwipAddr, port: u16) -> Result<(), LwipError> {
    // if conn && conn.fns && conn.fns.bind {
    //     return conn.fns.bind(conn, ipaddr, port);
    // }
    if conn.functions.bind.is_some() {
        let func = conn.functions.bind.unwrap();
    }
    return Err(LwipError::new(ERR_VAL, "value error"));
}

/*
 * @ingroup altcp
 * @see tcp_connect()
 */
pub fn altcp_connect(
    conn: &mut AlTcpContext,
    ipaddr: &mut LwipAddr,
    port: u16,
    connected: AlTcpConnectedFunc,
) -> Result<(), LwipError> {
    match conn.functions.connect {
        Some(x) => x(conn, ipaddr, port, connected),
        None => Err(LwipError::new(ERR_VAL, "no connect function to call"))
    }
}

/*
 * @ingroup altcp
 * @see tcp_listen_with_backlog_and_err()
 */
pub fn altcp_listen_with_backlog_and_err(
    conn: &mut AlTcpContext,
    backlog: u8,
    err: &mut err_t,
) ->Result<(), LwipError> {
    if conn.functions.listen.is_some() {
        conn.functions.listen.unwrap()(conn, backlog, err)
    }
    Err(LwipError::new(ERR_VAL, ""))
}

/*
 * @ingroup altcp
 * @see tcp_abort()
 */
pub fn altcp_abort(conn: &mut AlTcpContext) {
    // if (conn && conn.fns && conn.fns.abort) {
    //     conn.fns.abort(conn);
    // }
    if conn.functions.abort.is_some() {
        conn.functions.abort.unwrap()(conn)
    }
}

/*
 * @ingroup altcp
 * @see tcp_close()
 */
pub fn altcp_close(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.close) {
    //     return conn.fns.close(conn);
    // }
    // return ERR_VAL;
    if conn.functions.close.is_some() {
        conn.functions.close.unwrap()(conn)
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/*
 * @ingroup altcp
 * @see tcp_shutdown()
 */
pub fn altcp_shutdown(conn: &mut AlTcpContext, shut_rx: i32, shut_tx: i32) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.shutdown) {
    //     return conn.fns.shutdown(conn, shut_rx, shut_tx);
    // }
    // return ERR_VAL;
    if conn.functions.shutdown.is_some() {
        conn.functions.shutdown.unwrap()(conn, shut_rx, shut_tx)
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/*
 * @ingroup altcp
 * @see tcp_write()
 */
pub fn altcp_write(
    conn: &mut AlTcpContext,
    dataptr: &[u8],
    len: usize,
    apiflags: TcpWriteFlags,
) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.write) {
    //     return conn.fns.write(conn, dataptr, len, apiflags);
    // }
    // return ERR_VAL;
    if conn.functions.write.is_some() {
        return conn.functions.write.unwrap()(conn, dataptr, len, apiflags);
    }
    return Err(LwipError::new(ERR_VAL, "value error"));
}

/*
 * @ingroup altcp
 * @see tcp_output()
 */
pub fn altcp_output(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    altcp_tcp_output(conn)
}

/*
 * @ingroup altcp
 * @see tcp_mss()
 */
pub fn altcp_mss(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    altcp_tcp_mss(conn)
}

/*
 * @ingroup altcp
 * @see tcp_sndbuf()
 */
pub fn altcp_sndbuf(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    altcp_tcp_sndbuf(conn)
}

/*
 * @ingroup altcp
 * @see tcp_sndqueuelen()
 */
pub fn altcp_sndqueuelen(conn: &mut AlTcpContext) -> Result<(), LwipError> {
    altcp_tcp_sndqueuelen(conn)
}

pub fn altcp_nagle_disable(conn: &mut AlTcpContext) {
    altcp_tcp_nagle_disable(conn)
}

pub fn altcp_nagle_enable(conn: &mut AlTcpContext) {
    altcp_tcp_nagle_enable(conn)
}

pub fn altcp_nagle_disabled(conn: &mut AlTcpContext) -> Result<bool, LwipError> {
    altcp_tcp_nagle_disabled(conn)
}

/*w
 * @ingroup altcp
 * @see tcp_setprio()
 */
pub fn altcp_setprio(conn: &mut AlTcpContext, prio: u8) -> Result<(), LwipError> {
    altcp_tcp_setprio(conn, prio)
}

pub fn altcp_get_tcp_addrinfo(
    conn: &mut AlTcpContext,
    local: i32,
    addr: &mut LwipAddr,
    port: &mut u16,
) -> Result<(), LwipError> {
    altcp_tcp_get_tcp_addrinfo(conn, local, addr, port)
}

pub fn altcp_get_ip(conn: &mut AlTcpContext, local: bool) -> Result<LwipAddr, LwipError> {
    altcp_tcp_get_ip(conn, local)
}

pub fn altcp_get_port(conn: &mut AlTcpContext, local: bool) -> u16 {
    altcp_tcp_get_port(conn, local)
}

pub fn altcp_dbg_get_tcp_state(conn: &mut AlTcpContext) -> TcpState {
    altcp_tcp_dbg_get_tcp_state(conn)
}

/* Default implementations for the "virtual" functions */

pub fn altcp_default_set_poll(conn: &mut AlTcpContext, interval: u64) {
    altcp_poll(conn.inner_conn, conn.poll, interval)
}

pub fn altcp_default_recved(conn: &mut AlTcpContext, len: usize) {
    altcp_recved(conn.inner_conn, len)
}

pub fn altcp_default_bind(
    conn: &mut AlTcpContext,
    ipaddr: &mut LwipAddr,
    port: u16,
) -> Result<(), LwipError> {
    altcp_bind(conn.inner_conn, ipaddr, port)
}

pub fn altcp_default_shutdown(
    conn: &mut AlTcpContext,
    shut_tx: i32,
    shut_rx: i32,
) -> Result<(), LwipError> {
    altcp_shutdown(conn.inner_conn, shut_rx, shut_tx)
}

pub fn altcp_default_write(
    conn: &mut Vec<u8>,
    dataptr: &mut Vec<u8>,
    len: usize,
    flags: TcpWriteFlags,
) -> Result<(), LwipError> {
    altcp_write(conn.inner_conn, dataptr, len, flags)
}

pub fn altcp_default_output(conn: &mut Vec<u8>) -> Result<(), LwipError> {
    altcp_output(conn.inner_conn)
}

pub fn altcp_default_mss(conn: &mut Vec<u8>) -> Result<(), LwipError> {
    altcp_mss(conn.inner_conn)
}

pub fn altcp_default_sndbuf(conn: &mut Vec<u8>) -> Result<(), LwipError> {
    altcp_sndbuf(conn.inner_conn)
}

pub fn altcp_default_sndqueuelen(conn: &mut Vec<u8>) -> Result<(), LwipError> {
    altcp_sndqueuelen(conn.inner_conn)
}

pub fn altcp_default_nagle_disable(conn: &mut Vec<u8>) {
    altcp_nagle_disable(conn.inner_conn)
}

pub fn altcp_default_nagle_enable(conn: &mut Vec<u8>) {
    altcp_nagle_enable(conn.inner_conn)
}

pub fn altcp_default_nagle_disabled(conn: &mut Vec<u8>) -> Result<bool, LwipError> {
    altcp_nagle_disabled(conn.inner_conn)
}

pub fn altcp_default_setprio(conn: &mut Vec<u8>, prio: u8) -> Result<(), LwipError> {
    altcp_setprio(conn.inner_conn, prio)
}

pub fn altcp_default_dealloc(conn: &mut Vec<u8>) {
    unimplemented!()
    /* nothing to do */
}

pub fn altcp_default_get_tcp_addrinfo(
    conn: &mut Vec<u8>,
    local: i32,
    addr: &mut LwipAddr,
    port: &mut u16,
) -> Result<(), LwipError> {
    altcp_get_tcp_addrinfo(conn.inner_conn, local, addr, port)
}

pub fn altcp_default_get_ip(conn: &mut Vec<u8>, local: bool) ->Result<LwipAddr, LwipError> {
    altcp_get_ip(conn.inner_conn, local)
}

pub fn altcp_default_get_port(conn: &mut Vec<u8>, local: bool) -> u16 {
    altcp_get_port(conn.inner_conn, local)
}

pub fn altcp_default_dbg_get_tcp_state(conn: &mut Vec<u8>) -> TcpState {
    altcp_dbg_get_tcp_state(conn.inner_conn)
}
