/*
 * @file
 *
 * DHCPv6 client: IPv6 address autoconfiguration as per
 * RFC 3315 (stateful DHCPv6) and
 * RFC 3736 (stateless DHCPv6).
 */

/*
 * Copyright (c) 2018 Simon Goldschmidt
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 */

// #define LWIP_HDR_IP6_DHCP6_H

/* period (in milliseconds) of the application calling dhcp6_tmr() */
pub const DHCP6_TIMER_MSECS: u64 = 500;

pub struct dhcp6 {
    /* transaction identifier of last sent request */
    pub xid: u32,
    /* track PCB allocation state */
    pub pcb_allocated: u8,
    /* current DHCPv6 state machine state */
    pub state: u8,
    /* retries of current request */
    pub tries: u8,
    /* if request config is triggered while another action is active, this keeps track of it */
    pub request_config_pending: u8,
    /* #ticks with period DHCP6_TIMER_MSECS for request timeout */
    pub request_timeout: u16,
    /* @todo: add more members here to keep track of stateful DHCPv6 data, like lease times */
}

// pub fn  dhcp6_set_struct(netif: &mut NetIfc, dhcp6: &mut dhcp6);
/* Remove a struct dhcp6 previously set to the netif using dhcp6_set_struct() */
// #define dhcp6_remove_struct(netif) netif_set_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_DHCP6, NULL)
// pub fn  dhcp6_cleanup(netif: &mut NetIfc);

// pub fn  dhcp6_enable_stateful(netif: &mut NetIfc);
// pub fn  dhcp6_enable_stateless(netif: &mut NetIfc);
// pub fn  dhcp6_disable(netif: &mut NetIfc);

// pub fn  dhcp6_tmr();

// pub fn  dhcp6_nd6_ra_trigger(netif: &mut NetIfc, managed_addr_config: u8, other_config: u8);

/* This function must exist, in other to add offered NTP servers to
 * the NTP (or SNTP) engine.
 * See LWIP_DHCP6_MAX_NTP_SERVERS */
// extern void dhcp6_set_ntp_servers(num_ntp_servers: u8,  ntp_server_addrs: &mut LwipAddr);

pub fn netif_dhcp6_data(netif: netif) {
    (netif_get_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_DHCP6))
}
