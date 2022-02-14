// const struct eth_addr ethbroadcast = {{0xff, 0xff, 0xff, 0xff, 0xff, 0xff}};
// const struct eth_addr ethzero = {{0, 0, 0, 0, 0, 0}};

use core::fmt;
use crate::core::defines::{LwipAddr, LwipAddrType};

pub const ETH_HWADDR_LEN: usize = 6;

#[derive(Debug,Clone,Default)]
pub struct EthernetHeader {
    pub src_addr: [u8; ETH_HWADDR_LEN],
    pub dst_addr: [u8; ETH_HWADDR_LEN],
    pub ether_type: u16,
}

impl EthernetHeader {
    pub fn new() -> EthernetHeader {
        EthernetHeader{
            ..Default::default()
        }
    }

    pub fn from_slice(buf: &[u8]) -> EthernetHeader {
        EthernetHeader {
            src_addr: [buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]],
            dst_addr: [buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]],
            ether_type: u16::from_le_bytes([buf[12], buf[13]])
        }
    }
}

impl fmt::Display for EthernetHeader {
    fn fmt(&self, f: &mut fmt::formatter<'_>) -> fmt::Result {
        write!(f, "source: {}, destination: {}, type: {:#?}",
               raw_mac_addr_to_string(&self.src_addr),
               raw_mac_addr_to_string(&self.dst_addr),
            self.ether_type
        )
    }
}

pub const ETH_HDR_LEN: usize = 14;

pub struct EthernetVlanHeader {
    //! VLAN header inserted between ethernet header and payload when ethernet header type is ETHER_TYPE_VLAN
    pub prio_vid: u16,
    pub tpid: u16,
}

impl EthernetVlanHeader {
    pub fn get_vlan_id(&self) -> u16 {
        lwip_htons(self.prio_vid) & 0x0FFF
    }

    pub fn from_slice(buf: &[u8]) -> EthernetVlanHeader {
        EthernetVlanHeader {
            prio_vid: u16::from_be_bytes([buf[0], buf[1]]),
            tpid: u16::from_be_bytes([buf[2], buf[3]])
        }
    }
}

pub const ETH_VLAN_HDR_LEN: usize = 4;
// The 24-bit IANA IPv4-multicast OUI is 01-00-5e:
// IPv6 multicast uses this prefix
// pub const LL_IP4_MULTICAST_ADDR_0: u32 = 0x01;
// pub const LL_IP4_MULTICAST_ADDR_1: u32 = 0x00;
// pub const LL_IP4_MULTICAST_ADDR_2: u32 = 0x5e;
// pub const LL_IP6_MULTICAST_ADDR_0: u32 = 0x33;
// pub const LL_IP6_MULTICAST_ADDR_1: u32 = 0x33;

pub type MacAddressOui = [u8;3];
pub type Eui48 = [u8;6];

#[derive(Clone,Debug,Default)]
pub struct MacAddressRange {
    pub start: Eui48,
    pub end: Eui48
}

#[derive(Clone,Debug,Default,PartialEq)]
pub struct MacAddress {
    pub octets: Eui48,
}

impl MacAddress {
    pub fn new() -> MacAddress {
        MacAddress::default()
    }

    pub fn from_bytes(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> MacAddress {
        MacAddress {
            octets: [a,b,c,d,e,f]
        }
    }

    pub fn from_slice(bytes: &[u8;6]) -> MacAddress {
        MacAddress {
            octets: bytes.clone(),
        }
    }

    pub fn get_oui(&self) -> [u8;3] {
        [self.octets[0], self.octets[1], self.octets[2]]
    }

    pub fn get_ig_bit(&self) -> u8 {
        self.octets[0] & 0b00000001
    }

    pub fn is_unicast(&self) -> bool {
        (self.octets[0] & 0b00000001) == 0
    }

    pub fn is_multicast(&self) -> bool {
        (self.octets[0] & 0b00000001) == 1
    }

    pub fn is_univ_admin(&self) -> bool {
        (self.octets[0] & 0b00000010) == 0
    }

    pub fn is_local_admin(&self) -> bool {
        (self.octets[0] & 0b00000010) == 1
    }
}

pub fn raw_mac_addr_to_string(addr: &[u8;6]) -> String {
    format!("{:#02x}:{:#02x}:{:#02x}:{:#02x}:{:#02x}:{:#02x}", addr[0], addr[1], addr[2], addr[3], addr[4], addr[5])
}

// TODO: https://en.wikipedia.org/wiki/Organizationally_unique_identifier
