use crate::ipv4::addr::Ipv4Address;
use crate::ipv6::ip6_addr::Ipv6Address;
use crate::mac_address::MacAddress;

pub enum RoutingTableEntryType {
    MacAddress,
    Ipv4Address,
    Ipv6Address,
}

#[repr(C)]
pub union RoutingTableAddress {
    ipv4: Ipv4Address,
    ipv6: Ipv6Address,
}

#[repr(C)]
pub union RoutingTableMask {
    ipv4: Ipv4Address,
    ipv6: Ipv6Address,
    // MAC addresses dont have masks?
}

pub struct RoutingTableEntry {
    pub entry_type: RoutingTableEntryType,
    pub address: RoutingTableAddress,
    pub mask: RoutingTableMask,
    pub priority: u32,
}


#[derive(Clone,Debug,Default)]
pub struct RoutingTable {
    name: String,
    entries: Vec<RoutingTableEntry>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }
}
