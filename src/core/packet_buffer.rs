use crate::core::errors::{LwipError, LwipErrorCode};
use crate::debug::LWIP_DEBUGF;
use crate::errors::{LwipError, LwipErrorCode};

/**
 * @file
 * Packet buffer management
 */

/**
 * @defgroup pbuf Packet buffers (PBUF)
 * @ingroup infrastructure
 *
 * Packets are built from the pbuf data structure. It supports dynamic
 * memory allocation for packet contents or can reference externally
 * managed packet contents both in RAM and ROM. Quick allocation for
 * incoming packets is provided through pools with fixed sized pbufs.
 *
 * A packet may span over multiple pbufs, chained as a singly linked
 * list. This is called a "pbuf chain".
 *
 * Multiple packets may be queued, also using this singly linked list.
 * This is called a "packet queue".
 *
 * So, a packet queue consists of one or more pbuf chains, each of
 * which consist of one or more pbufs. CURRENTLY, PACKET QUEUES ARE
 * NOT SUPPORTED!!! Use helper structs to queue multiple packets.
 *
 * The differences between a pbuf chain and a packet queue are very
 * precise but subtle.
 *
 * The last pbuf of a packet has a ->tot_len field that equals the
 * ->len field. It can be found by traversing the list. If the last
 * pbuf of a packet has a ->next field other than NULL, more packets
 * are on the queue.
 *
 * Therefore, looping through a pbuf of a single packet, has an
 * loop end condition (tot_len ==  p.len), NOT (next == NULL).
 *
 * Example of custom pbuf usage: @ref zerocopyrx
 */

pub enum PayloadType {
    Data,
    Link,
    Internet,
    Transport,
    Application,
    Other,
    Unknown,
    NotSet,
}

#[derive(Debug,Clone,Default)]
pub struct PayloadMapEntry {
    pub offset: isize,
    pub length: isize,
    pub ptype: PayloadType
}

impl PayloadMapEntry {
    pub fn new() -> Self {
        Self {
            offset: -1,
            length: -1,
            ptype: PayloadType::NotSet,
        }
    }
}

/** Main packet buffer struct */
#[derive(Debug, Clone, Default)]
pub struct PacketBuffer {
    /// pass the data immediately to the receiving application
    pub push_flag: bool,
  /// indicates this is custom pbuf
  pub custom_flag: bool,
  /// indiciates this pbuf is a UDP multicast packet to be looped back
  pub udp_mcast_loop_flag: bool,
  /// indicates this pbuf was received as a link-level broadcast
  pub ll_bcast_flag: bool,
  /// indicates this pbuf includes a tcp FIN flag
  pub tcp_fin_flag: bool,
  /// indicates this was areceived as a link-level multicast
  pub ll_mcast_flag: bool,
  /// pointer to the actual data in the buffer/
  pub payload: Vec<u8>,
  /// input netif id
  pub input_netif_id: i64,
    /// payload map
    pub payload_map: Vec<PayloadMapEntry>
}

impl PacketBuffer {
    pub fn new() -> Self {
        Self {
            push_flag: false,
            custom_flag: false,
            udp_mcast_loop_flag: false,
            ll_bcast_flag: false,
            tcp_fin_flag: false,
            ll_mcast_flag: false,
            // TODO: support spreading payload out across multiple buffers; maybe with IoSlice?
            payload: Vec::new(),
            input_netif_id: -1,
            payload_map: Vec::new()
        }
    }


    pub fn cat(&mut self, pbuf_to_cat: &Self) {
        unimplemented!()
    }

    pub fn copy_contents(&mut self, source: &Self, offset_from: isize, offset_to: isize, length: usize) {
        unimplemented!()
    }

    pub fn copy_contents2(&mut self, source: &Vec<u8>, offset_from: isize, offset_to: isize, length: usize) {
        unimplemented!()
    }

    // get chunk of buffer at offset
    pub fn get_contiguous(&self, offset: isize, len: usize) -> Vec<u8> {
        unimplemented!()
    }

    // split a pbuf into 64k pbuffers
    pub fn split_64k(&self) -> Vec<Self> {
        unimplemented!()
    }

    // combine a group of pbufs into a single one
    pub fn coalesce(&mut self, buffers: &Vec<Self>) {
        unimplemented!()
    }

    pub fn get_at(&self, offset: usize, len: usize) -> Vec<u8> {
        unimplemented!()
    }

    pub fn put_at(&mut self, src: &Vec<u8>, offset: usize, len: usize) {
        unimplemented!()
    }

    pub fn memcmp(&self, other: &self) -> isize {
        unimplemented!()
    }

    // verify presence of bytes in a buffer, returning the offset if found, -1 otherwise
    pub fn memfind(&self, bytes_to_find: &Vec<u8>, start_offset: usize) -> isize {
        unimplemented!()
    }

    pub fn strfind(&self, str_to_find: &str, start_offset: usize) -> isize {
        unimplemented!()
    }

    pub fn payload_contains_header_type(&self, ptype: PayloadType) -> Result<PayloadMapEntry, LwipError>
    {
        let result = self.payload_map.iter().find(|entry| entry.ptype == ptype);
        return match result {
            Some(x) => Ok(x.clone()),
            None() => Err(LwipError::new(LwipErrorCode::NotFound, "payload type {} not found in payload map".format(ptype)))
        }
    }
}
