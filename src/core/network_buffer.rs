use crate::core::packet_buffer::PacketBuffer;
use crate::packet_buffer::PacketBuffer;
/** This netbuf has dest-addr/port set */
pub const NETBUF_FLAG_DESTADDR: u32 = 0x01;
/** This netbuf includes a checksum */
pub const NETBUF_FLAG_CHKSUM: u32 = 0x02;

/** "Network buffer" - contains data and addressing info */
#[derive(Debug,Clone,Default)]
pub struct NetworkBuffer {
    pub packet_buffer_id: u32,
    pub from_addr: IpAddress,
    pub from_port: u16,
    pub flags: u8,
    pub checksum: u16,
    pub dest_addr: IpAddress,
    pub dest_port: u16,
    pub dest_set: bool,
    pub checksum_set: bool,
}

impl NetworkBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_packet_buffer(&self) -> PacketBuffer {
        unimplemented!()
    }
    pub fn get_data(&self, offset: usize, size: usize) -> *const Vec<u8> {
        unimplemented!()
    }
    pub fn get_next(&self) -> *const PacketBuffer {
        unimplemented!()
    }
    pub fn get_prev(&self) -> *const PacketBuffer {
        unimplemented!()
    }
}
