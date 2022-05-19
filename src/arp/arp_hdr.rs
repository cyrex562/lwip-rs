use crate::ipv4::addr::Ipv4Address;
use crate::mac_address::MacAddress;

#[derive(Clone,Default,Debug)]
pub struct ArpMessage {
    pub hwtype: u16,
    pub proto: u16,
    pub hwlen: u16,
    pub protolen: u16,
    pub opcode: u16,
    pub sender_hw_addr: Vec<u8>,
    pub sender_proto_addr: Vec<u8>,
    pub target_hw_addr: Vec<u8>,
    pub target_proto_addr: Vec<u8>,
}

impl ArpMessage {
    pub fn new() -> Self {
        Self {
            hwtype: 0,
            proto: 0,
            hwlen: 0,
            protolen: 0,
            opcode: 0,
            sender_hw_addr: Vec::new(),
            sender_proto_addr: Vec::new(),
            target_hw_addr: Vec::new(),
            target_proto_addr: Vec::new(),
        }
    }

    pub fn from_u8_slice(raw: &[u8]) -> Self {
        let mut result = Self {
            hwtype: u16::from_be_bytes([raw[0], raw[1]]),
            proto: u16::from_be_bytes([raw[2], raw[3]]),
            hwlen: u16::from_be_bytes([raw[4], raw[5]]),
            protolen: u16::from_be_bytes([raw[6],raw[7]]),
            opcode: u16::from_be_bytes([raw[8], raw[9]]),
            sender_hw_addr: Vec::new(),
            sender_proto_addr: Vec::new(),
            target_hw_addr: Vec::new(),
            target_proto_addr: Vec::new(),
        };
        for i in 0.. result.hwlen {
            result.sender_hw_addr.push(raw[9 + i]);
            result.target_hw_addr.push(raw[9 + i + result.hwlen + result.protolen]);
        }
        for i in 0..result.protolen {
            result.sender_proto_addr.push(raw[9+i+result.hwlen]);
            result.target_proto_addr.push(raw[9+i+result.hwlen+result.protolen+result.hwlen]);
        }
        result
    }

    pub fn from_u8_vec(raw: &Vec<u8>, offset: usize) -> Self {
        let mut result = Self {
            hwtype: u16::from_be_bytes([raw[0+offset], raw[1+offset]]),
            proto: u16::from_be_bytes([raw[2+offset], raw[3+offset]]),
            hwlen: u16::from_be_bytes([raw[4+offset], raw[5+offset]]),
            protolen: u16::from_be_bytes([raw[6+offset],raw[7+offset]]),
            opcode: u16::from_be_bytes([raw[8+offset], raw[9+offset]]),
            sender_hw_addr: Vec::new(),
            sender_proto_addr: Vec::new(),
            target_hw_addr: Vec::new(),
            target_proto_addr: Vec::new(),
        };
        for i in 0.. result.hwlen {
            result.sender_hw_addr.push(raw[9 + i+offset]);
            result.target_hw_addr.push(raw[9 + i + result.hwlen + result.protolen+offset]);
        }
        for i in 0..result.protolen {
            result.sender_proto_addr.push(raw[9+i+result.hwlen+offset]);
            result.target_proto_addr.push(raw[9+i+result.hwlen+result.protolen+result.hwlen+offset]);
        }
        result
    }

    pub fn get_arp_len(&self) -> usize {
        (10 + (self.proto *2) + (self.hwtype * 2)) as usize
    }
}
