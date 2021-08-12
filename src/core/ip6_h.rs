/*
 * @file
 * IPv6 protocol definitions
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

// #define LWIP_HDR_PROT_IP6_H








/* This is the packed version of ip6_addr_t,
    used in network headers that are itself packed */

#  include "arch/bpstruct.h"


struct ip6_addr_packed {
  (addr: u32[4]);
} ;


#  include "arch/epstruct.h"

typedef struct ip6_addr_packed ip6_addr_p_t;

#define IP6_HLEN 40

pub const IP6_NEXTH_HOPBYHOP: u32 = 0;
#define IP6_NEXTH_TCP       6
#define IP6_NEXTH_UDP       17
#define IP6_NEXTH_ENCAPS    41
#define IP6_NEXTH_ROUTING   43
#define IP6_NEXTH_FRAGMENT  44
#define IP6_NEXTH_ICMP6     58
#define IP6_NEXTH_NONE      59
#define IP6_NEXTH_DESTOPTS  60
#define IP6_NEXTH_UDPLITE   136

/* The IPv6 header. */

#  include "arch/bpstruct.h"


struct ip6_hdr {
  /* version / traffic class / flow label */
  (_v_tc_fl: u32);
  /* payload length */
  (_plen: u16);
  /* next header */
  (_nexth: u8);
  /* hop limit */
  (_hoplim: u8);
  /* source and destination IP addresses */
  (ip6_addr_p_t src);
  (ip6_addr_p_t dest);
} ;


#  include "arch/epstruct.h"

#define IP6H_V(hdr)  ((lwip_ntohl((hdr)->_v_tc_fl) >> 28) & 0x0f)
#define IP6H_TC(hdr) ((lwip_ntohl((hdr)->_v_tc_fl) >> 20) & 0xff)
#define IP6H_FL(hdr) (lwip_ntohl((hdr)->_v_tc_fl) & 0x000fffff)
#define IP6H_PLEN(hdr) (lwip_ntohs((hdr)->_plen))
#define IP6H_NEXTH(hdr) ((hdr)->_nexth)
#define IP6H_NEXTH_P(hdr) ((hdr) + 6)
#define IP6H_HOPLIM(hdr) ((hdr)->_hoplim)
#define IP6H_VTCFL_SET(hdr, v, tc, fl) (hdr)->_v_tc_fl = (lwip_htonl((((u32)(v)) << 28) | (((u32)(tc)) << 20) | (fl)))
#define IP6H_PLEN_SET(hdr, plen) (hdr)->_plen = lwip_htons(plen)
#define IP6H_NEXTH_SET(hdr, nexth) (hdr)->_nexth = (nexth)
#define IP6H_HOPLIM_SET(hdr, hl) (hdr)->_hoplim = (hl)

/* ipv6 extended options header */
pub const IP6_PAD1_OPTION: u32 = 0;
#define IP6_PADN_OPTION             1
#define IP6_ROUTER_ALERT_OPTION     5
#define IP6_JUMBO_OPTION            194
#define IP6_HOME_ADDRESS_OPTION     201
#define IP6_ROUTER_ALERT_DLEN       2
pub const IP6_ROUTER_ALERT_VALUE_MLD: u32 = 0;


#  include "arch/bpstruct.h"


struct ip6_opt_hdr {
  /* router alert option type */
  (_opt_type: u8);
  /* router alert option data len */
  (_opt_dlen: u8);
} ;


#  include "arch/epstruct.h"

#define IP6_OPT_HLEN 2
#define IP6_OPT_TYPE_ACTION(hdr) ((((hdr)->_opt_type) >> 6) & 0x3)
#define IP6_OPT_TYPE_CHANGE(hdr) ((((hdr)->_opt_type) >> 5) & 0x1)
#define IP6_OPT_TYPE(hdr) ((hdr)->_opt_type)
#define IP6_OPT_DLEN(hdr) ((hdr)->_opt_dlen)

/* Hop-by-Hop header. */
#define IP6_HBH_HLEN    2


#  include "arch/bpstruct.h"


struct ip6_hbh_hdr {
  /* next header */
  (_nexth: u8);
  /* header length in 8-octet units */
  (_hlen: u8);
} ;


#  include "arch/epstruct.h"

#define IP6_HBH_NEXTH(hdr) ((hdr)->_nexth)

/* Destination header. */
#define IP6_DEST_HLEN   2


#  include "arch/bpstruct.h"


struct ip6_dest_hdr {
  /* next header */
  (_nexth: u8);
  /* header length in 8-octet units */
  (_hlen: u8);
} ;


#  include "arch/epstruct.h"

#define IP6_DEST_NEXTH(hdr) ((hdr)->_nexth)

/* Routing header */
#define IP6_ROUT_TYPE2  2
#define IP6_ROUT_RPL    3


#  include "arch/bpstruct.h"


struct ip6_rout_hdr {
  /* next header */
  (_nexth: u8);
  /* reserved */
  (_hlen: u8);
  /* fragment offset */
  (_routing_type: u8);
  /* fragmented packet identification */
  (_segments_left: u8);
} ;


#  include "arch/epstruct.h"

#define IP6_ROUT_NEXTH(hdr) ((hdr)->_nexth)
#define IP6_ROUT_TYPE(hdr) ((hdr)->_routing_type)
#define IP6_ROUT_SEG_LEFT(hdr) ((hdr)->_segments_left)

/* Fragment header. */
#define IP6_FRAG_HLEN    8
pub const IP6_FRAG_OFFSET_MASK: u32 = 0xfff8;pub const IP6_FRAG_OFFSET_MASK: u32 = 0xfff8;
#define IP6_FRAG_MORE_FLAG      0x0001


#  include "arch/bpstruct.h"


struct ip6_frag_hdr {
  /* next header */
  (_nexth: u8);
  /* reserved */
  (reserved: u8);
  /* fragment offset */
  (_fragment_offset: u16);
  /* fragmented packet identification */
  (_identification: u32);
} ;


#  include "arch/epstruct.h"

#define IP6_FRAG_NEXTH(hdr) ((hdr)->_nexth)
#define IP6_FRAG_MBIT(hdr) (lwip_ntohs((hdr)->_fragment_offset) & 0x1)
#define IP6_FRAG_ID(hdr) (lwip_ntohl((hdr)->_identification))


}



