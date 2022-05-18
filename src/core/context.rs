use crate::arp::arp_table::ArpTable;
use crate::netif::netif::NetworkInterface;

#[derive(Default,Debug,Clone)]
pub struct Options {
    pub test_mode: bool,
    pub checksum_on_copy: bool,
    pub netconn_sem_per_thread: bool,
    pub mem_size: usize,
    pub tcp_send_q_len: usize,
    pub tcp_send_buf: usize,
    pub tcp_wnd_sz: usie,
    pub tcp_wnd_scale: bool,
    pub tcp_rcv_scale: bool,
    pub mdns_responder: bool,
    pub arp_spt_static_entries: bool,
    pub mib2_stats: bool,
    pub netif_ext_status_callback: bool,
    /// number of pbufs
    pub num_pbuf: usize,
    /// number of raw pcbs
    pub num_raw_pcb: usize,
    /// number of udp pcbs
    pub num_udp_pcb: usize,
    /// number of tcp pcbs
    pub num_tcp_pcb: usize,
    /// number of listening tcp connections
    pub num_tcp_pcb_listen: usize,
    /// the number of simultaneously queued packets for reassembly
    pub num_reass_data: usize,
    /// number of simultaneously queued TCP segments
    pub num_tcp_seg: usize,
    /// number of simultaneously  queued outgoing ARP messages
    pub num_arp_queue: usize,
    /// number of simultaneously active timeouts
    pub num_timeouts: usize,
    /// the number of netbufs
    pub num_netbuf: usize,
    /// the number of netconns
    pub num_netconn: usize,
    /// number of tcpip_msg
    pub num_tcpip_msg_api: usize,
    /// number of tcpip_msg used for incoming packets
    pub num_tcpip_msg_inpkt: usize,
    /// number of buffers in the pbuf pool
    pub pbuf_pool_sz: usize,
    /// ARP options
    /// enable/disable arp
    pub arp: bool,
    // IP options
    /// forward ip packets across itnerfaces
    pub ip_forward: bool,
    /// if true, packets with IP options are allowed, otherwise, they are dropped
    pub ip_options_allowed: bool,
    /// if true, reassemble incoming IP packets
    pub ip_reassembly: bool,
    /// fragment outgoing packets if their size exceeds MTU
    pub ip_frag: bool,
    /// maximum time in multiples of IP_TMR_INTERVAL a fragmented IP packet waits for all fragments to arrive.
    pub ip_reass_maxage: i64,
    /// total max number of pbufs waiting to be reassembled.
    pub ip_reass_max_pbufs: usize,
    /// if true use a static mtu-sized buffer for ip fragmentation
    pub ip_frag_uses_static_pbuf: bool,
    /// default TTL value
    pub ip_dflt_ttl: u8,
    // ICMP options
    /// enable ICMP module
    pub icmp: bool,
    // RAW options
    /// enable app layer to hook into the IP layer itself
    pub raw: bool,
    // DHCP options
    /// enable DHCP
    pub dhcp: bool,
    // autoip options
    /// enable autoip
    pub autoip: bool,
    // SNMP options
    /// enable snmp
    pub snmp: bool,
    // IGMP options
    pub igmp: bool,
    // DNS options
    pub dns: bool,
    // UDP options
    pub udp: bool,
    // TCP options
    pub tcp: bool,
    pub listen_backlog: bool,
    // PBUF options
    /// number of bytes that should be allocated for a link level header
    pub pbuf_link_hlen: usize,
    /// the size of each pbuf in the pbuf pool
    pub pbuf_pool_bufsize: usize,
    // LOOPIF options
    pub have_loopif: bool,
    // NETCONN options
    pub netconn: bool,
    // socket support
    pub socket: bool,
    // enable SO_REUSEADDR
    pub so_reuse: bool,
    // stats options
    pub stats: bool,
    // ppp options
    pub ppp: bool,
    // threading options
    pub tcpip_core_locking: bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            arp: true,
            arp_spt_static_entries: true,
            autoip: false,
            checksum_on_copy: true,
            dhcp: true,
            dns: true,
            have_loopif: true,
            icmp: true,
            igmp: true,
            ip_dflt_ttl: 255,
            ip_forward: true,
            ip_frag: true,
            ip_frag_uses_static_pbuf: false,
            ip_options_allowed: true,
            ip_reass_max_pbufs: 4,
            ip_reass_maxage: 3,
            ip_reassembly: true,
            listen_backlog: false,
            mdns_responder: true,
            mem_size: 16000,
            mib2_stats: true,
            netconn: true,
            netconn_sem_per_thread: true,
            netif_ext_status_callback: true,
            num_arp_queue: 2,
            num_netbuf: 2,
            num_netconn: 32,
            num_pbuf: 16,
            num_raw_pcb: 4,
            num_reass_data: 1,
            num_tcp_pcb: 4,
            num_tcp_pcb_listen: 4,
            num_tcp_seg: 16,
            num_tcpip_msg_api: 8,
            num_tcpip_msg_inpkt: 8,
            num_timeouts: 8,
            num_udp_pcb: 4,
            pbuf_link_hlen: 16,
            pbuf_pool_bufsize: TCP_MSS + 40 + PBUF_LINK_HLEN,
            pbuf_pool_sz: 8,
            ppp: true,
            raw: true,
            snmp: false,
            so_reuse: true,
            socket: true,
            stats: true,
            tcp: true,
            tcp_rcv_scale: false,
            tcp_send_buf: 12 * DFLT_TCP_MSS,
            tcp_send_q_len: 40,
            tcp_wnd_scale: true,
            tcp_wnd_sz: 10 * DFLT_TCP_MSS,
            tcpip_core_locking: true,
            test_mode: true,
            udp: true,
        }
    }
}


#[derive(Debug,Clone,Default)]
pub struct LwipContext {
    pub netifs: Vec<NetworkInterface>,
    pub arp_table: ArpTable,
}

impl LwipContext {
    pub fn new() -> Self {
        Self {
            netifs: Vec::new(),
            arp_table: ArpTable::new(),
        }
    }
}
