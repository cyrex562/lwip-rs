#[derive(Default,Debug,Clone, PartialEq)]
pub struct MacAddress {
    addr: [u8; 6],
}

impl MacAddress {
    pub fn new() -> Self {
        Self {
            addr: [0;6]
        }
    }
}

impl From<&[u8]> for MacAddress {
    fn from(a: &[u8]) -> Self {
        Self {
            addr: [a[0],a[1],a[2],a[3],a[4],a[5]]
        }
    }
}

impl From <[u8;6]> for MacAddress {
    fn from(a: [u8; 6]) -> Self {
        Self {
            addr: [a[0],a[1],a[2],a[3],a[4],a[5]]
        }
    }
}

