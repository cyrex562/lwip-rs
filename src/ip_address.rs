pub enum IpAddressType {
    IPv4,
    IPv6,
}

#[derive(Clone,Debug,Default)]
pub struct IpAddress {
    address_bytes: Vec<u8>,
    address_type: IpAddressType,
}

impl IpAddress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn  init_ipv4(&mut self, addr: u32) {
        let mut u32_bytes: [u8;4] = [0,0,0,0];
        u32_bytes = addr::from()
    }
}
