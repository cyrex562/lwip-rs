/*
 * @file
 *
 * IPv6 addresses.
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
 * Structs and macros for handling IPv6 addresses.
 *
 * Please coordinate changes and requests with Ivan Delamer
 * <delamer@inicotech.com>
 */

// #define LWIP_HDR_IP6_ADDR_H













/* This is the aligned version of ip6_addr_t,
    used as local variable, on the stack, etc. */
use crate::core::def_h::{lwip_htonl, LWIP_MAKEU32, PP_HTONL};

pub struct ip6_addr {
  pub addr: [u32;4],
  pub zone: u8,
}

/* IPv6 address */
// typedef struct ip6_addr ip6_addr_t;
type ip6_addr_t = ip6_addr;

/* Set an IPv6 partial address given by byte-parts */
pub fn IP6_ADDR_PART(ip6addr: &mut ip6_addr, index: usize, a:u8,b:u8,c:u8,d:u8) {
    ip6addr.addr[index] = PP_HTONL(LWIP_MAKEU32(a, b, c, d))
}

/* Set a full IPv6 address by passing the 4 u32 indices in network byte order
    (use PP_HTONL() for constants) */
pub fn IP6_ADDR(ip6addr: &mut ip6_addr, idx0: u32, idx1: u32, idx2: u32, idx3: u32) {
  ip6addr.addr[0] = idx0;
  ip6addr.addr[1] = idx1;
  ip6addr.addr[2] = idx2;
  ip6addr.addr[3] = idx3;
  ip6_addr_clear_zoneip6addr;
}

/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK1(ip6addr: &ip6_addr) -> u16 {(((lwip_htonl(ip6addr.addr[0]) >> 16) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK2(ip6addr: &ip6_addr) -> u16 { (((lwip_htonl(ip6addr.addr[0])) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK3(ip6addr: &ip6_addr) -> u16 { (((lwip_htonl(ip6addr.addr[1]) >> 16) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK4(ip6addr: &ip6_addr) -> u16 { (((lwip_htonl(ip6addr.addr[1])) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK5(ip6addr: &ip6_addr) -> u16 { (((lwip_htonl(ip6addr.addr[2]) >> 16) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK6(ip6addr: &ip6_addr) -> u16 {(((lwip_htonl(ip6addr.addr[2])) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK7(ip6addr: &ip6_addr) -> u16 { (((lwip_htonl(ip6addr.addr[3]) >> 16) & 0xffff))}
/* Access address in 16-bit block */
pub fn IP6_ADDR_BLOCK8(ip6addr: &ip6_addr) -> u16 { (((lwip_htonl(ip6addr.addr[3])) & 0xffff))}

/* Copy IPv6 address - faster than ip6_addr_set: no NULL check */
pub fn ip6_addr_copy(dest: &mut ip6_addr, src: &ip6_addr) {
    dest.addr[0] = src.addr[0];
    dest.addr[1] = src.addr[1];
    dest.addr[2] = src.addr[2];
    dest.addr[3] = src.addr[3];
    ip6_addr_copy_zone((dest), (src));
}
/* Safely copy one IPv6 address to another (src may be NULL) */
pub fn ip6_addr_set(dest: &mut ip6_addr, src: &ip6_addr) {
    // (dest) -> addr[0] = (src) == NULL?
    // 0: (src) -> addr[0];
    // (dest) -> addr[1] = (src) == NULL?
    // 0: (src) -> addr[1];
    // (dest) -> addr[2] = (src) == NULL?
    // 0: (src) -> addr[2];
    // (dest) -> addr[3] = (src) == NULL?
    // 0: (src) -> addr[3];
    // ip6_addr_set_zone((dest), (src) == NULL? IP6_NO_ZONE: ip6_addr_zone(src));
    ip6_addr_copy(dest, src)
}

/* Copy packed IPv6 address to unpacked IPv6 address; zone is not set */
pub fn ip6_addr_copy_from_packed(dest: &mut ip6_addr, src: &mut ip6_addr) {
    dest.addr[0] = src.addr[0];
    dest.addr[1] = src.addr[1];
    dest.addr[2] = src.addr[2];
    dest.addr[3] = src.addr[3];
    ip6_addr_clear_zone(&dest);
}

/* Copy unpacked IPv6 address to packed IPv6 address; zone is lost */
pub fn ip6_addr_copy_to_packed(dest: &mut ip6_addr, src: &ip6_addr) {
    dest.addr[0] = src.addr[0];
    dest.addr[1] = src.addr[1];
    dest.addr[2] = src.addr[2];
    dest.addr[3] = src.addr[3];
}

/* Set complete address to zero */
pub fn ip6_addr_set_zero(ip6addr: &mut ip6_addr) {ip6addr.addr[0] = 0;
                                         ip6addr.addr[1] = 0;
                                         ip6addr.addr[2] = 0;
                                         ip6addr.addr[3] = 0;
                                         ip6_addr_clear_zoneip6addr;
}

/* Set address to ipv6 'any' (no need for lwip_htonl()) */
pub fn ip6_addr_set_any(ip6addr: &mut ip6_addr) { ip6_addr_set_zeroip6addr }
/* Set address to ipv6 loopback address */
pub fn ip6_addr_set_loopback(ip6addr: &mut ip6_addr) {
    ip6addr.addr[0] = 0;
    ip6addr.addr[1] = 0;
    ip6addr.addr[2] = 0;
    ip6addr.addr[3] = PP_HTONL(0x00000001);
    ip6_addr_clear_zoneip6addr;
}
/* Safely copy one IPv6 address to another and change byte order
 * from host- to network-order. */
pub fn ip6_addr_set_hton(dest: &mut ip6_addr, src: &ip6_addr) {(dest).addr[0] = lwip_htonl((src).addr[0]);
                                        (dest).addr[1] =  lwip_htonl((src).addr[1]);
                                        (dest).addr[2] =  lwip_htonl((src).addr[2]);
                                        (dest).addr[3] =  lwip_htonl((src).addr[3]);
                                        // TODO: ip6_addr_set_zone((dest), (src)) = ip6_addr_zone(src));
}


/* Compare IPv6 networks, ignoring zone information. To be used sparingly! */
pub fn ip6_addr_netcmp_zoneless(addr1: &ip6_addr, addr2: &ip6_addr) -> bool {
    (((addr1).addr[0] == (addr2).addr[0]) &&
    ((addr1).addr[1] == (addr2).addr[1]))
}

/*
 * Determine if two IPv6 address are on the same network.
 *
 * @param addr1 IPv6 address 1
 * @param addr2 IPv6 address 2
 * @return 1 if the network identifiers of both address match, 0 if not
 */
pub fn ip6_addr_netcmp(addr1: &ip6_addr, addr2: &ip6_addr) -> bool {
    (ip6_addr_netcmp_zoneless((addr1), (addr2)) &&
    ip6_addr_cmp_zone((addr1), (addr2)))
}

/* Exact-host comparison *after* ip6_addr_netcmp() succeeded, for efficiency. */
pub fn ip6_addr_nethostcmp(addr1: &ip6_addr, addr2: &ip6_addr) -> bool {
    (addr1.addr[2] == (addr2).addr[2]) &&
    ((addr1).addr[3] == (addr2).addr[3])
}

/* Compare IPv6 addresses, ignoring zone information. To be used sparingly! */
pub fn ip6_addr_cmp_zoneless(addr1: &ip6_addr, addr2: &ip6_addr) -> bool {
    (((addr1).addr[0] == (addr2).addr[0]) &&
    ((addr1).addr[1] == (addr2).addr[1]) &&
    ((addr1).addr[2] == (addr2).addr[2]) &&
    ((addr1).addr[3] == (addr2).addr[3]))
}
/*
 * Determine if two IPv6 addresses are the same. In particular, the address
 * part of both must be the same, and the zone must be compatible.
 *
 * @param addr1 IPv6 address 1
 * @param addr2 IPv6 address 2
 * @return 1 if the addresses are considered equal, 0 if not
 */
pub fn ip6_addr_cmp(addr1: &ip6_addr, addr2: &ip6_addr) -> bool {
    (ip6_addr_cmp_zoneless((addr1), (addr2)) &&
    ip6_addr_cmp_zone((addr1), (addr2)))
}

/* Compare IPv6 address to packed address and zone */
pub fn ip6_addr_cmp_packed(ip6addr: &ip6_addr, paddr: &ip6_addr, zone_idx: u32) -> bool {
    ((ip6addr.addr[0] == (paddr).addr[0]) &&
    (ip6addr.addr[1] == (paddr).addr[1]) &&
    (ip6addr.addr[2] == (paddr).addr[2]) &&
    (ip6addr.addr[3] == (paddr).addr[3]) &&
    ip6_addr_equals_zone(ip6addr, (zone_idx)))
}

pub fn ip6_get_subnet_id(ip6addr: &ip6_addr) -> u32 { (lwip_htonl(ip6addr.addr[2]) & 0x0000ffff) }

pub fn ip6_addr_isany_val(ip6addr: &ip6_addr) -> bool {
    ((ip6addr.addr[0] == 0) &&
    (ip6addr.addr[1] == 0) &&
    (ip6addr.addr[2] == 0) &&
    (ip6addr.addr[3] == 0))
}
pub fn ip6_addr_isany(ip6addr: &ip6_addr) -> bool { ip6_addr_isany_val(ip6addr) }

pub fn ip6_addr_isloopback(ip6addr: &ip6_addr) -> bool {
    ((ip6addr.addr[0] == 0) &&
    (ip6addr.addr[1] == 0) &&
    (ip6addr.addr[2] == 0) &&
    (ip6addr.addr[3] == PP_HTONL(0x00000001)))
}

pub fn ip6_addr_isglobal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xe0000000)) == PP_HTONL(0x20000000)) }

pub fn ip6_addr_islinklocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xffc00000)) == PP_HTONL(0xfe800000)) }

pub fn ip6_addr_issitelocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xffc00000)) == PP_HTONL(0xfec00000)) }

pub fn ip6_addr_isuniquelocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xfe000000)) == PP_HTONL(0xfc000000)) }

// #define ip6_addr_isipv4mappedipv6(ip6addr) (((ip6addr)->addr[0] == 0) && ((ip6addr)->addr[1] == 0) && (((ip6addr)->addr[2]) == PP_HTONL(0x0000FFFFUL)))
pub fn ip6_addr_isipv4mappedipv6(ip6addr: &ip6_addr_t) -> bool {
    ((ip6addr.addr[0] == 0) && (ip6addr.addr[1] == 0) && ((ip6addr.addr[2]) == PP_HTONL(0x0000FFFF)))
}

pub fn ip6_addr_ismulticast(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xff000000)) == PP_HTONL(0xff000000)) }
pub fn ip6_addr_multicast_transient_flag(ip6addr: &ip6_addr) -> u32 { (ip6addr.addr[0] & PP_HTONL(0x00100000)) }
pub fn ip6_addr_multicast_prefix_flag(ip6addr: &ip6_addr) -> u32 { (ip6addr.addr[0] & PP_HTONL(0x00200000)) }
pub fn ip6_addr_multicast_rendezvous_flag(ip6addr: &ip6_addr) -> u32 { (ip6addr.addr[0] & PP_HTONL(0x00400000)) }
pub fn ip6_addr_multicast_scope(ip6addr: &ip6_addr) -> u32 { ((lwip_htonl(ip6addr.addr[0]) >> 16) & 0xf) }
pub const IP6_MULTICAST_SCOPE_RESERVED: u32 = 0x0;
pub const IP6_MULTICAST_SCOPE_RESERVED0 : u32 =           0x0;
pub const IP6_MULTICAST_SCOPE_INTERFACE_LOCAL: u32 =    0x1;
pub const IP6_MULTICAST_SCOPE_LINK_LOCAL: u32        =  0x2;
pub const IP6_MULTICAST_SCOPE_RESERVED3 : u32 =          0x3;
pub const IP6_MULTICAST_SCOPE_ADMIN_LOCAL: u32 =          0x4;
pub const IP6_MULTICAST_SCOPE_SITE_LOCAL: u32 =         0x5;
pub const IP6_MULTICAST_SCOPE_ORGANIZATION_LOCAL: u32 =  0x8;
pub const IP6_MULTICAST_SCOPE_GLOBAL: u32 =              0xe;
pub const IP6_MULTICAST_SCOPE_RESERVEDF: u32 =           0xf;
pub fn ip6_addr_ismulticast_iflocal(ip6addr: &ip6_addr) -> bool{ ((ip6addr.addr[0] & PP_HTONL(0xff8f0000)) == PP_HTONL(0xff010000))}
pub fn ip6_addr_ismulticast_linklocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xff8f0000)) == PP_HTONL(0xff020000)) }
pub fn ip6_addr_ismulticast_adminlocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xff8f0000)) == PP_HTONL(0xff040000)) }
pub fn ip6_addr_ismulticast_sitelocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xff8f0000)) == PP_HTONL(0xff050000)) }
pub fn ip6_addr_ismulticast_orglocal(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xff8f0000)) == PP_HTONL(0xff080000)) }
pub fn ip6_addr_ismulticast_global(ip6addr: &ip6_addr) -> bool { ((ip6addr.addr[0] & PP_HTONL(0xff8f0000)) == PP_HTONL(0xff0e0000)) }

/* Scoping note: while interface-local and link-local multicast addresses do
 * have a scope (i.e., they are meaningful only in the context of a particular
 * interface), the following functions are not assigning or comparing zone
 * indices. The reason for this is backward compatibility. Any call site that
 * produces a non-global multicast address must assign a multicast address as
 * appropriate itself. */

pub fn ip6_addr_isallnodes_iflocal(ip6addr: &ip6_addr) -> bool {
    ((ip6addr.addr[0] == PP_HTONL(0xff010000)) &&
    (ip6addr.addr[1] == 0) &&
    (ip6addr.addr[2] == 0) &&
    (ip6addr.addr[3] == PP_HTONL(0x00000001)))
}

pub fn ip6_addr_isallnodes_linklocal(ip6addr: &ip6_addr) -> bool {
    (ip6addr.addr[0] == PP_HTONL(0xff020000)) &&
    (ip6addr.addr[1] == 0) &&
    (ip6addr.addr[2] == 0) &&
    (ip6addr.addr[3] == PP_HTONL(0x00000001))}


pub fn ip6_addr_set_allnodes_linklocal(ip6addr: &mut ip6_addr) {ip6addr.addr[0] = PP_HTONL(0xff020000);
                ip6addr.addr[1] = 0;
                ip6addr.addr[2] = 0;
                ip6addr.addr[3] = PP_HTONL(0x00000001);
                ip6_addr_clear_zone(ip6addr); }

pub fn ip6_addr_isallrouters_linklocal(ip6addr: &ip6_addr) -> bool {
    ((ip6addr.addr[0] == PP_HTONL(0xff020000)) &&
    (ip6addr.addr[1] == 0) &&
    (ip6addr.addr[2] == 0) &&
    (ip6addr.addr[3] == PP_HTONL(0x00000002)))
}
pub fn ip6_addr_set_allrouters_linklocal(ip6addr: &mut ip6_addr)  {ip6addr.addr[0] = PP_HTONL(0xff020000);
                ip6addr.addr[1] = 0;
                ip6addr.addr[2] = 0;
                ip6addr.addr[3] = PP_HTONL(0x00000002);
                ip6_addr_clear_zone(ip6addr); }

pub fn ip6_addr_issolicitednode(ip6addr: &ip6_addr) -> bool {
    ((ip6addr.addr[0] == PP_HTONL(0xff020000)) &&
    (ip6addr.addr[2] == PP_HTONL(0x00000001)) &&
    ((ip6addr.addr[3] & PP_HTONL(0xff000000)) == PP_HTONL(0xff000000)) )
}

pub fn ip6_addr_set_solicitednode(ip6addr: &mut ip6_addr, if_id: u32) {ip6addr.addr[0] = PP_HTONL(0xff020000);
                ip6addr.addr[1] = 0;
                ip6addr.addr[2] = PP_HTONL(0x00000001);
                ip6addr.addr[3] = (PP_HTONL(0xff000000) | (if_id));
                ip6_addr_clear_zone(ip6addr); }

pub fn ip6_addr_cmp_solicitednode(ip6addr: &ip6_addr, sn_addr: &ip6_addr) -> bool {
    ((ip6addr.addr[0] == PP_HTONL(0xff020000)) &&
    (ip6addr.addr[1] == 0) &&
    (ip6addr.addr[2] == PP_HTONL(0x00000001)) &&
    (ip6addr.addr[3] == (PP_HTONL(0xff000000) | (sn_addr).addr[3])))
}

/* IPv6 address states. */
pub const IP6_ADDR_INVALID: u8 = 0x00;
pub const IP6_ADDR_TENTATIVE: u8 =    0x08;
pub const IP6_ADDR_TENTATIVE_1: u8 =  0x09; /* 1 probe sent */
pub const IP6_ADDR_TENTATIVE_2: u8 = 0x0a; /* 2 probes sent */
pub const IP6_ADDR_TENTATIVE_3: u8 = 0x0b; /* 3 probes sent */
pub const IP6_ADDR_TENTATIVE_4: u8 =  0x0c; /* 4 probes sent */
pub const IP6_ADDR_TENTATIVE_5: u8 =  0x0d; /* 5 probes sent */
pub const IP6_ADDR_TENTATIVE_6: u8 =  0x0e; /* 6 probes sent */
pub const IP6_ADDR_TENTATIVE_7: u8 =  0x0f; /* 7 probes sent */
pub const IP6_ADDR_VALID: u8 =        0x10; /* This bit marks an address as valid (preferred or deprecated) */
pub const IP6_ADDR_PREFERRED: u8 =    0x30;
pub const IP6_ADDR_DEPRECATED: u8 =   0x10; /* Same as VALID (valid but not preferred) */
pub const IP6_ADDR_DUPLICATED: u8 =   0x40; /* Failed DAD test, not valid */
pub const IP6_ADDR_TENTATIVE_COUNT_MASK: u32 = 0x07; /* 1-7 probes sent */

// #define ip6_addr_isinvalid(addr_state) (addr_state == IP6_ADDR_INVALID)
pub fn ip6_addr_isinvalid(addr_state: u8) -> bool {
    addr_state == IP6_ADDR_INVALID
}
// #define ip6_addr_istentative(addr_state) (addr_state & IP6_ADDR_TENTATIVE)
pub fn ip6_addr_istentative(addr_state: u8) -> bool {
    addr_state & IP6_ADDR_TENTATIVE > 0
}
// #define ip6_addr_isvalid(addr_state) (addr_state & IP6_ADDR_VALID) /* Include valid, preferred, and deprecated. */
pub fn ip6_addr_isvalid(addr_state: u8) -> bool {
    addr_state & IP6_ADDR_VALID > 0
}
// #define ip6_addr_ispreferred(addr_state) (addr_state == IP6_ADDR_PREFERRED)
pub fn ip6_addr_ispreferred(addr_state: u8) -> bool {
    addr_state & IP6_ADDR_PREFERRED > 0
}
// #define ip6_addr_isdeprecated(addr_state) (addr_state == IP6_ADDR_DEPRECATED)
pub fn ip6_addr_isdeprecated(addr_state: u8) -> bool {
    addr_state == IP6_ADDR_DEPRECATED
}
// #define ip6_addr_isduplicated(addr_state) (addr_state == IP6_ADDR_DUPLICATED)


pub const IP6_ADDR_LIFE_STATIC: u32 =   (0);
pub const IP6_ADDR_LIFE_INFINITE: u32 =  (0xffffffff);
pub fn ip6_addr_life_isstatic(addr_life: u32) -> bool {  ((addr_life) == IP6_ADDR_LIFE_STATIC) }
pub fn ip6_addr_life_isinfinite(addr_life: u32 ) -> bool { ((addr_life) == IP6_ADDR_LIFE_INFINITE) }


// #define ip6_addr_debug_print_parts(debug, a, b, c, d, e, f, g, h) \
//   LWIP_DEBUGF(debug, ("%" X16_F ":%" X16_F ":%" X16_F ":%" X16_F ":%" X16_F ":%" X16_F ":%" X16_F ":%" X16_F, \
//                       a, b, c, d, e, f, g, h))
// #define ip6_addr_debug_print(debug, ipaddr) \
//   ip6_addr_debug_print_parts(debug, \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK1(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK2(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK3(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK4(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK5(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK6(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK7(ipaddr) : 0),    \
//                       ((ipaddr) != NULL ? IP6_ADDR_BLOCK8(ipaddr) : 0))
// #define ip6_addr_debug_print_val(debug, ipaddr) \
//   ip6_addr_debug_print_parts(debug, \
//                       IP6_ADDR_BLOCK1(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK2(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK3(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK4(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK5(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK6(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK7(&(ipaddr)),    \
//                       IP6_ADDR_BLOCK8(&(ipaddr)))

pub const IP6ADDR_STRLEN_MAX: usize =    46;

// ip6addr_aton: int(const char *cp, addr: &mut ip6_addr_t);
/* returns ptr to static buffer; not reentrant! */
// char *ip6addr_ntoa(const addr: &mut ip6_addr_t);
// char *ip6addr_ntoa_r(const addr: &mut ip6_addr_t, char *buf, buflen: int);





