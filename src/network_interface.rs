use log::debug;
use crate::acd::AcdStateInfo;
use crate::errors::{LwipError, LwipErrorCode};
use crate::errors::LwipErrorCode::{InvalidArgument, NotSet};
use crate::ip::ip_input;
use crate::ip_address::{IpAddress, IPV4_ADDR_ANY};
use crate::mac_address::MacAddress;
use crate::netif_hint::NetifHint;
use crate::packet_buffer::PacketBuffer;

/**
 * @file
 * lwIP network interface abstraction
 *
 * @defgroup netif Network interface (NETIF)
 * @ingroup callbackstyle_api
 *
 * @defgroup netif_ip4 IPv4 address handling
 * @ingroup netif
 *
 * @defgroup netif_ip6 IPv6 address handling
 * @ingroup netif
 *
 * @defgroup netif_cd Client data handling
 * Store data (void*) on a netif for application usage.
 * @see @ref LWIP_NUM_NETIF_CLIENT_DATA
 * @ingroup netif
 */

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
 */

// #define NETIF_STATUS_CALLBACK(n) do{ if ( n.status_callback) { ( n.status_callback)(n); }}while(0)


// #define NETIF_LINK_CALLBACK(n) do{ if ( n.link_callback) { ( n.link_callback)(n); }}while(0)

// static netif_ext_callback_t *ext_callback;
//struct netif *netif_list
// struct netif *netif_default;

// #define netif_index_to_num(index)   ((index) - 1)
// static netif_num: u8;

// static netif_client_id: u8;

pub const NETIF_REPORT_TYPE_IPV4: u8 =  0x01;
pub const NETIF_REPORT_TYPE_IPV6: u8 =  0x02;


// /* Throughout this file, IP addresses are expected to be in
//  * the same byte order as in IP_PCB. */

// /** Must be the maximum of all used hardware address lengths
//     across all types of interfaces in use.
//     This does not have to be changed, normally. */

pub const NETIF_MAX_HWADDR_LEN: usize = 6;


// /** The size of a fully constructed netif name which the
//  * netif can be identified by in APIs. Composed of
//  * 2 chars, 3 (max) digits, and 1 \0
//  */
pub const NETIF_NAMESIZE: usize = 6;

// /**
//  * @defgroup netif_flags Flags
//  * @ingroup netif
//  * @{
//  */
// /** Whether the network interface is 'up'. This is
//  * a software flag used to control whether this network
//  * interface is enabled and processes traffic.
//  * It must be set by the startup code before this netif can be used
//  * (also for dhcp/autoip).
//  */
pub const NETIF_FLAG_UP: u8 =           0x01;
// /** If set, the netif has broadcast capability.
//  * Set by the netif driver in its init function. */
pub const NETIF_FLAG_BROADCAST: u8 =    0x02;
// /** If set, the interface has an active link
//  *  (set by the network interface driver).
//  * Either set by the netif driver in its init function (if the link
//  * is up at that time) or at a later point once the link comes up
//  * (if link detection is supported by the hardware). */
pub const NETIF_FLAG_LINK_UP: u8 =      0x04;
// /** If set, the netif is an ethernet device using ARP.
//  * Set by the netif driver in its init function.
//  * Used to check input packet types and use of DHCP. */
pub const NETIF_FLAG_ETHARP: u8 =       0x08;
// /** If set, the netif is an ethernet device. It might not use
//  * ARP or TCP/IP if it is used for PPPoE only.
//  */
pub const NETIF_FLAG_ETHERNET: u8 =     0x10;
// /** If set, the netif has IGMP capability.
//  * Set by the netif driver in its init function. */
pub const NETIF_FLAG_IGMP: u8 =         0x20;
// /** If set, the netif has MLD6 capability.
//  * Set by the netif driver in its init function. */
pub const NETIF_FLAG_MLD6: u8 =         0x40;

// /**
//  * @}
//  */

pub enum LwipInternalNetifClientDataIndex
{
    LwipNetifClientDataIndexDhcp,
    LwipNetifClientDataIndexAutoip,
    LwipNetifClientDataIndexAcd,
    LwipNetifClientDataIndexIgmp,
    LwipNetifClientDataIndexDhcp6,
    LwipNetifClientDataIndexMld6,
    PNetifClientDataIndexMax
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
pub const NETIF_CHECKSUM_DISABLE_ALL: u32 = 0x0000; /* LWIP_CHECKSUM_CTRL_PER_NETIF */

// struct netif;

// /** MAC Filter Actions, these are passed to a netif's igmp_mac_filter or
//  * mld_mac_filter callback function. */
enum NetifMacFilterAction {
  /** Delete a filter entry */
  NetifDelMacFilter = 0,
  /** Add a filter entry */
  NetifAddMacFilter = 1
}

// /** Function prototype for netif init functions. Set up flags and output/linkoutput
//  * callback functions in this function.
//  *
//  * @param netif The netif to initialize
//  */
// TODO:
// typedef err_t (*netif_init_fn)(struct netif *netif);
// /** Function prototype for  netif.input functions. This function is saved as 'input'
//  * callback function in the netif struct. Call it when a packet has been received.
//  *
//  * @param p The received packet, copied into a pbuf
//  * @param inp The netif which received the packet
//  * @return ERR_OK if the packet was handled
//  *         != ERR_OK is the packet was NOT handled, in this case, the caller has
//  *                   to free the pbuf
//  */
// TODO
// typedef err_t (*netif_input_fn)(struct pbuf *p, struct netif *inp);


// /** Function prototype for  netif.output functions. Called by lwIP when a packet
//  * shall be sent. For ethernet netif, set this to 'etharp_output' and set
//  * 'linkoutput'.
//  *
//  * @param netif The netif which shall send a packet
//  * @param p The packet to send ( p.payload points to IP header)
//  * @param ipaddr The IP address to which the packet shall be sent
//  */
// TODO:
// typedef err_t (*netif_output_fn)(struct netif *netif, struct pbuf *p,
//        const ip4_addr_t *ipaddr);
 /* LWIP_IPV4*/

// /** Function prototype for  netif.output_ip6 functions. Called by lwIP when a packet
//  * shall be sent. For ethernet netif, set this to 'ethip6_output' and set
//  * 'linkoutput'.
//  *
//  * @param netif The netif which shall send a packet
//  * @param p The packet to send ( p.payload points to IP header)
//  * @param ipaddr The IPv6 address to which the packet shall be sent
//  */
// typedef err_t (*netif_output_ip6_fn)(struct netif *netif, struct pbuf *p,
//        const ip6_addr_t *ipaddr);
 // /* LWIP_IPV6 */

// /** Function prototype for  netif.linkoutput functions. Only used for ethernet
//  * netifs. This function is called by ARP when a packet shall be sent.
//  *
//  * @param netif The netif which shall send a packet
//  * @param p The packet to send (raw ethernet packet)
//  */
// typedef err_t (*netif_linkoutput_fn)(struct netif *netif, struct pbuf *p);
/** Function prototype for netif status- or link-callback functions. */
// typedef void (*netif_status_callback_fn)(struct netif *netif);

/** Function prototype for netif igmp_mac_filter functions */
// typedef err_t (*netif_igmp_mac_filter_fn)(struct netif *netif,
//        const ip4_addr_t *group, enum NetifMacFilterAction action);
 /* LWIP_IPV4 && LWIP_IGMP */
// IP_IPV6 && LWIP_IPV6_MLD
/** Function prototype for netif mld_mac_filter functions */
// typedef err_t (*netif_mld_mac_filter_fn)(struct netif *netif,
//        const ip6_addr_t *group, enum NetifMacFilterAction action);
 /* LWIP_IPV6 && LWIP_IPV6_MLD */

// #if LWIP_DHCP || LWIP_AUTOIP || LWIP_IGMP || LWIP_IPV6_MLD || LWIP_IPV6_DHCP6 || (LWIP_NUM_NETIF_CLIENT_DATA > 0)
// #if LWIP_NUM_NETIF_CLIENT_DATA > 0
// u8_t netif_alloc_client_data_id();

// ngroup netif_cd
//  * Set client data. Obtain ID from netif_alloc_client_data_id().
//  */
// #define netif_set_client_data(netif, id, data) netif_get_client_data(netif, id) = (data)
/** @ingroup netif_cd
 * Get client data. Obtain ID from netif_alloc_client_data_id().
 */
// #define netif_get_client_data(netif, id)       (netif)->client_data[(id)]


pub const NETIF_ADDR_IDX_MAX: u32 = 0x7FFF;

 /* LWIP_NETIF_HWADDRHINT || LWIP_VLAN_PCP*/

#[derive(Debug, Clone, Default)]
pub struct NetifIp6AddressPair {
    address: IpAddress,
    state: u8,
    valid_life: u32,
    preferred_life: u32,
}

pub struct NetifIpv4NetInfo {
    address: IpAddress,
    netmask: IpAddress,
    gateway: IpAddress, // TODO: should this be handled separately in some sort of routing?
}

/** Generic data structure used for all lwIP network interfaces.
 *  The following fields should be filled in by the initialization
 *  function for the device driver: hwaddr_len, hwaddr[], mtu, flags */
#[derive(Debug, Clone, Default)]
pub struct NetworkInterface {
    // /** pointer to next in linked list */
    // struct netif *next;
    next_netif_id: u32,
    netif_id: u32,
    // /** IP address configuration in network byte order */
    ip4_address_config: NetifIpv4NetInfo,
    // /* LWIP_IPV4 */
    //  /** Array of IPv6 addresses for this netif. */
    ip6_addresses: Vec<NetifIp6AddressPair>,
    // ip6_addr: [IpAddress;LWIP_IPV6_NUM_ADDRESSES],
    // /** The state of each IPv6 address (Tentative, Preferred, etc).
    //  * @see ip6_addr.h */
    // ip6_addr_state: [u8; LWIP_IPV6_NUM_ADDRESSES],
    // /** Remaining valid and preferred lifetime of each IPv6 address, in seconds.
    //  * For valid lifetimes, the special value of IP6_ADDR_LIFE_STATIC (0)
    //  * indicates the address is static and has no lifetimes. */
    /* LWIP_IPV6_ADDRESS_LIFETIMES */
    /* LWIP_IPV6 */
// This function is called by the network device driver
//    *  to pass a packet up the TCP/IP stack. */
    input: netif_input_fn,

    // /** This function is called by the IP module when it wants
    //  *  to send a packet on the interface. This function typically
    //  *  first resolves the hardware address, then sends the packet.
    //  *  For ethernet physical layer, this is usually etharp_output() */
    output: netif_output_fn,
    /* LWIP_IPV4 */
    // This function is called by ethernet_output() when it wants
//    *  to send a packet on the interface. This function outputs
//    *  the pbuf as-is on the link medium. */
    linkoutput: netif_linkoutput_fn,

    /** This function is called by the IPv6 module when it wants
      *  to send a packet on the interface. This function typically
      *  first resolves the hardware address, then sends the packet.
      *  For ethernet physical layer, this is usually ethip6_output() */
    output_ip6: netif_output_ip6_fn,
    /* LWIP_IPV6 */
    // IP_NETIF_STATUS_CALLBACK
    /** This function is called when the netif state is set to up or down
     */
    status_callback: Option<netif_status_callback_fn>,
    /* LWIP_NETIF_STATUS_CALLBACK */
    // IP_NETIF_LINK_CALLBACK
    /** This function is called when the netif link is set to up or down
     */
    link_callback: netif_status_callback_fn,
    /* LWIP_NETIF_LINK_CALLBACK */
    // IP_NETIF_REMOVE_CALLBACK
    /** This function is called when the netif has been removed */
    remove_callback: netif_status_callback_fn,
    /* LWIP_NETIF_REMOVE_CALLBACK */
    // This field can be set by the device driver and could point
//    *  to state information for the device. */
//   void *state;
    state_info: Vec<u8>,
    // #ifdef netif_get_client_data
//   void* client_data[LWIP_NETIF_CLIENT_DATA_INDEX_MAX + LWIP_NUM_NETIF_CLIENT_DATA];
    client_data: [Vec<u8>; LWIP_NETIF_CLIENT_DATA_INDEX_MAX + LWIP_NUM_NETIF_CLIENT_DATA],
    // IP_NETIF_HOSTNAME
    /* the hostname for this netif, NULL is a valid value */
    hostname: String,
    /* LWIP_NETIF_HOSTNAME */
    chksum_flags: u16,
    // /* LWIP_CHECKSUM_CTRL_PER_NETIF*/
// maximum transfer unit (in bytes) */
    mtu: u16,
    // && LWIP_ND6_ALLOW_RA_UPDATES
    /** maximum transfer unit (in bytes), updated by RA */
    // mtu6: u16;
    /* LWIP_IPV6 && LWIP_ND6_ALLOW_RA_UPDATES */
// link level hardware address of this interface */
//   u8_t hwaddr[NETIF_MAX_HWADDR_LEN];
    mac_address: MacAddress,
    /** number of bytes used in hwaddr */
    // hwaddr_len: u8;
    /** flags (@see @ref netif_flags) */
    flags: u8,
    /** descriptive abbreviation */
    name: String,
    /** number of this interface. Used for @ref if_api and @ref netifapi_netif,
      * as well as for IPv6 zones */
    num: u8,
    // _AUTOCONFIG
    /** is this netif enabled for IPv6 autoconfiguration */
    ip6_autoconfig_enabled: bool,
    /* LWIP_IPV6_AUTOCONFIG */
    // IP_IPV6_SEND_ROUTER_SOLICIT
    /** Number of Router Solicitation messages that remain to be sent. */
    rs_count: u8,
    /* LWIP_IPV6_SEND_ROUTER_SOLICIT */
    // B2_STATS
    /** link type (from "snmp_ifType" enum from snmp_mib2.h) */
    link_type: u8,
    /** (estimate) link speed */
    link_speed: u32,
    /** timestamp at last change made (up/down) */
    ts: u32,
    /** counters */
    // struct stats_mib2_netif_ctrs mib2_counters;
    /* MIB2_STATS */
// IP_IPV4 && LWIP_IGMP
    /** This function could be called to add or delete an entry in the multicast
         filter table of the ethernet MAC.*/
    igmp_mac_filter: netif_igmp_mac_filter_fn,
    /* LWIP_IPV4 && LWIP_IGMP */
    // IP_IPV6 && LWIP_IPV6_MLD
    /** This function could be called to add or delete an entry in the IPv6 multicast
         filter table of the ethernet MAC. */
    mld_mac_filter: Option<netif_mld_mac_filter_fn>,
    /* LWIP_IPV6 && LWIP_IPV6_MLD */
    // IP_ACD
//   struct acd *acd_list;
    pub(crate) acd_list: Vec<AcdStateInfo>,
    /* LWIP_ACD */
    // IP_NETIF_USE_HINTS
//   struct netif_hint *hints;
    netif_hints: Vec<NetifHint>,
    /* LWIP_NETIF_USE_HINTS */
    // ABLE_LOOPBACK
    /* List of packets to be queued for ourselves. */
    // struct pbuf *loop_first;
    // struct pbuf *loop_last;
    packets: Vec<PacketBuffer>,
    // #if LWIP_LOOPBACK_MAX_PBUFS
//   loop_cnt_current: u16;
    /* LWIP_LOOPBACK_MAX_PBUFS */
// IP_NETIF_LOOPBACK_MULTITHREADING
    /* Used if the original scheduling failed. */
    reschedule_poll: bool,
    /* LWIP_NETIF_LOOPBACK_MULTITHREADING */
    /* ENABLE_LOOPBACK */
    up: bool,
    link_up: bool,
    ethernet: bool,
    etharp: bool,
    igmp: bool,
}

impl NetworkInterface {
    pub fn low_level_init(&mut self, hwaddr: &MacAddress, mtu: u16, ) {
        self.mac_address = hwaddr.clone();
        self.mtu = mtu;
        self.flags |= NETIF_FLAG_BROADCAST | NETIF_FLAG_ETHARP | NETIF_FLAG_LINK_UP;
        if self.mld_mac_filter.is_some() {
            let func = self.mld_mac_filter.unwrap();
            let mut ip6_allnodes_ll = Ipv6Address::new();
            ip6_addr_set_allnodes_linklocal(&mut ip6_allnodes_ll);
            func(self, &ip6_allnodes_ll, NETIF_ADD_MAC_FILTER);
        }
    }
}

pub enum LwipNetifStateChange {
    None,
    Added,
    Removed,
    LinkChanged,
    StatusChanged,
    Ipv4AddressChanged,
    Ipv4GatewayChanged,
    Ipv4NetmaskChanged,
    Ipv4SettingsChanged,
    Ipv6Set,
    Ipv6AddrStateChanged,
    Ipv4AddrValid,
}


#[derive(Clone,Debug,Default)]
pub struct NetifExtCallbackArgs {
    state: u8,
    old_address: IpAddress,
    old_netmask: IpAddress,
    old_gateway: IpAddress,
    address_index: u32,
    old_state: u8,
    address: IpAddress,
}

impl NetifExtCallbackArgs {
    pub fn new() -> Self {
        Self::default()
    }
}

type NetifExtCallbackFn = fn(netif: &mut NetworkInterface, reason: u16, args: &NetifExtCallbackArgs);

pub struct NetifExtCallback
{
   callback_fn: NetifExtCallbackFn,
    next: u32,
}
