/*
 * @file
 * This is the IPv4 address tools implementation.
 *
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

//  used by IP4_ADDR_ANY and IP_ADDR_BROADCAST in ip_addr.h 
// const LwipAddr ip_addr_any = IPADDR4_INIT(IPADDR_ANY);
// const LwipAddr ip_addr_broadcast = IPADDR4_INIT(IPADDR_BROADCAST);

/*
 * Determine if an address is a broadcast address on a network interface
 *
 * @param addr address to be checked
 * @param netif the network interface against which the address is checked
 * @return returns non-zero if the address is a broadcast address
 */
use crate::core::defines::LwipAddr;
use crate::netif::defs::{NETIF_FLAG_BROADCAST, NetworkInterfaceCtx};


/* Checks if a netmask is valid (starting with ones, then only zeros)
 *
 * @param netmask the IPv4 netmask to check (in network byte order!)
 * @return 1 if the netmask is valid, 0 if it is not
 */
pub fn ip4_addr_netmask_valid(netmask: u32) -> bool {
    //! First thing to do is to check for the netmask being non zero (a nasty edge case). Given this is ok, you need to take the bitwise inverse.
    //
    // uint32_t y = ~x;
    // Then add one
    //
    // uint32_t z = y + 1;
    // Then if x was a proper netmask, there will be at most 1 bit set in this.
    //
    // To test that, simply and z with z - 1, which happens to be y. The result will be zero if all is OK, non zero otherwise.
    //
    // valid = (z & y) == 0;
    let mut y: u32 = !netmask;
    let mut z: u32 = y + 1;
    z & y == 0
    // //  first, check for the first zero
    // for i in 0..31 {
    //     if netmask & 1 << i == 1 {
    //         break;
    //     }
    // }
    //
    // for (mask = 1 << 31 ; mask != 0; mask >>= 1) {
    //   if ((nm_hostorder & mask) == 0) {
    //     break;
    //   }
    // }
    //  // then check that there is no one
    // for (; mask != 0; mask >>= 1) {
    //   if ((nm_hostorder & mask) != 0) {
    //     //  there is a one after the first zero -> invalid
    //     return 0;
    //   }
    // }
    // //  no one after the first zero -> valid
    // return true;
}

/*
 * Ascii internet address interpretation routine.
 * The value returned is in network order.
 *
 * @param cp IP address in ascii representation (e.g. "127.0.0.1")
 * @return ip address in network order
 */
// pub fn ipaddr_addr(cp: &String) -> LwipAddr {
//     unimplemented!()
// }

/*
 * Check whether "cp" is a valid ascii representation
 * of an Internet address and convert to a binary address.
 * Returns 1 if the address is valid, 0 if not.
 * This replaces inet_addr, the return value from which
 * cannot distinguish between failure and a local broadcast address.
 *
 * @param cp IP address in ascii representation (e.g. "127.0.0.1")
 * @param addr pointer to which to save the ip address in network order
 * @return 1 if cp could be converted to addr, 0 on failure
 */
// pub fn ip4addr_aton(cp: &String) -> LwipAddr {
//     let val: u32;
//     let base: u8;
//     let c: u8;
//     let parts: [u32; 4];
//     u32 * pp = parts;
//
//     unimplemented!()
// }

/*
 * Convert numeric IP address into decimal dotted ASCII representation.
 * returns ptr to static buffer; not reentrant!
 *
 * @param addr ip address in network order to convert
 * @return pointer to a global static (!) buffer that holds the ASCII
 *         representation of addr
 */
// pub fn ip4addr_ntoa(addr: &LwipAddr) -> String {
//     return ip4addr_ntoa_r(addr);
// }

/*
 * Same as ip4addr_ntoa, but reentrant since a user-supplied buffer is used.
 *
 * @param addr ip address in network order to convert
 * @param buf target buffer where the string is stored
 * @param buflen length of buf
 * @return either pointer to buf which now holds the ASCII
 *         representation of addr or NULL if buf was too small
 */
// pub fn ip4addr_ntoa_r(addr: &LwipAddr) -> String {
//     unimplemented!()
// }

//  255.255.255.255
pub const IPADDR_NONE: u32 = 0xffffffff;
//  127.0.0.1
pub const IPADDR_LOOPBACK: u32 = 0x7f000001;
//  0.0.0.0
pub const IPADDR_ANY: u32 = 0x00000000;
//  255.255.255.255
pub const IPADDR_BROADCAST: u32 = 0xffffffff;

// pub fn IP_CLASSA(a: u32) -> bool {
//     a & 0x80000000 == 0
// }

pub const IP_CLASSA_NET: u32 = 0xff000000;
pub const IP_CLASSA_NSHIFT: u32 = 24;
pub const IP_CLASSA_HOST: u32 = 0xffffffff & !IP_CLASSA_NET;
pub const IP_CLASSA_MAX: u32 = 128;

// pub fn IP_CLASSB(a: u32) -> bool {
//     a & c0000000 == 0x80000000
// }

pub const IP_CLASSB_NET: u32 = 0xffff0000;
pub const IP_CLASSB_NSHIFT: u32 = 16;
pub const IP_CLASSB_HOST: u32 = 0xffffffff & !IP_CLASSB_NET;
pub const IP_CLASSB_MAX: u32 = 65536;

// pub fn IP_CLASSC(a: u32) -> bool {
//     a & 0xe0000000 == 0xc0000000
// }

pub const IP_CLASSC_NET: u32 = 0xffffff00;
pub const IP_CLASSC_NSHIFT: u32 = 8;
pub const IP_CLASSC_HOST: u32 = 0xffffffff & !IP_CLASSC_NET;

// pub fn IP_CLASSD(a: u32) -> bool {
//     a & f0000000 == 0xe0000000
// }

pub const IP_CLASSD_NET: u32 = 0xf0000000;
//  These ones aren't really
pub const IP_CLASSD_NSHIFT: u32 = 28;
pub const IP_CLASSD_HOST: u32 = 0x0fffffff;

// pub fn IP_MULTICAST(a: u32) -> bool {
//     IP_CLASSD(a)
// }
//
// pub fn IP_EXPERIMENTAL(a: u32) -> bool {
//     a & 0xf0000000 == 0xf0000000
// }
//
// pub fn IP_BADCLASS(a: u32) -> bool {
//     unimplemented!()
// }

pub const IP_LOOPBACKNET: u32 = 127;

//  Set an IP address given by the four byte-parts
// pub fn IP4_ADDR(ipaddr: &mut LwipAddr, a: u8, b: u8, c: u8, d: u8) {
//     ipaddr.raw[0] = a;
//     ipaddr.raw[1] = b;
//     ipaddr.raw[2] = c;
//     ipaddr.raw[3] = d;
// }

//  Copy IP address - faster than ip4_addr_set: no NULL check
// pub fn ip4_addr_copy(dest: &mut LwipAddr, src: &mut LwipAddr) {
//     dest.addr = src.addr
// }

//  Safely copy one IP address to another (src may be NULL)
// #define ip4_addr_set(dest, src) ((dest).addr = \
//                                     ((src) == NULL ? 0 : \
//                                     (src).addr))
// pub fn ip4_addr_set(dest: &mut LwipAddr, src: &mut LwipAddr) {
//     dest.addr = src.addr
// }

//  Set complete address to zero
// #define ip4_addr_set_zero(ipaddr)     ((ipaddr).addr = 0)
// pub fn ip4_addr_set_zero(ipaddr: &mut LwipAddr) {
//     ipaddr.addr = 0
// }

//  Set address to IPADDR_ANY (no need for lwip_htonl())
// #define ip4_addr_set_any(ipaddr)      ((ipaddr).addr = IPADDR_ANY)
// pub fn ip4_addr_set_any(ipaddr: &mut LwipAddr) {
//     ipaddr.addr = IPADDR_ANY
// }

//  Set address to loopback address
// #define ip4_addr_set_loopback(ipaddr) ((ipaddr).addr = PP_HTONL(IPADDR_LOOPBACK))
// pub fn ip4_addr_set_loopback(ipaddr: &mut LwipAddr) {
//     ipaddr.addr = PP_HTONL(IPADDR_LOOPBACK)
// }

//  Check if an address is in the loopback region
// #define ip4_addr_isloopback(ipaddr)    (((ipaddr).addr & PP_HTONL(IP_CLASSA_NET)) == PP_HTONL((IP_LOOPBACKNET) << 24))
// pub fn ip4_addr_isloopback(ipaddr: &mut LwipAddr) -> bool {
//     (ipaddr.addr & PP_HTONL(IP_CLASSA_NET)) == PP_HTONL(IP_LOOPBACKNET) << 24
// }

// #define ip4_addr_set_hton(dest, src) ((dest).addr = \
//                                ((src) == NULL ? 0:\
//                                lwip_htonl((src).addr)))
// pub fn ip4_addr_set_hton(dest: &mut LwipAddr, src: &mut LwipAddr) {
//     dest.addr = src.addr;
//     dest.addr = lwip_htonl(src.addr);
// }

//  IPv4 only: set the IP address given as an u32
// #define ip4_addr_set_u32(dest_ipaddr, src_u32) ((dest_ipaddr).addr = (src_u32))
// pub fn ip4_addr_set_u32(dest: &mut LwipAddr, src: u32) {
//     dest.addr = src;
// }

//  IPv4 only: get the IP address as an u32
// #define ip4_addr_get_u32(src_ipaddr) ((src_ipaddr).addr)
// pub fn ip4_addr_get_u32(ip_addr: &mut LwipAddr) -> u32 {
//     ip_addr.addr
// }

//  Get the network address by combining host address with netmask
// #define ip4_addr_get_network(target, host, netmask) loop { ((target).addr = ((host).addr) & ((netmask).addr)); } while(0)
pub fn ip4_addr_get_network(target: &mut Ipv4Address, host: &Ipv4Address, netmask: &Ipv4Address) {
    *target = host & netmask
}

pub fn ip4_addr_netcmp(addr1: & Ipv4Address, addr2: &Ipv4Address, mask: &Ipv4Address) -> bool {
    (addr1.into() & mask.into()) == (addr2.into() & mask.into())
}

// pub fn ip4_addr_isany_val(addr1: &mut LwipAddr) -> bool {
//     addr1.addr == IPADDR_ANY
// }

// #define ip4_addr_isany(addr1) ((addr1) == NULL || ip4_addr_isany_val(*(addr1)))
// pub fn ip4_addr_isany(addr1: &mut LwipAddr) -> bool {
//     ip4_addr_is_any_val(addr1)
// }

// #define ip4_addr_ismulticast(addr1) (((addr1).addr & PP_HTONL(0xf0000000)) == PP_HTONL(0xe0000000))
// pub fn ip4_addr_ismulticast(addr1: &mut LwipAddr) -> bool {
//     (addr1.addr & PP_HTONL(0xf0000000)) == PP_HTONL(0xe0000000)
// }

// #define ip4_addr_islinklocal(addr1) (((addr1).addr & PP_HTONL(0xffff0000)) == PP_HTONL(0xa9fe0000))
// pub fn ip4_addr_islinklocal(addr1: &mut LwipAddr) -> bool {
//     addr1.addr & PP_HTONL(0xffff0000) == PP_HTONL(0xa9fe0000)
// }

//  Get one byte from the 4-byte address
// #define ip4_addr_get_byte(ipaddr, idx) (((&(ipaddr).addr))[idx])
// pub fn ip4_addr_get_byte(ipaddr: &mut LwipAddr, idx: u32) -> u8 {
//     ipaddr[idx]
// }

// #define ip4_addr1(ipaddr) ip4_addr_get_byte(ipaddr, 0)
// pub fn ip4_addr1(ipaddr: &mut LwipAddr) -> u8 {
//     ip4_addr_get_byte(ipaddr, 0)
// }

// #define ip4_addr2(ipaddr) ip4_addr_get_byte(ipaddr, 1)
// pub fn ip4_addr2(ipaddr: &mut LwipAddr) -> u8 {
//     ip4_addr_get_byte(ipaddr, 1)
// }

// #define ip4_addr3(ipaddr) ip4_addr_get_byte(ipaddr, 2)
// pub fn ip4_addr3(ipaddr: &mut LwipAddr) -> u8 {
//     ip4_addr_get_byte(ipaddr, 2)
// }

// #define ip4_addr4(ipaddr) ip4_addr_get_byte(ipaddr, 3)
// pub fn ip4_addr4(ipaddr: &mut LwipAddr) -> u8 {
//     ip4_addr_get_byte(ipaddr, 3)
// }

// #define ip4_addr_get_byte_val(ipaddr, idx) ((((ipaddr).addr >> (idx * 8)) & 0xff))
// pub fn ip4_addr_get_byte_val(ipaddr: &ip4_addr, idx: u8) -> u8 {
//     ((ipaddr.addr >> (idx * 8)) & 0xff) as u8
// }

// #define ip4_addr1_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 0)
// pub fn ip4_addr1_val(ipaddr: &ip4_addr) -> u8 {
//     ip4_addr_get_byte_val(ipaddr, 0)
// }

// #define ip4_addr2_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 1)
// pub fn ip4_addr2_val(ipaddr: &ip4_addr) -> u8 {
//     ip4_addr_get_byte_val(ipaddr, 1)
// }

// #define ip4_addr3_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 2)
// pub fn ip4_addr3_val(ipaddr: &ip4_addr) -> u8 {
//     ip4_addr_get_byte_val(ipaddr, 2)
// }

// #define ip4_addr4_val(ipaddr) ip4_addr_get_byte_val(ipaddr, 3)
// pub fn ip4_addr4_val(ipaddr: &ip4_addr) -> u8 {
//     ip4_addr_get_byte_val(ipaddr, 3)
// }

// #define IP4ADDR_STRLEN_MAX  16
// pub const IP4_ADDR_STRLEN_MAX: u32 = 16;

//  For backwards compatibility
// #define ip_ntoa(ipaddr)  ipaddr_ntoa(ipaddr)
// type ip_ntoa = ipaddr_ntoa;

#[derive(Debug,Clone,Default,PartialEq)]
pub struct Ipv4Address {
    pub octets: [u8;4],
}

impl Ipv4Address {
    pub fn new() -> Ipv4Address {
        Ipv4Address::default()
    }

    pub fn from_array(array: [u8;4]) -> Ipv4Address {
        Ipv4Address {
            octets: array
        }
    }

    pub fn from_bytes(a: u8, b: u8, c: u8, d: u8) -> Ipv4Address {
        Ipv4Address {
            octets: [a,b,c,d],
        }
    }

    pub fn from_u32(val: u32) -> Ipv4Address {
        let val_bytes = val.to_le_bytes();
        Ipv4Address {
            octets: val_bytes
        }
    }

    pub fn set_u32(&mut self, val: u32) {
        let val_bytes = val.to_le_bytes();
        self.octets = val_bytes
    }

    pub fn is_any(&self) -> bool {
        self.into() == 0x00000000
    }

    pub fn ntoa(&self) -> String {
        format!("{}.{}.{}.{}", self.octets[a], self.octets[b], self.octets[c], self.octets[d])
    }

    pub fn is_class_a(&self) -> bool {
        self.into() & 0x80000000 == 0
    }

    pub fn is_class_b(&self) -> bool {
        self.into() & 0xc0000000 == 0x80000000
    }

    pub fn is_class_c(&self) -> bool {
        self.into() & 0xe0000000 == 0xc0000000
    }

    pub fn is_class_d(&self) -> bool {
        self.into() & 0xf0000000 == 0xe0000000
    }

    pub fn is_multicast(&self) -> bool {
        // self.is_class_d()
        self.into() & 0xf0000000 == 0xe0000000
    }

    pub fn is_experimental(&self) -> bool {
        self.into() & 0xf0000000 == 0xf0000000
    }

    pub fn is_link_local(&self) -> bool {
        self.into() & 0xffff0000 == 0xa9fe0000
    }

    pub fn zeroize(&mut self) {
        self.octets[0] = 0;
        self.octets[1] = 0;
        self.octets[2] = 0;
        self.octets[3] = 0;
    }

    pub fn set_any(&mut self) {
        self.zeroize()
    }

    pub fn set_loopback(&mut self) {
        self.octets[0] = 127;
        self.octets[1] = 0;
        self.octets[2] = 0;
        self.octets[3] = 1;
    }

    pub fn is_broadcast(&self, netif: &NetworkInterfaceCtx) -> bool {
        let addr_u32 = self.into();
        //  all ones (broadcast) or all zeroes (old skool broadcast)
        return if (!addr_u32 == IPADDR_ANY) || (addr_u32 == I { ADDR_ANY }) {
            true
            //  no broadcast support on this network interface?
        } else if (netif.flags & NETIF_FLAG_BROADCAST) == 0 {
            // the given address cannot be a broadcast address nor can we check against any broadcast addresses
            false
            //  address matches network interface address exactly? => no broadcast
        } else if addr == netif.ip_addr.into() {
            false
            //   on the same (sub) network...
        } else if ip4_addr_netcmp(&ipaddr, &netif.ip, &netif.netmask) {
            //  ...and host identifier bits are all ones? =>... && ((addr & !ip4_addr_get_u32(netif_ip4_netmask(netif))) == (IPADDR_BROADCAST & !ip4_addr_get_u32(netif_ip4_netmask(netif)))) {
            //  => network broadcast address
            true
        } else {
            false
        }
    }

    pub fn is_badclass(&self) -> bool {
        unimplemented!()
    }
}

impl From<u32> for Ipv4Address {
    fn from(item: u32) -> Self {
        let bytes = item.to_le_bytes();
        Ipv4Address { octets: bytes}
    }
}

impl Into<u32> for Ipv4Address {
    fn into(&self) -> u32 {
        u32::from_le_bytes(self.octets.clone())
    }
}


#[derive(Debug, Clone, Default)]
pub struct Ipv4AddressRange {
    pub start: Ipv4Address,
    pub end: Ipv4Address
}

impl Ipv4AddressRange {
    pub fn new() -> Ipv4AddressRange {
        Ipv4AddressRange::default()
    }

    pub fn from_arrays(start: [u8;4], end: [u8;4]) -> Ipv4AddressRange {
        Ipv4AddressRange {
            start: Ipv4Address::from_array(start),
            end: Ipv4Address::from_array(end),
        }
    }
}
