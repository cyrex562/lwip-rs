// const struct eth_addr ethbroadcast = {{0xff, 0xff, 0xff, 0xff, 0xff, 0xff}};
// const struct eth_addr ethzero = {{0, 0, 0, 0, 0, 0}};

use crate::core::defines::{LwipAddr, LwipAddrType};

pub const ETH_HWADDR_LEN: usize = 6;

#[derive(Debug,Clone,Default)]
pub struct EthernetHeader {
    pub padding: [u8; ETH_PAD_SIZE],
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
}

pub const STD_ETH_HDR_LEN: usize = (14 + ETH_PAD_SIZE);

pub struct EthernetVlanHeader {
    //! VLAN header inserted between ethernet header and payload when ethernet header type is ETHER_TYPE_VLAN
    pub prio_vid: u16,
    pub tpid: u16,
}

impl EthernetVlanHeader {
    pub fn get_vlan_id(&self) -> u16 {
        lwip_htons(self.prio_vid) & 0x0FFF
    }
}

pub const VLAND_HDR_SIZE: usize = 4;
// The 24-bit IANA IPv4-multicast OUI is 01-00-5e:
// IPv6 multicast uses this prefix
pub const LL_IP4_MULTICAST_ADDR_0: u32 = 0x01;
pub const LL_IP4_MULTICAST_ADDR_1: u32 = 0x00;
pub const LL_IP4_MULTICAST_ADDR_2: u32 = 0x5e;
pub const LL_IP6_MULTICAST_ADDR_0: u32 = 0x33;
pub const LL_IP6_MULTICAST_ADDR_1: u32 = 0x33;

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

