use crate::core::altcp_h::altcp_allocator_t;
use crate::core::altcp_tcp::altcp_tcp_new_ip_type;
use crate::core::err_h::{LwipError, ERR_VAL};

use super::altcp_h::altcp_pcb;

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
 *   replacing "struct tcp_pcb" with "struct altcp_pcb" and prefixing all functions
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
 * * Replace "struct tcp_pcb" with "struct altcp_pcb"
 * * Prefix all called tcp API functions with "altcp_" instead of "tcp_" to link
 *   against the altcp functions
 * * @ref altcp_new (and @ref altcp_new_ip_type/@ref altcp_new_ip6) take
 *   an @ref altcp_allocator_t as an argument, whereas the original tcp API
 *   functions take no arguments.
 * * An @ref altcp_allocator_t allocator is an object that holds a pointer to an
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
 * altcp_allocator_t
 * -----------------
 * An altcp allocator is created by the application by combining an allocator
 * callback function and a corresponding state, e.g.:\code{.c}
 * static const unsigned char cert[] = {0x2D, ... (see mbedTLS doc for how to create this)};
 * struct altcp_tls_config * conf = altcp_tls_create_config_client(cert, sizeof(cert));
 * altcp_allocator_t tls_allocator = {
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
// extern const struct altcp_functions altcp_tcp_functions;

/*
 * For altcp layer implementations only: allocate a new struct altcp_pcb from the pool
 * and zero the memory
 */
pub fn altcp_alloc() -> altcp_pcb {
    // let ret: &mut altcp_pcb = memp_malloc(MEMP_ALTCP_PCB);
    // if (ret != NULL) {
    //   memset(ret, 0, mem::sizeof(altcp_pcb));
    // }
    return altcp_pcb::new();
}

/*
 * For altcp layer implementations only: return a struct altcp_pcb to the pool
 */
pub fn altcp_free(conn: &mut altcp_pcb) {
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
pub fn altcp_new_ip6(allocator: &mut altcp_allocator_t) -> altcp_pcb {
    return altcp_new_ip_type(allocator, IPADDR_TYPE_V6);
}

/*
 * @ingroup altcp
 * altcp_new: @ref altcp_new for IPv4
 */
pub fn altcp_new(allocator: &mut altcp_allocator_t) -> altcp_pcb {
    return altcp_new_ip_type(allocator, IPADDR_TYPE_V4);
}

/*
 * @ingroup altcp
 * altcp_new_ip_type: called by applications to allocate a new pcb with the help of an
 * allocator function.
 *
 * @param allocator allocator function and argument
 * @param ip_type IP version of the pcb (@ref lwip_ip_addr_type)
 * @return a new altcp_pcb or NULL on error
 */
pub fn altcp_new_ip_type(allocator: &mut altcp_allocator_t, ip_type: u8) -> altcp_pcb {
    // let conn: &mut altcp_pcb;
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
pub fn altcp_arg(conn: &mut altcp_pcb, arg: Option<&mut altcp_pcb>) {
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
pub fn altcp_accept(conn: &mut altcp_pcb, accept: altcp_accept_fn) {
    conn.accept = accept;
}

/*
 * @ingroup altcp
 * @see tcp_recv()
 */
pub fn altcp_recv(conn: &mut altcp_pcb, recv: Option<altcp_recv_fn>) {
    conn.recv = recv;
}

/*
 * @ingroup altcp
 * @see tcp_sent()
 */
pub fn altcp_sent(conn: &mut altcp_pcb, sent: Option<altcp_sent_fn>) {
    conn.sent = sent;
}

/*
 * @ingroup altcp
 * @see tcp_poll()
 */
pub fn altcp_poll(conn: &mut altcp_pcb, poll: Option<altcp_poll_fn>, interval: u8) {
    conn.poll = poll;
    conn.pollinterval = interval;
    if conn.fns.set_poll.is_some() {
        conn.fns.set_poll(conn, interval);
    }
}

/*
 * @ingroup altcp
 * @see tcp_err()
 */
pub fn altcp_err(conn: &mut altcp_pcb, err: Option<altcp_err_fn>) {
    if (conn) {
        conn.err = err;
    }
}

/* Generic functions calling the "virtual" ones */

/*
 * @ingroup altcp
 * @see tcp_recved()
 */
pub fn altcp_recved(conn: &mut altcp_pcb, len: u16) {
    // if conn && conn.fns && conn.fns.recved {
    //     conn.fns.recved(conn, len);
    // }
    if conn.fns.recved.is_some() {
        let func = conn.fns.recved.unwrap();
        func(conn, len);
    }
}

/*
 * @ingroup altcp
 * @see tcp_bind()
 */
pub fn altcp_bind(
    conn: &mut altcp_pcb,
    ipaddr: &mut ip_addr_t,
    port: u16,
) -> Result<(), LwipError> {
    // if conn && conn.fns && conn.fns.bind {
    //     return conn.fns.bind(conn, ipaddr, port);
    // }
    if conn.fns.bind.is_some() {
        let func = conn.fns.bind.unwrap();
    }
    return Err(LwipError::new(ERR_VAL, "value error"));
}

/*
 * @ingroup altcp
 * @see tcp_connect()
 */
pub fn altcp_connect(
    conn: &mut altcp_pcb,
    ipaddr: &mut ip_addr_t,
    port: u16,
    connected: altcp_connected_fn,
) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.connect) {
    //     return conn.fns.connect(conn, ipaddr, port, connected);
    // }
    if conn.fns.connect.is_some() {
        conn.fns.connect.unwrap()(conn, ipaddr, port, connected);
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/*
 * @ingroup altcp
 * @see tcp_listen_with_backlog_and_err()
 */
pub fn altcp_listen_with_backlog_and_err(
    conn: &mut altcp_pcb,
    backlog: u8,
    err: &mut err_t,
) -> Option<&mut altcp_pcb> {
    // if (conn && conn.fns && conn.fns.listen) {
    //     return conn.fns.listen(conn, backlog, err);
    // }
    if conn.fns.listen.is_some() {
        conn.fns.listen.unwrap()(conn, backlog, err)
    }
    None
}

/*
 * @ingroup altcp
 * @see tcp_abort()
 */
pub fn altcp_abort(conn: &mut altcp_pcb) {
    // if (conn && conn.fns && conn.fns.abort) {
    //     conn.fns.abort(conn);
    // }
    if conn.fns.abort.is_some() {
        conn.fns.abort.unwrap()(conn)
    }
}

/*
 * @ingroup altcp
 * @see tcp_close()
 */
pub fn altcp_close(conn: &mut altcp_pcb) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.close) {
    //     return conn.fns.close(conn);
    // }
    // return ERR_VAL;
    if conn.fns.close.is_some() {
        conn.fns.close.unwrap()(conn)
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/*
 * @ingroup altcp
 * @see tcp_shutdown()
 */
pub fn altcp_shutdown(conn: &mut altcp_pcb, shut_rx: i32, shut_tx: i32) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.shutdown) {
    //     return conn.fns.shutdown(conn, shut_rx, shut_tx);
    // }
    // return ERR_VAL;
    if conn.fns.shutdown.is_some() {
        conn.fns.shutdown.unwrap()(conn, shut_rx, shut_tx)
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/*
 * @ingroup altcp
 * @see tcp_write()
 */
pub fn altcp_write(
    conn: &mut altcp_pcb,
    dataptr: &mut Vec<u8>,
    len: u16,
    apiflags: u8,
) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.write) {
    //     return conn.fns.write(conn, dataptr, len, apiflags);
    // }
    // return ERR_VAL;
    if conn.fns.write.is_some() {
        return conn.fns.write.unwrap()(conn, dataptr, len, apiflags);
    }
    return Err(LwipError::new(ERR_VAL, "value error"));
}

/*
 * @ingroup altcp
 * @see tcp_output()
 */
pub fn altcp_output(conn: &mut altcp_pcb) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.output) {
    //     return conn.fns.output(conn);
    // }
    // return ERR_VAL;
    if conn.fns.output.is_some() {
        conn.fns.output.unwrap()(conn)
    }
    Err(LwipError::new(ERR_VAL, "value error"))
}

/*
 * @ingroup altcp
 * @see tcp_mss()
 */
pub fn altcp_mss(conn: &mut altcp_pcb) -> u16 {
    // if (conn && conn.fns && conn.fns.mss) {
    //     return conn.fns.mss(conn);
    // }
    // return 0;
    if conn.fns.mss.is_some() {
        return conn.fns.mss.unwrap()(conn);
    }
    0
}

/*
 * @ingroup altcp
 * @see tcp_sndbuf()
 */
pub fn altcp_sndbuf(conn: &mut altcp_pcb) -> u16 {
    // if (conn && conn.fns && conn.fns.sndbuf) {
    //     return conn.fns.sndbuf(conn);
    // }
    if conn.fns.sndbuf.is_some() {
        return conn.fns.sndbuf.unwrap()(conn);
    }
    0
}

/*
 * @ingroup altcp
 * @see tcp_sndqueuelen()
 */
pub fn altcp_sndqueuelen(conn: &mut altcp_pcb) -> u16 {
    // if (conn && conn.fns && conn.fns.sndqueuelen) {
    //     return conn.fns.sndqueuelen(conn);
    // }
    // return 0;
    if conn.fns.sndqueuelen.is_some() {
        return conn.fns.sndqueuelen.unwrap()(conn);
    }
    return 0;
}

pub fn altcp_nagle_disable(conn: &mut altcp_pcb) {
    // if (conn && conn.fns && conn.fns.nagle_disable) {
    //     conn.fns.nagle_disable(conn);
    // }
    if conn.fns.nagle_disable.is_some() {
        conn.fns.nagle_disable.unwrap()(conn)
    }
}

pub fn altcp_nagle_enable(conn: &mut altcp_pcb) {
    // if (conn && conn.fns && conn.fns.nagle_enable) {
    //     conn.fns.nagle_enable(conn);
    // }
    if conn.fns.nagle_enable.is_some() {
        conn.fns.nagle_enable.unwrap()(conn)
    }
}

pub fn altcp_nagle_disabled(conn: &mut altcp_pcb) -> u16 {
    // if (conn && conn.fns && conn.fns.nagle_disabled) {
    //     return conn.fns.nagle_disabled(conn);
    // }
    if conn.fns.nagle_disabled.is_some() {
        return conn.fns.nagle_disabled.unwrap()(conn) as u16;
    }
    return 0;
}

/*w
 * @ingroup altcp
 * @see tcp_setprio()
 */
pub fn altcp_setprio(conn: &mut altcp_pcb, prio: u8) {
    // if (conn && conn.fns && conn.fns.setprio) {
    //     conn.fns.setprio(conn, prio);
    // }
    if conn.fns.setprio.is_some() {
        conn.fns.setprio.unwrap()(conn, prio)
    }
}

pub fn altcp_get_tcp_addrinfo(
    conn: &mut altcp_pcb,
    local: i32,
    addr: &mut ip_addr_t,
    port: &mut u16,
) -> Result<(), LwipError> {
    // if (conn && conn.fns && conn.fns.addrinfo) {
    //     return conn.fns.addrinfo(conn, local, addr, port);
    // }
    // return ERR_VAL;
    if conn.fns.addrinfo.is_some() {
        return conn.fns.addrinfo.unwrap()(conn, local, addr, port);
    }
    return Err(LwipError::new(ERR_VAL, "value error"));
}

pub fn altcp_get_ip(conn: &mut altcp_pcb, local: i32) -> Option<ip_addr_t> {
    // if (conn && conn.fns && conn.fns.getip) {
    //     return conn.fns.getip(conn, local);
    // }
    // return NULL;
    if conn.fns.getip.is_some() {
        return conn.fns.getip.unwrap()(conn, local);
    }
    return None;
}

pub fn altcp_get_port(conn: &mut altcp_pcb, local: i32) -> i32 {
    // if (conn && conn.fns && conn.fns.getport) {
    //     return conn.fns.getport(conn, local);
    // }
    if conn.fns.getport.is_some() {
        return conn.fns.getport.unwrap()(conn, local) as i32;
    }
    return 0;
}

pub fn altcp_dbg_get_tcp_state(conn: &mut altcp_pcb) {
    if (conn && conn.fns && conn.fns.dbg_get_tcp_state) {
        return conn.fns.dbg_get_tcp_state(conn);
    }
    return CLOSED;
}

/* Default implementations for the "virtual" functions */

pub fn altcp_default_set_poll(conn: &mut altcp_pcb, interval: u8) {
    if (conn && conn.inner_conn) {
        altcp_poll(conn.inner_conn, conn.poll, interval);
    }
}

pub fn altcp_default_recved(conn: &mut altcp_pcb, len: u16) {
    if (conn && conn.inner_conn) {
        altcp_recved(conn.inner_conn, len);
    }
}

pub fn altcp_default_bind(conn: &mut altcp_pcb, ipaddr: &mut ip_addr_t, port: u16) {
    if (conn && conn.inner_conn) {
        return altcp_bind(conn.inner_conn, ipaddr, port);
    }
    return ERR_VAL;
}

pub fn altcp_default_shutdown(conn: &mut altcp_pcb, shut_rx: i32, shut_tx: i32) {
    if (conn) {
        if (shut_rx && shut_tx && conn.fns && conn.fns.close) {
            /* default shutdown for both sides is close */
            return conn.fns.close(conn);
        }
        if (conn.inner_conn) {
            return altcp_shutdown(conn.inner_conn, shut_rx, shut_tx);
        }
    }
    return ERR_VAL;
}

pub fn altcp_default_write(
    conn: &mut altcp_pcb,
    dataptr: &Vec<u8>,
    len: u16,
    apiflags: u8,
) -> Result<(), LwipError> {
    if (conn && conn.inner_conn) {
        return altcp_write(conn.inner_conn, dataptr, len, apiflags);
    }
    return Err(LwipError::new(ERR_VAL, "invalid value"));
}

pub fn altcp_default_output(conn: &mut altcp_pcb) {
    if (conn && conn.inner_conn) {
        return altcp_output(conn.inner_conn);
    }
    return ERR_VAL;
}

pub fn altcp_default_mss(conn: &mut altcp_pcb) -> u16 {
    if (conn && conn.inner_conn) {
        return altcp_mss(conn.inner_conn);
    }
    return 0;
}

pub fn altcp_default_sndbuf(conn: &mut altcp_pcb) -> u16 {
    if conn && conn.inner_conn {
        return altcp_sndbuf(conn.inner_conn);
    }
    return 0;
}

pub fn altcp_default_sndqueuelen(conn: &mut altcp_pcb) -> u16 {
    if conn && conn.inner_conn {
        return altcp_sndqueuelen(conn.inner_conn);
    }
    return 0;
}

pub fn altcp_default_nagle_disable(conn: &mut altcp_pcb) {
    if (conn && conn.inner_conn) {
        altcp_nagle_disable(conn.inner_conn);
    }
}

pub fn altcp_default_nagle_enable(conn: &mut altcp_pcb) {
    if (conn && conn.inner_conn) {
        altcp_nagle_enable(conn.inner_conn);
    }
}

pub fn altcp_default_nagle_disabled(conn: &mut altcp_pcb) {
    if (conn && conn.inner_conn) {
        return altcp_nagle_disabled(conn.inner_conn);
    }
    return 0;
}

pub fn altcp_default_setprio(conn: &mut altcp_pcb, prio: u8) {
    if (conn && conn.inner_conn) {
        altcp_setprio(conn.inner_conn, prio);
    }
}

pub fn altcp_default_dealloc(conn: &mut altcp_pcb) {
    
    /* nothing to do */
}

pub fn altcp_default_get_tcp_addrinfo(
    conn: &mut altcp_pcb,
    local: i32,
    addr: &mut ip_addr_t,
    port: &mut u16,
) {
    if (conn && conn.inner_conn) {
        return altcp_get_tcp_addrinfo(conn.inner_conn, local, addr, port);
    }
    return ERR_VAL;
}

pub fn altcp_default_get_ip(conn: &mut altcp_pcb, local: i32) {
    if (conn && conn.inner_conn) {
        return altcp_get_ip(conn.inner_conn, local);
    }
    return NULL;
}

pub fn altcp_default_get_port(conn: &mut altcp_pcb, local: i32) {
    if (conn && conn.inner_conn) {
        return altcp_get_port(conn.inner_conn, local);
    }
    return 0;
}

pub fn altcp_default_dbg_get_tcp_state(conn: &mut altcp_pcb) {
    if (conn && conn.inner_conn) {
        return altcp_dbg_get_tcp_state(conn.inner_conn);
    }
    return CLOSED;
}
