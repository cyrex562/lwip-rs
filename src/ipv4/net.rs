use crate::ipv4::addr::{ip4_get_network, Ipv4Address};

#[derive(Debug,Default,Clone,PartialEq)]
pub struct Ipv4Network {
    pub network_address: Ipv4Address,
    pub local_address: Ipv4Address,
    pub netmask: Ipv4Address,
    pub broadcast_addr: Ipv4Address,
}

impl Ipv4Network {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }

    pub fn addr_in_net(&self, tgt_addr: &Ipv4Address) -> bool {
        let tgt_addr_net = ip4_get_network(tgt_addr, &self.netmask);
        tgt_addr_net == self.network_address
    }
}

pub fn ipv4_addr_is_broadcast(addr: &Ip4Address, net: &Ipv4Network) -> bool {
    todo!()
}
