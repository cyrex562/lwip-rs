/*
 * fsm.h - {Link, IP} Control Protocol Finite State Machine definitions.
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
 * $Id: fsm.h,v 1.10 2004/11/13 02:28:15 paulus Exp $
 */

// #include "netif/ppp/ppp_opts.h"
#if PPP_SUPPORT /* don't build if not configured for use in lwipopts.h */




// #include "ppp.h"




/*
 * Packet header = Code, id, length.
 */
pub const HEADERLEN: u32 = 4; /*
 *  CP (LCP, IPCP, etc.) codes.
 */
pub const CONFREQ: u32 = 1; /* Configuration Request */
pub const CONFACK: u32 = 2; /* Configuration Ack */
pub const CONFNAK: u32 = 3; /* Configuration Nak */
pub const CONFREJ: u32 = 4; /* Configuration Reject */
pub const TERMREQ: u32 = 5; /* Termination Request */
pub const TERMACK: u32 = 6; /* Termination Ack */
pub const CODEREJ: u32 = 7; /* Code Reject */


/*
 * Each FSM is described by an fsm structure and fsm callbacks.
 */
typedef struct fsm {
    ppp_pcb *pcb;		/* PPP Interface */
    const struct fsm_callbacks *callbacks;	/* Callback routines */
    const char *term_reason;	/* Reason for closing protocol */
    u8_t seen_ack;		/* Have received valid Ack/Nak/Rej to Req */
				  /* -- This is our only flag, we might use u_int :1 if we have more flags */
    u16_t protocol;		/* Data Link Layer Protocol field value */
    u8_t state;			/* State */
    u8_t flags;			/* Contains option bits */
    u8_t id;			/* Current id */
    u8_t reqid;			/* Current request id */
    u8_t retransmits;		/* Number of retransmissions left */
    u8_t nakloops;		/* Number of nak loops since last ack */
    u8_t rnakloops;		/* Number of naks received */
    u8_t maxnakloops;		/* Maximum number of nak loops tolerated
				   (necessary because IPCP require a custom large max nak loops value) */
    u8_t term_reason_len;	/* Length of term_reason */
} fsm;


typedef struct fsm_callbacks {
    void (*resetci)		/* Reset our Configuration Information */
		(fsm *);
    int  (*cilen)		/* Length of our Configuration Information */
		(fsm *);
    void (*addci) 		/* Add our Configuration Information */
		(fsm *, u_char *, int *);
    int  (*ackci)		/* ACK our Configuration Information */
		(fsm *, u_char *, int);
    int  (*nakci)		/* NAK our Configuration Information */
		(fsm *, u_char *, int, int);
    int  (*rejci)		/* Reject our Configuration Information */
		(fsm *, u_char *, int);
    int  (*reqci)		/* Request peer's Configuration Information */
		(fsm *, u_char *, int *, int);
    void (*up)			/* Called when fsm reaches PPP_FSM_OPENED state */
		(fsm *);
    void (*down)		/* Called when fsm leaves PPP_FSM_OPENED state */
		(fsm *);
    void (*starting)		/* Called when we want the lower layer */
		(fsm *);
    void (*finished)		/* Called when we don't want the lower layer */
		(fsm *);
    void (*protreject)		/* Called when Protocol-Reject received */
		(int);
    void (*retransmit)		/* Retransmission is necessary */
		(fsm *);
    int  (*extcode)		/* Called when unknown code received */
		(fsm *, int, int, u_char *, int);
    const char *proto_name;	/* String name for protocol (for messages) */
} fsm_callbacks;


/*
 * Link states.
 */
pub const PPP_FSM_INITIAL: u32 = 0; /* Down, hasn't been opened */
pub const PPP_FSM_STARTING: u32 = 1; /* Down, been opened */
pub const PPP_FSM_CLOSED: u32 = 2; /* Up, hasn't been opened */
pub const PPP_FSM_STOPPED: u32 = 3; /* Open, waiting for down event */
pub const PPP_FSM_CLOSING: u32 = 4; /* Terminating the connection, not open */
pub const PPP_FSM_STOPPING: u32 = 5; /* Terminating, but open */
pub const PPP_FSM_REQSENT: u32 = 6; /* We've sent a Config Request */
pub const PPP_FSM_ACKRCVD: u32 = 7; /* We've received a Config Ack */
pub const PPP_FSM_ACKSENT: u32 = 8; /* We've sent a Config Ack */
pub const PPP_FSM_OPENED: u32 = 9; /* Connection available */


/*
 * Flags - indicate options controlling FSM operation
 */
pub const OPT_PASSIVE: u32 = 1; /* Don't die if we don't get a response */
pub const OPT_RESTART: u32 = 2; /* Treat 2nd OPEN as DOWN, UP */
pub const OPT_SILENT: u32 = 4; /* Wait for peer to speak first */


/*
 * Timeouts.
 */
#if 0 /* moved to ppp_opts.h */
pub const DEFTIMEOUT: u32 = 3; /* Timeout time in seconds */
pub const DEFMAXTERMREQS: u32 = 2; /* Maximum Terminate-Request transmissions */
pub const DEFMAXCONFREQS: u32 = 10; /* Maximum Configure-Request transmissions */
pub const DEFMAXNAKLOOPS: u32 = 5; /* Maximum number of nak loops */
 /* moved to ppp_opts.h */


/*
 * Prototypes
 */
void fsm_init(fsm *f);
void fsm_lowerup(fsm *f);
void fsm_lowerdown(fsm *f);
void fsm_open(fsm *f);
void fsm_close(fsm *f, const char *reason);
void fsm_input(fsm *f, u_char *inpacket, int l);
void fsm_protreject(fsm *f);
void fsm_sdata(fsm *f, u_char code, u_char id, const u_char *data, int datalen);




 /* FSM_H */
 /* PPP_SUPPORT */
