/**
 * @file
 *
 * Multicast listener discovery for IPv6. Aims to be compliant with RFC 2710.
 * No support for MLDv2.
 */

/*
 * Copyright (c) 2010 Inico Technologies Ltd.
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
 * Author: Ivan Delamer <delamer@inicotech.com>
 *
 *
 * Please coordinate changes and requests with Ivan Delamer
 * <delamer@inicotech.com>
 */


#define LWIP_HDR_MLD6_H









extern "C" {


/** MLD group */
struct mld_group {
  /** next link */
  next: &mut mld_group;
  /** multicast address */
  ip6_addr_t         group_address;
  /** signifies we were the last person to report */
  u8               last_reporter_flag;
  /** current state of the group */
  u8               group_state;
  /** timer for reporting */
  u16              timer;
  /** counter of simultaneous uses */
  u8               use;
};

#define MLD6_TMR_INTERVAL              100 /* Milliseconds */

pub fn   mld6_stop(netif: &mut netif);
pub fn    mld6_report_groups(netif: &mut netif);
pub fn    mld6_tmr(void);
mld6_lookfor_group: &mut mld_group(ifp: &mut netif, const ip6_addr_t *addr);
pub fn    mld6_input(p: &mut pbuf, inp: &mut netif);
pub fn   mld6_joingroup(const ip6_addr_t *srcaddr, const ip6_addr_t *groupaddr);
pub fn   mld6_joingroup_netif(netif: &mut netif, const ip6_addr_t *groupaddr);
pub fn   mld6_leavegroup(const ip6_addr_t *srcaddr, const ip6_addr_t *groupaddr);
pub fn   mld6_leavegroup_netif(netif: &mut netif, const ip6_addr_t *groupaddr);

/** @ingroup mld6
 * Get list head of MLD6 groups for netif.
 * Note: The allnodes group IP is NOT in the list, since it must always 
 * be received for correct IPv6 operation.
 * @see @ref netif_set_mld_mac_filter()
 */
#define netif_mld6_data(netif) ((struct mld_group *)netif_get_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_MLD6))


}





