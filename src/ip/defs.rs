use crate::ethernet::ether_types::EtherType::Ipv6;

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

    pub fn from_bytes(a: u8, b: u8, c: u8, d: u8) -> Ipv4Address {
        Ipv4Address {
            octets: [a,b,c,d],
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

#[derive(Clone,Debug,Default)]
pub struct Ipv6Address {
    pub octets: [u8;16]
}

impl Ipv6Address {
    pub fn new() -> Ipv6Address {
        Ipv6Address::default()
    }

    pub fn from_bytes(b0: u8, b1:u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8, b7:u8, b8: u8, b9: u8, b10: u8, b11: u8, b12: u8, b13: u8, b14: u8, b15: u8) -> Ipv6Address {
        Ipv6Address {
            octets: [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15]
        }
    }

    pub fn from_array(bytes: &[u8;16]) -> Ipv6Address {
        Ipv6Address {
            octets: bytes.clone(),
        }
    }

    pub fn from_u128(num: u128) -> Ipv6Address {
        Ipv6Address {
            octets: num.to_le_bytes()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ipv6AddressRange {
    start: Ipv6Address,
    end: Ipv6Address
}

impl Ipv6AddressRange {
    pub fn new() -> Ipv6AddressRange {
        Ipv6AddressRange::default()
    }

    pub fn from_arrays(start: &[u8;16], end: &[u8;16]) -> Ipv6AddressRange {
        Ipv6AddressRange {
            start: Ipv6Address::from_array(start),
            end: Ipv6Address::from_array(end),
        }
    }
}

// IPv6 special addresses https://en.wikipedia.org/wiki/IPv6#Larger_address_space
