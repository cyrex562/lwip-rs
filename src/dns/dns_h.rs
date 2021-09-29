/*
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


// #define LWIP_HDR_PROT_DNS_H







//  DNS server port address 

pub const DNS_SERVER_PORT: u16 =           53;


//  DNS field TYPE used for "Resource Records" 
pub const DNS_RRTYPE_A: u16 =              1;     //  a host address 
pub const DNS_RRTYPE_NS: u16 =             2;     //  an authoritative name server 
pub const DNS_RRTYPE_MD: u16 =             3;     //  a mail destination (Obsolete - use MX) 
pub const DNS_RRTYPE_MF: u16 =             4;     //  a mail forwarder (Obsolete - use MX) 
pub const DNS_RRTYPE_CNAME: u16 =          5;     //  the canonical name for an alias 
pub const DNS_RRTYPE_SOA: u16 =            6;     //  marks the start of a zone of authority 
pub const DNS_RRTYPE_MB: u16 =             7;     //  a mailbox domain name (EXPERIMENTAL) 
pub const DNS_RRTYPE_MG: u16 =             8;     //  a mail group member (EXPERIMENTAL) 
pub const DNS_RRTYPE_MR: u16 =             9;     //  a mail rename domain name (EXPERIMENTAL) 
pub const DNS_RRTYPE_None: u16 =           10;    //  a null RR (EXPERIMENTAL) 
pub const DNS_RRTYPE_WKS: u16 =            11;    //  a well known service description 
pub const DNS_RRTYPE_PTR: u16 =            12;    //  a domain name pointer 
pub const DNS_RRTYPE_HINFO: u16 =          13;    //  host information 
pub const DNS_RRTYPE_MINFO: u16 =          14;    //  mailbox or mail list information 
pub const DNS_RRTYPE_MX: u16 =             15;    //  mail exchange 
pub const DNS_RRTYPE_TXT: u16 =            16;    //  text strings 
pub const DNS_RRTYPE_AAAA: u16 =           28;    //  IPv6 address 
pub const DNS_RRTYPE_SRV: u16 =            33;    //  service location 
pub const DNS_RRTYPE_ANY: u16 =            255;   //  any type 

//  DNS field CLASS used for "Resource Records" 
pub const DNS_RRCLASS_IN: u16 =            1;     //  the Internet 
pub const DNS_RRCLASS_CS: u16 =            2;     //  the CSNET class (Obsolete - used only for examples in some obsolete RFCs) 
pub const DNS_RRCLASS_CH: u16 =            3;     //  the CHAOS class 
pub const DNS_RRCLASS_HS: u16 =            4;     //  Hesiod [Dyer 87] 
pub const DNS_RRCLASS_ANY: u16 =           255;   //  any class 
pub const DNS_RRCLASS_FLUSH: u32 = 0x800; //  Flush bit 

//  DNS protocol flags 
pub const DNS_FLAG1_RESPONSE: u32 = 0x80;
pub const DNS_FLAG1_OPCODE_STATUS: u32 =   0x10;
pub const DNS_FLAG1_OPCODE_INVERSE: u32 =  0x08;
pub const DNS_FLAG1_OPCODE_STANDARD: u32 = 0x00;
pub const DNS_FLAG1_AUTHORATIVE: u32 =     0x04;
pub const DNS_FLAG1_TRUNC: u32 =           0x02;
pub const DNS_FLAG1_RD: u32 =              0x01;
pub const DNS_FLAG2_RA: u32 =              0x80;
pub const DNS_FLAG2_ERR_MASK: u32 =        0x0f;
pub const DNS_FLAG2_ERR_NONE: u32 =        0x00;
pub const DNS_FLAG2_ERR_NAME: u32 =        0x03;

pub fn DNS_HDR_GET_OPCODE(hdr: &dns_hdr) {((((hdr).flags1) >> 3) & 0xF)}


//  DNS message header 
pub struct dns_hdr {
  pub id: u16,
  pub flags1: u8,
  pub flags2: u8,
  pub numquestions: u16,
  pub numanswers: u16,
  pub numauthrr: u16,
  pub numextrarr: u16,
} 


pub const SIZEOF_DNS_HDR: usize = 12;


//  Multicast DNS definitions 

//  UDP port for multicast DNS queries 

pub const DNS_MQUERY_PORT: u16 =             5353;


//  IPv4 group for multicast DNS queries: 224.0.0.251 

pub const DNS_MQUERY_IPV4_GROUP_INIT: () = IPADDR4_INIT_BYTES(224,0,0,251);


//  IPv6 group for multicast DNS queries: FF02::FB 

pub const DNS_MQUERY_IPV6_GROUP_INIT: () =  IPADDR6_INIT_HOST(0xFF020000,0,0,0xFB);



