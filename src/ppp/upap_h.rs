/*
 * upap.h - User/Password Authentication Protocol definitions.
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
 * $Id: upap.h,v 1.8 2002/12/04 23:03:33 paulus Exp $
 */

// #include "netif/ppp/ppp_opts.h"
// #if PPP_SUPPORT && PAP_SUPPORT  /* don't build if not configured for use in lwipopts.h */




// #include "ppp.h"




/*
 * Packet header = Code, id, length.
 */
pub const UPAP_HEADERLEN: u32 = 4;


/*
 * UPAP codes.
 */
pub const UPAP_AUTHREQ: u32 = 1;	/* Authenticate-Request */
pub const UPAP_AUTHACK: u32 = 2;	/* Authenticate-Ack */
pub const UPAP_AUTHNAK: u32 = 3;	/* Authenticate-Nak */


/*
 * Client states.
 */
pub const UPAPCS_INITIAL: u32 = 0;	/* Connection down */
pub const UPAPCS_CLOSED: u32 = 1;	/* Connection up, haven't requested auth */
pub const UPAPCS_PENDING: u32 = 2;	/* Connection down, have requested auth */
pub const UPAPCS_AUTHREQ: u32 = 3;	/* We've sent an Authenticate-Request */
pub const UPAPCS_OPEN: u32 = 4;	/* We've received an Ack */
pub const UPAPCS_BADAUTH: u32 = 5;	/* We've received a Nak */

/*
 * Server states.
 */
pub const UPAPSS_INITIAL: u32 = 0;	/* Connection down */
pub const UPAPSS_CLOSED: u32 = 1;	/* Connection up, haven't requested auth */
pub const UPAPSS_PENDING: u32 = 2;	/* Connection down, have requested auth */
pub const UPAPSS_LISTEN: u32 = 3;	/* Listening for an Authenticate */
pub const UPAPSS_OPEN: u32 = 4;	/* We've sent an Ack */
pub const UPAPSS_BADAUTH: u32 = 5;	/* We've sent a Nak */


/*
 * Timeouts.
 */
// #if 0 /* moved to ppp_opts.h */
pub const UPAP_DEFTIMEOUT: u32 = 3;	/* Timeout (seconds) for retransmitting req */
pub const UPAP_DEFREQTIME: u32 = 30;	/* Time to wait for auth-req from peer */
 /* moved to ppp_opts.h */

/*
 * Each interface is described by upap structure.
 */
// #if PAP_SUPPORT
typedef struct upap_state {
    const char *us_user;	/* User */
    us_userlen: u8;		/* User length */
    const char *us_passwd;	/* Password */
    us_passwdlen: u8;		/* Password length */
    us_clientstate: u8;	/* Client state */
// #if PPP_SERVER
    us_serverstate: u8;	/* Server state */
 /* PPP_SERVER */
_t us_id;		        /* Current id */
    us_transmits: u8;		/* Number of auth-reqs sent */
} upap_state;
 /* PAP_SUPPORT */


void upap_authwithpeer(ppp_pcb *pcb, const char *user, const char *password);
// #if PPP_SERVER
void upap_authpeer(ppp_pcb *pcb);
 /* PPP_SERVER */

extern const struct protent pap_protent;




 /* UPAP_H */
 /* PPP_SUPPORT && PAP_SUPPORT */
