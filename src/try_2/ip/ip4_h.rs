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

//

/* This is the packed version of ip4_addr,
used in network headers that are itself packed */

use std::convert::TryInto;

pub struct ip4_addr_packed {
    addr: u32,
}

type ip4_addr_p_t = ip4_addr_packed;

//  Size of the IPv4 header. Same as 'sizeof(ip_hdr)'.
// pub const IP_HLEN: u32 = 20;
pub const IP_HLEN: usize = 20;
//  Maximum size of the IPv4 header with options.
// pub const IP_HLEN_MAX: u32 = 60;
pub const IP_HLEN_MAX: usize = 60;

pub const IP_RF: u32 = 0x8000;
//  reserved fragment flag
pub const IP_DF: u32 = 0x4000; //  don't fragment flag
pub const IP_MF: u32 = 0x2000; //  more fragments flag
pub const IP_OFFMASK: u32 = 0x1fff; //  mask for fragmenting bits

//  The IPv4 header
#[derive(Clone, Debug, Default)]
pub struct Ip4Header {
    //  version / header length
    v_hl: u8,
    //  type of service
    tos: u8,
    //  total length
    tot_len: u16,
    //  identification
    id: u16,
    //  fragment offset field
    frag_offset: u16,
    //  time to live
    ttl: u8,
    //  protocol
    proto: u8,
    //  checksum
    chksum: u16,
    //  source and destination IP addresses
    src_address: u32,
    dst_address: u32,
}

impl Ip4Header {
    pub fn get_version(&self) -> u8 {
        self.v_hl >> 4
    }
    pub fn get_hdr_len(&self) -> u8 {
        self.v_hl & 0x0f
    }
    pub fn get_hdr_len_bytes(&self) -> usize {
        (self::IPH_HL() * 4) as usize
    }
    pub fn get_offset_bytes(&self) -> usize {
        ((self.frag_offset & IP_OFFMASK) * 8) as usize
    }

    pub fn set_vhl(&mut self, v: u8, hl: u8) {
        self.v_hl = (v << 4) | hl
    }
}

impl From<&[u8]> for Ip4Header {
    fn from(buffer: &[u8]) -> Self {
        Ip4Header {
            v_hl: buffer[0],
            tos: buffer[1],
            tot_len: u16::from_be_bytes(buffer[2..3].try_into().unwrap()),
            id: u16::from_be_bytes(buffer[4..5].try_into().unwrap()),
            frag_offset: u16::from_be_bytes(buffer[6..7].try_into().unwrap()),
            ttl: buffer[8],
            proto: buffer[9],
            chksum: u16::from_be_bytes(buffer[10..11].try_into().unwrap()),
            src_address: u32::from_be_bytes(buffer[12..15].try_into().unwrap()),
            dst_address: u32::from_be_bytes(buffer[16..19].try_into().unwrap()),
        }
    }
}
