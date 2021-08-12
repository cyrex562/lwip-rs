use crate::core::api_msg_h::api_msg;

/*
 * @file
 * netconn API (to be used from non-TCPIP threads)
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

/* Throughout this file, IP addresses and port numbers are expected to be in
 * the same byte order as in the corresponding pcb.
 */

/* Flags for netconn_write  */
pub const NETCONN_NOFLAG: u32 = 0x00;
pub const NETCONN_NOCOPY: u32 = 0x00; /* Only for source code compatibility */
pub const NETCONN_COPY: u32 = 0x01;
pub const NETCONN_MORE: u32 = 0x02;
pub const NETCONN_DONTBLOCK: u32 = 0x04;
pub const NETCONN_NOAUTORCVD: u32 = 0x08; /* prevent netconn_recv_data_tcp() from updating the tcp window - must be done manually via netconn_tcp_recvd() */
pub const NETCONN_NOFIN: u32 = 0x10; /* upper layer already received data, leave FIN in queue until called again */

/* Flags for struct netconn.flags  */
/* This netconn had an error, don't block on recvmbox/acceptmbox any more */
pub const NETCONN_FLAG_MBOXCLOSED: u32 = 0x01;
/* Should this netconn avoid blocking? */
pub const NETCONN_FLAG_NON_BLOCKING: u32 = 0x02;
/* Was the last connect action a non-blocking one? */
pub const NETCONN_FLAG_IN_NONBLOCKING_CONNECT: u32 = 0x04;

/* The mbox of this netconn is being deallocated, don't use it anymore */
pub const NETCONN_FLAG_MBOXINVALID: u32 = 0x08;

/* If a nonblocking write has been rejected before, poll_tcp needs to
check if the netconn is writable again */
pub const NETCONN_FLAG_CHECK_WRITESPACE: u32 = 0x10;

/* If this flag is set then only IPv6 communication is allowed on the
netconn. As per RFC#3493 this features defaults to OFF allowing
dual-stack usage by default. */
pub const NETCONN_FLAG_IPV6_V6ONLY: u32 = 0x20;

/* Received packet info will be recorded for this netconn */
pub const NETCONN_FLAG_PKTINFO: u32 = 0x40;

/* A FIN has been received but not passed to the application yet */
pub const NETCONN_FIN_RX_PENDING: u32 = 0x80;

/* Helpers to process several netconn_types by the same code */
// TODO: #define NETCONNTYPE_GROUP(t)         ((t)&0xF0)
// TODO: #define NETCONNTYPE_DATAGRAM(t)      ((t)&0xE0)

pub const NETCONN_TYPE_IPV6: u32 = 0x08;
// TODO: #define NETCONNTYPE_ISIPV6(t)        (((t)&NETCONN_TYPE_IPV6) != 0)
// TODO: #define NETCONNTYPE_ISUDPLITE(t)     (((t)&0xF3) == NETCONN_UDPLITE)
// TODO: #define NETCONNTYPE_ISUDPNOCHKSUM(t) (((t)&0xF3) == NETCONN_UDPNOCHKSUM)

// #else /* LWIP_IPV6 */
// #define NETCONNTYPE_ISIPV6(t)        (0)
// #define NETCONNTYPE_ISUDPLITE(t)     ((t) == NETCONN_UDPLITE)
// #define NETCONNTYPE_ISUDPNOCHKSUM(t) ((t) == NETCONN_UDPNOCHKSUM)

/* @ingroup netconn_common
 * Protocol family and type of the netconn
 */
pub const NETCONN_INVALID: u32 = 0;
/* TCP IPv4 */
pub const NETCONN_TCP: u32 = 0x10;
/* TCP IPv6 */
pub const NETCONN_TCP_IPV6: u32 = NETCONN_TCP | NETCONN_TYPE_IPV6;
/* UDP IPv4 */
pub const NETCONN_UDP: u32 = 0x20;
/* UDP IPv4 lite */
pub const NETCONN_UDPLITE: u32 = 0x21;
/* UDP IPv4 no checksum */
pub const NETCONN_UDPNOCHKSUM: u32 = 0x22;
/* UDP IPv6 (dual-stack by default, unless you call @ref netconn_set_ipv6only) */
pub const NETCONN_UDP_IPV6: u32 = NETCONN_UDP | NETCONN_TYPE_IPV6;
/* UDP IPv6 lite (dual-stack by default, unless you call @ref netconn_set_ipv6only) */
pub const NETCONN_UDPLITE_IPV6: u32 = NETCONN_UDPLITE | NETCONN_TYPE_IPV6;
/* UDP IPv6 no checksum (dual-stack by default, unless you call @ref netconn_set_ipv6only) */
pub const NETCONN_UDPNOCHKSUM_IPV6: u32 = NETCONN_UDPNOCHKSUM | NETCONN_TYPE_IPV6;
/* Raw connection IPv4 */
pub const NETCONN_RAW: u32 = 0x40;
/* Raw connection IPv6 (dual-stack by default, unless you call @ref netconn_set_ipv6only) */
pub const NETCONN_RAW_IPv6: u32 = NETCONN_RAW | NETCONN_TYPE_IPV6;

/* Current state of the netconn. Non-TCP netconns are always
 * in state NETCONN_NONE! */
enum netconn_state {
    NETCONN_NONE,
    NETCONN_WRITE,
    NETCONN_LISTEN,
    NETCONN_CONNECT,
    NETCONN_CLOSE,
}

/* Used to inform the callback function about changes
 *
 * Event explanation:
 *
 * In the netconn implementation, there are three ways to block a client:
 *
 * - accept mbox (sys_arch_mbox_fetch(&conn.acceptmbox, &accept_ptr, 0); in netconn_accept())
 * - receive mbox (sys_arch_mbox_fetch(&conn.recvmbox, &buf, 0); in netconn_recv_data())
 * - send queue is full (sys_arch_sem_wait(LWIP_API_MSG_SEM(msg), 0); in lwip_netconn_do_write())
 *
 * The events have to be seen as events signaling the state of these mboxes/semaphores. For non-blocking
 * connections, you need to know in advance whether a call to a netconn function call would block or not,
 * and these events tell you about that.
 *
 * RCVPLUS events say: Safe to perform a potentially blocking call call once more.
 * They are counted in sockets - three RCVPLUS events for accept mbox means you are safe
 * to call netconn_accept 3 times without being blocked.
 * Same thing for receive mbox.
 *
 * RCVMINUS events say: Your call to to a possibly blocking function is "acknowledged".
 * Socket implementation decrements the counter.
 *
 * For TX, there is no need to count, its merely a flag. SENDPLUS means you may send something.
 * SENDPLUS occurs when enough data was delivered to peer so netconn_send() can be called again.
 * A SENDMINUS event occurs when the next call to a netconn_send() would be blocking.
 */
enum netconn_evt {
    NETCONN_EVT_RCVPLUS,
    NETCONN_EVT_RCVMINUS,
    NETCONN_EVT_SENDPLUS,
    NETCONN_EVT_SENDMINUS,
    NETCONN_EVT_ERROR,
}

/* Used for netconn_join_leave_group() */
enum netconn_igmp {
    NETCONN_JOIN,
    NETCONN_LEAVE,
}

/* Used for netconn_gethostbyname_addrtype(), these should match the DNS_ADDRTYPE defines in dns.h */
// #define NETCONN_DNS_DEFAULT   NETCONN_DNS_IPV4_IPV6
pub const NETCONN_DNS_DEFAULT: u32 = NETCONN_DNS_IPV4_IPV6;
pub const NETCONN_DNS_IPV4: u32 = 0;
// #define NETCONN_DNS_IPV6      1
pub const NETCONN_DNS_IPV6: u32 = 1;
// #define NETCONN_DNS_IPV4_IPV6 2 /* try to resolve IPv4 first, try IPv6 if IPv4 fails only */
pub const NETCONN_DNS_IPV4_IPV6: u32 = 2;
// #define NETCONN_DNS_IPV6_IPV4 3 /* try to resolve IPv6 first, try IPv4 if IPv6 fails only */
pub const NETCONN_DNS_IPV6_IPV4: u32 = 3;

// /* forward-declare some structs to avoid to include their headers */
// struct ip_pcb;
// struct tcp_pcb;
// struct udp_pcb;
// struct raw_pcb;
// struct netconn;
// struct api_msg;

/* A callback prototype to inform about events for a netconn */
// typedef void (* netconn_callback)(struct netconn *, enum netconn_evt, len: u16);
type netconn_callback = fn(&mut netconn, netconn_evt, u16);

/* A netconn descriptor */
pub struct netconn {
    /* type of the netconn (TCP, UDP or RAW) */
    // enum netconn_type type;
    pub netconn_type: u32,
    /* current state of the netconn */
    // enum netconn_state state;
    pub state: netconn_state,
    /* the lwIP internal protocol control block */
    pub ip: ip_pcb,
    pub tcp_pcb: tcp,
    pub udp: udp_pcb,
    pub raw: raw_pcb,
    /* the last asynchronous unreported error this netconn had */
    pub pending_err: err_t,

    /* sem that is used to synchronously execute functions in the core context */
    pub op_completed: sys_sem_t,

    /* mbox where received packets are stored until they are fetched
    by the netconn application thread (can grow quite big) */
    pub recvmbox: sys_mbox_t,

    /* mbox where new connections are stored until processed
    by the application thread */
    pub acceptmbox: sys_mbox_t,

    /* number of threads waiting on an mbox. This is required to unblock
    all threads when closing while threads are waiting. */
    pub mbox_threads_waiting: i32,

    /* only used for socket layer */
    pub socket: i32,

    /* timeout to wait for sending data (which means enqueueing data for sending
    in internal buffers) in milliseconds */
    send_timeout: i32,

    /* timeout in milliseconds to wait for new data to be received
    (or connections to arrive for listening netconns) */
    recv_timeout: u32,

    /* maximum amount of bytes queued in recvmbox
    not used for TCP: adjust TCP_WND instead! */
    recv_bufsize: i32,
    /* number of bytes currently in recvmbox to be received,
    tested against recv_bufsize to limit bytes on recvmbox
    for UDP and RAW, used for FIONREAD */
    recv_avail: i32,

    /* values <0 mean linger is disabled, values > 0 are seconds to linger */
    linger: i16,

    /* flags holding more netconn-internal state, see NETCONN_FLAG_* defines */
    flags: u8,

    /* TCP: when data passed to netconn_write doesn't fit into the send buffer,
    this temporarily stores the message.
    Also used during connect and close. */
    current_msg: api_msg,

    /* A callback function that is informed about events for this netconn */
    callback: netconn_callback,
}

/* This vector type is passed to @ref netconn_write_vectors_partly to send
 * multiple buffers at once.
 * ATTENTION: This type has to directly map struct iovec since one is casted
 *            into the other!
 */
pub struct netvector {
    /* pointer to the application buffer that contains the data to send */
    ptr: Vec<u8>,
    /* size of the application data to send */
    len: usize,
}

/* Register an Network connection event */
// TODO:
// #define API_EVENT(c,e,l) if (c.callback) {         \
//                            (*c.callback)(c, e, l); \
//                          }

/* Network connection functions: */

/* @ingroup netconn_common
 * Create new netconn connection
 * @param t @ref netconn_type */

// TODO: #define netconn_new(t)                  netconn_new_with_proto_and_callback(t, 0, NULL)
// TODO: #define netconn_new_with_callback(t, c) netconn_new_with_proto_and_callback(t, 0, c)
// TODO: netconn_new_with_proto_and_callback: &mut netconn(enum netconn_type t, proto: u8,
//                                              netconn_callback callback);
// pub fn    netconn_prepare_delete(conn: &mut netconn);
// pub fn    netconn_delete(conn: &mut netconn);
/* Get the type of a netconn (as enum netconn_type). */
// TODO: #define netconn_type(conn) (conn.type)

// pub fn    netconn_getaddr(conn: &mut netconn, addr: &mut ip_addr_t, port: &mut u16, local: u8);
/* @ingroup netconn_common */
//  TODO: #define netconn_peer(c,i,p) netconn_getaddr(c,i,p,0)
/* @ingroup netconn_common */
//  TODO: #define netconn_addr(c,i,p) netconn_getaddr(c,i,p,1)

// pub fn    netconn_bind(conn: &mut netconn,  addr: &mut ip_addr_t, port: u16);
// pub fn    netconn_bind_if(conn: &mut netconn, if_idx: u8);
// pub fn    netconn_connect(conn: &mut netconn,  addr: &mut ip_addr_t, port: u16);
// pub fn    netconn_disconnect (conn: &mut netconn);
// pub fn    netconn_listen_with_backlog(conn: &mut netconn, backlog: u8);
/* @ingroup netconn_tcp */
// TODO: #define netconn_listen(conn) netconn_listen_with_backlog(conn, TCP_DEFAULT_LISTEN_BACKLOG)
// pub fn    netconn_accept(conn: &mut netconn, struct netconn **new_conn);
// pub fn    netconn_recv(conn: &mut netconn, struct netbuf **new_buf);
// pub fn    netconn_recv_udp_raw_netbuf(conn: &mut netconn, struct netbuf **new_buf);
// pub fn    netconn_recv_udp_raw_netbuf_flags(conn: &mut netconn, struct netbuf **new_buf, apiflags: u8);
// pub fn    netconn_recv_tcp_pbuf(conn: &mut netconn, struct pbuf **new_buf);
// pub fn    netconn_recv_tcp_pbuf_flags(conn: &mut netconn, struct pbuf **new_buf, apiflags: u8);
// pub fn    netconn_tcp_recvd(conn: &mut netconn, len: usize);
// pub fn    netconn_sendto(conn: &mut netconn, buf: &mut netbuf,
//                              const addr: &mut ip_addr_t, port: u16);
// pub fn    netconn_send(conn: &mut netconn, buf: &mut netbuf);
// pub fn    netconn_write_partly(conn: &mut netconn, dataptr: &Vec<u8>, size: usize,
//                              apiflags: u8, usize *bytes_written);
// pub fn    netconn_write_vectors_partly(conn: &mut netconn, vectors: &mut netvector, vectorcnt: u16,
//                                      apiflags: u8, usize *bytes_written);
/* @ingroup netconn_tcp */
//  TODO: #define netconn_write(conn, dataptr, size, apiflags) \
// netconn_write_partly(conn, dataptr, size, apiflags, NULL)
// // pub fn    netconn_close(conn: &mut netconn);
// // pub fn    netconn_shutdown(conn: &mut netconn, shut_rx: u8, shut_tx: u8);

// // pub fn    netconn_join_leave_group(conn: &mut netconn,  multiaddr: &mut ip_addr_t,
// //                              const netif_addr: &mut ip_addr_t, enum netconn_igmp join_or_leave);
// // pub fn    netconn_join_leave_group_netif(conn: &mut netconn,  multiaddr: &mut ip_addr_t,
// //                              if_idx: u8, enum netconn_igmp join_or_leave);

// // pub fn    netconn_gethostbyname_addrtype(name: &String, addr: &mut ip_addr_t, dns_addrtype: u8);
// #define netconn_gethostbyname(name, addr) netconn_gethostbyname_addrtype(name, addr, NETCONN_DNS_DEFAULT)
// #else /* LWIP_IPV4 && LWIP_IPV6 */
// pub fn    netconn_gethostbyname(name: &String, addr: &mut ip_addr_t);
// #define netconn_gethostbyname_addrtype(name, addr, dns_addrtype) netconn_gethostbyname(name, addr)

// pub fn    netconn_err(conn: &mut netconn);
// #define netconn_recv_bufsize(conn)      ((conn).recv_bufsize)

// #define netconn_set_flags(conn, set_flags)     do { (conn)->flags = ((conn)->flags |  (set_flags)); } while(0)
// #define netconn_clear_flags(conn, clr_flags)   do { (conn)->flags = ((conn)->flags & (~(clr_flags) & 0xff)); } while(0)
// #define netconn_is_flag_set(conn, flag)        (((conn)->flags & (flag)) != 0)

// /* Set the blocking status of netconn calls (@todo: write/send is missing) */
// #define netconn_set_nonblocking(conn, val)  do { if(val) { \
//   netconn_set_flags(conn, NETCONN_FLAG_NON_BLOCKING); \
// } else { \
//   netconn_clear_flags(conn, NETCONN_FLAG_NON_BLOCKING); }} while(0)
// /* Get the blocking status of netconn calls (@todo: write/send is missing) */
// #define netconn_is_nonblocking(conn)        (((conn)->flags & NETCONN_FLAG_NON_BLOCKING) != 0)

// /* @ingroup netconn_common
//  * TCP: Set the IPv6 ONLY status of netconn calls (see NETCONN_FLAG_IPV6_V6ONLY)
//  */
// #define netconn_set_ipv6only(conn, val)  do { if(val) { \
//   netconn_set_flags(conn, NETCONN_FLAG_IPV6_V6ONLY); \
// } else { \
//   netconn_clear_flags(conn, NETCONN_FLAG_IPV6_V6ONLY); }} while(0)
// /* @ingroup netconn_common
//  * TCP: Get the IPv6 ONLY status of netconn calls (see NETCONN_FLAG_IPV6_V6ONLY)
//  */
// #define netconn_get_ipv6only(conn)        (((conn)->flags & NETCONN_FLAG_IPV6_V6ONLY) != 0)

// /* Set the send timeout in milliseconds */
// #define netconn_set_sendtimeout(conn, timeout)      ((conn)->send_timeout = (timeout))
// /* Get the send timeout in milliseconds */
// #define netconn_get_sendtimeout(conn)               ((conn)->send_timeout)

// /* Set the receive timeout in milliseconds */
// #define netconn_set_recvtimeout(conn, timeout)      ((conn).recv_timeout = (timeout))
// /* Get the receive timeout in milliseconds */
// #define netconn_get_recvtimeout(conn)               ((conn).recv_timeout)

// /* Set the receive buffer in bytes */
// #define netconn_set_recvbufsize(conn, recvbufsize)  ((conn).recv_bufsize = (recvbufsize))
// /* Get the receive buffer in bytes */
// #define netconn_get_recvbufsize(conn)               ((conn).recv_bufsize)

// pub fn  netconn_thread_init();
// pub fn  netconn_thread_cleanup();
// #else /* LWIP_NETCONN_SEM_PER_THREAD */
// #define netconn_thread_init()
// #define netconn_thread_cleanup()
