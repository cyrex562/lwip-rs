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

#define LWIP_HDR_PROT_DHCP_H






extern "C" {


 /* DHCP message item offsets and length */
#define DHCP_CHADDR_LEN   16U
#define DHCP_SNAME_OFS    44U
#define DHCP_SNAME_LEN    64U
#define DHCP_FILE_OFS     108U
#define DHCP_FILE_LEN     128U
#define DHCP_MSG_LEN      236U
#define DHCP_OPTIONS_OFS  (DHCP_MSG_LEN + 4U) /* 4 byte: cookie */


#  include "arch/bpstruct.h"

PACK_STRUCT_BEGIN
/** minimum set of fields of any DHCP message */
struct dhcp_msg
{
  PACK_STRUCT_FLD_8(op: u8);
  PACK_STRUCT_FLD_8(htype: u8);
  PACK_STRUCT_FLD_8(hlen: u8);
  PACK_STRUCT_FLD_8(hops: u8);
  PACK_STRUCT_FIELD(u32 xid);
  PACK_STRUCT_FIELD(secs: u16);
  PACK_STRUCT_FIELD(flags: u16);
  PACK_STRUCT_FLD_S(ip4_addr_p_t ciaddr);
  PACK_STRUCT_FLD_S(ip4_addr_p_t yiaddr);
  PACK_STRUCT_FLD_S(ip4_addr_p_t siaddr);
  PACK_STRUCT_FLD_S(ip4_addr_p_t giaddr);
  PACK_STRUCT_FLD_8(chaddr: u8[DHCP_CHADDR_LEN]);
  PACK_STRUCT_FLD_8(sname: u8[DHCP_SNAME_LEN]);
  PACK_STRUCT_FLD_8(file: u8[DHCP_FILE_LEN]);
  PACK_STRUCT_FIELD(u32 cookie);
#define DHCP_MIN_OPTIONS_LEN 68U
/** make sure user does not configure this too small */

#  undef DHCP_OPTIONS_LEN

/** allow this to be configured in lwipopts.h, but not too small */

/** set this to be sufficient for your options in outgoing DHCP msgs */
#  define DHCP_OPTIONS_LEN DHCP_MIN_OPTIONS_LEN

  PACK_STRUCT_FLD_8(options: u8[DHCP_OPTIONS_LEN]);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

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
#define DHCP_BOOTREQUEST            1
#define DHCP_BOOTREPLY              2

/* DHCP message types */
#define DHCP_DISCOVER               1
#define DHCP_OFFER                  2
#define DHCP_REQUEST                3
#define DHCP_DECLINE                4
#define DHCP_ACK                    5
#define DHCP_NAK                    6
#define DHCP_RELEASE                7
#define DHCP_INFORM                 8

pub const DHCP_MAGIC_COOKIE: u32 = 0x63825363;UL

/* This is a list of options for BOOTP and DHCP, see RFC 2132 for descriptions */

/* BootP options */
pub const DHCP_OPTION_PAD: u32 = 0;
#define DHCP_OPTION_SUBNET_MASK     1 /* RFC 2132 3.3 */
#define DHCP_OPTION_ROUTER          3
#define DHCP_OPTION_DNS_SERVER      6
#define DHCP_OPTION_HOSTNAME        12
#define DHCP_OPTION_IP_TTL          23
#define DHCP_OPTION_MTU             26
#define DHCP_OPTION_BROADCAST       28
#define DHCP_OPTION_TCP_TTL         37
#define DHCP_OPTION_NTP             42
#define DHCP_OPTION_END             255

/* DHCP options */
#define DHCP_OPTION_REQUESTED_IP    50 /* RFC 2132 9.1, requested IP address */
#define DHCP_OPTION_LEASE_TIME      51 /* RFC 2132 9.2, time in seconds, in 4 bytes */
#define DHCP_OPTION_OVERLOAD        52 /* RFC2132 9.3, use file and/or sname field for options */

#define DHCP_OPTION_MESSAGE_TYPE    53 /* RFC 2132 9.6, important for DHCP */
#define DHCP_OPTION_MESSAGE_TYPE_LEN 1

#define DHCP_OPTION_SERVER_ID       54 /* RFC 2132 9.7, server IP address */
#define DHCP_OPTION_PARAMETER_REQUEST_LIST  55 /* RFC 2132 9.8, requested option types */

#define DHCP_OPTION_MAX_MSG_SIZE    57 /* RFC 2132 9.10, message size accepted >= 576 */
#define DHCP_OPTION_MAX_MSG_SIZE_LEN 2

#define DHCP_OPTION_T1              58 /* T1 renewal time */
#define DHCP_OPTION_T2              59 /* T2 rebinding time */
#define DHCP_OPTION_US              60
#define DHCP_OPTION_CLIENT_ID       61
#define DHCP_OPTION_TFTP_SERVERNAME 66
#define DHCP_OPTION_BOOTFILE        67

/* possible combinations of overloading the file and sname fields with options */
pub const DHCP_OVERLOAD_NONE: u32 = 0;
#define DHCP_OVERLOAD_FILE          1
#define DHCP_OVERLOAD_SNAME         2
#define DHCP_OVERLOAD_SNAME_FILE    3



}



