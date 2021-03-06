/**
 * @file
 * DNS - host name to IP address resolver.
 */

/*
 * Port to lwIP from uIP
 * by Jim Pettinato April 2007
 *
 * security fixes and more by Simon Goldschmidt
 *
 * uIP version Copyright (c) 2002-2003, Adam Dunkels.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote
 *    products derived from this software without specific prior
 *    written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS
 * OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 * GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */




// #include "lwip/arch.h"




/** DNS server port address */

pub const DNS_SERVER_PORT: u32 = 53;


/* DNS field TYPE used for "Resource Records" */
pub const DNS_RRTYPE_A: u32 = 1;     /* a host address */
pub const DNS_RRTYPE_NS: u32 = 2;     /* an authoritative name server */
pub const DNS_RRTYPE_MD: u32 = 3;     /* a mail destination (Obsolete - use MX) */
pub const DNS_RRTYPE_MF: u32 = 4;     /* a mail forwarder (Obsolete - use MX) */
pub const DNS_RRTYPE_CNAME: u32 = 5;     /* the canonical name for an alias */
pub const DNS_RRTYPE_SOA: u32 = 6;     /* marks the start of a zone of authority */
pub const DNS_RRTYPE_MB: u32 = 7;     /* a mailbox domain name (EXPERIMENTAL) */
pub const DNS_RRTYPE_MG: u32 = 8;     /* a mail group member (EXPERIMENTAL) */
pub const DNS_RRTYPE_MR: u32 = 9;     /* a mail rename domain name (EXPERIMENTAL) */
pub const DNS_RRTYPE_NULL: u32 = 10;    /* a null RR (EXPERIMENTAL) */
pub const DNS_RRTYPE_WKS: u32 = 11;    /* a well known service description */
pub const DNS_RRTYPE_PTR: u32 = 12;    /* a domain name pointer */
pub const DNS_RRTYPE_HINFO: u32 = 13;    /* host information */
pub const DNS_RRTYPE_MINFO: u32 = 14;    /* mailbox or mail list information */
pub const DNS_RRTYPE_MX: u32 = 15;    /* mail exchange */
pub const DNS_RRTYPE_TXT: u32 = 16;    /* text strings */
pub const DNS_RRTYPE_AAAA: u32 = 28;    /* IPv6 address */
pub const DNS_RRTYPE_SRV: u32 = 33;    /* service location */
pub const DNS_RRTYPE_ANY: u32 = 255;   /* any type */

/* DNS field CLASS used for "Resource Records" */
pub const DNS_RRCLASS_IN: u32 = 1;     /* the Internet */
pub const DNS_RRCLASS_CS: u32 = 2;     /* the CSNET class (Obsolete - used only for examples in some obsolete RFCs) */
pub const DNS_RRCLASS_CH: u32 = 3;     /* the CHAOS class */
pub const DNS_RRCLASS_HS: u32 = 4;     /* Hesiod [Dyer 87] */
pub const DNS_RRCLASS_ANY: u32 = 255;   /* any class */
pub const DNS_RRCLASS_FLUSH: u32 = 0x800; /* Flush bit */

/* DNS protocol flags */
pub const DNS_FLAG1_RESPONSE: u32 = 0x80;
pub const DNS_FLAG1_OPCODE_STATUS: u32 = 0x10;
pub const DNS_FLAG1_OPCODE_INVERSE: u32 = 0x08;
pub const DNS_FLAG1_OPCODE_STANDARD: u32 = 0x00;
pub const DNS_FLAG1_AUTHORATIVE: u32 = 0x04;
pub const DNS_FLAG1_TRUNC: u32 = 0x02;
pub const DNS_FLAG1_RD: u32 = 0x01;
pub const DNS_FLAG2_RA: u32 = 0x80;
pub const DNS_FLAG2_ERR_MASK: u32 = 0x0f;
pub const DNS_FLAG2_ERR_NONE: u32 = 0x00;
pub const DNS_FLAG2_ERR_NAME: u32 = 0x03;

#define DNS_HDR_GET_OPCODE(hdr) ((((hdr)->flags1) >> 3) & 0xF)

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
/** DNS message header */
struct dns_hdr {
  PACK_STRUCT_FIELD(u16_t id);
  PACK_STRUCT_FLD_8(u8_t flags1);
  PACK_STRUCT_FLD_8(u8_t flags2);
  PACK_STRUCT_FIELD(u16_t numquestions);
  PACK_STRUCT_FIELD(u16_t numanswers);
  PACK_STRUCT_FIELD(u16_t numauthrr);
  PACK_STRUCT_FIELD(u16_t numextrarr);
} PACK_STRUCT_STRUCT;

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"

e SIZEOF_DNS_HDR 12


/* Multicast DNS definitions */

/** UDP port for multicast DNS queries */

pub const DNS_MQUERY_PORT: u32 = 5353;


/* IPv4 group for multicast DNS queries: 224.0.0.251 */

pub const DNS_MQUERY_IPV4_GROUP_INIT: u32 = IPADDR4_INIT_BYTES;(224,0,0,251)


/* IPv6 group for multicast DNS queries: FF02::FB */

pub const DNS_MQUERY_IPV6_GROUP_INIT: u32 = IPADDR6_INIT_HOST;(0xFF020000,0,0,0xFB)





 /* LWIP_HDR_PROT_DNS_H */
