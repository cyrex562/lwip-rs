/*
 * @file
 * Socket API (to be used from non-TCPIP threads)
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

//

/* If your port already typedef's sa_family_t, define SA_FAMILY_T_DEFINED
to prevent this code from redefining it. */

// typedef sa_family_t: u8;

/* If your port already typedef's in_port_t, define IN_PORT_T_DEFINED
to prevent this code from redefining it. */

// typedef in_port_t: u16;

//  members are in network byte order
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

struct sockaddr_in6 {
    pub sin6_len: u8,
    pub sin6_family: u8,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

pub struct sockaddr {
    pub sa_len: u8,
    pub sa_family: u8,
    pub sa_data: [char; 14],
}

pub struct sockaddr_storage {
    pub s2_len: u8,
    pub ss_family: u8,
    pub s2_data1: [char; 2],
    pub s2_data2: [u32; 3],
    pub s2_data3: [u32; 3],
}

/* If your port already typedef's socklen_t, define SOCKLEN_T_DEFINED
to prevent this code from redefining it. */

// typedef socklen_t: u32;

pub const IOV_MAX: u32 = 0xFFFF;
// #elif IOV_MAX > 0xFFFF
// #error "IOV_MAX larger than supported by LwIP"

pub struct iovec {
    pub iov_base: Vec<u8>,
    pub iov_len: usize,
}

pub struct msghdr {
    pub msg_name: String,
    pub msg_namelen: usize,
    pub msg_iov: iovec,
    pub msg_iovlen: usize,
    pub msg_control: Vec<u8>,
    pub msg_controllen: usize,
    pub msg_flags: i32,
}

//  struct msghdr.msg_flags bit field values
pub const MSG_TRUNC: u32 = 0x04;
pub const MSG_CTRUNC: u32 = 0x08;

//  RFC 3542, Section 20: Ancillary Data
pub struct cmsghdr {
    pub cmsd_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
/* Data section follows header and possible padding, typically referred to as
cmsg_data: [u8;]; */

/* cmsg header/data alignment. NOTE: we align to native word size (double word
size on 16-bit arch) so structures are not placed at an unaligned address.
16-bit arch needs double word to ensure 32-bit alignment because socklen_t
could be 32 bits. If we ever have cmsg data with a 64-bit variable, alignment
will need to increase long long */
// pub fn ALIGN_H(size: usize) -> usize{ (((size) + sizeof - 1) & !(sizeof-1))}
// #define ALIGN_D(size) ALIGN_H(size)

// #define CMSG_FIRSTHDR(mhdr) \
//           ((mhdr).msg_controllen >= sizeof(cmsghdr) ? \
//            (struct cmsghdr *)(mhdr).msg_control : \
//            (struct cmsghdr *)None)

// #define CMSG_NXTHDR(mhdr, cmsg) \
//         (((cmsg) == None) ? CMSG_FIRSTHDR(mhdr) : \
//          (((cmsg) + ALIGN_H((cmsg).cmsg_len) \
//                             + ALIGN_D(sizeof(cmsghdr)) > \
//            ((mhdr).msg_control) + (mhdr).msg_controllen) ? \
//           (struct cmsghdr *)None : \
//           (struct cmsghdr *)(((cmsg) + \
//                                       ALIGN_H((cmsg).cmsg_len)))))

// #define CMSG_DATA(cmsg) (((cmsg) + \
//                          ALIGN_D(sizeof(cmsghdr))))

// #define CMSG_SPACE(length) (ALIGN_D(sizeof(cmsghdr)) + \
//                             ALIGN_H(length))

// #define CMSG_LEN(length) (ALIGN_D(sizeof(cmsghdr)) + \
//                            length)

//  Set socket options argument
pub const IFNAMSIZ: u32 = NETIF_NAMESIZE;
pub struct ifreq {
    pub ifr_name: String, //  Interface name
}

//  Socket protocol types (TCP/UDP/RAW)
pub const SOCK_STREAM: u32 = 1;
pub const SOCK_DGRAM: u32 = 2;
pub const SOCK_RAW: u32 = 3;

/*
 * Option flags per-socket. These must match the SOF_ flags in ip.h (checked in init.c)
 */
pub const SO_REUSEADDR: u32 = 0x0004; //  Allow local address reuse
pub const SO_KEEPALIVE: u32 = 0x0008; //  keep connections alive
pub const SO_BROADCAST: u32 = 0x0020; //  permit to send and to receive broadcast messages (see ip_sof_broadcast option)

/*
 * Additional options, not kept in so_options.
 */
pub const SO_DEBUG: u32 = 0x0001; //  Unimplemented: turn on debugging info recording
pub const SO_DEBUG: u32 = 0x0001;
pub const SO_ACCEPTCONN: u32 = 0x0002; //  socket has had listen()
pub const SO_DONTROUTE: u32 = 0x0010; //  Unimplemented: just use interface addresses
pub const SO_USELOOPBACK: u32 = 0x0040; //  Unimplemented: bypass hardware when possible
pub const SO_LINGER: u32 = 0x0080; //  linger on close if data present
                                   // #define SO_DONTLINGER   ((!SO_LINGER))
pub const SO_OOBINLINE: u32 = 0x0100; //  Unimplemented: leave received OOB data in line

pub const SO_REUSEPORT: u32 = 0x0200; //  Unimplemented: allow local address & port reuse
pub const SO_SNDBUF: u32 = 0x1001; //  Unimplemented: send buffer size
pub const SO_RCVBUF: u32 = 0x1002; //  receive buffer size
pub const SO_SNDLOWAT: u32 = 0x1003; //  Unimplemented: send low-water mark
pub const SO_RCVLOWAT: u32 = 0x1004; //  Unimplemented: receive low-water mark
pub const SO_SNDTIMEO: u32 = 0x1005; //  send timeout
pub const SO_RCVTIMEO: u32 = 0x1006; //  receive timeout
pub const SO_ERROR: u32 = 0x1007; //  get error status and clear
pub const SO_TYPE: u32 = 0x1008; //  get socket type
pub const SO_CONTIMEO: u32 = 0x1009; //  Unimplemented: connect timeout
pub const SO_NO_CHECK: u32 = 0x100a; //  don't create UDP checksum
pub const SO_BINDTODEVICE: u32 = 0x100b; //  bind to device

/*
 * Structure used for manipulating linger option.
 */
pub struct linger {
    pub letl_onoff: i32, //  option on/off
    pub l_linger: i32,   //  linger time in seconds
}

/*
 * Level number for (get/set)sockopt() to apply to socket itself.
 */
pub const SOL_SOCKET: u32 = 0xfff; //  options for socket level

pub const AF_UNSPEC: u32 = 0;
pub const AF_INET: u32 = 2;

pub const AF_INET6: u32 = 10;
//  LWIP_IPV6
pub const AF_INET6: u32 = AF_UNSPEC;

pub const PF_INET: u32 = AF_INET;
pub const PF_INET6: u32 = AF_INET6;
pub const PF_UNSPEC: u32 = AF_UNSPEC;

pub const IPPROTO_IP: u32 = 0;
pub const IPPROTO_ICMP: u32 = 1;
pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 17;

pub const IPPROTO_IPV6: u32 = 41;
pub const IPPROTO_ICMPV6: u32 = 58;

pub const IPPROTO_UDPLITE: u32 = 136;
pub const IPPROTO_RAW: u32 = 255;

//  Flags we can use with send and recv.
pub const MSG_PEEK: u32 = 0x01; //  Peeks at an incoming message
pub const MSG_WAITALL: u32 = 0; //  Unimplemented: Requests that the function block until the full amount of data requested can be returned
pub const MSG_OOB: u32 = 0x04; //  Unimplemented: Requests out-of-band data. The significance and semantics of out-of-band data are protocol-specific
pub const MSG_DONTWAIT: u32 = 0x08; //  Nonblocking i/o for this operation only
pub const MSG_MORE: u32 = 0x10; //  Sender will send more
pub const MSG_NOSIGNAL: u32 = 0x20; //  Uninmplemented: Requests not to send the SIGPIPE signal if an attempt to send is made on a stream-oriented socket that is no longer connected.

/*
 * Options for level IPPROTO_IP
 */
pub const IP_TOS: u32 = 1;
pub const IP_TTL: u32 = 2;
pub const IP_PKTINFO: u32 = 8;

/*
 * Options for level IPPROTO_TCP
 */
pub const TCP_NODELAY: u32 = 0x01; //  don't delay send to coalesce packets
pub const TCP_KEEPALIVE: u32 = 0x02; //  send KEEPALIVE probes when idle for pcb.keep_idle milliseconds
pub const TCP_KEEPALIVE: u32 = 0;
pub const TCP_KEEPIDLE: u32 = 0x03; //  set pcb.keep_idle  - Same as TCP_KEEPALIVE, but use seconds for get/setsockopt
pub const TCP_KEEPINTVL: u32 = 0x04; //  set pcb.keep_intvl - Use seconds for get/setsockopt
pub const TCP_KEEPCNT: u32 = 0x05; //  set pcb.keep_cnt   - Use number of probes sent for get/setsockopt

/*
 * Options for level IPPROTO_IPV6
 */
pub const IPV6_CHECKSUM: u32 = 7; //  RFC3542: calculate and insert the ICMPv6 checksum for raw sockets.
pub const IPV6_V6ONLY: u32 = 27; //  RFC3493: boolean control to restrict AF_INET6 sockets to IPv6 communications only.

/*
 * Options for level IPPROTO_UDPLITE
 */
pub const UDPLITE_SEND_CSCOV: u32 = 0x01; //  sender checksum coverage
pub const UDPLITE_RECV_CSCOV: u32 = 0x02; //  minimal receiver checksum coverage

/*
 * Options and types for UDP multicast traffic handling
 */
pub const IP_MULTICAST_TTL: u32 = 5;
pub const IP_MULTICAST_IF: u32 = 6;
pub const IP_MULTICAST_LOOP: u32 = 7;

/*
 * Options and types related to multicast membership
 */
pub const IP_ADD_MEMBERSHIP: u32 = 3;
pub const IP_DROP_MEMBERSHIP: u32 = 4;

pub struct ip_mreq {
    pub imr_multiaddr: in_addr, //  IP multicast address of group
    pub imr_interface: in_addr, //  local IP address of interface
}

pub struct in_pktinfo {
    pub ipi_ifindex: int,  //  Interface index
    pub ipi_addr: in_addr, //  Destination (from header) address
}

/*
 * Options and types related to IPv6 multicast membership
 */
pub const IPV6_JOIN_GROUP: u32 = 12;
pub const IPV6_ADD_MEMBERSHIP: u32 = IPV6_JOIN_GROUP;
pub const IPV6_LEAVE_GROUP: u32 = 13;
pub const IPV6_DROP_MEMBERSHIP: u32 = IPV6_LEAVE_GROUP;

pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr, //   IPv6 multicast addr
    pub ipv6mr_interface: i32,      //   interface index, or 0
}

/*
 * The Type of Service provides an indication of the abstract
 * parameters of the quality of service desired.  These parameters are
 * to be used to guide the selection of the actual service parameters
 * when transmitting a datagram through a particular network.  Several
 * networks offer service precedence, which somehow treats high
 * precedence traffic as more important than other traffic (generally
 * by accepting only traffic above a certain precedence at time of high
 * load).  The major choice is a three way tradeoff between low-delay,
 * high-reliability, and high-throughput.
 * The use of the Delay, Throughput, and Reliability indications may
 * increase the cost (in some sense) of the service.  In many networks
 * better performance for one of these parameters is coupled with worse
 * performance on another.  Except for very unusual cases at most two
 * of these three indications should be set.
 */
pub const IPTOS_TOS_MASK: u32 = 0x1E;
// #define IPTOS_TOS(tos)          ((tos) & IPTOS_TOS_MASK)
pub const IPTOS_LOWDELAY: u32 = 0x10;
pub const IPTOS_THROUGHPUT: u32 = 0x08;
pub const IPTOS_RELIABILITY: u32 = 0x04;
pub const IPTOS_LOWCOST: u32 = 0x02;
pub const IPTOS_MINCOST: u32 = IPTOS_LOWCOST;

/*
 * The Network Control precedence designation is intended to be used
 * within a network only.  The actual use and control of that
 * designation is up to each network. The Internetwork Control
 * designation is intended for use by gateway control originators only.
 * If the actual use of these precedence designations is of concern to
 * a particular network, it is the responsibility of that network to
 * control the access to, and use of, those precedence designations.
 */
pub const IPTOS_PREC_MASK: u32 = 0xe0;
// #define IPTOS_PREC(tos)                ((tos) & IPTOS_PREC_MASK)
pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;
pub const IPTOS_PREC_INTERNETCONTROL: u32 = 0xc0;
pub const IPTOS_PREC_CRITIC_ECP: u32 = 0xa0;
pub const IPTOS_PREC_FLASHOVERRIDE: u32 = 0x80;
pub const IPTOS_PREC_FLASH: u32 = 0x60;
pub const IPTOS_PREC_IMMEDIATE: u32 = 0x40;
pub const IPTOS_PREC_PRIORITY: u32 = 0x20;
pub const IPTOS_PREC_ROUTINE: u32 = 0x00;

/*
 * Commands for ioctlsocket(),  taken from the BSD file fcntl.h.
 * lwip_ioctl only supports FIONREAD and FIONBIO, for now
 *
 * Ioctl's have the command encoded in the lower word,
 * and the size of any in or out parameters in the upper
 * word.  The high 2 bits of the upper word are used
 * to encode the in/out status of the parameter; for now
 * we restrict parameters to at most 128 bytes.
 */

pub const IOCPARM_MASK: u32 = 0x7f; //  parameters must be < 128 bytes
pub const IOC_VOID: u32 = 0x20000000; //  no parameters
pub const IOC_OUT: u32 = 0x40000000; //  copy out parameters
pub const IOC_IN: u32 = 0x80000000; //  copy in parameters
pub const IOC_INOUT: u32 = (IOC_IN | IOC_OUT);
/* 0x20000000 distinguishes new &
old ioctl's */
// pub fn _IO(x: u32,y: u32) -> u32   {     ((IOC_VOID|((x)<<8)|(y)))}

// #define _IOR(x,y,t)     ((IOC_OUT|((sizeof(t)&IOCPARM_MASK)<<16)|((x)<<8)|(y)))

// #define _IOW(x,y,t)     ((IOC_IN|((sizeof(t)&IOCPARM_MASK)<<16)|((x)<<8)|(y)))

// pub const FIONREAD: u32 = _IOR;('f', 127,  long) //  get # bytes to read
// pub const FIONBIO: u32 = _IOW;('f', 126,  long) //  set/clear non-blocking i/o
//  Socket I/O Controls: unimplemented

// pub const SIOCSHIWAT: u32 = _IOW;('s',  0,  long)  //  set high watermark
// pub const SIOCGHIWAT: u32 = _IOR;('s',  1,  long)  //  get high watermark
// pub const SIOCSLOWAT: u32 = _IOW;('s',  2,  long)  //  set low watermark
// pub const SIOCGLOWAT: u32 = _IOR;('s',  3,  long)  //  get low watermark
// pub const SIOCATMARK: u32 = _IOR;('s',  7,  long)  //  at oob mark?
//  commands for fnctl

// pub const F_GETFL: u32 = 3;

// pub const F_SETFL: u32 = 4;

/* File status flags and file access modes for fnctl,
these are bits in an int. */

// pub const O_NONBLOCK: u32 = 1;  //  nonblocking I/O
// pub const O_NDELAY: u32 = O_NONBLOCK; //  same as O_NONBLOCK, for compatibility
// pub const O_RDONLY: u32 = 2;

// pub const O_WRONLY: u32 = 4;

// #define O_RDWR      (O_RDONLY|O_WRONLY)

// pub const SHUT_RD: u32 = 0;
// pub const SHUT_WR: u32 = 1;
// pub const SHUT_RDWR: u32 = 2;

//  FD_SET used for lwip_select

//#undef  FD_SETSIZE
//  Make FD_SETSIZE match NUM_SOCKETS in socket.c
// pub const FD_SETSIZE: u32 = MEMP_NUM_NETCONN;
// #define LWIP_SELECT_MAXNFDS (FD_SETSIZE + LWIP_SOCKET_OFFSET)
// #define FDSETSAFESET(n, code) loop { \
//   if (((n) - LWIP_SOCKET_OFFSET < MEMP_NUM_NETCONN) && (((n) - LWIP_SOCKET_OFFSET) >= 0)) { \
//   code; }} while(0)
// #define FDSETSAFEGET(n, code) (((n) - LWIP_SOCKET_OFFSET < MEMP_NUM_NETCONN) && (((n) - LWIP_SOCKET_OFFSET) >= 0) ?\
//   (code) : 0)
// #define FD_SET(n, p)  FDSETSAFESET(n, (p).fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] = ((p).fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] |  (1 << (((n)-LWIP_SOCKET_OFFSET) & 7))))
// #define FD_CLR(n, p)  FDSETSAFESET(n, (p).fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] = ((p).fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] & !(1 << (((n)-LWIP_SOCKET_OFFSET) & 7))))
// #define FD_ISSET(n,p) FDSETSAFEGET(n, (p).fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] &   (1 << (((n)-LWIP_SOCKET_OFFSET) & 7)))
// #define FD_ZERO(p)    memset((p), 0, sizeof(*(p)))

// typedef struct fd_set
// {
//    fd_bits: char [(FD_SETSIZE+7)/8];
// } fd_set;

// #elif FD_SETSIZE < (LWIP_SOCKET_OFFSET + MEMP_NUM_NETCONN)
// #error "external FD_SETSIZE too small for number of sockets"

// pub const LWIP_SELECT_MAXNFDS: u32 = FD_SETSIZE;

//  poll-related defines and types
//  @todo: find a better way to guard the definition of these defines and types if already defined

// pub const POLLIN: u32 = 0x1;pub const POLLIN: u32 = 0x1;pub const POLLIN: u32 = 0x1;pub const POLLIN: u32 = 0x1;
// pub const POLLOUT: u32 = 0; x2pub const POLLOUT: u32 = 0; pub const POLLOUT: u32 = 0;
// pub const POLLERR: u32 = 0x4;
// pub const POLLNVAL: u32 = 0x8;
//  Below values are unimplemented
// pub const POLLRDNORM:
pub const POLLRDBAND: u32 = 0x20;
pub const POLLPRI: u32 = 0x40;
pub const POLLWRNORM: u32 = 0x80;
pub const POLLWRBAND: u32 = 0x100;
pub const POLLHUP: u32 = 0x200;
// typedef  nfds_t: i32;
// struct pollfd
// {
//   let fd: i32;
//   short events;
//   short revents;
// };

/* LWIP_TIMEVAL_PRIVATE: if you want to use the provided: timeval
 * by your system, set this to 0 and include <sys/time.h> in cc.h */

// pub const LWIP_TIMEVAL_PRIVATE: u32 = 1;

// struct timeval {
//   long    tv_sec;         //  seconds
//   long    tv_usec;        //  and microseconds
// };

// #define lwip_socket_init() //  Compatibility define, no init needed.
// pub fn  lwip_socket_thread_init(); //  LWIP_NETCONN_SEM_PER_THREAD==1: initialize thread-local semaphore
// pub fn  lwip_socket_thread_cleanup(); //  LWIP_NETCONN_SEM_PER_THREAD==1: destroy thread-local semaphore
//  This helps code parsers/code completion by not having the COMPAT functions as defines
// pub const lwip_accept: u32 = accept;
// pub const lwip_bind: u32 = bind;
// pub const lwip_shutdown: u32 = shutdown;
// pub const lwip_getpeername: u32 = getpeername;
// pub const lwip_getsockname: u32 = getsockname;
// pub const lwip_setsockopt: u32 = setsockopt;
// pub const lwip_getsockopt: u32 = getsockopt;
// pub const lwip_close: u32 = closesocket;
// pub const lwip_connect: u32 = connect;
// pub const lwip_listen: u32 = listen;
// pub const lwip_recv: u32 = recv;
// pub const lwip_recvmsg: u32 = recvmsg;
// pub const lwip_recvfrom: u32 = recvfrom;
// pub const lwip_send: u32 = send;
// pub const lwip_sendmsg: u32 = sendmsg;
// pub const lwip_sendto: u32 = sendto;
// pub const lwip_socket: u32 = socket;

// pub const lwip_select: u32 = select;

// pub const lwip_poll: u32 = poll;

// pub const lwip_ioctl: u32 = ioctlsocket;
// pub const lwip_inet_ntop: u32 = inet_ntop;
// pub const lwip_inet_pton: u32 = inet_pton;

// pub const lwip_read: u32 = read;
// pub const lwip_readv: u32 = readv;
// pub const lwip_write: u32 = write;
// pub const lwip_writev: u32 = writev;
// //#undef lwip_close
// pub const lwip_close: u32 = close;
// #define closesocket(s)    close(s)
// fcntl: i32(s: i32, cmd: i32, ...);
// //#undef lwip_ioctl
// pub const lwip_ioctl: u32 = ioctl;
// pub const ioctlsocket: u32 = ioctl;

// lwip_accept: i32(s: i32, addr: &mut sockaddr, addrlen: &mut usize);
// lwip_bind: i32(s: i32,  name: &mut sockaddr, namelen: socklen_t);
// lwip_shutdown: i32(s: i32, how: i32);
// lwip_getpeername: i32 (s: i32, name: &mut sockaddr, namelen: &mut usize);
// lwip_getsockname: i32 (s: i32, name: &mut sockaddr, namelen: &mut usize);
// lwip_getsockopt: i32 (s: i32, level: i32, optname: i32, optval: &mut Vec<u8>, optlen: &mut usize);
// lwip_setsockopt: i32 (s: i32, level: i32, optname: i32, optval: &Vec<u8>, optlen: socklen_t);
//  lwip_close: i32(s: i32);
// lwip_connect: i32(s: i32,  name: &mut sockaddr, namelen: socklen_t);
// lwip_listen: i32(s: i32, backlog: i32);
// isize lwip_recv(s: i32, mem: &mut Vec<u8>, len: usize, flags: i32);
// isize lwip_read(s: i32, mem: &mut Vec<u8>, len: usize);
// isize lwip_readv(s: i32,  iov: &mut iovec, iovcnt: i32);
// isize lwip_recvfrom(s: i32, mem: &mut Vec<u8>, len: usize, flags: i32,
//       from: &mut sockaddr, fromlen: &mut usize);
// isize lwip_recvmsg(s: i32, message: &mut msghdr, flags: i32);
// isize lwip_send(s: i32, dataptr: &Vec<u8>, size: usize, flags: i32);
// isize lwip_sendmsg(s: i32,  message: &mut msghdr, flags: i32);
// isize lwip_sendto(s: i32, dataptr: &Vec<u8>, size: usize, flags: i32,
//  to: &mut sockaddr, tolen: socklen_t);
// lwip_socket: i32(domain: i32, type: i32, protocol: i32);
// isize lwip_write(s: i32, dataptr: &Vec<u8>, size: usize);
// isize lwip_writev(s: i32,  iov: &mut iovec, iovcnt: i32);

// lwip_select: i32(maxfdp1: i32, readset: &mut fd_set, writeset: &mut fd_set, exceptset: &mut fd_set,
//                 timeout: &mut timeval);

// lwip_poll: i32(fds: &mut pollfd, nfds: nfds_t, timeout: i32);

// lwip_ioctl: i32(s: i32, long cmd, arg: &mut Vec<u8>p);
// lwip_fcntl: i32(s: i32, cmd: i32, val: i32);
// lwip_inet_ntop: &String(af: i32, src: &Vec<u8>, dst: &mut String, size: socklen_t);
// lwip_inet_pton: i32(af: i32, src: &String, dst: &mut Vec<u8>);

// //  @ingroup socket
// #define accept(s,addr,addrlen)                    lwip_accept(s,addr,addrlen)
// //  @ingroup socket
// #define bind(s,name,namelen)                      lwip_bind(s,name,namelen)
// //  @ingroup socket
// #define shutdown(s,how)                           lwip_shutdown(s,how)
// //  @ingroup socket
// #define getpeername(s,name,namelen)               lwip_getpeername(s,name,namelen)
// //  @ingroup socket
// #define getsockname(s,name,namelen)               lwip_getsockname(s,name,namelen)
// //  @ingroup socket
// #define setsockopt(s,level,optname,opval,optlen)  lwip_setsockopt(s,level,optname,opval,optlen)
// //  @ingroup socket
// #define getsockopt(s,level,optname,opval,optlen)  lwip_getsockopt(s,level,optname,opval,optlen)
// //  @ingroup socket
// #define closesocket(s)                            lwip_close(s)
// //  @ingroup socket
// #define connect(s,name,namelen)                   lwip_connect(s,name,namelen)
// //  @ingroup socket
// #define listen(s,backlog)                         lwip_listen(s,backlog)
// //  @ingroup socket
// #define recv(s,mem,len,flags)                     lwip_recv(s,mem,len,flags)
// //  @ingroup socket
// #define recvmsg(s,message,flags)                  lwip_recvmsg(s,message,flags)
// //  @ingroup socket
// #define recvfrom(s,mem,len,flags,from,fromlen)    lwip_recvfrom(s,mem,len,flags,from,fromlen)
// //  @ingroup socket
// #define send(s,dataptr,size,flags)                lwip_send(s,dataptr,size,flags)
// //  @ingroup socket
// #define sendmsg(s,message,flags)                  lwip_sendmsg(s,message,flags)
// //  @ingroup socket
// #define sendto(s,dataptr,size,flags,to,tolen)     lwip_sendto(s,dataptr,size,flags,to,tolen)
// //  @ingroup socket
// #define socket(domain,type,protocol)              lwip_socket(domain,type,protocol)

// //  @ingroup socket
// #define select(maxfdp1,readset,writeset,exceptset,timeout)     lwip_select(maxfdp1,readset,writeset,exceptset,timeout)

// //  @ingroup socket
// #define poll(fds,nfds,timeout)                    lwip_poll(fds,nfds,timeout)

// //  @ingroup socket
// #define ioctlsocket(s,cmd,argp)                   lwip_ioctl(s,cmd,argp)
// //  @ingroup socket
// #define inet_ntop(af,src,dst,size)                lwip_inet_ntop(af,src,dst,size)
// //  @ingroup socket
// #define inet_pton(af,src,dst)                     lwip_inet_pton(af,src,dst)

// //  @ingroup socket
// #define read(s,mem,len)                           lwip_read(s,mem,len)
// //  @ingroup socket
// #define readv(s,iov,iovcnt)                       lwip_readv(s,iov,iovcnt)
// //  @ingroup socket
// #define write(s,dataptr,len)                      lwip_write(s,dataptr,len)
// //  @ingroup socket
// #define writev(s,iov,iovcnt)                      lwip_writev(s,iov,iovcnt)
// //  @ingroup socket
// #define close(s)                                  lwip_close(s)
// //  @ingroup socket
// #define fcntl(s,cmd,val)                          lwip_fcntl(s,cmd,val)
// //  @ingroup socket
// #define ioctl(s,cmd,argp)                         lwip_ioctl(s,cmd,argp)
