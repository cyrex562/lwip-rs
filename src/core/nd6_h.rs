/**
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

#define LWIP_HDR_PROT_ND6_H






extern "C" {


/** Neighbor solicitation message header. */

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct ns_header {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(code: u8);
  PACK_STRUCT_FIELD(chksum: u16);
  PACK_STRUCT_FIELD(u32 reserved);
  PACK_STRUCT_FLD_S(ip6_addr_p_t target_address);
  /* Options follow. */
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Neighbor advertisement message header. */

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct na_header {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(code: u8);
  PACK_STRUCT_FIELD(chksum: u16);
  PACK_STRUCT_FLD_8(flags: u8);
  PACK_STRUCT_FLD_8(reserved: u8[3]);
  PACK_STRUCT_FLD_S(ip6_addr_p_t target_address);
  /* Options follow. */
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"

#define ND6_FLAG_ROUTER      (0x80)
#define ND6_FLAG_SOLICITED   (0x40)
#define ND6_FLAG_OVERRIDE    (0x20)

/** Router solicitation message header. */

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct rs_header {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(code: u8);
  PACK_STRUCT_FIELD(chksum: u16);
  PACK_STRUCT_FIELD(u32 reserved);
  /* Options follow. */
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Router advertisement message header. */
#define ND6_RA_FLAG_MANAGED_ADDR_CONFIG (0x80)
#define ND6_RA_FLAG_OTHER_CONFIG (0x40)
#define ND6_RA_FLAG_HOME_AGENT (0x20)
#define ND6_RA_PREFERENCE_MASK (0x18)
#define ND6_RA_PREFERENCE_HIGH (0x08)
#define ND6_RA_PREFERENCE_MEDIUM (0x00)
#define ND6_RA_PREFERENCE_LOW (0x18)
#define ND6_RA_PREFERENCE_DISABLED (0x10)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct ra_header {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(code: u8);
  PACK_STRUCT_FIELD(chksum: u16);
  PACK_STRUCT_FLD_8(current_hop_limit: u8);
  PACK_STRUCT_FLD_8(flags: u8);
  PACK_STRUCT_FIELD(router_lifetime: u16);
  PACK_STRUCT_FIELD(u32 reachable_time);
  PACK_STRUCT_FIELD(u32 retrans_timer);
  /* Options follow. */
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Redirect message header. */

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct redirect_header {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(code: u8);
  PACK_STRUCT_FIELD(chksum: u16);
  PACK_STRUCT_FIELD(u32 reserved);
  PACK_STRUCT_FLD_S(ip6_addr_p_t target_address);
  PACK_STRUCT_FLD_S(ip6_addr_p_t destination_address);
  /* Options follow. */
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Link-layer address option. */
#define ND6_OPTION_TYPE_SOURCE_LLADDR (0x01)
#define ND6_OPTION_TYPE_TARGET_LLADDR (0x02)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct lladdr_option {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(length: u8);
  PACK_STRUCT_FLD_8(addr: u8[NETIF_MAX_HWADDR_LEN]);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Prefix information option. */
#define ND6_OPTION_TYPE_PREFIX_INFO (0x03)
#define ND6_PREFIX_FLAG_ON_LINK (0x80)
#define ND6_PREFIX_FLAG_AUTONOMOUS (0x40)
#define ND6_PREFIX_FLAG_ROUTER_ADDRESS (0x20)
#define ND6_PREFIX_FLAG_SITE_PREFIX (0x10)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct prefix_option {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(length: u8);
  PACK_STRUCT_FLD_8(prefix_length: u8);
  PACK_STRUCT_FLD_8(flags: u8);
  PACK_STRUCT_FIELD(u32 valid_lifetime);
  PACK_STRUCT_FIELD(u32 preferred_lifetime);
  PACK_STRUCT_FLD_8(reserved2: u8[3]);
  PACK_STRUCT_FLD_8(site_prefix_length: u8);
  PACK_STRUCT_FLD_S(ip6_addr_p_t prefix);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Redirected header option. */
#define ND6_OPTION_TYPE_REDIR_HDR (0x04)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct redirected_header_option {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(length: u8);
  PACK_STRUCT_FLD_8(reserved: u8[6]);
  /* Portion of redirected packet follows. */
  /* PACK_STRUCT_FLD_8(redirected: u8[8]); */
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** MTU option. */
#define ND6_OPTION_TYPE_MTU (0x05)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct mtu_option {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(length: u8);
  PACK_STRUCT_FIELD(reserved: u16);
  PACK_STRUCT_FIELD(u32 mtu);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Route information option. */
#define ND6_OPTION_TYPE_ROUTE_INFO (24)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct route_option {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(length: u8);
  PACK_STRUCT_FLD_8(prefix_length: u8);
  PACK_STRUCT_FLD_8(preference: u8);
  PACK_STRUCT_FIELD(u32 route_lifetime);
  PACK_STRUCT_FLD_S(ip6_addr_p_t prefix);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/** Recursive DNS Server Option. */
#define ND6_OPTION_TYPE_RDNSS (25)

#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
struct rdnss_option {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(length: u8);
  PACK_STRUCT_FIELD(reserved: u16);
  PACK_STRUCT_FIELD(u32 lifetime);
  PACK_STRUCT_FLD_S(ip6_addr_p_t rdnss_address[1]);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


#define SIZEOF_RDNSS_OPTION_BASE 8 /* size without addresses */


}



