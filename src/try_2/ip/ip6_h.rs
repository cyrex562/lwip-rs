/*
 * @file
 * IPv6 protocol definitions
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

// pub const in6addr_any: in6_addr = IN6ADDR_ANY_INIT;

//

/* This is the packed version of ip6_addr_t,
used in network headers that are itself packed */

pub struct ip6_addr_packed {
    pub addr: [u32; 4],
}

// typedef struct ip6_addr_packed ip6_addr_p_t;

pub const IP6_HLEN: usize = 40;
pub const IP6_NEXTH_HOPBYHOP: u32 = 0;
pub const IP6_NEXTH_TCP: u32 = 6;
pub const IP6_NEXTH_UDP: u32 = 17;
pub const IP6_NEXTH_ENCAPS: u32 = 41;
pub const IP6_NEXTH_ROUTING: u32 = 43;
pub const IP6_NEXTH_FRAGMENT: u32 = 44;
pub const IP6_NEXTH_ICMP6: u32 = 58;
pub const IP6_NEXTH_NONE: u32 = 59;
pub const IP6_NEXTH_DESTOPTS: u32 = 60;
pub const IP6_NEXTH_UDPLITE: u32 = 136;

//  The IPv6 header.

pub struct ip6_hdr {
    //  version / traffic class / flow label
    pub _v_tc_fl: u32,
    //  payload length
    pub _plen: u16,
    //  next header
    pub _nexth: u8,
    //  hop limit
    pub _hoplim: u8,
    //  source and destination IP addresses
    // (ip6_addr_p_t src);
    pub src: ip6_addr_p_t,
    // (ip6_addr_p_t dest);
    pub dest: ip6_addr_p_t,
}

// #define IP6H_V(hdr)  ((lwip_ntohl((hdr)._v_tc_fl) >> 28) & 0x0f)
// #define IP6H_TC(hdr) ((lwip_ntohl((hdr)._v_tc_fl) >> 20) & 0xff)
// #define IP6H_FL(hdr) (lwip_ntohl((hdr)._v_tc_fl) & 0x000fffff)
// #define IP6H_PLEN(hdr) (lwip_ntohs((hdr)._plen))
// #define IP6H_NEXTH(hdr) ((hdr)._nexth)
// #define IP6H_NEXTH_P(hdr) ((hdr) + 6)
// #define IP6H_HOPLIM(hdr) ((hdr)._hoplim)
// #define IP6H_VTCFL_SET(hdr, v, tc, fl) (hdr)._v_tc_fl = (lwip_htonl((((v)) << 28) | (((tc)) << 20) | (fl)))
// #define IP6H_PLEN_SET(hdr, plen) (hdr)._plen = lwip_htons(plen)
// #define IP6H_NEXTH_SET(hdr, nexth) (hdr)._nexth = (nexth)
// #define IP6H_HOPLIM_SET(hdr, hl) (hdr)._hoplim = (hl)

//  ipv6 extended options header
pub const IP6_PAD1_OPTION: u32 = 0;
pub const IP6_PADN_OPTION: u32 = 1;
pub const IP6_ROUTER_ALERT_OPTION: u32 = 5;
pub const IP6_JUMBO_OPTION: u32 = 194;
pub const IP6_HOME_ADDRESS_OPTION: u32 = 201;
pub const IP6_ROUTER_ALERT_DLEN: u32 = 2;
pub const IP6_ROUTER_ALERT_VALUE_MLD: u32 = 0;

pub struct ip6_opt_hdr {
    //  router alert option type
    pub _opt_type: u8,
    //  router alert option data len
    pub _opt_dlen: u8,
}

pub const IP6_OPT_HLEN: usize = 2;
// #define IP6_OPT_TYPE_ACTION(hdr) ((((hdr)._opt_type) >> 6) & 0x3)
// #define IP6_OPT_TYPE_CHANGE(hdr) ((((hdr)._opt_type) >> 5) & 0x1)
// #define IP6_OPT_TYPE(hdr) ((hdr)._opt_type)
// #define IP6_OPT_DLEN(hdr) ((hdr)._opt_dlen)

//  Hop-by-Hop header.
pub const IP6_HBH_HLEN: usize = 2;

pub struct ip6_hbh_hdr {
    //  next header
    pub _nexth: u8,
    //  header length in 8-octet units
    pub _hlen: u8,
}

// #define IP6_HBH_NEXTH(hdr) ((hdr)._nexth)

//  Destination header.
pub const IP6_DEST_HLEN: usize = 2;

pub struct ip6_dest_hdr {
    //  next header
    pub _nexth: u8,
    //  header length in 8-octet units
    pub _hlen: u8,
}

// #define IP6_DEST_NEXTH(hdr) ((hdr)._nexth)

//  Routing header
pub const IP6_ROUT_TYPE2: u32 = 2;
pub const IP6_ROUT_RPL: u32 = 3;

pub struct ip6_rout_hdr {
    //  next header
    pub _nexth: u8,
    //  reserved
    pub _hlen: u8,
    //  fragment offset
    pub _routing_type: u8,
    //  fragmented packet identification
    pub _segments_left: u8,
}

// #define IP6_ROUT_NEXTH(hdr) ((hdr)._nexth)
// #define IP6_ROUT_TYPE(hdr) ((hdr)._routing_type)
// #define IP6_ROUT_SEG_LEFT(hdr) ((hdr)._segments_left)

//  Fragment header.
pub const IP6_FRAG_HLEN: usize = 8;
pub const IP6_FRAG_OFFSET_MASK: u32 = 0xfff8;
pub const IP6_FRAG_OFFSET_MASK: u32 = 0xfff8;
pub const IP6_FRAG_MORE_FLAG: u16 = 0x0001;

pub struct ip6_frag_hdr {
    //  next header
    pub _nexth: u8,
    //  reserved
    pub reserved: u8,
    //  fragment offset
    pub _fragment_offset: u16,
    //  fragmented packet identification
    pub _identification: u32,
}

// #define IP6_FRAG_NEXTH(hdr) ((hdr)._nexth)
// #define IP6_FRAG_MBIT(hdr) (lwip_ntohs((hdr)._fragment_offset) & 0x1)
// #define IP6_FRAG_ID(hdr) (lwip_ntohl((hdr)._identification))
