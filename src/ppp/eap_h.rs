/*
 * eap.h - Extensible Authentication Protocol for PPP (RFC 2284)
 *
 * Copyright (c) 2001 by Sun Microsystems, Inc.
 * All rights reserved.
 *
 * Non-exclusive rights to redistribute, modify, translate, and use
 * this software in source and binary forms, in whole or in part, is
 * hereby granted, provided that the above copyright notice is
 * duplicated in any source form, and that neither the name of the
 * copyright holder nor the author is used to endorse or promote
 * products derived from this software.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 *
 * Original version by James Carlson
 *
 * $Id: eap.h,v 1.2 2003/06/11 23:56:26 paulus Exp $
 */

// #include "netif/ppp/ppp_opts.h"
// #if PPP_SUPPORT && EAP_SUPPORT  /* don't build if not configured for use in lwipopts.h */




// #include "ppp.h"

#ifdef	__cplusplus
extern 


/*
 * Packet header = Code, id, length.
 */
pub const EAP_HEADERLEN: u32 = 4; /* EAP message codes. */
pub const EAP_REQUEST: u32 = 1; #define	EAP_RESPONSE	2
pub const EAP_SUCCESS: u32 = 3; #define	EAP_FAILURE	4

/* EAP types */
pub const EAPT_IDENTITY: u32 = 1; #define	EAPT_NOTIFICATION	2
pub const EAPT_NAK: u32 = 3; /* (response only) */
pub const EAPT_MD5CHAP: u32 = 4; #define	EAPT_OTP		5	/* One-Time Password; RFC 1938 */
pub const EAPT_TOKEN: u32 = 6; /* Generic Token Card */
/* 7 and 8 are unassigned. */
pub const EAPT_RSA: u32 = 9; /* RSA Public Key Authentication */
pub const EAPT_DSS: u32 = 10; /* DSS Unilateral */
pub const EAPT_KEA: u32 = 11; /* KEA */
pub const EAPT_KEA_VALIDATE: u32 = 12; /* KEA-VALIDATE	*/
pub const EAPT_TLS: u32 = 13; /* EAP-TLS */
pub const EAPT_DEFENDER: u32 = 14; /* Defender Token (AXENT) */
pub const EAPT_W2K: u32 = 15; /* Windows 2000 EAP */
pub const EAPT_ARCOT: u32 = 16; /* Arcot Systems */
pub const EAPT_CISCOWIRELESS: u32 = 17; /* Cisco Wireless */
pub const EAPT_NOKIACARD: u32 = 18; /* Nokia IP smart card */
pub const EAPT_SRP: u32 = 19; /* Secure Remote Password */
/* 20 is deprecated */

/* EAP SRP-SHA1 Subtypes */
pub const EAPSRP_CHALLENGE: u32 = 1; /* Request 1 - Challenge */
pub const EAPSRP_CKEY: u32 = 1; /* Response 1 - Client Key */
pub const EAPSRP_SKEY: u32 = 2; /* Request 2 - Server Key */
pub const EAPSRP_CVALIDATOR: u32 = 2; /* Response 2 - Client Validator */
pub const EAPSRP_SVALIDATOR: u32 = 3; /* Request 3 - Server Validator */
pub const EAPSRP_ACK: u32 = 3; /* Response 3 - final ack */
pub const EAPSRP_LWRECHALLENGE: u32 = 4; /* Req/resp 4 - Lightweight rechal */

pub const SRPVAL_EBIT: u32 = 0x00000001; /* Use shared key for ECP */

#define	SRP_PSEUDO_ID	"pseudo_"
pub const SRP_PSEUDO_LEN: u32 = 7; #define MD5_SIGNATURE_SIZE	16
pub const EAP_MIN_CHALLENGE_LENGTH: u32 = 17; #define EAP_MAX_CHALLENGE_LENGTH	24
pub const EAP_MIN_MAX_POWER_OF_TWO_CHALLENGE_LENGTH: u32 = 3; /* 2^3-1 = 7, 17+7 = 24 */

#define	EAP_STATES	\
	"Initial", "Pending", "Closed", "Listen", "Identify", \
	"SRP1", "SRP2", "SRP3", "MD5Chall", "Open", "SRP4", "BadAuth"

#define	eap_client_active(pcb)	((pcb)->eap.es_client.ea_state == eapListen)
// #if PPP_SERVER
#define	eap_server_active(pcb)	\
	((pcb)->eap.es_server.ea_state >= eapIdentify && \
	 (pcb)->eap.es_server.ea_state <= eapMD5Chall)
 /* PPP_SERVER */

/*
 * Complete EAP state for one PPP session.
 */
enum eap_state_code {
	eapInitial = 0,	/* No EAP authentication yet requested */
	eapPending,	/* Waiting for LCP (no timer) */
	eapClosed,	/* Authentication not in use */
	eapListen,	/* Client ready (and timer running) */
	eapIdentify,	/* EAP Identify sent */
	eapSRP1,	/* Sent EAP SRP-SHA1 Subtype 1 */
	eapSRP2,	/* Sent EAP SRP-SHA1 Subtype 2 */
	eapSRP3,	/* Sent EAP SRP-SHA1 Subtype 3 */
	eapMD5Chall,	/* Sent MD5-Challenge */
	eapOpen,	/* Completed authentication */
	eapSRP4,	/* Sent EAP SRP-SHA1 Subtype 4 */
	eapBadAuth	/* Failed authentication */
};

struct eap_auth {
	const char *ea_name;	/* Our name */
	char ea_peer[MAXNAMELEN +1];	/* Peer's name */
	void *ea_session;	/* Authentication library linkage */
	u_char *ea_skey;	/* Shared encryption key */
	u_short ea_namelen;	/* Length of our name */
	u_short ea_peerlen;	/* Length of peer's name */
	enum eap_state_code ea_state;
	u_char ea_id;		/* Current id */
	u_char ea_requests;	/* Number of Requests sent/received */
	u_char ea_responses;	/* Number of Responses */
	u_char ea_type;		/* One of EAPT_* */
	u32_t ea_keyflags;	/* SRP shared key usage flags */
};


pub const EAP_MAX_CHALLENGE_LENGTH: u32 = 24; typedef struct eap_state {
	struct eap_auth es_client;	/* Client (authenticatee) data */
// #if PPP_SERVER
	struct eap_auth es_server;	/* Server (authenticator) data */
 /* PPP_SERVER */
	int es_savedtime;		/* Saved timeout */
	int es_rechallenge;		/* EAP rechallenge interval */
	int es_lwrechallenge;		/* SRP lightweight rechallenge inter */
	es_usepseudo: u8;		/* Use SRP Pseudonym if offered one */
	int es_usedpseudo;		/* Set if we already sent PN */
	int es_challen;			/* Length of challenge string */
	u_char es_challenge[EAP_MAX_CHALLENGE_LENGTH];
} eap_state;

/*
 * Timeouts.
 */
// #if 0 /* moved to ppp_opts.h */
pub const EAP_DEFTIMEOUT: u32 = 3; /* Timeout (seconds) for rexmit */
pub const EAP_DEFTRANSMITS: u32 = 10; /* max # times to transmit */
pub const EAP_DEFREQTIME: u32 = 20; /* Time to wait for peer request */
pub const EAP_DEFALLOWREQ: u32 = 20; /* max # times to accept requests */
 /* moved to ppp_opts.h */

void eap_authwithpeer(ppp_pcb *pcb, const char *localname);
void eap_authpeer(ppp_pcb *pcb, const char *localname);

extern const struct protent eap_protent;

#ifdef	__cplusplus
}


 /* PPP_EAP_H */

 /* PPP_SUPPORT && EAP_SUPPORT */
