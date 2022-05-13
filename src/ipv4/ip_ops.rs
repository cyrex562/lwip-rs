use crate::netif_hint::NetifHint;
use crate::ip_address::{IpAddress};

/**
 * @file
 * IP protocol definitions
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



// #include "lwip/arch.h"




pub const IP_PROTO_ICMP: u32 = 1;
pub const IP_PROTO_IGMP: u32 =    2;
pub const IP_PROTO_UDP: u32 = 17;
pub const IP_PROTO_UDPLITE: u32 = 136;
pub const IP_PROTO_TCP: u32 = 6; /** This operates on a void* by loading the first byte */
// #define IP_HDR_GET_VERSION(ptr)   ((*(u8_t*)(ptr)) >> 4)

// #define LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p)

// /*
//  * Option flags per-socket. These are the same like SO_XXX in sockets.h
//  */
pub const SOF_REUSEADDR: u8   =  0x04;  /* allow local address reuse */
pub const SOF_KEEPALIVE: u8 =     0x08;  /* keep connections alive */
pub const SOF_BROADCAST: u8 =     0x20;  /* permit to send and to receive broadcast messages (see IP_SOF_BROADCAST option) */
pub const SOF_INHERITED: u8 = SOF_REUSEADDR | SOF_KEEPALIVE;



pub struct IpContext {
    local_ip: IpAddress,
    remote_ip: IpAddress,
    netif_idx: u8,
    so_options: u8,
    tos: u8,
    ttl: u8,
}

impl IpContext {
    pub fn get_option(&self, opt: u8) -> u8 {
        self.so_options & opt
    }

    pub fn set_option(&mut self, opt: u8) {
        self.so_options = self.so_options | opt
    }

    pub fn reset_option(&mut self, opt: u8) {
        self.so_options = self.so_options & !opt
    }
}

pub const OBJ_ID_NOT_SET: u32 = 0xFFFFFFFF;

pub struct IpGlobals {
    // interface that accepted packet for current callback invocation
    current_netif_id: u32,
    // interface that received the packet for the current callback invocation
    current_input_netif_id: u32,
    // header of the input packet being processed
    current_ip4_header_id: u32,
    // header of the input ipv6 packet being processed
    current_ip6_header_id: u32,
    // total header length of the current ip4/ip6 header
    current_ip_header_tot_len: usize,
    // source ip address of current header
    current_ip_header_src: IpAddress,
    // destination ip address of the current header
    current_ip_header_dst: IpAddress
}

impl IpGlobals {
    pub fn ip_current_is_v6(&self) -> bool {
        self.current_ip6_header_id != OBJ_ID_NOT_SET
    }

    pub fn ip_current_header_proto(&self) -> u16 {
        if self.ip_current_is_v6() {
            IP6H_NEXTH(&self.current_ip6_header_id)
        }
        IPH_PROTO(&self.current_ip4_header_id)
    }

    pub fn ip_next_header_ptr(&self) -> usize {
        // get the transport layer header
        unimplemented!()
    }
}

pub fn ip_output(p: &mut PacketBuffer, src: &IpAddress, dst: &IpAddress, ttl: u8, tos: u8, proto: u16) {
    if IP_IS_V6(dst) { ip6_output(p, src, dst, ttl, tos, proto)} else { ip4_output(p, src, dst, ttl, tos, proto)}
}

pub fn ip_output_if_src(p: &mut PacketBuffer,
                        src: &IpAddress,
                        dst: &IpAddress,
                        ttl: u8,
                        tos: u8,
                        proto: u8,
                        netif: &mut NetworkInterface) {
    if IP_IS_V6(dst) {
        ip6_output_if_src(p, src, dst, ttl, tos, proto, netif)
    } else {
        ip4_output_if_src(p, src, dst, ttl, tos, proto, netif)
    }
}

pub fn ip_output_if_hdrincl(p: &mut PacketBuffer, src: &IpAddress, dst: &IpAddress, netif: &mut NetworkInterface) {
    if IP_IS_V6(dst) {
        ip6_output_if_src(p, src, dst, 0, 0, 0, netif)
    } else {
        ip4_output_if_src(p, src, dst, 0, 0, 0, netif)
    }
}

pub fn ip_output_hinted(p: &mut PacketBuffer, src: &IpAddress, dst: &IpAddress, ttl: u8, tos: u8, proto: u8, netif_hint: &NetifHint) {
    if IP_IS_V6(dst) {
        ip6_output_hinted(p, src, dst, ttl, tos, proto, netif_hint)
    } else {
        ip4_output_hinted(p, src, dst, ttl, tos, proto, netif_hint)
    }
}

 /* LWIP_HDR_PROT_IP_H */
