/*
 * @file
 * IPv4 address API
 */
#![allow(non_snake_case)]

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

// // #define LWIP_HDR_IP4_ADDR_H

//

//

/* This is the aligned version of ip4_addr,
used as local variable, on the stack, etc. */
use crate::core::def_h::{lwip_htonl, PP_HTONL};

pub struct ip4_addr {
    pub addr: u32,
}

/* Forward declaration to not include netif.h */
// NetIfc;

/* 255.255.255.255 */
// #define IPADDR_NONE         (0xffffffff)
pub const IPADDR_NONE: u32 = 0xffffffff;
/* 127.0.0.1 */
// #define IPADDR_LOOPBACK     (0x7f000001)
pub const IPADDR_LOOPBACK: u32 = 0x7f000001;
/* 0.0.0.0 */
// #define IPADDR_ANY          (0x00000000)
pub const IPADDR_ANY: u32 = 0x00000000;
/* 255.255.255.255 */
// #define IPADDR_BROADCAST    (0xffffffff)
pub const IPADDR_BROADCAST: u32 = 0xffffffff;
/* Definitions of the bits in an Internet address integer.

On subnets, host and network parts are found according to
the subnet mask, not these masks.  */
// #define IP_CLASSA(a)        ((((a)) & 0x80000000) == 0)
pub fn IP_CLASSA(a: u32) -> bool {
    a & 0x80000000 == 0
}

pub const IP_CLASSA_NET: u32 = 0xff000000;
// #define IP_CLASSA_NSHIFT    24
pub const IP_CLASSA_NSHIFT: u32 = 24;
// #define IP_CLASSA_HOST      (0xffffffff & !IP_CLASSA_NET)
pub const IP_CLASSA_HOST: u32 = 0xffffffff & !IP_CLASSA_NET;
// #define IP_CLASSA_MAX       128
pub const IP_CLASSA_MAX: u32 = 128;

// #define IP_CLASSB(a)        ((((a)) & 0xc0000000) == 0x80000000)
pub fn IP_CLASSB(a: u32) -> bool {
    a & c0000000 == 0x80000000
}

pub const IP_CLASSB_NET: u32 = 0xffff0000;
// #define IP_CLASSB_NSHIFT    16
pub const IP_CLASSB_NSHIFT: u32 = 16;
// #define IP_CLASSB_HOST      (0xffffffff & !IP_CLASSB_NET)
pub const IP_CLASSB_HOST: u32 = 0xffffffff & !IP_CLASSB_NET;
// #define IP_CLASSB_MAX       65536
pub const IP_CLASSB_MAX: u32 = 65536;

// #define IP_CLASSC(a)        ((((a)) & 0xe0000000) == 0xc0000000)
pub fn IP_CLASSC(a: u32) -> bool {
    a & 0xe0000000 == 0xc0000000
}
pub const IP_CLASSC_NET: u32 = 0xffffff00;
// #define IP_CLASSC_NSHIFT    8
pub const IP_CLASSC_NSHIFT: u32 = 8;
// #define IP_CLASSC_HOST      (0xffffffff & !IP_CLASSC_NET)
pub const IP_CLASSC_HOST: u32 = 0xffffffff & !IP_CLASSC_NET;

// #define IP_CLASSD(a)        (((a) & 0xf0000000) == 0xe0000000)
pub fn IP_CLASSD(a: u32) -> bool {
    a & f0000000 == 0xe0000000
}
pub const IP_CLASSD_NET: u32 = 0xf0000000; /* These ones aren't really */
// #define IP_CLASSD_NSHIFT    28                  /*   net and host fields, but */
pub const IP_CLASSD_NSHIFT: u32 = 28;
pub const IP_CLASSD_HOST: u32 = 0x0fffffff; /*   routing needn't know. */
// #define IP_MULTICAST(a)     IP_CLASSD(a)
pub fn IP_MULTICAST(a: u32) -> bool {
    IP_CLASSD(a)
}

// #define IP_EXPERIMENTAL(a)  (((a) & 0xf0000000) == 0xf0000000)
pub fn IP_EXPERIMENTAL(a: u32) -> bool {
    a & 0xf0000000 == 0xf0000000
}
// #define IP_BADCLASS(a)      (((a) & 0xf0000000) == 0xf0000000)
pub fn IP_BADCLASS(a: u32) -> bool {}

// #define IP_LOOPBACKNET      127                 /* official! */
pub const IP_LOOPBACKNET: u32 = 127;

/* Set an IP address given by the four byte-parts */
// #define IP4_ADDR(ipaddr, a,b,c,d)  (ipaddr).addr = PP_HTONL(LWIP_MAKEU32(a,b,c,d))
pub fn IP4_ADDR(ipaddr: &mut ip4_addr, a: u8, b: u8, c: u8, d: u8) {
    ipaddr.addr = PP_HTONL(LWIP_MAKEu32(a, b, c, d))
}

/* Copy IP address - faster than ip4_addr_set: no NULL check */
// #define ip4_addr_copy(dest, src) ((dest).addr = (src).addr)
pub fn ip4_addr_copy(dest: &mut ip4_addr, src: &mut ip4_addr) {
    dest.addr = src.addr
}

/* Safely copy one IP address to another (src may be NULL) */
// #define ip4_addr_set(dest, src) ((dest).addr = \
//                                     ((src) == NULL ? 0 : \
//                                     (src).addr))
pub fn ip4_addr_set(dest: &mut ip4_addr, src: &mut ip4_addr) {
    dest.addr = src.addr
}
/* Set complete address to zero */
// #define ip4_addr_set_zero(ipaddr)     ((ipaddr).addr = 0)
pub fn ip4_addr_set_zero(ipaddr: &mut ip4_addr) {
    ipaddr.addr = 0
}
/* Set address to IPADDR_ANY (no need for lwip_htonl()) */
// #define ip4_addr_set_any(ipaddr)      ((ipaddr).addr = IPADDR_ANY)
pub fn ip4_addr_set_any(ipaddr: &mut ip4_addr) {
    ipaddr.addr = IPADDR_ANY
}

/* Set address to loopback address */
// #define ip4_addr_set_loopback(ipaddr) ((ipaddr).addr = PP_HTONL(IPADDR_LOOPBACK))
pub fn ip4_addr_set_loopback(ipaddr: &mut ip4_addr) {
    ipaddr.addr = PP_HTONL(IPADDR_LOOPBACK)
}
/* Check if an address is in the loopback region */
// #define ip4_addr_isloopback(ipaddr)    (((ipaddr).addr & PP_HTONL(IP_CLASSA_NET)) == PP_HTONL((IP_LOOPBACKNET) << 24))
pub fn ip4_addr_isloopback(ipaddr: &mut ip4_addr) -> bool {
    (ipaddr.addr & PP_HTONL(IP_CLASSA_NET)) == PP_HTONL(IP_LOOPBACKNET) << 24
}
/* Safely copy one IP address to another and change byte order
 * from host- to network-order. */
// #define ip4_addr_set_hton(dest, src) ((dest).addr = \
//                                ((src) == NULL ? 0:\
//                                lwip_htonl((src).addr)))
pub fn ip4_addr_set_hton(dest: &mut ip4_addr, src: &mut ip4_addr) {
    dest.addr = src.addr;
    dest.addr = lwip_htonl(src.addr);
}
/* IPv4 only: set the IP address given as an u32 */
// #define ip4_addr_set_u32(dest_ipaddr, src_u32) ((dest_ipaddr).addr = (src_u32))
pub fn ip4_addr_set_u32(dest: &mut ip4_addr, src: u32) {
    dest.addr = src;
}

/* IPv4 only: get the IP address as an u32 */
// #define ip4_addr_get_u32(src_ipaddr) ((src_ipaddr).addr)
pub fn ip4_addr_get_u32(ip_addr: &mut ip4_addr) -> u32 {
    ip_addr.addr
}

/* Get the network address by combining host address with netmask */
// #define ip4_addr_get_network(target, host, netmask) loop { ((target).addr = ((host).addr) & ((netmask).addr)); } while(0)
pub fn ip4_addr_get_network(target: &mut ip4_addr, host: &mut ip4_addr, netmask: &mut ip4_addr) {
    target.addr = host.addr & netmask.addr
}

/*
 * Determine if two address are on the same network.
 *
 * @arg addr1 IP address 1
 * @arg addr2 IP address 2
 * @arg mask network identifier mask
 * @return !0 if the network identifiers of both address match
 */
// #define ip4_addr_netcmp(addr1, addr2, mask) (((addr1).addr & \
//                                               (mask).addr) == \
//                                              ((addr2).addr & \
//                                               (mask).addr))
pub fn ip4_addr_netcmp(addr1: &mut ip4_addr, addr2: &mut ip4_addr, mask: &mut ip4_addr) -> bool {
    (addr1.addr & mask.addr) == (addr2.addr & mask.addr)
}

// #define ip4_addr_cmp(addr1, addr2) ((addr1).addr == (addr2).addr)
pub fn ip4_addr_cmp(addr1: &mut ip4_addr, addr2: &mut ip4_addr) -> bool {
    addr1.addr == addr2.addr
}

// #define ip4_addr_isany_val(addr1)   ((addr1).addr == IPADDR_ANY)
pub fn ip4_addr_isany_val(addr1: &mut ip4_addr) -> bool {
    addr1.addr == IPADDR_ANY
}

// #define ip4_addr_isany(addr1) ((addr1) == NULL || ip4_addr_isany_val(*(addr1)))
pub fn ip4_addr_isany(addr1: &mut ip4_addr) -> bool {
    ip4_addr_is_any_val(addr1)
}

// #define ip4_addr_isbroadcast(addr1, netif) ip4_addr_isbroadcast_u32((addr1).addr, netif)
// ip4_addr_isbroadcast_u32: u8(u32 addr,  netif: &mut NetIfc);

// #define ip_addr_netmask_valid(netmask) ip4_addr_netmask_valid((netmask).addr)
// ip4_addr_netmask_valid: u8(u32 netmask);

// #define ip4_addr_ismulticast(addr1) (((addr1).addr & PP_HTONL(0xf0000000)) == PP_HTONL(0xe0000000))
pub fn ip4_addr_ismulticast(addr1: &mut ip4_addr) -> bool {
    (addr1.addr & PP_HTONL(0xf0000000)) == PP_HTONL(0xe0000000)
}

// #define ip4_addr_islinklocal(addr1) (((addr1).addr & PP_HTONL(0xffff0000)) == PP_HTONL(0xa9fe0000))
pub fn ip4_addr_islinklocal(addr1: &mut ip4_addr) -> bool {
    addr1.addr & PP_HTONL(0xffff0000) == PP_HTONL(0xa9fe0000)
}

// TODO: #define ip4_addr_debug_print_parts(debug, a, b, c, d) \
//   LWIP_DEBUGF(debug, ("%" U16_F ".%" U16_F ".%" U16_F ".%" U16_F, a, b, c, d))

// #define ip4_addr_debug_print(debug, ipaddr) \
//   ip4_addr_debug_print_parts(debug, \
//                       ((ipaddr) != NULL ? ip4_addr1_16(ipaddr) : 0),       \
//                       ((ipaddr) != NULL ? ip4_addr2_16(ipaddr) : 0),       \
//                       ((ipaddr) != NULL ? ip4_addr3_16(ipaddr) : 0),       \
//                       ((ipaddr) != NULL ? ip4_addr4_16(ipaddr) : 0))
// #define ip4_addr_debug_print_val(debug, ipaddr) \
//   ip4_addr_debug_print_parts(debug, \
//                       ip4_addr1_16_val(ipaddr),       \
//                       ip4_addr2_16_val(ipaddr),       \
//                       ip4_addr3_16_val(ipaddr),       \
//                       ip4_addr4_16_val(ipaddr))

/* Get one byte from the 4-byte address */
// #define ip4_addr_get_byte(ipaddr, idx) (((const u8*)(&(ipaddr).addr))[idx])
pub fn ip4_addr_get_byte(ipaddr: &mut ip4_addr, idx: u32) -> u8 {
    ipaddr[idx]
}
// #define ip4_addr1(ipaddr) ip4_addr_get_byte(ipaddr, 0)
pub fn ip4_addr1(ipaddr: &mut ip4_addr) -> u8 {
    ip4_addr_get_byte(ipaddr, 0)
}
// #define ip4_addr2(ipaddr) ip4_addr_get_byte(ipaddr, 1)
pub fn ip4_addr2(ipaddr: &mut ip4_addr) -> u8 {
    ip4_addr_get_byte(ipaddr, 1)
}
// #define ip4_addr3(ipaddr) ip4_addr_get_byte(ipaddr, 2)
pub fn ip4_addr3(ipaddr: &mut ip4_addr) -> u8 {
    ip4_addr_get_byte(ipaddr, 2)
}
// #define ip4_addr4(ipaddr) ip4_addr_get_byte(ipaddr, 3)
pub fn ip4_addr4(ipaddr: &mut ip4_addr) -> u8 {
    ip4_addr_get_byte(ipaddr, 3)
}

/* Get one byte from the 4-byte address, but argument is 'ip4_addr',
 * not a pointer */
// #define ip4_addr_get_byte_val(ipaddr, idx) ((((ipaddr).addr >> (idx * 8)) & 0xff))
pub fn ip4_addr_get_byte_val(ipaddr: &ip4_addr, idx: u8) -> u8 {
    ((ipaddr.addr >> (idx * 8)) & 0xff) as u8
}
// #define ip4_addr1_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 0)
pub fn ip4_addr1_val(ipaddr: &ip4_addr) -> u8 {
    ip4_addr_get_byte_val(ipaddr, 0)
}
// #define ip4_addr2_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 1)
pub fn ip4_addr2_val(ipaddr: &ip4_addr) -> u8 {
    ip4_addr_get_byte_val(ipaddr, 1)
}
// #define ip4_addr3_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 2)
pub fn ip4_addr3_val(ipaddr: &ip4_addr) -> u8 {
    ip4_addr_get_byte_val(ipaddr, 2)
}
// #define ip4_addr4_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 3)
pub fn ip4_addr4_val(ipaddr: &ip4_addr) -> u8 {
    ip4_addr_get_byte_val(ipaddr, 3)
}

/* These are cast to u16, with the intent that they are often arguments
 * to printf using the U16_F format from cc.h. */
// #define ip4_addr1_16(ipaddr) (ip4_addr1(ipaddr))
// #define ip4_addr2_16(ipaddr) (ip4_addr2(ipaddr))
// #define ip4_addr3_16(ipaddr) (ip4_addr3(ipaddr))
// #define ip4_addr4_16(ipaddr) (ip4_addr4(ipaddr))
// #define ip4_addr1_16_val(ipaddr) (ip4_addr1_val(ipaddr))
// #define ip4_addr2_16_val(ipaddr) (ip4_addr2_val(ipaddr))
// #define ip4_addr3_16_val(ipaddr) (ip4_addr3_val(ipaddr))
// #define ip4_addr4_16_val(ipaddr) (ip4_addr4_val(ipaddr))

// #define IP4ADDR_STRLEN_MAX  16
pub const IP4_ADDR_STRLEN_MAX: u32 = 16;

/* For backwards compatibility */
// #define ip_ntoa(ipaddr)  ipaddr_ntoa(ipaddr)
type ip_ntoa = ipaddr_ntoa;

// u32 ipaddr_addr(cp: &String);
// ip4addr_aton: i32(cp: &String, addr: &mut ip4_addr);
/* returns ptr to static buffer; not reentrant! */
// ip4addr_ntoa: &mut String(const addr: &mut ip4_addr);
// ip4addr_ntoa_r: &mut String(const addr: &mut ip4_addr, buf: &mut String, buflen: i32);

// }
