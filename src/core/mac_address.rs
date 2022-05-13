#[derive(Default,Debug,Clone)]
pub struct MacAddress {
    addr: [u8; 6],
}

impl MacAddress {
    pub fn new() -> Self {
        Self {
            addr: [0;6]
        }
    }
    pub fn from_octets(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self {
            addr: [a, b, c, d, e, f],
        }
    }
    pub fn from_vec(raw: &Vec<u8>, offset: usize) -> Self {
        Self {
            addr: [raw[0+offset], raw[1+offset], raw[2+offset], raw[3+offset], raw[4+offset], raw[5+offset]]
        }
    }
}


impl PartialEq for MacAddress {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
