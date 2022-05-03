#[derive(Default,Debug,Clone)]
pub struct MacAddress {
    addr: [u8; 6],
}

impl MacAddress {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self {
            addr: [a, b, c, d, e, f],
        }
    }
}


impl PartialEq for MacAddress {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
