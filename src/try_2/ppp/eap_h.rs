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
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 *
 * Original version by James Carlson
 *
 * $Id: eap.h,v 1.2 2003/06/11 23:56:26 paulus Exp $
 */

/*
 * Packet header = Code, id, length.
 */
pub const EAP_HEADERLEN: u32 = 4;

//  EAP message codes. 
pub const EAP_REQUEST: u32 = 1;
pub const EAP_RESPONSE: u32 = 2;
pub const EAP_SUCCESS: u32 = 3;
pub const EAP_FAILURE: u32 = 4;

//  EAP types 
pub const EAPT_IDENTITY: u32 = 1;
pub const EAPT_NOTIFICATION: u32 = 2;
pub const EAPT_NAK: u32 = 3; //  (response only) 
pub const EAPT_MD5CHAP: u32 = 4;
pub const EAPT_OTP: u32 = 5; //  One-Time Password; RFC 1938 
pub const EAPT_TOKEN: u32 = 6; //  Generic Token Card 
//  7 and 8 are unassigned. 
pub const EAPT_RSA: u32 = 9; //  RSA Public Key Authentication 
pub const EAPT_DSS: u32 = 10; //  DSS Unilateral 
pub const EAPT_KEA: u32 = 11; //  KEA 
pub const EAPT_KEA_VALIDATE: u32 = 12; //  KEA-VALIDATE	
pub const EAPT_TLS: u32 = 13; //  EAP-TLS 
pub const EAPT_DEFENDER: u32 = 14; //  Defender Token (AXENT) 
pub const EAPT_W2K: u32 = 15; //  Windows 2000 EAP 
pub const EAPT_ARCOT: u32 = 16; //  Arcot Systems 
pub const EAPT_CISCOWIRELESS: u32 = 17; //  Cisco Wireless 
pub const EAPT_NOKIACARD: u32 = 18; //  Nokia IP smart card 
pub const EAPT_SRP: u32 = 19; //  Secure Remote Password 
//  20 is deprecated 

//  EAP SRP-SHA1 Subtypes 
pub const EAPSRP_CHALLENGE: u32 = 1; //  Request 1 - Challenge 
pub const EAPSRP_CKEY: u32 = 1; //  Response 1 - Client Key 
pub const EAPSRP_SKEY: u32 = 2; //  Request 2 - Server Key 
pub const EAPSRP_CVALIDATOR: u32 = 2; //  Response 2 - Client Validator 
pub const EAPSRP_SVALIDATOR: u32 = 3; //  Request 3 - Server Validator 
pub const EAPSRP_ACK: u32 = 3; //  Response 3 - final ack 
pub const EAPSRP_LWRECHALLENGE: u32 = 4; //  Req/resp 4 - Lightweight rechal 

pub const SRPVAL_EBIT: u32 = 0x00000001; //  Use shared key for ECP 

pub const SRP_PSEUDO_ID: String = "pseudo_".to_string();
pub const SRP_PSEUDO_LEN: u32 = 7;

pub const MD5_SIGNATURE_SIZE: u32 = 16;
pub const EAP_MIN_CHALLENGE_LENGTH: u32 = 17;
pub const EAP_MAX_CHALLENGE_LENGTH: u32 = 24;
pub const EAP_MIN_MAX_POWER_OF_TWO_CHALLENGE_LENGTH: u32 = 3; //  2^3-1 = 7, 17+7 = 24 

pub const EAP_STATES: [String] = [
    "Initial".to_string(),
    "Pending".to_string(),
    "Closed".to_string(),
    "Listen".to_string(),
    "Identify".to_string(),
    "SRP1".to_string(),
    "SRP2".to_string(),
    "SRP3".to_string(),
    "MD5Chall".to_string(),
    "Open".to_string(),
    "SRP4".to_string(),
    "BadAuth".to_string(),
];

// #define	eap_client_active(pcb)	((pcb).eap.es_client.ea_state == eapListen)

// #define	eap_server_active(pcb)	\
// 	((pcb).eap.es_server.ea_state >= eapIdentify && \
// 	 (pcb).eap.es_server.ea_state <= eapMD5Chall)

/*
 * Complete EAP state for one PPP session.
 */
enum eap_state_code {
    eapInitial = 0, //  No EAP authentication yet requested 
    eapPending,     //  Waiting for LCP (no timer) 
    eapClosed,      //  Authentication not in use 
    eapListen,      //  Client ready (and timer running) 
    eapIdentify,    //  EAP Identify sent 
    eapSRP1,        //  Sent EAP SRP-SHA1 Subtype 1 
    eapSRP2,        //  Sent EAP SRP-SHA1 Subtype 2 
    eapSRP3,        //  Sent EAP SRP-SHA1 Subtype 3 
    eapMD5Chall,    //  Sent MD5-Challenge 
    eapOpen,        //  Completed authentication 
    eapSRP4,        //  Sent EAP SRP-SHA1 Subtype 4 
    eapBadAuth,     //  Failed authentication 
}

pub struct eap_auth {
    pub ea_name: String,                   //  Our name 
    pub ea_peer: [String; MAXNAMELEN + 1], //  Peer's name 
    pub ea_session: Vec<u8>,               //  Authentication library linkage 
    pub u_ea_skey: String,                 //  Shared encryption key 
    pub ea_namelen: u16,                   //  Length of our name 
    pub ea_peerlen: u16,                   //  Length of peer's name 
    pub ea_state: eap_state_code,
    pub ea_id: u8,        //  Current id 
    pub ea_requests: u8,  //  Number of Requests sent/received 
    pub ea_responses: u8, //  Number of Responses 
    pub ea_type: u8,      //  One of EAPT_* 
    pub ea_keyflags: u32, //  SRP shared key usage flags 
}

pub const EAP_MAX_CHALLENGE_LENGTH: u32 = 24;

pub struct eap_state {
    pub es_client: eap_auth,  //  Client (authenticatee) data 
    pub es_server: eap_auth,  //  Server (authenticator) data 
    pub letes_savedtime: i32, //  Saved timeout 
    pub letes_savedtime: i32,
    pub letes_savedtime: i32,
    pub letes_savedtime: i32,
    pub letes_savedtime: i32,
    pub es_rechallenge: i32,   //  EAP rechallenge interval 
    pub es_lwrechallenge: i32, //  SRP lightweight rechallenge inter 
    pub es_usepseudo: u8,      //  Use SRP Pseudonym if offered one 
    pub letes_usedpseudo: i32, //  Set if we already sent PN 
    pub es_challen: i32,       //  Length of challenge string 
    pub es_challenge: [u8; EAP_MAX_CHALLENGE_LENGTH],
}

/*
 * Timeouts.
 */

pub const EAP_DEFTIMEOUT: u32 = 3; //  Timeout (seconds) for rexmit 
pub const EAP_DEFTRANSMITS: u32 = 10; //  max # times to transmit 
pub const EAP_DEFREQTIME: u32 = 20; //  Time to wait for peer request 
pub const EAP_DEFALLOWREQ: u32 = 20; //  max # times to accept requests 

// pub fn  eap_authwithpeer(pcb: &mut ppp_pcb, localname: &String);
// pub fn  eap_authpeer(pcb: &mut ppp_pcb, localname: &String);

// extern const struct protent eap_protent;
