#[derive(Debug,Clone,Default,PartialEq)]
pub struct Ipv4Address {
    pub octets: [u8;4],
}

impl Ipv4Address {
    pub fn new() -> Ipv4Address {
        Ipv4Address::default()
    }

    pub fn from_array(array: [u8;4]) -> Ipv4Address {
        Ipv4Address {
            octets: array
        }
    }
}

impl From<u32> for Ipv4Address {
    fn from(item: u32) -> Self {
        let bytes = item.to_le_bytes();
        Ipv4Address { octets: bytes}
    }
}

impl Into<u32> for Ipv4Address {
    fn into(&self) -> u32 {
        u32::from_le_bytes(self.octets.clone())
    }
}


#[derive(Debug, Clone, Default)]
pub struct Ipv4AddressRange {
    pub start: Ipv4Address,
    pub end: Ipv4Address
}

impl Ipv4AddressRange {
    pub fn new() -> Ipv4AddressRange {
        Ipv4AddressRange::default()
    }

    pub fn from_arrays(start: [u8;4], end: [u8;4]) -> Ipv4AddressRange {
        Ipv4AddressRange {
            start: Ipv4Address::from_array(start),
            end: Ipv4Address::from_array(end),
        }
    }
}
