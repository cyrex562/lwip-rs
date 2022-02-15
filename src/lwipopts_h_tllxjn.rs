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

#define LWIP_HDR_LWIPOPTS_H__

pub const MEMP_NUM_SYS_TIMEOUT: u32 = 17;
#define LWIP_FUZZ_SYS_NOW
#define LWIP_RAND_FOR_FUZZ

/* Prevent having to link sys_arch.c (we don't test the API layers in unit tests) */
pub const NO_SYS: u32 = 1;
pub const LWIP_NETCONN: u32 = 0;
pub const LWIP_SOCKET: u32 = 0;
pub const SYS_LIGHTWEIGHT_PROT: u32 = 0;

pub const LWIP_IPV6: u32 = 1;
pub const IPV6_FRAG_COPYHEADER: u32 = 1;
pub const LWIP_IPV6_DUP_DETECT_ATTEMPTS: u32 = 0;

/* Enable some protocols to test them */
pub const LWIP_DHCP: u32 = 1;
pub const LWIP_AUTOIP: u32 = 1;

pub const LWIP_IGMP: u32 = 1;
pub const LWIP_DNS: u32 = 1;

pub const LWIP_ALTCP: u32 = 1;

/* Turn off checksum verification of fuzzed data */
pub const CHECKSUM_CHECK_IP: u32 = 0;
pub const CHECKSUM_CHECK_UDP: u32 = 0;
pub const CHECKSUM_CHECK_TCP: u32 = 0;
pub const CHECKSUM_CHECK_ICMP: u32 = 0;
pub const CHECKSUM_CHECK_ICMP6: u32 = 0;

/* Minimal changes to opt.h required for tcp unit tests: */
pub const MEM_SIZE: u32 = 16000;
pub const TCP_SND_QUEUELEN: u32 = 40;
pub const MEMP_NUM_TCP_SEG: u32 = TCP_SND_QUEUELEN;
pub const TCP_OVERSIZE: u32 = 1;
#define TCP_SND_BUF                     (12 * TCP_MSS)
#define TCP_WND                         (10 * TCP_MSS)
pub const LWIP_WND_SCALE: u32 = 1;
pub const TCP_RCV_SCALE: u32 = 2;
pub const PBUF_POOL_SIZE: u32 = 400; /* pbuf tests need ~200KByte */

/* Minimal changes to opt.h required for etharp unit tests: */
pub const ETHARP_SUPPORT_STATIC_ENTRIES: u32 = 1;

pub const LWIP_NUM_NETIF_CLIENT_DATA: u32 = 1;
pub const LWIP_SNMP: u32 = 1;
pub const MIB2_STATS: u32 = 1;
pub const LWIP_MDNS_RESPONDER: u32 = 1;

 /* LWIP_HDR_LWIPOPTS_H__ */
