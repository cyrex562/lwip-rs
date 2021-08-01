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





#define	IPCP_H





/*
 * Options.
 */
#define CI_ADDRS	1	/* IP Addresses */

#define CI_COMPRESSTYPE	2	/* Compression Type */

#define	CI_ADDR		3


#define CI_MS_DNS1      129	/* Primary DNS value */
#define CI_MS_DNS2      131     /* Secondary DNS value */


#define CI_MS_WINS1     130     /* Primary WINS value */
#define CI_MS_WINS2     132	/* Secondary WINS value */



#define MAX_STATES 16		/* from slcompress.h */

#define IPCP_VJMODE_OLD 1	/* "old" mode (option # = 0x0037) */
#define IPCP_VJMODE_RFC1172 2	/* "old-rfc"mode (option # = 0x002d) */
#define IPCP_VJMODE_RFC1332 3	/* "new-rfc"mode (option # = 0x002d, */
                                /*  maxslot and slot number compression) */

pub const IPCP_VJ_COMP: u32 = 0x002d;	/* current value for VJ compression option*/pub const IPCP_VJ_COMP: u32 = 0x002d;
#define IPCP_VJ_COMP_OLD 0x0037	/* "old" (i.e, broken) value for VJ */
				/* compression option*/ 


typedef struct ipcp_options {
    unsigned neg_addr: int               :1; /* Negotiate IP Address? */
    unsigned old_addrs: int              :1; /* Use old (IP-Addresses) option? */
    unsigned req_addr: int               :1; /* Ask peer to send IP address? */

    unsigned default_route: int          :1; /* Assign default route through interface? */
    unsigned replace_default_route: int  :1; /* Replace default route through interface? */


    unsigned proxy_arp: int              :1; /* Make proxy ARP entry for peer? */


    unsigned neg_vj: int                 :1; /* Van Jacobson Compression? */
    unsigned old_vj: int                 :1; /* use old (short) form of VJ option? */
    unsigned cflag: int                  :1;

    unsigned accept_local: int           :1; /* accept peer's value for ouraddr */
    unsigned accept_remote: int          :1; /* accept peer's value for hisaddr */

    unsigned req_dns1: int               :1; /* Ask peer to send primary DNS address? */
    unsigned req_dns2: int               :1; /* Ask peer to send secondary DNS address? */


    ouraddr: u32, hisaddr;	/* Addresses in NETWORK BYTE ORDER */

    dnsaddr: u32[2];	/* Primary and secondary MS DNS entries */


    winsaddr: u32[2];	/* Primary and secondary MS WINS entries */



    vj_protocol: u16;		/* protocol value to use in VJ option */
    u8  maxslotindex;		/* values for RFC1332 VJ compression neg. */

} ipcp_options;


char *ip_ntoa (u32);


extern const struct protent ipcp_protent;


}




