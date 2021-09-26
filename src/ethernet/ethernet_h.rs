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

// #define LWIP_HDR_PROT_ETHERNET_H

pub const ETH_HWADDR_LEN: u32 = ETHARP_HWADDR_LEN; /* compatibility mode */

// #define ETH_HWADDR_LEN    6

/* An Ethernet MAC address */
// struct eth_addr {
//   (addr: [u8;ETH_HWADDR_LEN]);
// } ;

/* Initialize a struct eth_addr with its 6 bytes (takes care of correct braces) */
// #define ETH_ADDR(b0, b1, b2, b3, b4, b5) {{b0, b1, b2, b3, b4, b5}}

/* Ethernet header */
pub struct eth_hdr {
    pub padding: [u8; ETH_PAD_SIZE],
    pub src_addr: [u8; ETH_HWADDR_LEN],
    pub dst_addr: [u8; ETH_HWADDR_LEN],
    pub ether_type: u16,
}

pub const SIZEOF_ETH_HDR: usize = (14 + ETH_PAD_SIZE);

/* VLAN header inserted between ethernet header and payload
 * if 'type' in ethernet header is ETHTYPE_VLAN.
 * See IEEE802.Q */
pub struct eth_vlan_hdr {
    pub prio_vid: u16,
    pub tpid: u16,
}

impl eth_vlan_hdr {
    pub fn VLAN_ID(self: &Self) -> u16 {
        lwip_htons(self.prio_vid) & 0x0FFF
    }
}

pub const VLAND_HDR_SIZE: usize = 4;

/* The 24-bit IANA IPv4-multicast OUI is 01-00-5e: */
pub const LL_IP4_MULTICAST_ADDR_0: u32 = 0x01;
pub const LL_IP4_MULTICAST_ADDR_1: u32 = 0x00;
pub const LL_IP4_MULTICAST_ADDR_2: u32 = 0x5e;

/* IPv6 multicast uses this prefix */
pub const LL_IP6_MULTICAST_ADDR_0: u32 = 0x33;
pub const LL_IP6_MULTICAST_ADDR_1: u32 = 0x33;

pub fn eth_addr_cmp(addr1: &[u8; 6], addr2: &[u8; 6]) -> bool {
    addr1 == addr2
}
