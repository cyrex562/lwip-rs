/*
 * Copyright (c) 2001-2003 Swedish Institute of Computer Science.
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

//  LWIP_OPTTEST_FILE 

// pub const LWIP_IPV4: u32 = 1;
// pub const LWIP_IPV6: u32 = 1;

pub const NO_SYS: u32 = 0;
// #define LWIP_SOCKET                (NO_SYS==0)
// #define LWIP_NETCONN               (NO_SYS==0)
// #define LWIP_NETIF_API             (NO_SYS==0)

// pub const LWIP_IGMP: u32 = LWIP_IPV4;
// pub const LWIP_ICMP: u32 = LWIP_IPV4;

// pub const LWIP_SNMP: u32 = LWIP_UDP;
// pub const MIB2_STATS: u32 = LWIP_SNMP;

// #define LWIP_SNMP_V3               (LWIP_SNMP)

// pub const LWIP_DNS: u32 = LWIP_UDP;
// pub const LWIP_MDNS_RESPONDER: u32 = LWIP_UDP;

// #define lwip_num_netif_client_data (LWIP_MDNS_RESPONDER)

// pub const LWIP_HAVE_LOOPIF: u32 = 1;
pub const LWIP_HAVE_LOOPIF: u32 = 1;
// pub const LWIP_NETIF_LOOPBACK: u32 = 1;
// pub const LWIP_LOOPBACK_MAX_PBUFS: u32 = 10;

// pub const TCP_LISTEN_BACKLOG: u32 = 1;

// pub const LWIP_COMPAT_SOCKETS: u32 = 1;
// pub const LWIP_SO_RCVTIMEO: u32 = 1;
// pub const LWIP_SO_RCVBUF: u32 = 1;

// pub const LWIP_TCPIP_CORE_LOCKING: u32 = 1;

// pub const LWIP_NETIF_LINK_CALLBACK: u32 = 1;
// pub const LWIP_NETIF_STATUS_CALLBACK: u32 = 1;
// pub const LWIP_NETIF_EXT_STATUS_CALLBACK: u32 = 1;

pub const LWIP_DBG_MIN_LEVEL: u32 = 0;
// pub const PPP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const MEM_DEBUG: u32 = LWIP_DBG_OFF;
// pub const MEMP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const PBUF_DEBUG: u32 = LWIP_DBG_OFF;
// pub const API_LIB_DEBUG: u32 = LWIP_DBG_OFF;
// pub const API_MSG_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCPIP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const NETIF_DEBUG: u32 = LWIP_DBG_OFF;
// pub const SOCKETS_DEBUG: u32 = LWIP_DBG_OFF;
// pub const DNS_DEBUG: u32 = LWIP_DBG_OFF;
// pub const AUTOIP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const DHCP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const IP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const IP_REASS_DEBUG: u32 = LWIP_DBG_OFF;
// pub const ICMP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const IGMP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const UDP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_INPUT_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_OUTPUT_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_RTO_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_CWND_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_WND_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_FR_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_QLEN_DEBUG: u32 = LWIP_DBG_OFF;
// pub const TCP_RST_DEBUG: u32 = LWIP_DBG_OFF;

// #define LWIP_DBG_TYPES_ON         (LWIP_DBG_ON|LWIP_DBG_TRACE|LWIP_DBG_STATE|LWIP_DBG_FRESH|LWIP_DBG_HALT)

//  ---------- Memory options ---------- 
/* MEM_ALIGNMENT: should be set to the alignment of the CPU for which
lwIP is compiled. 4 byte alignment -> define MEM_ALIGNMENT to 4, 2
byte alignment -> define MEM_ALIGNMENT to 2. */
/* MSVC port: i32el processors don't need 4-byte alignment,
but are faster that way! */
// pub const MEM_ALIGNMENT: u32 = 4;

/* MEM_SIZE: the size of the heap memory. If the application will send
a lot of data that needs to be copied, this should be set high. */
// pub const MEM_SIZE: u32 = 10240;

/* MEMP_NUM_PBUF: the number of memp struct pbufs. If the application
sends a lot of data out of ROM (or other static memory), this
should be set high. */
// pub const MEMP_NUM_PBUF: u32 = 16;
/* MEMP_NUM_RAW_PCB: the number of UDP protocol control blocks. One
per active RAW "connection". */
// pub const MEMP_NUM_RAW_PCB: u32 = 3;
/* MEMP_NUM_UDP_PCB: the number of UDP protocol control blocks. One
per active UDP "connection". */
// pub const MEMP_NUM_UDP_PCB: u32 = 4;
/* MEMP_NUM_TCP_PCB: the number of simulatenously active TCP
connections. */
// pub const MEMP_NUM_TCP_PCB: u32 = 5;
/* MEMP_NUM_TCP_PCB_LISTEN: the number of listening TCP
connections. */
// pub const MEMP_NUM_TCP_PCB_LISTEN: u32 = 8;
/* MEMP_NUM_TCP_SEG: the number of simultaneously queued TCP
segments. */
// pub const MEMP_NUM_TCP_SEG: u32 = 16;
/* MEMP_NUM_SYS_TIMEOUT: the number of simulateously active
timeouts. */
// pub const MEMP_NUM_SYS_TIMEOUT: u32 = 17;

/* The following four are used only with the sequential API and can be
set to 0 if the application only will use the raw API. */
//  MEMP_NUM_NETBUF: the number of struct netbufs. 
// pub const MEMP_NUM_NETBUF: u32 = 2;
//  MEMP_NUM_NETCONN: the number of struct netconns. 
// pub const MEMP_NUM_NETCONN: u32 = 10;
/* MEMP_NUM_TCPIP_MSG_*: the number of struct tcpip_msg, which is used
for sequential API communication and incoming packets. Used in
src/api/tcpip.c. */
// pub const MEMP_NUM_TCPIP_MSG_API: u32 = 16;
// pub const MEMP_NUM_TCPIP_MSG_INPKT: u32 = 16;

//  ---------- Pbuf options ---------- 
//  PBUF_POOL_SIZE: the number of buffers in the pbuf pool. 
// pub const PBUF_POOL_SIZE: u32 = 120;

//  PBUF_POOL_BUFSIZE: the size of each pbuf in the pbuf pool. 
// pub const PBUF_POOL_BUFSIZE: u32 = 256;

/* SYS_LIGHTWEIGHT_PROT
 * define SYS_LIGHTWEIGHT_PROT in lwipopts.h if you want inter-task protection
 * for certain critical regions during buffer allocation, deallocation and memory
 * allocation and deallocation.
 */
// #define SYS_LIGHTWEIGHT_PROT    (NO_SYS==0)

//  ---------- TCP options ---------- 
// pub const LWIP_TCP: u32 = 1;
// pub const TCP_TTL: u32 = 255;

// #define lwip_altcp              (LWIP_TCP)

// #define lwip_altcp_tls          (LWIP_TCP)
// #define LWIP_ALTCP_TLS_MBEDTLS  (LWIP_TCP)

/* Controls if TCP should queue segments that arrive out of
order. Define to 0 if your device is low on memory. */
// pub const TCP_QUEUE_OOSEQ: u32 = 1;

//  TCP Maximum segment size. 
// pub const TCP_MSS: u32 = 1024;

//  TCP sender buffer space (bytes). 
// pub const TCP_SND_BUF: u32 = 2048;

/* TCP sender buffer space (pbufs). This must be at least = 2 *
TCP_SND_BUF/TCP_MSS for things to work. */
// #define TCP_SND_QUEUELEN       (4 * TCP_SND_BUF/TCP_MSS)

/* TCP writable space (bytes). This must be less than or equal
to TCP_SND_BUF. It is the amount of space which must be
available in the tcp snd_buf for select to return writable */
// #define TCP_SNDLOWAT           (TCP_SND_BUF/2)

//  TCP receive window. 
// #define TCP_WND                 (20 * 1024)

//  Maximum number of retransmissions of data segments. 
// pub const TCP_MAXRTX: u32 = 12;

//  Maximum number of retransmissions of SYN segments. 
// pub const TCP_SYNMAXRTX: u32 = 4;

//  ---------- ARP options ---------- 
// pub const LWIP_ARP: u32 = 1;
// pub const ARP_TABLE_SIZE: u32 = 10;
// pub const arp_queueing: u32 = 1;

//  ---------- IP options ---------- 
/* Define ip_forward to 1 if you wish to have the ability to forward
IP packets across network interfaces. If you are going to run lwIP
on a device with only one network interface, define this to 0. */
// pub const ip_forward: u32 = 1;

/* IP reassembly and segmentation.These are orthogonal even
 * if they both deal with IP fragments */
// pub const ip_reassembly: u32 = 1;
// #define ip_reass_max_pbufs      (10 * ((1500 + PBUF_POOL_BUFSIZE - 1) / PBUF_POOL_BUFSIZE))
// pub const MEMP_NUM_REASSDATA: u32 = ip_reass_max_pbufs;
// pub const ip_frag: u32 = 1;
// pub const IPV6_FRAG_COPYHEADER: u32 = 1;

//  ---------- ICMP options ---------- 
// pub const icmp_ttl: u32 = 255;

//  ---------- DHCP options ---------- 
/* Define LWIP_DHCP to 1 if you want DHCP configuration of
interfaces. */
// pub const LWIP_DHCP: u32 = LWIP_UDP;

/* 1 if you want to do an ARP check on the offered address
(recommended). */
// #define dhcp_does_arp_check    (LWIP_DHCP)

//  ---------- AUTOIP options ------- 
// #define LWIP_AUTOIP            (LWIP_DHCP)
// #define lwip_dhcp_autoip_coop  (LWIP_DHCP && LWIP_AUTOIP)

//  ---------- UDP options ---------- 
// pub const LWIP_UDP: u32 = 1;
// pub const LWIP_UDPLITE: u32 = LWIP_UDP;
// pub const UDP_TTL: u32 = 255;

//  ---------- RAW options ---------- 
// pub const LWIP_RAW: u32 = 1;

//  ---------- Statistics options ---------- 

// pub const LWIP_STATS: u32 = 1;
// pub const LWIP_STATS_DISPLAY: u32 = 1;

// pub const LINK_STATS: u32 = 1;
// pub const IP_STATS: u32 = 1;
// pub const ICMP_STATS: u32 = 1;
// pub const IGMP_STATS: u32 = 1;
// pub const IPFRAG_STATS: u32 = 1;
// pub const UDP_STATS: u32 = 1;
// pub const TCP_STATS: u32 = 1;
// pub const MEM_STATS: u32 = 1;
// pub const MEMP_STATS: u32 = 1;
// pub const PBUF_STATS: u32 = 1;
// pub const SYS_STATS: u32 = 1;

//  ---------- NETBIOS options ---------- 
// pub const LWIP_NETBIOS_RESPOND_NAME_QUERY: u32 = 1;

//  ---------- PPP options ---------- 

// pub const PPP_SUPPORT: u32 = 1;      //  Set > 0 for PPP 

// pub const NUM_PPP: u32 = 1;      //  Max PPP sessions. 

/* Select modules to enable.  Ideally these would be set in the makefile but
 * we're limited by the command line length so you need to modify the settings
 * in this file.
 */
// pub const PPPOE_SUPPORT: u32 = 1;
// pub const PPPOS_SUPPORT: u32 = 1;

// pub const PAP_SUPPORT: u32 = 1;      //  Set > 0 for PAP. 
// pub const CHAP_SUPPORT: u32 = 1;      //  Set > 0 for CHAP. 
// pub const MSCHAP_SUPPORT: u32 = 0;      //  Set > 0 for MSCHAP 
// pub const CBCP_SUPPORT: u32 = 0;      //  Set > 0 for CBCP (NOT FUNCTIONAL!) 
// pub const CCP_SUPPORT: u32 = 0;      //  Set > 0 for CCP 
// pub const VJ_SUPPORT: u32 = 1;      //  Set > 0 for VJ header compression. 
// pub const MD5_SUPPORT: u32 = 1;      //  Set > 0 for MD5 (see also CHAP) 

//  The following defines must be done even in OPTTEST mode: 

fn sys_check_core_locking();
// #define LWIP_ASSERT_CORE_LOCKED()  sys_check_core_locking()
fn sys_mark_tcpip_thread();
// #define LWIP_MARK_TCPIP_THREAD()   sys_mark_tcpip_thread()

fn sys_lock_tcpip_core();
// #define LOCK_TCPIP_CORE()          sys_lock_tcpip_core()
fn sys_unlock_tcpip_core();
// #define UNLOCK_TCPIP_CORE()        sys_unlock_tcpip_core()
