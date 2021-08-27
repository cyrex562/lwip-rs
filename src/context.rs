pub struct LwipContext {
    pub current_netif: NetIfc,
    pub current_ip4_header: ip4_hdr,
    pub current_ip6_header: ip6_hdr,
    pub current_ip_header_to_len: usize,
    pub current_iphdr_src: LwipAddr,
    pub current_iphdr_dst: LwipAddr,
}

impl LwipContext {}

// ip_globals
// {
//   /* The interface that accepted the packet for the current callback invocation. */
//   current_netif: &mut NetIfc;
//   /* The interface that received the packet for the current callback invocation. */
//   current_input_netif: &mut NetIfc;

//   /* Header of the input packet currently being processed. */
//   const current_ip4_header: &mut ip_hdr;

//   /* Header of the input IPv6 packet currently being processed. */
//   current_ip6_header: &mut ip6_hdr;

//   /* Total header length of current_ip4/6_header (i.e. after this, the UDP/TCP header starts) */
//   let current_ip_header_tot_len: u16;
//   /* Source IP address of current_header */
//   let current_iphdr_src: LwipAddr;
//   /* Destination IP address of current_header */
//   let current_iphdr_dest: LwipAddr;
// };
// extern struct ip_globals ip_data;
