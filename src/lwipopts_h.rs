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
 * Author: Simon Goldschmidt
 *
 */



pub const LWIP_TESTMODE: u32 = 1; #define LWIP_IPV6                       1

pub const LWIP_CHECKSUM_ON_COPY: u32 = 1; #define TCP_CHECKSUM_ON_COPY_SANITY_CHECK 1
#define TCP_CHECKSUM_ON_COPY_SANITY_CHECK_FAIL(printfmsg) LWIP_ASSERT("TCP_CHECKSUM_ON_COPY_SANITY_CHECK_FAIL", 0)

/* We link to special sys_arch.c (for basic non-waiting API layers unit tests) */
pub const NO_SYS: u32 = 0; #define SYS_LIGHTWEIGHT_PROT            0
#define LWIP_NETCONN                    !NO_SYS
#define LWIP_SOCKET                     !NO_SYS
#define LWIP_NETCONN_FULLDUPLEX         LWIP_SOCKET
pub const LWIP_NETCONN_SEM_PER_THREAD: u32 = 1; #define LWIP_NETBUF_RECVINFO            1
pub const LWIP_HAVE_LOOPIF: u32 = 1; #define TCPIP_THREAD_TEST

/* Enable DHCP to test it */
pub const LWIP_DHCP: u32 = 1; /* Enable DNS, with random source port to avoid alloc in dns_init */
pub const LWIP_DNS: u32 = 1; #define LWIP_DNS_SECURE (LWIP_DNS_SECURE_RAND_XID | LWIP_DNS_SECURE_RAND_SRC_PORT)

/* Minimal changes to opt.h required for tcp unit tests: */
pub const MEM_SIZE: u32 = 16000; #define TCP_SND_QUEUELEN                40
#define MEMP_NUM_TCP_SEG                TCP_SND_QUEUELEN
#define TCP_SND_BUF                     (12 * TCP_MSS)
#define TCP_WND                         (10 * TCP_MSS)
pub const LWIP_WND_SCALE: u32 = 1; #define TCP_RCV_SCALE                   0
pub const PBUF_POOL_SIZE: u32 = 400; /* pbuf tests need ~200KByte */

/* Enable IGMP and MDNS for MDNS tests */
pub const LWIP_IGMP: u32 = 1; #define LWIP_MDNS_RESPONDER             1
#define LWIP_NUM_NETIF_CLIENT_DATA      (LWIP_MDNS_RESPONDER)

/* Minimal changes to opt.h required for etharp unit tests: */
pub const ETHARP_SUPPORT_STATIC_ENTRIES: u32 = 1; #define MEMP_NUM_SYS_TIMEOUT            (LWIP_NUM_SYS_TIMEOUT_INTERNAL + 8)

/* MIB2 stats are required to check IPv4 reassembly results */
pub const MIB2_STATS: u32 = 1; /* netif tests want to test this, so enable: */
pub const LWIP_NETIF_EXT_STATUS_CALLBACK: u32 = 1; /* Check lwip_stats.mem.illegal instead of asserting */
#define LWIP_MEM_ILLEGAL_FREE(msg)      /* to nothing */

 /* LWIP_HDR_LWIPOPTS_H */
