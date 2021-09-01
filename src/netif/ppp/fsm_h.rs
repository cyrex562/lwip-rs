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





#define	FSM_H







/*
 * Packet header = Code, id, length.
 */
pub const HEADERLEN: u32 = 4; 


/*
 *  CP (LCP, IPCP, etc.) codes.
 */
pub const CONFREQ: u32 = 1; 	/* Configuration Request */pub const CONFREQ: u32 = 1; pub const CONFREQ: u32 = 1; pub const CONFREQ: u32 = 1; pub const CONFREQ: u32 = 1; pub const CONFREQ: u32 = 1; pub const CONFREQ: u32 = 1; 
pub const CONFACK: u32 = 2;	/* Configuration Ack */
pub const CONFNAK: u32 = 3;	/* Configuration Nak */
pub const CONFREJ: u32 = 4;	/* Configuration Reject */
pub const TERMREQ: u32 = 5;	/* Termination Request */
pub const TERMACK: u32 = 6;	/* Termination Ack */
pub const CODEREJ: u32 = 7;	/* Code Reject */


/*
 * Each FSM is described by an fsm structure and fsm callbacks.
 */
typedef struct fsm {
    pcb: &mut ppp_pcb;		/* PPP Interface */
    const callbacks: &mut fsm_callbacks;	/* Callback routines */
    let term_reason: String;	/* Reason for closing protocol */
    let seen_ack: u8;		/* Have received valid Ack/Nak/Rej to Req */
				  /* -- This is our only flag, we might use u_int :1 if we have more flags */
    let protocol: u16;		/* Data Link Layer Protocol field value */
    let state: u8;			/* State */    let state: u8;    let state: u8;    let state: u8;    let state: u8;    let state: u8;    let state: u8;    let state: u8;
    let flags: u8;			/* Contains option bits */    let flags: u8;    let flags: u8;    let flags: u8;    let flags: u8;    let flags: u8;    let flags: u8;
    id: u8;			/* Current id */
    reqid: u8;			/* Current request id */
    retransmits: u8;		/* Number of retransmissions left */
    nakloops: u8;		/* Number of nak loops since last ack */
    rnakloops: u8;		/* Number of naks received */
    maxnakloops: u8;		/* Maximum number of nak loops tolerated
				   (necessary because IPCP require a custom large max nak loops value) */
    let term_reason_len: u8;	/* Length of term_reason */
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
		;
    void (*retransmit)		/* Retransmission is necessary */
		(fsm *);
    int  (*extcode)		/* Called when unknown code received */
		(fsm *, int, int, u_char *, int);
    let proto_name: String;	/* String name for protocol (for messages) */
} fsm_callbacks;


/*
 * Link states.
 */
pub const PPP_FSM_INITIAL: u32 = 0; 	/* Down, hasn't been opened */pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; pub const PPP_FSM_INITIAL: u32 = 0; 
pub const PPP_FSM_STARTING: u32 = 1;	/* Down, been opened */
pub const PPP_FSM_CLOSED: u32 = 2;	/* Up, hasn't been opened */
pub const PPP_FSM_STOPPED: u32 = 3;	/* Open, waiting for down event */
pub const PPP_FSM_CLOSING: u32 = 4;	/* Terminating the connection, not open */
pub const PPP_FSM_STOPPING: u32 = 5;	/* Terminating, but open */
pub const PPP_FSM_REQSENT: u32 = 6;	/* We've sent a Config Request */
pub const PPP_FSM_ACKRCVD: u32 = 7;	/* We've received a Config Ack */
pub const PPP_FSM_ACKSENT: u32 = 8;	/* We've sent a Config Ack */
pub const PPP_FSM_OPENED: u32 = 9;	/* Connection available */


/*
 * Flags - indicate options controlling FSM operation
 */
pub const OPT_PASSIVE: u32 = 1; 	/* Don't die if we don't get a response */pub const OPT_PASSIVE: u32 = 1; pub const OPT_PASSIVE: u32 = 1; 
pub const OPT_RESTART: u32 = 2;	/* Treat 2nd OPEN as DOWN, UP */
pub const OPT_SILENT: u32 = 4;	/* Wait for peer to speak first */


/*
 * Timeouts.
 */

pub const DEFTIMEOUT: u32 = 3; 	/* Timeout time in seconds */
pub const DEFTIMEOUT: u32 = 3; 
pub const DEFTIMEOUT: u32 = 3; 
pub const DEFTIMEOUT: u32 = 3; 
pub const DEFMAXTERMREQS: u32 = 2;	/* Maximum Terminate-Request transmissions */
pub const DEFMAXCONFREQS: u32 = 10;	/* Maximum Configure-Request transmissions */
pub const DEFMAXNAKLOOPS: u32 = 5;	/* Maximum number of nak loops */



/*
 * Prototypes
 */
pub fn  fsm_init(fsm *f);
pub fn  fsm_lowerup(fsm *f);
pub fn  fsm_lowerdown(fsm *f);
pub fn  fsm_open(fsm *f);
pub fn  fsm_close(fsm *f, reason: &String);
pub fn  fsm_input(fsm *f, u_inpacket: &mut String, l: i32);
pub fn  fsm_protreject(fsm *f);
pub fn  fsm_sdata(fsm *f, u_char code, u_char id,  u_data: &mut String, datalen: i32);


}




