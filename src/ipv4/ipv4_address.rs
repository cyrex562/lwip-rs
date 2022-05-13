use crate::ipv4::ipv4_network::Ipv4Network;
use std::net::Ipv4Addr;
use crate::ipv4::ipv4_defines::{IPV4_CLASS_A_NET, IPV4_LOOPBACK_NET};

#[repr(C)]
pub union Ipv4AddressU {
    pub u8_addr: [u8;4],
    pub u32_addr: u32,
}

#[derive(Default,Debug,Clone,PartialEq)]
pub struct Ipv4Address {
    pub u: Ipv4AddressU,
}

impl Ipv4Address {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }

    pub fn from_vec(raw: &Vec<u8>, offset: usize) -> Self {
        Self {
            u: Ipv4AddressU { u8_addr: [raw[0+offset],raw[1+offset],raw[2+offset],raw[3+offset]]}
        }
    }

    pub fn from_octets(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            u: Ipv4AddressU {u8_addr: [a,b,c,d]}
        }
    }

    pub fn from_u32(a: u32) -> Self {
        Self {
            u: Ipv4AddressU {u32_addr: a}
        }
    }

    pub fn is_broadcast(&self) -> bool {
        if self.u.u32_addr == 0 {
            return true
        }
        if self.u.u32_addr == 0xffffffff {
            return true
        }
        false
    }


}

/// Check if a netmask is valid, starting with ones and ending with zeroes
/// netmask: netmask to check in u32 network byte order format
pub fn ipv4_netmask_valid(netmask: u32) -> bool {
    let mut mask = 0u32;
    mask = 1 << 31;
    let mut nm_host_order: u32 = netmask.to_le();
    while mask !=0 {
        if nm_host_order & mask == 0 {
            break;
        }
        mask >>= 1;
    }

    mask = 0;
    while mask != 0 {
        if nm_host_order & mask != 0 {
            return false
        }
        mask >>= 1;
    }

    true
}

pub fn ipv4_addr_aton(cp: &String) -> u32 {
    let addr = cp.parse::<Ipv4Addr>().unwrap();
    addr.to_u32()
}

pub fn ipv4_addr_ntoa(addr: u32) -> String {
    let a = addr.parse::<Ipv4Addr>().unwrap();
    a.to_string()
}

pub fn ipv4_is_class_a(a: u32) -> bool {
    a & 0x80000000 == 0
}

pub fn ipv4_is_class_b(a: u32) -> bool {
    a & 0xc0000000 == 0x80000000
}

pub fn ipv4_is_class_c(a: u32) -> bool {
    a & 0xe0000000 == 0xc0000000
}

pub fn ipv4_is_class_d(a: u32) -> bool {
    a & 0xf0000000 == 0xe0000000
}

pub fn ipv4_is_experimental(a: u32) -> bool {
    a & 0xf0000000 == 0xf0000000
}

pub fn ipv4_is_bad_class(a: u32) -> bool {
    a & 0xf0000000 == 0xf0000000
}


pub fn ipv4_addr_set_zero(addr: &mut Ipv4Address) {
        addr.u.u32_addr = 0;
}

pub fn ipv4_addr_set_any(addr: &mut Ipv4Address) {
        addr.u.u32_addr = 0;
}

pub fn ipv4_addr_set_loopback(addr: &mut Ipv4Address) {
    addr.u.u32_addr = 0x7f000001;
}

pub fn ipv4_addr_is_loopback(addr: &Ipv4Address) -> bool {
    addr.u.u32_addr & IPV4_CLASS_A_NET ==( IPV4_LOOPBACK_NET << 24)
}

pub fn ipv4_get_network(host: &Ipv4Address, netmask: &Ipv4Address) -> Ipv4Address {
    Ipv4Address::from_u32(host.u.u32_addr & netmask.u.u32_addr)
}

pub fn ipv4_net_eq(addr1: &Ipv4Address, addr2: &Ipv4Address, mask: &Ipv4Address) -> bool {
    addr1.u.u32_addr & mask.u.u32_addr == addr2.u.u32_addr & mask.u.u32_addr
}

pub fn ipv4_addr_is_any(addr: &Ipv4Address) -> bool {
    addr.u.u32 == 0
}

pub fn ipv4_addr_is_multicast(addr: &Ipv4Address) -> bool {
    addr.u.u32 & 0xf0000000 == 0xe0000000
}

pub fn ipv4_addr_is_link_local(addr: &Ipv4Address) -> bool {
    addr.u.u32 & 0xffff0000 == 0xa9fe0000
}



pub const IPV4_ADDR_ANY: Ipv4Address = Ipv4Address::from_octets(0,0,0,0);
pub const IPV4_ADDR_BCAST: Ipv4Address = Ipv4Address::from_octets(255, 255, 255, 255);
pub const IPV4_ADDR_NONE: Ipv4Address = Ipv4Address::from_octets(255,255,255,255);
pub const IPV4_ADDR_LOOPBACK: Ipv4Address = Ipv4Address::from_u32(0x7f000001);
