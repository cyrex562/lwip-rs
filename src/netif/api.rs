//*
// @file
// netif API (to be used from non-TCPIP threads)
 //

//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
// 3. The name of the author may not be used to endorse or promote products
//    derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
// SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
// OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
// OF SUCH DAMAGE.
//
// This file is part of the lwIP TCP/IP stack.
//
 //



// #include "lwip/opt.h"

// #if LWIP_NETIF_API // don't build if not configured for use in lwipopts.h //

// #include "lwip/sys.h"
// #include "lwip/netif.h"
// #include "lwip/dhcp.h"
// #include "lwip/autoip.h"
// #include "lwip/priv/tcpip_priv.h"
// #include "lwip/priv/api_msg.h"
// #include "lwip/prot/ethernet.h"




use crate::core::mac_address::MacAddress;
use crate::ipv4::addr::Ipv4Address;
use crate::LwipError;
use crate::netif::netif::NetworkInterface;

// // API for application //
// #if LWIP_ARP && LWIP_IPV4
// // Used for netfiapi_arp_* APIs //
pub enum netifapi_arp_entry {
    NetifapiArpPerm // Permanent entry //
  // // Other entry types can be added here //
}

// //* @ingroup netifapi_arp //
// err_t netifapi_arp_add(const ip4_addr_t *ipaddr, struct eth_addr *ethaddr, enum netifapi_arp_entry type);
// //* @ingroup netifapi_arp //
// err_t netifapi_arp_remove(const ip4_addr_t *ipaddr, enum netifapi_arp_entry type);
//  // LWIP_ARP && LWIP_IPV4 //

// err_t netifapi_netif_add(struct netif *netif,
//
//                          const ip4_addr_t *ipaddr, const ip4_addr_t *netmask, const ip4_addr_t *gw,
//  // LWIP_IPV4 //
//                    void *state, netif_init_fn init, netif_input_fn input);


// err_t netifapi_netif_set_addr(struct netif *netif, const ip4_addr_t *ipaddr,
//                               const ip4_addr_t *netmask, const ip4_addr_t *gw);
 // LWIP_IPV4//

// err_t netifapi_netif_common(struct netif *netif, netifapi_void_fn voidfunc,
//                             netifapi_errt_fn errtfunc);

//* @ingroup netifapi_netif //
// err_t netifapi_netif_name_to_index(const char *name, u8_t *index);
//* @ingroup netifapi_netif //
// err_t netifapi_netif_index_to_name(u8_t index, char *name);

//* @ingroup netifapi_netif
// @see netif_remove()
  //
// #define netifapi_netif_remove(n)        netifapi_netif_common(n, netif_remove, NULL)
//* @ingroup netifapi_netif
// @see netif_set_up()
  //
// #define netifapi_netif_set_up(n)        netifapi_netif_common(n, netif_set_up, NULL)
//* @ingroup netifapi_netif
// @see netif_set_down()
  //
// #define netifapi_netif_set_down(n)      netifapi_netif_common(n, netif_set_down, NULL)
//* @ingroup netifapi_netif
// @see netif_set_default()
  //
// #define netifapi_netif_set_default(n)   netifapi_netif_common(n, netif_set_default, NULL)
//* @ingroup netifapi_netif
// @see netif_set_link_up()
  //
// #define netifapi_netif_set_link_up(n)   netifapi_netif_common(n, netif_set_link_up, NULL)
//* @ingroup netifapi_netif
// @see netif_set_link_down()
  //
// #define netifapi_netif_set_link_down(n) netifapi_netif_common(n, netif_set_link_down, NULL)

//*
// @defgroup netifapi_dhcp4 DHCPv4
// @ingroup netifapi
// To be called from non-TCPIP threads
 //
//* @ingroup netifapi_dhcp4
// @see dhcp_start()
  //
// #define netifapi_dhcp_start(n)            netifapi_netif_common(n, NULL, dhcp_start)
//*
// @ingroup netifapi_dhcp4
// @deprecated Use netifapi_dhcp_release_and_stop() instead.
 //
// #define netifapi_dhcp_stop(n)             netifapi_netif_common(n, dhcp_stop, NULL)
//* @ingroup netifapi_dhcp4
// @see dhcp_inform()
  //
// #define netifapi_dhcp_inform(n)           netifapi_netif_common(n, dhcp_inform, NULL)
//* @ingroup netifapi_dhcp4
// @see dhcp_renew()
  //
// #define netifapi_dhcp_renew(n)            netifapi_netif_common(n, NULL, dhcp_renew)
//*
// @ingroup netifapi_dhcp4
// @deprecated Use netifapi_dhcp_release_and_stop() instead.
 //
// #define netifapi_dhcp_release(n)          netifapi_netif_common(n, NULL, dhcp_release)
//* @ingroup netifapi_dhcp4
// @see dhcp_release_and_stop()
  //
// #define netifapi_dhcp_release_and_stop(n) netifapi_netif_common(n, dhcp_release_and_stop, NULL)

//*
// @defgroup netifapi_autoip AUTOIP
// @ingroup netifapi
// To be called from non-TCPIP threads
 //
//* @ingroup netifapi_autoip
// @see autoip_start()
  //
// #define netifapi_autoip_start(n)      netifapi_netif_common(n, NULL, autoip_start)
//* @ingroup netifapi_autoip
// @see autoip_stop()
  //
// #define netifapi_autoip_stop(n)       netifapi_netif_common(n, NULL, autoip_stop)




 // LWIP_NETIF_API //

 // LWIP_HDR_NETIFAPI_H //



//*
// @file
// Network Interface Sequential API module
//
// @defgroup netifapi NETIF API
// @ingroup sequential_api
// Thread-safe functions to be called from non-TCPIP threads
//
// @defgroup netifapi_netif NETIF related
// @ingroup netifapi
// To be called from non-TCPIP threads
 //

//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
// 3. The name of the author may not be used to endorse or promote products
//    derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
// SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
// OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
// OF SUCH DAMAGE.
//
// This file is part of the lwIP TCP/IP stack.
//
 //



// #if LWIP_NETIF_API // don't build if not configured for use in lwipopts.h //








// #define NETIFAPI_VAR_REF(name)      API_VAR_REF(name)
// #define NETIFAPI_VAR_DECLARE(name)  API_VAR_DECLARE(struct netifapi_msg, name)
// #define NETIFAPI_VAR_ALLOC(name)    API_VAR_ALLOC(struct netifapi_msg, MEMP_NETIFAPI_MSG, name, ERR_MEM)
// #define NETIFAPI_VAR_FREE(name)     API_VAR_FREE(MEMP_NETIFAPI_MSG, name)

//*
// Call netif_add() inside the tcpip_thread context.
 //
// static err_t
// netifapi_do_netif_add(struct tcpip_api_call_data *m)
pub fn netifapi_do_netif_add(m: &tcpip_api_call_data) -> Result<(), LwipError>
{
  // cast through void* to silence alignment warnings.
// We know it works because the structs have been instantiated as struct netifapi_msg //
//   struct netifapi_msg *msg = (struct netifapi_msg *)(void *)m;
//
//   if (!netif_add(  msg.netif,
// // #if LWIP_IPV4
//                   API_EXPR_REF( msg.msg.add.ipaddr),
//                   API_EXPR_REF( msg.msg.add.netmask),
//                   API_EXPR_REF( msg.msg.add.gw),
// // #endif // LWIP_IPV4 //
//                    msg.msg.add.state,
//                    msg.msg.add.init,
//                    msg.msg.add.input)) {
//     return ERR_IF;
//   } else {
//     return ERR_OK;
//   }
    todo!()
}

// #if LWIP_IPV4
//*
// Call netif_set_addr() inside the tcpip_thread context.
 //
// static err_t
// netifapi_do_netif_set_addr(struct tcpip_api_call_data *m)
pub fn netifapi_do_netif_set_addr(m: &tcpip_api_call_data) -> Result<(), LwipError>
{
  // cast through void* to silence alignment warnings.
// We know it works because the structs have been instantiated as struct netifapi_msg //
//   struct netifapi_msg *msg = (struct netifapi_msg *)(void *)m;
//
//   netif_set_addr(  msg.netif,
//                   API_EXPR_REF( msg.msg.add.ipaddr),
//                   API_EXPR_REF( msg.msg.add.netmask),
//                   API_EXPR_REF( msg.msg.add.gw));
//   return ERR_OK;
    todo!()
}
// #endif // LWIP_IPV4 //

//*
// * Call netif_name_to_index() inside the tcpip_thread context.
//
// static err_t
// netifapi_do_name_to_index(struct tcpip_api_call_data *m)
pub fn netifapi_do_name_to_index(m: &tcpip_api_call_data) -> Result<(), LwipError>
{
//   // cast through void* to silence alignment warnings.
// // We know it works because the structs have been instantiated as struct netifapi_msg //
//   struct netifapi_msg *msg = (struct netifapi_msg *)(void *)m;
//
//    msg.msg.ifs.index = netif_name_to_index( msg.msg.ifs.name);
//   return ERR_OK;
    todo!()
}

//*
// * Call netif_index_to_name() inside the tcpip_thread context.
//
// static err_t
// netifapi_do_index_to_name(struct tcpip_api_call_data *m)
pub fn netifapi_do_index_to_name(m: &tcpip_api_call_data)
{
  // cast through void* to silence alignment warnings.
// We know it works because the structs have been instantiated as struct netifapi_msg //
//   struct netifapi_msg *msg = (struct netifapi_msg *)(void *)m;
//
//   if (!netif_index_to_name( msg.msg.ifs.index,  msg.msg.ifs.name)) {
//     // return failure via empty name //
//      msg.msg.ifs.name[0] = '\0';
//   }
//   return ERR_OK;
    todo!()
}

//*
// Call the "errtfunc" (or the "voidfunc" if "errtfunc" is NULL) inside the
// tcpip_thread context.
 //
// static err_t
// netifapi_do_netif_common(struct tcpip_api_call_data *m)
pub fn netifapi_do_netif_common(m: &tcpip_api_call_data) -> Result<(), LwipError>
{
  // cast through void* to silence alignment warnings.
// We know it works because the structs have been instantiated as struct netifapi_msg //
//   struct netifapi_msg *msg = (struct netifapi_msg *)(void *)m;
//
//   if ( msg.msg.common.errtfunc != NULL) {
//     return  msg.msg.common.errtfunc( msg.netif);
//   } else {
//      msg.msg.common.voidfunc( msg.netif);
//     return ERR_OK;
//   }
    todo!()
}

// #if LWIP_ARP && LWIP_IPV4
//*
// @ingroup netifapi_arp
// Add or update an entry in the ARP cache.
// For an update, ipaddr is used to find the cache entry.
//
// @param ipaddr IPv4 address of cache entry
// @param ethaddr hardware address mapped to ipaddr
// @param type type of ARP cache entry
// @return ERR_OK: entry added/updated, else error from err_t
 //
// err_t
// netifapi_arp_add(const ip4_addr_t *ipaddr, struct eth_addr *ethaddr, enum netifapi_arp_entry type)
pub fn netifapi_arp_add(ip_addr: &Ipv4Address, mac_addr: &MacAddress, arp_entry_type: netifapi_arp_entry) -> Result<(), LwipError>
{
//   err_t err;
//
//   // We only support permanent entries currently //
//   LWIP_UNUSED_ARG(type);
//
// // #if ETHARP_SUPPORT_STATIC_ENTRIES && LWIP_TCPIP_CORE_LOCKING
//   LOCK_TCPIP_CORE();
//   err = etharp_add_static_entry(ipaddr, ethaddr);
//   UNLOCK_TCPIP_CORE();
// #else
//   // @todo add new vars to struct netifapi_msg and create a 'do' func //
//   LWIP_UNUSED_ARG(ipaddr);
//   LWIP_UNUSED_ARG(ethaddr);
//   err = ERR_VAL;
// // #endif // ETHARP_SUPPORT_STATIC_ENTRIES && LWIP_TCPIP_CORE_LOCKING //
//
//   return err;
    todo!()
}

//*
// @ingroup netifapi_arp
// Remove an entry in the ARP cache identified by ipaddr
//
// @param ipaddr IPv4 address of cache entry
// @param type type of ARP cache entry
// @return ERR_OK: entry removed, else error from err_t
 //
// err_t
// netifapi_arp_remove(const ip4_addr_t *ipaddr, enum netifapi_arp_entry type)
pub fn netifapi_arp_remove(ip_addr: &Ipv4Address, arp_entry_type: netifapi_arp_entry) -> Result<(), LwipError>
{
//   err_t err;
//
//   // We only support permanent entries currently //
//   LWIP_UNUSED_ARG(type);
//
// // #if ETHARP_SUPPORT_STATIC_ENTRIES && LWIP_TCPIP_CORE_LOCKING
//   LOCK_TCPIP_CORE();
//   err = etharp_remove_static_entry(ipaddr);
//   UNLOCK_TCPIP_CORE();
// #else
//   // @todo add new vars to struct netifapi_msg and create a 'do' func //
//   LWIP_UNUSED_ARG(ipaddr);
//   err = ERR_VAL;
// // #endif // ETHARP_SUPPORT_STATIC_ENTRIES && LWIP_TCPIP_CORE_LOCKING //
//
//   return err;
    todo!()
}
// #endif // LWIP_ARP && LWIP_IPV4 //

//*
// @ingroup netifapi_netif
// Call netif_add() in a thread-safe way by running that function inside the
// tcpip_thread context.
//
// @note for params @see netif_add()
 //
// err_t
// netifapi_netif_add(struct netif *netif,
// // #if LWIP_IPV4
//                    const ip4_addr_t *ipaddr, const ip4_addr_t *netmask, const ip4_addr_t *gw,
// // #endif // LWIP_IPV4 //
//                    void *state, netif_init_fn init, netif_input_fn input)
pub fn netifapi_netif_add(netif: &mut NetworkInterface, ip_addr: &Ipv4Address, netmask: &Ipv4Address, gw: &Ipv4Address, state: &Vec<u8>) -> Result<(), LwipError>
{
//   err_t err;
//   NETIFAPI_VAR_DECLARE(msg);
//   NETIFAPI_VAR_ALLOC(msg);
//
// // #if LWIP_IPV4
//   if (ipaddr == NULL) {
//     ipaddr = IP4_ADDR_ANY4;
//   }
//   if (netmask == NULL) {
//     netmask = IP4_ADDR_ANY4;
//   }
//   if (gw == NULL) {
//     gw = IP4_ADDR_ANY4;
//   }
// // #endif // LWIP_IPV4 //
//
//   NETIFAPI_VAR_REF(msg).netif = netif;
// // #if LWIP_IPV4
//   NETIFAPI_VAR_REF(msg).msg.add.ipaddr  = NETIFAPI_VAR_REF(ipaddr);
//   NETIFAPI_VAR_REF(msg).msg.add.netmask = NETIFAPI_VAR_REF(netmask);
//   NETIFAPI_VAR_REF(msg).msg.add.gw      = NETIFAPI_VAR_REF(gw);
// // #endif // LWIP_IPV4 //
//   NETIFAPI_VAR_REF(msg).msg.add.state   = state;
//   NETIFAPI_VAR_REF(msg).msg.add.init    = init;
//   NETIFAPI_VAR_REF(msg).msg.add.input   = input;
//   err = tcpip_api_call(netifapi_do_netif_add, &API_VAR_REF(msg).call);
//   NETIFAPI_VAR_FREE(msg);
//   return err;
    todo!()
}

// #if LWIP_IPV4
//*
// @ingroup netifapi_netif
// Call netif_set_addr() in a thread-safe way by running that function inside the
// tcpip_thread context.
//
// @note for params @see netif_set_addr()
//  //
// err_t
// netifapi_netif_set_addr(struct netif *netif,
//                         const ip4_addr_t *ipaddr,
//                         const ip4_addr_t *netmask,
//                         const ip4_addr_t *gw)
pub fn netifapi_netif_set_addr(netif: &mut NetworkInterface, ip_addr: &Ipv4Address, netmask: &Ipv4Address, gw: &Ipv4Address) -> Result<(), LwipError>
{
  // err_t err;
  // NETIFAPI_VAR_DECLARE(msg);
  // NETIFAPI_VAR_ALLOC(msg);
  //
  // if (ipaddr == NULL) {
  //   ipaddr = IP4_ADDR_ANY4;
  // }
  // if (netmask == NULL) {
  //   netmask = IP4_ADDR_ANY4;
  // }
  // if (gw == NULL) {
  //   gw = IP4_ADDR_ANY4;
  // }
  //
  // NETIFAPI_VAR_REF(msg).netif = netif;
  // NETIFAPI_VAR_REF(msg).msg.add.ipaddr  = NETIFAPI_VAR_REF(ipaddr);
  // NETIFAPI_VAR_REF(msg).msg.add.netmask = NETIFAPI_VAR_REF(netmask);
  // NETIFAPI_VAR_REF(msg).msg.add.gw      = NETIFAPI_VAR_REF(gw);
  // err = tcpip_api_call(netifapi_do_netif_set_addr, &API_VAR_REF(msg).call);
  // NETIFAPI_VAR_FREE(msg);
  // return err;
    todo!()
}
// #endif // LWIP_IPV4 //

//*
// call the "errtfunc" (or the "voidfunc" if "errtfunc" is NULL) in a thread-safe
// way by running that function inside the tcpip_thread context.
//
// @note use only for functions where there is only "netif" parameter.
 //
// err_t
// netifapi_netif_common(struct netif *netif, netifapi_void_fn voidfunc,
//                       netifapi_errt_fn errtfunc)
pub fn netifapi_netif_common(netif: &mut NetworkInterface) -> Result<(), LwipError>
{
  // err_t err;
  // NETIFAPI_VAR_DECLARE(msg);
  // NETIFAPI_VAR_ALLOC(msg);
  //
  // NETIFAPI_VAR_REF(msg).netif = netif;
  // NETIFAPI_VAR_REF(msg).msg.common.voidfunc = voidfunc;
  // NETIFAPI_VAR_REF(msg).msg.common.errtfunc = errtfunc;
  // err = tcpip_api_call(netifapi_do_netif_common, &API_VAR_REF(msg).call);
  // NETIFAPI_VAR_FREE(msg);
  // return err;
    todo!()
}

//*
// * @ingroup netifapi_netif
// * Call netif_name_to_index() in a thread-safe way by running that function inside the
// * tcpip_thread context.
// *
// * @param name the interface name of the netif
// * @param idx output index of the found netif
//
// err_t
// netifapi_netif_name_to_index(const char *name, u8_t *idx)
pub fn netifapi_netif_name_to_index(name: &str, idx: &u8) -> Result<(), LwipError>
{
//   err_t err;
//   NETIFAPI_VAR_DECLARE(msg);
//   NETIFAPI_VAR_ALLOC(msg);
// //idx = 0;
//
// // #if LWIP_MPU_COMPATIBLE
//   strncpy(NETIFAPI_VAR_REF(msg).msg.ifs.name, name, NETIF_NAMESIZE - 1);
//   NETIFAPI_VAR_REF(msg).msg.ifs.name[NETIF_NAMESIZE - 1] = '\0';
// #else
//   NETIFAPI_VAR_REF(msg).msg.ifs.name = LWIP_CONST_CAST(char *, name);
// // #endif // LWIP_MPU_COMPATIBLE //
//   err = tcpip_api_call(netifapi_do_name_to_index, &API_VAR_REF(msg).call);
//   if (!err) {
// //idx = NETIFAPI_VAR_REF(msg).msg.ifs.index;
//   }
//   NETIFAPI_VAR_FREE(msg);
//   return err;
    todo!()
}

//*
// * @ingroup netifapi_netif
// * Call netif_index_to_name() in a thread-safe way by running that function inside the
// * tcpip_thread context.
// *
// * @param idx the interface index of the netif
// * @param name output name of the found netif, empty '\0' string if netif not found.
// *             name should be of at least NETIF_NAMESIZE bytes
//
// err_t
// netifapi_netif_index_to_name(u8_t idx, char *name)
pub fn netifapi_netif_index_to_name(idx: u8, name: &str) -> Result<(), LwipError>
{
//   err_t err;
//   NETIFAPI_VAR_DECLARE(msg);
//   NETIFAPI_VAR_ALLOC(msg);
//
//   NETIFAPI_VAR_REF(msg).msg.ifs.index = idx;
// #if !LWIP_MPU_COMPATIBLE
//   NETIFAPI_VAR_REF(msg).msg.ifs.name = name;
// // #endif // LWIP_MPU_COMPATIBLE //
//   err = tcpip_api_call(netifapi_do_index_to_name, &API_VAR_REF(msg).call);
// // #if LWIP_MPU_COMPATIBLE
//   if (!err) {
//     strncpy(name, NETIFAPI_VAR_REF(msg).msg.ifs.name, NETIF_NAMESIZE - 1);
//     name[NETIF_NAMESIZE - 1] = '\0';
//   }
// // #endif // LWIP_MPU_COMPATIBLE //
//   NETIFAPI_VAR_FREE(msg);
//   return err;
    todo!()
}

// #endif // LWIP_NETIF_API //
