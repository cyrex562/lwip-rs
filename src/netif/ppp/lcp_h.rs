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





#define	LCP_H







/*
 * Options.
 */
#define CI_VENDOR	0	/* Vendor Specific */
#define CI_MRU		1	/* Maximum Receive Unit */
#define CI_ASYNCMAP	2	/* Async Control Character Map */
#define CI_AUTHTYPE	3	/* Authentication Type */
#define CI_QUALITY	4	/* Quality Protocol */
#define CI_MAGICNUMBER	5	/* Magic Number */
#define CI_PCOMPRESSION	7	/* Protocol Field Compression */
#define CI_ACCOMPRESSION 8	/* Address/Control Field Compression */
#define CI_FCSALTERN	9	/* FCS-Alternatives */
#define CI_SDP		10	/* Self-Describing-Pad */
#define CI_NUMBERED	11	/* Numbered-Mode */
#define CI_CALLBACK	13	/* callback */
#define CI_MRRU		17	/* max reconstructed receive unit; multilink */
#define CI_SSNHF	18	/* short sequence numbers for multilink */
#define CI_EPDISC	19	/* endpodiscriminator: i32 */
#define CI_MPPLUS	22	/* Multi-Link-Plus-Procedure */
#define CI_LDISC	23	/* Link-Discriminator */
#define CI_LCPAUTH	24	/* LCP Authentication */
#define CI_COBS		25	/* Consistent Overhead Byte Stuffing */
#define CI_PREFELIS	26	/* Prefix Elision */
#define CI_MPHDRFMT	27	/* MP Header Format */
#define CI_I18N		28	/* Internationalization */
#define CI_SDL		29	/* Simple Data Link */

/*
 * LCP-specific packet types (code numbers).
 */
#define PROTREJ		8	/* Protocol Reject */
#define ECHOREQ		9	/* Echo Request */
#define ECHOREP		10	/* Echo Reply */
#define DISCREQ		11	/* Discard Request */
#define IDENTIF		12	/* Identification */
#define TIMEREM		13	/* Time Remaining */

/* Value used as data for CI_CALLBACK option */
#define CBCP_OPT	6	/* Use callback control protocol */


#define DEFMRU	1500		/* Try for this */
#define MINMRU	128		/* No MRUs below this */
#define MAXMRU	16384		/* Normally limit MRU to this */


/* An endpodiscriminator: i32, used with multilink. */
#define MAX_ENDP_LEN	20	/* maximum length of discriminator value */
struct epdisc {
    unsigned char	class_; /* -- The word "class" is reserved in C++. */
    unsigned char	length;
    unsigned char	value[MAX_ENDP_LEN];
};

/*
 * The state of options is described by an lcp_options structure.
 */
typedef struct lcp_options {
    unsigned passive: i32           :1; /* Don't die if we don't get a response */
    unsigned silent: i32            :1; /* Wait for the other end to start first */

    unsigned restart: i32           :1; /* Restart vs. exit after close */

    unsigned neg_mru: i32           :1; /* Negotiate the MRU? */
    unsigned neg_asyncmap: i32      :1; /* Negotiate the async map? */

    unsigned neg_upap: i32          :1; /* Ask for UPAP authentication? */


    unsigned neg_chap: i32          :1; /* Ask for CHAP authentication? */


    unsigned neg_eap: i32           :1; /* Ask for EAP authentication? */

    unsigned neg_magicnumber: i32   :1; /* Ask for magic number? */
    unsigned neg_pcompression: i32  :1; /* HDLC Protocol Field Compression? */
    unsigned neg_accompression: i32 :1; /* HDLC Address/Control Field Compression? */

    unsigned neg_lqr: i32           :1; /* Negotiate use of Link Quality Reports */

    unsigned neg_cbcp: i32          :1; /* Negotiate use of CBCP */

    unsigned neg_mrru: i32          :1; /* negotiate multilink MRRU */

    unsigned neg_ssnhf: i32         :1; /* negotiate short sequence numbers */
    unsigned neg_endpoint: i32      :1; /* negotiate endpodiscriminator: i32 */

    mru: u16;			/* Value of MRU */

    mrru: u16;			/* Value of MRRU, and multilink enable */


    chap_mdtype: u8;		/* which MD types (hashing algorithm) */

    asyncmap: u32;		/* Value of async map */
    magicnumber: u32;
    u8  numloops;		/* Number of loops during magic number neg. */

    lqr_period: u32;	/* Reporting period for LQR 1/100ths second */

    struct epdisc endpoint;	/* endpodiscriminator: i32 */
} lcp_options;

pub fn  lcp_open(ppp_pcb *pcb);
pub fn  lcp_close(ppp_pcb *pcb, reason: &String);
pub fn  lcp_lowerup(ppp_pcb *pcb);
pub fn  lcp_lowerdown(ppp_pcb *pcb);
pub fn  lcp_sprotrej(ppp_pcb *pcb, u_p: &mut String, len: i32);    /* send protocol reject */

extern const struct protent lcp_protent;


/* Default number of times we receive our magic number from the peer
   before deciding the link is looped-back. */
#define DEFLOOPBACKFAIL	10



}




