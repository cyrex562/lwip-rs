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

use crate::core::defines::LwipAddr;
use crate::core::error::LwipError;
use crate::core::options::LWIP_NETIF_LOOPBACK;
use crate::ip::ip4_addr_h::ip4_addr;
use crate::netif::defs::{NETIF_FLAG_LINK_UP, NETIF_FLAG_UP, NetworkInterfaceCtx};
use crate::packetbuffer::pbuf_h::PacketBuffer;

/* Throughout this file, IP addresses are expected to be in
 * the same byte order as in IP_PCB. */

/* Must be the maximum of all used hardware address lengths
across all types of interfaces in use.
This does not have to be changed, normally. */

/* The size of a fully constructed netif name which the
 * netif can be identified by in APIs. Composed of
 * 2 chars, 3 (max) digits, and 1 \0
 */

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
/* If set, the netif has broadcast capability.
 * Set by the netif driver in its init function. */
/* If set, the interface has an active link
 *  (set by the network interface driver).
 * Either set by the netif driver in its init function (if the link
 * is up at that time) or at a later poonce: i32 the link comes up
 * (if link detection is supported by the hardware). */
/* If set, the netif is an ethernet device using ARP.
 * Set by the netif driver in its init function.
 * Used to check input packet types and use of DHCP. */
/* If set, the netif is an ethernet device. It might not use
 * ARP or TCP/IP if it is used for PPPoE only.
 */
/* If set, the netif has IGMP capability.
 * Set by the netif driver in its init function. */
/* If set, the netif has MLD6 capability.
 * Set by the netif driver in its init function. */

/* Function prototype for netif.input functions. This function is saved as 'input'
 * callback function in the netif struct. Call it when a packet has been received.
 *
 * @param p The received packet, copied into a pbuf
 * @param inp The netif which received the packet
 * @return ERR_OK if the packet was handled
 *         != ERR_OK is the packet was NOT handled, in this case, the caller has
 *                   to free the pbuf
 */

/* Generic data structure used for all lwIP network interfaces.
 *  The following fields should be filled in by the initialization
 *  function for the device driver: hwaddr_len, hwaddr[], mtu, flags */

// pub fn  netif_init();

// netif_add_noaddr: &mut NetIfc(netif: &mut NetIfc, state: &mut Vec<u8>, NetifInitFn init, NetifInputFn input);

// // netif_add: &mut NetIfc(netif: &mut NetIfc,
// //                             const ipaddr: &mut LwipAddr,  netmask: &mut LwipAddr,  gw: &mut LwipAddr,
// //                             state: &mut Vec<u8>, NetifInitFn init, NetifInputFn input);
// pub fn  netif_set_addr(netif: &mut NetIfc,  ipaddr: &mut LwipAddr,  netmask: &mut LwipAddr,
//                     const gw: &mut LwipAddr);
//  LWIP_IPV4 
// netif_add: &mut NetIfc(netif: &mut NetIfc, state: &mut Vec<u8>, NetifInitFn init, NetifInputFn input);

// pub fn  netif_remove(Netif * netif);

/* Returns a network interface given its name. The name is of the form
"et0", where the first two letters are the "name" field in the
netif structure, and the digit is in the num field in the same
structure. */
// netif_find: &mut NetIfc(name: &String);

// pub fn  netif_set_default(netif: &mut NetIfc);

// pub fn  netif_set_ipaddr(netif: &mut NetIfc,  ipaddr: &mut LwipAddr);
// pub fn  netif_set_netmask(netif: &mut NetIfc,  netmask: &mut LwipAddr);
// pub fn  netif_set_gw(netif: &mut NetIfc,  gw: &mut LwipAddr);
//  @ingroup netif_ip4 
// pub fn netif_ip4_addr(netif: &NetworkInterface) {
//     ip_2_ip4(&(netif.ip_addr))
// }
//  @ingroup netif_ip4 
// pub fn netif_ip4_netmask(netif: &NetworkInterface) {
//     (ip_2_ip4(&(netif.netmask)))
// }

//  @ingroup netif_ip4 
// pub fn netif_ip4_gw(netif: &NetworkInterface) {
//     (ip_2_ip4(&(netif.gw)))
// }
//  @ingroup netif_ip4 
// pub fn netif_ip_addr4(netif: &NetworkInterface) {
//     (&(netif.ip_addr))
// }
//  @ingroup netif_ip4 
// pub fn netif_ip_netmask4(netif: &NetworkInterface) {
//     (&(netif.netmask))
// }
//  @ingroup netif_ip4 
// pub fn netif_ip_gw4(netif: &NetworkInterface) {
//     (&(netif.gw))
// }

// fn netif_set_up(netif: &mut NetworkInterface);
// fn netif_set_down(netif: &mut NetworkInterface);

// pub fn  netif_set_status_callback(
//   netif: &mut NetIfc,
//   status_callback: NetifStatusCallbackFn);

// pub fn  netif_set_remove_callback(netif: &mut NetIfc, NetifStatusCallbackFn remove_callback);

// pub fn  netif_set_link_callback(netif: &mut NetIfc, NetifStatusCallbackFn link_callback);

//  @ingroup netif 
// pub fn netif_set_hostname(netif: &mut NetIfc, name: &String) { if((netif) != NULL) { (netif).hostname = name; }}

//  @ingroup netif 
// #define netif_get_hostname(netif) (((netif) != NULL) ? ((netif).hostname) : NULL)
// pub fn netif_get_hostname(netif: &mut NetIfc) -> Option<String> {
//   Some(netif.hostname)
// }

//  @ingroup netif 
// pub fn netif_set_igmp_mac_filter(netif: &NetIfc, function: NetifIgmpMacFilterFn) { if((netif) != NULL) { (netif).igmp_mac_filter = function; }}

// pub fn netif_get_igmp_mac_filter(netif: &NetIfc) {
//   // (((netif) != NULL) ? ((netif).igmp_mac_filter) : NULL)
//   netif.igmp_mac_filter
// }

//  @ingroup netif 
// #define netif_set_mld_mac_filter(netif, function) loop { if((netif) != NULL) { (netif).mld_mac_filter = function; }}while(0)
// pub fn netif_set_mld_mac_filter(netif &NetIfc)

// #define netif_get_mld_mac_filter(netif) (((netif) != NULL) ? ((netif).mld_mac_filter) : NULL)
// #define netif_mld_mac_filter(netif, addr, action) loop { if((netif) && (netif).mld_mac_filter) { (netif).mld_mac_filter((netif), (addr), (action)); }}while(0)

// pub fn  netif_loop_output(netif: &mut NetIfc, p: &mut PacketBuffer);
// pub fn  netif_poll(netif: &mut NetIfc);

// pub fn  netif_poll_all();

// pub fn  netif_input(p: &mut PacketBuffer, inp: &mut NetIfc);

// pub fn netif_ip_addr6(netif: &NetIfc, i: usize)  -> u8 {&netif.ip6_addr[i]}
//  @ingroup netif_ip6 
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
//  //  !LWIP_IPV6_ADDRESS_LIFETIMES 
// #define netif_ip6_addr_isstatic(netif, i)  (1) //  all addresses are static 
// #define netif_mtu6(netif) ((netif).mtu6)
//  //  LWIP_ND6_ALLOW_RA_UPDATES 
// #define netif_mtu6(netif) ((netif).mtu)

// #define NETIF_SET_HINTS(netif, netifhint)  (netif).hints = (netifhint)
// #define NETIF_RESET_HINTS(netif)      (netif).hints = NULL
//  //  LWIP_NETIF_USE_HINTS 
// #define NETIF_SET_HINTS(netif, netifhint)
// #define NETIF_RESET_HINTS(netif)

// netif_name_to_index: u8(name: &String);
// char * netif_index_to_name(idx: u8, name: &mut String);
// netif_get_by_index: &mut NetIfc(idx: u8);

//  Interface indexes always start at 1 per RFC 3493, section 4, num starts at 0 (internal index is 0..254)
// #define netif_get_index(netif)      (((netif).num + 1))
// #define NETIF_NO_INDEX              (0)

/*
 * @ingroup netif
 * Extended netif status callback (NSC) reasons flags.
 * May be extended in the future!
 */
// typedef netif_nsc_reason_t: u16;
/* netif administrative status changed.\n
 * up is called AFTER netif is set up.\n
 * down is called BEFORE the netif is actually set down. */

/* @ingroup netif
 * Argument supplied to netif_ext_callback_fn.
 */
// typedef union
// {
//   //  Args to LWIP_NSC_LINK_CHANGED callback 
//   struct link_changed_s
//   {
//     //  1: up; 0: down 
//     state: u8;
//   } link_changed;
//   //  Args to LWIP_NSC_STATUS_CHANGED callback 
//   struct status_changed_s
//   {
//     //  1: up; 0: down 
//     state: u8;
//   } status_changed;
//   //  Args to LWIP_NSC_IPV4_ADDRESS_CHANGED|LWIP_NSC_IPV4_GATEWAY_CHANGED|LWIP_NSC_IPV4_NETMASK_CHANGED|LWIP_NSC_IPV4_SETTINGS_CHANGED callback 
//   struct ipv4_changed_s
//   {
//     //  Old IPv4 address 
//     const old_address: &mut LwipAddr;
//     const old_netmask: &mut LwipAddr;
//     const old_gw: &mut LwipAddr;
//   } ipv4_changed;
//   //  Args to LWIP_NSC_IPV6_SET callback 
//   struct ipv6_set_s
//   {
//     //  Index of changed IPv6 address 
//     s8_t addr_index;
//     //  Old IPv6 address 
//     const old_address: &mut LwipAddr;
//   } ipv6_set;
//   //  Args to LWIP_NSC_IPV6_ADDR_STATE_CHANGED callback 
//   struct ipv6_addr_state_changed_s
//   {
//     //  Index of affected IPv6 address 
//     s8_t addr_index;
//     //  Old IPv6 address state 
//     old_state: u8;
//     //  Affected IPv6 address 
//     const address: &mut LwipAddr;
//   } ipv6_addr_state_changed;
// } NetifExtCallbackArgsT;

/*
 * @ingroup netif
 * Function used for extended netif status callbacks
 * Note: When parsing reason argument, keep in mind that more reasons may be added in the future!
 * @param netif netif that is affected by change
 * @param reason change reason
 * @param args depends on reason, see reason description
 */

// struct netif_ext_callback;
// typedef struct netif_ext_callback
// {
//   netif_ext_callback_fn callback_fn;
//   // struct netif_ext_callback* next;
// } netif_ext_callback_t;

// #define NETIF_DECLARE_EXT_CALLBACK(name) static netif_ext_callback_t name;
// pub fn  netif_add_ext_callback(callback: &mut netif_ext_callback_t, netif_ext_callback_fn fn);
// pub fn  netif_remove_ext_callback(callback: &mut netif_ext_callback_t);
// pub fn  netif_invoke_ext_callback(netif: &mut NetIfc, netif_nsc_reason_t reason,  NetifExtCallbackArgsT* args);

// #define NETIF_DECLARE_EXT_CALLBACK(name)
// #define netif_add_ext_callback(callback, fn)
// #define netif_remove_ext_callback(callback)
// #define netif_invoke_ext_callback(netif, reason, args)


/*
 * @file
 * netif API (to be used from non-TCPIP threads)
 */

/*
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
 */

// #define LWIP_HDR_NETIFAPI_H

//  API for application

//  @ingroup netifapi_arp
// pub fn  netifapi_arp_add( ipaddr: &mut LwipAddr, ethaddr: &mut MacAddress, type: NetifapiArpEntry);
// //  @ingroup netifapi_arp
// pub fn  netifapi_arp_remove( ipaddr: &mut LwipAddr, type: NetifapiArpEntry);

// pub fn  netifapi_netif_add(netif: &mut NetIfc,

//                          const ipaddr: &mut LwipAddr,  netmask: &mut LwipAddr,  gw: &mut LwipAddr,

//                          state: &mut Vec<u8>, NetifInitFn init, NetifInputFn input);

// pub fn  netifapi_netif_set_addr(netif: &mut NetIfc,  ipaddr: &mut LwipAddr,
//                               const netmask: &mut LwipAddr,  gw: &mut LwipAddr);

// pub fn  netifapi_netif_common(netif: &mut NetIfc, netifapi_void_fn voidfunc,
//                             netifapi_errt_fn errtfunc);

// //  @ingroup netifapi_netif
// pub fn  netifapi_netif_name_to_index(name: &String, index: &mut Vec<u8>);
// //  @ingroup netifapi_netif
// pub fn  netifapi_netif_index_to_name(index: u8, name: &mut String);

// /* @ingroup netifapi_netif
//   * @see netif_remove()
//   */
// #define netifapi_netif_remove(n)        netifapi_netif_common(n, netif_remove, None)
// /* @ingroup netifapi_netif
//   * @see netif_set_up()
//   */
// #define netifapi_netif_set_up(n)        netifapi_netif_common(n, netif_set_up, None)
// /* @ingroup netifapi_netif
//   * @see netif_set_down()
//   */
// #define netifapi_netif_set_down(n)      netifapi_netif_common(n, netif_set_down, None)
// /* @ingroup netifapi_netif
//   * @see netif_set_default()
//   */
// #define netifapi_netif_set_default(n)   netifapi_netif_common(n, netif_set_default, None)
// /* @ingroup netifapi_netif
//   * @see netif_set_link_up()
//   */
// #define netifapi_netif_set_link_up(n)   netifapi_netif_common(n, netif_set_link_up, None)
// /* @ingroup netifapi_netif
//   * @see netif_set_link_down()
//   */
// #define netifapi_netif_set_link_down(n) netifapi_netif_common(n, netif_set_link_down, None)

/*
 * @defgroup netifapi_dhcp4 DHCPv4
 * @ingroup netifapi
 * To be called from non-TCPIP threads
 */
/* @ingroup netifapi_dhcp4
 * @see dhcp_start()
 */
// #define netifapi_dhcp_start(n)            netifapi_netif_common(n, None, dhcp_start)
/*
 * @ingroup netifapi_dhcp4
 * @deprecated Use netifapi_dhcp_release_and_stop() instead.
 */
// #define netifapi_dhcp_stop(n)             netifapi_netif_common(n, dhcp_stop, None)
/* @ingroup netifapi_dhcp4
 * @see dhcp_inform()
 */
// #define netifapi_dhcp_inform(n)           netifapi_netif_common(n, dhcp_inform, None)
/* @ingroup netifapi_dhcp4
 * @see dhcp_renew()
 */
// #define netifapi_dhcp_renew(n)            netifapi_netif_common(n, None, dhcp_renew)
/*
 * @ingroup netifapi_dhcp4
 * @deprecated Use netifapi_dhcp_release_and_stop() instead.
 */
// #define netifapi_dhcp_release(n)          netifapi_netif_common(n, None, dhcp_release)
/* @ingroup netifapi_dhcp4
 * @see dhcp_release_and_stop()
 */
// #define netifapi_dhcp_release_and_stop(n) netifapi_netif_common(n, dhcp_release_and_stop, None)

/*
 * @defgroup netifapi_autoip AUTOIP
 * @ingroup netifapi
 * To be called from non-TCPIP threads
 */
/* @ingroup netifapi_autoip
 * @see autoip_start()
 */
// #define netifapi_autoip_start(n)      netifapi_netif_common(n, None, autoip_start)
/* @ingroup netifapi_autoip
 * @see autoip_stop()
 */
// #define netifapi_autoip_stop(n)       netifapi_netif_common(n, None, autoip_stop)
