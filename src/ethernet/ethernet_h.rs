use crate::core::common::lwip_htons;

//  compatibility mode 

//  Ethernet header 

/* VLAN header inserted between ethernet header and payload
 * if 'type' in ethernet header is ETHTYPE_VLAN.
 * See IEEE802.Q */





pub fn eth_addr_cmp(addr1: &[u8; 6], addr2: &[u8; 6]) -> bool {
    addr1 == addr2
}
