use crate::core::common::PacketHeader;
use crate::core::mac_address::MacAddress;
use crate::LwipError;

#[derive(Default,Debug,Clone)]
pub struct EthernetHeader {
    pub dest: MacAddress,
    pub src: MacAddress,
    pub ether_type: u16,
}

impl EthernetHeader {
    pub fn new() -> Self {
        Self {
            dest: MacAddress::new(),
            src: MacAddress::new(),
            ether_type: 0,
        }
    }
}

impl From<&[u8]> for EthernetHeader {
    fn from(raw: &[u8]) -> Self {
        Self {
            dest: MacAddress::from([raw[0], raw[1], raw[2], raw[3], raw[4], raw[5]]),
            src: MacAddress::from([raw[6], raw[7], raw[8], raw[9], raw[10], raw[11]]),
            ether_type: u16::from_be_bytes([raw[12],raw[13]])
        }
    }
}

impl PacketHeader for EthernetHeader {

}


