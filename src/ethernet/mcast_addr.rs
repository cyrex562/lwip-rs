use crate::core::mac_address::MacAddress;
use crate::ethernet::ether_type::EtherType;

#[derive(Debug,Clone,Default)]
pub struct EthernetMulticastAddress {
    pub start_address: MacAddress,
    pub end_address: Option<MacAddress>,
    pub ethertype: EtherType,
}
