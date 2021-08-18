/*
 * @file
 * Ethernet output function - handles OUTGOING ethernet level traffic, implements
 * ARP resolving.
 * To be used in most low-level netif implementations
 */

/*
 * Copyright (c) 2001-2003 Swedish Institute of Computer Science.
 * Copyright (c) 2003-2004 Leon Woestenberg <leon.woestenberg@axon.tv>
 * Copyright (c) 2003-2004 Axon Digital Design B.V., The Netherlands.
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

// #define LWIP_HDR_NETIF_ETHARP_H

//

/* 1 seconds period */
// #define ARP_TMR_INTERVAL 1000
pub const ARP_TMR_INTERVAL: u32 = 1000;

/* struct for queueing outgoing packets for unknown address
 * defined here to be accessed by memp.h
 */
pub struct etharp_q_entry {
    // next: &mut etharp_q_entry;
    p: &mut pbuf,
}

// #define etharp_init() /* Compatibility define, no init needed. */
// pub fn  etharp_tmr();
// isize etharp_find_addr(netif: &mut netif,  ipaddr: &mut ip4_addr,
//  struct eth_addr **eth_ret,  ip4_addr **ip_ret);
// etharp_get_entry: i32(i: usize, ip4_addr **ipaddr, struct netif **netif, struct eth_addr **eth_ret);
// pub fn  etharp_output(netif: &mut netif, q: &mut pbuf,  ipaddr: &mut ip4_addr);
// pub fn  etharp_query(netif: &mut netif,  ipaddr: &mut ip4_addr, q: &mut pbuf);
// pub fn  etharp_request(netif: &mut netif,  ipaddr: &mut ip4_addr);
/* For Ethernet network interfaces, we might want to send "gratuitous ARP";
 *  this is an ARP packet sent by a node in order to spontaneously cause other
 *  nodes to update an entry in their ARP cache.
 *  From RFC 3220 "IP Mobility Support for IPv4" section 4.6. */
// #define etharp_gratuitous(netif) etharp_request((netif), netif_ip4_addr(netif))
// pub fn  etharp_cleanup_netif(netif: &mut netif);
// pub fn  etharp_add_static_entry(const ipaddr: &mut ip4_addr, ethaddr: &mut eth_addr);
// pub fn  etharp_remove_static_entry(const ipaddr: &mut ip4_addr);
// pub fn  etharp_input(p: &mut pbuf, netif: &mut netif);

// }
