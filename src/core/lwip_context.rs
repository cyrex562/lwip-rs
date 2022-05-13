use crate::arp::arp_table::ArpTable;
use crate::netif::network_interface::NetworkInterface;

#[derive(Debug,Clone,Default)]
pub struct LwipContext {
    pub netifs: Vec<NetworkInterface>,
    pub arp_table: ArpTable,
}

impl LwipContext {
    pub fn new() -> Self {
        Self {
            netifs: Vec::new()
        }
    }
}
