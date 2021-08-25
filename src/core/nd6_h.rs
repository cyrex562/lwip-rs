/*
 * @file
 * ND6 protocol definitions
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
 *
 */

// #define LWIP_HDR_PROT_ND6_H









/* Neighbor solicitation message header. */




struct ns_header {
  type: u8,
  code: u8,
  chksum: u16,
  reserved: u32,
  (ip6_addr_p_t target_address);
  /* Options follow. */
} ;





/* Neighbor advertisement message header. */




struct na_header {
  type: u8,
  code: u8,
  chksum: u16,
  flags: u8,
  (reserved: [u8;3]);
  (ip6_addr_p_t target_address);
  /* Options follow. */
} ;




#define ND6_FLAG_ROUTER      (0x80)
#define ND6_FLAG_SOLICITED   (0x40)
#define ND6_FLAG_OVERRIDE    (0x20)

/* Router solicitation message header. */




struct rs_header {
  type: u8,
  code: u8,
  chksum: u16,
  reserved: u32,
  /* Options follow. */
} ;





/* Router advertisement message header. */
#define ND6_RA_FLAG_MANAGED_ADDR_CONFIG (0x80)
#define ND6_RA_FLAG_OTHER_CONFIG (0x40)
#define ND6_RA_FLAG_HOME_AGENT (0x20)
#define ND6_RA_PREFERENCE_MASK (0x18)
#define ND6_RA_PREFERENCE_HIGH (0x08)
#define ND6_RA_PREFERENCE_MEDIUM (0x00)
#define ND6_RA_PREFERENCE_LOW (0x18)
#define ND6_RA_PREFERENCE_DISABLED (0x10)




struct ra_header {
  type: u8,
  code: u8,
  chksum: u16,
  current_hop_limit: u8,
  flags: u8,
  router_lifetime: u16,
  reachable_time: u32,
  retrans_timer: u32,
  /* Options follow. */
} ;





/* Redirect message header. */




struct redirect_header {
  type: u8,
  code: u8,
  chksum: u16,
  reserved: u32,
  (ip6_addr_p_t target_address);
  (ip6_addr_p_t destination_address);
  /* Options follow. */
} ;





/* Link-layer address option. */
#define ND6_OPTION_TYPE_SOURCE_LLADDR (0x01)
#define ND6_OPTION_TYPE_TARGET_LLADDR (0x02)




struct lladdr_option {
  type: u8,
  length: u8,
  (addr: [u8;NETIF_MAX_HWADDR_LEN]);
} ;





/* Prefix information option. */
#define ND6_OPTION_TYPE_PREFIX_INFO (0x03)
#define ND6_PREFIX_FLAG_ON_LINK (0x80)
#define ND6_PREFIX_FLAG_AUTONOMOUS (0x40)
#define ND6_PREFIX_FLAG_ROUTER_ADDRESS (0x20)
#define ND6_PREFIX_FLAG_SITE_PREFIX (0x10)




struct prefix_option {
  type: u8,
  length: u8,
  prefix_length: u8,
  flags: u8,
  valid_lifetime: u32,
  preferred_lifetime: u32,
  (reserved2: [u8;3]);
  site_prefix_length: u8,
  (ip6_addr_p_t prefix);
} ;





/* Redirected header option. */
#define ND6_OPTION_TYPE_REDIR_HDR (0x04)




struct redirected_header_option {
  type: u8,
  length: u8,
  (reserved: [u8;6]);
  /* Portion of redirected packet follows. */
  /* (redirected: [u8;8]); */
} ;





/* MTU option. */
#define ND6_OPTION_TYPE_MTU (0x05)




struct mtu_option {
  type: u8,
  length: u8,
  reserved: u16,
  mtu: u32,
} ;





/* Route information option. */
#define ND6_OPTION_TYPE_ROUTE_INFO (24)




struct route_option {
  type: u8,
  length: u8,
  prefix_length: u8,
  preference: u8,
  route_lifetime: u32,
  (ip6_addr_p_t prefix);
} ;





/* Recursive DNS Server Option. */
#define ND6_OPTION_TYPE_RDNSS (25)




struct rdnss_option {
  type: u8,
  length: u8,
  reserved: u16,
  lifetime: u32,
  (ip6_addr_p_t rdnss_address[1]);
} ;





pub const SIZEOF_RDNSS_OPTION_BASE: u32 = 8;  /* size without addresses */


}



