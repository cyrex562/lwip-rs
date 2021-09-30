use std::net::IpAddr;

/*
 * @file
 * DHCP client API
 */

/*
 * Copyright (c) 2001-2004 Leon Woestenberg <leon.woestenberg@gmx.net>
 * Copyright (c) 2001-2004 Axon Digital Design B.V., The Netherlands.
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
 * Author: Leon Woestenberg <leon.woestenberg@gmx.net>
 *
 */

// #define LWIP_HDR_DHCP_H

//  period (in seconds) of the application calling dhcp_coarse_tmr() 
pub const DHCP_COARSE_TIMER_SECS: u32 = 60;
//  period (in milliseconds) of the application calling dhcp_coarse_tmr() 
pub const DHCP_COARSE_TIMER_MSECS: u32 = (DHCP_COARSE_TIMER_SECS * 1000);
//  period (in milliseconds) of the application calling dhcp_fine_tmr() 
pub const DHCP_FINE_TIMER_MSECS: u32 = 500;

pub const DHCP_BOOT_FILE_LEN: usize = 128;

//  AutoIP cooperation flags (struct dhcp.autoip_coop_state) 
pub enum dhcp_autoip_coop_state_enum_t {
    DHCP_AUTOIP_COOP_STATE_OFF = 0,
    DHCP_AUTOIP_COOP_STATE_ON = 1,
}

pub struct dhcp {
    //  transaction identifier of last sent request 
    pub xid: u32,
    //  track PCB allocation state 
    pub pcb_allocated: u8,
    //  current DHCP state machine state 
    pub state: u8,
    //  retries of current request 
    pub tries: u8,

    pub autoip_coop_state: u8,

    pub subnet_mask_given: u8,

    pub request_timeout: u16, //  #ticks with period DHCP_FINE_TIMER_SECS for request timeout 
    pub t1_timeout: u16,      //  #ticks with period DHCP_COARSE_TIMER_SECS for renewal time 
    pub t2_timeout: u16,      //  #ticks with period DHCP_COARSE_TIMER_SECS for rebind time 
    pub t1_renew_time: u16,   //  #ticks with period DHCP_COARSE_TIMER_SECS until next renew try 
    pub t2_rebind_time: u16,  //  #ticks with period DHCP_COARSE_TIMER_SECS until next rebind try 
    pub lease_used: u16, //  #ticks with period DHCP_COARSE_TIMER_SECS since last received DHCP ack 
    pub t0_timeout: u16, //  #ticks with period DHCP_COARSE_TIMER_SECS for lease time 
    pub server_ip_addr: LwipAddr, //  dhcp server address that offered this lease (LwipAddr because passed to UDP) 
    pub offered_ip_addr: ip4_addr,
    pub offered_sn_mask: ip4_addr,
    pub offered_gw_addr: ip4_addr,

    pub offered_t0_lease: u32,  //  lease period (in seconds) 
    pub offered_t1_renew: u32,  //  recommended renew time (usually 50% of lease period) 
    pub offered_t2_rebind: u32, //  recommended rebind time (usually 87.5 of lease period)  

    pub offered_si_addr: ip4_addr,
    pub boot_file_name: String,
}

// pub fn  dhcp_set_struct(netif: &mut NetIfc, dhcp: &mut dhcp);
//  Remove a struct dhcp previously set to the netif using dhcp_set_struct() 
// #define dhcp_remove_struct(netif) netif_set_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_DHCP, NULL)
// pub fn  dhcp_cleanup(netif: &mut NetIfc);
// pub fn  dhcp_start(netif: &mut NetIfc);
// pub fn  dhcp_renew(netif: &mut NetIfc);
// pub fn  dhcp_release(netif: &mut NetIfc);
// pub fn  dhcp_stop(netif: &mut NetIfc);
// pub fn  dhcp_release_and_stop(netif: &mut NetIfc);
// pub fn  dhcp_inform(netif: &mut NetIfc);
// pub fn  dhcp_network_changed(netif: &mut NetIfc);
// pub fn  dhcp_arp_reply(netif: &mut NetIfc,  addr: &mut LwipAddr);

// dhcp_supplied_address: u8( netif: &mut NetIfc);
//  to be called every minute 
// pub fn  dhcp_coarse_tmr();
//  to be called every half second 
// pub fn  dhcp_fine_tmr();

/* This function must exist, in other to add offered NTP servers to
 * the NTP (or SNTP) engine.
 * See lwip_dhcp_max_ntp_servers */
// extern void dhcp_set_ntp_servers(num_ntp_servers: u8,  ip4_addr* ntp_server_addrs);

// #define netif_dhcp_data(netif) ((struct dhcp*)netif_get_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_DHCP))
