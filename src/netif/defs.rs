use crate::core::defines::LwipAddr;
use crate::core::error::LwipError;
use crate::packetbuffer::pbuf_h::PacketBuffer;

pub const NETIF_REPORT_TYPE_IPV4: u32 = 0x01;
pub const NETIF_REPORT_TYPE_IPV6: u32 = 0x02;


// static netif_ext_callback_t *ext_callback;
// netif_list: &mut NetIfc;
// netif__ => &mut NetIfc;
// static netif_num: u8;
// static netif_client_id: u8;
// static NetIfc loop_netif;

// #define ENABLE_LOOPBACK (LWIP_NETIF_LOOPBACK || LWIP_HAVE_LOOPIF)
pub const ENABLE_LOOPBACK: bool = LWIP_NETIF_LOOPBACK || LWIP_HAVE_LOOPIF;

pub const NETIF_MAX_HWADDR_LEN: usize = 6;
pub const NETIF_NAMESIZE: usize = 6;
pub const NETIF_FLAG_UP: u32 = 0x01;
pub const NETIF_FLAG_BROADCAST: u32 = 0x02;
pub const NETIF_FLAG_LINK_UP: u32 = 0x04;
pub const NETIF_FLAG_ETHARP: u32 = 0x08;
pub const NETIF_FLAG_ETHERNET: u32 = 0x10;
pub const NETIF_FLAG_IGMP: u32 = 0x20;
pub const NETIF_FLAG_MLD6: u32 = 0x40;

pub enum LwipInternalNetifClientDataIndex {
    LwipNetifClientDataIndexDhcp,
    LwipNetifClientDataIndexAutoip,
    LwipNetifClientDataIndexIgmp,
    LwipNetifClientDataIndexDhcp6,
    LwipNetifClientDataIndexMld6,
    LwipNetifClientDataIndexMax,
}

pub const NETIF_CHECKSUM_GEN_IP: u32 = 0x0001;
pub const NETIF_CHECKSUM_GEN_UDP: u32 = 0x0002;
pub const NETIF_CHECKSUM_GEN_TCP: u32 = 0x0004;
pub const NETIF_CHECKSUM_GEN_ICMP: u32 = 0x0008;
pub const NETIF_CHECKSUM_GEN_ICMP6: u32 = 0x0010;
pub const NETIF_CHECKSUM_CHECK_IP: u32 = 0x0100;
pub const NETIF_CHECKSUM_CHECK_UDP: u32 = 0x0200;
pub const NETIF_CHECKSUM_CHECK_TCP: u32 = 0x0400;
pub const NETIF_CHECKSUM_CHECK_ICMP: u32 = 0x0800;
pub const NETIF_CHECKSUM_CHECK_ICMP6: u32 = 0x1000;
pub const NETIF_CHECKSUM_ENABLE_ALL: u32 = 0xFFFF;
pub const NETIF_CHECKSUM_DISABLE_ALL: u32 = 0x0000;

enum NetifMacFilterAction {
    //! MAC Filter Actions, these are passed to a netif's igmp_mac_filter or mld_mac_filter callback function.
    ///  Delete a filter entry
    NetifDelMacFilter = 0,
    ///  Add a filter entry
    NetifAddMacFilter = 1,
}

/// Function prototype for netif init functions. Set up flags and output/linkoutput callback functions in this function.
/// @param netif The netif to initialize
pub type NetifInitFn = fn(netif: &mut NetworkInterface) -> Result<(), LwipError>;
pub type NetifInputFn = fn(p: &mut PacketBuffer, inp: &mut NetworkInterface) -> Result<(), LwipError>;

/// Function prototype for netif.output functions. Called by lwIP when a packet shall be sent. For ethernet netif, set this to 'etharp_output' and set 'linkoutput'.
/// @param netif The netif which shall send a packet
/// @param p The packet to send (p.payload points to IP header)
/// @param ipaddr The IP address to which the packet shall be sent
pub type NetifOutputFn = fn(netif: &mut NetworkInterface, p: &mut PacketBuffer, ipaddr: &mut LwipAddr) -> Result<(), LwipError>;

/// Function prototype for netif.output_ip6 functions. Called by lwIP when a packet shall be sent. For ethernet netif, set this to 'ethip6_output' and set 'linkoutput'.
/// @param netif The netif which shall send a packet
/// @param p The packet to send (p.payload points to IP header)
/// @param ipaddr The IPv6 address to which the packet shall be sent
pub type NetifOutputIp6Fn = fn(netif: &mut NetworkInterface, p: &mut PacketBuffer, ipaddr: &mut ip6_addr_t) -> Result<(), LwipError>;

///  Function prototype for netif status- or link-callback functions.
pub type NetifStatusCallbackFn = fn(netif: &mut NetworkInterface);

///  Function prototype for netif igmp_mac_filter functions
pub type NetifIgmpMacFilterFn = fn(
    netif: &mut NetworkInterface,
    group: &mut LwipAddr,
    action: NetifMacFilterAction,
) -> Result<(), LwipError>;

///  Function prototype for netif mld_mac_filter functions
pub type NetifMldMacFilterFn = fn(
    netif: &mut NetworkInterface,
    group: &mut ip6_addr_t,
    action: NetifMacFilterAction,
) -> Result<(), LwipError>;


pub const NETIF_ADDR_IDX_MAX: u32 = 0x7FFF;

pub struct NetifHint {
    pub addr_hint: u16,
}

pub const LWIP_NETIF_USE_HINTS: u32 = 0;

#[derive(Debug, Clone, Default)]
pub struct NetworkInterface {
    //  pointer to next in linked list
    // next: &mut NetIfc;
    //  IP address configuration in network byte order
    pub ip_addr: LwipAddr,
    pub netmask: LwipAddr,
    pub gw: LwipAddr,
    //  Array of IPv6 addresses for this netif.
    // LwipAddr ip6_addr[LWIP_IPV6_NUM_ADDRESSES];
    pub ip6_addr: Vec<LwipAddr>,
    /* The state of each IPv6 address (Tentative, Preferred, etc).
     * @see ip6_addr.h */
    // ip6_addr_state: [u8;LWIP_IPV6_NUM_ADDRESSES];
    /* Remaining valid and preferred lifetime of each IPv6 address, in seconds.
     * For valid lifetimes, the special value of IP6_ADDR_LIFE_STATIC (0)
     * indicates the address is static and has no lifetimes. */
    // ip6_addr_valid_life: [u32;LWIP_IPV6_NUM_ADDRESSES];
    // ip6_addr_pref_life: [u32;LWIP_IPV6_NUM_ADDRESSES];
    /* This function is called by the network device driver
     *  to pass a packet up the TCP/IP stack. */
    pub input: NetifInputFn,
    /* This function is called by the IP module when it wants
     *  to send a packet on the interface. This function typically
     *  first resolves the hardware address, then sends the packet.
     *  For ethernet physical layer, this is usually etharp_output() */
    pub output: NetifOutputFn,
    /* This function is called by ethernet_output() when it wants
     *  to send a packet on the interface. This function outputs
     *  the pbuf as-is on the link medium. */
    pub linkoutput: netif_linkoutput_fn,
    /* This function is called by the IPv6 module when it wants
     *  to send a packet on the interface. This function typically
     *  first resolves the hardware address, then sends the packet.
     *  For ethernet physical layer, this is usually ethip6_output() */
    pub output_ip6: NetifOutputIp6Fn,
    /* This function is called when the netif state is set to up or down
     */
    pub status_callback: NetifStatusCallbackFn,
    /* This function is called when the netif link is set to up or down
     */
    pub link_callback: NetifStatusCallbackFn,
    //  This function is called when the netif has been removed
    pub remove_callback: NetifStatusCallbackFn,
    /* This field can be set by the device driver and could point
     *  to state information for the device. */
    pub state: Vec<u8>,
    pub client_data: Vec<u8>,
    //  the hostname for this netif, NULL is a valid value
    pub hostname: String,
    pub chksum_flags: u16,
    //  maximum transfer unit (in bytes)
    pub mtu: u16,
    //  maximum transfer unit (in bytes), updated by RA
    pub mtu6: u16,
    //  link level hardware address of this interface
    pub hwaddr: LwipAddr,
    //  number of bytes used in hwaddr
    //  flags (@see @ref netif_flags)
    pub flags: u8,
    //  descriptive abbreviation
    pub name: String,
    /* number of this interface. Used for @ref if_api and @ref netifapi_netif,
     * as well as for IPv6 zones */
    pub num: u64,
    //  is this netif enabled for IPv6 autoconfiguration
    pub ip6_autoconfig_enabled: bool,
    //  Number of Router Solicitation messages that remain to be sent.
    pub rs_count: usize,
    //  link type (from "snmp_ifType" snmp_mib2: from.h)
    pub link_type: u8,
    //  (estimate) link speed
    pub link_speed: u32,
    //  timestamp at last change made (up/down)
    pub ts: u64,
    //  counters
    pub mib2_counters: stats_mib2_netif_ctrs,
    /* This function could be called to add or delete an entry in the multicast
    filter table of the ethernet MAC.*/
    pub igmp_mac_filter: NetifIgmpMacFilterFn,
    /* This function could be called to add or delete an entry in the IPv6 multicast
    filter table of the ethernet MAC. */
    pub mld_mac_filter: NetifMldMacFilterFn,
    pub hints: Vec<NetifHint>,
    //  List of packets to be queued for ourselves.
    pub loop_first: PacketBuffer,
    pub loop_last: PacketBuffer,
    pub loop_cnt_current: u16,
}

impl NetworkInterface {
    pub fn netif_set_checksum_ctrl(&mut self, chksumflugs: u16) {
        self.chksum_flags = chksumflugs
    }

    pub fn checksum_enabled(&self, checksum_flag: u16) -> bool {
        self.chksum_flags & checksum_flag != 0
    }

    pub fn set_checksum_ctrl(&mut self, chksumflags: u16) {
        unimplemented!()
    }

    pub fn etharp_set_addrhint(&mut self, addrhint: &mut Vec<u8>) {
        netif.hints.addr_hint = addrhint;
    }
}

//  used for initialization only
pub const LWIP_NSC_NONE: u32 = 0x0000;
//  netif was added. arg: NULL. Called AFTER netif was added.
pub const LWIP_NSC_NETIF_ADDED: u32 = 0x0001;
//  netif was removed. arg: NULL. Called BEFORE netif is removed.
pub const LWIP_NSC_NETIF_REMOVED: u32 = 0x0002;
//  link changed
pub const LWIP_NSC_LINK_CHANGED: u32 = 0x0004;
pub const LWIP_NSC_STATUS_CHANGED: u32 = 0x0008;
//  IPv4 address has changed
pub const LWIP_NSC_IPV4_ADDRESS_CHANGED: u32 = 0x0010;
//  IPv4 gateway has changed
pub const LWIP_NSC_IPV4_GATEWAY_CHANGED: u32 = 0x0020;
//  IPv4 netmask has changed
pub const LWIP_NSC_IPV4_NETMASK_CHANGED: u32 = 0x0040;
//  called AFTER IPv4 address/gateway/netmask changes have been applied
pub const LWIP_NSC_IPV4_SETTINGS_CHANGED: u32 = 0x0080;
//  IPv6 address was added
pub const LWIP_NSC_IPV6_SET: u32 = 0x0100;
//  IPv6 address state has changed
pub const LWIP_NSC_IPV6_ADDR_STATE_CHANGED: u32 = 0x0200;

pub struct netif_ext_callback_args_t {
    pub state: u8,
    pub old_address: LwipAddr,
    pub old_netmask: LwipAddr,
    pub old_gw: LwipAddr,
    pub addr_index: usize,
}

// typedef void (*netif_ext_callback_fn)(netif: &mut NetIfc, netif_nsc_reason_t reason,  netif_ext_callback_args_t* args);
type netif_ext_callback_fn =
    fn(netif: &mut NetworkInterface, reason: netif_nsc_reason_t, args: &netif_ext_callback_args_t);

pub fn netif_set_flags(netif: &mut NetworkInterface, set_flags: u8) {
    netif.flags = netif.flags | set_flags;
}

pub fn netif_clear_flags(netif: &mut NetworkInterface, clr_flags: u8) {
    (netif).flags = ((netif).flags & (!(clr_flags) & 0xff));
}

pub fn netif_is_flag_set(nefif: &NetworkInterface, flag: u8) -> bool {
    ((netif.flags & (flag)) != 0)
}

pub fn netif_is_up(netif: &NetworkInterface) -> bool {
    netif.flags & NETIF_FLAG_UP > 0
}

// pub fn  netif_set_link_up(netif: &mut NetIfc);
// pub fn  netif_set_link_down(netif: &mut NetIfc);
//  Ask if a link is up
pub fn netif_is_link_up(netif: &NetworkInterface) -> bool {
    netif.flags & NETIF_FLAG_LINK_UP > 0
}
