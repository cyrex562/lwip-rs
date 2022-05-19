use crate::core::packet_buffer::PacketBuffer;
use crate::packet_buffer::PacketBuffer;

pub const IPV4_RES_FRAG_FLAG: u16 = 0x8000;
pub const IPV4_DONT_FRAG_FLAG: u16 = 0x4000;
pub const IPV4_MORE_FRAG_FLAG: u16 = 0x2000;
pub const IPV4_FRAG_MASK: u16 = 0x1FFF;


/// Ipv4 Header
#[derive(Debug, Clone, Default)]
pub struct Ipv4Header {
    // version / header length
    v_hl : u8,
    // type of svc
    pub tos: u8,
    // tot len
    pub len: u16,
    // id
    pub id: u16,
    // frag off
    pub frag_off: u16,
    // TTL
    pub ttl: u8,
    // protocol
    pub proto: u8,
    // checksum
    pub checksum: u16,
    // src ipv4 addr
    pub src_addr: u32,
    // dst ipv4 addr
    pub dst_addr: u32,
}

impl Ipv4Header {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }

    pub fn get_version(&self) -> u8 {
        self.v_hl >> 4
    }

    pub fn get_hdr_len(&self) -> u8 {
        self.v_hl & 0x0f
    }

    pub fn get_hdr_len_bytes(&self) -> usize {
        self.hdr_len() * 4 as usize
    }

    pub fn get_offset_bytes(&self) -> usize {
        (self.offset & IPV4_FRAG_MASK) * 8
    }

    pub fn set_v_hl(&mut self, v: u8, hl: u8) {
        self.v_hl = (v << 4) | hl
    }

}

impl From<&PacketBuffer> for Ipv4Header {
    fn from(pkt_buf: &PacketBuffer) -> Self {
        todo!()
    }
}
