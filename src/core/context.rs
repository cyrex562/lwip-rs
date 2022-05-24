use crate::arp::arp_table::ArpTable;
use crate::core::hosts::HostsList;
use crate::timers::timeouts::LwipCyclicHandler;
use crate::netif::netif::NetworkInterface;
use crate::timers::defines::LwipCyclicHandler;

#[derive(Default,Debug,Clone)]
pub struct Options {
    pub test_mode: bool,
    pub checksum_on_copy: bool,
    pub netconn_sem_per_thread: bool,
    /// the number of memp struct pbufs
    pub mem_size: usize,
    pub tcp_send_q_len: usize,
    pub tcp_send_buf: usize,
    pub tcp_wnd: usie,
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
    // ipv4 support
    pub ipv4: bool,
    // ipv6 support
    pub ipv6: bool,
    // netif api enabled
    pub netif_api: bool,
    // snmpv3 support
    pub snmpv3: bool,
    // netif client data
    pub num_netif_client_data: usize,
    // maximum loopback pbufs
    pub loopback_max_pbufs: usize,
    //
    pub tcp_listen_backlog: bool,
    //
    pub compat_sockets: bool,
    //
    pub so_rcv_buf: bool,
    //
    pub netif_status_callback: bool,
    //
    pub pool_size: usize,
    //
    pub pool_buf_sz: usize,
    //
    pub al_tcp: bool,
    //
    pub tcp_ttl: u8,
    //
    pub al_tcp_tls: bool,
    //
    pub tcp_queue_ooseq: bool,
    //
    pub tcp_mss: usize,
    //
    pub tcp_snd_queue_len: usize,
    // tcp writeable space in bytes; must be LTE TCP_SND_BUF. amount of space that must be available in the tcp snd buf for select to return writable
    pub tcp_snd_low_at: usize,
    // tcp max retransmit
    pub tcp_max_rtx: usize,
    // tcp max SYN retransmit
    pub tcp_max_syn_rtx: usize,
    //
    pub icmp_ttl: u8,
    // doe arp check on offered DHCP address
    pub dhcp_does_arp_check: bool,
    //
    pub dhcp_autoip_coop: bool,
    //
    pub udp_lite: bool,
    //
    pub udp_ttl: usize,
    //
    pub netbios_respond_name_query: bool,
    // max PPP sessions
    pub num_ppp: usize,
    //
    pub pppoe_support: bool,
    //
    pub pppos_support: bool,
    //
    pub pap_support: bool,
    //
    pub chap_support: bool,
    //
    pub mschap_support: bool,
    //
    pub cbcp_support: bool,
    //
    pub ccp_support: bool,
    //
    pub vj_support: bool,
    //
    pub md5_support: bool,
    //
    pub netconn_full_duplex: bool,
    //
    pub ipv6_frag_copyheader: bool,
    //
    pub ipv6_dup_detect_attempts: bool,
    //
    pub tcp_oversize: bool,
    //
    pub tcp_timer_interval: i64,
}

impl Options {
    pub fn new() -> Self {
        Self {

            al_tcp: true,
            al_tcp_tls: true,
            arp: true,
            arp_spt_static_entries: true,
            autoip: false,
            checksum_on_copy: true,
            compat_sockets: true,
            dhcp: true,
            dhcp_autoip_coop: true,
            dhcp_does_arp_check: true,
            dns: true,
            have_loopif: true,
            icmp: true,
            icmp_ttl: 255,
            igmp: true,
            ip_dflt_ttl: 255,
            ip_forward: true,
            ip_frag: true,
            ip_frag_uses_static_pbuf: false,
            ip_options_allowed: true,
            ip_reass_max_pbufs: 4,
            ip_reass_maxage: 3,
            ip_reassembly: true,
            ipv4: true,
            ipv6: true,
            listen_backlog: false,
            loopback_max_pbufs: 10,
            mdns_responder: true,
            mem_size: 16000,
            mib2_stats: true,
            netbios_respond_name_query: true,
            netconn: true,
            netconn_sem_per_thread: true,
            netif_api: true,
            netif_ext_status_callback: true,
            netif_status_callback: true,
            num_arp_queue: 2,
            num_netbuf: 2,
            num_netconn: 32,
            num_netif_client_data: 16,
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
            pool_buf_sz: 256,
            pool_size: 120,
            ppp: true,
            raw: true,
            snmp: false,
            snmpv3: true,
            so_rcv_buf: true,
            so_reuse: true,
            socket: true,
            stats: true,
            tcp: true,
            tcp_listen_backlog: true,
            tcp_max_rtx: 12,
            tcp_max_syn_rtx: 4,
            tcp_mss: 1024,
            tcp_queue_ooseq: true,
            tcp_rcv_scale: false,
            tcp_send_buf: 2048,
            tcp_send_q_len: 40,
            tcp_snd_low_at: TCP_SND_BUF / 2,
            tcp_snd_queue_len: 4 * TCP_SND_BUF / TCP_MSS,
            tcp_ttl: 255,
            tcp_wnd: 20 * 1024,
            tcp_wnd_scale: true,
            tcpip_core_locking: true,
            test_mode: true,
            udp: true,
            udp_lite: true,
            udp_ttl: 255,
            num_ppp: 1,
            pppoe_support: true,
            pppos_support: true,
            pap_support: true,
            chap_support: true,
            mschap_support: false,
            cbcp_support: false,
            ccp_support: false,
            vj_support: false,
            md5_support: true,
            netconn_full_duplex: true,
            ipv6_frag_copyheader: true,
            ipv6_dup_detect_attempts: false,
            tcp_oversize: true,
            tcp_timer_interval: 1000,
        }
    }
}


#[derive(Debug,Clone,Default)]
pub struct LwipContext {
    pub netifs: Vec<NetworkInterface>,
    pub arp_table: ArpTable,
    pub options: Options,
    pub tcpip_tcp_timer_active: bool,
    pub tcp_active_pcbs: Vec<TcpPcb>,
    pub tcp_tw_pcbs: Vec<TcpPcb>,
    pub cyclic_timer_handlers: Vec<LwipCyclicHandler>,
    pub hosts_table: HostsList,
}

impl LwipContext {
    pub fn new() -> Self {
        Self {
            netifs: Vec::new(),
            arp_table: ArpTable::new(),
            options: Options::new(),
            tcpip_tcp_timer_active: false,
            tcp_active_pcbs: vec![],
            tcp_tw_pcbs: vec![],
            cyclic_timer_handlers: vec![],
            hosts_table: HostsList::new(),
        }
    }
}
