use crate::core::api_h::{netvector, netconn};

/*
 * @file
 * netconn API lwIP internal implementations (do not use in application code)
 */

/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
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
 * Author: Adam Dunkels <adam@sics.se>
 *
 */

/* Note: Netconn API is always available when sockets are enabled -
 * sockets are implemented on top of them */


// TODO:
// #define API_MSG_M_DEF_SEM(m)  *m
// #else
// #define API_MSG_M_DEF_SEM(m)  API_MSG_M_DEF(m)

// #else /* LWIP_MPU_COMPATIBLE */
// #define API_MSG_M_DEF_SEM(m)  API_MSG_M_DEF(m)


/* For the netconn API, these values are use as a bitmask! */
// #define NETCONN_SHUT_RD   1
pub const NETCONN_SHUT_RD: u32 = 1;
// #define NETCONN_SHUT_WR   2
pub const NETCONN_SHUT_WR: u32 = 2;
// #define NETCONN_SHUT_RDWR (NETCONN_SHUT_RD | NETCONN_SHUT_WR)
pub const NETCONN_SHUT_RWDR: u32 = NETCONN_SHUT_RD | NETCONN_SHUT_WR;

pub struct api_msg_n {
    proto: u8,
}

pub struct api_msg_bc {
    // API_MSG_M_DEF_C(ip_addr_t, ipaddr),
    ipaddr: ip_addr_t,
    port: u16,
    if_idx: u8,
}

pub struct api_msg_ad {
    // API_MSG_M_DEF(ipaddr): ip_addr_t,
    ipaddr: ip_addr_t,
    // API_MSG_M_DEF: u16(port),
    port: u16,
    local: u8,
}

pub struct api_msg_w {
    /* current vector to write */
    vector: netvector,
    /* number of unwritten vectors */
    vector_cnt: u16,
    /* offset into current vector */
    vector_off: usize,
    /* total length across vectors */
    len: usize,
    /* offset into total length/output of bytes written when err == ERR_OK */
    offset: usize,
    apiflags: u8,

    time_started: u32,

}

pub struct api_msg_r {
    len: usize,
}

pub struct api_msg_sd {
    shut: u8,

    time_started: u32,
    // #else /* LWIP_SO_SNDTIMEO || LWIP_SO_LINGER */
    polls_left: u8,

}

pub struct api_msg_jl {
    // API_MSG_M_DEF_C(ip_addr_t, multiaddr),
    multiaddr: ip_addr_t,
    // API_MSG_M_DEF_C(ip_addr_t, netif_addr),
    netif_addr: ip_addr_t,
    if_idx: u8,
    join_or_leave: netconn_igmp,
}

pub struct api_msg_lb {
    backlog: u8,
}

/* IP addresses and port numbers are expected to be in
 * the same byte order as in the corresponding pcb.
 */
/* This struct includes everything that is necessary to execute a function
for a netconn in another thread context (mainly used to process netconns
in the tcpip_thread context to be thread safe). */
pub struct api_msg {
    /* The netconn which to process - always needed: it includes the semaphore
    which is used to block the application thread until the function finished. */
    conn: netconn,
    /* The return value of the function executed in tcpip_thread. */
    err: err_t,
    /* Depending on the executed function, one of these union members is used */

    /* used for lwip_netconn_do_send */
    b: netbuf,
    /* used for lwip_netconn_do_newconn */
    n: api_msg_n,
    /* used for lwip_netconn_do_bind and lwip_netconn_do_connect */
    bc: api_msg_bc,
    /* used for lwip_netconn_do_getaddr */
    ad: api_msg_ad,
    /* used for lwip_netconn_do_write */
    w: api_msg_w,
    /* used for lwip_netconn_do_recv */
    r: api_msg_r,
    /* used for lwip_netconn_do_close (/shutdown) */
    sd: api_msg_sd,
    /* used for lwip_netconn_do_join_leave_group */
    jl: api_msg_jl,
    lb: api_msg_lb,

    op_completed_sem: sys_sem_t,

}

// TODO:
// // #define LWIP_API_MSG_SEM(msg)          ((msg)->op_completed_sem)
// #else /* LWIP_NETCONN_SEM_PER_THREAD */
// // #define LWIP_API_MSG_SEM(msg)          (&(msg)->conn.op_completed)

/* As lwip_netconn_do_gethostbyname requires more arguments but doesn't require a netconn,
it has its own struct (to avoid struct api_msg getting bigger than necessary).
lwip_netconn_do_gethostbyname must be called using tcpip_callback instead of tcpip_apimsg
(see netconn_gethostbyname). */ struct dns_api_msg {
    /* Hostname to query or dotted IP address string */

    // char name[DNS_MAX_NAME_LENGTH];
// #else /* LWIP_MPU_COMPATIBLE */
    name: String,

    /* The resolved address is stored here */
    // API_MSG_M_DEF(addr): ip_addr_t,
    addr: ip_addr_t,

    /* Type of resolve call */
    dns_addrtype: u8,

    /* This semaphore is posted when the name is resolved, the application thread
    should wait on it. */
    // API_MSG_M_DEF_SEM(sem): sys_sem_t,
    sem: sys_sem_t,
    /* Errors are given back here */
    // API_MSG_M_DEF(err): err_t,
    err: err_t,
}


// lwip_netconn_is_deallocated_msg: i32(void *msg);

// lwip_netconn_is_err_msg: i32(void *msg, err: &mut err_t);
// pub fn  lwip_netconn_do_newconn         (void *m);
// pub fn  lwip_netconn_do_delconn         (void *m);
// pub fn  lwip_netconn_do_bind            (void *m);
// pub fn  lwip_netconn_do_bind_if         (void *m);
// pub fn  lwip_netconn_do_connect         (void *m);
// pub fn  lwip_netconn_do_disconnect      (void *m);
// pub fn  lwip_netconn_do_listen          (void *m);
// pub fn  lwip_netconn_do_send            (void *m);
// pub fn  lwip_netconn_do_recv            (void *m);
//
// pub fn  lwip_netconn_do_accepted        (void *m);
//
// pub fn  lwip_netconn_do_write           (void *m);
// pub fn  lwip_netconn_do_getaddr         (void *m);
// pub fn  lwip_netconn_do_close           (void *m);
// pub fn  lwip_netconn_do_shutdown        (void *m);
//
// pub fn  lwip_netconn_do_join_leave_group(void *m);
// pub fn  lwip_netconn_do_join_leave_group_netif(void *m);
//
// pub fn  lwip_netconn_do_gethostbyname(arg: &mut Vec<u8>);

// struct netconn* netconn_alloc(enum netconn_type t, netconn_callback callback);
// pub fn  netconn_free(conn: &mut netconn);

/* netifapi related lwIP internal definitions */

// #define NETIFAPI_IPADDR_DEF(type, m)  type m
// #else /* LWIP_MPU_COMPATIBLE */
// #define NETIFAPI_IPADDR_DEF(type, m)  const type * m

// typedef void (*netifapi_void_fn)(netif: &mut netif); type netifapi_void_fn = fn(netif: &mut netif);
// typedef err_t (*netifapi_errt_fn)(netif: &mut netif); type netifapi_errt_fn = fn(netif: &mut netif);

pub struct netifapi_msg_add {
    ipaddr: ip4_addr,
    netmask: ip4_addr,
    gw: ip4_addr,
}

pub struct netifapi_msg_common {
    voidfunc: netifapi_void_fn,
    errtfunc: netifapi_errt_fn,
}

pub struct netifapi_msg {
    call: tcpip_api_call_data,
    netif: netif,
    add: netifapi_msg_add,
    common: netifapi_msg_common,
    name: String,
    index: u8,
}
