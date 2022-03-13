use std::fmt;
use std::fmt::Formatter;

#[derive(Clone,Debug,Default, PartialEq)]
pub struct MacAddress {
    address: [u8;6]
}

impl MacAddress {
    pub fn new() -> Self {
        Self {
            address: [0,0,0,0,0,0]
        }
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", self.address[0], self.address[1], self.address[2], self.address[3], self.address[4], self.address[5])
    }
}

pub const BCAST_MAC_ADDR: MacAddress = MacAddress{ address: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff]};
