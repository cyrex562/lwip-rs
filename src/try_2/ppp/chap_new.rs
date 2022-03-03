/*
 * chap-new.c - New CHAP implementation.
 *
 * Copyright (c) 2003 Paul Mackerras. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * 3. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by Paul Mackerras
 *     <paulus@samba.org>".
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */





// 







/*
 * CHAP packets begin with a standard header with code, id, len (2 bytes).
 */
pub const CHAP_HDRLEN: u32 = 4; 

/*
 * Values for the code field.
 */
pub const CHAP_CHALLENGE: u32 = 1; 
pub const CHAP_RESPONSE: u32 = 2; 
pub const CHAP_SUCCESS: u32 = 3; 
pub const CHAP_FAILURE: u32 = 4; 

/*
 * CHAP digest codes.
 */
pub const CHAP_MD5: u32 = 5; 

pub const CHAP_MICROSOFT: u32 = 0x80;pub const CHAP_MICROSOFT: u32 = 0x80;
pub const CHAP_MICROSOFT_V2: u32 = 0x81;


/*
 * Semi-arbitrary limits on challenge and response fields.
 */
pub const MAX_CHALLENGE_LEN: u32 = 64; 
pub const MAX_RESPONSE_LEN: u32 = 64; 

/*
 * These limits apply to challenge and response packets we send.
 * The +4 is the +1 that we actually need rounded up.
 */
pub const CHAL_MAX_PKTLEN: usize =	(PPP_HDRLEN + CHAP_HDRLEN + 4 + MAX_CHALLENGE_LEN + MAXNAMELEN);
pub const RESP_MAX_PKTLEN: usize =	(PPP_HDRLEN + CHAP_HDRLEN + 4 + MAX_RESPONSE_LEN + MAXNAMELEN);

//  bitmask of supported algorithms 

pub const MDTYPE_MICROSOFT_V2: u32 = 0x1;
pub const MDTYPE_MICROSOFT: u32 = 0x2;
pub const MDTYPE_MD5: u32 = 0x4;
pub const MDTYPE_NONE: u32 = 0;


//  Return the digest alg. ID for the most preferred digest type. 
pub fn CHAP_DIGEST(mdtype: u32) -> u32 {

	if mdtype & MDTYPE_MD5 {
		return CHAP_MD5;
	} else if mdtype &MDTYPE_MICROSOFT_V2 {
		return CHAP_MICROSOFT_V2;
	} else if mdtype & MDTYPE_MICROSOFT {
		return CHAP_MICROSOFT
	}
	return 0


}



//  Return the bit flag (lsb set) for our most preferred digest type. 
pub fn CHAP_MDTYPE(mdtype: u32) -> u32{ ((mdtype) ^ ((mdtype) - 1)) & (mdtype)}

//  Return the bit flag for a given digest algorithm ID. 

pub fn CHAP_MDTYPE_D(digest: u32) -> u32{ 
	match digest {
		CHAP_MICROSOFT_V2 => MDTYPE_MICROSOFT_V2,
		CHAP_MICROSOFT => MDTYPE_MICROSOFT,
		CHAP_MD5 => MDTYPE_MD5,
		_ => 0
	}
}


//  Can we do the requested digest? 
pub fn CHAP_CANDIGEST(mdtype: u32, digest: u32) -> u32 {
	match digest {
		CHAP_MICROSOFT_V2 => mdtype &MDTYPE_MICROSOFT_V2,
		CHAP_MICROSOFT => mdtype & MDTYPE_MICROSOFT,
		CHAP_MD5 => mdtype & MDTYPE_MD5,
		_ => 0
	}
}


/*
 * The code for each digest type has to supply one of these.
 */
// pub struct chap_digest_type {
// 	let code: i32;


// 	/*
// 	 * Note: challenge and response arguments below are formatted as
// 	 * a length byte followed by the actual challenge/response data.
// 	 */
// 	void (*generate_challenge)(pcb: &mut ppp_pcb,  challenge: &mut String);

// 	int (*verify_response)(pcb: &mut ppp_pcb, id: i32, name: &String,
//   secret: &mut String, secret_len: i32,
//   challenge: &mut String,   response: &mut String,
// 		message: &mut String, message_space: i32);

// 	void (*make_response)(pcb: &mut ppp_pcb,  response: &mut String, id: i32, our_name: &String,
//   challenge: &mut String, secret: &String, secret_len: i32,
// 		 priv: &mut String);

// 	int (*check_success)(pcb: &mut ppp_pcb,  pkt: &mut String, len: i32,  priv: &mut String);

// 	void (*handle_failure)(pcb: &mut ppp_pcb,  pkt: &mut String, len: i32);
// };

/*
 * Each interface is described by chap structure.
 */

pub struct chap_client_state {
	pub flags: u8,
	pub name: String,
 	pub digest: chap_digest_type,
	pub priv: String,		//  private area for digest's use 
}


pub struct chap_server_state {
	pub flags: u8,
	pub id: u8,
	pub name: String,
 pub digest: chap_digest_type,
	pub pubchallenge_xmits: i32,
	pub pubchallenge_pktlen: i32,
	 pub challenge: String,
} 




//  Hook for a plugin to validate CHAP challenge 
// extern int (*chap_verify_hook)(name: &mut String, ourname: &mut String, id: i32,
//  digest: &mut chap_digest_type,
// 			 challenge: &mut String,  response: &mut String,
// 			message: &mut String, message_space: i32);



//  Called by authentication code to start authenticating the peer. 
// extern void chap_auth_peer(pcb: &mut ppp_pcb, our_name: &String, digest_code: i32);


//  Called by auth. code to start authenticating us to the peer. 
// extern void chap_auth_with_peer(pcb: &mut ppp_pcb, our_name: &String, digest_code: i32);

//  Represents the CHAP protocol to the main pppd code 
// extern const struct protent chap_protent;







