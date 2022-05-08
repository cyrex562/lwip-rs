use crate::errors::{LwipError, LwipErrorCode};
use crate::errors::LwipErrorCode::{InvalidArgument, NotSet};
use crate::ip::ip_input;
use crate::ip_address::{IpAddress, IPV4_ADDR_ANY};
use crate::netif::network_interface::NetworkInterface;
use crate::network_interface;
use crate::network_interface::{LWIP_NSC_IPV4_ADDRESS_CHANGED, LWIP_NSC_IPV4_GATEWAY_CHANGED, LWIP_NSC_IPV4_NETMASK_CHANGED, LWIP_NSC_IPV4_SETTINGS_CHANGED, LWIP_NSC_LINK_CHANGED, LWIP_NSC_NETIF_ADDED, LWIP_NSC_NETIF_REMOVED, LWIP_NSC_NONE, LWIP_NSC_STATUS_CHANGED, NETIF_CHECKSUM_DISABLE_ALL, NETIF_CHECKSUM_ENABLE_ALL, NETIF_FLAG_IGMP, NETIF_REPORT_TYPE_IPV4, NETIF_REPORT_TYPE_IPV6, NetifExtCallbackArgs, NetworkInterface};
use crate::packet_buffer::PacketBuffer;





/// Forwards a received packet for input processing with ethernet_input() or ip_input()
/// depending on netif flags. Don't call directly, pass to netif_add() and call
///  netif.input(). Only works if the netif driver correctly sets NETIF_FLAG_ETHARP and/or
/// NETIF_FLAG_ETHERNET flag!
pub fn netif_input(packet: &mut PacketBuffer, in_netif: &mut NetworkInterface) -> Result<(), LwipError> {
    // LWIP_ASSERT_CORE_LOCKED()

    // LWIP_ASSERT("netif_input: invalid pbuf", p != NULL);
    // LWIP_ASSERT("netif_input: invalid netif", inp != NULL);
    if in_netif.etharp && in_netif.ethernet {
        return ethernet_input(packet, in_netif);
    } else{
        return ip_input(packet, in_netif);
    }
}

pub fn netif_init() {

// #define LOOPIF_ADDRINIT &loop_ipaddr, &loop_netmask, &loop_gw,
//   ip4_addr_t loop_ipaddr, loop_netmask, loop_gw;
//   IP4_ADDR(&loop_gw, 127, 0, 0, 1);
//   IP4_ADDR(&loop_ipaddr, 127, 0, 0, 1);
//   IP4_ADDR(&loop_netmask, 255, 0, 0, 0);

// #if NO_SYS
    netif_add(&loop_netif, LOOPIF_ADDRINIT, NULL, netif_loopif_init, ip_input);
// #else  /* NO_SYS */
    netif_add(&loop_netif, LOOPIF_ADDRINIT, NULL, netif_loopif_init, tcpip_input);
// #endif /* NO_SYS */

    IP_ADDR6_HOST(loop_netif.ip6_addr, 0, 0, 0, 0x00000001);
    loop_netif.ip6_addr_state[0] = IP6_ADDR_VALID;

    netif_set_link_up(&loop_netif);
    netif_set_up(&loop_netif);
}

/// Initialize a lwip network interface structure for a loopback interface
/// Returns ERR_OK if the loopif is initialized; ERR_MEM  if private data couldn't be
/// allocated
///
/// # Arguments
/// * netif - the lwip network interface structure for this loopif
///
pub fn netif_loopif_init(netif: &mut NetworkInterface, if_name: &String) -> Result<(), LwipError> {
    MIB2_INIT_NETIF(netif, snmp_ifType_softwareLoopback, 0);
    netif.name = if_name.clone();
    netif.output = netif_loop_output_ipv4;
    netif.output_ip6 = netif_loop_output_ipv6;

    netif_set_flags(netif, NETIF_FLAG_IGMP);

    netif_set_checksum(netif, NETIF_CHECKSUM_DISABLE_ALL as u16);
    return ERR_OK;
}

// #if LWIP_CHECKSUM_CTRL_PER_NETIF
// #define NETIF_SET_CHECKSUM_CTRL(netif, chksumflags) do { \
//   (netif)->chksum_flags = chksumflags; } while(0)
pub fn netif_set_checksum(netif: &mut NetworkInterface, checksum_flags: u16) {
    netif.chksum_flags = checksum_flags;
}

//#define NETIF_CHECKSUM_ENABLED(netif, chksumflag) (((netif) == NULL) || (((netif)->chksum_flags & (chksumflag)) != 0))
pub fn netif_checksum_enabled(netif: &NetworkInterface, checksum_flag: u16) -> bool {
    netif.chksum_flags & checksum_flag != 0
}

// #if LWIP_SINGLE_NETIF
// #define NETIF_FOREACH(netif) if (((netif) = netif_default) != NULL)
// #else /* LWIP_SINGLE_NETIF */
/** The list of network interfaces. */
// extern struct netif *netif_list;
// #define NETIF_FOREACH(netif) for ((netif) = netif_list; (netif) != NULL; (netif) = (netif)->next)
 /* LWIP_SINGLE_NETIF */
// e default network interface. */
// extern struct netif *netif_default;

// void netif_init();

// struct netif *netif_add_noaddr(struct netif *netif, void *state, netif_init_fn init, netif_input_fn input);


// struct netif *netif_add(struct netif *netif,
//                             const ip4_addr_t *ipaddr, const ip4_addr_t *netmask, const ip4_addr_t *gw,
//                             void *state, netif_init_fn init, netif_input_fn input);
// void netif_set_addr(struct netif *netif, const ip4_addr_t *ipaddr, const ip4_addr_t *netmask,
//                     const ip4_addr_t *gw);
// #else /* LWIP_IPV4 */
// struct netif *netif_add(struct netif *netif, void *state, netif_init_fn init, netif_input_fn input);
 /* LWIP_IPV4 */
// etif_remove(struct netif * netif);

// /* Returns a network interface given its name. The name is of the form
//    "et0", where the first two letters are the "name" field in the
//    netif structure, and the digit is in the num field in the same
//    structure. */
// struct netif *netif_find(const char *name);

// void netif_set_default(struct netif *netif);


// void netif_set_ipaddr(struct netif *netif, const ip4_addr_t *ipaddr);
// void netif_set_netmask(struct netif *netif, const ip4_addr_t *netmask);
// void netif_set_gw(struct netif *netif, const ip4_addr_t *gw);
// /** @ingroup netif_ip4 */
// #define netif_ip4_addr(netif)    ((const ip4_addr_t*)ip_2_ip4(&((netif)->ip_addr)))


// /** @ingroup netif_ip4 */
// #define netif_ip4_netmask(netif) ((const ip4_addr_t*)ip_2_ip4(&((netif)->netmask)))
// /** @ingroup netif_ip4 */
// #define netif_ip4_gw(netif)      ((const ip4_addr_t*)ip_2_ip4(&((netif)->gw)))
// /** @ingroup netif_ip4 */
// #define netif_ip_addr4(netif)    ((const ip_addr_t*)&((netif)->ip_addr))
// /** @ingroup netif_ip4 */
// #define netif_ip_netmask4(netif) ((const ip_addr_t*)&((netif)->netmask))
// /** @ingroup netif_ip4 */
// #define netif_ip_gw4(netif)      ((const ip_addr_t*)&((netif)->gw))
//  /* LWIP_IPV4 */

// #define netif_set_flags(netif, set_flags)     do { (netif)->flags = ((netif)->flags |  (set_flags)); } while(0)
pub fn netif_set_flags(netif: &mut NetworkInterface, set_flags: u8) {
    netif.flags |= set_flags;
}

// #define netif_clear_flags(netif, clr_flags)   do { (netif)->flags = ((netif)->flags & (~(clr_flags) & 0xff)); } while(0)
pub fn netif_clear_flags(netif: &mut NetworkInterface, clr_flags: u8) {
    netif.flags &= (!clr_flags & 0xff)
}

// #define netif_is_flag_set(netif, flag)        (((netif)->flags & (flag)) != 0)
pub fn netif_is_flag_set(netif: &NetworkInterface, flag: u8) -> bool {
    netif.flags & flag != 0
}

/// Add a network interface to the list of lwIP netifs.
pub fn netif_add_noaddr(netif: &mut NetworkInterface, state: &Vec<u8>, init: netif_init_fn, input: netif_input_fn) -> Result<(), LwipError>
{
  return netif_add(netif, None, None, None, state, init, input);
}

/**
 * @ingroup netif
 * Add a network interface to the list of lwIP netifs.
 *
 * @param netif a pre-allocated netif structure
 * @param ipaddr IP address for the new netif
 * @param netmask network mask for the new netif
 * @param gw default gateway IP address for the new netif
 * @param state opaque data passed to the new netif
 * @param init callback function that initializes the interface
 * @param input callback function that is called to pass
 * ingress packets up in the protocol layer stack.<br>
 * It is recommended to use a function that passes the input directly
 * to the stack (netif_input(), NO_SYS=1 mode) or via sending a
 * message to TCPIP thread (tcpip_input(), NO_SYS=0 mode).<br>
 * These functions use netif flags NETIF_FLAG_ETHARP and NETIF_FLAG_ETHERNET
 * to decide whether to forward to ethernet_input() or ip_input().
 * In other words, the functions only work when the netif
 * driver is implemented correctly!<br>
 * Most members of struct netif should be be initialized by the
 * netif init function = netif driver (init parameter of this function).<br>
 * IPv6: Don't forget to call netif_create_ip6_linklocal_address() after
 * setting the MAC address in struct netif.hwaddr
 * (IPv6 requires a link-local address).
 *
 * @return netif, or NULL if failed.
 */
pub fn netif_add(netif: &mut NetworkInterface,
                 ip4_addr: Option<IpAddress>,
                 ip4_netmask: Option<IpAddress>,
                 ip4_gw: Option<IpAddress>,
                 state: &Vec<u8>,
                 init_fn: netif_init_fn,
                 input_fn: netif_input_fn) -> Result<(), LwipError> {
    /* reset new interface configuration state */
    ip_addr_set_zero_ip4(&netif.ip_addr);
    ip_addr_set_zero_ip4(&netif.netmask);
    ip_addr_set_zero_ip4(&netif.gw);
    netif.output = netif_null_output_ip4;

    // for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i++) {
    //   ip_addr_set_zero_ip6(& netif.ip6_addr[i]);
    //    netif.ip6_addr_state[i] = IP6_ADDR_INVALID;
    //
    //    netif.ip6_addr_valid_life[i] = IP6_ADDR_LIFE_STATIC;
    //    netif.ip6_addr_pref_life[i] = IP6_ADDR_LIFE_STATIC;
    //
    // }
    netif.output_ip6 = netif_null_output_ip6;

    NETIF_SET_CHECKSUM_CTRL(netif, NETIF_CHECKSUM_ENABLE_ALL);
    netif.mtu = 0;
    netif.flags = 0;


    /* IPv6 address autoconfiguration should be enabled by default */
    netif.ip6_autoconfig_enabled = true;

    nd6_restart_netif(netif);


    netif.status_callback = None;


    netif.link_callback = None;


    netif.igmp_mac_filter = None;


    netif.mld_mac_filter = None;

    /* remember netif specific state information data */
    netif.state = state;
    netif.num = netif_num;
    netif.input = input;

// #if LWIP_ACD
//    netif.acd_list = NULL;
// #endif /* LWIP_ACD */
    NETIF_RESET_HINTS(netif);
// #if ENABLE_LOOPBACK
//    netif.loop_first = NULL;
//    netif.loop_last = NULL;
// #if LWIP_LOOPBACK_MAX_PBUFS
    netif.loop_cnt_current = 0;
// #endif /* LWIP_LOOPBACK_MAX_PBUFS */
// #if LWIP_NETIF_LOOPBACK_MULTITHREADING
    netif.reschedule_poll = false;
// #endif /* LWIP_NETIF_LOOPBACK_MULTITHREADING */
// #endif /* ENABLE_LOOPBACK */

// #if LWIP_IPV4
    netif_set_addr(netif, ipaddr, netmask, gw);
// #endif /* LWIP_IPV4 */

    /* call user specified initialization function for netif */
    if init_fn(netif).is_ok() {
        return Ok(());
    }
// #if LWIP_IPV6 && LWIP_ND6_ALLOW_RA_UPDATES
    /* Initialize the MTU for IPv6 to the one set by the netif driver.
       This can be updated later by RA. */
    netif.mtu6 = netif.mtu;

    // TODO: assign netif unique id or unused id
    // TODO: set neet and prev ids
    mib2_netif_added(netif);

// #if LWIP_IGMP
    /* start IGMP processing */
    if netif.igmp {
        igmp_start(netif);
    }

    debug!("added interface {}", &netif.name);

    netif_invoke_ext_callback(netif, LWIP_NSC_NETIF_ADDED, NULL);

    Ok(())
}

pub fn netif_do_ip_addr_changed(old_addr: &IpAddress, new_addr: &IpAddress)
{
  tcp_netif_ip_addr_changed(old_addr, new_addr);
  udp_netif_ip_addr_changed(old_addr, new_addr);
  raw_netif_ip_addr_changed(old_addr, new_addr);
}


pub fn netif_do_set_ipaddr(netif: &mut NetworkInterface, new_addr: &IpAddress, old_addr: &IpAddress) -> Result<(), LwipError>
{
    if new_addr == old_addr || new_addr == netif.ip4_address_config.address {
        return Err(LwipError::new(LwipErrorCode::NotSet, "address to set is the same as the current address"));
    }

    netif.ip4_address_config.address = new_addr.clone();

    debug!("netif address being changed");
    netif_do_ip_addr_changed(old_addr, &new_addr);

    acd_netif_ip_addr_changed(netif, old_addr, &new_addr);

    mib2_remove_ip4(netif);
    mib2_remove_route_ip4(0, netif);
    mib2_add_ip4(netif);
    mib2_add_route_ip4(0, netif);
    netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV4);

    NETIF_STATUS_CALLBACK(netif);
    Ok(())
}

/**
 * @ingroup netif_ip4
 * Change the IP address of a network interface
 *
 * @param netif the network interface to change
 * @param ipaddr the new IP address
 *
 * @note call netif_set_addr() if you also want to change netmask and
 * default gateway
 */
pub fn netif_set_ipaddr(netif: &mut NetworkInterface, ipaddr: &IpAddress) {
    // LWIP_ASSERT_CORE_LOCKED()

    if netif_do_set_ipaddr(netif, ipaddr, &old_addr) {
        let mut args = NetifExtCallbackArgs::default();
        args.old_address = old_addr.clone();
        netif_invoke_ext_callback(netif, LWIP_NSC_IPV4_ADDRESS_CHANGED, &args);
    }
}

pub fn netif_do_set_netmask(netif: &mut NetworkInterface, netmask: &IpAddress, old_netmask: &mut IpAddress) -> Result<(), LwipError> {
    if netmask == netif.ip4_address_config.netmask {
        return Err(LwipError::new(NotSet, "new netmask is the same as the existing netmask"));
    }
    *old_netmask = netif.ip4_address_config.netmask.clone();
    netif.ip4_address_config.netmask = netmask.clone();
    mib2_remove_route_ip4(0, netif);
    /* set new netmask to netif */
    ip4_addr_set(ip_2_ip4(&netif.netmask), netmask);
    IP_SET_TYPE_VAL(netif.netmask, IPADDR_TYPE_V4);
    mib2_add_route_ip4(0, netif);
    debug!("netmask of interface {} set to {}", &netif.name, &netif.ip4_address_config.netmask);
    Ok(())
}

/**
 * @ingroup netif_ip4
 * Change the netmask of a network interface
 *
 * @param netif the network interface to change
 * @param netmask the new netmask
 *
 * @note call netif_set_addr() if you also want to change ip address and
 * default gateway
 */
pub fn netif_set_netmask(netif: &mut NetworkInterface, netmask: &IpAddress) {
    let mut old_nm = IpAddress::new();
    if netif_do_set_netmask(netif, netmask, &mut old_nm).is_ok() {
        let mut args = NetifExtCallbackArgs::new();
        args.old_netmask = old_nm;
        netif_invoke_ext_callback(netif, LWIP_NSC_IPV4_NETMASK_CHANGED, &args);
    }
}

pub fn netif_do_set_gw(netif: &mut NetworkInterface, new_gw: &IpAddress, old_gw: &mut IpAddress) -> Result<(), LwipError> {
    /* address is actually being changed? */
    if new_gw == netif.ip4_address_config.gateway {
        return Err(LwipError::new(InvalidArgument, "new gateway is the same as the current gateway"));
    }

    *old_gw = netif.ip4_address_config.gateway.clone();
    netif.ip4_address_config.gateway = new_gw.clone();

    debug!("set gateway address of netif {} to {}", &netif.name, &netif.ip4_address_config.gateway);
    Ok(())
}

/// Change the default gateway for a network interface
pub fn netif_set_gw(netif: &mut NetworkInterface, gw: &IpAddress) {
    let mut old_gw = IpAddress: new();
    if netif_do_set_gw(netif, gw, &mut old_gw).is_ok() {
        let mut args = NetifExtCallbackArgs { old_gateway: old_gw.clone(), ..Default::default() };
        netif_invoke_ext_callback(netif, LWIP_NSC_IPV4_GATEWAY_CHANGED, &args);
    }
}

/// Set IPv4 Address configuration for network interface
pub fn netif_set_addr(netif: &mut NetworkInterface, address: &IpAddress, netmask: &IpAddress, gateway: &IpAddress) -> Result<(), LwipError> {
    let mut change_reasons: Vec<u32> = Vec::new();
    change_reasons[0] = LWIP_NSC_NONE;
    let mut cb_args = NetifExtCallbackArgs::new();
    let mut old_nm = IpAddress::new();
    let mut old_addr = IpAddress::new();
    let mut old_gw = IPAddress::new();

    if netif_do_set_ipaddr(netif, address, &mut old_addr).is_ok() {
        change_reasons[0] = LWIP_NSC_IPV4_ADDRESS_CHANGED;
        cb_args.old_address = old_addr.clone();
    }
    if netif_do_set_netmask(netif, netmask, &mut old_nm).is_ok() {
        cb_args.old_netmask = old_nm.clone();
        change_reasons.push(LWIP_NSC_IPV4_NETMASK_CHANGED);
    }
    if netif_do_set_gw(netif, gw, &mut old_gw).is_ok() {
        cb_args.old_gateway = old_gw;
        change_reasons.push(LWIP_NSC_IPV4_GATEWAY_CHANGED);
    }

    if !change_reasons.contains(&LWIP_NSC_NONE) {
        change_reasons.push(LWIP_NSC_IPV4_SETTINGS_CHANGED);
    } else {
        netif_invoke_ext_callback(netif, &change_reasons, &cb_args);
    }

    Ok(())
}

pub fn netif_remove(netif: &mut NetworkInterface) -> Result<(), LwipError> {
    let mut reason: Vec<u32> = Vec::new();
    reason.push(LWIP_NSC_NETIF_REMOVED);
    let mut args = NetifExtCallbackArgs::new();
    netif_invoke_ext_callback(netif, &reason, &args);
    if netif.igmp {
        igmp_stop(netif);
    }

    if netif.mld6 {
        mld6_stop(netif);
    }

    netif_set_down(netif);

    mib2_remove_ip4(netif);

    netif.remove_callback(netif);

    // TODO: if designated default, then set another interface ot the default, or make the default empty
    // TODO: remove netif from the list and fix up next/prev indexes

    debug!("removed netif");

    Ok(())
}


pub fn netif_set_default(netif: &mut NetworkInterface)
{
  // LWIP_ASSERT_CORE_LOCKED()
    mib2_add_route_ip4(1, netif);
    // TODO: set this netif as the default in a global context somewhere
    debug!("set default netif to {}", &netif.name);
}


pub fn netif_set_up(netif: &mut NetworkInterface)
{
    netif.link_up = true;
    MIB2_COPY_SYSUPTIME_TO(& netif.ts);
    NETIF_STATUS_CALLBACK(netif);
    let args = NetifExtCallbackArgs {state: 1, ..Default::default()};
    netif_invoke_ext_callback(netif, LWIP_NSC_STATUS_CHANGED, &args);
    netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV4 | NETIF_REPORT_TYPE_IPV6);
    nd6_restart_netif(netif);
}

pub fn netif_issue_reports(netif: &mut NetworkInterface, report_type: u8) {
    /* Only send reports when both link and admin states are up */
    if !netif.link_up || !netif.up {
        return;
    }
    if report_type == NETIF_REPORT_TYPE_IPV4 && netif.ip4_address_config.address != IPV4_ADDR_ANY {
        if netif.etharp {
            etharp_gratuitous(netif);
        }
        if netif.igmp {
            igmp_report_groups(netif);
        }
    }

    if report_type == NETIF_REPORT_TYPE_IPV6 {
        mld6_report_groups(netif);
    }
}

pub fn netif_set_down(netif: &mut NetworkInterface) {
    let args = NetifExtCallbackArgs { state: 0, ..Default::default() };
    netif_invoke_ext_callback(netif, LWIP_NSC_STATUS_CHANGED, &args);
    netif.up = false;
    MIB2_COPY_SYSUPTIME_TO(&netif.ts);

    if netif.etharp {
        etharp_cleanup_netif(netif);
    }
    nd6_cleanup_netif(netif);

    NETIF_STATUS_CALLBACK(netif);
}

pub fn netif_set_link_up(netif: &mut NetworkInterface) {
    netif.link_up = True;
    dhcp_network_changed_link_up(netif);
    autoip_network_changed_link_up(netif);
    netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV4 | NETIF_REPORT_TYPE_IPV6);
    nd6_restart_netif(netif);
    NETIF_LINK_CALLBACK(netif);
    let args = NetifExtCallbackArgs { state: 1, ..Default::default() };
    netif_invoke_ext_callback(netif, LWIP_NSC_LINK_CHANGED, &args);
}

pub fn netif_set_link_down(netif: &mut NetworkInterface) {
    if netif.link_up {
        netif.link_up = false;
        autoip_network_changed_link_down(netif);
        acd_network_changed_link_down(netif);

        netif.mtu6 = netif.mtu;

        NETIF_LINK_CALLBACK(netif);
        let args = NetifExtCallbackArgs { state: 0, ..Default::default() };
        netif_invoke_ext_callback(netif, LWIP_NSC_LINK_CHANGED, &args);
    }
}

pub fn netif_loop_output(netif: &mut NetworkInterface, p: &PacketBuffer) -> Result<(), LwipError> {
    let mut r = PacketBuffer::new();
    let mut last: &mut PacketBuffer;
    let mut clen = 0usize;
    let mut schedule_pool = false;

    r = p.clone();

    schedule_poll = true;

    LINK_STATS_INC(link.xmit);
    MIB2_STATS_NETIF_ADD(stats_if, ifoutoctets, p.tot_len);
    MIB2_STATS_NETIF_INC(stats_if, ifoutucastpkts);

    if schedule_poll {
        if tcpip_try_callback(netif_poll, netif) != ERR_OK {
            SYS_ARCH_PROTECT(lev);
            netif.reschedule_poll = true;
            SYS_ARCH_UNPROTECT(lev);
        }
    }


    Ok(())
}

pub fn netif_loop_output_ipv4(netif: &mut NetworkInterface, p: &PacketBuffer) -> Result<(), LwipError> {
    netif_loop_output(netif, p)
}

pub fn netif_loop_output_ipv6(netif: &mut NetworkInterface, p: &PacketBuffer) -> Result<(), LwipError> {
    netif_loop_output(netif, p)
}


pub fn netif_poll(netif: &mut NetworkInterface) {
    // TODO: get packet from list
    let mut in_pkt = PacketBuffer::new();
    LINK_STATS_INC(link.recv);
    MIB2_STATS_NETIF_ADD(stats_if, ifinoctets, in_pkt.tot_len);
    MIB2_STATS_NETIF_INC(stats_if, ifinucastpkts);
    ip_input(&in_pkt, netif);
}

pub fn netif_poll_all()
{
  // TODO: iterate over netif list and call poll for each
}
