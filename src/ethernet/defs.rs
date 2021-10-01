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


pub const ETHERNET_BROADCAST_ADDRESS: LwipAddr = LwipAddr {
    addr_type: LwipAddrType::AddrTypeEthernet,
    raw: [ff,ff,ff,ff,ff,ff,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ipv6_address_state: 0,
    ipv6_address_valid_life: 0,
    ipv6_address_preferred_life: 0,
    ipv6_address_zone: 0
}
