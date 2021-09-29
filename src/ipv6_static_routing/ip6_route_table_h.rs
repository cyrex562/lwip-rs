/*
 * @file
 *
 * IPv6 static route table.
 */

/*
 * Copyright (c) 2015 Nest Labs, Inc.
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
 * Author: Pradip De <pradipd@google.com>
 *
 *
 * Please coordinate changes and requests with Pradip De
 * <pradipd@google.com>
 */

// #define __LWIP_IP6_ROUTE_TABLE_H__

/*
 * LWIP_IPV6_NUM_ROUTES: Number of IPV6 routes that can be kept in the static route table.
 */

// #define LWIP_IPV6_NUM_ROUTE_ENTRIES         (8)

pub const IP6_MAX_PREFIX_LEN: usize = (128);
pub const IP6_PREFIX_ALLOWED_GRANULARITY: u8 = (8);
//  Prefix length cannot be greater than 128 bits and needs to be at a byte boundary 
pub fn ip6_prefix_valid(prefix_len: usize) -> bool {
    (((prefix_len) <= IP6_MAX_PREFIX_LEN) && (((prefix_len) % IP6_PREFIX_ALLOWED_GRANULARITY) == 0))
}

pub struct ip6_prefix {
    pub addr: ip6_addr,
    pub prefix_len: u8,
    //  prefix length in bits at byte boundaries 
}

pub struct ip6_route_entry {
    pub prefix: ip6_prefix,
    pub netif: netif,
    pub gateway: ip6_addr,
}

// pub fn  ip6_add_route_entry( ip6_prefix: &mut ip6_prefix, netif: &mut NetIfc,
//                           const gateway: &mut ip6_addr_t, idx: &mut i8);
// pub fn  ip6_remove_route_entry( ip6_prefix: &mut ip6_prefix);
// s8_t ip6_find_route_entry( ip6_dest_addr: &mut ip6_addr_t);
// ip6_static_route: &mut NetIfc( src: &mut ip6_addr_t,  dest: &mut ip6_addr_t);
// const ip6_get_gateway: &mut ip6_addr_t(netif: &mut NetIfc,  dest: &mut ip6_addr_t);
// const ip6_get_route_table: &mut ip6_route_entry();
