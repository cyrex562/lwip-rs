use crate::netif::netif_h::NetIfc;
use crate::ip::ip6_h::ip6_hdr;
use crate::core::defines::LwipAddr;
use std::collections::HashMap;
use crate::core::altcp_h::AlTcpContext;
use crate::tcp::abstract_tcp::context::AlTcpContext;
use crate::core::options::LwipOptions;
use crate::core::timer::Timer;



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
    pub timers: Vec<Timer>
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
            timers: vec![]
        }
    }
}

// ip_globals
// {
//   //  The interface that accepted the packet for the current callback invocation. 
//   current_netif: &mut NetIfc;
//   //  The interface that received the packet for the current callback invocation. 
//   current_input_netif: &mut NetIfc;

//   //  Header of the input packet currently being processed. 
//   const current_ip4_header: &mut ip_hdr;

//   //  Header of the input IPv6 packet currently being processed. 
//   current_ip6_header: &mut ip6_hdr;

//   //  Total header length of current_ip4/6_header (i.e. after this, the UDP/TCP header starts) 
//   let current_ip_header_tot_len: u16;
//   //  Source IP address of current_header 
//   let current_iphdr_src: LwipAddr;
//   //  Destination IP address of current_header 
//   let current_iphdr_dest: LwipAddr;
// };
// extern struct ip_globals ip_data;
