// /**
//  * @file
//  * AutoIP Automatic LinkLocal IP Configuration
//  *
//  * This is a AutoIP implementation for the lwIP TCP/IP stack. It aims to conform
//  * with RFC 3927. It uses IPv4 address conflict detection to evaluate the chosen
//  * address. The ACD module aims to be conform to RFC 5227.
//  * RFC 5227 is extracted out of RFC 3927 so the acd module fits nicely in autoip.
//  *
//  * @defgroup autoip AUTOIP
//  * @ingroup ip4
//  * AUTOIP related functions
//  * USAGE:
//  *
//  * define @ref LWIP_AUTOIP 1 in your lwipopts.h
//  *
//  * Without DHCP:
//  * - Call autoip_start() after netif_add().
//  *
//  * With DHCP:
//  * - define @ref LWIP_DHCP_AUTOIP_COOP 1 in your lwipopts.h.
//  * - Configure your DHCP Client.
//  *
//  * @see netifapi_autoip
//  */

// /*
//  *
//  * Copyright (c) 2007 Dominik Spies <kontakt@dspies.de>
//  * All rights reserved.
//  *
//  * Redistribution and use in source and binary forms, with or without modification,
//  * are permitted provided that the following conditions are met:
//  *
//  * 1. Redistributions of source code must retain the above copyright notice,
//  *    this list of conditions and the following disclaimer.
//  * 2. Redistributions in binary form must reproduce the above copyright notice,
//  *    this list of conditions and the following disclaimer in the documentation
//  *    and/or other materials provided with the distribution.
//  * 3. The name of the author may not be used to endorse or promote products
//  *    derived from this software without specific prior written permission.
//  *
//  * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
//  * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
//  * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
//  * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
//  * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
//  * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
//  * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
//  * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
//  * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
//  * OF SUCH DAMAGE.
//  *
//  * Author: Dominik Spies <kontakt@dspies.de>
//  */



// #if LWIP_IPV4 && LWIP_AUTOIP /* don't build if not configured for use in lwipopts.h */


/* #include "lwip/udp.h" */




use crate::ipv4::acd::AcdCallbackResult;
use crate::ipv4::addr::Ipv4Address;
use crate::LwipError;
use crate::netif::netif::NetworkInterface;

/** AutoIP state information per netif */
// struct autoip
// {
//   /** the currently selected, probed, announced or used LL IP-Address */
//  llipaddr: ip4_addr_t;
//   /** current AutoIP state machine state */
//   state: u8;
//   /** total number of probed/used Link Local IP-Addresses */
//   tried_llipaddr: u8;
//   /** acd struct */
//   struct acd acd;
// };
pub struct AutoIPStateInfo {
    pub llipaddr: Ipv4Address,
    pub state: AutoIPState,
    pub tried_llipaddr: usize,
    // acd struct
}


/* 169.254.0.0 */
pub const AUTOIP_NET: u32 = 0xA9FE0000; /* 169.254.1.0 */
pub const AUTOIP_RANGE_START: u32 =      (AUTOIP_NET | 0x0100);
/* 169.254.254.255 */
pub const AUTOIP_RANGE_END: u32 =        (AUTOIP_NET | 0xFEFF);

/* AutoIP client states */
pub enum AutoIPState {
    AutoipStateOff,
    AutoipStateChecking,
    AutoipStateBound
}







// /**
//  * Macro that generates the initial IP address to be tried by AUTOIP.
//  * If you want to override this, define it to something else in lwipopts.h.
//  */
// #ifndef LWIP_AUTOIP_CREATE_SEED_ADDR
// #define LWIP_AUTOIP_CREATE_SEED_ADDR(netif) \
//   lwip_htonl(AUTOIP_RANGE_START + ((u32_t)((( netif.hwaddr[4])) | \
//                  ((u32_t)(( netif.hwaddr[5]))) << 8)))
// #endif /* LWIP_AUTOIP_CREATE_SEED_ADDR */

// /**
//  * @ingroup autoip
//  * Set a statically allocated struct autoip to work with.
//  * Using this prevents autoip_start to allocate it using mem_malloc.
//  *
//  * @param netif the netif for which to set the struct autoip
//  * @param autoip (uninitialised) autoip struct allocated by the application
//  */
pub fn autoip_set_struct(netif: &mut NetworkInterface, autoip: &mut AutoIPStateInfo)
{
  // // LWIP_ASSERT_CORE_LOCKED()
  // // LWIP_ASSERT("netif != NULL", netif != NULL);
  // // LWIP_ASSERT("autoip != NULL", autoip != NULL);
  // // LWIP_ASSERT("netif already has a struct autoip set",
  //             netif_autoip_data(netif) == NULL);
  //
  // /* clear data structure */
  // memset(autoip, 0, sizeof(struct autoip));
  // /*  autoip.state = AutoipStateOff; */
  // netif_set_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_AUTOIP, autoip);
    todo!()
}

// /**
//  * @ingroup autoip
//  * Remove a struct autoip previously set to the netif using autoip_set_struct()
//  *
//  * @param netif the netif for which to set the struct autoip
//  */
pub fn autoip_remove_struct(netif: &mut NetworkInterface)
{
  // // LWIP_ASSERT_CORE_LOCKED()
  // // LWIP_ASSERT("netif != NULL", netif != NULL);
  // // LWIP_ASSERT("netif has no struct autoip set",
  //             netif_autoip_data(netif) != NULL);
  //
  // netif_set_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_AUTOIP, NULL);
    todo!()
}

// /** Restart AutoIP client and check the next address (conflict detected)
//  *
//  * @param netif The netif under AutoIP control
//  */
pub fn autoip_restart(netif: &mut NetworkInterface)
{
  // struct autoip *autoip = netif_autoip_data(netif);
  //  autoip.tried_llipaddr++;
  // autoip_start(netif);
    todo!()
}


// /**
//  * Create an IP-Address out of range 169.254.1.0 to 169.254.254.255
//  *
//  * @param netif network interface on which create the IP-Address
//  * @param ipaddr ip address to initialize
//  */
pub fn autoip_create_addr(netif: &mut NetworkInterface, ip_addr: &Ipv4Address)
{
  // struct autoip *autoip = netif_autoip_data(netif);
  //
  // /* Here we create an IP-Address out of range 169.254.1.0 to 169.254.254.255
  //  * compliant to RFC 3927 Section 2.1
  //  * We have 254 * 256 possibilities */
  //
  // u32_t addr = lwip_ntohl(LWIP_AUTOIP_CREATE_SEED_ADDR(netif));
  // addr +=  autoip.tried_llipaddr;
  // addr = AUTOIP_NET | (addr & 0xffff);
  // /* Now, 169.254.0.0 <= addr <= 169.254.255.255 */
  //
  // if (addr < AUTOIP_RANGE_START) {
  //   addr += AUTOIP_RANGE_END - AUTOIP_RANGE_START + 1;
  // }
  // if (addr > AUTOIP_RANGE_END) {
  //   addr -= AUTOIP_RANGE_END - AUTOIP_RANGE_START + 1;
  // }
  // // LWIP_ASSERT("AUTOIP address not in range", (addr >= AUTOIP_RANGE_START) &&
  //             (addr <= AUTOIP_RANGE_END));
  // ip4_addr_set_u32(ipaddr, lwip_htonl(addr));
  //
  // LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
  //             ("autoip_create_addr(): tried_llipaddr=%"U16_F", %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
  //              (u16_t)( autoip.tried_llipaddr), ip4_addr1_16(ipaddr), ip4_addr2_16(ipaddr),
  //              ip4_addr3_16(ipaddr), ip4_addr4_16(ipaddr)));
    todo!()
}


// /**
//  * Configure interface for use with current LL IP-Address
//  *
//  * @param netif network interface to configure with current LL IP-Address
//  */
pub fn autoip_bind(netif: &mut NetworkInterface) -> Result<(), LwipError>
{
  // struct autoip *autoip = netif_autoip_data(netif);
  // ip4_addr_t sn_mask, gw_addr;
  //
  //  autoip.state = AUTOIP_STATE_BOUND;
  //
  // LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE,
  //             ("autoip_bind(netif=%p) %c%c%"U16_F" %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
  //              (void *)netif,  netif.name[0],  netif.name[1], (u16_t) netif.num,
  //              ip4_addr1_16(& autoip.llipaddr), ip4_addr2_16(& autoip.llipaddr),
  //              ip4_addr3_16(& autoip.llipaddr), ip4_addr4_16(& autoip.llipaddr)));
  //
  // IP4_ADDR(&sn_mask, 255, 255, 0, 0);
  // IP4_ADDR(&gw_addr, 0, 0, 0, 0);
  //
  // netif_set_addr(netif, & autoip.llipaddr, &sn_mask, &gw_addr);
  // /* interface is used by routing now that an address is set */
  //
  // return ERR_OK;
    todo!()
}

// /**
// * Handle conflict information from ACD module
// *
// * @param netif   network interface to handle conflict information on
// * @param state   acd_callback_enum_t
//  */
pub fn autoip_conflict_callback(netif: &mut NetworkInterface, state: AcdCallbackResult)
{
  // struct autoip *autoip = netif_autoip_data(netif);
  //
  // switch (state) {
  //   case ACD_IP_OK:
  //     autoip_bind(netif);
  //     break;
  //   case ACD_RESTART_CLIENT:
  //     autoip_restart(netif);
  //     break;
  //   case ACD_DECLINE:
  //     /* "delete" conflicting address so a new one will be selected in
  //      * autoip_start() */
  //     ip4_addr_set_any(& autoip.llipaddr);
  //     autoip_stop(netif);
  //     break;
  //     default:
  //     break;
  // }
    todo!()
}

// /**
//  * @ingroup autoip
//  * Start AutoIP client
//  *
//  * @param netif network interface on which start the AutoIP client
//  */
pub fn autoip_start(netif: &mut NetworkInterface) -> Result<(), LwipError>
{
  // struct autoip *autoip = netif_autoip_data(netif);
  // err_t result = ERR_OK;
  //
  // // LWIP_ASSERT_CORE_LOCKED()
  // LWIP_ERROR("netif is not up, old style port?", netif_is_up(netif), return ERR_ARG;);
  //
  // if (autoip == NULL) {
  //   /* no AutoIP client attached yet? */
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE,
  //               ("autoip_start(): starting new AUTOIP client\n"));
  //   autoip = (struct autoip *)mem_calloc(1, sizeof(struct autoip));
  //   if (autoip == NULL) {
  //     LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE,
  //                 ("autoip_start(): could not allocate autoip\n"));
  //     return ERR_MEM;
  //   }
  //   /* store this AutoIP client in the netif */
  //   netif_set_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_AUTOIP, autoip);
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE, ("autoip_start(): allocated autoip"));
  // }
  //
  // if ( autoip.state == AUTOIP_STATE_OFF) {
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
  //               ("autoip_start(netif=%p) %c%c%"U16_F"\n", (void *)netif,  netif.name[0],
  //                 netif.name[1], (u16_t) netif.num));
  //
  //   /* add acd struct to list*/
  //   acd_add(netif, & autoip.acd, autoip_conflict_callback);
  //
  //   /* In accordance to RFC3927 section 2.1:
  //    * Keep using the same link local address as much as possible.
  //    * Only when there is none or when there was a conflict, select a new one.
  //    */
  //   if (!ip4_addr_islinklocal(& autoip.llipaddr)) {
  //     autoip_create_addr(netif, &( autoip.llipaddr));
  //   }
  //    autoip.state = AUTOIP_STATE_CHECKING;
  //   acd_start(netif, & autoip.acd,  autoip.llipaddr);
  // } else {
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
  //               ("autoip_start(): already started on netif=%p %c%c%"U16_F"\n",
  //               (void *)netif,  netif.name[0],
  //                 netif.name[1], (u16_t) netif.num));
  // }
  //
  // return result;
    todo!()
}


// /**
//  * Handle a possible change in the network configuration: link up
//  *
//  * If there is an AutoIP address configured and AutoIP is not in cooperation
//  * with DHCP, start probing for previous address.
//  */
pub fn autoip_network_changed_link_up(netif: &mut NetworkInterface)
{
  // struct autoip *autoip = netif_autoip_data(netif);
  //
  // if (autoip && ( autoip.state != AUTOIP_STATE_OFF) && !LWIP_DHCP_AUTOIP_COOP) {
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE,
  //               ("autoip_network_changed_link_up(): start acd"));
  //    autoip.state = AUTOIP_STATE_CHECKING;
  //   /* Start acd check again for the last used address */
  //   acd_start(netif, & autoip.acd,  autoip.llipaddr);
  // }
    todo!()
}

// /**
//  * Handle a possible change in the network configuration: link down
//  *
//  * If there is an AutoIP address configured and AutoIP is in cooperation
//  * with DHCP, then stop the autoip module. When the link goes up, we do not want
//  * the autoip module to start again. DHCP will initiate autoip when needed.
//  */
pub fn autoip_network_changed_link_down(netif: &mut NetworkInterface)
{
  // struct autoip *autoip = netif_autoip_data(netif);
  //
  // if (autoip && ( autoip.state != AUTOIP_STATE_OFF) && LWIP_DHCP_AUTOIP_COOP) {
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE,
  //               ("autoip_network_changed_link_down(): stop autoip"));
  //   autoip_stop(netif);
  // }
    todo!()
}

// /**
//  * @ingroup autoip
//  * Stop AutoIP client
//  *
//  * @param netif network interface on which stop the AutoIP client
//  */
pub fn autoip_stop(netif: &mut NetworkInterface) -> Result<(), LwipError>
{
  // struct autoip *autoip = netif_autoip_data(netif);
  //
  // // LWIP_ASSERT_CORE_LOCKED()
  // if (autoip != NULL) {
  //    autoip.state = AUTOIP_STATE_OFF;
  //   if (ip4_addr_islinklocal(netif_ip4_addr(netif))) {
  //     netif_set_addr(netif, IP4_ADDR_ANY4, IP4_ADDR_ANY4, IP4_ADDR_ANY4);
  //   }
  //   LWIP_DEBUGF(AUTOIP_DEBUG | LWIP_DBG_TRACE,("autoip_stop()"));
  // }
  // return ERR_OK;
    todo!()
}

// /** check if AutoIP supplied  netif.ip_addr
//  *
//  * @param netif the netif to check
//  * @return 1 if AutoIP supplied  netif.ip_addr (state BOUND),
//  *         0 otherwise
//  */

pub fn autoip_supplied_address(netif: &mut NetworkInterface) -> bool
{
  // struct autoip *autoip = netif_autoip_data(netif);
  // return     (autoip != NULL)
  //         && (ip4_addr_eq(netif_ip4_addr(netif), &( autoip.llipaddr)))
  //         && ( autoip.state == AUTOIP_STATE_BOUND);
    todo!()
}


pub fn autoip_accept_packet(netif: &mut NetworkInterface, addr: &Ipv4Address) -> bool
{
  // struct autoip *autoip = netif_autoip_data(netif);
  // return     (autoip != NULL)
  //         && (ip4_addr_eq(addr, &( autoip.llipaddr)))
  //         && ( autoip.state == AUTOIP_STATE_BOUND);
    todo!()
}

// #endif /* LWIP_IPV4 && LWIP_AUTOIP */
