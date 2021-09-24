/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Adam Dunkels <adam@sics.se>
 *
 */

// #define LWIP_HDR_NETIF_H

use crate::core::opt_h::LWIP_NETIF_LOOPBACK;

// #define ENABLE_LOOPBACK (LWIP_NETIF_LOOPBACK || LWIP_HAVE_LOOPIF)
pub const ENABLE_LOOPBACK: bool = LWIP_NETIF_LOOPBACK || LWIP_HAVE_LOOPIF;

/* Throughout this file, IP addresses are expected to be in
 * the same byte order as in IP_PCB. */

/* Must be the maximum of all used hardware address lengths
across all types of interfaces in use.
This does not have to be changed, normally. */

pub const NETIF_MAX_HWADDR_LEN: usize = 6;

/* The size of a fully constructed netif name which the
 * netif can be identified by in APIs. Composed of
 * 2 chars, 3 (max) digits, and 1 \0
 */
pub const NETIF_NAMESIZE: usize = 6;

/*
 * @defgroup netif_flags Flags
 * @ingroup netif
 * @{
 */

/* Whether the network interface is 'up'. This is
 * a software flag used to control whether this network
 * interface is enabled and processes traffic.
 * It must be set by the startup code before this netif can be used
 * (also for dhcp/autoip).
 */
pub const NETIF_FLAG_UP: u32 = 0x01;
/* If set, the netif has broadcast capability.
 * Set by the netif driver in its init function. */
pub const NETIF_FLAG_BROADCAST: u32 = 0x02;
/* If set, the interface has an active link
 *  (set by the network interface driver).
 * Either set by the netif driver in its init function (if the link
 * is up at that time) or at a later poonce: i32 the link comes up
 * (if link detection is supported by the hardware). */
pub const NETIF_FLAG_LINK_UP: u32 = 0x04;
/* If set, the netif is an ethernet device using ARP.
 * Set by the netif driver in its init function.
 * Used to check input packet types and use of DHCP. */
pub const NETIF_FLAG_ETHARP: u32 = 0x08;
/* If set, the netif is an ethernet device. It might not use
 * ARP or TCP/IP if it is used for PPPoE only.
 */
pub const NETIF_FLAG_ETHERNET: u32 = 0x10;
/* If set, the netif has IGMP capability.
 * Set by the netif driver in its init function. */
pub const NETIF_FLAG_IGMP: u32 = 0x20;
/* If set, the netif has MLD6 capability.
 * Set by the netif driver in its init function. */
pub const NETIF_FLAG_MLD6: u32 = 0x40;

/*
 * @}
 */

enum lwip_internal_netif_client_data_index {
    LWIP_NETIF_CLIENT_DATA_INDEX_DHCP,

    LWIP_NETIF_CLIENT_DATA_INDEX_AUTOIP,

    LWIP_NETIF_CLIENT_DATA_INDEX_IGMP,

    LWIP_NETIF_CLIENT_DATA_INDEX_DHCP6,

    LWIP_NETIF_CLIENT_DATA_INDEX_MLD6,

    LWIP_NETIF_CLIENT_DATA_INDEX_MAX,
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

/* MAC Filter Actions, these are passed to a netif's igmp_mac_filter or
 * mld_mac_filter callback function. */
enum netif_mac_filter_action {
    /* Delete a filter entry */
    NETIF_DEL_MAC_FILTER = 0,
    /* Add a filter entry */
    NETIF_ADD_MAC_FILTER = 1,
}

/* Function prototype for netif init functions. Set up flags and output/linkoutput
 * callback functions in this function.
 *
 * @param netif The netif to initialize
 */
// typedef err_t (*netif_init_fn)(netif: &mut NetIfc);

pub type netif_init_fn = fn(netif: &mut NetIfc) -> Result<(), LwipError>;
/* Function prototype for netif.input functions. This function is saved as 'input'
 * callback function in the netif struct. Call it when a packet has been received.
 *
 * @param p The received packet, copied into a pbuf
 * @param inp The netif which received the packet
 * @return ERR_OK if the packet was handled
 *         != ERR_OK is the packet was NOT handled, in this case, the caller has
 *                   to free the pbuf
 */
// typedef err_t (*netif_input_fn)(p: &mut pbuf, inp: &mut NetIfc);
pub type netif_input_fn = fn(p: &mut pbuf, inp: &mut NetIfc) -> Result<(), LwipError>;

/* Function prototype for netif.output functions. Called by lwIP when a packet
 * shall be sent. For ethernet netif, set this to 'etharp_output' and set
 * 'linkoutput'.
 *
 * @param netif The netif which shall send a packet
 * @param p The packet to send (p.payload points to IP header)
 * @param ipaddr The IP address to which the packet shall be sent
 */
pub type netif_output_fn =
    fn(netif: &mut NetIfc, p: &mut pbuf, ipaddr: &mut ip4_addr) -> Result<(), LwipError>;

/* Function prototype for netif.output_ip6 functions. Called by lwIP when a packet
 * shall be sent. For ethernet netif, set this to 'ethip6_output' and set
 * 'linkoutput'.
 *
 * @param netif The netif which shall send a packet
 * @param p The packet to send (p.payload points to IP header)
 * @param ipaddr The IPv6 address to which the packet shall be sent
 */
// typedef err_t (*netif_output_ip6_fn)(netif: &mut NetIfc, p: &mut pbuf,
//        const ipaddr: &mut ip6_addr_t);
pub type netif_output_ip6_fn =
    fn(netif: &mut NetIfc, p: &mut pbuf, ipaddr: &mut ip6_addr_t) -> Result<(), LwipError>;

/* Function prototype for netif.linkoutput functions. Only used for ethernet
 * netifs. This function is called by ARP when a packet shall be sent.
 *
 * @param netif The netif which shall send a packet
 * @param p The packet to send (raw ethernet packet)
 */
// typedef err_t (*netif_linkoutput_fn)(netif: &mut NetIfc, p: &mut pbuf);

/* Function prototype for netif status- or link-callback functions. */
// typedef void (*netif_status_callback_fn)(netif: &mut NetIfc);
pub type netif_status_callback_fn = fn(netif: &mut NetIfc);

/* Function prototype for netif igmp_mac_filter functions */

// typedef err_t (*netif_igmp_mac_filter_fn)(netif: &mut NetIfc,
//        const group: &mut ip4_addr, action: netif_mac_filter_action);
pub type netif_igmp_mac_filter_fn = fn(
    netif: &mut NetIfc,
    group: &mut ip4_addr,
    action: netif_mac_filter_action,
) -> Result<(), LwipError>;

/* Function prototype for netif mld_mac_filter functions */

// typedef err_t (*netif_mld_mac_filter_fn)(netif: &mut NetIfc,
//        const group: &mut ip6_addr_t, action: netif_mac_filter_action);
pub type netif_mld_mac_filter_fn = fn(
    netif: &mut NetIfc,
    group: &mut ip6_addr_t,
    action: netif_mac_filter_action,
) -> Result<(), LwipError>;

// netif_alloc_client_data_id: u8();

/* @ingroup netif_cd
 * Set client data. Obtain ID from netif_alloc_client_data_id().
 */
// #define netif_set_client_data(netif, id, data) netif_get_client_data(netif, id) = (data)

/* @ingroup netif_cd
 * Get client data. Obtain ID from netif_alloc_client_data_id().
 */
// #define netif_get_client_data(netif, id)       (netif).client_data[(id)]
// pub fn netif_get_client_data<T>(netif: &netif, id: u32) -> T {
//     netif.client_data[id] as T
// }

// typedef netif_addr_idx_t: u16;
pub const NETIF_ADDR_IDX_MAX: u32 = 0x7FFF;

// typedef netif_addr_idx_t: u8;
pub const NETIF_ADDR_IDX_MAX: u32 = 0x7F;

// #define LWIP_NETIF_USE_HINTS              1
pub struct netif_hint {
    pub addr_hint: u16,
}
/* LWIP_NETIF_HWADDRHINT */
pub const LWIP_NETIF_USE_HINTS: u32 = 0;

/* Generic data structure used for all lwIP network interfaces.
 *  The following fields should be filled in by the initialization
 *  function for the device driver: hwaddr_len, hwaddr[], mtu, flags */
pub struct NetIfc {
    /* pointer to next in linked list */
    // next: &mut NetIfc;
    /* IP address configuration in network byte order */
    pub ip_addr: LwipAddr,
    pub netmask: LwipAddr,
    pub gw: LwipAddr,
    /* Array of IPv6 addresses for this netif. */
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
    pub input: netif_input_fn,
    /* This function is called by the IP module when it wants
     *  to send a packet on the interface. This function typically
     *  first resolves the hardware address, then sends the packet.
     *  For ethernet physical layer, this is usually etharp_output() */
    pub output: netif_output_fn,
    /* This function is called by ethernet_output() when it wants
     *  to send a packet on the interface. This function outputs
     *  the pbuf as-is on the link medium. */
    pub linkoutput: netif_linkoutput_fn,
    /* This function is called by the IPv6 module when it wants
     *  to send a packet on the interface. This function typically
     *  first resolves the hardware address, then sends the packet.
     *  For ethernet physical layer, this is usually ethip6_output() */
    pub output_ip6: netif_output_ip6_fn,
    /* This function is called when the netif state is set to up or down
     */
    pub status_callback: netif_status_callback_fn,
    /* This function is called when the netif link is set to up or down
     */
    pub link_callback: netif_status_callback_fn,
    /* This function is called when the netif has been removed */
    pub remove_callback: netif_status_callback_fn,
    /* This field can be set by the device driver and could point
     *  to state information for the device. */
    pub state: Vec<u8>,
    pub client_data: Vec<u8>,
    /* the hostname for this netif, NULL is a valid value */
    pub hostname: String,
    pub chksum_flags: u16,
    /* maximum transfer unit (in bytes) */
    pub mtu: u16,
    /* maximum transfer unit (in bytes), updated by RA */
    pub mtu6: u16,
    /* link level hardware address of this interface */
    pub hwaddr: LwipAddr,
    /* number of bytes used in hwaddr */
    /* flags (@see @ref netif_flags) */
    pub flags: u8,
    /* descriptive abbreviation */
    pub name: String,
    /* number of this interface. Used for @ref if_api and @ref netifapi_netif,
     * as well as for IPv6 zones */
    pub num: u64,
    /* is this netif enabled for IPv6 autoconfiguration */
    pub ip6_autoconfig_enabled: bool,
    /* Number of Router Solicitation messages that remain to be sent. */
    pub rs_count: usize,
    /* link type (from "snmp_ifType" snmp_mib2: from.h) */
    pub link_type: u8,
    /* (estimate) link speed */
    pub link_speed: u32,
    /* timestamp at last change made (up/down) */
    pub ts: u64,
    /* counters */
    pub mib2_counters: stats_mib2_netif_ctrs,
    /* This function could be called to add or delete an entry in the multicast
    filter table of the ethernet MAC.*/
    pub igmp_mac_filter: netif_igmp_mac_filter_fn,
    /* This function could be called to add or delete an entry in the IPv6 multicast
    filter table of the ethernet MAC. */
    pub mld_mac_filter: netif_mld_mac_filter_fn,
    pub hints: &mut netif_hint,
    /* List of packets to be queued for ourselves. */
    pub loop_first: &mut pbuf,
    pub loop_last: &mut pbuf,
    pub loop_cnt_current: u16,
}

impl NetIfc {
    pub fn NETIF_SET_CHECKSUM_CTRL(self: &Self, chksumflugs: u16) {
        self.chksum_flags = chksumflugs
    }

    pub fn CHECKSUM_ENABLED(self: &Self, chsksumflag: u16) -> bool {
        self.chksum_flags & chksumflag != false
    }

    pub fn SET_CHECKSUM_CTRL(self: &Self, chksumflags: u16) {
        unimplemented!()
    }
}

// pub fn  netif_init();

// netif_add_noaddr: &mut NetIfc(netif: &mut NetIfc, state: &mut Vec<u8>, netif_init_fn init, netif_input_fn input);

// // netif_add: &mut NetIfc(netif: &mut NetIfc,
// //                             const ipaddr: &mut ip4_addr,  netmask: &mut ip4_addr,  gw: &mut ip4_addr,
// //                             state: &mut Vec<u8>, netif_init_fn init, netif_input_fn input);
// pub fn  netif_set_addr(netif: &mut NetIfc,  ipaddr: &mut ip4_addr,  netmask: &mut ip4_addr,
//                     const gw: &mut ip4_addr);
/* LWIP_IPV4 */
// netif_add: &mut NetIfc(netif: &mut NetIfc, state: &mut Vec<u8>, netif_init_fn init, netif_input_fn input);

// pub fn  netif_remove(Netif * netif);

/* Returns a network interface given its name. The name is of the form
"et0", where the first two letters are the "name" field in the
netif structure, and the digit is in the num field in the same
structure. */
// netif_find: &mut NetIfc(name: &String);

// pub fn  netif_set_default(netif: &mut NetIfc);

// pub fn  netif_set_ipaddr(netif: &mut NetIfc,  ipaddr: &mut ip4_addr);
// pub fn  netif_set_netmask(netif: &mut NetIfc,  netmask: &mut ip4_addr);
// pub fn  netif_set_gw(netif: &mut NetIfc,  gw: &mut ip4_addr);
/* @ingroup netif_ip4 */
pub fn netif_ip4_addr(netif: &NetIfc) {
    ip_2_ip4(&(netif.ip_addr))
}
/* @ingroup netif_ip4 */
pub fn netif_ip4_netmask(netif: &NetIfc) {
    (ip_2_ip4(&(netif.netmask)))
}

/* @ingroup netif_ip4 */
pub fn netif_ip4_gw(netif: &NetIfc) {
    (ip_2_ip4(&(netif.gw)))
}
/* @ingroup netif_ip4 */
pub fn netif_ip_addr4(netif: &NetIfc) {
    (&(netif.ip_addr))
}
/* @ingroup netif_ip4 */
pub fn netif_ip_netmask4(netif: &NetIfc) {
    (&(netif.netmask))
}
/* @ingroup netif_ip4 */
pub fn netif_ip_gw4(netif: &NetIfc) {
    (&(netif.gw))
}

pub fn netif_set_flags(netif: &NetIfc, set_flags: u8) {
    (netif).flags = ((netif).flags | (set_flags));
}

pub fn netif_clear_flags(netif: &NetIfc, clr_flags: u8) {
    (netif).flags = ((netif).flags & (!(clr_flags) & 0xff));
}

pub fn netif_is_flag_set(nefif: &NetIfc, flag: u8) {
    ((netif.flags & (flag)) != 0)
}

fn netif_set_up(netif: &mut NetIfc);
fn netif_set_down(netif: &mut NetIfc);
/* @ingroup netif
 * Ask if an interface is up
 */
pub fn netif_is_up(netif: &NetIfc) -> bool {
    if netif.flags & NETIF_FLAG_UP {
        true
    } else {
        false
    }
}

// pub fn  netif_set_status_callback(
//   netif: &mut NetIfc,
//   status_callback: netif_status_callback_fn);

// pub fn  netif_set_remove_callback(netif: &mut NetIfc, netif_status_callback_fn remove_callback);

// pub fn  netif_set_link_up(netif: &mut NetIfc);
// pub fn  netif_set_link_down(netif: &mut NetIfc);
/* Ask if a link is up */
pub fn netif_is_link_up(netif: &NetIfc) -> bool {
    (if netif.flags & NETIF_FLAG_LINK_UP {
        true
    } else {
        false
    })
}

// pub fn  netif_set_link_callback(netif: &mut NetIfc, netif_status_callback_fn link_callback);

/* @ingroup netif */
// pub fn netif_set_hostname(netif: &mut NetIfc, name: &String) { if((netif) != NULL) { (netif).hostname = name; }}

/* @ingroup netif */
// #define netif_get_hostname(netif) (((netif) != NULL) ? ((netif).hostname) : NULL)
// pub fn netif_get_hostname(netif: &mut NetIfc) -> Option<String> {
//   Some(netif.hostname)
// }

/* @ingroup netif */
// pub fn netif_set_igmp_mac_filter(netif: &NetIfc, function: netif_igmp_mac_filter_fn) { if((netif) != NULL) { (netif).igmp_mac_filter = function; }}

// pub fn netif_get_igmp_mac_filter(netif: &NetIfc) {
//   // (((netif) != NULL) ? ((netif).igmp_mac_filter) : NULL)
//   netif.igmp_mac_filter
// }

/* @ingroup netif */
// #define netif_set_mld_mac_filter(netif, function) loop { if((netif) != NULL) { (netif).mld_mac_filter = function; }}while(0)
// pub fn netif_set_mld_mac_filter(netif &NetIfc)

// #define netif_get_mld_mac_filter(netif) (((netif) != NULL) ? ((netif).mld_mac_filter) : NULL)
// #define netif_mld_mac_filter(netif, addr, action) loop { if((netif) && (netif).mld_mac_filter) { (netif).mld_mac_filter((netif), (addr), (action)); }}while(0)

// pub fn  netif_loop_output(netif: &mut NetIfc, p: &mut pbuf);
// pub fn  netif_poll(netif: &mut NetIfc);

// pub fn  netif_poll_all();

// pub fn  netif_input(p: &mut pbuf, inp: &mut NetIfc);

// pub fn netif_ip_addr6(netif: &NetIfc, i: usize)  -> u8 {&netif.ip6_addr[i]}
/* @ingroup netif_ip6 */
// pub fn netif_ip6_addr(netif: &NetIfc, i: usize) -> u8{ (ip_2_ip6(&((netif).ip6_addr[i])))}
// pub fn  netif_ip6_addr_set(netif: &mut NetIfc, addr_idx: i8,  addr6: &mut ip6_addr_t);
// pub fn  netif_ip6_addr_set_parts(netif: &mut NetIfc, s8_t addr_idx, i0: u32, i1: u32, i2: u32, i3: u32);
// pub fn netif_ip6_addr_state(netif: &NetIfc, i: usize) -> u8{ ((netif).ip6_addr_state[i])}
// pub fn  netif_ip6_addr_set_state(netif: &mut NetIfc, addr_idx: usize, state: u8);
// s8_t netif_get_ip6_addr_match(netif: &mut NetIfc,  ip6addr: &mut ip6_addr_t);
// pub fn  netif_create_ip6_linklocal_address(netif: &mut NetIfc, from_mac_48bit: u8);
// pub fn  netif_add_ip6_address(netif: &mut NetIfc,  ip6addr: &mut ip6_addr_t, chosen_idx: &mut i8);
// pub fn netif_set_ip6_autoconfig_enabled(netif: &mut NetIfc, action: bool) loop { if(netif) { (netif).ip6_autoconfig_enabled = (action); }}while(0)

// #define netif_ip6_addr_valid_life(netif, i)  \
//     (((netif) != NULL) ? ((netif).ip6_addr_valid_life[i]) : IP6_ADDR_LIFE_STATIC)
// #define netif_ip6_addr_set_valid_life(netif, i, secs) \
//     loop { if (netif != NULL) { (netif).ip6_addr_valid_life[i] = (secs); }} while (0)
// #define netif_ip6_addr_pref_life(netif, i)  \
//     (((netif) != NULL) ? ((netif).ip6_addr_pref_life[i]) : IP6_ADDR_LIFE_STATIC)
// #define netif_ip6_addr_set_pref_life(netif, i, secs) \
//     loop { if (netif != NULL) { (netif).ip6_addr_pref_life[i] = (secs); }} while (0)
// #define netif_ip6_addr_isstatic(netif, i)  \
//     (netif_ip6_addr_valid_life((netif), (i)) == IP6_ADDR_LIFE_STATIC)
//  /* !LWIP_IPV6_ADDRESS_LIFETIMES */
// #define netif_ip6_addr_isstatic(netif, i)  (1) /* all addresses are static */
// #define netif_mtu6(netif) ((netif).mtu6)
//  /* LWIP_ND6_ALLOW_RA_UPDATES */
// #define netif_mtu6(netif) ((netif).mtu)

// #define NETIF_SET_HINTS(netif, netifhint)  (netif).hints = (netifhint)
// #define NETIF_RESET_HINTS(netif)      (netif).hints = NULL
//  /* LWIP_NETIF_USE_HINTS */
// #define NETIF_SET_HINTS(netif, netifhint)
// #define NETIF_RESET_HINTS(netif)

// netif_name_to_index: u8(name: &String);
// char * netif_index_to_name(idx: u8, name: &mut String);
// netif_get_by_index: &mut NetIfc(idx: u8);

/* Interface indexes always start at 1 per RFC 3493, section 4, num starts at 0 (internal index is 0..254)*/
// #define netif_get_index(netif)      (((netif).num + 1))
// #define NETIF_NO_INDEX              (0)

/*
 * @ingroup netif
 * Extended netif status callback (NSC) reasons flags.
 * May be extended in the future!
 */
// typedef netif_nsc_reason_t: u16;

/* used for initialization only */
pub const LWIP_NSC_NONE: u32 = 0x0000;
/* netif was added. arg: NULL. Called AFTER netif was added. */
pub const LWIP_NSC_NETIF_ADDED: u32 = 0x0001;
/* netif was removed. arg: NULL. Called BEFORE netif is removed. */
pub const LWIP_NSC_NETIF_REMOVED: u32 = 0x0002;
/* link changed */
pub const LWIP_NSC_LINK_CHANGED: u32 = 0x0004;
/* netif administrative status changed.\n
 * up is called AFTER netif is set up.\n
 * down is called BEFORE the netif is actually set down. */
pub const LWIP_NSC_STATUS_CHANGED: u32 = 0x0008;
/* IPv4 address has changed */
pub const LWIP_NSC_IPV4_ADDRESS_CHANGED: u32 = 0x0010;
/* IPv4 gateway has changed */
pub const LWIP_NSC_IPV4_GATEWAY_CHANGED: u32 = 0x0020;
/* IPv4 netmask has changed */
pub const LWIP_NSC_IPV4_NETMASK_CHANGED: u32 = 0x0040;
/* called AFTER IPv4 address/gateway/netmask changes have been applied */
pub const LWIP_NSC_IPV4_SETTINGS_CHANGED: u32 = 0x0080;
/* IPv6 address was added */
pub const LWIP_NSC_IPV6_SET: u32 = 0x0100;
/* IPv6 address state has changed */
pub const LWIP_NSC_IPV6_ADDR_STATE_CHANGED: u32 = 0x0200;

/* @ingroup netif
 * Argument supplied to netif_ext_callback_fn.
 */
// typedef union
// {
//   /* Args to LWIP_NSC_LINK_CHANGED callback */
//   struct link_changed_s
//   {
//     /* 1: up; 0: down */
//     state: u8;
//   } link_changed;
//   /* Args to LWIP_NSC_STATUS_CHANGED callback */
//   struct status_changed_s
//   {
//     /* 1: up; 0: down */
//     state: u8;
//   } status_changed;
//   /* Args to LWIP_NSC_IPV4_ADDRESS_CHANGED|LWIP_NSC_IPV4_GATEWAY_CHANGED|LWIP_NSC_IPV4_NETMASK_CHANGED|LWIP_NSC_IPV4_SETTINGS_CHANGED callback */
//   struct ipv4_changed_s
//   {
//     /* Old IPv4 address */
//     const old_address: &mut LwipAddr;
//     const old_netmask: &mut LwipAddr;
//     const old_gw: &mut LwipAddr;
//   } ipv4_changed;
//   /* Args to LWIP_NSC_IPV6_SET callback */
//   struct ipv6_set_s
//   {
//     /* Index of changed IPv6 address */
//     s8_t addr_index;
//     /* Old IPv6 address */
//     const old_address: &mut LwipAddr;
//   } ipv6_set;
//   /* Args to LWIP_NSC_IPV6_ADDR_STATE_CHANGED callback */
//   struct ipv6_addr_state_changed_s
//   {
//     /* Index of affected IPv6 address */
//     s8_t addr_index;
//     /* Old IPv6 address state */
//     old_state: u8;
//     /* Affected IPv6 address */
//     const address: &mut LwipAddr;
//   } ipv6_addr_state_changed;
// } netif_ext_callback_args_t;

pub struct netif_ext_callback_args_t {
    pub state: u8,
    pub old_address: LwipAddr,
    pub old_netmask: LwipAddr,
    pub old_gw: LwipAddr,
    pub addr_index: usize,
}

/*
 * @ingroup netif
 * Function used for extended netif status callbacks
 * Note: When parsing reason argument, keep in mind that more reasons may be added in the future!
 * @param netif netif that is affected by change
 * @param reason change reason
 * @param args depends on reason, see reason description
 */
// typedef void (*netif_ext_callback_fn)(netif: &mut NetIfc, netif_nsc_reason_t reason,  netif_ext_callback_args_t* args);
type netif_ext_callback_fn =
    fn(netif: &mut NetIfc, reason: netif_nsc_reason_t, args: &netif_ext_callback_args_t);

// struct netif_ext_callback;
// typedef struct netif_ext_callback
// {
//   netif_ext_callback_fn callback_fn;
//   // struct netif_ext_callback* next;
// } netif_ext_callback_t;

// #define NETIF_DECLARE_EXT_CALLBACK(name) static netif_ext_callback_t name;
// pub fn  netif_add_ext_callback(callback: &mut netif_ext_callback_t, netif_ext_callback_fn fn);
// pub fn  netif_remove_ext_callback(callback: &mut netif_ext_callback_t);
// pub fn  netif_invoke_ext_callback(netif: &mut NetIfc, netif_nsc_reason_t reason,  netif_ext_callback_args_t* args);

// #define NETIF_DECLARE_EXT_CALLBACK(name)
// #define netif_add_ext_callback(callback, fn)
// #define netif_remove_ext_callback(callback)
// #define netif_invoke_ext_callback(netif, reason, args)
