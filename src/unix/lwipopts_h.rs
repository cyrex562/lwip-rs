/*
 * @file
 *
 * lwIP Options Configuration
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

// #define LWIP_LWIPOPTS_H

/*
 * Include user defined options first. Anything not defined in these files
 * will be set to standard values. Override anything you dont like!
 */



/*
   -----------------------------------------------
   ---------- Platform specific locking ----------
   -----------------------------------------------
*/

/*
 * SYS_LIGHTWEIGHT_PROT==1: if you want inter-task protection for certain
 * critical regions during buffer allocation, deallocation and memory
 * allocation and deallocation.
 */
pub const SYS_LIGHTWEIGHT_PROT: u32 = 0;

/*
 * NO_SYS==1: Provides VERY minimal functionality. Otherwise,
 * use lwIP facilities.
 */
pub const NO_SYS: u32 = 0;

/*
   ------------------------------------
   ---------- Memory options ----------
   ------------------------------------
*/

/*
 * MEM_ALIGNMENT: should be set to the alignment of the CPU
 *    4 byte alignment -> #define MEM_ALIGNMENT 4
 *    2 byte alignment -> #define MEM_ALIGNMENT 2
 */
pub const MEM_ALIGNMENT: u32 = 1; 

/*
 * MEM_SIZE: the size of the heap memory. If the application will send
 * a lot of data that needs to be copied, this should be set high.
 */
pub const MEM_SIZE: u32 = 1600; 

/*
   ------------------------------------------------
   ---------- Internal Memory Pool Sizes ----------
   ------------------------------------------------
*/
/*
 * MEMP_NUM_PBUF: the number of memp struct pbufs (used for PBUF_ROM and PBUF_REF).
 * If the application sends a lot of data out of ROM (or other static memory),
 * this should be set high.
 */
pub const MEMP_NUM_PBUF: u32 = 16; 

/*
 * MEMP_NUM_RAW_PCB: Number of raw connection PCBs
 * (requires the LWIP_RAW option)
 */
pub const MEMP_NUM_RAW_PCB: u32 = 4; 

/*
 * MEMP_NUM_UDP_PCB: the number of UDP protocol control blocks. One
 * per active UDP "connection".
 * (requires the LWIP_UDP option)
 */
pub const MEMP_NUM_UDP_PCB: u32 = 4; 

/*
 * MEMP_NUM_TCP_PCB: the number of simulatenously active TCP connections.
 * (requires the LWIP_TCP option)
 */
pub const MEMP_NUM_TCP_PCB: u32 = 4; 

/*
 * MEMP_NUM_TCP_PCB_LISTEN: the number of listening TCP connections.
 * (requires the LWIP_TCP option)
 */
pub const MEMP_NUM_TCP_PCB_LISTEN: u32 = 4; 

/*
 * MEMP_NUM_TCP_SEG: the number of simultaneously queued TCP segments.
 * (requires the LWIP_TCP option)
 */
pub const MEMP_NUM_TCP_SEG: u32 = 16; 

/*
 * MEMP_NUM_REASSDATA: the number of simultaneously IP packets queued for
 * reassembly (whole packets, not fragments!)
 */
pub const MEMP_NUM_REASSDATA: u32 = 1; 

/*
 * MEMP_NUM_ARP_QUEUE: the number of simulateously queued outgoing
 * packets (pbufs) that are waiting for an ARP request (to resolve
 * their destination address) to finish.
 * (requires the ARP_QUEUEING option)
 */
pub const MEMP_NUM_ARP_QUEUE: u32 = 2; 

/*
 * MEMP_NUM_SYS_TIMEOUT: the number of simulateously active timeouts.
 * (requires NO_SYS==0)
 */
pub const MEMP_NUM_SYS_TIMEOUT: u32 = 8; 

/*
 * MEMP_NUM_NETBUF: the number of struct netbufs.
 * (only needed if you use the sequential API, like api_lib.c)
 */
pub const MEMP_NUM_NETBUF: u32 = 2; 

/*
 * MEMP_NUM_NETCONN: the number of struct netconns.
 * (only needed if you use the sequential API, like api_lib.c)
 */
pub const MEMP_NUM_NETCONN: u32 = 32; 

/*
 * MEMP_NUM_TCPIP_MSG_API: the number of struct tcpip_msg, which are used
 * for callback/timeout API communication.
 * (only needed if you use tcpip.c)
 */
pub const MEMP_NUM_TCPIP_MSG_API: u32 = 8; 

/*
 * MEMP_NUM_TCPIP_MSG_INPKT: the number of struct tcpip_msg, which are used
 * for incoming packets.
 * (only needed if you use tcpip.c)
 */
pub const MEMP_NUM_TCPIP_MSG_INPKT: u32 = 8; 

/*
 * PBUF_POOL_SIZE: the number of buffers in the pbuf pool.
 */
pub const PBUF_POOL_SIZE: u32 = 8; 

/*
   ---------------------------------
   ---------- ARP options ----------
   ---------------------------------
*/
/*
 * LWIP_ARP==1: Enable ARP functionality.
 */
// #define LWIP_ARP                        1

/*
   --------------------------------
   ---------- IP options ----------
   --------------------------------
*/
/*
 * IP_FORWARD==1: Enables the ability to forward IP packets across network
 * interfaces. If you are going to run lwIP on a device with only one network
 * interface, define this to 0.
 */
pub const IP_FORWARD: u32 = 0;

/*
 * IP_OPTIONS: Defines the behavior for IP options.
 *      IP_OPTIONS_ALLOWED==0: All packets with IP options are dropped.
 *      IP_OPTIONS_ALLOWED==1: IP options are allowed (but not parsed).
 */
pub const IP_OPTIONS_ALLOWED: u32 = 1; 

/*
 * IP_REASSEMBLY==1: Reassemble incoming fragmented IP packets. Note that
 * this option does not affect outgoing packet sizes, which can be controlled
 * via IP_FRAG.
 */
pub const IP_REASSEMBLY: u32 = 1; 

/*
 * IP_FRAG==1: Fragment outgoing IP packets if their size exceeds MTU. Note
 * that this option does not affect incoming packet sizes, which can be
 * controlled via IP_REASSEMBLY.
 */
pub const IP_FRAG: u32 = 1; 

/*
 * IP_REASS_MAXAGE: Maximum time (in multiples of IP_TMR_INTERVAL - so seconds, normally)
 * a fragmented IP packet waits for all fragments to arrive. If not all fragments arrived
 * in this time, the whole packet is discarded.
 */
pub const IP_REASS_MAXAGE: u32 = 3; 

/*
 * IP_REASS_MAX_PBUFS: Total maximum amount of pbufs waiting to be reassembled.
 * Since the received pbufs are enqueued, be sure to configure
 * PBUF_POOL_SIZE > IP_REASS_MAX_PBUFS so that the stack is still able to receive
 * packets even if the maximum amount of fragments is enqueued for reassembly!
 */
pub const IP_REASS_MAX_PBUFS: u32 = 4; 

/*
 * IP_FRAG_USES_STATIC_BUF==1: Use a static MTU-sized buffer for IP
 * fragmentation. Otherwise pbufs are allocated and reference the original
    * packet data to be fragmented.
*/
pub const IP_FRAG_USES_STATIC_BUF: u32 = 0;

/*
 * IP_DEFAULT_TTL: Default value for Time-To-Live used by transport layers.
 */
pub const IP_DEFAULT_TTL: u32 = 255; 

/*
   ----------------------------------
   ---------- ICMP options ----------
   ----------------------------------
*/
/*
 * LWIP_ICMP==1: Enable ICMP module inside the IP stack.
 * Be careful, disable that make your product non-compliant to RFC1122
 */
// #define LWIP_ICMP                       1

/*
   ---------------------------------
   ---------- RAW options ----------
   ---------------------------------
*/
/*
 * LWIP_RAW==1: Enable application layer to hook into the IP layer itself.
 */
// #define LWIP_RAW                        1

/*
   ----------------------------------
   ---------- DHCP options ----------
   ----------------------------------
*/
/*
 * LWIP_DHCP==1: Enable DHCP module.
 */
pub const LWIP_DHCP: u32 = 0;


/*
   ------------------------------------
   ---------- AUTOIP options ----------
   ------------------------------------
*/
/*
 * LWIP_AUTOIP==1: Enable AUTOIP module.
 */
pub const LWIP_AUTOIP: u32 = 0;

/*
   ----------------------------------
   ---------- SNMP options ----------
   ----------------------------------
*/
/*
 * LWIP_SNMP==1: Turn on SNMP module. UDP must be available for SNMP
 * transport.
 */
pub const LWIP_SNMP: u32 = 0;

/*
   ----------------------------------
   ---------- IGMP options ----------
   ----------------------------------
*/
/*
 * LWIP_IGMP==1: Turn on IGMP module.
 */
pub const LWIP_IGMP: u32 = 0;

/*
   ----------------------------------
   ---------- DNS options -----------
   ----------------------------------
*/
/*
 * LWIP_DNS==1: Turn on DNS module. UDP must be available for DNS
 * transport.
 */
pub const LWIP_DNS: u32 = 0;

/*
   ---------------------------------
   ---------- UDP options ----------
   ---------------------------------
*/
/*
 * LWIP_UDP==1: Turn on UDP.
 */
// #define LWIP_UDP                        1

/*
   ---------------------------------
   ---------- TCP options ----------
   ---------------------------------
*/
/*
 * LWIP_TCP==1: Turn on TCP.
 */
// #define LWIP_TCP                        1

pub const LWIP_LISTEN_BACKLOG: u32 = 0;

/*
   ----------------------------------
   ---------- Pbuf options ----------
   ----------------------------------
*/
/*
 * PBUF_LINK_HLEN: the number of bytes that should be allocated for a
 * link level header. The default is 14, the standard value for
 * Ethernet.
 */
pub const PBUF_LINK_HLEN: u32 = 16; 

/*
 * PBUF_POOL_BUFSIZE: the size of each pbuf in the pbuf pool. The default is
 * designed to accomodate single full size TCP frame in one pbuf, including
 * TCP_MSS, IP header, and link header.
*
 */
#define PBUF_POOL_BUFSIZE               LWIP_MEM_ALIGN_SIZE(TCP_MSS+40+PBUF_LINK_HLEN)

/*
   ------------------------------------
   ---------- LOOPIF options ----------
   ------------------------------------
*/
/*
 * LWIP_HAVE_LOOPIF==1: Support loop interface (127.0.0.1) and loopif.c
 */
pub const LWIP_HAVE_LOOPIF: u32 = 0;

/*
   ----------------------------------------------
   ---------- Sequential layer options ----------
   ----------------------------------------------
*/

/*
 * LWIP_NETCONN==1: Enable Netconn API (require to use api_lib.c)
 */
// #define LWIP_NETCONN                    1

/*
   ------------------------------------
   ---------- Socket options ----------
   ------------------------------------
*/
/*
 * LWIP_SOCKET==1: Enable Socket API (require to use sockets.c)
 */
// #define LWIP_SOCKET                     1

/*
 * SO_REUSE==1: Enable SO_REUSEADDR
 */
pub const SO_REUSE: u32 = 1; 

/*
   ----------------------------------------
   ---------- Statistics options ----------
   ----------------------------------------
*/
/*
 * LWIP_STATS==1: Enable statistics collection in lwip_stats.
 */
pub const LWIP_STATS: u32 = 0;
/*
   ---------------------------------
   ---------- PPP options ----------
   ---------------------------------
*/
/*
 * PPP_SUPPORT==1: Enable PPP.
 */
pub const PPP_SUPPORT: u32 = 0;



/*
   ---------------------------------------
   ---------- Threading options ----------
   ---------------------------------------
*/

// #define LWIP_TCPIP_CORE_LOCKING    1


pub fn  sys_check_core_locking();
// #define LWIP_ASSERT_CORE_LOCKED()  sys_check_core_locking()
pub fn  sys_mark_tcpip_thread();
// #define LWIP_MARK_TCPIP_THREAD()   sys_mark_tcpip_thread()


pub fn  sys_lock_tcpip_core();
#define LOCK_TCPIP_CORE()          sys_lock_tcpip_core()
pub fn  sys_unlock_tcpip_core();
#define UNLOCK_TCPIP_CORE()        sys_unlock_tcpip_core()




