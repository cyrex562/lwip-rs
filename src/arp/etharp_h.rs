use crate::ip::ip4_addr;

/*
 * @file
 * ARP protocol definitions
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

// #define LWIP_HDR_PROT_ETHARP_H

pub const ETHARP_HWADDR_LEN: usize = ETH_HWADDR_LEN;

/*
 * struct ip4_addr_wordaligned is used in the definition of the ARP packet format in
 * order to support compilers that don't have structure packing.
 */

pub struct ip4_addr_wordaligned {
    pub addrw: [u16; 2],
}

/* MEMCPY-like copying of IP addresses where addresses are known to be
 * 16-bit-aligned if the port is correctly configured (so a port could define
 * this to copying 2 u16's) - no NULL-pointer-checking needed. */

pub fn IPADDR_WORDALIGNED_COPY_TO_ip4_addr(dest: ip4_addr, src: ip4_addr_wordaligned) {
    SMEMCPY(dest, src, sizeof(ip4_addr))
}

/* MEMCPY-like copying of IP addresses where addresses are known to be
 * 16-bit-aligned if the port is correctly configured (so a port could define
 * this to copying 2 u16's) - no NULL-pointer-checking needed. */

pub fn IPADDR_WORDALIGNED_COPY_FROM_ip4_addr(dest: ip4_addr_wordaligned, src: ip4_addr) {
    SMEMCPY(dest, src, sizeof(ip4_addr))
}
/* the ARP message, see RFC 826 ("Packet format") */
pub struct etharp_hdr {
    pub hwtype: u16,
    pub proto: u16,
    pub hwlen: u8,
    pub protolen: u8,
    pub opcode: u16,
    pub shwaddr: eth_addr,
    pub sipaddr: ip4_addr_wordaligned,
    pub dhwaddr: eth_addr,
    pub dipaddr: ip4_addr_wordaligned,
}

pub const SIZEOF_ETHARP_HDR: usize = 28;

/* ARP message types (opcodes) */
pub enum etharp_opcode {
    ARP_REQUEST = 1,
    ARP_REPLY = 2,
}
