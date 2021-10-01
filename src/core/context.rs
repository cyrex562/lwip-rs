use crate::netif::netif_h::NetIfc;
use crate::ip::ip6_h::ip6_hdr;
use crate::core::defines::LwipAddr;
use std::collections::HashMap;
use crate::arp::defs::ArpEntry;
use crate::core::altcp_h::AlTcpContext;
use crate::tcp::abstract_tcp::context::AlTcpContext;
use crate::core::options::LwipOptions;
use crate::core::timer::Timer;
use crate::ip::ip4_h::Ip4Header;


#[derive(Debug, Clone, Default)]
pub struct LwipContext {
    pub current_netif: Option<NetIfc>,
    pub current_ip4_header: Option<ip4_hdr>,
    pub current_ip6_header: Option<ip6_hdr>,
    pub current_ip_header_to_len: usize,
    pub current_iphdr_src: Option<LwipAddr>,
    pub current_iphdr_dst: Option<LwipAddr>,
    pub al_tcp_pcb_map: HashMap<u32, AlTcpContext>,
    pub options: LwipOptions,
    pub timers: Vec<Timer>,
    pub arp_table: Vec<ArpEntry>,
    pub current_net_ifc: NetIfc,
    pub current_input_net_ifc: NetIfc,
    pub current_ip4_hdr: Ip4Header,
    pub current_ip6_hdr: Ip6Header,

}

impl LwipContext {
    pub fn new() -> LwipContext {
        LwipContext {
            current_netif: None,
            current_ip4_header: None,
            current_ip6_header: None,
            current_ip_header_to_len: 0,
            current_iphdr_src: None,
            current_iphdr_dst: None,
            al_tcp_pcb_map: HashMap::new(),
            options: Default::default(),
            timers: vec![],
            arp_table: vec![],
            current_net_ifc: Default::default(),
            current_input_net_ifc: Default::default(),
            current_ip4_hdr: Default::default(),
            current_ip6_hdr: ()
        }
    }
}

