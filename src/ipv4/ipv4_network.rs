use crate::ipv4::ipv4_address::Ipv4Address;

#[derive(Debug,Default,Clone,PartialEq)]
pub struct Ipv4Network {
    pub network_id: Ipv4Address,
    pub netmask: Ipv4Address,
    pub broadcast_addr: Ipv4Address,
}

impl Ipv4Network {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }
    pub fn is_broadcast(&self, addr: &Ipv4Address) -> bool {
        todo!()
        // check if address is on the same net
        // check if the host id of the address is all 1's
    }
}
