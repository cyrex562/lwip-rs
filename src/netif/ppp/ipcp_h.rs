/*
 * ipcp.h - IP Control Protocol definitions.
 *
 * Copyright (c) 1984-2000 Carnegie Mellon University. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. The name "Carnegie Mellon University" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For permission or any legal
 *    details, please contact
 *      Office of Technology Transfer
 *      Carnegie Mellon University
 *      5000 Forbes Avenue
 *      Pittsburgh, PA  15213-3890
 *      (412) 268-4387, fax: (412) 268-7395
 *      tech-transfer@andrew.cmu.edu
 *
 * 4. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by Computing Services
 *     at Carnegie Mellon University (http://www.cmu.edu/computing/)."
 *
 * CARNEGIE MELLON UNIVERSITY DISCLAIMS ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL CARNEGIE MELLON UNIVERSITY BE LIABLE
 * FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * $Id: ipcp.h,v 1.14 2002/12/04 23:03:32 paulus Exp $
 */

// #define	IPCP_H

/*
 * Options.
 */
pub const CI_ADDRS: u32 = 1; /* IP Addresses */
pub const CI_COMPRESSTYPE: u32 = 2; /* Compression Type */

pub const CI_ADDR: u32 = 3;

pub const CI_MS_DNS1: u32 = 129; /* Primary DNS value */

pub const CI_MS_DNS1: u32 = 129;
pub const CI_MS_DNS2: u32 = 131;

pub const CI_MS_DNS1: u32 = 129;

pub const CI_MS_WINS1: u32 = 130;
pub const CI_MS_WINS2: u32 = 132;

pub const CI_MS_DNS1: u32 = 129;

pub const CI_MS_DNS1: u32 = 129;

pub const CI_MS_DNS1: u32 = 129;

pub const MAX_STATES: u32 = 16; /* from slcompress.h */

pub const IPCP_VJMODE_OLD: u32 = 1; /* "old" mode (option # = 0x0037) */
pub const IPCP_VJMODE_RFC1172: u32 = 2; /* "old-rfc"mode (option # = 0x002d) */
pub const IPCP_VJMODE_RFC1332: u32 = 3; /* "new-rfc"mode (option # = 0x002d, */
/*  maxslot and slot number compression) */

pub const IPCP_VJ_COMP: u32 = 0x002d; /* current value for VJ compression option*/
pub const IPCP_VJ_COMP: u32 = 0x002d;
pub const IPCP_VJ_COMP_OLD: u32 = 0x0037; /* "old" (i.e, broken) value for VJ */
/* compression option*/

pub struct ipcp_options {
    pub neg_addr: bool,  /* Negotiate IP Address? */
    pub old_addrs: bool, /* Use old (IP-Addresses) option? */
    pub req_addr: bool,  /* Ask peer to send IP address? */

    pub default_route: bool, /* Assign default route through interface? */
    pub replace_default_route: bool, /* Replace default route through interface? */
    pub proxy_arp: bool,     /* Make proxy ARP entry for peer? */
    pub neg_vj: bool,        /* Van Jacobson Compression? */
    pub old_vj: i32,         /* use old (short) form of VJ option? */
    pub cflag: i32,

    pub accept_local: i32,  /* accept peer's value for ouraddr */
    pub accept_remote: i32, /* accept peer's value for hisaddr */

    pub req_dns1: i32, /* Ask peer to send primary DNS address? */
    pub req_dns2: i32, /* Ask peer to send secondary DNS address? */

    pub ouraddr: u32,
    pub hisaddr: u32, /* Addresses in NETWORK BYTE ORDER */

    pub dnsaddr: [u32; 2], /* Primary and secondary MS DNS entries */

    pub winsaddr: [u32; 2], /* Primary and secondary MS WINS entries */

    pub vj_protocol: u16, /* protocol value to use in VJ option */
    pub maxslotindex: u8, /* values for RFC1332 VJ compression neg. */
}
