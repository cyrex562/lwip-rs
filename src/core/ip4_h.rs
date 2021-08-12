/*
 * @file
 * IPv4 protocol definitions
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

// #define LWIP_HDR_PROT_IP4_H








/* This is the packed version of ip4_addr,
    used in network headers that are itself packed */



pub struct ip4_addr_packed {
  addr: u32,
}

type ip4_addr_p_t = ip4_addr_packed;


/* Size of the IPv4 header. Same as 'sizeof(struct ip_hdr)'. */
// #define IP_HLEN 20
pub const IP_HLEN: usize = 20;
/* Maximum size of the IPv4 header with options. */
// #define IP_HLEN_MAX 60
pub const IP_HLEN_MAX: usize = 60;

pub const IP_RF: u32 = 0x8000;
/* reserved fragment flag */
pub const IP_DF: u32 = 0x4000;        /* don't fragment flag */
pub const IP_MF: u32 = 0x2000;        /* more fragments flag */
pub const IP_OFFMASK: u32 = 0x1fff;   /* mask for fragmenting bits */


/* The IPv4 header */
pub struct ip_hdr {
  /* version / header length */
  _v_hl: u8,
  /* type of service */
  _tos: u8,
  /* total length */
  _len: u16,
  /* identification */
  _id: u16,
  /* fragment offset field */
  _offset: u16,
  /* time to live */
  _ttl: u8,
  /* protocol*/
  _proto: u8,
  /* checksum */
  _chksum: u16,
  /* source and destination IP addresses */
  src: ip4_addr_p_t,
  dest: ip4_addr_p_t,
}



/* Macros to get struct ip_hdr fields: */
// #define IPH_V(hdr)  ((hdr)->_v_hl >> 4)
pub fn IPH_V(hdr: &ip_hdr) -> u8 {
    hdr._v_hl >> 4
}
// #define IPH_HL(hdr) ((hdr)->_v_hl & 0x0f)
pub fn IPH_HL(hdr: &ip_hdr) -> u8 {
    hdr._v_hl & 0x0f
}
// #define IPH_HL_BYTES(hdr) ((IPH_HL(hdr) * 4))
pub fn IPH_HL_BYTES(hdr: &ip_hdr) -> usize {
    (IPH_HL(hdr) * 4) as usize
}

// #define IPH_TOS(hdr) ((hdr)->_tos)
pub fn IPH_TOS(hdr: &ip_hdr) -> u8 {
    hdr._tos
}
// #define IPH_LEN(hdr) ((hdr)->_len)
pub fn IPH_LEN(hdr: &ip_hdr) -> u16 {
    hdr._len
}
// #define IPH_ID(hdr) ((hdr)->_id)
pub fn IPH_ID(hdr: &ip_hdr) -> u16 {
    hdr._id
}
// #define IPH_OFFSET(hdr) ((hdr)->_offset)
pub fn IPH_OFFSET(hdr: &ip_hdr) -> u16 {
    hdr._offset
}
// #define IPH_OFFSET_BYTES(hdr) (((lwip_ntohs(IPH_OFFSET(hdr)) & IP_OFFMASK) * 8U))
pub fn IPH_OFFSET_BYTES(hdr: &ip_hdr) -> usize {
    ((IPH_OFFSET(hdr) & IP_OFFMASK) * 8) as usize
}
// #define IPH_TTL(hdr) ((hdr)->_ttl)
pub fn IPH_TTL(hdr: &ip_hdr) -> u8 {
    hdr._ttl
}
// #define IPH_PROTO(hdr) ((hdr)->_proto)
pub fn IPH_PROTO(hdr: &ip_hdr) -> u8 {
    hdr._proto
}

// #define IPH_CHKSUM(hdr) ((hdr)->_chksum)
pub fn IPH_CHKSUM(hdr: &ip_hdr) -> u16 {
    hdr._chksum
}

/* Macros to set struct ip_hdr fields: */
// #define IPH_VHL_SET(hdr, v, hl) (hdr)->_v_hl = ((((v) << 4) | (hl)))
pub fn IPH_VHL_SET(hdr: &mut ip_hdr, v: u8, hl: u8) {
    hdr._v_hl = (v << 4) | hl
}

// #define IPH_TOS_SET(hdr, tos) (hdr)->_tos = (tos)
pub fn IPH_TOS_SET(hdr: &mut ip_hdr, tos: u8) {
    hdr._tos = tos
}

// #define IPH_LEN_SET(hdr, len) (hdr)->_len = (len)
pub fn IPH_LEN_SET(hdr: &mut ip_hdr, len: u16) {
    hdr._len = len
}

// #define IPH_ID_SET(hdr, id) (hdr)->_id = (id)
pub fn IPH_ID_SET(hdr: &mut ip_hdr, id: u16) {
    hdr._id = id
}
// #define IPH_OFFSET_SET(hdr, off) (hdr)->_offset = (off)
pub fn IPH_OFFSET_SET(hdr: &mut ip_hdr, off: u16) {
    hdr._offset = off
}

// #define IPH_TTL_SET(hdr, ttl) (hdr)->_ttl = (ttl)
pub fn IPH_TTL_SET(hdr: &mut ip_hdr, ttl: u8) {
    hdr._ttl = ttl
}
// #define IPH_PROTO_SET(hdr, proto) (hdr)->_proto = (proto)
pub fn IPH_PROTO_SET(hdr: &mut ip_hdr, proto: u8) {
    hdr._proto = proto
}
// #define IPH_CHKSUM_SET(hdr, chksum) (hdr)->_chksum = (chksum)
pub fn IPH_CHKSUM_SET(hdr: &mut ip_hdr, chksum: u16) {
    hdr._chksum = chksum
}


