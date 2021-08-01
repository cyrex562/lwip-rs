/*
 * @file
 * Modules initialization
 *
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
 */






























#  include "arch/bpstruct.h"


struct packed_struct_test {
  (u8  dummy1);
  (dummy2: u32);
} ;


#  include "arch/epstruct.h"

#define PACKED_STRUCT_TEST_EXPECTED_SIZE 5



/* Compile-time sanity checks for configuration errors.
 * These can be done independently of LWIP_DEBUG, without penalty.
 */

#error "BYTE_ORDER is not defined, you have to define it in your cc.h"


#error "If you want to use broadcast filter per pcb on recv operations, you have to define IP_SOF_BROADCAST=1 in your lwipopts.h"


#error "If you want to use UDP Lite, you have to define LWIP_UDP=1 in your lwipopts.h"


#error "If you want to use DHCP, you have to define LWIP_UDP=1 in your lwipopts.h"


#error "If you want to use LWIP_MULTICAST_TX_OPTIONS, you have to define LWIP_UDP=1 and/or LWIP_RAW=1 in your lwipopts.h"


#error "If you want to use DNS, you have to define LWIP_UDP=1 in your lwipopts.h"



#error "If you want to use ARP Queueing, you have to define MEMP_NUM_ARP_QUEUE>=1 in your lwipopts.h"


#error "If you want to use RAW, you have to define MEMP_NUM_RAW_PCB>=1 in your lwipopts.h"


#error "If you want to use UDP, you have to define MEMP_NUM_UDP_PCB>=1 in your lwipopts.h"


#error "If you want to use TCP, you have to define MEMP_NUM_TCP_PCB>=1 in your lwipopts.h"


#error "If you want to use IGMP, you have to define MEMP_NUM_IGMP_GROUP>1 in your lwipopts.h"


#error "If you want to use IGMP, you have to define LWIP_MULTICAST_TX_OPTIONS==1 in your lwipopts.h"


#error "IGMP needs LWIP_IPV4 enabled in your lwipopts.h"


#error "If you want to use Sequential API, you have to define MEMP_NUM_TCPIP_MSG_API>=1 in your lwipopts.h"

/* There must be sufficient timeouts, taking into account requirements of the subsystems. */

#error "MEMP_NUM_SYS_TIMEOUT is too low to accomodate all required timeouts"


#error "MEMP_NUM_REASSDATA > IP_REASS_MAX_PBUFS doesn't make sense since each struct ip_reassdata must hold 2 pbufs at least!"




#error "If you want to use TCP, TCP_WND must fit in an u32, so, you have to reduce it in your lwipopts.h"


#error "The maximum valid window scale value is 14!"


#error "TCP_WND is bigger than the configured LWIP_WND_SCALE allows!"


#error "TCP_WND is too small for the configured LWIP_WND_SCALE (results in zero window)!"

#else /* LWIP_WND_SCALE */

#error "If you want to use TCP, TCP_WND must fit in an u16, so, you have to reduce it in your lwipopts.h (or enable window scaling)"



#error "If you want to use TCP, TCP_SND_QUEUELEN must fit in an u16, so, you have to reduce it in your lwipopts.h"


#error "TCP_SND_QUEUELEN must be at least 2 for no-copy TCP writes to work"


#error "If you want to use TCP, TCP_MAXRTX and TCP_SYNMAXRTX must less or equal to 12 (due to tcp_backoff table), so, you have to reduce them in your lwipopts.h"


#error "If you want to use TCP backlog, TCP_DEFAULT_LISTEN_BACKLOG must fit into an u8"


#error "To use LWIP_TCP_SACK_OUT, TCP_QUEUE_OOSEQ needs to be enabled"


#error "LWIP_TCP_MAX_SACK_NUM must be greater than 0"


#error "If you want to use NETIF API, you have to define NO_SYS=0 in your lwipopts.h"


#error "If you want to use Sequential API, you have to define NO_SYS=0 in your lwipopts.h"


#error "If you want to use PPP API, you have to define NO_SYS=0 in your lwipopts.h"


#error "If you want to use PPP API, you have to enable PPP_SUPPORT in your lwipopts.h"


#error "If you want to use DHCP/AUTOIP cooperation mode, you have to define LWIP_DHCP=1 and LWIP_AUTOIP=1 in your lwipopts.h"


#error "If you want to use DHCP ARP checking, you have to define LWIP_DHCP=1 and LWIP_ARP=1 in your lwipopts.h"


#error "If you want to use AUTOIP, you have to define LWIP_ARP=1 in your lwipopts.h"


#error "One and exactly one of LWIP_EVENT_API and LWIP_CALLBACK_API has to be enabled in your lwipopts.h"


#error "The application layered tcp API does not work with LWIP_EVENT_API"


#error "MEM_LIBC_MALLOC and MEM_USE_POOLS may not both be simultaneously enabled in your lwipopts.h"


#error "MEM_USE_POOLS requires custom pools (MEMP_USE_CUSTOM_POOLS) to be enabled in your lwipopts.h"


#error "PBUF_POOL_BUFSIZE must be greater than MEM_ALIGNMENT or the offset may take the full first pbuf"


#error "you have to define define DNS_LOCAL_HOSTLIST_INIT {{'host1', 0x123}, {'host2', 0x234}} to initialize DNS_LOCAL_HOSTLIST"


#error "PPP_SUPPORT needs at least one of PPPOS_SUPPORT, PPPOE_SUPPORT or PPPOL2TP_SUPPORT turned on"


#error "PPP_SUPPORT needs PPP_IPV4_SUPPORT and/or PPP_IPV6_SUPPORT turned on"


#error "PPP_IPV4_SUPPORT needs LWIP_IPV4 turned on"


#error "PPP_IPV6_SUPPORT needs LWIP_IPV6 turned on"


#error "LWIP_ETHERNET needs to be turned on for LWIP_ARP or PPPOE_SUPPORT"


#error "When using LWIP_TCPIP_CORE_LOCKING_INPUT, LWIP_TCPIP_CORE_LOCKING must be enabled, too"


#error "LWIP_NETIF_TX_SINGLE_PBUF needs TCP_OVERSIZE enabled to create single-pbuf TCP packets"



#error "NETCONN_COPY != TCP_WRITE_FLAG_COPY"


#error "NETCONN_MORE != TCP_WRITE_FLAG_MORE"






/* Compile-time checks for deprecated options.
 */

#error "MEMP_NUM_TCPIP_MSG option is deprecated. Remove it from your lwipopts.h."


#error "TCP_REXMIT_DEBUG option is deprecated. Remove it from your lwipopts.h."


#error "RAW_STATS option is deprecated. Remove it from your lwipopts.h."


#error "ETHARP_QUEUE_FIRST option is deprecated. Remove it from your lwipopts.h."


#error "ETHARP_ALWAYS_INSERT option is deprecated. Remove it from your lwipopts.h."


#error "LWIP_COMPAT_MUTEX cannot prevent priority inversion. It is recommended to implement priority-aware mutexes. (Define LWIP_COMPAT_MUTEX_ALLOWED to disable this error.)"



pub const LWIP_DISABLE_TCP_SANITY_CHECKS: u32 = 0;


pub const LWIP_DISABLE_MEMP_SANITY_CHECKS: u32 = 0;


/* MEMP sanity checks */




#error "lwip_sanity_check: WARNING: MEMP_NUM_NETCONN cannot be 0 when using sockets!"

#else /* MEMP_MEM_MALLOC */

#error "lwip_sanity_check: WARNING: MEMP_NUM_NETCONN should be less than the sum of MEMP_NUM_{TCP,RAW,UDP}_PCB+MEMP_NUM_TCP_PCB_LISTEN. If you know what you are doing, define LWIP_DISABLE_MEMP_SANITY_CHECKS to 1 to disable this error."




#error "MEMP_MEM_MALLOC and MEM_USE_POOLS cannot be enabled at the same time"


#error "LWIP_HOOK_MEMP_AVAILABLE doesn't make sense with MEMP_MEM_MALLOC"



/* TCP sanity checks */



#error "lwip_sanity_check: WARNING: MEMP_NUM_TCP_SEG should be at least as big as TCP_SND_QUEUELEN. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: TCP_SND_BUF must be at least as much as (2 * TCP_MSS) for things to work smoothly. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: TCP_SND_QUEUELEN must be at least as much as (2 * TCP_SND_BUF/TCP_MSS) for things to work. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: TCP_SNDLOWAT must be less than TCP_SND_BUF. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: TCP_SNDLOWAT must at least be 4*MSS below overflow: u16!"


#error "lwip_sanity_check: WARNING: TCP_SNDQUEUELOWAT must be less than TCP_SND_QUEUELEN. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: PBUF_POOL_BUFSIZE does not provide enough space for protocol headers. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: TCP_WND is larger than space provided by PBUF_POOL_SIZE * (PBUF_POOL_BUFSIZE - protocol headers). If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."


#error "lwip_sanity_check: WARNING: TCP_WND is smaller than MSS. If you know what you are doing, define LWIP_DISABLE_TCP_SANITY_CHECKS to 1 to disable this error."




/*
 * @ingroup lwip_nosys
 * Initialize all modules.
 * Use this in NO_SYS mode. Use tcpip_init() otherwise.
 */
pub fn 
lwip_init()
{

  a: int = 0;
  LWIP_UNUSED_ARG(a);
  LWIP_ASSERT("LWIP_CONST_CAST not implemented correctly. Check your lwIP port.", LWIP_CONST_CAST(void *, &a) == &a);


  LWIP_ASSERT("Struct packing not implemented correctly. Check your lwIP port.", sizeof(struct packed_struct_test) == PACKED_STRUCT_TEST_EXPECTED_SIZE);


  /* Modules initialization */
  stats_init();

  sys_init();

  mem_init();
  memp_init();
  pbuf_init();
  netif_init();

  ip_init();

  etharp_init();



  raw_init();


  udp_init();


  tcp_init();


  igmp_init();


  dns_init();


  ppp_init();



  sys_timeouts_init();

}
