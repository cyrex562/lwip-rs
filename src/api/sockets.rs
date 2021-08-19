use std::net::{IpAddr, SocketAddr};

/*
 * @file
 * Sockets BSD-Like API module
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
 * Improved by Marc Boucher <marc@mbsi.ca> and David Haas <dhaas@alum.rpi.edu>
 *
 */
































/* If the netconn API is not required publicly, then we include the necessary
   files here to get the implementation */

// #undef LWIP_NETCONN
// #define LWIP_NETCONN 1



// #undef LWIP_NETCONN
pub const LWIP_NETCONN: u32 = 0;


// #define API_SELECT_CB_VAR_REF(name)               (name)
// #define API_SELECT_CB_VAR_DECLARE(name)           API_VAR_DECLARE(LwipSelectCallback, name)
// #define API_SELECT_CB_VAR_ALLOC(name, retblock)   API_VAR_ALLOC_EXT(LwipSelectCallback, MEMP_SELECT_CB, name, retblock)
// #define API_SELECT_CB_VAR_FREE(name)              API_VAR_FREE(MEMP_SELECT_CB, name)


pub fn  IP4ADDR_PORT_TO_SOCKADDR(sin: LwipSockAddr, ipaddr: IpAddr, port: u16) { 
      (sin).sin_len = sizeof(sockaddr_in); 
      (sin).sin_family = AF_INET; 
      (sin).sin_port = lwip_htons((port)); 
      inet_addr_from_ip4addr(&(sin).sin_addr, ipaddr); 
      memset((sin).sin_zero, 0, SIN_ZERO_LEN); }

pub fn SOCKADDR4_TO_IP4ADDR_PORT(
  sin: LwipSockAddr, 
  ipaddr: IpAddr, port: u16) {
    inet_addr_to_ip4addr(ip_2_ip4(ipaddr), &((sin).sin_addr)); 
    (port) = lwip_ntohs((sin).sin_port); }


pub fn IP6ADDR_PORT_TO_SOCKADDR(
  sin6: LwipSockAddr, 
  ipaddr: IpAddr, 
  port: u16) { 
      (sin6).sin6_len = sizeof(sockaddr_in6); 
      (sin6).sin6_family = AF_INET6; 
      (sin6).sin6_port = lwip_htons((port)); 
      (sin6).sin6_flowinfo = 0; 
      inet6_addr_from_ip6addr(&(sin6).sin6_addr, ipaddr); 
      (sin6).sin6_scope_id = ip6_addr_zone(ipaddr); }

pub fn SOCKADDR6_TO_IP6ADDR_PORT(sin6: LwipSockAddr, ipaddr: IpAddr, port: u16) {
    inet6_addr_to_ip6addr(ip_2_ip6(ipaddr), &((sin6).sin6_addr)); 
    if (ip6_addr_has_scope(ip_2_ip6(ipaddr), IP6_UNKNOWN)) { 
      ip6_addr_set_zone(ip_2_ip6(ipaddr), ((sin6).sin6_scope_id)); 
    } 
    (port) = lwip_ntohs((sin6).sin6_port); }



// pub fn sockaddr_to_ipaddr_port(sockaddr: &mut sockaddr, ipaddr: &mut ip_addr_t, port: &mut u16);

pub fn IS_SOCK_ADDR_LEN_VALID(namelen: usize) -> bool  {
  (((namelen) == sizeof(sockaddr_in)) || ((namelen) == sizeof(sockaddr_in6)))
}

pub fn IS_SOCK_ADDR_TYPE_VALID(name: LwipSockAddr) -> bool  {  
  (name.sa_family == AF_INET) ||  AF_INET6
}




pub fn SOCK_ADDR_TYPE_MATCH(name: LwipSockAddr, sock: LwipSocket) -> bool {
       (((name.sa_family == AF_INET) && !(NETCONNTYPE_ISIPV6(sock.conn.netconntype))) || 
       ((name.sa_family == AF_INET6) && (NETCONNTYPE_ISIPV6(sock.conn.netconntype))))}



pub fn IPADDR_PORT_TO_SOCKADDR(sockaddr: &mut sockaddr, ipaddr: IpAddr, port: u16) { 
    if (IP_IS_ANY_TYPE_VAL(*ipaddr) || IP_IS_V6_VAL(*ipaddr)) { 
      IP6ADDR_PORT_TO_SOCKADDR(sockaddr, ip_2_ip6(ipaddr), port); 
    } else { 
      IP4ADDR_PORT_TO_SOCKADDR(sockaddr, ip_2_ip4(ipaddr), port); 
    } } 

pub fn SOCKADDR_TO_IPADDR_PORT(sockaddr: &mut sockaddr, ipaddr: IpAddr, port: u16) {sockaddr_to_ipaddr_port(sockaddr, &ipaddr, &(port))}


pub fn DOMAIN_TO_NETCONN_TYPE(domain: (), netconntype: ()) {
  
  // (((domain) == AF_INET) ? (netconntype) : ((netconntype) | NETCONN_TYPE_IPV6))

  if domain == AF_INET {
    netconntype
  } else {
    netconntype | NETCONN_TYPE_IPV6
  }

}


// #elif LWIP_IPV6 /* LWIP_IPV4 && LWIP_IPV6 */
pub fn IS_SOCK_ADDR_LEN_VALID(namelen: usize) -> bool{ (namelen) == sizeof(sockaddr_in6)}

pub fn IS_SOCK_ADDR_TYPE_VALID(name: sockaddr)  {  ((name).sa_family == AF_INET6)}


// #define SOCK_ADDR_TYPE_MATCH(name, sock) 1


pub fn IPADDR_PORT_TO_SOCKADDR(sockaddr: sockaddr_in, ipaddr: ip_addr_t, port: u16) {
        IP6ADDR_PORT_TO_SOCKADDR((sockaddr), ip_2_ip6(ipaddr), port)}


pub fn SOCKADDR_TO_IPADDR_PORT(sockaddr: sockaddr_in, ipaddr: ip_addr_t, port: u16) {
SOCKADDR6_TO_IP6ADDR_PORT((sockaddr), ipaddr, port)}


// pub fn DOMAIN_TO_NETCONN_TYPE(domain, netconn_type) (netconn_type)
 /*. LWIP_IPV4: LWIP_IPV4 && LWIP_IPV6 */


 pub fn IS_SOCK_ADDR_LEN_VALID(namelen: usize) -> bool{  ((namelen) == sizeof(sockaddr_in))}


 pub fn IS_SOCK_ADDR_TYPE_VALID(name: sockaddr) -> bool{    ((name).sa_family == AF_INET)}


//  #define SOCK_ADDR_TYPE_MATCH(name, sock) 1


pub fn IPADDR_PORT_TO_SOCKADDR(
  sockaddr: sockaddr, 
  ipaddr: ip_addr_t, 
  port: u16) {
        IP4ADDR_PORT_TO_SOCKADDR((sockaddr), ip_2_ip4(ipaddr), port)}


pub fn SOCKADDR_TO_IPADDR_PORT(
  sockaddr: sockaddr, 
  ipaddr: ip_addr_t, 
  port: u16) {
        SOCKADDR4_TO_IP4ADDR_PORT((sockaddr), ipaddr, port)}


// #define DOMAIN_TO_NETCONN_TYPE(domain, netconn_type) (netconn_type)


pub fn IS_SOCK_ADDR_TYPE_VALID_OR_UNSPEC(name: sockaddr) -> bool{    (((name).sa_family == AF_UNSPEC) || IS_SOCK_ADDR_TYPE_VALID(name))}

pub fn SOCK_ADDR_TYPE_MATCH_OR_UNSPEC(name: sockaddr, sock: socket) {(((name).sa_family == AF_UNSPEC) || SOCK_ADDR_TYPE_MATCH(name, sock))}

// #define IS_SOCK_ADDR_ALIGNED(name)      ((((mem_ptr_t)(name)) % 4) == 0)


pub fn LWIP_SOCKOPT_CHECK_OPTLEN(
  sock: LwipSocket, 
  optlen: usize, 
  opttype: u16) { 
    if ((optlen) < sizeof(opttype)) { 
      done_socket(sock); 
      return EINVAL; 
    }
}

pub fn LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock: LwipSocket, optlen: usize, opttype: u16) { 
LWIP_SOCKOPT_CHECK_OPTLEN(sock, optlen, opttype); 
  if ((sock).conn == NULL) { done_socket(sock); return EINVAL; } }

pub fn LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock: LwipSocket, optlen: usize, opttype: u16) { 
  LWIP_SOCKOPT_CHECK_OPTLEN(sock, optlen, opttype); 
  if (((sock).conn == NULL) || ((sock).conn.pcb.tcp == NULL)) { done_socket(sock); return EINVAL; } }


pub fn LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock: LwipSocket, optlen: usize, opttype: u16, netconntype: u16) { 
  LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, optlen, opttype); 
  if (NETCONNTYPE_GROUP(netconn_type((sock).conn)) != netconntype) { done_socket(sock); return ENOPROTOOPT; } }


// #define LWIP_SETGETSOCKOPT_DATA_VAR_REF(name)     (name)
// #define LWIP_SETGETSOCKOPT_DATA_VAR_DECLARE(name) API_VAR_DECLARE(struct lwip_setgetsockopt_data, name)
// #define LWIP_SETGETSOCKOPT_DATA_VAR_FREE(name)    API_VAR_FREE(MEMP_SOCKET_SETGETSOCKOPT_DATA, name)

pub fn LWIP_SETGETSOCKOPT_DATA_VAR_ALLOC(name: (), sock: LwipSocket) 
{ 
  name = memp_malloc(MEMP_SOCKET_SETGETSOCKOPT_DATA); 
  if (name == NULL) { 
    sock_set_errno(sock, ENOMEM); 
    done_socket(sock); 
    return -1; 
  } }
 /* LWIP_MPU_COMPATIBLE */
// #define LWIP_SETGETSOCKOPT_DATA_VAR_ALLOC(name, sock)



// #define LWIP_SO_SNDRCVTIMEO_OPTTYPE int
// #define LWIP_SO_SNDRCVTIMEO_SET(optval, val) (*(int *)(optval) = (val))
// #define LWIP_SO_SNDRCVTIMEO_GET_MS(optval)   ((long)*(const int*)(optval))

// #define LWIP_SO_SNDRCVTIMEO_OPTTYPE struct timeval
pub fn LWIP_SO_SNDRCVTIMEO_SET(optval: u16, val: u16)   { 
  let loc: u32 = (val); 
  ((optval)).tv_sec = (long)((loc) / 1000); 
  ((optval)).tv_usec = (long)(((loc) % 1000) * 1000); }
// #define LWIP_SO_SNDRCVTIMEO_GET_MS(optval) ((((const struct timeval *)(optval)).tv_sec * 1000) + (((const struct timeval *)(optval)).tv_usec / 1000))



/* A struct sockaddr replacement that has the same alignment as sockaddr_in/
 *  if: sockaddr_in6 instantiated.
 */
// union sockaddr_aligned {
//   struct sockaddr sa;

//   sin6: sockaddr_in6;


//   sin: sockaddr_in;

// };

/* Define the number of IPv4 multicast memberships, default is one per socket */

// #define LWIP_SOCKET_MAX_MEMBERSHIPS NUM_SOCKETS



/* This is to keep track of IP_ADD_MEMBERSHIP calls to drop the membership when
   a socket is closed */
pub struct lwip_socket_multicast_pair {
  /* the socket */
  pub sock: LwipSocket,
  /* the interface address */
   pub if_addr: IpAddr,
  /* the group address */
  pub multi_addr: IpAddr,
}

// static struct lwip_socket_multicast_pair socket_ipv4_multicast_memberships[LWIP_SOCKET_MAX_MEMBERSHIPS];

// static int  lwip_socket_register_membership(s: i32,  if_addr: &mut ip4_addr,  multi_addr: &mut ip4_addr);
// pub fn lwip_socket_unregister_membership(s: i32,  if_addr: &mut ip4_addr,  multi_addr: &mut ip4_addr);
// pub fn lwip_socket_drop_registered_memberships(s: i32);



/* This is to keep track of IP_JOIN_GROUP calls to drop the membership when
   a socket is closed */
pub struct lwip_socket_multicast_mld6_pair {
  /* the socket */
  pub sock: lwip_sock,
  /* the interface index */
  pub if_idx: u8,
  /* the group address */
  pub multi_addr: ip6_addr_t,
}

// static struct lwip_socket_multicast_mld6_pair socket_ipv6_multicast_memberships[LWIP_SOCKET_MAX_MEMBERSHIPS];

// static int  lwip_socket_register_mld6_membership(s: i32,  if_idx: i32,  multi_addr: &mut ip6_addr_t);
// // pub fn lwip_socket_unregister_mld6_membership(s: i32,  if_idx: i32,  multi_addr: &mut ip6_addr_t);
// pub fn lwip_socket_drop_registered_mld6_memberships(s: i32);


/* The global array of available sockets */
// static struct lwip_sock sockets[NUM_SOCKETS];



/* protect the select_cb_list using core lock */
// #define LWIP_SOCKET_SELECT_DECL_PROTECT(lev)
// #define LWIP_SOCKET_SELECT_PROTECT(lev)   LOCK_TCPIP_CORE()
// #define LWIP_SOCKET_SELECT_UNPROTECT(lev) UNLOCK_TCPIP_CORE()
 /* LWIP_TCPIP_CORE_LOCKING */
/* protect the select_cb_list using SYS_LIGHTWEIGHT_PROT */
// #define LWIP_SOCKET_SELECT_DECL_PROTECT(lev)  SYS_ARCH_DECL_PROTECT(lev)
// #define LWIP_SOCKET_SELECT_PROTECT(lev)       SYS_ARCH_PROTECT(lev)
// #define LWIP_SOCKET_SELECT_UNPROTECT(lev)     SYS_ARCH_UNPROTECT(lev)
/* This counter is increased from lwip_select when the list is changed
    and checked in select_check_waiters to see if it has changed. */
// static volatile select_cb_ctr: i32;

/* The global list of tasks waiting for select */
// static select_cb_list: &mut lwip_select_cb;

pub fn sock_set_errno(sk: LwipSocket, e: i32) { 
  let sockerr: i32 = (e); 
  set_errno(sockerr); 
} 

/* Forward declaration of some functions */

// pub fn event_callback(conn: &mut netconn, enum netconn_evt evt, len: u16);
// #define DEFAULT_SOCKET_EVENTCB event_callback
// pub fn select_check_waiters(s: i32, has_recvevent: i32, has_sendevent: i32, has_errevent: i32);

// #define DEFAULT_SOCKET_EVENTCB NULL


// pub fn lwip_getsockopt_callback(arg: &mut Vec<u8>);
// pub fn lwip_setsockopt_callback(arg: &mut Vec<u8>);

// static lwip_getsockopt_impl: i32(s: i32, level: i32, optname: i32, optval: &mut (), socklen_t *optlen);
// static lwip_setsockopt_impl: i32(s: i32, level: i32, optname: i32, optval: &Vec<u8>, optlen: socklen_t);
// static free_socket_locked: i32(sock: &mut lwip_sock, is_tcp: i32, struct netconn **conn, union lwip_sock_lastdata *lastdata);
// pub fn free_socket_free_elements(is_tcp: i32, conn: &mut netconn, union lwip_sock_lastdata *lastdata);


pub fn sockaddr_to_ipaddr_port(
  sockaddr: &mut LwipSockAddr, 
  addr: &mut LwipAddr, 
  port: &mut u16)
{
  if ((sockaddr.sa_family) == AF_INET6) {
    SOCKADDR6_TO_IP6ADDR_PORT(sockaddr, addr, *port);
    addr.addr_type = IPADDR_TYPE_V6;
  } else {
    SOCKADDR4_TO_IP4ADDR_PORT(sockaddr, addr, *port);
    addr.addr_type = IPADDR_TYPE_V4;
  }
}


/* LWIP_NETCONN_SEM_PER_THREAD==1: initialize thread-local semaphore */
pub fn 
lwip_socket_thread_init()
{
  netconn_thread_init();
}

/* LWIP_NETCONN_SEM_PER_THREAD==1: destroy thread-local semaphore */
pub fn 
lwip_socket_thread_cleanup()
{
  netconn_thread_cleanup();
}


/* Thread-safe increment of sock.fd_used, with overflow check */
pub fn sock_inc_used(sock: &mut lwip_sock) -> i32
{
  let ret: i32;
  SYS_ARCH_DECL_PROTECT(lev);

  LWIP_ASSERT("sock != NULL", sock != NULL);

  SYS_ARCH_PROTECT(lev);
  if (sock.fd_free_pending) {
    /* prevent new usage of this socket if free is pending */
    ret = 0;
  } else {
    sock.fd_used += 1;
    ret = 1;
    LWIP_ASSERT("sock.fd_used != 0", sock.fd_used != 0);
  }
  SYS_ARCH_UNPROTECT(lev);
  return ret;
}

/* Like sock_inc_used(), but called under SYS_ARCH_PROTECT lock. */
pub fn sock_inc_used_locked(sock: &mut lwip_sock) -> i32
{
  LWIP_ASSERT("sock != NULL", sock != NULL);

  if (sock.fd_free_pending) {
    LWIP_ASSERT("sock.fd_used != 0", sock.fd_used != 0);
    return 0;
  }

  sock.fd_used += 1;
  LWIP_ASSERT("sock.fd_used != 0", sock.fd_used != 0);
  return 1;
}

/* In full-duplex mode,sock.fd_used != 0 prevents a socket descriptor from being
 * released (and possibly reused) when used from more than one thread
 * (e.g. read-while-write or close-while-write, etc)
 * This function is called at the end of functions using (try)get_socket*().
 */
pub fn
done_socket(sock: &mut lwip_sock)
{
  let freed: i32 = 0;
  let is_tcp: i32 = 0;
   let conn: &mut netconn = NULL;
  let lastdata: lwip_sock_lastdata;
  SYS_ARCH_DECL_PROTECT(lev);
  LWIP_ASSERT("sock != NULL", sock != NULL);

  SYS_ARCH_PROTECT(lev);
  LWIP_ASSERT("sock.fd_used > 0", sock.fd_used > 0);
  if (--sock.fd_used == 0) {
    if (sock.fd_free_pending) {
      /* free the socket */
      sock.fd_used = 1;
      is_tcp = sock.fd_free_pending & LWIP_SOCK_FD_FREE_TCP;
      freed = free_socket_locked(sock, is_tcp, &conn, &lastdata);
    }
  }
  SYS_ARCH_UNPROTECT(lev);

  if (freed) {
    free_socket_free_elements(is_tcp, conn, &lastdata);
  }
}

 /* LWIP_NETCONN_FULLDUPLEX */
// #define sock_inc_used(sock)         1
// #define sock_inc_used_locked(sock)  1
// #define done_socket(sock)


/* Translate a socket 'int' into a pointer (only fails if the index is invalid) */
pub fn tryget_socket_unconn_nouse(fd: i32) -> LwipSocket
{
  let s: i32 = fd - LWIP_SOCKET_OFFSET;
  if ((s < 0) || (s >= NUM_SOCKETS)) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("tryget_socket_unconn(%d): invalid\n", fd));
    return NULL;
  }
  return &sockets[s];
}

pub fn lwip_socket_dbg_get_socket(fd: i32) -> lwip_sock
{
  return tryget_socket_unconn_nouse(fd);
}

/* Translate a socket 'int' into a pointer (only fails if the index is invalid) */
pub fn tryget_socket_unconn(fd: i32) -> lwip_sock
{
  let ret: &mut lwip_sock = tryget_socket_unconn_nouse(fd);
  if (ret != NULL) {
    if (!sock_inc_used(ret)) {
      return NULL;
    }
  }
  return ret;
}

/* Like tryget_socket_unconn(), but called under SYS_ARCH_PROTECT lock. */
pub fn tryget_socket_unconn_locked(fd: i32) -> lwip_sock
{
  let ret: &mut lwip_sock = tryget_socket_unconn_nouse(fd);
  if (ret != NULL) {
    if (!sock_inc_used_locked(ret)) {
      return NULL;
    }
  }
  return ret;
}

/*
 * Same as get_socket but doesn't set errno
 *
 * @param fd externally used socket index
 * @return struct lwip_sock for the socket or NULL if not found
 */
pub fn tryget_socket(fd: i32) -> lwip_sock
{
  let sock: &mut lwip_sock = tryget_socket_unconn(fd);
  if (sock != NULL) {
    if (sock.conn) {
      return sock;
    }
    done_socket(sock);
  }
  return NULL;
}

/*
 * Map a externally used socket index to the internal socket representation.
 *
 * @param fd externally used socket index
 * @return struct lwip_sock for the socket or NULL if not found
 */
pub fn get_socket(fd: i32) -> lwip_sock {
  let sock: &mut lwip_sock = tryget_socket(fd);
  if (!sock) {
    if ((fd < LWIP_SOCKET_OFFSET) || (fd >= (LWIP_SOCKET_OFFSET + NUM_SOCKETS))) {
      LWIP_DEBUGF(SOCKETS_DEBUG, ("get_socket(%d): invalid\n", fd));
    }
    set_errno(EBADF);
    return NULL;
  }
  return sock;
}

/*
 * Allocate a new socket for a given netconn.
 *
 * @param newconn the netconn for which to allocate a socket
 * @param accepted 1 if socket has been created by accept(),
 *                 0 if socket has been created by socket()
 * @return the index of the new socket; -1 on error
 */
pub fn alloc_socket(newconn: &mut netconn, accepted: i32) -> i32
{
  let i: i32;
  SYS_ARCH_DECL_PROTECT(lev);
  

  /* allocate a new socket identifier */
  // for (i = 0; i < NUM_SOCKETS; i += 1) {
  //   /* Protect socket array */
  //   SYS_ARCH_PROTECT(lev);
  //   if (!sockets[i].conn) {

  //     if (sockets[i].fd_used) {
  //       SYS_ARCH_UNPROTECT(lev);
  //       continue;
  //     }
  //     sockets[i].fd_used    = 1;
  //     sockets[i].fd_free_pending = 0;

  //     sockets[i].conn       = newconn;
  //     /* The socket is not yet known to anyone, so no need to protect
  //        after having marked it as used. */
  //     SYS_ARCH_UNPROTECT(lev);
  //     sockets[i].lastdata.pbuf = NULL;

  //     LWIP_ASSERT("sockets[i].select_waiting == 0", sockets[i].select_waiting == 0);
  //     sockets[i].rcvevent   = 0;
  //     /* TCP sendbuf is empty, but the socket is not yet writable until connected
  //      * (unless it has been created by accept()). */
  //     sockets[i].sendevent  = (NETCONNTYPE_GROUP(newconn.type) == NETCONN_TCP ? (accepted != 0) : 1);
  //     sockets[i].errevent   = 0;

  //     return i + LWIP_SOCKET_OFFSET;
  //   }
  //   SYS_ARCH_UNPROTECT(lev);
  // }
  return -1;
}

/* Free a socket (under lock)
 *
 * @param sock the socket to free
 * @param is_tcp != 0 for TCP sockets, used to free lastdata
 * @param conn the socekt's netconn is stored here, must be freed externally
 * @param lastdata lastdata is stored here, must be freed externally
 */
pub fn free_socket_locked(
  sock: &mut lwip_sock, 
  is_tcp: i32, 
  conn: &netconn,
  lastdata: lwip_sock_lastdata) -> i32
{

  LWIP_ASSERT("sock.fd_used > 0", sock.fd_used > 0);
  sock.fd_used -= 1;
  if (sock.fd_used > 0) {
    // sock.fd_free_pending = LWIP_SOCK_FD_FREE_FREE | (is_tcp ? LWIP_SOCK_FD_FREE_TCP : 
      // 0);
      let fd_free_pending_val = 0;
      if is_tcp {
        fd_free_pending_val = LWIP_SOCK_FD_FREE_TCP;
      }
      sock.fd_free_pending = LWIP_SOCK_FD_FREE_FREE | fd_free_pending_val;
    return 0;
  }
 /* LWIP_NETCONN_FULLDUPLEX */
  


  *lastdata = sock.lastdata;
  sock.lastdata.pbuf = NULL;
  *conn = sock.conn;
  sock.conn = NULL;
  return 1;
}

/* Free a socket's leftover members.
 */
pub fn
free_socket_free_elements(is_tcp: i32, conn: &mut netconn, lastdata: lwip_sock_lastdata)
{
  if (lastdata.pbuf != NULL) {
    if (is_tcp) {
      pbuf_free(lastdata.pbuf);
    } else {
      netbuf_delete(lastdata.netbuf);
    }
  }
  if (conn != NULL) {
    /* netconn_prepare_delete() has already been called, here we only free the conn */
    netconn_delete(conn);
  }
}

/* Free a socket. The socket's netconn must have been
 * delete before!
 *
 * @param sock the socket to free
 * @param is_tcp != 0 for TCP sockets, used to free lastdata
 */
pub fn
free_socket(sock: &mut lwip_sock, is_tcp: i32)
{
  freed: i32;
   let conn: &mut netconn;
  union lwip_sock_lastdata lastdata;
  SYS_ARCH_DECL_PROTECT(lev);

  /* Protect socket array */
  SYS_ARCH_PROTECT(lev);

  freed = free_socket_locked(sock, is_tcp, &conn, &lastdata);
  SYS_ARCH_UNPROTECT(lev);
  /* don't use 'sock' after this line, as another task might have allocated it */

  if (freed) {
    free_socket_free_elements(is_tcp, conn, &lastdata);
  }
}

/* Below this, the well-known socket functions are implemented.
 * Use google.com or opengroup.org to get a good description :-)
 *
 * Exceptions are documented!
 */

pub fn lwip_accept(s: i32, addr: &mut sockaddr, socklen_t *addrlen)
{
  sock: &mut lwip_sock, *nsock;
  newconn: &mut netconn;
  ip_addr_t naddr;
  port: u16 = 0;
  newsock: i32;
  let err: err_t;
  recvevent: i32;
  SYS_ARCH_DECL_PROTECT(lev);

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_accept(%d)...\n", s));
  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  /* wait for a new connection */
  err = netconn_accept(sock.conn, &newconn);
  if (err != ERR_OK) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_accept(%d): netconn_acept failed, err=%d\n", s, err));
    if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_TCP) {
      sock_set_errno(sock, EOPNOTSUPP);
    } else if (err == ERR_CLSD) {
      sock_set_errno(sock, EINVAL);
    } else {
      sock_set_errno(sock, err_to_errno(err));
    }
    done_socket(sock);
    return -1;
  }
  LWIP_ASSERT("newconn != NULL", newconn != NULL);

  newsock = alloc_socket(newconn, 1);
  if (newsock == -1) {
    netconn_delete(newconn);
    sock_set_errno(sock, ENFILE);
    done_socket(sock);
    return -1;
  }
  LWIP_ASSERT("invalid socket index", (newsock >= LWIP_SOCKET_OFFSET) && (newsock < NUM_SOCKETS + LWIP_SOCKET_OFFSET));
  nsock = &sockets[newsock - LWIP_SOCKET_OFFSET];

  /* See event_callback: If data comes in right away after an accept, even
   * though the server task might not have created a new socket yet.
   * In that case, newconn.socket is counted down (newconn.socket--),
   * so nsock.rcvevent is >= 1 here!
   */
  SYS_ARCH_PROTECT(lev);
  recvevent = (i16)(-1 - newconn.socket);
  newconn.socket = newsock;
  SYS_ARCH_UNPROTECT(lev);

  if (newconn.callback) {
    LOCK_TCPIP_CORE();
    while (recvevent > 0) {
      recvevent -= 1;
      newconn.callback(newconn, NETCONN_EVT_RCVPLUS, 0);
    }
    UNLOCK_TCPIP_CORE();
  }

  /* Note that POSIX only requires us to check addr is non-NULL. addrlen must
   * not be NULL if addr is valid.
   */
  if ((addr != NULL) && (addrlen != NULL)) {
    union sockaddr_aligned tempaddr;
    /* get the IP address and port of the remote host */
    err = netconn_peer(newconn, &naddr, &port);
    if (err != ERR_OK) {
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_accept(%d): netconn_peer failed, err=%d\n", s, err));
      netconn_delete(newconn);
      free_socket(nsock, 1);
      sock_set_errno(sock, err_to_errno(err));
      done_socket(sock);
      return -1;
    }

    IPADDR_PORT_TO_SOCKADDR(&tempaddr, &naddr, port);
    if (*addrlen > tempaddr.sa.sa_len) {
      *addrlen = tempaddr.sa.sa_len;
    }
    MEMCPY(addr, &tempaddr, *addrlen);

    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_accept(%d) returning new sock=%d addr=", s, newsock));
    ip_addr_debug_print_val(SOCKETS_DEBUG, naddr);
    LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F"\n", port));
  } else {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_accept(%d) returning new sock=%d", s, newsock));
  }

  sock_set_errno(sock, 0);
  done_socket(sock);
  done_socket(nsock);
  return newsock;
}

pub fn lwip_bind(s: i32,  name: &mut sockaddr, namelen: socklen_t)
{
  sock: &mut lwip_sock;
  ip_addr_t local_addr;
  local_port: u16;
  let err: err_t;

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (!SOCK_ADDR_TYPE_MATCH(name, sock)) {
    /* sockaddr does not match socket type (IPv4/IPv6) */
    sock_set_errno(sock, err_to_errno(ERR_VAL));
    done_socket(sock);
    return -1;
  }

  /* check size, family and alignment of 'name' */
  LWIP_ERROR("lwip_bind: invalid address", (IS_SOCK_ADDR_LEN_VALID(namelen) &&
             IS_SOCK_ADDR_TYPE_VALID(name) && IS_SOCK_ADDR_ALIGNED(name)),
             sock_set_errno(sock, err_to_errno(ERR_ARG)); done_socket(sock); return -1;);
  

  SOCKADDR_TO_IPADDR_PORT(name, &local_addr, local_port);
  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_bind(%d, addr=", s));
  ip_addr_debug_print_val(SOCKETS_DEBUG, local_addr);
  LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F")\n", local_port));


  /* Dual-stack: Unmap IPv4 mapped IPv6 addresses */
  if (IP_IS_V6_VAL(local_addr) && ip6_addr_isipv4mappedipv6(ip_2_ip6(&local_addr))) {
    unmap_ipv4_mapped_ipv6(ip_2_ip4(&local_addr), ip_2_ip6(&local_addr));
    IP_SET_TYPE_VAL(local_addr, IPADDR_TYPE_V4);
  }


  err = netconn_bind(sock.conn, &local_addr, local_port);

  if (err != ERR_OK) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_bind(%d) failed, err=%d\n", s, err));
    sock_set_errno(sock, err_to_errno(err));
    done_socket(sock);
    return -1;
  }

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_bind(%d) succeeded\n", s));
  sock_set_errno(sock, 0);
  done_socket(sock);
  return 0;
}

pub fn lwip_close(s: i32)
{
  sock: &mut lwip_sock;
  is_tcp: i32 = 0;
  let err: err_t;

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_close(%d)\n", s));

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (sock.conn != NULL) {
    is_tcp = NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP;
  } else {
    LWIP_ASSERT("sock.lastdata == NULL", sock.lastdata.pbuf == NULL);
  }


  /* drop all possibly joined IGMP memberships */
  lwip_socket_drop_registered_memberships(s);


  /* drop all possibly joined MLD6 memberships */
  lwip_socket_drop_registered_mld6_memberships(s);


  err = netconn_prepare_delete(sock.conn);
  if (err != ERR_OK) {
    sock_set_errno(sock, err_to_errno(err));
    done_socket(sock);
    return -1;
  }

  free_socket(sock, is_tcp);
  set_errno(0);
  return 0;
}

pub fn lwip_connect(s: i32,  name: &mut sockaddr, namelen: socklen_t)
{
  sock: &mut lwip_sock;
  let err: err_t;

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (!SOCK_ADDR_TYPE_MATCH_OR_UNSPEC(name, sock)) {
    /* sockaddr does not match socket type (IPv4/IPv6) */
    sock_set_errno(sock, err_to_errno(ERR_VAL));
    done_socket(sock);
    return -1;
  }

  
  if (name.sa_family == AF_UNSPEC) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_connect(%d, AF_UNSPEC)\n", s));
    err = netconn_disconnect(sock.conn);
  } else {
    ip_addr_t remote_addr;
    remote_port: u16;

    /* check size, family and alignment of 'name' */
    LWIP_ERROR("lwip_connect: invalid address", IS_SOCK_ADDR_LEN_VALID(namelen) &&
               IS_SOCK_ADDR_TYPE_VALID_OR_UNSPEC(name) && IS_SOCK_ADDR_ALIGNED(name),
               sock_set_errno(sock, err_to_errno(ERR_ARG)); done_socket(sock); return -1;);

    SOCKADDR_TO_IPADDR_PORT(name, &remote_addr, remote_port);
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_connect(%d, addr=", s));
    ip_addr_debug_print_val(SOCKETS_DEBUG, remote_addr);
    LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F")\n", remote_port));


    /* Dual-stack: Unmap IPv4 mapped IPv6 addresses */
    if (IP_IS_V6_VAL(remote_addr) && ip6_addr_isipv4mappedipv6(ip_2_ip6(&remote_addr))) {
      unmap_ipv4_mapped_ipv6(ip_2_ip4(&remote_addr), ip_2_ip6(&remote_addr));
      IP_SET_TYPE_VAL(remote_addr, IPADDR_TYPE_V4);
    }


    err = netconn_connect(sock.conn, &remote_addr, remote_port);
  }

  if (err != ERR_OK) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_connect(%d) failed, err=%d\n", s, err));
    sock_set_errno(sock, err_to_errno(err));
    done_socket(sock);
    return -1;
  }

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_connect(%d) succeeded\n", s));
  sock_set_errno(sock, 0);
  done_socket(sock);
  return 0;
}

/*
 * Set a socket into listen mode.
 * The socket may not have been used for another connection previously.
 *
 * @param s the socket to set to listening mode
 * @param backlog (ATTENTION: needs TCP_LISTEN_BACKLOG=1)
 * @return 0 on success, non-zero on failure
 */
pub fn lwip_listen(s: i32, backlog: i32)
{
  sock: &mut lwip_sock;
  let err: err_t;

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_listen(%d, backlog=%d)\n", s, backlog));

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  /* limit the "backlog" parameter to fit in an u8 */
  backlog = LWIP_MIN(LWIP_MAX(backlog, 0), 0xff);

  err = netconn_listen_with_backlog(sock.conn, backlog);

  if (err != ERR_OK) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_listen(%d) failed, err=%d\n", s, err));
    if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_TCP) {
      sock_set_errno(sock, EOPNOTSUPP);
    } else {
      sock_set_errno(sock, err_to_errno(err));
    }
    done_socket(sock);
    return -1;
  }

  sock_set_errno(sock, 0);
  done_socket(sock);
  return 0;
}


/* Helper function to loop over receiving pbufs from netconn
 * until "len" bytes are received or we're otherwise done.
 * Keeps sock.lastdata for peeking or partly copying.
 */
static isize
lwip_recv_tcp(sock: &mut lwip_sock, mem: &mut (), len: usize, flags: i32)
{
  apiflags: u8 = NETCONN_NOAUTORCVD;
  isize recvd = 0;
  isize recv_left = (len <= SSIZE_MAX) ? (isize)len : SSIZE_MAX;

  LWIP_ASSERT("no socket given", sock != NULL);
  LWIP_ASSERT("this should be checked internally", NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP);

  if (flags & MSG_DONTWAIT) {
    apiflags |= NETCONN_DONTBLOCK;
  }

  loop {
    p: &mut pbuf;
    let err: err_t;
    copylen: u16;

    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recv_tcp: top while sock.lastdata=%p\n", sock.lastdata.pbuf));
    /* Check if there is data left from the last recv operation. */
    if (sock.lastdata.pbuf) {
      p = sock.lastdata.pbuf;
    } else {
      /* No data was left from the previous operation, so we try to get
         some from the network. */
      err = netconn_recv_tcp_pbuf_flags(sock.conn, &p, apiflags);
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recv_tcp: netconn_recv err=%d, pbuf=%p\n",
                                  err, p));

      if (err != ERR_OK) {
        if (recvd > 0) {
          /* already received data, return that (this trusts in getting the same error from
             netconn layer again next time netconn_recv is called) */
          // goto lwip_recv_tcp_done;
        }
        /* We should really do some error checking here. */
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recv_tcp: p == NULL, error is \"%s\"!\n",
                                    lwip_strerr(err)));
        sock_set_errno(sock, err_to_errno(err));
        if (err == ERR_CLSD) {
          return 0;
        } else {
          return -1;
        }
      }
      LWIP_ASSERT("p != NULL", p != NULL);
      sock.lastdata.pbuf = p;
    }

    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recv_tcp: buflen=%"U16_F" recv_left=%d off=%d\n",
                                p.tot_len, recv_left, recvd));

    if (recv_left > p.tot_len) {
      copylen = p.tot_len;
    } else {
      copylen = recv_left;
    }
    if (recvd + copylen < recvd) {
      /* overflow */
      copylen = (SSIZE_MAX - recvd);
    }

    /* copy the contents of the received buffer into
    the supplied memory pointer mem */
    pbuf_copy_partial(p, mem + recvd, copylen, 0);

    recvd += copylen;

    /* TCP combines multiple pbufs for one recv */
    LWIP_ASSERT("invalid copylen, len would underflow", recv_left >= copylen);
    recv_left -= copylen;

    /* Unless we peek the incoming message... */
    if ((flags & MSG_PEEK) == 0) {
      /* ... check if there is data left in the pbuf */
      LWIP_ASSERT("invalid copylen", p.tot_len >= copylen);
      if (p.tot_len - copylen > 0) {
        /* If so, it should be saved in the sock structure for the next recv call.
           We store the pbuf but hide/free the consumed data: */
        sock.lastdata.pbuf = pbuf_free_header(p, copylen);
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recv_tcp: lastdata now pbuf=%p\n", sock.lastdata.pbuf));
      } else {
        sock.lastdata.pbuf = NULL;
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recv_tcp: deleting pbuf=%p\n", p));
        pbuf_free(p);
      }
    }
    /* once we have some data to return, only add more if we don't need to wait */
    apiflags |= NETCONN_DONTBLOCK | NETCONN_NOFIN;
    /* @todo: do we need to support peeking more than one pbuf? */
  } while ((recv_left > 0) && !(flags & MSG_PEEK));
lwip_recv_tcp_done:
  if ((recvd > 0) && !(flags & MSG_PEEK)) {
    /* ensure window update after copying all data */
    netconn_tcp_recvd(sock.conn, recvd);
  }
  sock_set_errno(sock, 0);
  return recvd;
}


/* Convert a netbuf's address data to struct sockaddr */
static int
lwip_sock_make_addr(conn: &mut netconn, fromaddr: &mut ip_addr_t, port: u16,
                    from: &mut sockaddr, socklen_t *fromlen)
{
  truncated: i32 = 0;
  union sockaddr_aligned saddr;

  

  LWIP_ASSERT("fromaddr != NULL", fromaddr != NULL);
  LWIP_ASSERT("from != NULL", from != NULL);
  LWIP_ASSERT("fromlen != NULL", fromlen != NULL);


  /* Dual-stack: Map IPv4 addresses to IPv4 mapped IPv6 */
  if (NETCONNTYPE_ISIPV6(netconn_type(conn)) && IP_IS_V4(fromaddr)) {
    ip4_2_ipv4_mapped_ipv6(ip_2_ip6(fromaddr), ip_2_ip4(fromaddr));
    IP_SET_TYPE(fromaddr, IPADDR_TYPE_V6);
  }


  IPADDR_PORT_TO_SOCKADDR(&saddr, fromaddr, port);
  if (*fromlen < saddr.sa.sa_len) {
    truncated = 1;
  } else if (*fromlen > saddr.sa.sa_len) {
    *fromlen = saddr.sa.sa_len;
  }
  MEMCPY(from, &saddr, *fromlen);
  return truncated;
}


/* Helper function to get a tcp socket's remote address info */
static int
lwip_recv_tcp_from(sock: &mut lwip_sock, from: &mut sockaddr, socklen_t *fromlen, dbg_fn: &String, dbg_s: i32, isize dbg_ret)
{
  if (sock == NULL) {
    return 0;
  }
  
  
  


  if (from && fromlen)

  {
    /* get remote addr/port from tcp_pcb */
    port: u16;
    ip_addr_t tmpaddr;
    netconn_getaddr(sock.conn, &tmpaddr, &port, 0);
    LWIP_DEBUGF(SOCKETS_DEBUG, ("%s(%d):  addr=", dbg_fn, dbg_s));
    ip_addr_debug_print_val(SOCKETS_DEBUG, tmpaddr);
    LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F" len=%d\n", port, dbg_ret));
    if (from && fromlen) {
      return lwip_sock_make_addr(sock.conn, &tmpaddr, port, from, fromlen);
    }
  }
  return 0;
}


/* Helper function to receive a netbuf from a udp or raw netconn.
 * Keeps sock.lastdata for peeking.
 */
pub fn lwip_recvfrom_udp_raw(sock: &mut lwip_sock, flags: i32, msg: &mut msghdr, datagram_len: &mut u16, dbg_s: i32) -> Result<(), LwipError>
{
  buf: &mut netbuf;
  apiflags: u8;
  let err: err_t;
  buflen: u16, copylen, copied;
  i: i32;

  
  LWIP_ERROR("lwip_recvfrom_udp_raw: invalid arguments", (msg.msg_iov != NULL) || (msg.msg_iovlen <= 0), return ERR_ARG;);

  if (flags & MSG_DONTWAIT) {
    apiflags = NETCONN_DONTBLOCK;
  } else {
    apiflags = 0;
  }

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvfrom_udp_raw[UDP/RAW]: top sock.lastdata=%p\n", sock.lastdata.netbuf));
  /* Check if there is data left from the last recv operation. */
  buf = sock.lastdata.netbuf;
  if (buf == NULL) {
    /* No data was left from the previous operation, so we try to get
        some from the network. */
    err = netconn_recv_udp_raw_netbuf_flags(sock.conn, &buf, apiflags);
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvfrom_udp_raw[UDP/RAW]: netconn_recv err=%d, netbuf=%p\n",
                                err, buf));

    if (err != ERR_OK) {
      return err;
    }
    LWIP_ASSERT("buf != NULL", buf != NULL);
    sock.lastdata.netbuf = buf;
  }
  buflen = buf.p.tot_len;
  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvfrom_udp_raw: buflen=%"U16_F"\n", buflen));

  copied = 0;
  /* copy the pbuf payload into the iovs */
  for (i = 0; (i < msg.msg_iovlen) && (copied < buflen); i+= 1) {
    len_left: u16 = (buflen - copied);
    if (msg.msg_iov[i].iov_len > len_left) {
      copylen = len_left;
    } else {
      copylen = msg.msg_iov[i].iov_len;
    }

    /* copy the contents of the received buffer into
        the supplied memory buffer */
    pbuf_copy_partial(buf.p, msg.msg_iov[i].iov_base, copylen, copied);
    copied = (copied + copylen);
  }

  /* Check to see from where the data was.*/

  if (msg.msg_name && msg.msg_namelen)

  {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvfrom_udp_raw(%d):  addr=", dbg_s));
    ip_addr_debug_print_val(SOCKETS_DEBUG, *netbuf_fromaddr(buf));
    LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F" len=%d\n", netbuf_fromport(buf), copied));
    if (msg.msg_name && msg.msg_namelen) {
      lwip_sock_make_addr(sock.conn, netbuf_fromaddr(buf), netbuf_fromport(buf),
                          msg.msg_name, &msg.msg_namelen);
    }
  }

  /* Initialize flag output */
  msg.msg_flags = 0;

  if (msg.msg_control) {
    wrote_msg: u8 = 0;

    /* Check if packet info was recorded */
    if (buf.flags & NETBUF_FLAG_DESTADDR) {
      if (IP_IS_V4(&buf.toaddr)) {

        if (msg.msg_controllen >= CMSG_SPACE(sizeof(struct in_pktinfo))) {
          chdr: &mut cmsghdr = CMSG_FIRSTHDR(msg); /* This will always return a header!! */
          pkti: &mut in_pktinfo = (struct in_pktinfo *)CMSG_DATA(chdr);
          chdr.cmsg_level = IPPROTO_IP;
          chdr.cmsg_type = IP_PKTINFO;
          chdr.cmsg_len = CMSG_LEN(sizeof(struct in_pktinfo));
          pkti.ipi_ifindex = buf.p.if_idx;
          inet_addr_from_ip4addr(&pkti.ipi_addr, ip_2_ip4(netbuf_destaddr(buf)));
          msg.msg_controllen = CMSG_SPACE(sizeof(struct in_pktinfo));
          wrote_msg = 1;
        } else {
          msg.msg_flags |= MSG_CTRUNC;
        }

      }
    }


    if (!wrote_msg) {
      msg.msg_controllen = 0;
    }
  }

  /* If we don't peek the incoming message: zero lastdata pointer and free the netbuf */
  if ((flags & MSG_PEEK) == 0) {
    sock.lastdata.netbuf = NULL;
    netbuf_delete(buf);
  }
  if (datagram_len) {
    *datagram_len = buflen;
  }
  return ERR_OK;
}

isize
lwip_recvfrom(s: i32, mem: &mut (), len: usize, flags: i32,
              from: &mut sockaddr, socklen_t *fromlen)
{
  sock: &mut lwip_sock;
  sret: usize;

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvfrom(%d, %p, %"SZT_F", 0x%x, ..)\n", s, mem, len, flags));
  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP) {
    ret = lwip_recv_tcp(sock, mem, len, flags);
    lwip_recv_tcp_from(sock, from, fromlen, "lwip_recvfrom", s, ret);
    done_socket(sock);
    return ret;
  } else

  {
    datagram_len: u16 = 0;
    struct iovec vec;
    struct msghdr msg;
    let err: err_t;
    vec.iov_base = mem;
    vec.iov_len = len;
    msg.msg_control = NULL;
    msg.msg_controllen = 0;
    msg.msg_flags = 0;
    msg.msg_iov = &vec;
    msg.msg_iovlen = 1;
    msg.msg_name = from;
    msg.msg_namelen = (fromlen ? *fromlen : 0);
    err = lwip_recvfrom_udp_raw(sock, flags, &msg, &datagram_len, s);
    if (err != ERR_OK) {
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvfrom[UDP/RAW](%d): buf == NULL, error is \"%s\"!\n",
                                  s, lwip_strerr(err)));
      sock_set_errno(sock, err_to_errno(err));
      done_socket(sock);
      return -1;
    }
    ret = (isize)LWIP_MIN(LWIP_MIN(len, datagram_len), SSIZE_MAX);
    if (fromlen) {
      *fromlen = msg.msg_namelen;
    }
  }

  sock_set_errno(sock, 0);
  done_socket(sock);
  return ret;
}

isize
lwip_read(s: i32, mem: &mut (), len: usize)
{
  return lwip_recvfrom(s, mem, len, 0, NULL, NULL);
}

isize
lwip_readv(s: i32,  iov: &mut iovec, iovcnt: i32)
{
  struct msghdr msg;

  msg.msg_name = NULL;
  msg.msg_namelen = 0;
  /* Hack: we have to cast via number to cast from 'const' pointer to non-const.
     Blame the opengroup standard for this inconsistency. */
  msg.msg_iov = LWIP_CONST_CAST(struct iovec *, iov);
  msg.msg_iovlen = iovcnt;
  msg.msg_control = NULL;
  msg.msg_controllen = 0;
  msg.msg_flags = 0;
  return lwip_recvmsg(s, &msg, 0);
}

isize
lwip_recv(s: i32, mem: &mut (), len: usize, flags: i32)
{
  return lwip_recvfrom(s, mem, len, flags, NULL, NULL);
}

isize
lwip_recvmsg(s: i32, message: &mut msghdr, flags: i32)
{
  sock: &mut lwip_sock;
  i: i32;
  sbuflen: usize;

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvmsg(%d, message=%p, flags=0x%x)\n", s, message, flags));
  LWIP_ERROR("lwip_recvmsg: invalid message pointer", message != NULL, return ERR_ARG;);
  LWIP_ERROR("lwip_recvmsg: unsupported flags", (flags & ~(MSG_PEEK|MSG_DONTWAIT)) == 0,
             set_errno(EOPNOTSUPP); return -1;);

  if ((message.msg_iovlen <= 0) || (message.msg_iovlen > IOV_MAX)) {
    set_errno(EMSGSIZE);
    return -1;
  }

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  /* check for valid vectors */
  buflen = 0;
  for (i = 0; i < message.msg_iovlen; i+= 1) {
    if ((message.msg_iov[i].iov_base == NULL) || ((isize)message.msg_iov[i].iov_len <= 0) ||
        ((isize)message.msg_iov[i].iov_len != message.msg_iov[i].iov_len) ||
        ((isize)(buflen + (isize)message.msg_iov[i].iov_len) <= 0)) {
      sock_set_errno(sock, err_to_errno(ERR_VAL));
      done_socket(sock);
      return -1;
    }
    buflen = (isize)(buflen + (isize)message.msg_iov[i].iov_len);
  }

  if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP) {

    recv_flags: i32 = flags;
    message.msg_flags = 0;
    /* recv the data */
    buflen = 0;
    for (i = 0; i < message.msg_iovlen; i+= 1) {
      /* try to receive into this vector's buffer */
      isize recvd_local = lwip_recv_tcp(sock, message.msg_iov[i].iov_base, message.msg_iov[i].iov_len, recv_flags);
      if (recvd_local > 0) {
        /* sum up received bytes */
        buflen += recvd_local;
      }
      if ((recvd_local < 0) || (recvd_local < message.msg_iov[i].iov_len) ||
          (flags & MSG_PEEK)) {
        /* returned prematurely (or peeking, which might actually be limitated to the first iov) */
        if (buflen <= 0) {
          /* nothing received at all, propagate the error */
          buflen = recvd_local;
        }
        break;
      }
      /* pass MSG_DONTWAIT to lwip_recv_tcp() to prevent waiting for more data */
      recv_flags |= MSG_DONTWAIT;
    }
    if (buflen > 0) {
      /* reset socket error since we have received something */
      sock_set_errno(sock, 0);
    }
    /* " If the socket is connected, the msg_name and msg_namelen members shall be ignored." */
    done_socket(sock);
    return buflen;
 /* LWIP_TCP */
    sock_set_errno(sock, err_to_errno(ERR_ARG));
    done_socket(sock);
    return -1;

  }
  /* else, UDP and RAW NETCONNs */

  {
    datagram_len: u16 = 0;
    let err: err_t;
    err = lwip_recvfrom_udp_raw(sock, flags, message, &datagram_len, s);
    if (err != ERR_OK) {
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_recvmsg[UDP/RAW](%d): buf == NULL, error is \"%s\"!\n",
                                  s, lwip_strerr(err)));
      sock_set_errno(sock, err_to_errno(err));
      done_socket(sock);
      return -1;
    }
    if (datagram_len > buflen) {
      message.msg_flags |= MSG_TRUNC;
    }

    sock_set_errno(sock, 0);
    done_socket(sock);
    return datagram_len;
  }
 /* LWIP_UDP || LWIP_RAW */
  sock_set_errno(sock, err_to_errno(ERR_ARG));
  done_socket(sock);
  return -1;

}

isize
lwip_send(s: i32, data: &Vec<u8>, size: usize, flags: i32)
{
  sock: &mut lwip_sock;
  let err: err_t;
  write_flags: u8;
  written: usize;

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_send(%d, data=%p, size=%"SZT_F", flags=0x%x)\n",
                              s, data, size, flags));

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_TCP) {

    done_socket(sock);
    return lwip_sendto(s, data, size, flags, NULL, 0);
 /* (LWIP_UDP || LWIP_RAW) */
    sock_set_errno(sock, err_to_errno(ERR_ARG));
    done_socket(sock);
    return -1;

  }

  write_flags = (NETCONN_COPY |
                       ((flags & MSG_MORE)     ? NETCONN_MORE      : 0) |
                       ((flags & MSG_DONTWAIT) ? NETCONN_DONTBLOCK : 0));
  written = 0;
  err = netconn_write_partly(sock.conn, data, size, write_flags, &written);

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_send(%d) err=%d written=%"SZT_F"\n", s, err, written));
  sock_set_errno(sock, err_to_errno(err));
  done_socket(sock);
  /* casting 'written' to isize is OK here since the netconn API limits it to SSIZE_MAX */
  return (err == ERR_OK ? (isize)written : -1);
}

isize
lwip_sendmsg(s: i32,  msg: &mut msghdr, flags: i32)
{
  sock: &mut lwip_sock;

  write_flags: u8;
  written: usize;

  err: err_t = ERR_OK;

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  LWIP_ERROR("lwip_sendmsg: invalid msghdr", msg != NULL,
             sock_set_errno(sock, err_to_errno(ERR_ARG)); done_socket(sock); return -1;);
  LWIP_ERROR("lwip_sendmsg: invalid msghdr iov", msg.msg_iov != NULL,
             sock_set_errno(sock, err_to_errno(ERR_ARG)); done_socket(sock); return -1;);
  LWIP_ERROR("lwip_sendmsg: maximum iovs exceeded", (msg.msg_iovlen > 0) && (msg.msg_iovlen <= IOV_MAX),
             sock_set_errno(sock, EMSGSIZE); done_socket(sock); return -1;);
  LWIP_ERROR("lwip_sendmsg: unsupported flags", (flags & ~(MSG_DONTWAIT | MSG_MORE)) == 0,
             sock_set_errno(sock, EOPNOTSUPP); done_socket(sock); return -1;);

  LWIP_UNUSED_ARG(msg.msg_control);
  LWIP_UNUSED_ARG(msg.msg_controllen);
  LWIP_UNUSED_ARG(msg.msg_flags);

  if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP) {

    write_flags = (NETCONN_COPY |
                         ((flags & MSG_MORE)     ? NETCONN_MORE      : 0) |
                         ((flags & MSG_DONTWAIT) ? NETCONN_DONTBLOCK : 0));

    written = 0;
    err = netconn_write_vectors_partly(sock.conn, (struct netvector *)msg.msg_iov, msg.msg_iovlen, write_flags, &written);
    sock_set_errno(sock, err_to_errno(err));
    done_socket(sock);
    /* casting 'written' to isize is OK here since the netconn API limits it to SSIZE_MAX */
    return (err == ERR_OK ? (isize)written : -1);
 /* LWIP_TCP */
    sock_set_errno(sock, err_to_errno(ERR_ARG));
    done_socket(sock);
    return -1;

  }
  /* else, UDP and RAW NETCONNs */

  {
    struct netbuf chain_buf;
    i: i32;
    isize size = 0;

    
    LWIP_ERROR("lwip_sendmsg: invalid msghdr name", (((msg.msg_name == NULL) && (msg.msg_namelen == 0)) ||
               IS_SOCK_ADDR_LEN_VALID(msg.msg_namelen)),
               sock_set_errno(sock, err_to_errno(ERR_ARG)); done_socket(sock); return -1;);

    /* initialize chain buffer with destination */
    memset(&chain_buf, 0, sizeof(struct netbuf));
    if (msg.msg_name) {
      remote_port: u16;
      SOCKADDR_TO_IPADDR_PORT((const struct sockaddr *)msg.msg_name, &chain_buf.addr, remote_port);
      netbuf_fromport(&chain_buf) = remote_port;
    }

    for (i = 0; i < msg.msg_iovlen; i+= 1) {
      size += msg.msg_iov[i].iov_len;
      if ((msg.msg_iov[i].iov_len > INT_MAX) || (size < msg.msg_iov[i].iov_len)) {
        /* overflow */
        // goto sendmsg_emsgsize;
      }
    }
    if (size > 0xFFFF) {
      /* overflow */
      // goto sendmsg_emsgsize;
    }
    /* Allocate a new netbuf and copy the data into it. */
    if (netbuf_alloc(&chain_buf, size) == NULL) {
      err = ERR_MEM;
    } else {
      /* flatten the IO vectors */
      offset: usize = 0;
      for (i = 0; i < msg.msg_iovlen; i+= 1) {
        MEMCPY(&(chain_buf.p.payload)[offset], msg.msg_iov[i].iov_base, msg.msg_iov[i].iov_len);
        offset += msg.msg_iov[i].iov_len;
      }

      {
        /* This can be improved by using LWIP_CHKSUM_COPY() and aggregating the checksum for each IO vector */
        chksum: u16 = ~inet_chksum_pbuf(chain_buf.p);
        netbuf_set_chksum(&chain_buf, chksum);
      }

      err = ERR_OK;
    }
 /* LWIP_NETIF_TX_SINGLE_PBUF */
    /* create a chained netbuf from the IO vectors. NOTE: we assemble a pbuf chain
       manually to avoid having to allocate, chain, and delete a netbuf for each iov */
    for (i = 0; i < msg.msg_iovlen; i+= 1) {
      p: &mut pbuf;
      if (msg.msg_iov[i].iov_len > 0xFFFF) {
        /* overflow */
        // goto sendmsg_emsgsize;
      }
      p = pbuf_alloc(PBUF_TRANSPORT, 0, PBUF_REF);
      if (p == NULL) {
        err = ERR_MEM; /* let netbuf_delete() cleanup chain_buf */
        break;
      }
      p.payload = msg.msg_iov[i].iov_base;
      p.len = p.tot_len = msg.msg_iov[i].iov_len;
      /* netbuf empty, add new pbuf */
      if (chain_buf.p == NULL) {
        chain_buf.p = chain_buf.ptr = p;
        /* add pbuf to existing pbuf chain */
      } else {
        if (chain_buf.p.tot_len + p.len > 0xffff) {
          /* overflow */
          pbuf_free(p);
          // goto sendmsg_emsgsize;
        }
        pbuf_cat(chain_buf.p, p);
      }
    }
    /* save size of total chain */
    if (err == ERR_OK) {
      size = netbuf_len(&chain_buf);
    }


    if (err == ERR_OK) {

      /* Dual-stack: Unmap IPv4 mapped IPv6 addresses */
      if (IP_IS_V6_VAL(chain_buf.addr) && ip6_addr_isipv4mappedipv6(ip_2_ip6(&chain_buf.addr))) {
        unmap_ipv4_mapped_ipv6(ip_2_ip4(&chain_buf.addr), ip_2_ip6(&chain_buf.addr));
        IP_SET_TYPE_VAL(chain_buf.addr, IPADDR_TYPE_V4);
      }


      /* send the data */
      err = netconn_send(sock.conn, &chain_buf);
    }

    /* deallocated the buffer */
    netbuf_free(&chain_buf);

    sock_set_errno(sock, err_to_errno(err));
    done_socket(sock);
    return (err == ERR_OK ? size : -1);
sendmsg_emsgsize:
    sock_set_errno(sock, EMSGSIZE);
    netbuf_free(&chain_buf);
    done_socket(sock);
    return -1;
  }
 /* LWIP_UDP || LWIP_RAW */
  sock_set_errno(sock, err_to_errno(ERR_ARG));
  done_socket(sock);
  return -1;

}

isize
lwip_sendto(s: i32, data: &Vec<u8>, size: usize, flags: i32,
            const to: &mut sockaddr, tolen: socklen_t)
{
  sock: &mut lwip_sock;
  let err: err_t;
  short_size: u16;
  remote_port: u16;
  struct netbuf buf;

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP) {

    done_socket(sock);
    return lwip_send(s, data, size, flags);
 /* LWIP_TCP */
    
    sock_set_errno(sock, err_to_errno(ERR_ARG));
    done_socket(sock);
    return -1;

  }

  if (size > LWIP_MIN(0xFFFF, SSIZE_MAX)) {
    /* cannot fit into one datagram (at least for us) */
    sock_set_errno(sock, EMSGSIZE);
    done_socket(sock);
    return -1;
  }
  short_size = size;
  LWIP_ERROR("lwip_sendto: invalid address", (((to == NULL) && (tolen == 0)) ||
             (IS_SOCK_ADDR_LEN_VALID(tolen) &&
              ((to != NULL) && (IS_SOCK_ADDR_TYPE_VALID(to) && IS_SOCK_ADDR_ALIGNED(to))))),
             sock_set_errno(sock, err_to_errno(ERR_ARG)); done_socket(sock); return -1;);
  

  /* initialize a buffer */
  buf.p = buf.ptr = NULL;

  buf.flags = 0;

  if (to) {
    SOCKADDR_TO_IPADDR_PORT(to, &buf.addr, remote_port);
  } else {
    remote_port = 0;
    ip_addr_set_any(NETCONNTYPE_ISIPV6(netconn_type(sock.conn)), &buf.addr);
  }
  netbuf_fromport(&buf) = remote_port;


  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_sendto(%d, data=%p, short_size=%"U16_F", flags=0x%x to=",
                              s, data, short_size, flags));
  ip_addr_debug_print_val(SOCKETS_DEBUG, buf.addr);
  LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F"\n", remote_port));

  /* make the buffer poto: i32 the data that should be sent */

  /* Allocate a new netbuf and copy the data into it. */
  if (netbuf_alloc(&buf, short_size) == NULL) {
    err = ERR_MEM;
  } else {

    if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_RAW) {
      chksum: u16 = LWIP_CHKSUM_COPY(buf.p.payload, data, short_size);
      netbuf_set_chksum(&buf, chksum);
    } else

    {
      MEMCPY(buf.p.payload, data, short_size);
    }
    err = ERR_OK;
  }
 /* LWIP_NETIF_TX_SINGLE_PBUF */
  err = netbuf_ref(&buf, data, short_size);

  if (err == ERR_OK) {

    /* Dual-stack: Unmap IPv4 mapped IPv6 addresses */
    if (IP_IS_V6_VAL(buf.addr) && ip6_addr_isipv4mappedipv6(ip_2_ip6(&buf.addr))) {
      unmap_ipv4_mapped_ipv6(ip_2_ip4(&buf.addr), ip_2_ip6(&buf.addr));
      IP_SET_TYPE_VAL(buf.addr, IPADDR_TYPE_V4);
    }


    /* send the data */
    err = netconn_send(sock.conn, &buf);
  }

  /* deallocated the buffer */
  netbuf_free(&buf);

  sock_set_errno(sock, err_to_errno(err));
  done_socket(sock);
  return (err == ERR_OK ? short_size : -1);
}

pub fn lwip_socket(domain: i32, type: i32, protocol: i32)
{
   let conn: &mut netconn;
  i: i32;

   /* @todo: check this */

  /* create a netconn */
  match (type) {
    SOCK_RAW =>
      conn = netconn_new_with_proto_and_callback(DOMAIN_TO_NETCONN_TYPE(domain, NETCONN_RAW),
             protocol, DEFAULT_SOCKET_EVENTCB);
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_socket(%s, SOCK_RAW, %d) = ",
                                  domain == PF_INET ? "PF_INET" : "UNKNOWN", protocol));
      break;
    SOCK_DGRAM =>
      conn = netconn_new_with_callback(DOMAIN_TO_NETCONN_TYPE(domain,
                                       ((protocol == IPPROTO_UDPLITE) ? NETCONN_UDPLITE : NETCONN_UDP)),
                                       DEFAULT_SOCKET_EVENTCB);
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_socket(%s, SOCK_DGRAM, %d) = ",
                                  domain == PF_INET ? "PF_INET" : "UNKNOWN", protocol));

      if (conn) {
        /* netconn layer enables pktinfo by default, sockets default to off */
        conn.flags &= ~NETCONN_FLAG_PKTINFO;
      }

      break;
    SOCK_STREAM =>
      conn = netconn_new_with_callback(DOMAIN_TO_NETCONN_TYPE(domain, NETCONN_TCP), DEFAULT_SOCKET_EVENTCB);
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_socket(%s, SOCK_STREAM, %d) = ",
                                  domain == PF_INET ? "PF_INET" : "UNKNOWN", protocol));
      break;
    _ =>
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_socket(%d, %d/UNKNOWN, %d) = -1\n",
                                  domain, type, protocol));
      set_errno(EINVAL);
      return -1;
  }

  if (!conn) {
    LWIP_DEBUGF(SOCKETS_DEBUG, ("-1 / ENOBUFS (could not create netconn)\n"));
    set_errno(ENOBUFS);
    return -1;
  }

  i = alloc_socket(conn, 0);

  if (i == -1) {
    netconn_delete(conn);
    set_errno(ENFILE);
    return -1;
  }
  conn.socket = i;
  done_socket(&sockets[i - LWIP_SOCKET_OFFSET]);
  LWIP_DEBUGF(SOCKETS_DEBUG, ("%d\n", i));
  set_errno(0);
  return i;
}

isize
lwip_write(s: i32, data: &Vec<u8>, size: usize)
{
  return lwip_send(s, data, size, 0);
}

isize
lwip_writev(s: i32,  iov: &mut iovec, iovcnt: i32)
{
  struct msghdr msg;

  msg.msg_name = NULL;
  msg.msg_namelen = 0;
  /* Hack: we have to cast via number to cast from 'const' pointer to non-const.
     Blame the opengroup standard for this inconsistency. */
  msg.msg_iov = LWIP_CONST_CAST(struct iovec *, iov);
  msg.msg_iovlen = iovcnt;
  msg.msg_control = NULL;
  msg.msg_controllen = 0;
  msg.msg_flags = 0;
  return lwip_sendmsg(s, &msg, 0);
}


/* Add select_cb to select_cb_list. */
pub fn
lwip_link_select_cb(select_cb: &mut lwip_select_cb)
{
  LWIP_SOCKET_SELECT_DECL_PROTECT(lev);

  /* Protect the select_cb_list */
  LWIP_SOCKET_SELECT_PROTECT(lev);

  /* Put this select_cb on top of list */
  select_cb.next = select_cb_list;
  if (select_cb_list != NULL) {
    select_cb_list.prev = select_cb;
  }
  select_cb_list = select_cb;

  /* Increasing this counter tells select_check_waiters that the list has changed. */
  select_cb_ctr+= 1;


  /* Now we can safely unprotect */
  LWIP_SOCKET_SELECT_UNPROTECT(lev);
}

/* Remove select_cb from select_cb_list. */
pub fn
lwip_unlink_select_cb(select_cb: &mut lwip_select_cb)
{
  LWIP_SOCKET_SELECT_DECL_PROTECT(lev);

  /* Take us off the list */
  LWIP_SOCKET_SELECT_PROTECT(lev);
  if (select_cb.next != NULL) {
    select_cb.next.prev = select_cb.prev;
  }
  if (select_cb_list == select_cb) {
    LWIP_ASSERT("select_cb.prev == NULL", select_cb.prev == NULL);
    select_cb_list = select_cb.next;
  } else {
    LWIP_ASSERT("select_cb.prev != NULL", select_cb.prev != NULL);
    select_cb.prev.next = select_cb.next;
  }

  /* Increasing this counter tells select_check_waiters that the list has changed. */
  select_cb_ctr+= 1;

  LWIP_SOCKET_SELECT_UNPROTECT(lev);
}



/*
 * Go through the readset and writeset lists and see which socket of the sockets
 * set in the sets has events. On return, readset, writeset and exceptset have
 * the sockets enabled that had events.
 *
 * @param maxfdp1 the highest socket index in the sets
 * @param readset_in    set of sockets to check for read events
 * @param writeset_in   set of sockets to check for write events
 * @param exceptset_in  set of sockets to check for error events
 * @param readset_out   set of sockets that had read events
 * @param writeset_out  set of sockets that had write events
 * @param exceptset_out set os sockets that had error events
 * @return number of sockets that had events (read/write/exception) (>= 0)
 */
static int
lwip_selscan(maxfdp1: i32, fd_set *readset_in, fd_set *writeset_in, fd_set *exceptset_in,
             fd_set *readset_out, fd_set *writeset_out, fd_set *exceptset_out)
{
  i: i32, nready = 0;
  fd_set lreadset, lwriteset, lexceptset;
  sock: &mut lwip_sock;
  SYS_ARCH_DECL_PROTECT(lev);

  FD_ZERO(&lreadset);
  FD_ZERO(&lwriteset);
  FD_ZERO(&lexceptset);

  /* Go through each socket in each list to count number of sockets which
     currently match */
  for (i = LWIP_SOCKET_OFFSET; i < maxfdp1; i+= 1) {
    /* if this FD is not in the set, continue */
    if (!(readset_in && FD_ISSET(i, readset_in)) &&
        !(writeset_in && FD_ISSET(i, writeset_in)) &&
        !(exceptset_in && FD_ISSET(i, exceptset_in))) {
      continue;
    }
    /* First get the socket's status (protected)... */
    SYS_ARCH_PROTECT(lev);
    sock = tryget_socket_unconn_locked(i);
    if (sock != NULL) {
      lastdata: &mut () = sock.lastdata.pbuf;
      i16 rcvevent = sock.rcvevent;
      sendevent: u16 = sock.sendevent;
      errevent: u16 = sock.errevent;
      SYS_ARCH_UNPROTECT(lev);

      /* ... then examine it: */
      /* See if netconn of this socket is ready for read */
      if (readset_in && FD_ISSET(i, readset_in) && ((lastdata != NULL) || (rcvevent > 0))) {
        FD_SET(i, &lreadset);
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_selscan: fd=%d ready for reading\n", i));
        nready+= 1;
      }
      /* See if netconn of this socket is ready for write */
      if (writeset_in && FD_ISSET(i, writeset_in) && (sendevent != 0)) {
        FD_SET(i, &lwriteset);
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_selscan: fd=%d ready for writing\n", i));
        nready+= 1;
      }
      /* See if netconn of this socket had an error */
      if (exceptset_in && FD_ISSET(i, exceptset_in) && (errevent != 0)) {
        FD_SET(i, &lexceptset);
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_selscan: fd=%d ready for exception\n", i));
        nready+= 1;
      }
      done_socket(sock);
    } else {
      SYS_ARCH_UNPROTECT(lev);
      /* no a valid open socket */
      return -1;
    }
  }
  /* copy local sets to the ones provided as arguments */
  *readset_out = lreadset;
  *writeset_out = lwriteset;
  *exceptset_out = lexceptset;

  LWIP_ASSERT("nready >= 0", nready >= 0);
  return nready;
}


/* Mark all of the set sockets in one of the three passed: fdsets to select as used.
 * All sockets are marked (and later unmarked), whether they are open or not.
 * This is OK as lwip_selscan aborts select when non-open sockets are found.
 */
pub fn
lwip_select_inc_sockets_used_set(maxfdp: i32, fd_set *fdset, fd_set *used_sockets)
{
  SYS_ARCH_DECL_PROTECT(lev);
  if (fdset) {
    i: i32;
    for (i = LWIP_SOCKET_OFFSET; i < maxfdp; i+= 1) {
      /* if this FD is in the set, lock it (unless already done) */
      if (FD_ISSET(i, fdset) && !FD_ISSET(i, used_sockets)) {
        sock: &mut lwip_sock;
        SYS_ARCH_PROTECT(lev);
        sock = tryget_socket_unconn_locked(i);
        if (sock != NULL) {
          /* leave the socket used until released by lwip_select_dec_sockets_used */
          FD_SET(i, used_sockets);
        }
        SYS_ARCH_UNPROTECT(lev);
      }
    }
  }
}

/* Mark all sockets passed to select as used to prevent them from being freed
 * from other threads while select is running.
 * Marked sockets are added to 'used_sockets' to mark them only once an be able
 * to unmark them correctly.
 */
pub fn
lwip_select_inc_sockets_used(maxfdp: i32, fd_set *fdset1, fd_set *fdset2, fd_set *fdset3, fd_set *used_sockets)
{
  FD_ZERO(used_sockets);
  lwip_select_inc_sockets_used_set(maxfdp, fdset1, used_sockets);
  lwip_select_inc_sockets_used_set(maxfdp, fdset2, used_sockets);
  lwip_select_inc_sockets_used_set(maxfdp, fdset3, used_sockets);
}

/* Let go all sockets that were marked as used when starting select */
pub fn
lwip_select_dec_sockets_used(maxfdp: i32, fd_set *used_sockets)
{
  i: i32;
  for (i = LWIP_SOCKET_OFFSET; i < maxfdp; i+= 1) {
    /* if this FD is not in the set, continue */
    if (FD_ISSET(i, used_sockets)) {
      sock: &mut lwip_sock = tryget_socket_unconn_nouse(i);
      LWIP_ASSERT("socket gone at the end of select", sock != NULL);
      if (sock != NULL) {
        done_socket(sock);
      }
    }
  }
}
 /* LWIP_NETCONN_FULLDUPLEX */
#define lwip_select_inc_sockets_used(maxfdp1, readset, writeset, exceptset, used_sockets)
#define lwip_select_dec_sockets_used(maxfdp1, used_sockets)


pub fn lwip_select(maxfdp1: i32, fd_set *readset, fd_set *writeset, fd_set *exceptset,
            timeout: &mut timeval)
{
  waitres: u32 = 0;
  nready: i32;
  fd_set lreadset, lwriteset, lexceptset;
  msectimeout: u32;
  i: i32;
  maxfdp2: i32;

  waited: i32 = 0;


  used_sockets: fd_set;

  SYS_ARCH_DECL_PROTECT(lev);

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_select(%d, %p, %p, %p, tvsec=%"S32_F" tvusec=%"S32_F")\n",
                              maxfdp1, readset,  writeset,  exceptset,
                              timeout ? (i32)timeout.tv_sec : (i32) - 1,
                              timeout ? (i32)timeout.tv_usec : (i32) - 1));

  if ((maxfdp1 < 0) || (maxfdp1 > LWIP_SELECT_MAXNFDS)) {
    set_errno(EINVAL);
    return -1;
  }

  lwip_select_inc_sockets_used(maxfdp1, readset, writeset, exceptset, &used_sockets);

  /* Go through each socket in each list to count number of sockets which
     currently match */
  nready = lwip_selscan(maxfdp1, readset, writeset, exceptset, &lreadset, &lwriteset, &lexceptset);

  if (nready < 0) {
    /* one of the sockets in one of the fd_sets was invalid */
    set_errno(EBADF);
    lwip_select_dec_sockets_used(maxfdp1, &used_sockets);
    return -1;
  } else if (nready > 0) {
    /* one or more sockets are set, no need to wait */
    LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_select: nready=%d\n", nready));
  } else {
    /* If we don't have any current events, then suspend if we are supposed to */
    if (timeout && timeout.tv_sec == 0 && timeout.tv_usec == 0) {
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_select: no timeout, returning 0\n"));
      /* This is OK as the local are: fdsets empty and nready is zero,
         or we would have returned earlier. */
    } else {
      /* None ready: add our semaphore to list:
         We don't actually need any dynamic memory. Our entry on the
         list is only valid while we are in this function, so it's ok
         to use local variables (unless we're running in MPU compatible
         mode). */
      API_SELECT_CB_VAR_DECLARE(select_cb);
      API_SELECT_CB_VAR_ALLOC(select_cb, set_errno(ENOMEM); lwip_select_dec_sockets_used(maxfdp1, &used_sockets); return -1);
      memset(&API_SELECT_CB_VAR_REF(select_cb), 0, sizeof(LwipSelectCallback));

      API_SELECT_CB_VAR_REFselect_cb.readset = readset;
      API_SELECT_CB_VAR_REFselect_cb.writeset = writeset;
      API_SELECT_CB_VAR_REFselect_cb.exceptset = exceptset;

      API_SELECT_CB_VAR_REFselect_cb.sem = LWIP_NETCONN_THREAD_SEM_GET();
 /* LWIP_NETCONN_SEM_PER_THREAD */
      if (sys_sem_new(&API_SELECT_CB_VAR_REFselect_cb.sem, 0) != ERR_OK) {
        /* failed to create semaphore */
        set_errno(ENOMEM);
        lwip_select_dec_sockets_used(maxfdp1, &used_sockets);
        API_SELECT_CB_VAR_FREE(select_cb);
        return -1;
      }


      lwip_link_select_cb(&API_SELECT_CB_VAR_REF(select_cb));

      /* Increase select_waiting for each socket we are interested in */
      maxfdp2 = maxfdp1;
      for (i = LWIP_SOCKET_OFFSET; i < maxfdp1; i+= 1) {
        if ((readset && FD_ISSET(i, readset)) ||
            (writeset && FD_ISSET(i, writeset)) ||
            (exceptset && FD_ISSET(i, exceptset))) {
          sock: &mut lwip_sock;
          SYS_ARCH_PROTECT(lev);
          sock = tryget_socket_unconn_locked(i);
          if (sock != NULL) {
            sock.select_waiting+= 1;
            if (sock.select_waiting == 0) {
              /* overflow - too many threads waiting */
              sock.select_waiting -= 1;
              nready = -1;
              maxfdp2 = i;
              SYS_ARCH_UNPROTECT(lev);
              done_socket(sock);
              set_errno(EBUSY);
              break;
            }
            SYS_ARCH_UNPROTECT(lev);
            done_socket(sock);
          } else {
            /* Not a valid socket */
            nready = -1;
            maxfdp2 = i;
            SYS_ARCH_UNPROTECT(lev);
            set_errno(EBADF);
            break;
          }
        }
      }

      if (nready >= 0) {
        /* Call lwip_selscan again: there could have been events between
           the last scan (without us on the list) and putting us on the list! */
        nready = lwip_selscan(maxfdp1, readset, writeset, exceptset, &lreadset, &lwriteset, &lexceptset);
        if (!nready) {
          /* Still none ready, just wait to be woken */
          if (timeout == 0) {
            /* Wait forever */
            msectimeout = 0;
          } else {
            long msecs_long = ((timeout.tv_sec * 1000) + ((timeout.tv_usec + 500) / 1000));
            if (msecs_long <= 0) {
              /* Wait 1ms at least (0 means wait forever) */
              msectimeout = 1;
            } else {
              msectimeout = (u32)msecs_long;
            }
          }

          waitres = sys_arch_sem_wait(SELECT_SEM_PTR(API_SELECT_CB_VAR_REFselect_cb.sem), msectimeout);

          waited = 1;

        }
      }

      /* Decrease select_waiting for each socket we are interested in */
      for (i = LWIP_SOCKET_OFFSET; i < maxfdp2; i+= 1) {
        if ((readset && FD_ISSET(i, readset)) ||
            (writeset && FD_ISSET(i, writeset)) ||
            (exceptset && FD_ISSET(i, exceptset))) {
          sock: &mut lwip_sock;
          SYS_ARCH_PROTECT(lev);
          sock = tryget_socket_unconn_locked(i);
          if (sock != NULL) {
            /* for now, handle select_waiting==0... */
            LWIP_ASSERT("sock.select_waiting > 0", sock.select_waiting > 0);
            if (sock.select_waiting > 0) {
              sock.select_waiting -= 1;
            }
            SYS_ARCH_UNPROTECT(lev);
            done_socket(sock);
          } else {
            SYS_ARCH_UNPROTECT(lev);
            /* Not a valid socket */
            nready = -1;
            set_errno(EBADF);
          }
        }
      }

      lwip_unlink_select_cb(&API_SELECT_CB_VAR_REF(select_cb));


      if (API_SELECT_CB_VAR_REFselect_cb.sem_signalled && (!waited || (waitres == SYS_ARCH_TIMEOUT))) {
        /* don't leave the thread-local semaphore signalled */
        sys_arch_sem_wait(API_SELECT_CB_VAR_REFselect_cb.sem, 1);
      }
 /* LWIP_NETCONN_SEM_PER_THREAD */
      sys_sem_free(&API_SELECT_CB_VAR_REFselect_cb.sem);

      API_SELECT_CB_VAR_FREE(select_cb);

      if (nready < 0) {
        /* This happens when a socket got closed while waiting */
        lwip_select_dec_sockets_used(maxfdp1, &used_sockets);
        return -1;
      }

      if (waitres == SYS_ARCH_TIMEOUT) {
        /* Timeout */
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_select: timeout expired\n"));
        /* This is OK as the local are: fdsets empty and nready is zero,
           or we would have returned earlier. */
      } else {
        /* See what's set now after waiting */
        nready = lwip_selscan(maxfdp1, readset, writeset, exceptset, &lreadset, &lwriteset, &lexceptset);
        LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_select: nready=%d\n", nready));
      }
    }
  }

  lwip_select_dec_sockets_used(maxfdp1, &used_sockets);
  set_errno(0);
  if (readset) {
    *readset = lreadset;
  }
  if (writeset) {
    *writeset = lwriteset;
  }
  if (exceptset) {
    *exceptset = lexceptset;
  }
  return nready;
}



/* Options for the lwip_pollscan function. */
enum lwip_pollscan_opts
{
  /* Clear revents in each struct pollfd. */
  LWIP_POLLSCAN_CLEAR = 1,

  /* Increment select_waiting in each struct lwip_sock. */
  LWIP_POLLSCAN_INC_WAIT = 2,

  /* Decrement select_waiting in each struct lwip_sock. */
  LWIP_POLLSCAN_DEC_WAIT = 4
};

/*
 * Update revents in each struct pollfd.
 * Optionally update select_waiting in struct lwip_sock.
 *
 * @param fds          array of structures to update
 * @param nfds         number of structures in fds
 * @param opts         what to update and how
 * @return number of structures that have revents != 0
 */
static int
lwip_pollscan(fds: &mut pollfd, nfds_t nfds, enum lwip_pollscan_opts opts)
{
  nready: i32 = 0;
  nfds_t fdi;
  sock: &mut lwip_sock;
  SYS_ARCH_DECL_PROTECT(lev);

  /* Go through each in: pollfd the array. */
  for (fdi = 0; fdi < nfds; fdi+= 1) {
    if ((opts & LWIP_POLLSCAN_CLEAR) != 0) {
      fds[fdi].revents = 0;
    }

    /* Negative fd means the caller wants us to ignore this struct.
       POLLNVAL means we already detected that the fd is invalid;
       if another thread has since opened a new socket with that fd,
       we must not use that socket. */
    if (fds[fdi].fd >= 0 && (fds[fdi].revents & POLLNVAL) == 0) {
      /* First get the socket's status (protected)... */
      SYS_ARCH_PROTECT(lev);
      sock = tryget_socket_unconn_locked(fds[fdi].fd);
      if (sock != NULL) {
        void* lastdata = sock.lastdata.pbuf;
        i16 rcvevent = sock.rcvevent;
        sendevent: u16 = sock.sendevent;
        errevent: u16 = sock.errevent;

        if ((opts & LWIP_POLLSCAN_INC_WAIT) != 0) {
          sock.select_waiting+= 1;
          if (sock.select_waiting == 0) {
            /* overflow - too many threads waiting */
            sock.select_waiting -= 1;
            nready = -1;
            SYS_ARCH_UNPROTECT(lev);
            done_socket(sock);
            break;
          }
        } else if ((opts & LWIP_POLLSCAN_DEC_WAIT) != 0) {
          /* for now, handle select_waiting==0... */
          LWIP_ASSERT("sock.select_waiting > 0", sock.select_waiting > 0);
          if (sock.select_waiting > 0) {
            sock.select_waiting -= 1;
          }
        }
        SYS_ARCH_UNPROTECT(lev);
        done_socket(sock);

        /* ... then examine it: */
        /* See if netconn of this socket is ready for read */
        if ((fds[fdi].events & POLLIN) != 0 && ((lastdata != NULL) || (rcvevent > 0))) {
          fds[fdi].revents |= POLLIN;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_pollscan: fd=%d ready for reading\n", fds[fdi].fd));
        }
        /* See if netconn of this socket is ready for write */
        if ((fds[fdi].events & POLLOUT) != 0 && (sendevent != 0)) {
          fds[fdi].revents |= POLLOUT;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_pollscan: fd=%d ready for writing\n", fds[fdi].fd));
        }
        /* See if netconn of this socket had an error */
        if (errevent != 0) {
          /* POLLERR is output only. */
          fds[fdi].revents |= POLLERR;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_pollscan: fd=%d ready for exception\n", fds[fdi].fd));
        }
      } else {
        /* Not a valid socket */
        SYS_ARCH_UNPROTECT(lev);
        /* POLLNVAL is output only. */
        fds[fdi].revents |= POLLNVAL;
        return -1;
      }
    }

    /* Will return the number of structures that have events,
       not the number of events. */
    if (fds[fdi].revents != 0) {
      nready+= 1;
    }
  }

  LWIP_ASSERT("nready >= 0", nready >= 0);
  return nready;
}


/* Mark all sockets as used.
 *
 * All sockets are marked (and later unmarked), whether they are open or not.
 * This is OK as lwip_pollscan aborts select when non-open sockets are found.
 */
pub fn
lwip_poll_inc_sockets_used(fds: &mut pollfd, nfds_t nfds)
{
  nfds_t fdi;

  if(fds) {
    /* Go through each in: pollfd the array. */
    for (fdi = 0; fdi < nfds; fdi+= 1) {
      /* Increase the reference counter */
      tryget_socket_unconn(fds[fdi].fd);
    }
  }
}

/* Let go all sockets that were marked as used when starting poll */
pub fn
lwip_poll_dec_sockets_used(fds: &mut pollfd, nfds_t nfds)
{
  nfds_t fdi;

  if(fds) {
    /* Go through each in: pollfd the array. */
    for (fdi = 0; fdi < nfds; fdi+= 1) {
      sock: &mut lwip_sock = tryget_socket_unconn_nouse(fds[fdi].fd);
      if (sock != NULL) {
        done_socket(sock);
      }
    }
  }
}
 /* LWIP_NETCONN_FULLDUPLEX */
#define lwip_poll_inc_sockets_used(fds, nfds)
#define lwip_poll_dec_sockets_used(fds, nfds)


pub fn lwip_poll(fds: &mut pollfd, nfds_t nfds, timeout: i32)
{
  waitres: u32 = 0;
  nready: i32;
  msectimeout: u32;

  waited: i32 = 0;


  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_poll(%p, %d, %d)\n",
                  fds, nfds, timeout));
  LWIP_ERROR("lwip_poll: invalid fds", ((fds != NULL && nfds > 0) || (fds == NULL && nfds == 0)),
             set_errno(EINVAL); return -1;);

  lwip_poll_inc_sockets_used(fds, nfds);

  /* Go through each to: pollfd count number of structures
     which currently match */
  nready = lwip_pollscan(fds, nfds, LWIP_POLLSCAN_CLEAR);

  if (nready < 0) {
    lwip_poll_dec_sockets_used(fds, nfds);
    return -1;
  }

  /* If we don't have any current events, then suspend if we are supposed to */
  if (!nready) {
    API_SELECT_CB_VAR_DECLARE(select_cb);

    if (timeout == 0) {
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_poll: no timeout, returning 0\n"));
      // goto return_success;
    }
    API_SELECT_CB_VAR_ALLOC(select_cb, set_errno(EAGAIN); lwip_poll_dec_sockets_used(fds, nfds); return -1);
    memset(&API_SELECT_CB_VAR_REF(select_cb), 0, sizeof(LwipSelectCallback));

    /* None ready: add our semaphore to list:
       We don't actually need any dynamic memory. Our entry on the
       list is only valid while we are in this function, so it's ok
       to use local variables. */

    API_SELECT_CB_VAR_REFselect_cb.poll_fds = fds;
    API_SELECT_CB_VAR_REFselect_cb.poll_nfds = nfds;

    API_SELECT_CB_VAR_REFselect_cb.sem = LWIP_NETCONN_THREAD_SEM_GET();
 /* LWIP_NETCONN_SEM_PER_THREAD */
    if (sys_sem_new(&API_SELECT_CB_VAR_REFselect_cb.sem, 0) != ERR_OK) {
      /* failed to create semaphore */
      set_errno(EAGAIN);
      lwip_poll_dec_sockets_used(fds, nfds);
      API_SELECT_CB_VAR_FREE(select_cb);
      return -1;
    }


    lwip_link_select_cb(&API_SELECT_CB_VAR_REF(select_cb));

    /* Increase select_waiting for each socket we are interested in.
       Also, check for events again: there could have been events between
       the last scan (without us on the list) and putting us on the list! */
    nready = lwip_pollscan(fds, nfds, LWIP_POLLSCAN_INC_WAIT);

    if (!nready) {
      /* Still none ready, just wait to be woken */
      if (timeout < 0) {
        /* Wait forever */
        msectimeout = 0;
      } else {
        /* timeout == 0 would have been handled earlier. */
        LWIP_ASSERT("timeout > 0", timeout > 0);
        msectimeout = timeout;
      }
      waitres = sys_arch_sem_wait(SELECT_SEM_PTR(API_SELECT_CB_VAR_REFselect_cb.sem), msectimeout);

      waited = 1;

    }

    /* Decrease select_waiting for each socket we are interested in,
       and check which events occurred while we waited. */
    nready = lwip_pollscan(fds, nfds, LWIP_POLLSCAN_DEC_WAIT);

    lwip_unlink_select_cb(&API_SELECT_CB_VAR_REF(select_cb));


    if (select_cb.sem_signalled && (!waited || (waitres == SYS_ARCH_TIMEOUT))) {
      /* don't leave the thread-local semaphore signalled */
      sys_arch_sem_wait(API_SELECT_CB_VAR_REFselect_cb.sem, 1);
    }
 /* LWIP_NETCONN_SEM_PER_THREAD */
    sys_sem_free(&API_SELECT_CB_VAR_REFselect_cb.sem);

    API_SELECT_CB_VAR_FREE(select_cb);

    if (nready < 0) {
      /* This happens when a socket got closed while waiting */
      lwip_poll_dec_sockets_used(fds, nfds);
      return -1;
    }

    if (waitres == SYS_ARCH_TIMEOUT) {
      /* Timeout */
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_poll: timeout expired\n"));
      // goto return_success;
    }
  }

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_poll: nready=%d\n", nready));
return_success:
  lwip_poll_dec_sockets_used(fds, nfds);
  set_errno(0);
  return nready;
}

/*
 * Check whether event_callback should wake up a thread waiting in
 * lwip_poll.
 */
static int
lwip_poll_should_wake(const scb: &mut lwip_select_cb, fd: i32, has_recvevent: i32, has_sendevent: i32, has_errevent: i32)
{
  nfds_t fdi;
  for (fdi = 0; fdi < scb.poll_nfds; fdi+= 1) {
    const pollfd: &mut pollfd = &scb.poll_fds[fdi];
    if (pollfd.fd == fd) {
      /* Do not update pollfd.revents right here;
         that would be a data race because lwip_pollscan
         accesses revents without protecting. */
      if (has_recvevent && (pollfd.events & POLLIN) != 0) {
        return 1;
      }
      if (has_sendevent && (pollfd.events & POLLOUT) != 0) {
        return 1;
      }
      if (has_errevent) {
        /* POLLERR is output only. */
        return 1;
      }
    }
  }
  return 0;
}



/*
 * Callback registered in the netconn layer for each socket-netconn.
 * Processes recvevent (data available) and wakes up tasks waiting for select.
 *
 * @note for LWIP_TCPIP_CORE_LOCKING any caller of this function
 * must have the core lock held when signaling the following events
 * as they might cause select_list_cb to be checked:
 *   NETCONN_EVT_RCVPLUS
 *   NETCONN_EVT_SENDPLUS
 *   NETCONN_EVT_ERROR
 * This requirement will be asserted in select_check_waiters()
 */
pub fn
event_callback(conn: &mut netconn, enum netconn_evt evt, len: u16)
{
  s: i32, check_waiters;
  sock: &mut lwip_sock;
  SYS_ARCH_DECL_PROTECT(lev);

  

  /* Get socket */
  if (conn) {
    s = conn.socket;
    if (s < 0) {
      /* Data comes in right away after an accept, even though
       * the server task might not have created a new socket yet.
       * Just count down (or up) if that's the case and we
       * will use the data later. Note that only receive events
       * can happen before the new socket is set up. */
      SYS_ARCH_PROTECT(lev);
      if (conn.socket < 0) {
        if (evt == NETCONN_EVT_RCVPLUS) {
          /* conn.socket is -1 on initialization
             lwip_accept adjusts sock.recvevent if conn.socket < -1 */
          conn.socket -= 1;
        }
        SYS_ARCH_UNPROTECT(lev);
        return;
      }
      s = conn.socket;
      SYS_ARCH_UNPROTECT(lev);
    }

    sock = get_socket(s);
    if (!sock) {
      return;
    }
  } else {
    return;
  }

  check_waiters = 1;
  SYS_ARCH_PROTECT(lev);
  /* Set event as required */
  match (evt) {
    NETCONN_EVT_RCVPLUS =>
      sock.rcvevent+= 1;
      if (sock.rcvevent > 1) {
        check_waiters = 0;
      }
      break;
    NETCONN_EVT_RCVMINUS =>
      sock.rcvevent -= 1;
      check_waiters = 0;
      break;
    NETCONN_EVT_SENDPLUS =>
      if (sock.sendevent) {
        check_waiters = 0;
      }
      sock.sendevent = 1;
      break;
    NETCONN_EVT_SENDMINUS =>
      sock.sendevent = 0;
      check_waiters = 0;
      break;
    NETCONN_EVT_ERROR =>
      sock.errevent = 1;
      break;
    _ =>
      LWIP_ASSERT("unknown event", 0);
      break;
  }

  if (sock.select_waiting && check_waiters) {
    /* Save which events are active */
    has_recvevent: i32, has_sendevent, has_errevent;
    has_recvevent = sock.rcvevent > 0;
    has_sendevent = sock.sendevent != 0;
    has_errevent = sock.errevent != 0;
    SYS_ARCH_UNPROTECT(lev);
    /* Check any select calls waiting on this socket */
    select_check_waiters(s, has_recvevent, has_sendevent, has_errevent);
  } else {
    SYS_ARCH_UNPROTECT(lev);
  }
  done_socket(sock);
}

/*
 * Check if any select waiters are waiting on this socket and its events
 *
 * @note on synchronization of select_cb_list:
 * LWIP_TCPIP_CORE_LOCKING: the select_cb_list must only be accessed while holding
 * the core lock. We do a single pass through the list and signal any waiters.
 * Core lock should already be held when calling here!!!!

 * !LWIP_TCPIP_CORE_LOCKING: we use SYS_ARCH_PROTECT but unlock on each iteration
 * of the loop, thus creating a possibility where a thread could modify the
 * select_cb_list during our UNPROTECT/PROTECT. We use a generational counter to
 * detect this change and restart the list walk. The list is expected to be small
 */
pub fn select_check_waiters(s: i32, has_recvevent: i32, has_sendevent: i32, has_errevent: i32)
{
  scb: &mut lwip_select_cb;

  last_select_cb_ctr: i32;
  SYS_ARCH_DECL_PROTECT(lev);


  LWIP_ASSERT_CORE_LOCKED();


  SYS_ARCH_PROTECT(lev);
again:
  /* remember the state of select_cb_list to detect changes */
  last_select_cb_ctr = select_cb_ctr;

  for (scb = select_cb_list; scb != NULL; scb = scb.next) {
    if (scb.sem_signalled == 0) {
      /* semaphore not signalled yet */
      do_signal: i32 = 0;

      if (scb.poll_fds != NULL) {
        do_signal = lwip_poll_should_wake(scb, s, has_recvevent, has_sendevent, has_errevent);
      }


      else


      {
        /* Test this select call for our socket */
        if (has_recvevent) {
          if (scb.readset && FD_ISSET(s, scb.readset)) {
            do_signal = 1;
          }
        }
        if (has_sendevent) {
          if (!do_signal && scb.writeset && FD_ISSET(s, scb.writeset)) {
            do_signal = 1;
          }
        }
        if (has_errevent) {
          if (!do_signal && scb.exceptset && FD_ISSET(s, scb.exceptset)) {
            do_signal = 1;
          }
        }
      }

      if (do_signal) {
        scb.sem_signalled = 1;
        /* For !LWIP_TCPIP_CORE_LOCKING, we don't call SYS_ARCH_UNPROTECT() before signaling
           the semaphore, as this might lead to the select thread taking itself off the list,
           invalidating the semaphore. */
        sys_sem_signal(SELECT_SEM_PTR(scb.sem));
      }
    }

  }

    /* unlock interrupts with each step */
    SYS_ARCH_UNPROTECT(lev);
    /* this makes sure interrupt protection time is short */
    SYS_ARCH_PROTECT(lev);
    if (last_select_cb_ctr != select_cb_ctr) {
      /* someone has changed select_cb_list, restart at the beginning */
      // goto again;
    }
    /* remember the state of select_cb_list to detect changes */
    last_select_cb_ctr = select_cb_ctr;
  }
  SYS_ARCH_UNPROTECT(lev);

}


/*
 * Close one end of a full-duplex connection.
 */
pub fn lwip_shutdown(s: i32, how: i32)
{
  sock: &mut lwip_sock;
  let err: err_t;
  shut_rx: u8 = 0, shut_tx = 0;

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_shutdown(%d, how=%d)\n", s, how));

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  if (sock.conn != NULL) {
    if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_TCP) {
      sock_set_errno(sock, EOPNOTSUPP);
      done_socket(sock);
      return -1;
    }
  } else {
    sock_set_errno(sock, ENOTCONN);
    done_socket(sock);
    return -1;
  }

  if (how == SHUT_RD) {
    shut_rx = 1;
  } else if (how == SHUT_WR) {
    shut_tx = 1;
  } else if (how == SHUT_RDWR) {
    shut_rx = 1;
    shut_tx = 1;
  } else {
    sock_set_errno(sock, EINVAL);
    done_socket(sock);
    return -1;
  }
  err = netconn_shutdown(sock.conn, shut_rx, shut_tx);

  sock_set_errno(sock, err_to_errno(err));
  done_socket(sock);
  return (err == ERR_OK ? 0 : -1);
}

static int
lwip_getaddrname(s: i32, name: &mut sockaddr, socklen_t *namelen, local: u8)
{
  sock: &mut lwip_sock;
  union sockaddr_aligned saddr;
  ip_addr_t naddr;
  port: u16;
  let err: err_t;

  sock = get_socket(s);
  if (!sock) {
    return -1;
  }

  /* get the IP address and port */
  err = netconn_getaddr(sock.conn, &naddr, &port, local);
  if (err != ERR_OK) {
    sock_set_errno(sock, err_to_errno(err));
    done_socket(sock);
    return -1;
  }


  /* Dual-stack: Map IPv4 addresses to IPv4 mapped IPv6 */
  if (NETCONNTYPE_ISIPV6(netconn_type(sock.conn)) &&
      IP_IS_V4_VAL(naddr)) {
    ip4_2_ipv4_mapped_ipv6(ip_2_ip6(&naddr), ip_2_ip4(&naddr));
    IP_SET_TYPE_VAL(naddr, IPADDR_TYPE_V6);
  }


  IPADDR_PORT_TO_SOCKADDR(&saddr, &naddr, port);

  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getaddrname(%d, addr=", s));
  ip_addr_debug_print_val(SOCKETS_DEBUG, naddr);
  LWIP_DEBUGF(SOCKETS_DEBUG, (" port=%"U16_F")\n", port));

  if (*namelen > saddr.sa.sa_len) {
    *namelen = saddr.sa.sa_len;
  }
  MEMCPY(name, &saddr, *namelen);

  sock_set_errno(sock, 0);
  done_socket(sock);
  return 0;
}

pub fn lwip_getpeername(s: i32, name: &mut sockaddr, socklen_t *namelen)
{
  return lwip_getaddrname(s, name, namelen, 0);
}

pub fn lwip_getsockname(s: i32, name: &mut sockaddr, socklen_t *namelen)
{
  return lwip_getaddrname(s, name, namelen, 1);
}

pub fn lwip_getsockopt(s: i32, level: i32, optname: i32, optval: &mut (), socklen_t *optlen)
{
  err: i32;
  sock: &mut lwip_sock = get_socket(s);

  cberr: err_t;
  LWIP_SETGETSOCKOPT_DATA_VAR_DECLARE(data);


  if (!sock) {
    return -1;
  }

  if ((NULL == optval) || (NULL == optlen)) {
    sock_set_errno(sock, EFAULT);
    done_socket(sock);
    return -1;
  }


  /* core-locking can just call the -impl function */
  LOCK_TCPIP_CORE();
  err = lwip_getsockopt_impl(s, level, optname, optval, optlen);
  UNLOCK_TCPIP_CORE();

 /* LWIP_TCPIP_CORE_LOCKING */


  /* MPU_COMPATIBLE copies the optval data, so check for max size here */
  if (*optlen > LWIP_SETGETSOCKOPT_MAXOPTLEN) {
    sock_set_errno(sock, ENOBUFS);
    done_socket(sock);
    return -1;
  }


  LWIP_SETGETSOCKOPT_DATA_VAR_ALLOC(data, sock);
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.s = s;
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.level = level;
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optname = optname;
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optlen = *optlen;

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optval.p = optval;

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.err = 0;

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.completed_sem = LWIP_NETCONN_THREAD_SEM_GET();

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.completed_sem = &sock.conn.op_completed;

  cberr = tcpip_callback(lwip_getsockopt_callback, &LWIP_SETGETSOCKOPT_DATA_VAR_REF(data));
  if (cberr != ERR_OK) {
    LWIP_SETGETSOCKOPT_DATA_VAR_FREE(data);
    sock_set_errno(sock, err_to_errno(cberr));
    done_socket(sock);
    return -1;
  }
  sys_arch_sem_wait((sys_sem_t *)(LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.completed_sem), 0);

  /* write back optlen and optval */
  *optlen = LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optlen;

  MEMCPY(optval, LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optval,
         LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optlen);


  /* maybe lwip_getsockopt_internal has changed err */
  err = LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.err;
  LWIP_SETGETSOCKOPT_DATA_VAR_FREE(data);


  sock_set_errno(sock, err);
  done_socket(sock);
  return err ? -1 : 0;
}


/* lwip_getsockopt_callback: only used without CORE_LOCKING
 * to get into the tcpip_thread
 */
pub fn
lwip_getsockopt_callback(arg: &mut Vec<u8>)
{
  data: &mut lwip_setgetsockopt_data;
  LWIP_ASSERT("arg != NULL", arg != NULL);
  data = (struct lwip_setgetsockopt_data *)arg;

  data.err = lwip_getsockopt_impl(data.s, data.level, data.optname,

                                   data.optval,
 /* LWIP_MPU_COMPATIBLE */
                                   data.optval.p,

                                   &data.optlen);

  sys_sem_signal((sys_sem_t *)(data.completed_sem));
}


static int
lwip_sockopt_to_ipopt(optname: i32)
{
  /* Map SO_* values to our internal SOF_* values
   * We should not rely on #defines in socket.h
   * being in sync with ip.h.
   */
  match (optname) {
  SO_BROADCAST =>
    return SOF_BROADCAST;
  SO_KEEPALIVE =>
    return SOF_KEEPALIVE;
  SO_REUSEADDR =>
    return SOF_REUSEADDR;
  _ =>
    LWIP_ASSERT("Unknown socket option", 0);
    return 0;
  }
}

/* lwip_getsockopt_impl: the actual implementation of getsockopt:
 * same argument as lwip_getsockopt, either called directly or through callback
 */
static int
lwip_getsockopt_impl(s: i32, level: i32, optname: i32, optval: &mut (), socklen_t *optlen)
{
  err: i32 = 0;
  sock: &mut lwip_sock = tryget_socket(s);
  if (!sock) {
    return EBADF;
  }


  if (LWIP_HOOK_SOCKETS_GETSOCKOPT(s, sock, level, optname, optval, optlen, &err)) {
    return err;
  }


  match (level) {

    /* Level: SOL_SOCKET */
    SOL_SOCKET =>
      match (optname) {


        SO_ACCEPTCONN =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, int);
          if (NETCONNTYPE_GROUP(sock.conn.netconntype) != NETCONN_TCP) {
            done_socket(sock);
            return ENOPROTOOPT;
          }
          if ((sock.conn.pcb.tcp != NULL) && (sock.conn.pcb.tcp.state == LISTEN)) {
            *(int *)optval = 1;
          } else {
            *(int *)optval = 0;
          }
          break;


        /* The option flags */
        SO_BROADCAST =>
        SO_KEEPALIVE =>

        SO_REUSEADDR =>

          if ((optname == SO_BROADCAST) &&
              (NETCONNTYPE_GROUP(sock.conn.netconntype) != NETCONN_UDP)) {
            done_socket(sock);
            return ENOPROTOOPT;
          }

          optname = lwip_sockopt_to_ipopt(optname);

          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, int);
          *(int *)optval = ip_get_option(sock.conn.pcb.ip, optname);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, SOL_SOCKET, optname=0x%x, ..) = %s\n",
                                      s, optname, (*(int *)optval ? "on" : "off")));
          break;

        SO_TYPE =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, *optlen, int);
          match (NETCONNTYPE_GROUP(netconn_type(sock.conn))) {
            NETCONN_RAW =>
              *(int *)optval = SOCK_RAW;
              break;
            NETCONN_TCP =>
              *(int *)optval = SOCK_STREAM;
              break;
            NETCONN_UDP =>
              *(int *)optval = SOCK_DGRAM;
              break;
            _ => /* unrecognized socket type */
              *(int *)optval = netconn_type(sock.conn);
              LWIP_DEBUGF(SOCKETS_DEBUG,
                          ("lwip_getsockopt(%d, SOL_SOCKET, SO_TYPE): unrecognized socket type %d\n",
                           s, *(int *)optval));
          }  /* match (netconn_type(sock.conn)) */
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, SOL_SOCKET, SO_TYPE) = %d\n",
                                      s, *(int *)optval));
          break;

        SO_ERROR =>
          LWIP_SOCKOPT_CHECK_OPTLEN(sock, *optlen, int);
          *(int *)optval = err_to_errno(netconn_err(sock.conn));
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, SOL_SOCKET, SO_ERROR) = %d\n",
                                      s, *(int *)optval));
          break;


        SO_SNDTIMEO =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, *optlen, LWIP_SO_SNDRCVTIMEO_OPTTYPE);
          LWIP_SO_SNDRCVTIMEO_SET(optval, netconn_get_sendtimeout(sock.conn));
          break;


        SO_RCVTIMEO =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, *optlen, LWIP_SO_SNDRCVTIMEO_OPTTYPE);
          LWIP_SO_SNDRCVTIMEO_SET(optval, netconn_get_recvtimeout(sock.conn));
          break;


        SO_RCVBUF =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, *optlen, int);
          *(int *)optval = netconn_get_recvbufsize(sock.conn);
          break;


        SO_LINGER => {
          conn_linger: i16;
          linger: &mut linger = (struct linger *)optval;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, *optlen, struct linger);
          conn_linger = sock.conn.linger;
          if (conn_linger >= 0) {
            linger.l_onoff = 1;
            linger.l_linger = conn_linger;
          } else {
            linger.l_onoff = 0;
            linger.l_linger = 0;
          }
        }
        break;


        SO_NO_CHECK =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, *optlen, int, NETCONN_UDP);

          if (udp_is_flag_set(sock.conn.pcb.udp, UDP_FLAGS_UDPLITE)) {
            /* this flag is only available for UDP, not for UDP lite */
            done_socket(sock);
            return EAFNOSUPPORT;
          }

          *(int *)optval = udp_is_flag_set(sock.conn.pcb.udp, UDP_FLAGS_NOCHKSUM) ? 1 : 0;
          break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, SOL_SOCKET, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;

    /* Level: IPPROTO_IP */
    IPPROTO_IP =>
      match (optname) {
        IP_TTL =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, int);
          *(int *)optval = sock.conn.pcb.ip.ttl;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IP, IP_TTL) = %d\n",
                                      s, *(int *)optval));
          break;
        IP_TOS =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, int);
          *(int *)optval = sock.conn.pcb.ip.tos;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IP, IP_TOS) = %d\n",
                                      s, *(int *)optval));
          break;

        IP_MULTICAST_TTL =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, u8);
          if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_UDP) {
            done_socket(sock);
            return ENOPROTOOPT;
          }
          *optval = udp_get_multicast_ttl(sock.conn.pcb.udp);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IP, IP_MULTICAST_TTL) = %d\n",
                                      s, *(int *)optval));
          break;
        IP_MULTICAST_IF =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, struct in_addr);
          if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_UDP) {
            done_socket(sock);
            return ENOPROTOOPT;
          }
          inet_addr_from_ip4addr((struct in_addr *)optval, udp_get_multicast_netif_addr(sock.conn.pcb.udp));
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IP, IP_MULTICAST_IF) = 0x%"X32_F"\n",
                                      s, *(u32 *)optval));
          break;
        IP_MULTICAST_LOOP =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, u8);
          if ((sock.conn.pcb.udp.flags & UDP_FLAGS_MULTICAST_LOOP) != 0) {
            *optval = 1;
          } else {
            *optval = 0;
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IP, IP_MULTICAST_LOOP) = %d\n",
                                      s, *(int *)optval));
          break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IP, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;


    /* Level: IPPROTO_TCP */
    IPPROTO_TCP =>
      /* Special case: all IPPROTO_TCP option take an int */
      LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, *optlen, int, NETCONN_TCP);
      if (sock.conn.pcb.tcp.state == LISTEN) {
        done_socket(sock);
        return EINVAL;
      }
      match (optname) {
        TCP_NODELAY =>
          *(int *)optval = tcp_nagle_disabled(sock.conn.pcb.tcp);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_TCP, TCP_NODELAY) = %s\n",
                                      s, (*(int *)optval) ? "on" : "off") );
          break;
        TCP_KEEPALIVE =>
          *(int *)optval = sock.conn.pcb.tcp.keep_idle;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_TCP, TCP_KEEPALIVE) = %d\n",
                                      s, *(int *)optval));
          break;


        TCP_KEEPIDLE =>
          *(int *)optval = (sock.conn.pcb.tcp.keep_idle / 1000);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_TCP, TCP_KEEPIDLE) = %d\n",
                                      s, *(int *)optval));
          break;
        TCP_KEEPINTVL =>
          *(int *)optval = (sock.conn.pcb.tcp.keep_intvl / 1000);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_TCP, TCP_KEEPINTVL) = %d\n",
                                      s, *(int *)optval));
          break;
        TCP_KEEPCNT =>
          *(int *)optval = sock.conn.pcb.tcp.keep_cnt;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_TCP, TCP_KEEPCNT) = %d\n",
                                      s, *(int *)optval));
          break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_TCP, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;



    /* Level: IPPROTO_IPV6 */
    IPPROTO_IPV6 =>
      match (optname) {
        IPV6_V6ONLY =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, *optlen, int);
          *(int *)optval = (netconn_get_ipv6only(sock.conn) ? 1 : 0);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IPV6, IPV6_V6ONLY) = %d\n",
                                      s, *(int *)optval));
          break;
        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_IPV6, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;



    /* Level: IPPROTO_UDPLITE */
    IPPROTO_UDPLITE =>
      /* Special case: all IPPROTO_UDPLITE option take an int */
      LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, *optlen, int);
      /* If this is no UDP lite socket, ignore any options. */
      if (!NETCONNTYPE_ISUDPLITE(netconn_type(sock.conn))) {
        done_socket(sock);
        return ENOPROTOOPT;
      }
      match (optname) {
        UDPLITE_SEND_CSCOV =>
          *(int *)optval = sock.conn.pcb.udp.chksum_len_tx;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_UDPLITE, UDPLITE_SEND_CSCOV) = %d\n",
                                      s, (*(int *)optval)) );
          break;
        UDPLITE_RECV_CSCOV =>
          *(int *)optval = sock.conn.pcb.udp.chksum_len_rx;
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_UDPLITE, UDPLITE_RECV_CSCOV) = %d\n",
                                      s, (*(int *)optval)) );
          break;
        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_UDPLITE, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;

    /* Level: IPPROTO_RAW */
    IPPROTO_RAW =>
      match (optname) {

        IPV6_CHECKSUM =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, *optlen, int, NETCONN_RAW);
          if (sock.conn.pcb.raw.chksum_reqd == 0) {
            *(int *)optval = -1;
          } else {
            *(int *)optval = sock.conn.pcb.raw.chksum_offset;
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_RAW, IPV6_CHECKSUM) = %d\n",
                                      s, (*(int *)optval)) );
          break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, IPPROTO_RAW, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;
    _ =>
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_getsockopt(%d, level=0x%x, UNIMPL: optname=0x%x, ..)\n",
                                  s, level, optname));
      err = ENOPROTOOPT;
      break;
  } /* match (level) */

  done_socket(sock);
  return err;
}

pub fn lwip_setsockopt(s: i32, level: i32, optname: i32, optval: &Vec<u8>, optlen: socklen_t)
{
  err: i32 = 0;
  sock: &mut lwip_sock = get_socket(s);

  cberr: err_t;
  LWIP_SETGETSOCKOPT_DATA_VAR_DECLARE(data);


  if (!sock) {
    return -1;
  }

  if (NULL == optval) {
    sock_set_errno(sock, EFAULT);
    done_socket(sock);
    return -1;
  }


  /* core-locking can just call the -impl function */
  LOCK_TCPIP_CORE();
  err = lwip_setsockopt_impl(s, level, optname, optval, optlen);
  UNLOCK_TCPIP_CORE();

 /* LWIP_TCPIP_CORE_LOCKING */


  /* MPU_COMPATIBLE copies the optval data, so check for max size here */
  if (optlen > LWIP_SETGETSOCKOPT_MAXOPTLEN) {
    sock_set_errno(sock, ENOBUFS);
    done_socket(sock);
    return -1;
  }


  LWIP_SETGETSOCKOPT_DATA_VAR_ALLOC(data, sock);
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.s = s;
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.level = level;
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optname = optname;
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optlen = optlen;

  MEMCPY(LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optval, optval, optlen);
 /* LWIP_MPU_COMPATIBLE */
  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.optval.pc = (const void *)optval;

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.err = 0;

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.completed_sem = LWIP_NETCONN_THREAD_SEM_GET();

  LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.completed_sem = &sock.conn.op_completed;

  cberr = tcpip_callback(lwip_setsockopt_callback, &LWIP_SETGETSOCKOPT_DATA_VAR_REF(data));
  if (cberr != ERR_OK) {
    LWIP_SETGETSOCKOPT_DATA_VAR_FREE(data);
    sock_set_errno(sock, err_to_errno(cberr));
    done_socket(sock);
    return -1;
  }
  sys_arch_sem_wait((sys_sem_t *)(LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.completed_sem), 0);

  /* maybe lwip_getsockopt_internal has changed err */
  err = LWIP_SETGETSOCKOPT_DATA_VAR_REFdata.err;
  LWIP_SETGETSOCKOPT_DATA_VAR_FREE(data);


  sock_set_errno(sock, err);
  done_socket(sock);
  return err ? -1 : 0;
}


/* lwip_setsockopt_callback: only used without CORE_LOCKING
 * to get into the tcpip_thread
 */
pub fn
lwip_setsockopt_callback(arg: &mut Vec<u8>)
{
  data: &mut lwip_setgetsockopt_data;
  LWIP_ASSERT("arg != NULL", arg != NULL);
  data = (struct lwip_setgetsockopt_data *)arg;

  data.err = lwip_setsockopt_impl(data.s, data.level, data.optname,

                                   data.optval,
 /* LWIP_MPU_COMPATIBLE */
                                   data.optval.pc,

                                   data.optlen);

  sys_sem_signal((sys_sem_t *)(data.completed_sem));
}


/* lwip_setsockopt_impl: the actual implementation of setsockopt:
 * same argument as lwip_setsockopt, either called directly or through callback
 */
static int
lwip_setsockopt_impl(s: i32, level: i32, optname: i32, optval: &Vec<u8>, optlen: socklen_t)
{
  err: i32 = 0;
  sock: &mut lwip_sock = tryget_socket(s);
  if (!sock) {
    return EBADF;
  }


  if (LWIP_HOOK_SOCKETS_SETSOCKOPT(s, sock, level, optname, optval, optlen, &err)) {
    return err;
  }


  match (level) {

    /* Level: SOL_SOCKET */
    SOL_SOCKET =>
      match (optname) {

        /* SO_ACCEPTCONN is get-only */

        /* The option flags */
        SO_BROADCAST =>
        SO_KEEPALIVE =>

        SO_REUSEADDR =>

          if ((optname == SO_BROADCAST) &&
              (NETCONNTYPE_GROUP(sock.conn.netconntype) != NETCONN_UDP)) {
            done_socket(sock);
            return ENOPROTOOPT;
          }

          optname = lwip_sockopt_to_ipopt(optname);

          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, optlen, int);
          if (*(const int *)optval) {
            ip_set_option(sock.conn.pcb.ip, optname);
          } else {
            ip_reset_option(sock.conn.pcb.ip, optname);
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, SOL_SOCKET, optname=0x%x, ..) . %s\n",
                                      s, optname, (*(const int *)optval ? "on" : "off")));
          break;

          /* SO_TYPE is get-only */
          /* SO_ERROR is get-only */


        SO_SNDTIMEO => {
          long ms_long;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, optlen, LWIP_SO_SNDRCVTIMEO_OPTTYPE);
          ms_long = LWIP_SO_SNDRCVTIMEO_GET_MS(optval);
          if (ms_long < 0) {
            done_socket(sock);
            return EINVAL;
          }
          netconn_set_sendtimeout(sock.conn, ms_long);
          break;
        }


        SO_RCVTIMEO => {
          long ms_long;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, optlen, LWIP_SO_SNDRCVTIMEO_OPTTYPE);
          ms_long = LWIP_SO_SNDRCVTIMEO_GET_MS(optval);
          if (ms_long < 0) {
            done_socket(sock);
            return EINVAL;
          }
          netconn_set_recvtimeout(sock.conn, (u32)ms_long);
          break;
        }


        SO_RCVBUF =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, optlen, int);
          netconn_set_recvbufsize(sock.conn, *(const int *)optval);
          break;


        SO_LINGER => {
          const linger: &mut linger = (const struct linger *)optval;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, optlen, struct linger);
          if (linger.l_onoff) {
            lingersec: i32 = linger.l_linger;
            if (lingersec < 0) {
              done_socket(sock);
              return EINVAL;
            }
            if (lingersec > 0xFFFF) {
              lingersec = 0xFFFF;
            }
            sock.conn.linger = (i16)lingersec;
          } else {
            sock.conn.linger = -1;
          }
        }
        break;


        SO_NO_CHECK =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, int, NETCONN_UDP);

          if (udp_is_flag_set(sock.conn.pcb.udp, UDP_FLAGS_UDPLITE)) {
            /* this flag is only available for UDP, not for UDP lite */
            done_socket(sock);
            return EAFNOSUPPORT;
          }

          if (*(const int *)optval) {
            udp_set_flags(sock.conn.pcb.udp, UDP_FLAGS_NOCHKSUM);
          } else {
            udp_clear_flags(sock.conn.pcb.udp, UDP_FLAGS_NOCHKSUM);
          }
          break;

        SO_BINDTODEVICE => {
          const iface: &mut ifreq;
          n: &mut NetIfc = NULL;

          LWIP_SOCKOPT_CHECK_OPTLEN_CONN(sock, optlen, struct ifreq);

          iface = (const struct ifreq *)optval;
          if (iface.ifr_name[0] != 0) {
            n = netif_find(iface.ifr_name);
            if (n == NULL) {
              done_socket(sock);
              return ENODEV;
            }
          }

          match (NETCONNTYPE_GROUP(netconn_type(sock.conn))) {

            NETCONN_TCP =>
              tcp_bind_netif(sock.conn.pcb.tcp, n);
              break;


            NETCONN_UDP =>
              udp_bind_netif(sock.conn.pcb.udp, n);
              break;


            NETCONN_RAW =>
              raw_bind_netif(sock.conn.pcb.raw, n);
              break;

            _ =>
              LWIP_ASSERT("Unhandled netconn type in SO_BINDTODEVICE", 0);
              break;
          }
        }
        break;
        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, SOL_SOCKET, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;

    /* Level: IPPROTO_IP */
    IPPROTO_IP =>
      match (optname) {
        IP_TTL =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, optlen, int);
          sock.conn.pcb.ip.ttl = (*(const int *)optval);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_IP, IP_TTL, ..) . %d\n",
                                      s, sock.conn.pcb.ip.ttl));
          break;
        IP_TOS =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, optlen, int);
          sock.conn.pcb.ip.tos = (*(const int *)optval);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_IP, IP_TOS, ..). %d\n",
                                      s, sock.conn.pcb.ip.tos));
          break;

        IP_PKTINFO =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, int, NETCONN_UDP);
          if (*(const int *)optval) {
            sock.conn.flags |= NETCONN_FLAG_PKTINFO;
          } else {
            sock.conn.flags &= ~NETCONN_FLAG_PKTINFO;
          }
          break;


        IP_MULTICAST_TTL =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, u8, NETCONN_UDP);
          udp_set_multicast_ttl(sock.conn.pcb.udp, (*optval));
          break;
        IP_MULTICAST_IF => {
          ip4_addr if_addr;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, struct in_addr, NETCONN_UDP);
          inet_addr_to_ip4addr(&if_addr, (const struct in_addr *)optval);
          udp_set_multicast_netif_addr(sock.conn.pcb.udp, &if_addr);
        }
        break;
        IP_MULTICAST_LOOP =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, u8, NETCONN_UDP);
          if (*optval) {
            udp_set_flags(sock.conn.pcb.udp, UDP_FLAGS_MULTICAST_LOOP);
          } else {
            udp_clear_flags(sock.conn.pcb.udp, UDP_FLAGS_MULTICAST_LOOP);
          }
          break;


        IP_ADD_MEMBERSHIP =>
        IP_DROP_MEMBERSHIP => {
          /* If this is a TCP or a RAW socket, ignore these options. */
          igmp_err: err_t;
          const imr: &mut ip_mreq = (const struct ip_mreq *)optval;
          ip4_addr if_addr;
          ip4_addr multi_addr;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, struct ip_mreq, NETCONN_UDP);
          inet_addr_to_ip4addr(&if_addr, &imr.imr_interface);
          inet_addr_to_ip4addr(&multi_addr, &imr.imr_multiaddr);
          if (optname == IP_ADD_MEMBERSHIP) {
            if (!lwip_socket_register_membership(s, &if_addr, &multi_addr)) {
              /* cannot track membership (out of memory) */
              err = ENOMEM;
              igmp_err = ERR_OK;
            } else {
              igmp_err = igmp_joingroup(&if_addr, &multi_addr);
            }
          } else {
            igmp_err = igmp_leavegroup(&if_addr, &multi_addr);
            lwip_socket_unregister_membership(s, &if_addr, &multi_addr);
          }
          if (igmp_err != ERR_OK) {
            err = EADDRNOTAVAIL;
          }
        }
        break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_IP, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;


    /* Level: IPPROTO_TCP */
    IPPROTO_TCP =>
      /* Special case: all IPPROTO_TCP option take an int */
      LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, int, NETCONN_TCP);
      if (sock.conn.pcb.tcp.state == LISTEN) {
        done_socket(sock);
        return EINVAL;
      }
      match (optname) {
        TCP_NODELAY =>
          if (*(const int *)optval) {
            tcp_nagle_disable(sock.conn.pcb.tcp);
          } else {
            tcp_nagle_enable(sock.conn.pcb.tcp);
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_TCP, TCP_NODELAY) . %s\n",
                                      s, (*(const int *)optval) ? "on" : "off") );
          break;
        TCP_KEEPALIVE =>
          sock.conn.pcb.tcp.keep_idle = (u32)(*(const int *)optval);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_TCP, TCP_KEEPALIVE) . %"U32_F"\n",
                                      s, sock.conn.pcb.tcp.keep_idle));
          break;


        TCP_KEEPIDLE =>
          sock.conn.pcb.tcp.keep_idle = 1000 * (u32)(*(const int *)optval);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_TCP, TCP_KEEPIDLE) . %"U32_F"\n",
                                      s, sock.conn.pcb.tcp.keep_idle));
          break;
        TCP_KEEPINTVL =>
          sock.conn.pcb.tcp.keep_intvl = 1000 * (u32)(*(const int *)optval);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_TCP, TCP_KEEPINTVL) . %"U32_F"\n",
                                      s, sock.conn.pcb.tcp.keep_intvl));
          break;
        TCP_KEEPCNT =>
          sock.conn.pcb.tcp.keep_cnt = (u32)(*(const int *)optval);
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_TCP, TCP_KEEPCNT) . %"U32_F"\n",
                                      s, sock.conn.pcb.tcp.keep_cnt));
          break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_TCP, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;



    /* Level: IPPROTO_IPV6 */
    IPPROTO_IPV6 =>
      match (optname) {
        IPV6_V6ONLY =>
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, optlen, int);
          if (*(const int *)optval) {
            netconn_set_ipv6only(sock.conn, 1);
          } else {
            netconn_set_ipv6only(sock.conn, 0);
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_IPV6, IPV6_V6ONLY, ..) . %d\n",
                                      s, (netconn_get_ipv6only(sock.conn) ? 1 : 0)));
          break;

        IPV6_JOIN_GROUP =>
        IPV6_LEAVE_GROUP => {
          /* If this is a TCP or a RAW socket, ignore these options. */
          mld6_err: err_t;
          netif: &mut NetIfc;
          multi_addr: ip6_addr_t;
          const imr: &mut ipv6_mreq = (const struct ipv6_mreq *)optval;
          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, struct ipv6_mreq, NETCONN_UDP);
          inet6_addr_to_ip6addr(&multi_addr, &imr.ipv6mr_multiaddr);
          LWIP_ASSERT("Invalid netif index", imr.ipv6mr_interface <= 0xFFu);
          netif = netif_get_by_index(imr.ipv6mr_interface);
          if (netif == NULL) {
            err = EADDRNOTAVAIL;
            break;
          }

          if (optname == IPV6_JOIN_GROUP) {
            if (!lwip_socket_register_mld6_membership(s, imr.ipv6mr_interface, &multi_addr)) {
              /* cannot track membership (out of memory) */
              err = ENOMEM;
              mld6_err = ERR_OK;
            } else {
              mld6_err = mld6_joingroup_netif(netif, &multi_addr);
            }
          } else {
            mld6_err = mld6_leavegroup_netif(netif, &multi_addr);
            lwip_socket_unregister_mld6_membership(s, imr.ipv6mr_interface, &multi_addr);
          }
          if (mld6_err != ERR_OK) {
            err = EADDRNOTAVAIL;
          }
        }
        break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_IPV6, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;



    /* Level: IPPROTO_UDPLITE */
    IPPROTO_UDPLITE =>
      /* Special case: all IPPROTO_UDPLITE option take an int */
      LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB(sock, optlen, int);
      /* If this is no UDP lite socket, ignore any options. */
      if (!NETCONNTYPE_ISUDPLITE(netconn_type(sock.conn))) {
        done_socket(sock);
        return ENOPROTOOPT;
      }
      match (optname) {
        UDPLITE_SEND_CSCOV =>
          if ((*(const int *)optval != 0) && ((*(const int *)optval < 8) || (*(const int *)optval > 0xffff))) {
            /* don't allow illegal values! */
            sock.conn.pcb.udp.chksum_len_tx = 8;
          } else {
            sock.conn.pcb.udp.chksum_len_tx =  * (const int *)optval;
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_UDPLITE, UDPLITE_SEND_CSCOV) . %d\n",
                                      s, (*(const int *)optval)) );
          break;
        UDPLITE_RECV_CSCOV =>
          if ((*(const int *)optval != 0) && ((*(const int *)optval < 8) || (*(const int *)optval > 0xffff))) {
            /* don't allow illegal values! */
            sock.conn.pcb.udp.chksum_len_rx = 8;
          } else {
            sock.conn.pcb.udp.chksum_len_rx =  * (const int *)optval;
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_UDPLITE, UDPLITE_RECV_CSCOV) . %d\n",
                                      s, (*(const int *)optval)) );
          break;
        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_UDPLITE, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;

    /* Level: IPPROTO_RAW */
    IPPROTO_RAW =>
      match (optname) {

        IPV6_CHECKSUM =>
          /* It should not be possible to disable the checksum generation with ICMPv6
           * as per RFC 3542 chapter 3.1 */
          if (sock.conn.pcb.raw.protocol == IPPROTO_ICMPV6) {
            done_socket(sock);
            return EINVAL;
          }

          LWIP_SOCKOPT_CHECK_OPTLEN_CONN_PCB_TYPE(sock, optlen, int, NETCONN_RAW);
          if (*(const int *)optval < 0) {
            sock.conn.pcb.raw.chksum_reqd = 0;
          } else if (*(const int *)optval & 1) {
            /* Per RFC3542, odd offsets are not allowed */
            done_socket(sock);
            return EINVAL;
          } else {
            sock.conn.pcb.raw.chksum_reqd = 1;
            sock.conn.pcb.raw.chksum_offset =  * (const int *)optval;
          }
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_RAW, IPV6_CHECKSUM, ..) . %d\n",
                                      s, sock.conn.pcb.raw.chksum_reqd));
          break;

        _ =>
          LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, IPPROTO_RAW, UNIMPL: optname=0x%x, ..)\n",
                                      s, optname));
          err = ENOPROTOOPT;
          break;
      }  /* match (optname) */
      break;
    _ =>
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_setsockopt(%d, level=0x%x, UNIMPL: optname=0x%x, ..)\n",
                                  s, level, optname));
      err = ENOPROTOOPT;
      break;
  }  /* match (level) */

  done_socket(sock);
  return err;
}

pub fn lwip_ioctl(s: i32, long cmd, arg: &mut Vec<u8>p)
{
  sock: &mut lwip_sock = get_socket(s);
  val: u8;

  recv_avail: i32;


  if (!sock) {
    return -1;
  }

  match (cmd) {

    FIONREAD =>
      if (!argp) {
        sock_set_errno(sock, EINVAL);
        done_socket(sock);
        return -1;
      }

      if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) != NETCONN_TCP) {
        nb: &mut netbuf;
        if (sock.lastdata.netbuf) {
          nb = sock.lastdata.netbuf;
          *((int *)argp) = nb.p.tot_len;
        } else {
          rxbuf: &mut netbuf;
          err: err_t = netconn_recv_udp_raw_netbuf_flags(sock.conn, &rxbuf, NETCONN_DONTBLOCK);
          if (err != ERR_OK) {
            *((int *)argp) = 0;
          } else {
            sock.lastdata.netbuf = rxbuf;
            *((int *)argp) = rxbuf.p.tot_len;
          }
        }
        done_socket(sock);
        return 0;
      }



      /* we come here if either LWIP_FIONREAD_LINUXMODE==0 or this is a TCP socket */
      SYS_ARCH_GET(sock.conn.recv_avail, recv_avail);
      if (recv_avail < 0) {
        recv_avail = 0;
      }

      /* Check if there is data left from the last recv operation. /maq 041215 */
      if (sock.lastdata.netbuf) {
        if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP) {
          recv_avail += sock.lastdata.pbuf.tot_len;
        } else {
          recv_avail += sock.lastdata.netbuf.p.tot_len;
        }
      }
      *((int *)argp) = recv_avail;

      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_ioctl(%d, FIONREAD, %p) = %"U16_F"\n", s, argp, *((u16 *)argp)));
      sock_set_errno(sock, 0);
      done_socket(sock);
      return 0;
 /* LWIP_SO_RCVBUF */
      break;



    case (long)FIONBIO:
      val = 0;
      if (argp && *(int *)argp) {
        val = 1;
      }
      netconn_set_nonblocking(sock.conn, val);
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_ioctl(%d, FIONBIO, %d)\n", s, val));
      sock_set_errno(sock, 0);
      done_socket(sock);
      return 0;

    _ =>
      break;
  } /* match (cmd) */
  LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_ioctl(%d, UNIMPL: 0x%lx, %p)\n", s, cmd, argp));
  sock_set_errno(sock, ENOSYS); /* not yet implemented */
  done_socket(sock);
  return -1;
}

/* A minimal implementation of fcntl.
 * Currently only the commands F_GETFL and F_SETFL are implemented.
 * The flag O_NONBLOCK and access modes are supported for F_GETFL, only
 * the flag O_NONBLOCK is implemented for F_SETFL.
 */
pub fn lwip_fcntl(s: i32, cmd: i32, val: i32)
{
  sock: &mut lwip_sock = get_socket(s);
  ret: i32 = -1;
  op_mode: i32 = 0;

  if (!sock) {
    return -1;
  }

  match (cmd) {
    F_GETFL =>
      ret = netconn_is_nonblocking(sock.conn) ? O_NONBLOCK : 0;
      sock_set_errno(sock, 0);

      if (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP) {

        LOCK_TCPIP_CORE();

        SYS_ARCH_DECL_PROTECT(lev);
        /* the proper thing to do here would be to get into the tcpip_thread,
           but locking should be OK as well since we only *read* some flags */
        SYS_ARCH_PROTECT(lev);


        if (sock.conn.pcb.tcp) {
          if (!(sock.conn.pcb.tcp.flags & TF_RXCLOSED)) {
            op_mode |= O_RDONLY;
          }
          if (!(sock.conn.pcb.tcp.flags & TF_FIN)) {
            op_mode |= O_WRONLY;
          }
        }


        UNLOCK_TCPIP_CORE();

        SYS_ARCH_UNPROTECT(lev);

      } else {
        op_mode |= O_RDWR;
      }

      /* ensure O_RDWR for (O_RDONLY|O_WRONLY) != O_RDWR cases */
      ret |= (op_mode == (O_RDONLY | O_WRONLY)) ? O_RDWR : op_mode;

      break;
    F_SETFL =>
      /* Bits corresponding to the file access mode and the file creation flags [..] that are set in arg shall be ignored */
      val &= ~(O_RDONLY | O_WRONLY | O_RDWR);
      if ((val & ~O_NONBLOCK) == 0) {
        /* only O_NONBLOCK, all other bits are zero */
        netconn_set_nonblocking(sock.conn, val & O_NONBLOCK);
        ret = 0;
        sock_set_errno(sock, 0);
      } else {
        sock_set_errno(sock, ENOSYS); /* not yet implemented */
      }
      break;
    _ =>
      LWIP_DEBUGF(SOCKETS_DEBUG, ("lwip_fcntl(%d, UNIMPL: %d, %d)\n", s, cmd, val));
      sock_set_errno(sock, ENOSYS); /* not yet implemented */
      break;
  }
  done_socket(sock);
  return ret;
}


pub fn fcntl(s: i32, cmd: i32, ...)
{
  va_list ap;
  val: i32;

  va_start(ap, cmd);
  val = va_arg(ap, int);
  va_end(ap);
  return lwip_fcntl(s, cmd, val);
}


const char *
lwip_inet_ntop(af: i32, src: &Vec<u8>, dst: &mut String, size: socklen_t)
{
  ret: &String = NULL;
  size_int: i32 = size;
  if (size_int < 0) {
    set_errno(ENOSPC);
    return NULL;
  }
  match (af) {

    AF_INET =>
      ret = ip4addr_ntoa_r((const ip4_addr *)src, dst, size_int);
      if (ret == NULL) {
        set_errno(ENOSPC);
      }
      break;


    AF_INET6 =>
      ret = ip6addr_ntoa_r((const ip6_addr_t *)src, dst, size_int);
      if (ret == NULL) {
        set_errno(ENOSPC);
      }
      break;

    _ =>
      set_errno(EAFNOSUPPORT);
      break;
  }
  return ret;
}

pub fn lwip_inet_pton(af: i32, src: &String, dst: &mut ())
{
  err: i32;
  match (af) {

    AF_INET =>
      err = ip4addr_aton(src, (ip4_addr *)dst);
      break;


    AF_INET6 => {
      /* convert into temporary variable since might: ip6_addr_t be larger
         than in6_addr when scopes are enabled */
      addr: ip6_addr_t;
      err = ip6addr_aton(src, &addr);
      if (err) {
        memcpy(dst, &addr.addr, sizeof(addr.addr));
      }
      break;
    }

    _ =>
      err = -1;
      set_errno(EAFNOSUPPORT);
      break;
  }
  return err;
}


/* Register a new IGMP membership. On socket close, the membership is dropped automatically.
 *
 * ATTENTION: this function is called from tcpip_thread (or under CORE_LOCK).
 *
 * @return 1 on success, 0 on failure
 */
static int
lwip_socket_register_membership(s: i32,  if_addr: &mut ip4_addr,  multi_addr: &mut ip4_addr)
{
  sock: &mut lwip_sock = get_socket(s);
  i: i32;

  if (!sock) {
    return 0;
  }

  for (i = 0; i < LWIP_SOCKET_MAX_MEMBERSHIPS; i+= 1) {
    if (socket_ipv4_multicast_memberships[i].sock == NULL) {
      socket_ipv4_multicast_memberships[i].sock = sock;
      ip4_addr_copy(socket_ipv4_multicast_memberships[i].if_addr, *if_addr);
      ip4_addr_copy(socket_ipv4_multicast_memberships[i].multi_addr, *multi_addr);
      done_socket(sock);
      return 1;
    }
  }
  done_socket(sock);
  return 0;
}

/* Unregister a previously registered membership. This prevents dropping the membership
 * on socket close.
 *
 * ATTENTION: this function is called from tcpip_thread (or under CORE_LOCK).
 */
pub fn
lwip_socket_unregister_membership(s: i32,  if_addr: &mut ip4_addr,  multi_addr: &mut ip4_addr)
{
  sock: &mut lwip_sock = get_socket(s);
  i: i32;

  if (!sock) {
    return;
  }

  for (i = 0; i < LWIP_SOCKET_MAX_MEMBERSHIPS; i+= 1) {
    if ((socket_ipv4_multicast_memberships[i].sock == sock) &&
        ip4_addr_cmp(&socket_ipv4_multicast_memberships[i].if_addr, if_addr) &&
        ip4_addr_cmp(&socket_ipv4_multicast_memberships[i].multi_addr, multi_addr)) {
      socket_ipv4_multicast_memberships[i].sock = NULL;
      ip4_addr_set_zero(&socket_ipv4_multicast_memberships[i].if_addr);
      ip4_addr_set_zero(&socket_ipv4_multicast_memberships[i].multi_addr);
      break;
    }
  }
  done_socket(sock);
}

/* Drop all memberships of a socket that were not dropped explicitly via setsockopt.
 *
 * ATTENTION: this function is NOT called from tcpip_thread (or under CORE_LOCK).
 */
pub fn
lwip_socket_drop_registered_memberships(s: i32)
{
  sock: &mut lwip_sock = get_socket(s);
  i: i32;

  if (!sock) {
    return;
  }

  for (i = 0; i < LWIP_SOCKET_MAX_MEMBERSHIPS; i+= 1) {
    if (socket_ipv4_multicast_memberships[i].sock == sock) {
      ip_addr_t multi_addr, if_addr;
      ip_addr_copy_from_ip4(multi_addr, socket_ipv4_multicast_memberships[i].multi_addr);
      ip_addr_copy_from_ip4(if_addr, socket_ipv4_multicast_memberships[i].if_addr);
      socket_ipv4_multicast_memberships[i].sock = NULL;
      ip4_addr_set_zero(&socket_ipv4_multicast_memberships[i].if_addr);
      ip4_addr_set_zero(&socket_ipv4_multicast_memberships[i].multi_addr);

      netconn_join_leave_group(sock.conn, &multi_addr, &if_addr, NETCONN_LEAVE);
    }
  }
  done_socket(sock);
}



/* Register a new MLD6 membership. On socket close, the membership is dropped automatically.
 *
 * ATTENTION: this function is called from tcpip_thread (or under CORE_LOCK).
 *
 * @return 1 on success, 0 on failure
 */
static int
lwip_socket_register_mld6_membership(s: i32,  if_idx: i32,  multi_addr: &mut ip6_addr_t)
{
  sock: &mut lwip_sock = get_socket(s);
  i: i32;

  if (!sock) {
    return 0;
  }

  for (i = 0; i < LWIP_SOCKET_MAX_MEMBERSHIPS; i+= 1) {
    if (socket_ipv6_multicast_memberships[i].sock == NULL) {
      socket_ipv6_multicast_memberships[i].sock   = sock;
      socket_ipv6_multicast_memberships[i].if_idx = if_idx;
      ip6_addr_copy(socket_ipv6_multicast_memberships[i].multi_addr, *multi_addr);
      done_socket(sock);
      return 1;
    }
  }
  done_socket(sock);
  return 0;
}

/* Unregister a previously registered MLD6 membership. This prevents dropping the membership
 * on socket close.
 *
 * ATTENTION: this function is called from tcpip_thread (or under CORE_LOCK).
 */
pub fn
lwip_socket_unregister_mld6_membership(s: i32,  if_idx: i32,  multi_addr: &mut ip6_addr_t)
{
  sock: &mut lwip_sock = get_socket(s);
  i: i32;

  if (!sock) {
    return;
  }

  for (i = 0; i < LWIP_SOCKET_MAX_MEMBERSHIPS; i+= 1) {
    if ((socket_ipv6_multicast_memberships[i].sock   == sock) &&
        (socket_ipv6_multicast_memberships[i].if_idx == if_idx) &&
        ip6_addr_cmp(&socket_ipv6_multicast_memberships[i].multi_addr, multi_addr)) {
      socket_ipv6_multicast_memberships[i].sock   = NULL;
      socket_ipv6_multicast_memberships[i].if_idx = NETIF_NO_INDEX;
      ip6_addr_set_zero(&socket_ipv6_multicast_memberships[i].multi_addr);
      break;
    }
  }
  done_socket(sock);
}

/* Drop all MLD6 memberships of a socket that were not dropped explicitly via setsockopt.
 *
 * ATTENTION: this function is NOT called from tcpip_thread (or under CORE_LOCK).
 */
pub fn
lwip_socket_drop_registered_mld6_memberships(s: i32)
{
  sock: &mut lwip_sock = get_socket(s);
  i: i32;

  if (!sock) {
    return;
  }

  for (i = 0; i < LWIP_SOCKET_MAX_MEMBERSHIPS; i+= 1) {
    if (socket_ipv6_multicast_memberships[i].sock == sock) {
      ip_addr_t multi_addr;
      if_idx: u8;

      ip_addr_copy_from_ip6(multi_addr, socket_ipv6_multicast_memberships[i].multi_addr);
      if_idx = socket_ipv6_multicast_memberships[i].if_idx;

      socket_ipv6_multicast_memberships[i].sock   = NULL;
      socket_ipv6_multicast_memberships[i].if_idx = NETIF_NO_INDEX;
      ip6_addr_set_zero(&socket_ipv6_multicast_memberships[i].multi_addr);

      netconn_join_leave_group_netif(sock.conn, &multi_addr, if_idx, NETCONN_LEAVE);
    }
  }
  done_socket(sock);
}



