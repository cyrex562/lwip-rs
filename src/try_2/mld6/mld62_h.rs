/*
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

//

//  MLD group
pub struct MldGroup {
    //  next link
    // next: &mut mld_group;
    //  multicast address
    pub group_address: LwipAddr,
    //  signifies we were the last person to report
    pub last_reporter_flag: bool,
    //  current state of the group
    pub group_state: bool,
    //  timer for reporting
    pub timer: u64,
    //  counter of simultaneous uses
    pub uses: usize,
}

pub const MLD6_TMR_INTERVAL: u64 = 100; //  Milliseconds

// pub fn   mld6_stop(netif: &mut NetIfc);
// pub fn    mld6_report_groups(netif: &mut NetIfc);
// pub fn    mld6_tmr();
// mld6_lookfor_group: &mut mld_group(ifp: &mut NetIfc,  addr: &mut ip6_addr_t);
// pub fn    mld6_input(p: &mut PacketBuffer, inp: &mut NetIfc);
// pub fn   mld6_joingroup( srcaddr: &mut ip6_addr_t,  groupaddr: &mut ip6_addr_t);
// pub fn   mld6_joingroup_netif(netif: &mut NetIfc,  groupaddr: &mut ip6_addr_t);
// pub fn   mld6_leavegroup( srcaddr: &mut ip6_addr_t,  groupaddr: &mut ip6_addr_t);
// pub fn   mld6_leavegroup_netif(netif: &mut NetIfc,  groupaddr: &mut ip6_addr_t);

/* @ingroup mld6
 * Get list head of MLD6 groups for netif.
 * Note: The allnodes group IP is NOT in the list, since it must always
 * be received for correct IPv6 operation.
 * @see @ref netif_set_mld_mac_filter()
 */
// #define netif_mld6_data(netif) (netif_get_client_data(netif, LwipNetifClientDataIndexMld6))
