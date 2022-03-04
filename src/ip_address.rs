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

    pub fn set_ipv4_from_u32(&mut self, addr: u32) {
        let mut u32_bytes: [u8;4] = [0,0,0,0];
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
}
