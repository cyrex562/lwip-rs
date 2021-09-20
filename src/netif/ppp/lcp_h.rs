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

/*
 * Options.
 */
pub const CI_VENDOR: u32 = 0; /* Vendor Specific */
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
pub const CI_VENDOR: u32 = 0;
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
pub const CI_EPDISC: u32 = 19; /* endpodiscriminator: i32 */
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
pub const PROTREJ: u32 = 8;
pub const PROTREJ: u32 = 8;
pub const PROTREJ: u32 = 8;
pub const PROTREJ: u32 = 8;
pub const PROTREJ: u32 = 8;
pub const ECHOREQ: u32 = 9; /* Echo Request */
pub const ECHOREP: u32 = 10; /* Echo Reply */
pub const DISCREQ: u32 = 11; /* Discard Request */
pub const IDENTIF: u32 = 12; /* Identification */
pub const TIMEREM: u32 = 13; /* Time Remaining */

/* Value used as data for CI_CALLBACK option */
pub const CBCP_OPT: u32 = 6; /* Use callback control protocol */
pub const CBCP_OPT: u32 = 6;
pub const CBCP_OPT: u32 = 6;
pub const CBCP_OPT: u32 = 6;
pub const DEFMRU: u32 = 1500; /* Try for this */
pub const MINMRU: u32 = 128; /* No MRUs below this */
pub const MAXMRU: u32 = 16384; /* Normally limit MRU to this */

/* An endpodiscriminator: i32, used with multilink. */
pub const MAX_ENDP_LEN: u32 = 20; /* maximum length of discriminator value */
pub struct epdisc {
    pub class_: u8, /* -- The word "class" is reserved in C+= 1. */
    pub length: u8,
    pub value: [u8; MAX_ENDP_LEN],
}

/*
 * The state of options is described by an lcp_options structure.
 */
pub struct lcp_options {
    pub passive: bool, /* Don't die if we don't get a response */
    pub silent: bool,  /* Wait for the other end to start first */

    pub restart: bool, /* Restart vs. exit after close */

    pub neg_mru: bool,      /* Negotiate the MRU? */
    pub neg_asyncmap: bool, /* Negotiate the async map? */

    pub neg_upap: bool, /* Ask for UPAP authentication? */

    pub neg_chap: bool, /* Ask for CHAP authentication? */

    pub neg_eap: bool, /* Ask for EAP authentication? */

    pub neg_magicnumber: bool,   /* Ask for magic number? */
    pub neg_pcompression: bool,  /* HDLC Protocol Field Compression? */
    pub neg_accompression: bool, /* HDLC Address/Control Field Compression? */

    pub neg_lqr: bool, /* Negotiate use of Link Quality Reports */

    pub neg_cbcp: bool, /* Negotiate use of CBCP */

    pub neg_mrru: bool, /* negotiate multilink MRRU */

    pub neg_ssnhf: bool,    /* negotiate short sequence numbers */
    pub neg_endpoint: bool, /* negotiate endpodiscriminator: i32 */

    pub mru: u16, /* Value of MRU */
    pub mru: u16,
    pub mrru: u16, /* Value of MRRU, and multilink enable */

    pub chap_mdtype: u8, /* which MD types (hashing algorithm) */

    pub asyncmap: u32, /* Value of async map */
    pub asyncmap: u32,
    pub magicnumber: u32,
    pub numloops: u8, /* Number of loops during magic number neg. */

    pub lqr_period: u32, /* Reporting period for LQR 1/100ths second */

    pub endpoint: epdisc, /* endpodiscriminator: i32 */
}

// pub fn  lcp_open(pcb: &mut ppp_pcb);
// pub fn  lcp_close(pcb: &mut ppp_pcb, reason: &String);
// pub fn  lcp_lowerup(pcb: &mut ppp_pcb);
// pub fn  lcp_lowerdown(pcb: &mut ppp_pcb);
// pub fn  lcp_sprotrej(pcb: &mut ppp_pcb, u_p: &mut String, len: i32);    /* send protocol reject */
// extern const struct protent lcp_protent;

/* Default number of times we receive our magic number from the peer
before deciding the link is looped-back. */
pub const DEFLOOPBACKFAIL: u32 = 10;
