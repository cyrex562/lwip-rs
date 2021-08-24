/*
 * @file
 * IPv4 API
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

// #define LWIP_HDR_IP4_H

















// #define LWIP_IPV4_SRC_ROUTING   1

pub const LWIP_IPV4_SRC_ROUTING: u32 = 0;


/* Currently, the function ip_output_if_opt() is only used with IGMP */
#define IP_OPTIONS_SEND   (LWIP_IPV4 && LWIP_IGMP)

#define ip_init() /* Compatibility define, no init needed. */
ip4_route: &mut NetIfc(const dest: &mut ip4_addr);

ip4_route_src: &mut NetIfc(const src: &mut ip4_addr,  dest: &mut ip4_addr);
 /* LWIP_IPV4_SRC_ROUTING */
#define ip4_route_src(src, dest) ip4_route(dest)

pub fn  ip4_input(p: &mut pbuf, inp: &mut NetIfc);
pub fn  ip4_output(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
       ttl: u8, tos: u8, proto: u8);
pub fn  ip4_output_if(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
       ttl: u8, tos: u8, proto: u8, netif: &mut NetIfc);
pub fn  ip4_output_if_src(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
       ttl: u8, tos: u8, proto: u8, netif: &mut NetIfc);

pub fn  ip4_output_hinted(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
       ttl: u8, tos: u8, proto: u8, netif_hint: &mut netif_hint);


pub fn  ip4_output_if_opt(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
       ttl: u8, tos: u8, proto: u8, netif: &mut NetIfc, ip_options: &mut (),
       optlen: u16);
pub fn  ip4_output_if_opt_src(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
       ttl: u8, tos: u8, proto: u8, netif: &mut NetIfc, ip_options: &mut (),
       optlen: u16);



pub fn   ip4_set_default_multicast_netif(default_multicast_netif: &mut NetIfc);


#define ip4_netif_get_local_ip(netif) (((netif) != NULL) ? netif_ip_addr4(netif) : NULL)


pub fn  ip4_debug_print(p: &mut pbuf);

#define ip4_debug_print(p)










