/**
 * @file
 * Application layered TCP connection API (to be used from TCPIP thread)
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















struct altcp_pcb;
struct altcp_functions;

typedef err_t (*altcp_accept_fn)(void *arg, struct altcp_pcb *new_conn, err_t err);
typedef err_t (*altcp_connected_fn)(void *arg, struct altcp_pcb *conn, err_t err);
typedef err_t (*altcp_recv_fn)(void *arg, struct altcp_pcb *conn, struct pbuf *p, err_t err);
typedef err_t (*altcp_sent_fn)(void *arg, struct altcp_pcb *conn, u16_t len);
typedef err_t (*altcp_poll_fn)(void *arg, struct altcp_pcb *conn);
typedef void  (*altcp_err_fn)(void *arg, err_t err);

typedef struct altcp_pcb* (*altcp_new_fn)(void *arg, u8_t ip_type);

struct altcp_pcb {
  const struct altcp_functions *fns;
  struct altcp_pcb *inner_conn;
  void *arg;
  void *state;
  /* application callbacks */
  altcp_accept_fn     accept;
  altcp_connected_fn  connected;
  altcp_recv_fn       recv;
  altcp_sent_fn       sent;
  altcp_poll_fn       poll;
  altcp_err_fn        err;
  u8_t pollinterval;
};

/** @ingroup altcp
 *  Struct containing an allocator and its state. */
typedef struct altcp_allocator_s {
  /** Allocator function */
  altcp_new_fn  alloc;
  /** Argument to allocator function */
  void         *arg;
} altcp_allocator_t;

struct altcp_pcb *altcp_new(altcp_allocator_t *allocator);
struct altcp_pcb *altcp_new_ip6(altcp_allocator_t *allocator);
struct altcp_pcb *altcp_new_ip_type(altcp_allocator_t *allocator, u8_t ip_type);

void altcp_arg(struct altcp_pcb *conn, void *arg);
void altcp_accept(struct altcp_pcb *conn, altcp_accept_fn accept);
void altcp_recv(struct altcp_pcb *conn, altcp_recv_fn recv);
void altcp_sent(struct altcp_pcb *conn, altcp_sent_fn sent);
void altcp_poll(struct altcp_pcb *conn, altcp_poll_fn poll, u8_t interval);
void altcp_err(struct altcp_pcb *conn, altcp_err_fn err);

void  altcp_recved(struct altcp_pcb *conn, u16_t len);
err_t altcp_bind(struct altcp_pcb *conn, const ip_addr_t *ipaddr, u16_t port);
err_t altcp_connect(struct altcp_pcb *conn, const ip_addr_t *ipaddr, u16_t port, altcp_connected_fn connected);

/* return conn for source code compatibility to tcp callback API only */
struct altcp_pcb *altcp_listen_with_backlog_and_err(struct altcp_pcb *conn, u8_t backlog, err_t *err);
#define altcp_listen_with_backlog(conn, backlog) altcp_listen_with_backlog_and_err(conn, backlog, NULL)
/** @ingroup altcp */
#define altcp_listen(conn) altcp_listen_with_backlog_and_err(conn, TCP_DEFAULT_LISTEN_BACKLOG, NULL)

void altcp_abort(struct altcp_pcb *conn);
err_t altcp_close(struct altcp_pcb *conn);
err_t altcp_shutdown(struct altcp_pcb *conn, int shut_rx, int shut_tx);

err_t altcp_write(struct altcp_pcb *conn, const void *dataptr, u16_t len, u8_t apiflags);
err_t altcp_output(struct altcp_pcb *conn);

u16_t altcp_mss(struct altcp_pcb *conn);
u16_t altcp_sndbuf(struct altcp_pcb *conn);
u16_t altcp_sndqueuelen(struct altcp_pcb *conn);
void  altcp_nagle_disable(struct altcp_pcb *conn);
void  altcp_nagle_enable(struct altcp_pcb *conn);
int   altcp_nagle_disabled(struct altcp_pcb *conn);

void  altcp_setprio(struct altcp_pcb *conn, u8_t prio);

err_t altcp_get_tcp_addrinfo(struct altcp_pcb *conn, int local, ip_addr_t *addr, u16_t *port);
ip_addr_t *altcp_get_ip(struct altcp_pcb *conn, int local);
u16_t altcp_get_port(struct altcp_pcb *conn, int local);


void  altcp_keepalive_disable(struct altcp_pcb *conn);
void  altcp_keepalive_enable(struct altcp_pcb *conn, u32_t idle, u32_t intvl, u32_t count);



enum tcp_state altcp_dbg_get_tcp_state(struct altcp_pcb *conn);



}




/* ALTCP disabled, define everything to link against tcp callback API (e.g. to get a small non-ssl httpd) */



pub const altcp_accept_fn: u32 = tcp_accept_fn;
pub const altcp_connected_fn: u32 = tcp_connected_fn;
pub const altcp_recv_fn: u32 = tcp_recv_fn;
pub const altcp_sent_fn: u32 = tcp_sent_fn;
pub const altcp_poll_fn: u32 = tcp_poll_fn;
pub const altcp_err_fn: u32 = tcp_err_fn;

pub const altcp_pcb: u32 = tcp_pcb;
pub const altcp_tcp_new_ip_type: u32 = tcp_new_ip_type;
pub const altcp_tcp_new: u32 = tcp_new;
pub const altcp_tcp_new_ip6: u32 = tcp_new_ip6;

#define altcp_new(allocator) tcp_new()
#define altcp_new_ip6(allocator) tcp_new_ip6()
#define altcp_new_ip_type(allocator, ip_type) tcp_new_ip_type(ip_type)

pub const altcp_arg: u32 = tcp_arg;
pub const altcp_accept: u32 = tcp_accept;
pub const altcp_recv: u32 = tcp_recv;
pub const altcp_sent: u32 = tcp_sent;
pub const altcp_poll: u32 = tcp_poll;
pub const altcp_err: u32 = tcp_err;

pub const altcp_recved: u32 = tcp_recved;
pub const altcp_bind: u32 = tcp_bind;
pub const altcp_connect: u32 = tcp_connect;

pub const altcp_listen_with_backlog_and_err: u32 = tcp_listen_with_backlog_and_err;
pub const altcp_listen_with_backlog: u32 = tcp_listen_with_backlog;
pub const altcp_listen: u32 = tcp_listen;

pub const altcp_abort: u32 = tcp_abort;
pub const altcp_close: u32 = tcp_close;
pub const altcp_shutdown: u32 = tcp_shutdown;

pub const altcp_write: u32 = tcp_write;
pub const altcp_output: u32 = tcp_output;

pub const altcp_mss: u32 = tcp_mss;
pub const altcp_sndbuf: u32 = tcp_sndbuf;
pub const altcp_sndqueuelen: u32 = tcp_sndqueuelen;
pub const altcp_nagle_disable: u32 = tcp_nagle_disable;
pub const altcp_nagle_enable: u32 = tcp_nagle_enable;
pub const altcp_nagle_disabled: u32 = tcp_nagle_disabled;
pub const altcp_setprio: u32 = tcp_setprio;

pub const altcp_get_tcp_addrinfo: u32 = tcp_get_tcp_addrinfo;
#define altcp_get_ip(pcb, local) ((local) ? (&(pcb)->local_ip) : (&(pcb)->remote_ip))


pub const altcp_dbg_get_tcp_state: u32 = tcp_dbg_get_tcp_state;


 /* LWIP_ALTCP */

 /* LWIP_HDR_ALTCP_H */
