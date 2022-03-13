/*
 * lcp.h - Link Control Protocol definitions.
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
 * $Id: lcp.h,v 1.20 2004/11/14 22:53:42 carlsonj Exp $
 */

// #include "netif/ppp/ppp_opts.h"
// #if PPP_SUPPORT /* don't build if not configured for use in lwipopts.h */




// #include "ppp.h"




/*
 * Options.
 */
pub const CI_VENDOR: u32 = 0; /* Vendor Specific */
pub const CI_MRU: u32 = 1; /* Maximum Receive Unit */
pub const CI_ASYNCMAP: u32 = 2; /* Async Control Character Map */
pub const CI_AUTHTYPE: u32 = 3; /* Authentication Type */
pub const CI_QUALITY: u32 = 4; /* Quality Protocol */
pub const CI_MAGICNUMBER: u32 = 5; /* Magic Number */
pub const CI_PCOMPRESSION: u32 = 7; /* Protocol Field Compression */
pub const CI_ACCOMPRESSION: u32 = 8; /* Address/Control Field Compression */
pub const CI_FCSALTERN: u32 = 9; /* FCS-Alternatives */
pub const CI_SDP: u32 = 10; /* Self-Describing-Pad */
pub const CI_NUMBERED: u32 = 11; /* Numbered-Mode */
pub const CI_CALLBACK: u32 = 13; /* callback */
pub const CI_MRRU: u32 = 17; /* max reconstructed receive unit; multilink */
pub const CI_SSNHF: u32 = 18; /* short sequence numbers for multilink */
pub const CI_EPDISC: u32 = 19; /* endpoint discriminator */
pub const CI_MPPLUS: u32 = 22; /* Multi-Link-Plus-Procedure */
pub const CI_LDISC: u32 = 23; /* Link-Discriminator */
pub const CI_LCPAUTH: u32 = 24; /* LCP Authentication */
pub const CI_COBS: u32 = 25; /* Consistent Overhead Byte Stuffing */
pub const CI_PREFELIS: u32 = 26; /* Prefix Elision */
pub const CI_MPHDRFMT: u32 = 27; /* MP Header Format */
pub const CI_I18N: u32 = 28; /* Internationalization */
pub const CI_SDL: u32 = 29; /* Simple Data Link */

/*
 * LCP-specific packet types (code numbers).
 */
pub const PROTREJ: u32 = 8; /* Protocol Reject */
pub const ECHOREQ: u32 = 9; /* Echo Request */
pub const ECHOREP: u32 = 10; /* Echo Reply */
pub const DISCREQ: u32 = 11; /* Discard Request */
pub const IDENTIF: u32 = 12; /* Identification */
pub const TIMEREM: u32 = 13; /* Time Remaining */

/* Value used as data for CI_CALLBACK option */
pub const CBCP_OPT: u32 = 6; /* Use callback control protocol */

// #if 0 /* moved to ppp_opts.h */
pub const DEFMRU: u32 = 1500; /* Try for this */
pub const MINMRU: u32 = 128; /* No MRUs below this */
pub const MAXMRU: u32 = 16384; /* Normally limit MRU to this */
 /* moved to ppp_opts.h */

/* An endpoint discriminator, used with multilink. */
pub const MAX_ENDP_LEN: u32 = 20; /* maximum length of discriminator value */
struct epdisc {
    unsigned char	class_; /* -- The word "class" is reserved in C++. */
    unsigned char	length;
    unsigned char	value[MAX_ENDP_LEN];
};

/*
 * The state of options is described by an lcp_options structure.
 */
typedef struct lcp_options {
    unsigned int passive           :1; /* Don't die if we don't get a response */
    unsigned int silent            :1; /* Wait for the other end to start first */
// #if 0 /* UNUSED */
    unsigned int restart           :1; /* Restart vs. exit after close */
 /* UNUSED */
signed int neg_mru           :1; /* Negotiate the MRU? */
    unsigned int neg_asyncmap      :1; /* Negotiate the async map? */
// #if PAP_SUPPORT
    unsigned int neg_upap          :1; /* Ask for UPAP authentication? */
 /* PAP_SUPPORT */
AP_SUPPORT
    unsigned int neg_chap          :1; /* Ask for CHAP authentication? */
 /* CHAP_SUPPORT */
P_SUPPORT
    unsigned int neg_eap           :1; /* Ask for EAP authentication? */
 /* EAP_SUPPORT */
signed int neg_magicnumber   :1; /* Ask for magic number? */
    unsigned int neg_pcompression  :1; /* HDLC Protocol Field Compression? */
    unsigned int neg_accompression :1; /* HDLC Address/Control Field Compression? */
// #if LQR_SUPPORT
    unsigned int neg_lqr           :1; /* Negotiate use of Link Quality Reports */
 /* LQR_SUPPORT */
signed int neg_cbcp          :1; /* Negotiate use of CBCP */
#ifdef HAVE_MULTILINK
    unsigned int neg_mrru          :1; /* negotiate multilink MRRU */
 /* HAVE_MULTILINK */
signed int neg_ssnhf         :1; /* negotiate short sequence numbers */
    unsigned int neg_endpoint      :1; /* negotiate endpoint discriminator */

    mru: u16;			/* Value of MRU */
#ifdef HAVE_MULTILINK
    mrru: u16;			/* Value of MRRU, and multilink enable */
 /* MULTILINK */
AP_SUPPORT
    chap_mdtype: u8;		/* which MD types (hashing algorithm) */
 /* CHAP_SUPPORT */
2_t asyncmap;		/* Value of async map */
    u32_t magicnumber;
    numloops: u8;		/* Number of loops during magic number neg. */
// #if LQR_SUPPORT
    u32_t lqr_period;	/* Reporting period for LQR 1/100ths second */
 /* LQR_SUPPORT */
ruct epdisc endpoint;	/* endpoint discriminator */
} lcp_options;

void lcp_open(ppp_pcb *pcb);
void lcp_close(ppp_pcb *pcb, const char *reason);
void lcp_lowerup(ppp_pcb *pcb);
void lcp_lowerdown(ppp_pcb *pcb);
void lcp_sprotrej(ppp_pcb *pcb, u_char *p, int len);    /* send protocol reject */

extern const struct protent lcp_protent;

// #if 0 /* moved to ppp_opts.h */
/* Default number of times we receive our magic number from the peer
   before deciding the link is looped-back. */
pub const DEFLOOPBACKFAIL: u32 = 10; /* moved to ppp_opts.h */




 /* LCP_H */
 /* PPP_SUPPORT */
