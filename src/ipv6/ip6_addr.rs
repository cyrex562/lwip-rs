// @file
// IPv6 addresses.

// /*
//  * Copyright (c) 2010 Inico Technologies Ltd.
//  * All rights reserved.
//  *
//  * Redistribution and use in source and binary forms, with or without modification,
//  * are permitted provided that the following conditions are met:
//  *
//  * 1. Redistributions of source code must retain the above copyright notice,
//  *    this list of conditions and the following disclaimer.
//  * 2. Redistributions in binary form must reproduce the above copyright notice,
//  *    this list of conditions and the following disclaimer in the documentation
//  *    and/or other materials provided with the distribution.
//  * 3. The name of the author may not be used to endorse or promote products
//  *    derived from this software without specific prior written permission.
//  *
//  * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
//  * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
//  * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
//  * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
//  * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
//  * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
//  * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
//  * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
//  * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
//  * OF SUCH DAMAGE.
//  *
//  * This file is part of the lwIP TCP/IP stack.
//  *
//  * Author: Ivan Delamer <delamer@inicotech.com>
//  *
//  * Functions for handling IPv6 addresses.
//  *
//  * Please coordinate changes and requests with Ivan Delamer
//  * <delamer@inicotech.com>
//  */

use crate::errors::{LwipError, LwipErrorCode};

#[derive(Default,Debug,Clone, PartialEq)]
pub struct Ipv6Address {
    pub raw: [u8;16],
    pub zone: u8,
}

/** Set an IPv6 partial address given by byte-parts */
// #define IP6_ADDR_PART(ip6addr, index, a,b,c,d) \
//   (ip6addr)->addr[index] = PP_HTONL(LWIP_MAKEU32(a,b,c,d))

impl Ipv6Address {
    pub fn new() -> Self {
        Self {
            raw: [0;16],
            zone: 0,
        }
    }

    pub fn from_u32(a: u32, b: u32, c: u32, d: u32) -> Self {
        let a_bytes: [u8;4] = a.to_ne_bytes();
        let b_bytes: [u8;4] = b.to_ne_bytes();
        let c_bytes: [u8;4] = c.to_ne_bytes();
        let d_bytes: [u8;4] = d.to_ne_bytes();
        Self {
            raw: [a_bytes[0],a_bytes[1],a_bytes[2],a_bytes[3],b_bytes[0],b_bytes[1],b_bytes[2],b_bytes[3],c_bytes[0],c_bytes[1],c_bytes[2],c_bytes[3],d_bytes[0],d_bytes[1],d_bytes[2],d_bytes[3]],
            zone: 0
        }
    }

    fn get_u32_chunk(&self, index: usize) -> Result< u32, LwipError> {
        if index + 1 > 4 {
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "index must be <= 3"));
        }
        let byte_index = index * 4;
        let mut bytes: [u8;4] = [self.raw[byte_index], self.raw[byte_index+1], self.raw[byte_index+2], self.raw[byte_index+3]];
        Ok(u32::from_ne_bytes(bytes))
    }

    pub fn get_u32_chunk_0(&self) -> u32 {
        self.get_u32_chunk(0).unwrap()
    }

    pub fn get_u32_chunk_1(&self) -> u32 {
        self.get_u32_chunk(1).unwrap()
    }

    pub fn get_u32_chunk_2(&self) -> u32 {
        self.get_u32_chunk(2).unwrap()
    }

    pub fn get_u32_chunk_3(&self) -> u32 {
        self.get_u32_chunk(3).unwrap()
    }

    pub fn get_u16_chunk(&self, index: usize) -> Result<u16, LwipError> {
        if index + 1 > 15 {
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "index must be <= 15"));
        }
        let result: [u8;2] = [self.raw[index], self.raw[index+1]];
        Ok(u16::from_ne_bytes(result))
    }

    pub fn zero(&mut self) {
        self.raw = [0;u16];
        self.zone = 0;
    }

    pub fn zones_eq(&self, b: &Self) -> bool {
        self.zone == b.zone
    }

    pub fn eq_zoneless(&self, b: &Self) -> bool {
        self.raw == b.raw
    }

    pub fn net_eq_zoneless(&self, b: &Self) -> bool {
       let a_0 = self.get_u32_chunk_0();
        let b_0 = b.get_u32_chunk_0();
        let a_1 = self.get_u32_chunk(1).unwrap();
        let b_1 = self.get_u32_chunk(1).unwrap();
        a_0 == b_0 && a_1 == b_1
    }

    pub fn net_eq(&self, b: &Self) -> bool {
        self.net_eq_zoneless(b) && self.zones_eq(b)
    }

    pub fn nethost_eq(&self, b: &Self) -> bool {
        let a_2 = self.get_u32_chunk(2).unwrap();
        let a_3 = self.get_u32_chunk(3).unwrap();
        let b_2 = self.get_u32_chunk(2).unwrap();
        let b_3 = self.get_u32_chunk(3).unwrap();
        a_2 == b_2 && a_3 == b_3
    }

    pub fn addr_zone_eq(&self, b: &Self) -> bool {
        self.eq_zoneless(b) && self.zones_eq(b)
    }

    pub fn compare(&self, b: &Self) -> bool {
        self.addr_zone_eq(b)
    }

    pub fn get_subnet_id(&self) -> u32 {
        self.get_u32_chunk(2).unwrap() & 0x0000ffff
    }

    pub fn is_any(&self) -> bool {
        self.raw == IP6_ADDR_ANY.raw
    }

    pub fn is_loopback(&self) -> bool {
        self.raw == IP6_ADDR_LOOPBACK.raw
    }

    pub fn is_global(&self) -> bool {
        let a_0 = self.get_u32_chunk(0).unwrap();
        a_0 & 0xe0000000 == 0x20000000
    }

    pub fn is_link_local(&self) -> bool {
         let a_0 = self.get_u32_chunk(0).unwrap();
        a_0 & 0xffc00000 == 0xfe800000
    }

    pub fn is_site_local(&self) -> bool {
        let a_0 = self.get_u32_chunk(0).unwrap();
        a_0 & 0xffc00000 == 0xfec00000
    }

    pub fn is_unique_local(&self) -> bool {
        let a_0 = self.get_u32_chunk(0).unwrap();
        a_0 & 0xfe000000 == 0xfc000000
    }

    pub fn is_ipv4_mapped_ipv6(&self) -> bool {
        let a_0 = self.get_u32_chunk(0).unwrap();
        let a_1 = self.get_u32_chunk(1).unwrap();
        let a_2 = self.get_u32_chunk(2).unwrap();
        a_0 == 0 && a_1 == 0 && a_2 == 0x0000FFFF
    }

    pub fn is_ipv4_compatible(&self) -> bool {
        let a_0 = self.get_u32_chunk(0).unwrap();
        let a_1 = self.get_u32_chunk(1).unwrap();
        let a_2 = self.get_u32_chunk(2).unwrap();
        let a_3 = self.get_u32_chunk(3).unwrap();
        a_0 == 0 && a_1 == 0 && a_2 == 0 && a_3 > 1
    }

    pub fn is_multicast(&self) -> bool {
            let a_0 = self.get_u32_chunk(0).unwrap();
        a_0 & 0xff000000 == 0xff000000
    }
}

pub const IP6_ADDR_ANY: Ipv6Address = Ipv6Address::new_from_u32(0,0,0,0);
pub const IP6_ADDR_LOOPBACK: Ipv6Address = Ipv6Address::new_from_u32(0,0,0,1);


// TODO:
// #define lwip_xchar(i)        ((char)((i) < 10 ? '0' + (i) : 'A' + (i) - 10))

/**
 * Check whether "cp" is a valid ascii representation
 * of an IPv6 address and convert to a binary address.
 * Returns 1 if the address is valid, 0 if not.
 *
 * @param cp IPv6 address in ascii representation (e.g. "FF01::1")
 * @param addr pointer to which to save the ip address in network order
 * @return 1 if cp could be converted to addr, 0 on failure
 */
// int
// ip6addr_aton(const char *cp, ip6_addr_t *addr)
// {
//   u32_t addr_index, zero_blocks, current_block_index, current_block_value;
//   const char *s;
// // #if LWIP_IPV4
//   int check_ipv4_mapped = 0;
// // #endif /* LWIP_IPV4 */
//
//   /* Count the number of colons, to count the number of blocks in a "::" sequence
//      zero_blocks may be 1 even if there are no :: sequences */
//   zero_blocks = 8;
//   for (s = cp; *s != 0; s++) {
//     if (*s == ':') {
//       zero_blocks--;
// // #if LWIP_IPV4
//     } else if (*s == '.') {
//       if ((zero_blocks == 5) ||(zero_blocks == 2)) {
//         check_ipv4_mapped = 1;
//         /* last block could be the start of an IPv4 address */
//         zero_blocks--;
//       } else {
//         /* invalid format */
//         return 0;
//       }
//       break;
// // #endif /* LWIP_IPV4 */
//     } else if (!lwip_isxdigit(*s)) {
//       break;
//     }
//   }
//
//   /* parse each block */
//   addr_index = 0;
//   current_block_index = 0;
//   current_block_value = 0;
//   for (s = cp; *s != 0; s++) {
//     if (*s == ':') {
//       if (addr) {
//         if (current_block_index & 0x1) {
//            addr.addr[addr_index++] |= current_block_value;
//         }
//         else {
//            addr.addr[addr_index] = current_block_value << 16;
//         }
//       }
//       current_block_index++;
// // #if LWIP_IPV4
//       if (check_ipv4_mapped) {
//         if (current_block_index == 6) {
//          ip4: ip4_addr_t;
//           int ret = ip4addr_aton(s + 1, &ip4);
//           if (ret) {
//             if (addr) {
//                addr.addr[3] = lwip_htonl(ip4.addr);
//               current_block_index++;
//               goto fix_byte_order_and_return;
//             }
//             return 1;
//           }
//         }
//       }
// // #endif /* LWIP_IPV4 */
//       current_block_value = 0;
//       if (current_block_index > 7) {
//         /* address too long! */
//         return 0;
//       }
//       if (s[1] == ':') {
//         if (s[2] == ':') {
//           /* invalid format: three successive colons */
//           return 0;
//         }
//         s++;
//         /* "::" found, set zeros */
//         while (zero_blocks > 0) {
//           zero_blocks--;
//           if (current_block_index & 0x1) {
//             addr_index++;
//           } else {
//             if (addr) {
//                addr.addr[addr_index] = 0;
//             }
//           }
//           current_block_index++;
//           if (current_block_index > 7) {
//             /* address too long! */
//             return 0;
//           }
//         }
//       }
//     } else if (lwip_isxdigit(*s)) {
//       /* add current digit */
//       current_block_value = (current_block_value << 4) +
//           (lwip_isdigit(*s) ? (u32_t)(*s - '0') :
//           (u32_t)(10 + (lwip_islower(*s) ? *s - 'a' : *s - 'A')));
//     } else {
//       /* unexpected digit, space? CRLF? */
//       break;
//     }
//   }
//
//   if (addr) {
//     if (current_block_index & 0x1) {
//        addr.addr[addr_index++] |= current_block_value;
//     }
//     else {
//        addr.addr[addr_index] = current_block_value << 16;
//     }
// // #if LWIP_IPV4
// fix_byte_order_and_return:
// // #endif
//     /* convert to network byte order. */
//     for (addr_index = 0; addr_index < 4; addr_index++) {
//        addr.addr[addr_index] = lwip_htonl( addr.addr[addr_index]);
//     }
//
//     ip6_addr_clear_zone(addr);
// // #if LWIP_IPV6_SCOPES
//     if (*s == '%') {
//       const char *scopestr = s + 1;
//       if (*scopestr) {
//         struct netif *netif = netif_find(scopestr);
//         if (netif) {
//           ip6_addr_assign_zone(addr, IP6_UNKNOWN, netif);
//         }
//       }
//     }
// // #endif
//   }
//
//   if (current_block_index != 7) {
//     return 0;
//   }
//
//   return 1;
// }

/**
 * Convert numeric IPv6 address into ASCII representation.
 * returns ptr to static buffer; not reentrant!
 *
 * @param addr ip6 address in network order to convert
 * @return pointer to a global static (!) buffer that holds the ASCII
 *         representation of addr
 */
// char *
// ip6addr_ntoa(const ip6_addr_t *addr)
// {
//   static char str[40];
//   return ip6addr_ntoa_r(addr, str, 40);
// }

/**
 * Same as ipaddr_ntoa, but reentrant since a user-supplied buffer is used.
 *
 * @param addr ip6 address in network order to convert
 * @param buf target buffer where the string is stored
 * @param buflen length of buf
 * @return either pointer to buf which now holds the ASCII
 *         representation of addr or NULL if buf was too small
 */
// char *
// ip6addr_ntoa_r(const ip6_addr_t *addr, char *buf, int buflen)
// {
//   u32_t current_block_index, current_block_value, next_block_value;
//   s32_t i;
//   u8_t zero_flag, empty_block_flag;
//
// // #if LWIP_IPV4
//   if (ip6_addr_isipv4mappedipv6(addr)) {
//     /* This is an IPv4 mapped address */
//    addr4: ip4_addr_t;
//     char *ret;
// #define IP4MAPPED_HEADER "::FFFF:"
//     char *buf_ip4 = buf + sizeof(IP4MAPPED_HEADER) - 1;
//     int buflen_ip4 = buflen - sizeof(IP4MAPPED_HEADER) + 1;
//     if (buflen < (int)sizeof(IP4MAPPED_HEADER)) {
//       return NULL;
//     }
//     memcpy(buf, IP4MAPPED_HEADER, sizeof(IP4MAPPED_HEADER));
//     addr4.addr =  addr.addr[3];
//     ret = ip4addr_ntoa_r(&addr4, buf_ip4, buflen_ip4);
//     if (ret != buf_ip4) {
//       return NULL;
//     }
//     return buf;
//   }
// // #endif /* LWIP_IPV4 */
//   i = 0;
//   empty_block_flag = 0; /* used to indicate a zero chain for "::' */
//
//   for (current_block_index = 0; current_block_index < 8; current_block_index++) {
//     /* get the current 16-bit block */
//     current_block_value = lwip_htonl( addr.addr[current_block_index >> 1]);
//     if ((current_block_index & 0x1) == 0) {
//       current_block_value = current_block_value >> 16;
//     }
//     current_block_value &= 0xffff;
//
//     /* Check for empty block. */
//     if (current_block_value == 0) {
//       if (current_block_index == 7 && empty_block_flag == 1) {
//         /* special case, we must render a ':' for the last block. */
//         buf[i++] = ':';
//         if (i >= buflen) {
//           return NULL;
//         }
//         break;
//       }
//       if (empty_block_flag == 0) {
//         /* generate empty block "::", but only if more than one contiguous zero block,
//          * according to current formatting suggestions RFC 5952. */
//         next_block_value = lwip_htonl( addr.addr[(current_block_index + 1) >> 1]);
//         if ((current_block_index & 0x1) == 0x01) {
//             next_block_value = next_block_value >> 16;
//         }
//         next_block_value &= 0xffff;
//         if (next_block_value == 0) {
//           empty_block_flag = 1;
//           buf[i++] = ':';
//           if (i >= buflen) {
//             return NULL;
//           }
//           continue; /* move on to next block. */
//         }
//       } else if (empty_block_flag == 1) {
//         /* move on to next block. */
//         continue;
//       }
//     } else if (empty_block_flag == 1) {
//       /* Set this flag value so we don't produce multiple empty blocks. */
//       empty_block_flag = 2;
//     }
//
//     if (current_block_index > 0) {
//       buf[i++] = ':';
//       if (i >= buflen) {
//         return NULL;
//       }
//     }
//
//     if ((current_block_value & 0xf000) == 0) {
//       zero_flag = 1;
//     } else {
//       buf[i++] = lwip_xchar(((current_block_value & 0xf000) >> 12));
//       zero_flag = 0;
//       if (i >= buflen) {
//         return NULL;
//       }
//     }
//
//     if (((current_block_value & 0xf00) == 0) && (zero_flag)) {
//       /* do nothing */
//     } else {
//       buf[i++] = lwip_xchar(((current_block_value & 0xf00) >> 8));
//       zero_flag = 0;
//       if (i >= buflen) {
//         return NULL;
//       }
//     }
//
//     if (((current_block_value & 0xf0) == 0) && (zero_flag)) {
//       /* do nothing */
//     }
//     else {
//       buf[i++] = lwip_xchar(((current_block_value & 0xf0) >> 4));
//       zero_flag = 0;
//       if (i >= buflen) {
//         return NULL;
//       }
//     }
//
//     buf[i++] = lwip_xchar((current_block_value & 0xf));
//     if (i >= buflen) {
//       return NULL;
//     }
//   }
//
//   buf[i] = 0;
//
//   return buf;
// }

// #endif /* LWIP_IPV6 */
