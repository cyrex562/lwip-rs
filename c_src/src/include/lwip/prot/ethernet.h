/**
 * @file
 * Ethernet protocol definitions
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



// #include "lwip/arch.h"
// #include "lwip/prot/ieee.h"





#ifdef ETHARP_HWADDR_LEN
#define ETH_HWADDR_LEN    ETHARP_HWADDR_LEN /* compatibility mode */
#else
pub const ETH_HWADDR_LEN: u32 = 6; #ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
/** An Ethernet MAC address */
struct eth_addr {
  PACK_STRUCT_FLD_8(u8_t addr[ETH_HWADDR_LEN]);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END
#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"


/** Initialize a struct eth_addr with its 6 bytes (takes care of correct braces) */
#define ETH_ADDR(b0, b1, b2, b3, b4, b5) {{b0, b1, b2, b3, b4, b5}}

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
/** Ethernet header */
struct eth_hdr {
// #if ETH_PAD_SIZE
  PACK_STRUCT_FLD_8(u8_t padding[ETH_PAD_SIZE]);

_STRUCT_FLD_S(struct eth_addr dest);
  PACK_STRUCT_FLD_S(struct eth_addr src);
  PACK_STRUCT_FIELD(u16_t type);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END
#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"


#define SIZEOF_ETH_HDR (14 + ETH_PAD_SIZE)

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
/** VLAN header inserted between ethernet header and payload
 * if 'type' in ethernet header is ETHTYPE_VLAN.
 * See IEEE802.Q */
struct eth_vlan_hdr {
  PACK_STRUCT_FIELD(u16_t prio_vid);
  PACK_STRUCT_FIELD(u16_t tpid);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END
#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"


pub const SIZEOF_VLAN_HDR: u32 = 4; #define VLAN_ID(vlan_hdr) (lwip_htons((vlan_hdr)->prio_vid) & 0xFFF)

/** The 24-bit IANA IPv4-multicast OUI is 01-00-5e: */
pub const LL_IP4_MULTICAST_ADDR_0: u32 = 0x01; #define LL_IP4_MULTICAST_ADDR_1 0x00
pub const LL_IP4_MULTICAST_ADDR_2: u32 = 0x5e; /** IPv6 multicast uses this prefix */
pub const LL_IP6_MULTICAST_ADDR_0: u32 = 0x33; #define LL_IP6_MULTICAST_ADDR_1 0x33

/* eth_addr_cmp is deprecated, use eth_addr_eq */
#define eth_addr_cmp(addr1, addr2) eth_addr_eq((addr1), (addr2))
#define eth_addr_eq(addr1, addr2) (memcmp((addr1)->addr, (addr2)->addr, ETH_HWADDR_LEN) == 0)




 /* LWIP_HDR_PROT_ETHERNET_H */
