// #define IFNAME0 'e'
// #define IFNAME1 'n'

use crate::errors::LwipError;
use crate::errors::LwipErrorCode::InvalidArgument;

pub const LWIP_ARP_FILTER_NETIF: u32 = 0;
pub const ETH_HWADDR_LEN: usize = 6;
pub const ETHARP_HWADDR_LEN: usize = ETH_HWADDR_LEN;
pub const ETH_PAD_SIZE: usize = 0;

#[derive(Default,Debug,Clone)]
pub struct EthAddr {
    addr: [u8; ETH_HWADDR_LEN as usize],
}

// TPID: 16 bits: (0x8100)
// TCI:
//     PCP: 3 bits: class-of-service
//     DEI: 1 bit: drop-eligible indicator
//     VID: 12-bits VLAN tag
#[derive(Default,Debug,Clone)]
pub struct VlanTag {
    pub tpid: u16,
    pub tci: u16,
}

impl VlanTag {
    pub fn set_pcp(&mut self, new_pcp_val: u8) -> Result<(), LwipError> {
        if new_pcp_val > 0b111 {
            return Err(LwipError::new(InvalidArgument, format!("invalid value for new pcp val: {}", new_pcp_val).as_str()))
        }
        self.tci = self.tci & new_pcp_val << 13;
        Ok(())
    }

    pub fn get_pcp(&self) -> u16 {
        self.tci & 0xe000 //0b1110000000000000
    }

    pub fn set_dei(&mut self, dei: u8) -> Result<(), LwipError> {
        if dei > 1 {
            return Err(LwipError::new(InvalidArgument, format!("invalid value for new dei val: {}", dei).as_str()))
        }

        self.tci = self.tci & dei << 12;

        Ok(())
    }

    pub fn get_dei(&self) -> u16 {
        self.tci & 0x1000 // 0b0001000000000000
    }

    pub fn set_vid(&mut self, vid: u16) {
        self.tci = self.tci | vid;
    }

    pub fn get_vid(&self) -> u16 {
        self.tci & 0x0fff
    }
}

impl EthAddr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self {
            addr: [a, b, c, d, e, f],
        }
    }
}

impl PartialEq for EthAddr {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

#[derive(Default,Debug,Clone)]
pub struct EthHdr {
    padding: [u8; ETH_PAD_SIZE],
    dest: EthAddr,
    src: EthAddr,
    // TODO: 802.1 q-in-q
    vlan_tags: VlanTag,
    ether_type: u16,
}

pub const SIZEOF_ETH_HDR: usize = 14 + ETH_PAD_SIZE;

pub struct EthVlanHdr {
    prio_vid: u16,
    tpid: u16,
}

impl EthVlanHdr {
    pub fn vlan_id(&self) -> u16 {
        self.prio_vid.to_be() & 0xFFF
    }
}

pub const SIZEOF_VLAN_HDR: usize = 4;

// The 24-bit IANA IPv4-multicast OUI is 01-00-5e:
pub const LL_IP4_MULTICAST_ADDR_0: u8 = 0x01;
pub const LL_IP4_MULTICAST_ADDR_1: u8 = 0x00;
pub const LL_IP4_MULTICAST_ADDR_2: u8 = 0x5e;

// IPv6 multicast prefix
pub const LL_IP6_MULTICAST_ADDR_0: u8 = 0x33;
pub const LL_IP6_MULTICAST_ADDR_1: u8 = 0x33;

/**
 * Helper struct to hold private data used to operate your ethernet interface.
 * Keeping the ethernet address of the MAC in this struct is not necessary
 * as it is already kept in the struct netif.
 * But this is only an example, anyway...
 */
// pub struct ethernetif {
//     ethaddr: EthAddr,
//     /* Add whatever per-interface state that is needed here. */
// }

pub fn low_level_init(&mut netif: netif) {
    
}
