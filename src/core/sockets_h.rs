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



// #define LWIP_HDR_SOCKETS_H

















/* If your port already typedef's sa_family_t, define SA_FAMILY_T_DEFINED
   to prevent this code from redefining it. */

typedef sa_family_t: u8;

/* If your port already typedef's in_port_t, define IN_PORT_T_DEFINED
   to prevent this code from redefining it. */

typedef in_port_t: u16;



/* members are in network byte order */
struct sockaddr_in {
  u8            sin_len;
  sa_family_t     sin_family;
  in_port_t       sin_port;
  struct in_addr  sin_addr;
#define SIN_ZERO_LEN 8
  char            sin_zero[SIN_ZERO_LEN];
};



struct sockaddr_in6 {
  u8            sin6_len;      /* length of this structure    */
  sa_family_t     sin6_family;   /* AF_INET6                    */
  in_port_t       sin6_port;     /* Transport layer port #      */
  u32           sin6_flowinfo; /* IPv6 flow information       */
  struct in6_addr sin6_addr;     /* IPv6 address                */
  u32           sin6_scope_id; /* Set of interfaces for scope */
};


struct sockaddr {
  u8        sa_len;
  sa_family_t sa_family;
  char        sa_data[14];
};

struct sockaddr_storage {
  u8        s2_len;
  sa_family_t ss_family;
  char        s2_data1[2];
  u32       s2_data2[3];

  u32       s2_data3[3];

};

/* If your port already typedef's socklen_t, define SOCKLEN_T_DEFINED
   to prevent this code from redefining it. */

typedef socklen_t: u32;



pub const IOV_MAX: u32 = 0xFFFF;
#elif IOV_MAX > 0xFFFF
#error "IOV_MAX larger than supported by LwIP"



struct iovec {
  void  *iov_base;
  iov_len: usize;
};


struct msghdr {
  void         *msg_name;
  socklen_t     msg_namelen;
  msg_iov: &mut iovec;
  int           msg_iovlen;
  void         *msg_control;
  socklen_t     msg_controllen;
  int           msg_flags;
};

/* struct msghdr.msg_flags bit field values */
pub const MSG_TRUNC: u32 = 0x04;pub const MSG_TRUNC: u32 = 0x04;
#define MSG_CTRUNC  0x08

/* RFC 3542, Section 20: Ancillary Data */
struct cmsghdr {
  socklen_t  cmsg_len;   /* number of bytes, including header */
  int        cmsg_level; /* originating protocol */
  int        cmsg_type;  /* protocol-specific type */
};
/* Data section follows header and possible padding, typically referred to as
      unsigned char cmsg_data[]; */

/* cmsg header/data alignment. NOTE: we align to native word size (double word
size on 16-bit arch) so structures are not placed at an unaligned address.
16-bit arch needs double word to ensure 32-bit alignment because socklen_t
could be 32 bits. If we ever have cmsg data with a 64-bit variable, alignment
will need to increase long long */
#define ALIGN_H(size) (((size) + sizeof(long) - 1U) & ~(sizeof(long)-1U))
#define ALIGN_D(size) ALIGN_H(size)

#define CMSG_FIRSTHDR(mhdr) \
          ((mhdr)->msg_controllen >= sizeof(struct cmsghdr) ? \
           (struct cmsghdr *)(mhdr)->msg_control : \
           (struct cmsghdr *)NULL)

#define CMSG_NXTHDR(mhdr, cmsg) \
        (((cmsg) == NULL) ? CMSG_FIRSTHDR(mhdr) : \
         (((cmsg) + ALIGN_H((cmsg)->cmsg_len) \
                            + ALIGN_D(sizeof(struct cmsghdr)) > \
           ((mhdr)->msg_control) + (mhdr)->msg_controllen) ? \
          (struct cmsghdr *)NULL : \
          (struct cmsghdr *)((void*)((cmsg) + \
                                      ALIGN_H((cmsg)->cmsg_len)))))

#define CMSG_DATA(cmsg) ((void*)((cmsg) + \
                         ALIGN_D(sizeof(struct cmsghdr))))

#define CMSG_SPACE(length) (ALIGN_D(sizeof(struct cmsghdr)) + \
                            ALIGN_H(length))

#define CMSG_LEN(length) (ALIGN_D(sizeof(struct cmsghdr)) + \
                           length)

/* Set socket options argument */
#define IFNAMSIZ NETIF_NAMESIZE
struct ifreq {
  char ifr_name[IFNAMSIZ]; /* Interface name */
};

/* Socket protocol types (TCP/UDP/RAW) */
#define SOCK_STREAM     1
#define SOCK_DGRAM      2
#define SOCK_RAW        3

/*
 * Option flags per-socket. These must match the SOF_ flags in ip.h (checked in init.c)
 */
pub const SO_REUSEADDR: u32 = 0x0004; /* Allow local address reuse */pub const SO_REUSEADDR: u32 = 0x0004;pub const SO_REUSEADDR: u32 = 0x0004;
#define SO_KEEPALIVE   0x0008 /* keep connections alive */
#define SO_BROADCAST   0x0020 /* permit to send and to receive broadcast messages (see IP_SOF_BROADCAST option) */


/*
 * Additional options, not kept in so_options.
 */
pub const SO_DEBUG: u32 = 0x0001; /* Unimplemented: turn on debugging info recording */pub const SO_DEBUG: u32 = 0x0001;pub const SO_DEBUG: u32 = 0x0001;pub const SO_DEBUG: u32 = 0x0001;pub const SO_DEBUG: u32 = 0x0001;
#define SO_ACCEPTCONN   0x0002 /* socket has had listen() */
#define SO_DONTROUTE    0x0010 /* Unimplemented: just use interface addresses */
#define SO_USELOOPBACK  0x0040 /* Unimplemented: bypass hardware when possible */
#define SO_LINGER       0x0080 /* linger on close if data present */
#define SO_DONTLINGER   ((int)(~SO_LINGER))
pub const SO_OOBINLINE: u32 = 0x0100; /* Unimplemented: leave received OOB data in line */pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;pub const SO_OOBINLINE: u32 = 0x0100;
#define SO_REUSEPORT    0x0200 /* Unimplemented: allow local address & port reuse */
#define SO_SNDBUF       0x1001 /* Unimplemented: send buffer size */
#define SO_RCVBUF       0x1002 /* receive buffer size */
#define SO_SNDLOWAT     0x1003 /* Unimplemented: send low-water mark */
#define SO_RCVLOWAT     0x1004 /* Unimplemented: receive low-water mark */
#define SO_SNDTIMEO     0x1005 /* send timeout */
#define SO_RCVTIMEO     0x1006 /* receive timeout */
#define SO_ERROR        0x1007 /* get error status and clear */
#define SO_TYPE         0x1008 /* get socket type */
#define SO_CONTIMEO     0x1009 /* Unimplemented: connect timeout */
#define SO_NO_CHECK     0x100a /* don't create UDP checksum */
#define SO_BINDTODEVICE 0x100b /* bind to device */

/*
 * Structure used for manipulating linger option.
 */
struct linger {
  l_onoff: int;                /* option on/off */
  l_linger: int;               /* linger time in seconds */
};

/*
 * Level number for (get/set)sockopt() to apply to socket itself.
 */
pub const SOL_SOCKET: u32 = 0xfff;    /* options for socket level */


pub const AF_UNSPEC: u32 = 0;
#define AF_INET         2

#define AF_INET6        10
#else /* LWIP_IPV6 */
#define AF_INET6        AF_UNSPEC

#define PF_INET         AF_INET
#define PF_INET6        AF_INET6
#define PF_UNSPEC       AF_UNSPEC

pub const IPPROTO_IP: u32 = 0;
#define IPPROTO_ICMP    1
#define IPPROTO_TCP     6
#define IPPROTO_UDP     17

#define IPPROTO_IPV6    41
#define IPPROTO_ICMPV6  58

#define IPPROTO_UDPLITE 136
#define IPPROTO_RAW     255

/* Flags we can use with send and recv. */
pub const MSG_PEEK: u32 = 0x01;    /* Peeks at an incoming message */pub const MSG_PEEK: u32 = 0x01;pub const MSG_PEEK: u32 = 0x01;pub const MSG_PEEK: u32 = 0x01;pub const MSG_PEEK: u32 = 0x01;pub const MSG_PEEK: u32 = 0x01;
#define MSG_WAITALL    0x02    /* Unimplemented: Requests that the function block until the full amount of data requested can be returned */
#define MSG_OOB        0x04    /* Unimplemented: Requests out-of-band data. The significance and semantics of out-of-band data are protocol-specific */
#define MSG_DONTWAIT   0x08    /* Nonblocking i/o for this operation only */
#define MSG_MORE       0x10    /* Sender will send more */
#define MSG_NOSIGNAL   0x20    /* Uninmplemented: Requests not to send the SIGPIPE signal if an attempt to send is made on a stream-oriented socket that is no longer connected. */


/*
 * Options for level IPPROTO_IP
 */
#define IP_TOS             1
#define IP_TTL             2
#define IP_PKTINFO         8


/*
 * Options for level IPPROTO_TCP
 */
pub const TCP_NODELAY: u32 = 0x01;    /* don't delay send to coalesce packets */pub const TCP_NODELAY: u32 = 0x01;pub const TCP_NODELAY: u32 = 0x01;pub const TCP_NODELAY: u32 = 0x01;pub const TCP_NODELAY: u32 = 0x01;
#define TCP_KEEPALIVE  0x02    /* send KEEPALIVE probes when idle for pcb.keep_idle milliseconds */
#define TCP_KEEPIDLE   0x03    /* set pcb.keep_idle  - Same as TCP_KEEPALIVE, but use seconds for get/setsockopt */
#define TCP_KEEPINTVL  0x04    /* set pcb.keep_intvl - Use seconds for get/setsockopt */
#define TCP_KEEPCNT    0x05    /* set pcb.keep_cnt   - Use number of probes sent for get/setsockopt */



/*
 * Options for level IPPROTO_IPV6
 */
#define IPV6_CHECKSUM       7  /* RFC3542: calculate and insert the ICMPv6 checksum for raw sockets. */
#define IPV6_V6ONLY         27 /* RFC3493: boolean control to restrict AF_INET6 sockets to IPv6 communications only. */



/*
 * Options for level IPPROTO_UDPLITE
 */
pub const UDPLITE_SEND_CSCOV: u32 = 0x01; /* sender checksum coverage */pub const UDPLITE_SEND_CSCOV: u32 = 0x01;
#define UDPLITE_RECV_CSCOV 0x02 /* minimal receiver checksum coverage */




/*
 * Options and types for UDP multicast traffic handling
 */
#define IP_MULTICAST_TTL   5
#define IP_MULTICAST_IF    6
#define IP_MULTICAST_LOOP  7



/*
 * Options and types related to multicast membership
 */
#define IP_ADD_MEMBERSHIP  3
#define IP_DROP_MEMBERSHIP 4

typedef struct ip_mreq {
    struct in_addr imr_multiaddr; /* IP multicast address of group */
    struct in_addr imr_interface; /* local IP address of interface */
} ip_mreq;



struct in_pktinfo {
  unsigned int   ipi_ifindex;  /* Interface index */
  struct in_addr ipi_addr;     /* Destination (from header) address */
};



/*
 * Options and types related to IPv6 multicast membership
 */
#define IPV6_JOIN_GROUP      12
#define IPV6_ADD_MEMBERSHIP  IPV6_JOIN_GROUP
#define IPV6_LEAVE_GROUP     13
#define IPV6_DROP_MEMBERSHIP IPV6_LEAVE_GROUP

typedef struct ipv6_mreq {
  struct in6_addr ipv6mr_multiaddr; /*  IPv6 multicast addr */
  unsigned int    ipv6mr_interface; /*  interface index, or 0 */
} ipv6_mreq;


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
#define IPTOS_TOS(tos)          ((tos) & IPTOS_TOS_MASK)
pub const IPTOS_LOWDELAY: u32 = 0x10;pub const IPTOS_LOWDELAY: u32 = 0x10;pub const IPTOS_LOWDELAY: u32 = 0x10;pub const IPTOS_LOWDELAY: u32 = 0x10;
#define IPTOS_THROUGHPUT        0x08
#define IPTOS_RELIABILITY       0x04
#define IPTOS_LOWCOST           0x02
#define IPTOS_MINCOST           IPTOS_LOWCOST

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
#define IPTOS_PREC(tos)                ((tos) & IPTOS_PREC_MASK)
pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;pub const IPTOS_PREC_NETCONTROL: u32 = 0xe0;
#define IPTOS_PREC_INTERNETCONTROL      0xc0
#define IPTOS_PREC_CRITIC_ECP           0xa0
#define IPTOS_PREC_FLASHOVERRIDE        0x80
#define IPTOS_PREC_FLASH                0x60
#define IPTOS_PREC_IMMEDIATE            0x40
#define IPTOS_PREC_PRIORITY             0x20
#define IPTOS_PREC_ROUTINE              0x00


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

pub const IOCPARM_MASK: u32 = 0x7f;U           /* parameters must be < 128 bytes */pub const IOCPARM_MASK: u32 = 0x7f;pub const IOCPARM_MASK: u32 = 0x7f;pub const IOCPARM_MASK: u32 = 0x7f;
#define IOC_VOID        0x20000000    /* no parameters */
#define IOC_OUT         0x40000000    /* copy out parameters */
#define IOC_IN          0x80000000    /* copy in parameters */
#define IOC_INOUT       (IOC_IN|IOC_OUT)
                                        /* 0x20000000 distinguishes new &
                                           old ioctl's */
#define _IO(x,y)        ((long)(IOC_VOID|((x)<<8)|(y)))

#define _IOR(x,y,t)     ((long)(IOC_OUT|((sizeof(t)&IOCPARM_MASK)<<16)|((x)<<8)|(y)))

#define _IOW(x,y,t)     ((long)(IOC_IN|((sizeof(t)&IOCPARM_MASK)<<16)|((x)<<8)|(y)))



#define FIONREAD    _IOR('f', 127, unsigned long) /* get # bytes to read */


#define FIONBIO     _IOW('f', 126, unsigned long) /* set/clear non-blocking i/o */


/* Socket I/O Controls: unimplemented */

#define SIOCSHIWAT  _IOW('s',  0, unsigned long)  /* set high watermark */
#define SIOCGHIWAT  _IOR('s',  1, unsigned long)  /* get high watermark */
#define SIOCSLOWAT  _IOW('s',  2, unsigned long)  /* set low watermark */
#define SIOCGLOWAT  _IOR('s',  3, unsigned long)  /* get low watermark */
#define SIOCATMARK  _IOR('s',  7, unsigned long)  /* at oob mark? */


/* commands for fnctl */

#define F_GETFL 3


#define F_SETFL 4


/* File status flags and file access modes for fnctl,
   these are bits in an int. */

#define O_NONBLOCK  1 /* nonblocking I/O */


#define O_NDELAY    O_NONBLOCK /* same as O_NONBLOCK, for compatibility */


#define O_RDONLY    2


#define O_WRONLY    4


#define O_RDWR      (O_RDONLY|O_WRONLY)



  #define SHUT_RD   0
  #define SHUT_WR   1
  #define SHUT_RDWR 2


/* FD_SET used for lwip_select */

#undef  FD_SETSIZE
/* Make FD_SETSIZE match NUM_SOCKETS in socket.c */
#define FD_SETSIZE    MEMP_NUM_NETCONN
// #define LWIP_SELECT_MAXNFDS (FD_SETSIZE + LWIP_SOCKET_OFFSET)
#define FDSETSAFESET(n, code) do { \
  if (((n) - LWIP_SOCKET_OFFSET < MEMP_NUM_NETCONN) && (((int)(n) - LWIP_SOCKET_OFFSET) >= 0)) { \
  code; }} while(0)
#define FDSETSAFEGET(n, code) (((n) - LWIP_SOCKET_OFFSET < MEMP_NUM_NETCONN) && (((int)(n) - LWIP_SOCKET_OFFSET) >= 0) ?\
  (code) : 0)
#define FD_SET(n, p)  FDSETSAFESET(n, (p)->fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] = (u8)((p)->fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] |  (1 << (((n)-LWIP_SOCKET_OFFSET) & 7))))
#define FD_CLR(n, p)  FDSETSAFESET(n, (p)->fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] = (u8)((p)->fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] & ~(1 << (((n)-LWIP_SOCKET_OFFSET) & 7))))
#define FD_ISSET(n,p) FDSETSAFEGET(n, (p)->fd_bits[((n)-LWIP_SOCKET_OFFSET)/8] &   (1 << (((n)-LWIP_SOCKET_OFFSET) & 7)))
#define FD_ZERO(p)    memset((void*)(p), 0, sizeof(*(p)))

typedef struct fd_set
{
  unsigned char fd_bits [(FD_SETSIZE+7)/8];
} fd_set;

#elif FD_SETSIZE < (LWIP_SOCKET_OFFSET + MEMP_NUM_NETCONN)
#error "external FD_SETSIZE too small for number of sockets"
#else
// #define LWIP_SELECT_MAXNFDS FD_SETSIZE


/* poll-related defines and types */
/* @todo: find a better way to guard the definition of these defines and types if already defined */

pub const POLLIN: u32 = 0x1;pub const POLLIN: u32 = 0x1;pub const POLLIN: u32 = 0x1;pub const POLLIN: u32 = 0x1;
#define POLLOUT    0x2
#define POLLERR    0x4
#define POLLNVAL   0x8
/* Below values are unimplemented */
pub const POLLRDNORM: u32 = 0x10;pub const POLLRDNORM: u32 = 0x10;pub const POLLRDNORM: u32 = 0x10;pub const POLLRDNORM: u32 = 0x10;pub const POLLRDNORM: u32 = 0x10;pub const POLLRDNORM: u32 = 0x10;
#define POLLRDBAND 0x20
#define POLLPRI    0x40
#define POLLWRNORM 0x80
#define POLLWRBAND 0x100
#define POLLHUP    0x200
typedef unsigned nfds_t: int;
struct pollfd
{
  fd: int;
  short events;
  short revents;
};


/* LWIP_TIMEVAL_PRIVATE: if you want to use the struct timeval provided
 * by your system, set this to 0 and include <sys/time.h> in cc.h */

// #define LWIP_TIMEVAL_PRIVATE 1



struct timeval {
  long    tv_sec;         /* seconds */
  long    tv_usec;        /* and microseconds */
};


#define lwip_socket_init() /* Compatibility define, no init needed. */
pub fn  lwip_socket_thread_init(); /* LWIP_NETCONN_SEM_PER_THREAD==1: initialize thread-local semaphore */
pub fn  lwip_socket_thread_cleanup(); /* LWIP_NETCONN_SEM_PER_THREAD==1: destroy thread-local semaphore */


/* This helps code parsers/code completion by not having the COMPAT functions as defines */
#define lwip_accept       accept
#define lwip_bind         bind
#define lwip_shutdown     shutdown
#define lwip_getpeername  getpeername
#define lwip_getsockname  getsockname
#define lwip_setsockopt   setsockopt
#define lwip_getsockopt   getsockopt
#define lwip_close        closesocket
#define lwip_connect      connect
#define lwip_listen       listen
#define lwip_recv         recv
#define lwip_recvmsg      recvmsg
#define lwip_recvfrom     recvfrom
#define lwip_send         send
#define lwip_sendmsg      sendmsg
#define lwip_sendto       sendto
#define lwip_socket       socket

#define lwip_select       select


#define lwip_poll         poll

#define lwip_ioctl        ioctlsocket
#define lwip_inet_ntop    inet_ntop
#define lwip_inet_pton    inet_pton


#define lwip_read         read
#define lwip_readv        readv
#define lwip_write        write
#define lwip_writev       writev
#undef lwip_close
#define lwip_close        close
#define closesocket(s)    close(s)
fcntl: int(s: int, cmd: int, ...);
#undef lwip_ioctl
#define lwip_ioctl        ioctl
#define ioctlsocket       ioctl



lwip_accept: int(s: int, addr: &mut sockaddr, socklen_t *addrlen);
lwip_bind: int(s: int, const name: &mut sockaddr, socklen_t namelen);
lwip_shutdown: int(s: int, how: int);
lwip_getpeername: int (s: int, name: &mut sockaddr, socklen_t *namelen);
lwip_getsockname: int (s: int, name: &mut sockaddr, socklen_t *namelen);
lwip_getsockopt: int (s: int, level: int, optname: int, void *optval, socklen_t *optlen);
lwip_setsockopt: int (s: int, level: int, optname: int, optval: &Vec<u8>, socklen_t optlen);
 lwip_close: int(s: int);
lwip_connect: int(s: int, const name: &mut sockaddr, socklen_t namelen);
lwip_listen: int(s: int, backlog: int);
isize lwip_recv(s: int, void *mem, usize len, flags: int);
isize lwip_read(s: int, void *mem, usize len);
isize lwip_readv(s: int, const iov: &mut iovec, iovcnt: int);
isize lwip_recvfrom(s: int, void *mem, usize len, flags: int,
      from: &mut sockaddr, socklen_t *fromlen);
isize lwip_recvmsg(s: int, message: &mut msghdr, flags: int);
isize lwip_send(s: int, dataptr: &Vec<u8>, usize size, flags: int);
isize lwip_sendmsg(s: int, const message: &mut msghdr, flags: int);
isize lwip_sendto(s: int, dataptr: &Vec<u8>, usize size, flags: int,
    const to: &mut sockaddr, socklen_t tolen);
lwip_socket: int(domain: int, type: int, protocol: int);
isize lwip_write(s: int, dataptr: &Vec<u8>, usize size);
isize lwip_writev(s: int, const iov: &mut iovec, iovcnt: int);

lwip_select: int(maxfdp1: int, fd_set *readset, fd_set *writeset, fd_set *exceptset,
                timeout: &mut timeval);


lwip_poll: int(fds: &mut pollfd, nfds_t nfds, timeout: int);

lwip_ioctl: int(s: int, long cmd, arg: &mut Vec<u8>p);
lwip_fcntl: int(s: int, cmd: int, val: int);
const char *lwip_inet_ntop(af: int, src: &Vec<u8>, char *dst, socklen_t size);
lwip_inet_pton: int(af: int, const char *src, void *dst);



/* @ingroup socket */
#define accept(s,addr,addrlen)                    lwip_accept(s,addr,addrlen)
/* @ingroup socket */
#define bind(s,name,namelen)                      lwip_bind(s,name,namelen)
/* @ingroup socket */
#define shutdown(s,how)                           lwip_shutdown(s,how)
/* @ingroup socket */
#define getpeername(s,name,namelen)               lwip_getpeername(s,name,namelen)
/* @ingroup socket */
#define getsockname(s,name,namelen)               lwip_getsockname(s,name,namelen)
/* @ingroup socket */
#define setsockopt(s,level,optname,opval,optlen)  lwip_setsockopt(s,level,optname,opval,optlen)
/* @ingroup socket */
#define getsockopt(s,level,optname,opval,optlen)  lwip_getsockopt(s,level,optname,opval,optlen)
/* @ingroup socket */
#define closesocket(s)                            lwip_close(s)
/* @ingroup socket */
#define connect(s,name,namelen)                   lwip_connect(s,name,namelen)
/* @ingroup socket */
#define listen(s,backlog)                         lwip_listen(s,backlog)
/* @ingroup socket */
#define recv(s,mem,len,flags)                     lwip_recv(s,mem,len,flags)
/* @ingroup socket */
#define recvmsg(s,message,flags)                  lwip_recvmsg(s,message,flags)
/* @ingroup socket */
#define recvfrom(s,mem,len,flags,from,fromlen)    lwip_recvfrom(s,mem,len,flags,from,fromlen)
/* @ingroup socket */
#define send(s,dataptr,size,flags)                lwip_send(s,dataptr,size,flags)
/* @ingroup socket */
#define sendmsg(s,message,flags)                  lwip_sendmsg(s,message,flags)
/* @ingroup socket */
#define sendto(s,dataptr,size,flags,to,tolen)     lwip_sendto(s,dataptr,size,flags,to,tolen)
/* @ingroup socket */
#define socket(domain,type,protocol)              lwip_socket(domain,type,protocol)

/* @ingroup socket */
#define select(maxfdp1,readset,writeset,exceptset,timeout)     lwip_select(maxfdp1,readset,writeset,exceptset,timeout)


/* @ingroup socket */
#define poll(fds,nfds,timeout)                    lwip_poll(fds,nfds,timeout)

/* @ingroup socket */
#define ioctlsocket(s,cmd,argp)                   lwip_ioctl(s,cmd,argp)
/* @ingroup socket */
#define inet_ntop(af,src,dst,size)                lwip_inet_ntop(af,src,dst,size)
/* @ingroup socket */
#define inet_pton(af,src,dst)                     lwip_inet_pton(af,src,dst)


/* @ingroup socket */
#define read(s,mem,len)                           lwip_read(s,mem,len)
/* @ingroup socket */
#define readv(s,iov,iovcnt)                       lwip_readv(s,iov,iovcnt)
/* @ingroup socket */
#define write(s,dataptr,len)                      lwip_write(s,dataptr,len)
/* @ingroup socket */
#define writev(s,iov,iovcnt)                      lwip_writev(s,iov,iovcnt)
/* @ingroup socket */
#define close(s)                                  lwip_close(s)
/* @ingroup socket */
#define fcntl(s,cmd,val)                          lwip_fcntl(s,cmd,val)
/* @ingroup socket */
#define ioctl(s,cmd,argp)                         lwip_ioctl(s,cmd,argp)






}





