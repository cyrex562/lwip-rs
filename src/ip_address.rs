pub enum IpAddressType {
    IPv4,
    IPv6,
}

// IPv4 any 0.0.0.0
pub const IPV4_ANY_ADDR_BYTES: [u8; 4] = [0, 0, 0, 0];
pub const IPV4_ANY_ADDR_U32: u32 = 0;
// IPv4 loopback 127.0.0.1
pub const IPV4_LOOPBACK_BYTES: [u8; 4] = [127, 0, 0, 1];
pub const IPV4_LOOPBACK_U32: u32 = 0x7F000001;

// IPV6 any 0:0:0:0:0:0:0:0/128
pub const IPV6_ANY_ADDR_BYTES: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ];
pub const IPV6_ANY_ADDR_U128: u128 = 0;
// IPv6 loopback 0:0:0:0:0:0:0:1/128
pub const IPV6_LOOPBACK_BYTES: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, ];


#[derive(Clone, Debug, Default, PartialEq)]
pub struct IpAddress {
    address_bytes: Vec<u8>,
    address_type: IpAddressType,
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.address_type == IpAddressType::IPv4 {
            write!(f, "{}.{}.{}.{}", self.address_bytes[0], self.address_bytes[1], self.address_bytes[2], self.address_bytes[3])
        } else {
            write!(f, "{:01x}{:01x}:{:01x}{:01x}:{:01x}{:01x}:{:01x}{:01x}:{:01x}{:01x}:{:01x}{:01x}", self.address_bytes[0], self.address_bytes[1], self.address_bytes[2], self.address_bytes[3], self.address_bytes[4], self.address_bytes[5], self.address_bytes[6], self.address_bytes[7], self.address_bytes[8], self.address_bytes[9], self.address_bytes[10], self.address_bytes[11], self.address_bytes[12], self.address_bytes[13], self.address_bytes[14], self.address_bytes[15])
        }
    }
}

impl IpAddress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_ipv4_from_u32(&mut self, addr: u32) {
        let mut u32_bytes: [u8; 4] = [0, 0, 0, 0];
        u32_bytes = addr::from();
        self.address_bytes.clone_from_slice(&u32_bytes)
    }

    pub fn set_ipv4_from_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.address_bytes[0] = a;
        self.address_bytes[1] = b;
        self.address_bytes[2] = c;
        self.address_bytes[3] = d;
    }

    pub fn set_ipv6_from_bytes(&mut self, a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8, j: u8, k: u8, m: u8, n: u8, p: u8, q: u8, r: u8, s: u8) {
        self.address_bytes[0] = a;
        self.address_bytes[1] = b;
        self.address_bytes[2] = c;
        self.address_bytes[3] = d;
        self.address_bytes[4] = e;
        self.address_bytes[5] = f;
        self.address_bytes[6] = g;
        self.address_bytes[7] = h;
        self.address_bytes[8] = j;
        self.address_bytes[9] = k;
        self.address_bytes[10] = m;
        self.address_bytes[11] = n;
        self.address_bytes[12] = p;
        self.address_bytes[13] = q;
        self.address_bytes[14] = r;
        self.address_bytes[15] = s;
    }

    pub fn is_any(&self) -> bool {
        if self.address_type == IpAddressType::IPv4 {
            return self.address_bytes == IPV4_ANY_ADDR_BYTES;
        }
        return self.address_bytes == IPV6_ANY_ADDR_BYTES;
    }

    pub fn clear(&mut self) {
        self.address_bytes.clear()
    }

    pub fn networks_eq(&self, netmask: IpAddress) -> bool {
        unimplemented!()
    }

    pub fn eq_zoneless(&self, other: &IpAddress) -> bool {
        unimplemented!()
    }

    pub fn is_broadcast(&self, network: &IpAddress) -> bool {
        unimplemented!()
    }

    pub fn is_multicast(&self) -> bool {
        unimplemented!()
    }

    pub fn is_linklocal(&self) -> bool {
        unimplemented!()
    }

    pub fn map_ip4_to_ip6(&self) -> Self {
        unimplemented!()
    }

    pub fn umap_ip6_to_ip4(&self) -> Self {
        unimplemented!()
    }
}
