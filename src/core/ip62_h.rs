/**
 * @file
 *
 * IPv6 layer.
 */

/*
 * Copyright (c) 2010 Inico Technologies Ltd.
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
 * Author: Ivan Delamer <delamer@inicotech.com>
 *
 *
 * Please coordinate changes and requests with Ivan Delamer
 * <delamer@inicotech.com>
 */

#define LWIP_HDR_IP6_H














extern "C" {


ip6_route: &mut netif(const ip6_addr_t *src, const ip6_addr_t *dest);
const ip6_select_source_address: &mut ip_addr_t(netif: &mut netif, const ip6_addr_t * dest);
pub fn          ip6_input(p: &mut pbuf, inp: &mut netif);
pub fn          ip6_output(p: &mut pbuf, const ip6_addr_t *src, const ip6_addr_t *dest,
                         hl: u8, tc: u8, nexth: u8);
pub fn          ip6_output_if(p: &mut pbuf, const ip6_addr_t *src, const ip6_addr_t *dest,
                            hl: u8, tc: u8, nexth: u8, netif: &mut netif);
pub fn          ip6_output_if_src(p: &mut pbuf, const ip6_addr_t *src, const ip6_addr_t *dest,
                            hl: u8, tc: u8, nexth: u8, netif: &mut netif);

pub fn          ip6_output_hinted(p: &mut pbuf, const ip6_addr_t *src, const ip6_addr_t *dest,
                                hl: u8, tc: u8, nexth: u8, netif_hint: &mut netif_hint);


pub fn          ip6_options_add_hbh_ra(struct pbuf * p, nexth: u8, value: u8);


#define ip6_netif_get_local_ip(netif, dest) (((netif) != NULL) ? \
  ip6_select_source_address(netif, dest) : NULL)


pub fn  ip6_debug_print(p: &mut pbuf);
#else
#define ip6_debug_print(p)




}





