use crate::api::api_h::NetConnDesc;
use crate::packetbuffer::pbuf_h::PacketBuffer;

// sockaddr_in / sockaddr_in6
pub struct LwipSockAddr {
    pub family: u16,    // ADDRESS FAMILY
    pub port: u16,      // port
    pub addr: [u8; 16], // address
    pub flow_info: u32, // ipv6 flow information
    pub scope_id: u32,  // scope id
}

impl LwipSockAddr {
    pub fn new() -> LwipSockAddr {
        LwipSockAddr {
            family: 0,
            port: 0,
            addr: [0; 16],
            flow_info: 0,
            scope_id: 0,
        }
    }
}

pub struct LwipSocket {
    pub sockfd: i32,
    pub conn: NetConnDesc,
    pub netbuf: netbuf,
    pub pbuf: PacketBuffer,
    pub events_received: usize,
    pub events_acked: usize,
    pub error_happend: bool,
    pub num_threads_waiting: usize,
    pub fd_used: bool,
    pub fd_free_pending: bool,
}

pub enum LwipAddrType {
    AddrTypeEthernet,
    AddrTypeIpv4,
    AddrTypeIpv6,
    AddrTypePort,
}

#[derive(Debug, Clone, Default)]
pub struct LwipAddr {
    pub addr_type: LwipAddrType,
    pub raw: [u8; 20],
    pub ipv6_address_state: u16,
    pub ipv6_address_valid_life: u64,
    pub ipv6_address_preferred_life: u64,
    pub ipv6_address_zone: u32,
}

impl LwipAddr {
    pub fn new() -> LwipAddr {
        LwipAddr::default()
    }
}
