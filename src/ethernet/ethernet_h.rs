use crate::core::common::lwip_htons;

pub const ETH_HWADDR_LEN: usize = 6; /* compatibility mode */

/* Ethernet header */
pub struct EthernetHeader {
    pub padding: [u8; ETH_PAD_SIZE],
    pub src_addr: [u8; ETH_HWADDR_LEN],
    pub dst_addr: [u8; ETH_HWADDR_LEN],
    pub ether_type: u16,
}

pub const STD_ETH_HDR_LEN: usize = (14 + ETH_PAD_SIZE);

/* VLAN header inserted between ethernet header and payload
 * if 'type' in ethernet header is ETHTYPE_VLAN.
 * See IEEE802.Q */
pub struct EthernetVlanHeader {
    pub prio_vid: u16,
    pub tpid: u16,
}

impl EthernetVlanHeader {
    pub fn get_vlan_id(&self) -> u16 {
        lwip_htons(self.prio_vid) & 0x0FFF
    }
}

pub const VLAND_HDR_SIZE: usize = 4;

/* The 24-bit IANA IPv4-multicast OUI is 01-00-5e: */
pub const LL_IP4_MULTICAST_ADDR_0: u32 = 0x01;
pub const LL_IP4_MULTICAST_ADDR_1: u32 = 0x00;
pub const LL_IP4_MULTICAST_ADDR_2: u32 = 0x5e;

/* IPv6 multicast uses this prefix */
pub const LL_IP6_MULTICAST_ADDR_0: u32 = 0x33;
pub const LL_IP6_MULTICAST_ADDR_1: u32 = 0x33;

pub fn eth_addr_cmp(addr1: &[u8; 6], addr2: &[u8; 6]) -> bool {
    addr1 == addr2
}
