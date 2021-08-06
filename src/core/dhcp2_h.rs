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












/* period (in seconds) of the application calling dhcp_coarse_tmr() */
#define DHCP_COARSE_TIMER_SECS  60
/* period (in milliseconds) of the application calling dhcp_coarse_tmr() */
#define DHCP_COARSE_TIMER_MSECS (DHCP_COARSE_TIMER_SECS * 1000)
/* period (in milliseconds) of the application calling dhcp_fine_tmr() */
#define DHCP_FINE_TIMER_MSECS   500

#define DHCP_BOOT_FILE_LEN      128U

/* AutoIP cooperation flags (struct dhcp.autoip_coop_state) */
typedef enum {
  DHCP_AUTOIP_COOP_STATE_OFF  = 0,
  DHCP_AUTOIP_COOP_STATE_ON   = 1
} dhcp_autoip_coop_state_enum_t;

struct dhcp
{
  /* transaction identifier of last sent request */
  xid: u32;
  /* track PCB allocation state */
  pcb_allocated: u8;
  /* current DHCP state machine state */
  state: u8;
  /* retries of current request */
  tries: u8;

  autoip_coop_state: u8;

  subnet_mask_given: u8;

  request_timeout: u16; /* #ticks with period DHCP_FINE_TIMER_SECS for request timeout */
  t1_timeout: u16;  /* #ticks with period DHCP_COARSE_TIMER_SECS for renewal time */
  t2_timeout: u16;  /* #ticks with period DHCP_COARSE_TIMER_SECS for rebind time */
  t1_renew_time: u16;  /* #ticks with period DHCP_COARSE_TIMER_SECS until next renew try */
  t2_rebind_time: u16; /* #ticks with period DHCP_COARSE_TIMER_SECS until next rebind try */
  lease_used: u16; /* #ticks with period DHCP_COARSE_TIMER_SECS since last received DHCP ack */
  t0_timeout: u16; /* #ticks with period DHCP_COARSE_TIMER_SECS for lease time */
  ip_addr_t server_ip_addr; /* dhcp server address that offered this lease (ip_addr_t because passed to UDP) */
  ip4_addr offered_ip_addr;
  ip4_addr offered_sn_mask;
  ip4_addr offered_gw_addr;

  offered_t0_lease: u32; /* lease period (in seconds) */
  offered_t1_renew: u32; /* recommended renew time (usually 50% of lease period) */
  offered_t2_rebind: u32; /* recommended rebind time (usually 87.5 of lease period)  */

  ip4_addr offered_si_addr;
  char boot_file_name[DHCP_BOOT_FILE_LEN];

};


pub fn  dhcp_set_struct(netif: &mut netif, dhcp: &mut dhcp);
/* Remove a struct dhcp previously set to the netif using dhcp_set_struct() */
#define dhcp_remove_struct(netif) netif_set_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_DHCP, NULL)
pub fn  dhcp_cleanup(netif: &mut netif);
pub fn  dhcp_start(netif: &mut netif);
pub fn  dhcp_renew(netif: &mut netif);
pub fn  dhcp_release(netif: &mut netif);
pub fn  dhcp_stop(netif: &mut netif);
pub fn  dhcp_release_and_stop(netif: &mut netif);
pub fn  dhcp_inform(netif: &mut netif);
pub fn  dhcp_network_changed(netif: &mut netif);

pub fn  dhcp_arp_reply(netif: &mut netif,  addr: &mut ip4_addr);

dhcp_supplied_address: u8(const netif: &mut netif);
/* to be called every minute */
pub fn  dhcp_coarse_tmr();
/* to be called every half second */
pub fn  dhcp_fine_tmr();


/* This function must exist, in other to add offered NTP servers to
 * the NTP (or SNTP) engine.
 * See LWIP_DHCP_MAX_NTP_SERVERS */
extern void dhcp_set_ntp_servers(num_ntp_servers: u8,  ip4_addr* ntp_server_addrs);


#define netif_dhcp_data(netif) ((struct dhcp*)netif_get_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_DHCP))


}





