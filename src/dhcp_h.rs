/**
 * @file
 * DHCP protocol definitions
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



// #include "lwip/opt.h"
// #include "lwip/arch.h"
// #include "lwip/prot/ip4.h"




 /* DHCP message item offsets and length */
pub const DHCP_CHADDR_LEN: u32 = 16U;
pub const DHCP_SNAME_OFS: u32 = 44U;
pub const DHCP_SNAME_LEN: u32 = 64U;
pub const DHCP_FILE_OFS: u32 = 108U;
pub const DHCP_FILE_LEN: u32 = 128U;
pub const DHCP_MSG_LEN: u32 = 236U;
#define DHCP_OPTIONS_OFS  (DHCP_MSG_LEN + 4U) /* 4 byte: cookie */

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
/** minimum set of fields of any DHCP message */
struct dhcp_msg
{
  PACK_STRUCT_FLD_8(u8_t op);
  PACK_STRUCT_FLD_8(u8_t htype);
  PACK_STRUCT_FLD_8(u8_t hlen);
  PACK_STRUCT_FLD_8(u8_t hops);
  PACK_STRUCT_FIELD(u32_t xid);
  PACK_STRUCT_FIELD(u16_t secs);
  PACK_STRUCT_FIELD(u16_t flags);
  PACK_STRUCT_FLD_S(ip4_addr_p_t ciaddr);
  PACK_STRUCT_FLD_S(ip4_addr_p_t yiaddr);
  PACK_STRUCT_FLD_S(ip4_addr_p_t siaddr);
  PACK_STRUCT_FLD_S(ip4_addr_p_t giaddr);
  PACK_STRUCT_FLD_8(u8_t chaddr[DHCP_CHADDR_LEN]);
  PACK_STRUCT_FLD_8(u8_t sname[DHCP_SNAME_LEN]);
  PACK_STRUCT_FLD_8(u8_t file[DHCP_FILE_LEN]);
  PACK_STRUCT_FIELD(u32_t cookie);
pub const DHCP_MIN_OPTIONS_LEN: u32 = 68U;
/** make sure user does not configure this too small */
#if ((defined(DHCP_OPTIONS_LEN)) && (DHCP_OPTIONS_LEN < DHCP_MIN_OPTIONS_LEN))
#  undef DHCP_OPTIONS_LEN

low this to be configured in lwipopts.h, but not too small */
#if (!defined(DHCP_OPTIONS_LEN))
/** set this to be sufficient for your options in outgoing DHCP msgs */
#  define DHCP_OPTIONS_LEN DHCP_MIN_OPTIONS_LEN

_STRUCT_FLD_8(u8_t options[DHCP_OPTIONS_LEN]);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END
#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"



/* DHCP client states */
typedef enum {
  DHCP_STATE_OFF             = 0,
  DHCP_STATE_REQUESTING      = 1,
  DHCP_STATE_INIT            = 2,
  DHCP_STATE_REBOOTING       = 3,
  DHCP_STATE_REBINDING       = 4,
  DHCP_STATE_RENEWING        = 5,
  DHCP_STATE_SELECTING       = 6,
  DHCP_STATE_INFORMING       = 7,
  DHCP_STATE_CHECKING        = 8,
  DHCP_STATE_PERMANENT       = 9,  /* not yet implemented */
  DHCP_STATE_BOUND           = 10,
  DHCP_STATE_RELEASING       = 11, /* not yet implemented */
  DHCP_STATE_BACKING_OFF     = 12
} dhcp_state_enum_t;

/* DHCP op codes */
pub const DHCP_BOOTREQUEST: u32 = 1;
pub const DHCP_BOOTREPLY: u32 = 2;

/* DHCP message types */
pub const DHCP_DISCOVER: u32 = 1;
pub const DHCP_OFFER: u32 = 2;
pub const DHCP_REQUEST: u32 = 3;
pub const DHCP_DECLINE: u32 = 4;
pub const DHCP_ACK: u32 = 5;
pub const DHCP_NAK: u32 = 6;
pub const DHCP_RELEASE: u32 = 7;
pub const DHCP_INFORM: u32 = 8;

pub const DHCP_MAGIC_COOKIE: u32 = 0x63825363UL;

/* This is a list of options for BOOTP and DHCP, see RFC 2132 for descriptions */

/* BootP options */
pub const DHCP_OPTION_PAD: u32 = 0;
pub const DHCP_OPTION_SUBNET_MASK: u32 = 1; /* RFC 2132 3.3 */
pub const DHCP_OPTION_ROUTER: u32 = 3;
pub const DHCP_OPTION_DNS_SERVER: u32 = 6;
pub const DHCP_OPTION_HOSTNAME: u32 = 12;
pub const DHCP_OPTION_IP_TTL: u32 = 23;
pub const DHCP_OPTION_MTU: u32 = 26;
pub const DHCP_OPTION_BROADCAST: u32 = 28;
pub const DHCP_OPTION_TCP_TTL: u32 = 37;
pub const DHCP_OPTION_NTP: u32 = 42;
pub const DHCP_OPTION_END: u32 = 255;

/* DHCP options */
pub const DHCP_OPTION_REQUESTED_IP: u32 = 50; /* RFC 2132 9.1, requested IP address */
pub const DHCP_OPTION_LEASE_TIME: u32 = 51; /* RFC 2132 9.2, time in seconds, in 4 bytes */
pub const DHCP_OPTION_OVERLOAD: u32 = 52; /* RFC2132 9.3, use file and/or sname field for options */

pub const DHCP_OPTION_MESSAGE_TYPE: u32 = 53; /* RFC 2132 9.6, important for DHCP */
pub const DHCP_OPTION_MESSAGE_TYPE_LEN: u32 = 1;

pub const DHCP_OPTION_SERVER_ID: u32 = 54; /* RFC 2132 9.7, server IP address */
pub const DHCP_OPTION_PARAMETER_REQUEST_LIST: u32 = 55; /* RFC 2132 9.8, requested option types */

pub const DHCP_OPTION_MAX_MSG_SIZE: u32 = 57; /* RFC 2132 9.10, message size accepted >= 576 */
pub const DHCP_OPTION_MAX_MSG_SIZE_LEN: u32 = 2;

pub const DHCP_OPTION_T1: u32 = 58; /* T1 renewal time */
pub const DHCP_OPTION_T2: u32 = 59; /* T2 rebinding time */
pub const DHCP_OPTION_US: u32 = 60;
pub const DHCP_OPTION_CLIENT_ID: u32 = 61;
pub const DHCP_OPTION_TFTP_SERVERNAME: u32 = 66;
pub const DHCP_OPTION_BOOTFILE: u32 = 67;

/* possible combinations of overloading the file and sname fields with options */
pub const DHCP_OVERLOAD_NONE: u32 = 0;
pub const DHCP_OVERLOAD_FILE: u32 = 1;
pub const DHCP_OVERLOAD_SNAME: u32 = 2;
pub const DHCP_OVERLOAD_SNAME_FILE: u32 = 3;





 /* LWIP_HDR_PROT_DHCP_H */
