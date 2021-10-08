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
    pub octets: [u8;16],
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

    pub fn to_u128(&self) -> u128 {
        u128::from_le_bytes(self.octets)
    }

    pub fn to_dwords(&self) -> [u32;4] {
        [u32::from_le_bytes([self.octets[0], self.octets[1], self.octets[2], self.octets[3]]),
        u32::from_le_bytes([self.octets[4], self.octets[5], self.octets[6], self.octets[7]]),
        u32::from_le_bytes([self.octets[8], self.octets[9], self.octets[10], self.octets[11]]),
        u32::from_le_bytes([self.octets[12], self.octets[13], self.octets[14], self.octets[15]])]
    }

    pub fn from_dwords(dword_a: u32, dword_b: u32, dword_c: u32, dword_d: u32) -> Ipv6Address {
        let bytes_a: [u8;4] = dword_a.to_le_bytes();
        let bytes_b: [u8;4] = dword_b.to_le_bytes();
        let bytes_c: [u8;4] = dword_c.to_le_bytes();
        let bytes_d: [u8;4] = dword_d.to_le_bytes();
        let mut out = Ipv6Address::default();
        out.octets[0] = bytes_a[0];
        out.octets[1] = bytes_a[1];
        out.octets[2] = bytes_a[2];
        out.octets[3] = bytes_a[3];
        out.octets[4] = bytes_b[0];
        out.octets[5] = bytes_b[1];
        out.octets[6] = bytes_b[2];
        out.octets[7] = bytes_b[3];
        out.octets[8] = bytes_c[0];
        out.octets[9] = bytes_c[1];
        out.octets[10] = bytes_c[2];
        out.octets[11] = bytes_c[3];
        out.octets[12] = bytes_d[0];
        out.octets[13] = bytes_d[1];
        out.octets[14] = bytes_d[2];
        out.octets[15] = bytes_d[3];
        out
    }

    pub fn get_u16_block(&self, block: usize) -> u16 {
        let block_bytes: [u8;2] = [self.octets[block], self.octets[block+1]];
        u16::from_le_bytes(block_bytes)
    }

    pub fn zero(&mut self) {
        for i in 0..self.octets.len() {
            self.octets[i] = 0
        }
    }

    pub fn set_loopback(&mut self) {
        let mut x: u32 = 0x1;
        let y = Ipv6Address::from_dwords(0,0,0, x.to_be());
        self.octets = y.octets.clone();
    }

    pub fn swap_endianness(&mut self) {
        let x = u128::from_le_bytes(self.octets);
        let y = x.to_be_bytes();
        self.octets = y.clone();
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
