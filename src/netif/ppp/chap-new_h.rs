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





#define CHAP_H







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
pub const CHAP_MICROSOFT_V2: u32 = 0; x81


/*
 * Semi-arbitrary limits on challenge and response fields.
 */
pub const MAX_CHALLENGE_LEN: u32 = 64; 
pub const MAX_RESPONSE_LEN: u32 = 64; 

/*
 * These limits apply to challenge and response packets we send.
 * The +4 is the +1 that we actually need rounded up.
 */
#define CHAL_MAX_PKTLEN	(PPP_HDRLEN + CHAP_HDRLEN + 4 + MAX_CHALLENGE_LEN + MAXNAMELEN)
#define RESP_MAX_PKTLEN	(PPP_HDRLEN + CHAP_HDRLEN + 4 + MAX_RESPONSE_LEN + MAXNAMELEN)

/* bitmask of supported algorithms */

pub const MDTYPE_MICROSOFT_V2: u32 = 0x1;pub const MDTYPE_MICROSOFT_V2: u32 = 0x1;
pub const MDTYPE_MICROSOFT: u32 = 0; x2

pub const MDTYPE_MD5: u32 = 0x4;
pub const MDTYPE_NONE: u32 = 0;


/* Return the digest alg. ID for the most preferred digest type. */
#define CHAP_DIGEST(mdtype) \
    ((mdtype) & MDTYPE_MD5)? CHAP_MD5: \
    ((mdtype) & MDTYPE_MICROSOFT_V2)? CHAP_MICROSOFT_V2: \
    ((mdtype) & MDTYPE_MICROSOFT)? CHAP_MICROSOFT: \
    0
 /* !MSCHAP_SUPPORT */
#define CHAP_DIGEST(mdtype) \
    ((mdtype) & MDTYPE_MD5)? CHAP_MD5: \
    0


/* Return the bit flag (lsb set) for our most preferred digest type. */
#define CHAP_MDTYPE(mdtype) ((mdtype) ^ ((mdtype) - 1)) & (mdtype)

/* Return the bit flag for a given digest algorithm ID. */

#define CHAP_MDTYPE_D(digest) \
    ((digest) == CHAP_MICROSOFT_V2)? MDTYPE_MICROSOFT_V2: \
    ((digest) == CHAP_MICROSOFT)? MDTYPE_MICROSOFT: \
    ((digest) == CHAP_MD5)? MDTYPE_MD5: \
    0
 /* !MSCHAP_SUPPORT */
#define CHAP_MDTYPE_D(digest) \
    ((digest) == CHAP_MD5)? MDTYPE_MD5: \
    0


/* Can we do the requested digest? */

#define CHAP_CANDIGEST(mdtype, digest) \
    ((digest) == CHAP_MICROSOFT_V2)? (mdtype) & MDTYPE_MICROSOFT_V2: \
    ((digest) == CHAP_MICROSOFT)? (mdtype) & MDTYPE_MICROSOFT: \
    ((digest) == CHAP_MD5)? (mdtype) & MDTYPE_MD5: \
    0
 /* !MSCHAP_SUPPORT */
#define CHAP_CANDIGEST(mdtype, digest) \
    ((digest) == CHAP_MD5)? (mdtype) & MDTYPE_MD5: \
    0


/*
 * The code for each digest type has to supply one of these.
 */
struct chap_digest_type {
	let letcode: i32;


	/*
	 * Note: challenge and response arguments below are formatted as
	 * a length byte followed by the actual challenge/response data.
	 */
	void (*generate_challenge)(pcb: &mut ppp_pcb,  challenge: &mut String);
	int (*verify_response)(pcb: &mut ppp_pcb, id: i32, name: &String,
		const  secret: &mut String, secret_len: i32,
		const  challenge: &mut String,   response: &mut String,
		message: &mut String, message_space: i32);

	void (*make_response)(pcb: &mut ppp_pcb,  response: &mut String, id: i32, our_name: &String,
		const  challenge: &mut String, secret: &String, secret_len: i32,
		 priv: &mut String);
	int (*check_success)(pcb: &mut ppp_pcb,  pkt: &mut String, len: i32,  priv: &mut String);
	void (*handle_failure)(pcb: &mut ppp_pcb,  pkt: &mut String, len: i32);
};

/*
 * Each interface is described by chap structure.
 */

typedef struct chap_client_state {
	let flags: u8;
	name: String;
	const digest: &mut chap_digest_type;
	 let priv: String;		/* private area for digest's use */
} chap_client_state;


typedef struct chap_server_state {
	let flags: u8;
	let id: u8;
	name: String;
	const digest: &mut chap_digest_type;
	let letchallenge_xmits: i32;
	let letchallenge_pktlen: i32;
	 let challenge: String;
} chap_server_state;




/* Hook for a plugin to validate CHAP challenge */
extern int (*chap_verify_hook)(name: &mut String, ourname: &mut String, id: i32,
			const digest: &mut chap_digest_type,
			 challenge: &mut String,  response: &mut String,
			message: &mut String, message_space: i32);



/* Called by authentication code to start authenticating the peer. */
extern void chap_auth_peer(pcb: &mut ppp_pcb, our_name: &String, digest_code: i32);


/* Called by auth. code to start authenticating us to the peer. */
extern void chap_auth_with_peer(pcb: &mut ppp_pcb, our_name: &String, digest_code: i32);

/* Represents the CHAP protocol to the main pppd code */
extern const struct protent chap_protent;


}




