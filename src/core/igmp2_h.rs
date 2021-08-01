/*
 * @file
 * IGMP API
 */

/*
 * Copyright (c) 2002 CITEL Technologies Ltd.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of CITEL Technologies Ltd nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY CITEL TECHNOLOGIES AND CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL CITEL TECHNOLOGIES OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * This file is a contribution to the lwIP TCP/IP stack.
 * The Swedish Institute of Computer Science and Adam Dunkels
 * are specifically granted permission to redistribute this
 * source code.
*/


// #define LWIP_HDR_IGMP_H












/* IGMP timer */
#define IGMP_TMR_INTERVAL              100 /* Milliseconds */
#define IGMP_V1_DELAYING_MEMBER_TMR   (1000/IGMP_TMR_INTERVAL)
#define IGMP_JOIN_DELAYING_MEMBER_TMR (500 /IGMP_TMR_INTERVAL)

/* Compatibility defines (don't use for new code) */
#define IGMP_DEL_MAC_FILTER            NETIF_DEL_MAC_FILTER
#define IGMP_ADD_MAC_FILTER            NETIF_ADD_MAC_FILTER

/*
 * igmp group structure - there is
 * a list of groups for each interface
 * these should really be linked from the interface, but
 * if we keep them separate we will not affect the lwip original code
 * too much
 *
 * There will be a group for the all systems group address but this
 * will not run the state machine as it is used to kick off reports
 * from all the other groups
 */
struct igmp_group {
  /* next link */
  next: &mut igmp_group;
  /* multicast address */
  ip4_addr_t         group_address;
  /* signifies we were the last person to report */
  u8               last_reporter_flag;
  /* current state of the group */
  u8               group_state;
  /* timer for reporting, negative is OFF */
  u16              timer;
  /* counter of simultaneous uses */
  u8               use;
};

/*  Prototypes */
pub fn    igmp_init();
pub fn   igmp_start(netif: &mut netif);
pub fn   igmp_stop(netif: &mut netif);
pub fn    igmp_report_groups(netif: &mut netif);
igmp_lookfor_group: &mut igmp_group(ifp: &mut netif, const addr: &mut ip4_addr_t);
pub fn    igmp_input(p: &mut pbuf, inp: &mut netif, const dest: &mut ip4_addr_t);
pub fn   igmp_joingroup(const ifaddr: &mut ip4_addr_t, const groupaddr: &mut ip4_addr_t);
pub fn   igmp_joingroup_netif(netif: &mut netif, const groupaddr: &mut ip4_addr_t);
pub fn   igmp_leavegroup(const ifaddr: &mut ip4_addr_t, const groupaddr: &mut ip4_addr_t);
pub fn   igmp_leavegroup_netif(netif: &mut netif, const groupaddr: &mut ip4_addr_t);
pub fn    igmp_tmr();

/* @ingroup igmp
 * Get list head of IGMP groups for netif.
 * Note: The allsystems group IP is contained in the list as first entry.
 * @see @ref netif_set_igmp_mac_filter()
 */
#define netif_igmp_data(netif) ((struct igmp_group *)netif_get_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_IGMP))


}





