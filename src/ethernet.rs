// #define IFNAME0 'e'
// #define IFNAME1 'n'

pub const LWIP_ARP_FILTER_NETIF: u32 = 0;
pub const ETH_HWADDR_LEN: usize = 6;
pub const ETHARP_HWADDR_LEN: usize = ETH_HWADDR_LEN;
pub const ETH_PAD_SIZE: usize = 0;

pub struct eth_addr {
    addr: [u8; ETH_HWADDR_LEN as usize],
}

impl eth_addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self {
            addr: [a, b, c, d, e, f],
        }
    }
}

pub struct eth_hdr {
    padding: [u8; ETH_PAD_SIZE],
    dest: eth_addr,
    src: eth_addr,
    ether_type: u16,
}

/**
 * Helper struct to hold private data used to operate your ethernet interface.
 * Keeping the ethernet address of the MAC in this struct is not necessary
 * as it is already kept in the struct netif.
 * But this is only an example, anyway...
 */
pub struct ethernetif {
    ethaddr: eth_addr,
    /* Add whatever per-interface state that is needed here. */
}
