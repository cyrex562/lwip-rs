/**
 * @file
 * DHCPv6 protocol definitions
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt <goldsimon@gmx.de>
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
 *
 */



// #include "lwip/opt.h"




pub const DHCP6_CLIENT_PORT: u32 = 546; #define DHCP6_SERVER_PORT  547


 /* DHCPv6 message item offsets and length */
pub const DHCP6_TRANSACTION_ID_LEN: u32 = 3; #ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
/** minimum set of fields of any DHCPv6 message */
struct dhcp6_msg
{
  PACK_STRUCT_FLD_8(u8_t msgtype);
  PACK_STRUCT_FLD_8(u8_t transaction_id[DHCP6_TRANSACTION_ID_LEN]);
  /* options follow */
} PACK_STRUCT_STRUCT;

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"



/* DHCP6 client states */
typedef enum {
  DHCP6_STATE_OFF               = 0,
  DHCP6_STATE_STATELESS_IDLE    = 1,
  DHCP6_STATE_REQUESTING_CONFIG = 2
} dhcp6_state_enum_t;

/* DHCPv6 message types */
pub const DHCP6_SOLICIT: u32 = 1; #define DHCP6_ADVERTISE             2
pub const DHCP6_REQUEST: u32 = 3; #define DHCP6_CONFIRM               4
pub const DHCP6_RENEW: u32 = 5; #define DHCP6_REBIND                6
pub const DHCP6_REPLY: u32 = 7; #define DHCP6_RELEASE               8
pub const DHCP6_DECLINE: u32 = 9; #define DHCP6_RECONFIGURE           10
pub const DHCP6_INFOREQUEST: u32 = 11; #define DHCP6_RELAYFORW             12
pub const DHCP6_RELAYREPL: u32 = 13; /* More message types see https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters.xhtml */

/** DHCPv6 status codes */
pub const DHCP6_STATUS_SUCCESS: u32 = 0; /* Success. */
pub const DHCP6_STATUS_UNSPECFAIL: u32 = 1; /* Failure, reason unspecified; this status code is sent by either a client or a server to indicate a failure not explicitly specified in this document. */
pub const DHCP6_STATUS_NOADDRSAVAIL: u32 = 2; /* Server has no addresses available to assign to the IA(s). */
pub const DHCP6_STATUS_NOBINDING: u32 = 3; /* Client record (binding) unavailable. */
pub const DHCP6_STATUS_NOTONLINK: u32 = 4; /* The prefix for the address is not appropriate for the link to which the client is attached. */
pub const DHCP6_STATUS_USEMULTICAST: u32 = 5; /* Sent by a server to a client to force the client to send messages to the server using the All_DHCP_Relay_Agents_and_Servers address. */
/* More status codes see https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters.xhtml */

/** DHCPv6 DUID types */
pub const DHCP6_DUID_LLT: u32 = 1; /* LLT: Link-layer Address Plus Time */
pub const DHCP6_DUID_EN: u32 = 2; /* EN: Enterprise number */
pub const DHCP6_DUID_LL: u32 = 3; /* LL: Link-layer Address */
pub const DHCP6_DUID_UUID: u32 = 4; /* UUID (RFC 6355) */

/* DHCPv6 options */
pub const DHCP6_OPTION_CLIENTID: u32 = 1; #define DHCP6_OPTION_SERVERID       2
pub const DHCP6_OPTION_IA_NA: u32 = 3; #define DHCP6_OPTION_IA_TA          4
pub const DHCP6_OPTION_IAADDR: u32 = 5; #define DHCP6_OPTION_ORO            6
pub const DHCP6_OPTION_PREFERENCE: u32 = 7; #define DHCP6_OPTION_ELAPSED_TIME   8
pub const DHCP6_OPTION_RELAY_MSG: u32 = 9; #define DHCP6_OPTION_AUTH           11
pub const DHCP6_OPTION_UNICAST: u32 = 12; #define DHCP6_OPTION_STATUS_CODE    13
pub const DHCP6_OPTION_RAPID_COMMIT: u32 = 14; #define DHCP6_OPTION_USER_CLASS     15
pub const DHCP6_OPTION_VENDOR_CLASS: u32 = 16; #define DHCP6_OPTION_VENDOR_OPTS    17
pub const DHCP6_OPTION_INTERFACE_ID: u32 = 18; #define DHCP6_OPTION_RECONF_MSG     19
pub const DHCP6_OPTION_RECONF_ACCEPT: u32 = 20; /* More options see https://www.iana.org/assignments/dhcpv6-parameters/dhcpv6-parameters.xhtml */
pub const DHCP6_OPTION_DNS_SERVERS: u32 = 23; /* RFC 3646 */
pub const DHCP6_OPTION_DOMAIN_LIST: u32 = 24; /* RFC 3646 */
pub const DHCP6_OPTION_SNTP_SERVERS: u32 = 31; /* RFC 4075 */





 /* LWIP_HDR_PROT_DHCP6_H */
