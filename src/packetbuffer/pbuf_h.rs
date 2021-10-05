//! PacketBuffer support


use std::iter::Map;
use crate::ethernet::ether_types::EtherType;


//  Base flags for pbuf_type definitions: 
/* Indicates that the payload directly follows the PacketBuffer.
 *  This makes @ref pbuf_header work in both directions. */
pub const PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS: u32 = 0x80;
/* Indicates the data stored in this pbuf can change. If this pbuf needs
 * to be queued, it must be copied/duplicated. */
pub const PBUF_TYPE_FLAG_DATA_VOLATILE: u32 = 0x40;
/* 4 bits are reserved for 16 allocation sources (e.g. heap, pool1, pool2, etc)
 * Internally, we use: 0=heap, 1=MEMP_PBUF, 2=MEMP_PBUF_POOL -> 13 types free*/
pub const PBUF_TYPE_ALLOC_SRC_MASK: u32 = 0x0F;
/* Indicates this pbuf is used for RX (if not set, indicates use for TX).
 * This information can be used to keep some spare RX buffers e.g. for
 * receiving TCP ACKs to unblock a connection) */
pub const PBUF_ALLOC_FLAG_RX: u32 = 0x0100;
//  Indicates the application needs the pbuf payload to be in one piece 
pub const PBUF_ALLOC_FLAG_DATA_CONTIGUOUS: u32 = 0x0200;

pub const PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP: u32 = 0x00;

// #define PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF      0x01
pub const PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF: u32 = 0x01;

// #define PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL 0x02
pub const PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL: u32 = 0x02;

//  First pbuf allocation type for applications 
pub const PBUF_TYPE_ALLOC_SRC_MASK_APP_MIN: u32 = 0x03;
//  Last pbuf allocation type for applications 
// #define PBUF_TYPE_ALLOC_SRC_MASK_APP_MAX            PBUF_TYPE_ALLOC_SRC_MASK
pub const PBUF_TYPE_ALLOC_SRC_MASK_APP_MAX: u32 = PBUF_TYPE_ALLOC_SRC_MASK;


//  indicates this packet's data should be immediately passed to the application 
pub const PBUF_FLAG_PUSH: u32 = 0x01;
/* indicates this is a custom pbuf: PacketBuffer_free calls pbuf_custom.custom_free_function()
when the last reference is released (plus custom PBUF_RAM cannot be trimmed) */
pub const PBUF_FLAG_IS_CUSTOM: u32 = 0x02;
//  indicates this pbuf is UDP multicast to be looped back 
pub const PBUF_FLAG_MCASTLOOP: u32 = 0x04;
//  indicates this pbuf was received as link-level broadcast 
pub const PBUF_FLAG_LLBCAST: u32 = 0x08;
//  indicates this pbuf was received as link-level multicast 
pub const PBUF_FLAG_LLMCAST: u32 = 0x10;
//  indicates this pbuf includes a TCP FIN flag 
pub const PBUF_FLAG_TCP_FIN: u32 = 0x20;

#[derive(Clone,Debug,Default)]
pub struct PacketBufferLayer {
    pub offset: isize,
    pub content_type: PacketBufferContentType,
}

impl PacketBufferLayer {
    pub fn new() -> PacketBufferLayer {
        PacketBufferLayer::default()
    }
}



#[derive(Clone, Debug)]
pub enum PacketBufferContentType {
    Unknown = 0,
    Ethernet = 1,
    Vlan = EtherType::Vlan as isize,
    Ipv4 = EtherType::IPv4 as isize,
    Arp = EtherType::ARP as isize,
    PppoeDisc = EtherType::PppoeDisc as isize,
    PppoeSession = EtherType::PppoeSession as isize,
    // IP Protocols 0..255
    // Ether Types 1536..65535
    // Other Type 65536..Inf
}

//  Main packet buffer struct
#[derive(Debug,Clone,Default)]
pub struct PacketBuffer {
    // map of offsets and types
    pub contents_map: Vec<PacketBufferLayer>,
    // the data
    pub buffer: Vec<u8>,
    //  misc flags
    pub flags: u8,
    //  For incoming packets, this contains the input netif's index 
    pub netif_id: i64,
}

impl PacketBuffer {
    pub fn new() -> PacketBuffer {
        PacketBuffer::default()
    }
}
