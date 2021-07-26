/**
 * @file
 * Application layered TCP connection API (to be used from TCPIP thread)\n
 *
 * This file contains the generic API.
 * For more details see @ref altcp_api.
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

// typedef err_t (*altcp_accept_fn)(arg: &mut Vec<u8>, new_conn: &mut altcp_pcb, err: err_t);
type altcp_accept_fn = fn(arg: &mut Vec<u8>, new_conn: &mut altcp_pcb, err: err_t) -> err_t;
// typedef err_t (*altcp_connected_fn)(arg: &mut Vec<u8>, conn: &mut altcp_pcb, err: err_t);
type altcp_connected_fn = fn(arg: &mut Vec<u8>, conn: &mut altcp_pcb, err: err_t) -> err_t;
// typedef err_t (*altcp_recv_fn)(arg: &mut Vec<u8>, conn: &mut altcp_pcb, p: &mut pbuf, err: err_t);
type altcp_recv_fn = fn(arg: &mut Vec<u8>, conn: &mut altcp_pcb, p: &mut pbuf, err: err_t) -> err_t;
// typedef err_t (*altcp_sent_fn)(arg: &mut Vec<u8>, conn: &mut altcp_pcb, len: u16);
type altcp_sent_fn = fn(arg: &mut Vec<u8>, conn: &mut altcp_pcb, len: u16) -> err_t;
// typedef err_t (*altcp_poll_fn)(arg: &mut Vec<u8>, conn: &mut altcp_pcb);
type altcp_poll_fn = fn(arg: &mut Vec<u8>, conn: &mut altcp_pcb) -> err_t;
// typedef void  (*altcp_err_fn)(arg: &mut Vec<u8>, err: err_t);
type altcp_err_fn = fn(arg: &mut Vec<u8>, err: err_t);
// typedef struct altcp_pcb* (*altcp_new_fn)(arg: &mut Vec<u8>, ip_type: u8);
type altcp_new_fn = fn(arg: &mut Vec<u8>, ip_type: u8) -> &mut altcp_pcb;
// typedef void (*altcp_set_poll_fn)(conn: &mut altcp_pcb, interval: u8);
type altcp_set_poll_fn = fn(conn: &mut altcp_pcb, u8: interval);
// typedef void (*altcp_recved_fn)(conn: &mut altcp_pcb, len: u16);
type altcp_recved_fn = fn(conn: &mut altcp_pcb, len: u16);
// typedef err_t (*altcp_bind_fn)(conn: &mut altcp_pcb, const ipaddr: &mut ip_addr_t, port: u16);
type altcp_bind_fn = fn(conn: &mut altcp_pcb, ip_addr: &ip_addr_t, port: u16) -> err_t;
// typedef err_t (*altcp_connect_fn)(conn: &mut altcp_pcb, const ipaddr: &mut ip_addr_t, port: u16, altcp_connected_fn connected);
type altcp_connect_fn =
    fn(conn: &mut altcp_pcb, ipaddr: &ip_addr_t, port: u16, connected: altcp_connected_fn) -> err_t;
// typedef struct altcp_pcb *(*altcp_listen_fn)(conn: &mut altcp_pcb, backlog: u8, err: &mut err_t);
type altcp_listen_fn = fn(conn: &mut altcp_pcb, backlog: u8, err: &mut err_t) -> &mut altcp_pcb;
// typedef void  (*altcp_abort_fn)(conn: &mut altcp_pcb);
type altcp_abort_fn = fn(conn: &mut altcp_pcb);
// typedef err_t (*altcp_close_fn)(conn: &mut altcp_pcb);
type altcp_close_fn = fn(conn: &mut altcp_pcb) -> err_t;
// typedef err_t (*altcp_shutdown_fn)(conn: &mut altcp_pcb, shut_rx: int, shut_tx: int);
type altcp_shutdown_fn = fn(conn: &mut altcp_pcb, shut_rx: i32, shut_Tx: i32) -> err_t;
// typedef err_t (*altcp_write_fn)(conn: &mut altcp_pcb, dataptr: &Vec<u8>, len: u16, apiflags: u8);
type altcp_write_fn =
    fn(conn: &mut altcp_pcb, dataptr: &mut Vec<u8>, len: u16, apiflags: u8) -> err_t;
// typedef err_t (*altcp_output_fn)(conn: &mut altcp_pcb);
type altcp_output_fn = fn(conn: &mut altcp_pcb) -> err_t;
// typedef u16 (*altcp_mss_fn)(conn: &mut altcp_pcb);
type altcp_mss_fn = fn(conn: &mut altcp_pcb) -> u16;
// typedef u16 (*altcp_sndbuf_fn)(conn: &mut altcp_pcb);
type altcp_sndbuf_fn = fn(conn: &mut altcp_pcb) -> u16;
// typedef u16 (*altcp_sndqueuelen_fn)(conn: &mut altcp_pcb);
type altcp_sndqueuelen_fn = fn(conn: &mut altcp_pcb) -> u16;
// typedef void  (*altcp_nagle_disable_fn)(conn: &mut altcp_pcb);
type altcp_nagle_disable_fn = fn(conn: &mut altcp_pcb);
// typedef void  (*altcp_nagle_enable_fn)(conn: &mut altcp_pcb);
type altcp_nagle_enable_fn = fn(conn: &mut altcp_pcb);
// typedef int   (*altcp_nagle_disabled_fn)(conn: &mut altcp_pcb);
type altcp_nagle_disabled_fn = fn(conn: &mut altcp_pcb) -> i32;
// typedef void  (*altcp_setprio_fn)(conn: &mut altcp_pcb, prio: u8);
type altcp_setprio_fn = fn(conn: &mut altcp_pcb, prio: u8);
// typedef void  (*altcp_dealloc_fn)(conn: &mut altcp_pcb);
type altcp_dealloc_fn = fn(conn: &mut altcp_pcb);
// typedef err_t (*altcp_get_tcp_addrinfo_fn)(conn: &mut altcp_pcb, local: int, addr: &mut ip_addr_t, port: &mut u16);
type altcp_get_tcp_addrinfo_fn =
    fn(conn: &mut altcp_pcb, local: i32, addr: &ip_addr_t, port: &u16) -> err_t;
// typedef ip_addr_t *(*altcp_get_ip_fn)(conn: &mut altcp_pcb, local: int);
type altcp_get_ip_fn = fn(conn: &mut altcp_pcb, local: i32) -> &ip_addr_t;
// typedef u16 (*altcp_get_port_fn)(conn: &mut altcp_pcb, local: int);
type altcp_get_port_fn = fn(conn: &mut altcp_pcb, local: i32) -> u16;
// typedef enum tcp_state (*altcp_dbg_get_tcp_state_fn)(conn: &mut altcp_pcb);
type altcp_dbg_get_tcp_state_fn = fn(conn: &mut altcp_pcb) -> tcp_state;

pub struct altcp_functions {
    set_poll: altcp_set_poll_fn,
    recved: altcp_recved_fn,
    bind: altcp_bind_fn,
    connect: altcp_connect_fn,
    listen: altcp_listen_fn,
    abort: altcp_abort_fn,
    close: altcp_close_fn,
    shutdown: altcp_shutdown_fn,
    write: altcp_write_fn,
    output: altcp_output_fn,
    mss: altcp_mss_fn,
    sndbuf: altcp_sndbuf_fn,
    sndqueuelen: altcp_sndqueuelen_fn,
    nagle_disable: altcp_nagle_disable_fn,
    nagle_enable: altcp_nagle_enable_fn,
    nagle_disabled: altcp_nagle_disabled_fn,
    setprio: altcp_setprio_fn,
    dealloc: altcp_dealloc_fn,
    addrinfo: altcp_get_tcp_addrinfo_fn,
    getip: altcp_get_ip_fn,
    getport: altcp_get_port_fn,
    dbg_get_tcp_state: altcp_dbg_get_tcp_state_fn,
}

pub struct altcp_pcb {
    fns: &mut altcp_functions,
    // inner_conn: &mut altcp_pcb;
    // TODO: figure out how to handle self-referencing inner struct
    // arg: &mut Vec<u8>;
    arg: Vec<u8>,
    // void *state;
    state: Vec<u8>,
    /* application callbacks */
    // altcp_accept_fn     accept;
    accept: altcp_accept_fn,
    // altcp_connected_fn  connected;
    connected: altcp_connected_fn,
    // altcp_recv_fn       recv;
    recv: altcp_recv_fn,
    // altcp_sent_fn       sent;
    sent: altcp_sent_fn,
    // altcp_poll_fn       poll;
    poll: altcp_poll_fn,
    // altcp_err_fn        err;
    err: altcp_err_fn,
    // pollinterval: u8;
    pollinterval: u8,
}

/** @ingroup altcp */
// typedef struct altcp_allocator_s {
//   /** Allocator function */
//   altcp_new_fn  alloc;
//   /** Argument to allocator function */
//   void         *arg;
// } altcp_allocator_t;
pub struct altcp_allocator_t {
    alloc: altcp_new_fn,
    arg: Vec<u8>,
}

pub struct altcp_proxyconnect_config {
    // ip_addr_t proxy_addr;
    pub proxy_addr: ip_addr_t,
    // proxy_port: u16;
    pub proxy_port: u16,
}

pub struct altcp_proxyconnect_tls_config {
    // struct altcp_proxyconnect_config proxy;
    proxy: altcp_proxyconnect_config,
    // tls_config: &mut altcp_tls_config;
    tls_config: altcp_tls_config,
}
