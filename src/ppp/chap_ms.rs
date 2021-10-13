/*
 * chap_ms.c - Microsoft MS-CHAP compatible implementation.
 *
 * Copyright (c) 1995 Eric Rosenquist.  All rights reserved.
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
 * 3. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

/*
 * Modifications by Lauri Pesonen / lpesonen@clinet.fi, april 1997
 *
 *   Implemented LANManager type password response to MS-CHAP challenges.
 *   Now pppd provides both NT style and LANMan style blocks, and the
 *   prefered is set by option "ms-lanman". Default is to use NT.
 *   The hash text (StdText) was taken from Win95 RASAPI32.DLL.
 *
 *   You should also use DOMAIN\\USERNAME as described in README.MSCHAP80
 */

/*
 * Modifications by Frank Cusack, frank@google.com, March 2002.
 *
 *   Implemented MS-CHAPv2 functionality, heavily based on sample
 *   implementation in RFC 2759.  Implemented MPPE functionality,
 *   heavily based on sample implementation in RFC 3079.
 *
 * Copyright (c) 2002 Google, Inc.  All rights reserved.
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
 * 3. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 */

use crate::ppp::pppcrypt::pppcrypt_56_to_64_bit_key;
use log::{debug,info,warn,error};
use crate::core::context::LwipContext;
use crate::core::crypto::{des_setkey_enc, DesContext};
use crate::ppp::magic::magic_random_bytes;
use crate::ppp::ppp_h::PppCtx;

pub const SHA1_SIGNATURE_SIZE: isize = 20;
pub const MD4_SIGNATURE_SIZE: isize = 16; //  16 bytes in a MD4 message digest
pub const MAX_NT_PASSWORD: isize = 256; //Max (Unicode) chars in an NT pass *
pub const MS_CHAP_RESPONSE_LEN: isize = 49; //  Response length for MS-CHAP
pub const MS_CHAP2_RESPONSE_LEN: isize = 49; //  Response length for MS-CHAPv2
pub const MS_AUTH_RESPONSE_LENGTH: isize = 40; //  MS-CHAPv2 authenticator response, as ASCII

//  Error codes for MS-CHAP failure messages. 
pub const MS_CHAP_ERROR_RESTRICTED_LOGON_HOURS: u32 = 646;
pub const MS_CHAP_ERROR_ACCT_DISABLED: u32 = 647;
pub const MS_CHAP_ERROR_PASSWD_EXPIRED: u32 = 648;
pub const MS_CHAP_ERROR_NO_DIALIN_PERMISSION: u32 = 649;
pub const MS_CHAP_ERROR_AUTHENTICATION_FAILURE: u32 = 691;
pub const MS_CHAP_ERROR_CHANGING_PASSWORD: u32 = 709;

/*
 * Offsets within the response field for MS-CHAP
 */
pub const MS_CHAP_LANMANRESP: u32 = 0;
pub const MS_CHAP_LANMANRESP_LEN: u32 = 24;
pub const MS_CHAP_NTRESP: u32 = 24;
pub const MS_CHAP_NTRESP_LEN: u32 = 24;
pub const MS_CHAP_USENT: u32 = 48;

/*
 * Offsets within the response field for MS-CHAP2
 */
pub const MS_CHAP2_PEER_CHALLENGE: u32 = 0;
pub const MS_CHAP2_PEER_CHAL_LEN: u32 = 16;
pub const MS_CHAP2_RESERVED_LEN: u32 = 8;
pub const MS_CHAP2_NTRESP: u32 = 24;
pub const MS_CHAP2_NTRESP_LEN: u32 = 24;
pub const MS_CHAP2_FLAGS: u32 = 48;

//  These values are the RADIUS attribute values--see RFC 2548. 
pub const MPPE_ENC_POL_ENC_ALLOWED: u32 = 1;
pub const MPPE_ENC_POL_ENC_REQUIRED: u32 = 2;
pub const MPPE_ENC_TYPES_RC4_40: u32 = 2;
pub const MPPE_ENC_TYPES_RC4_128: u32 = 4;

//  used by plugins (using above values) 
// extern void set_mppe_enc_types(int, int);

//  Are we the authenticator or authenticatee?  For MS-CHAPv2 key derivation. 
pub const MS_CHAP2_AUTHENTICATEE: u32 = 0;
pub const MS_CHAP2_AUTHENTICATOR: u32 = 1;

// pub fn	ascii2unicode ( [u8;], int, u_char[]);
// pub fn	NTPasswordHash (u_char *, int, u_char[MD4_SIGNATURE_SIZE]);
// pub fn	challenge_response ( u_char *,  u_char *, u_char[24]);
// pub fn	ChallengeHash ( u_char[16],  u_char *,  char *, u_char[8]);
// pub fn	ChapMS_NT ( u_char *,  char *, int, u_char[24]);
// pub fn	ChapMS2_NT ( u_char *,  u_char[16],  char *,  char *, int,
// 				u_char[24]);
// pub fn	GenerateAuthenticatorResponsePlain
// 			( char*, int, u_char[24],  u_char[16],  u_char *,
//  char *, u_char[41]);

// pub fn	ChapMS_LANMan (u_char *, char *, int, u_char *);

// pub fn GenerateAuthenticatorResponse( PasswordHashHash: [u8;MD4_SIGNATURE_SIZE],
// 			NTResponse: [u8;24],  PeerChallenge: [u8;16],
//  u_rchallenge: &mut String, username: &String,
// 			authResponse: [u8;MS_AUTH_RESPONSE_LENGTH+1]);

// pub fn	Set_Start_Key (pcb: &mut ppp_pcb,  u_char *,  char *, int);
// pub fn	SetMasterKeys (pcb: &mut ppp_pcb,  char *, int, u_char[24], int);

// pub fn ChapMS (pcb: &mut ppp_pcb,  u_char *,  char *, int, u_char *);
// pub fn ChapMS2 (pcb: &mut ppp_pcb,  u_char *,  u_char *,  char *,  char *, int,
// 		  u_char *, u_char[MS_AUTH_RESPONSE_LENGTH+1], int);

// bool	ms_lanman = 0;    	//  Use LanMan password instead of NT 
//  Has meaning only with MS-CHAP challenges 

//  For MPPE debug 
//  Use "[]|}{?/><,`!2&&(" (sans quotes) for RFC 3079 MS-CHAPv2 test value 
// static mschap_challenge: &mut String = None;
//  Use "!@\#$%^&*()_+:3|!" (sans quotes, backslash is to escape #) for ... 
// static mschap2_peer_challenge: &mut String = None;

/*
 * Command-line options.
 */
// static option_t chapms_option_list[] = {

// 	{ "ms-lanman", o_bool, &ms_lanman,
// 	  "Use LanMan passwd when using MS-CHAP", 1 },

// 	{ "mschap-challenge", o_string, &mschap_challenge,
// 	  "specify CHAP challenge" },
// 	{ "mschap2-peer-challenge", o_string, &mschap2_peer_challenge,
// 	  "specify CHAP peer challenge" },

// 	{ None }
// };

/*
 * chapms_generate_challenge - generate a challenge for MS-CHAP.
 * For MS-CHAP the challenge length is fixed at 8 bytes.
 * The length goes in challenge[0] and the actual challenge starts
 * at challenge[1].
 */
pub fn chapms_generate_challenge(pcb: &mut PppCtx, challenge: &mut String) {
    let mut challenge_idx: isize = 0;
    challenge[challenge_idx] = 0x8;
    challenge_idx +=1;
    if mschap_challenge && strlen(mschap_challenge) == 8 {
        memcpy(challenge, mschap_challenge, 8);
    } else {
        magic_random_bytes(challenge, 8);
    }
}

pub fn chapms2_generatse_challenge(pcb: &mut PppCtx, challenge: &mut String) {
    // *challenge += 1 = 16;

    if (mschap_challenge && strlen(mschap_challenge) == 16) {
        memcpy(challenge, mschap_challenge, 16);
    } else {
        magic_random_bytes(challenge, 16);
    }
}

pub fn chapms_verify_response(
    pcb: &mut PppCtx,
    id: i32,
    name: &String,
    secret: &mut Vec<u8>,
    secret_len: isize,
    challenge: &mut Vec<u8>,
    response: &mut Vec<u8>,
    message: &mut Vec<u8>,
    message_space: i32,
) -> i32 {
    let md: String;
    let letdiff: i32;
    let mut challenge_len: isize;
    let mut challenge_idx: isize = 0;
    let mut response_len: isize;
    let mut response_idx: isize = 0;

    challenge_len = challenge[challenge_idx]; //  skip length, is 8
    challenge_idx += 1;
    response_len = response[response_idx];
    response_idx += 1;
    if response_len != MS_CHAP_RESPONSE_LEN {
        // goto bad;
    }

    if !response[MS_CHAP_USENT] {
        //  Should really propagate this into the error packet. 
        error!("Peer request for LANMAN auth not supported");
        // goto bad;
    }

    //  Generate the expected response. 
    chap_ms(pcb, challenge, secret, secret_len, &mut md);

    //  Determine which part of response to verify against 
    if (!response[MS_CHAP_USENT]) {
        diff = memcmp(
            &response[MS_CHAP_LANMANRESP],
            &md[MS_CHAP_LANMANRESP],
            MS_CHAP_LANMANRESP_LEN,
        );
    } else {
        diff = memcmp(
            &response[MS_CHAP_NTRESP],
            &md[MS_CHAP_NTRESP],
            MS_CHAP_NTRESP_LEN,
        );
    }

    if (diff == 0) {
        ppp_slprintf(message, message_space, "Access granted");
        return 1;
    }

    // bad:
    //  See comments below for MS-CHAP V2 
    ppp_slprintf(
        message,
        message_space,
        "E=691 R=1 C=%0.*B V=0",
        challenge_len,
        challenge,
    );
    return 0;
}

pub fn chapms2_verify_response(
    pcb: &mut PppCtx,
    id: i32,
    name: &String,
    secret: &mut String,
    secret_len: i32,
    challenge: &mut String,
    response: &mut String,
    message: &mut String,
    message_space: i32,
) -> i32 {
    let md: String;
    // saresponse: [u8;MS_AUTH_RESPONSE_LENGTH+1];
    let saresponse: String;
    let challenge_len: i32;
    let response_len: i32;

    challenge_len = *challenge += 1; //  skip length, is 16 
    response_len = *response += 1;
    if (response_len != MS_CHAP2_RESPONSE_LEN) {
        // goto bad;	//  not even the right length 
    }
    //  Generate the expected response and our mutual auth. 
    ChapMS2(
        pcb,
        challenge,
        &response[MS_CHAP2_PEER_CHALLENGE],
        name,
        secret,
        secret_len,
        &mut md,
        saresponse,
        MS_CHAP2_AUTHENTICATOR,
    );

    //  compare MDs and send the appropriate status 
    /*
     * Per RFC 2759, success message must be formatted as
     *     "S=<auth_string> M=<message>"
     * where
     *     <auth_string> is the Authenticator Response (mutual auth)
     *     <message> is a text message
     *
     * However, some versions of Windows (win98 tested) do not know
     * about the M=<message> part (required per RFC 2759) and flag
     * it as an error (reported incorrectly as an encryption error
     * to the user).  Since the RFC requires it, and it can be
     * useful information, we supply it if the peer is a conforming
     * system.  Luckily (?), win98 sets the Flags field to 0x04
     * (contrary to RFC requirements) so we can use that to
     * distinguish between conforming and non-conforming systems.
     *
     * Special thanks to Alex Swiridov <say@real.kharkov.ua> for
     * help debugging this.
     */
    if (memcmp(
        &md[MS_CHAP2_NTRESP],
        &response[MS_CHAP2_NTRESP],
        MS_CHAP2_NTRESP_LEN,
    ) == 0)
    {
        if (response[MS_CHAP2_FLAGS]) {
            ppp_slprintf(message, message_space, "S=%s", saresponse);
        } else {
            ppp_slprintf(
                message,
                message_space,
                "S=%s M=%s",
                saresponse,
                "Access granted",
            );
        }
        return 1;
    }

    // bad:
    /*
     * Failure message must be formatted as
     *     "E=e R=r C=c V=v M=m"
     * where
     *     e = error code (we use 691, ERROR_AUTHENTICATION_FAILURE)
     *     r = retry (we use 1, ok to retry)
     *     c = challenge to use for next response, we reuse previous
     *     v = Change Password version supported, we use 0
     *     m = text message
     *
     * The M=m part is only for MS-CHAPv2.  Neither win2k nor
     * win98 (others untested) display the message to the user anyway.
     * They also both ignore the E=e code.
     *
     * Note that it's safe to reuse the same challenge as we don't
     * actually accept another response based on the error message
     * (and no clients try to resend a response anyway).
     *
     * Basically, this whole bit is useless code, even the small
     * implementation here is only because of overspecification.
     */
    ppp_slprintf(
        message,
        message_space,
        "E=691 R=1 C=%0.*B V=0 M=%s",
        challenge_len,
        challenge,
        "Access denied",
    );
    return 0;
}

pub fn chapms_make_response(
    pcb: &mut PppCtx,
    response: &mut String,
    id: i32,
    our_name: &String,
    challenge: &mut String,
    secret: &String,
    secret_len: i32,
    private_: &mut String,
) {
    challenge += 1; //  skip length, should be 8 
    *response += 1 = MS_CHAP_RESPONSE_LEN;
    chap_ms(pcb, challenge, secret, secret_len, response);
}

pub fn chapms2_make_response(
    pcb: &mut PppCtx,
    response: &mut String,
    id: i32,
    our_name: &String,
    challenge: &mut String,
    secret: &String,
    secret_len: i32,
    private_: &mut String,
) {
    challenge += 1; //  skip length, should be 16 
    *response += 1 = MS_CHAP2_RESPONSE_LEN;
    ChapMS2(
        pcb,
        challenge,
        mschap2_peer_challenge,
        None,
        our_name,
        secret,
        secret_len,
        response,
        private_,
        MS_CHAP2_AUTHENTICATEE,
    );
}

pub fn chapms2_check_success(
    pcb: &mut PppCtx,
    msg: &mut String,
    len: i32,
    private_: &mut String,
) -> i32 {
    if ((len < MS_AUTH_RESPONSE_LENGTH + 2) || strncmp(msg, "S=", 2) != 0) {
        //  Packet does not start with "S=" 
        ppp_error("MS-CHAPv2 Success packet is badly formed.");
        return 0;
    }
    msg += 2;
    len -= 2;
    if (len < MS_AUTH_RESPONSE_LENGTH || memcmp(msg, private_, MS_AUTH_RESPONSE_LENGTH)) {
        //  Authenticator Response did not match expected. 
        ppp_error("MS-CHAPv2 mutual authentication failed.");
        return 0;
    }
    //  Authenticator Response matches. 
    msg += MS_AUTH_RESPONSE_LENGTH; //  Eat it 
    len -= MS_AUTH_RESPONSE_LENGTH;
    if ((len >= 3) && !strncmp(msg, " M=", 3)) {
        msg += 3; //  Eat the delimiter 
    } else if (len) {
        //  Packet has extra text which does not begin " M=" 
        ppp_error("MS-CHAPv2 Success packet is badly formed.");
        return 0;
    }
    return 1;
}

pub fn chapms_handle_failure(pcb: &mut PppCtx, inp: &mut String, len: i32) {
    let leterr: i32;
    let p: String;
    let msg: String;

    //  We want a null-terminated string for strxxx(). 
    len = LWIP_MIN(len, 63);
    MEMCPY(msg, inp, len);
    msg[len] = 0;
    p = msg;

    /*
     * Deal with MS-CHAP formatted failure messages; just prthe: i32
     * M=<message> part (if any).  For MS-CHAP we're not really supposed
     * to use M=<message>, but it shouldn't hurt.  See
     * chapms[2]_verify_response.
     */
    if (!strncmp(p, "E=", 2)) {
        err = strtol(p + 2, None, 10);
    }
    //  Remember the error code. 
    else {
    }
    // goto print_msg; //  Message is badly formatted. 
    if (len && ((p = strstr(p, " M=")) != None)) {
        //  M=<message> field found. 
        p += 3;
    } else {
        //  No M=<message>; use the error code. 
        match (err) {
            MS_CHAP_ERROR_RESTRICTED_LOGON_HOURS => {
                p = "E=646 Restricted logon hours";
            }
            MS_CHAP_ERROR_ACCT_DISABLED => {
                p = "E=647 Account disabled";
            }
            MS_CHAP_ERROR_PASSWD_EXPIRED => {
                p = "E=648 Password expired";
            }
            MS_CHAP_ERROR_NO_DIALIN_PERMISSION => {
                p = "E=649 No dialin permission";
            }
            MS_CHAP_ERROR_AUTHENTICATION_FAILURE => {
                p = "E=691 Authentication failure";
            }
            MS_CHAP_ERROR_CHANGING_PASSWORD => {
                //  Should never see this, we don't support Change Password. 
                p = "E=709 Error changing password";
            }
            _ => {
                ppp_error("Unknown MS-CHAP authentication failure: %.*v", len, inp);
            }
        }
    }
    // print_msg:
    if p != None {
        ppp_error("MS-CHAP authentication failed: %v", p);
    }
}

pub fn challenge_response(ctx: &mut LwipContext, challenge: &mut String, password_hash: &[u8], response: &[u8]) {
    let mut zpassword_hash: [u8; 21] = [0;21];
    let mut des: DesContext;
    let mut des_key: [u8; 8] = [0;8];

    // MEMCPY(zpassword_hash, password_hash, MD4_SIGNATURE_SIZE);
    zpassword_hash.copy_from_slice(password_hash);

    debug!("challenge response: zpassword_hash: {:?}B: {:?}", zpassword_hash.len(), zpassword_hash);

    pppcrypt_56_to_64_bit_key(&zpassword_hash, &mut des_key);
    // lwip_des_init(&des);
    des_setkey_enc(&des, &des_key);
    des_crypt_ecb(&des, challenge, response + 0);
    des_free(&des);

    pppcrypt_56_to_64_bit_key(&zpassword_hash[7..], &mut des_key);
    // lwip_des_init(&des);
    des_setkey_enc(&des, des_key);
    des_crypt_ecb(&des, challenge, response + 8);
    des_free(&des);

    pppcrypt_56_to_64_bit_key(zpassword_hash + 14, &mut des_key);
    des_init(&des);
    des_setkey_enc(&des, des_key);
    des_crypt_ecb(&des, challenge, response + 16);
    des_free(&des);

    dbglog("challenge_response - response %.24B", response);
}

pub fn ChallengeHash(
    PeerChallenge: &[u8],
    u_rchallenge: &mut String,
    username: &String,
    Challenge: &[u8],
) {
    let sha1Context: lwip_sha1_context;
    let sha1Hash: [u8; SHA1_SIGNATURE_SIZE];
    let user: String;

    //  remove domain from "domain\username" 
    if ((user = strrchr(username, '\\')) != None) {
        user += 1;
    } else {
        user = username;
    }

    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, PeerChallenge, 16);
    lwip_sha1_update(&sha1Context, rchallenge, 16);
    lwip_sha1_update(&sha1Context, user, strlen(user));
    lwip_sha1_finish(&sha1Context, sha1Hash);
    lwip_sha1_free(&sha1Context);

    MEMCPY(Challenge, sha1Hash, 8);
}

/*
 * Convert the ASCII version of the password to Unicode.
 * This implicitly supports 8-bit ISO8859/1 characters.
 * This gives us the little-endian representation, which
 * is assumed by all M$ CHAP RFCs.  (Unicode byte ordering
 * is machine-dependent.)
 */
pub fn ascii2unicode(ascii: &mut String, ascii_len: i32, unicode: &mut String) {
    let leti: i32;

    BZERO(unicode, ascii_len * 2);
    // for (i = 0; i < ascii_len; i+= 1){
    // unicode[i * 2] = (u_char) ascii[i];}
}

pub fn NTPasswordHash(u_secret: &mut String, secret_len: i32, hash: &mut [u8]) {
    let md4Context: lwip_md4_context;

    lwip_md4_init(&md4Context);
    lwip_md4_starts(&md4Context);
    lwip_md4_update(&md4Context, secret, secret_len);
    lwip_md4_finish(&md4Context, hash);
    lwip_md4_free(&md4Context);
}

pub fn ChapMS_NT(
    u_rchallenge: &mut String,
    secret: &String,
    secret_len: i32,
    NTResponse: &mut [u8],
) {
    let mut unicodePassword: [u8; MAX_NT_PASSWORD * 2];
    let mut PasswordHash: [u8; MD4_SIGNATURE_SIZE];

    //  Hash the Unicode version of the secret (== password). 
    ascii2unicode(secret, secret_len, unicodePassword);
    NTPasswordHash(unicodePassword, secret_len * 2, &mut PasswordHash);

    challenge_response(ctx, rchallenge, &PasswordHash, NTResponse);
}

pub fn ChapMS2_NT(
    u_rchallenge: &mut String,
    PeerChallenge: &mut [u8],
    username: &String,
    secret: &String,
    secret_len: i32,
    NTResponse: &mut [u8],
) {
    let mut unicodePassword: [u8; MAX_NT_PASSWORD * 2];
    let mut PasswordHash: [u8; MD4_SIGNATURE_SIZE];
    let mut Challenge: [u8; 8];

    ChallengeHash(PeerChallenge, rchallenge, username, &Challenge);

    //  Hash the Unicode version of the secret (== password). 
    ascii2unicode(secret, secret_len, unicodePassword);
    NTPasswordHash(unicodePassword, secret_len * 2, &mut PasswordHash);

    challenge_response(ctx, Challenge, &PasswordHash, NTResponse);
}

pub const u_StdText: String = "KGS!@#$%".to_string(); //  key from rasapi32.dll 

pub fn ChapMS_LANMan(
    u_rchallenge: &mut String,
    secret: &mut String,
    secret_len: i32,
    response: &mut String,
) {
    let mut i: i32;
    let mut UcasePassword: [u8; MAX_NT_PASSWORD];
    let mut PasswordHash: [u8; MD4_SIGNATURE_SIZE];
    let mut des: lwip_des_context;
    let mut des_key: [u8; 8];

    //  LANMan password is case insensitive 
    BZERO(UcasePassword, sizeof(UcasePassword));
    // for (i = 0; i < secret_len; i+= 1){
    //    UcasePassword[i] = (u_char)toupper(secret[i]);}

    pppcrypt_56_to_64_bit_key(UcasePassword + 0, des_key);
    lwip_des_init(&des);
    lwip_des_setkey_enc(&des, des_key);
    lwip_des_crypt_ecb(&des, StdText, PasswordHash + 0);
    lwip_des_free(&des);

    pppcrypt_56_to_64_bit_key(UcasePassword + 7, des_key);
    lwip_des_init(&des);
    lwip_des_setkey_enc(&des, des_key);
    lwip_des_crypt_ecb(&des, StdText, PasswordHash + 8);
    lwip_des_free(&des);

    challenge_response(ctx, rchallenge, &PasswordHash, &response[MS_CHAP_LANMANRESP]);
}

/*
 * "Magic" constants used in response generation, from RFC 2759.
 */
pub const Magic1: [u8; 39] = //  "Magic server to client signing constant" 
    [
        0x4D, 0x61, 0x67, 0x69, 0x63, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x6F,
        0x20, 0x63, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x20, 0x73, 0x69, 0x67, 0x6E, 0x69, 0x6E, 0x67,
        0x20, 0x63, 0x6F, 0x6E, 0x73, 0x74, 0x61, 0x6E, 0x74,
    ];
pub const Magic2: [u8; 41] = //  "Pad to make it do more than one iteration" 
    [
        0x50, 0x61, 0x64, 0x20, 0x74, 0x6F, 0x20, 0x6D, 0x61, 0x6B, 0x65, 0x20, 0x69, 0x74, 0x20,
        0x64, 0x6F, 0x20, 0x6D, 0x6F, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x6E, 0x20, 0x6F, 0x6E,
        0x65, 0x20, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
    ];

pub fn GenerateAuthenticatorResponse(
    PasswordHashHash: &mut [u8],
    NTResponse: &mut [u8],
    PeerChallenge: &mut [u8],
    u_rchallenge: &mut String,
    username: &String,
    authResponse: &mut [u8],
) {
    let mut i;
    let mut sha1Context: lwip_sha1_context;
    let mut Digest: [u8; SHA1_SIGNATURE_SIZE];
    let mut Challenge: [u8; 8];
    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, PasswordHashHash, MD4_SIGNATURE_SIZE);
    lwip_sha1_update(&sha1Context, NTResponse, 24);
    lwip_sha1_update(&sha1Context, Magic1, sizeof(Magic1));
    lwip_sha1_finish(&sha1Context, Digest);
    lwip_sha1_free(&sha1Context);

    ChallengeHash(PeerChallenge, rchallenge, username, &Challenge);

    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, Digest, sizeof(Digest));
    lwip_sha1_update(&sha1Context, Challenge, sizeof(Challenge));
    lwip_sha1_update(&sha1Context, Magic2, sizeof(Magic2));
    lwip_sha1_finish(&sha1Context, Digest);
    lwip_sha1_free(&sha1Context);

    //  Convert to ASCII hex string. 
    // for (i = 0; i < LWIP_MAX((MS_AUTH_RESPONSE_LENGTH / 2), sizeof(Digest)); i+= 1){
    // sprintf(&authResponse[i * 2], "%02X", Digest[i]);}
}

pub fn GenerateAuthenticatorResponsePlain(
    secret: &String,
    secret_len: i32,
    NTResponse: &mut [u8],
    PeerChallenge: &mut [u8],
    u_rchallenge: &mut String,
    username: &String,
    authResponse: &mut [u8],
) {
    let unicodePassword: [u8; MAX_NT_PASSWORD * 2];
    let PasswordHash: [u8; MD4_SIGNATURE_SIZE];
    let PasswordHashHash: [u8; MD4_SIGNATURE_SIZE];

    //  Hash (x2) the Unicode version of the secret (== password). 
    ascii2unicode(secret, secret_len, unicodePassword);
    NTPasswordHash(unicodePassword, secret_len * 2, &mut PasswordHash);
    NTPasswordHash(PasswordHash, sizeof(PasswordHash), &mut PasswordHashHash);

    GenerateAuthenticatorResponse(
        &mut PasswordHashHash,
        NTResponse,
        PeerChallenge,
        rchallenge,
        username,
        authResponse,
    );
}

/*
 * Set mppe_xxxx_key from MS-CHAP credentials. (see RFC 3079)
 */
pub fn Set_Start_Key(
    pcb: &mut PppCtx,
    u_rchallenge: &mut String,
    secret: &String,
    secret_len: i32,
) {
    let unicodePassword: [u8; MAX_NT_PASSWORD * 2];
    let PasswordHash: [u8; MD4_SIGNATURE_SIZE];
    let PasswordHashHash: [u8; MD4_SIGNATURE_SIZE];
    let sha1Context: lwip_sha1_context;
    let Digest: [U8; SHA1_SIGNATURE_SIZE]; //  >= MPPE_MAX_KEY_LEN 

    //  Hash (x2) the Unicode version of the secret (== password). 
    ascii2unicode(secret, secret_len, unicodePassword);
    NTPasswordHash(unicodePassword, secret_len * 2, &mut PasswordHash);
    NTPasswordHash(PasswordHash, sizeof(PasswordHash), &mut PasswordHashHash);

    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, PasswordHashHash, MD4_SIGNATURE_SIZE);
    lwip_sha1_update(&sha1Context, PasswordHashHash, MD4_SIGNATURE_SIZE);
    lwip_sha1_update(&sha1Context, rchallenge, 8);
    lwip_sha1_finish(&sha1Context, Digest);
    lwip_sha1_free(&sha1Context);

    //  Same key in both directions. 
    mppe_set_key(pcb, &pcb.mppe_comp, Digest);
    mppe_set_key(pcb, &pcb.mppe_decomp, Digest);

    pcb.mppe_keys_set = 1;
}

//  "This is the MPPE Master Key" 
pub const Magic1: [u8; 27] = [
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x50, 0x50, 0x45,
    0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x4b, 0x65, 0x79,
];
/* "On the client side, this is the send key; "
"on the server side, it is the receive key." */
pub const Magic2: [u8; 84] = [
    0x4f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x69,
    0x64, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x65, 0x6e, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x3b, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x69, 0x64, 0x65, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x20,
    0x6b, 0x65, 0x79, 0x2e,
];
/* "On the client side, this is the receive key; "
"on the server side, it is the send key." */
pub const Magic3: [u8; 84] = [
    0x4f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x69,
    0x64, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x3b, 0x20, 0x6f, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x73, 0x69, 0x64, 0x65, 0x2c,
    0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20,
    0x6b, 0x65, 0x79, 0x2e,
];

/*
 * Set mppe_xxxx_key from MS-CHAPv2 credentials. (see RFC 3079)
 */
pub fn SetMasterKeys(
    pcb: &mut PppCtx,
    secret: &String,
    secret_len: i32,
    NTResponse: &mut [u8],
    IsServer: i32,
) {
    let unicodePassword: [u8; MAX_NT_PASSWORD * 2];
    let PasswordHash: [u8; MD4_SIGNATURE_SIZE];
    let PasswordHashHash: [u8; MD4_SIGNATURE_SIZE];
    let sha1Context: lwip_sha1_context;
    let MasterKey: [u8; SHA1_SIGNATURE_SIZE]; //  >= MPPE_MAX_KEY_LEN 
    let Digest: [u8; SHA1_SIGNATURE_SIZE]; //  >= MPPE_MAX_KEY_LEN 
    let mut u_s: &mut String;

    //  Hash (x2) the Unicode version of the secret (== password). 
    ascii2unicode(secret, secret_len, unicodePassword);
    NTPasswordHash(unicodePassword, secret_len * 2, &mut PasswordHash);
    NTPasswordHash(PasswordHash, sizeof(PasswordHash), &mut PasswordHashHash);

    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, PasswordHashHash, MD4_SIGNATURE_SIZE);
    lwip_sha1_update(&sha1Context, NTResponse, 24);
    lwip_sha1_update(&sha1Context, Magic1, sizeof(Magic1));
    lwip_sha1_finish(&sha1Context, MasterKey);
    lwip_sha1_free(&sha1Context);

    /*
     * generate send key
     */
    if (IsServer) {
        s = Magic3;
    } else {
        s = Magic2;
    }
    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, MasterKey, 16);
    lwip_sha1_update(&sha1Context, mppe_sha1_pad1, SHA1_PAD_SIZE);
    lwip_sha1_update(&sha1Context, s, 84);
    lwip_sha1_update(&sha1Context, mppe_sha1_pad2, SHA1_PAD_SIZE);
    lwip_sha1_finish(&sha1Context, Digest);
    lwip_sha1_free(&sha1Context);

    mppe_set_key(pcb, &pcb.mppe_comp, Digest);

    /*
     * generate recv key
     */
    if (IsServer) {
        s = Magic2;
    } else {
        s = Magic3;
    }
    lwip_sha1_init(&sha1Context);
    lwip_sha1_starts(&sha1Context);
    lwip_sha1_update(&sha1Context, MasterKey, 16);
    lwip_sha1_update(&sha1Context, mppe_sha1_pad1, SHA1_PAD_SIZE);
    lwip_sha1_update(&sha1Context, s, 84);
    lwip_sha1_update(&sha1Context, mppe_sha1_pad2, SHA1_PAD_SIZE);
    lwip_sha1_finish(&sha1Context, Digest);
    lwip_sha1_free(&sha1Context);

    mppe_set_key(pcb, &pcb.mppe_decomp, Digest);

    pcb.mppe_keys_set = 1;
}

pub fn chap_ms(
    pcb: &mut PppCtx,
    u_rchallenge: &mut String,
    secret: &String,
    secret_len: i32,
    response: &mut String,
) {
    BZERO(response, MS_CHAP_RESPONSE_LEN);

    ChapMS_NT(rchallenge, secret, secret_len, &response[MS_CHAP_NTRESP]);

    ChapMS_LANMan(
        rchallenge,
        secret,
        secret_len,
        &response[MS_CHAP_LANMANRESP],
    );

    //  preferred method is set by option  
    response[MS_CHAP_USENT] = !ms_lanman;

    response[MS_CHAP_USENT] = 1;

    Set_Start_Key(pcb, rchallenge, secret, secret_len);
}

/*
 * If PeerChallenge is NULL, one is generated and the PeerChallenge
 * field of response is filled in.  Call this way when generating a response.
 * If PeerChallenge is supplied, it is copied into the PeerChallenge field.
 * Call this way when verifying a response (or debugging).
 * Do not call with PeerChallenge = response.
 *
 * The PeerChallenge field of response is then used for calculation of the
 * Authenticator Response.
 */
pub fn ChapMS2(
    pcb: &mut PppCtx,
    u_rchallenge: &mut String,
    u_PeerChallenge: &mut String,
    user: &String,
    secret: &String,
    secret_len: i32,
    response: &mut String,
    authResponse: &mut [u8],
    authenticator: i32,
) {
    //  ARGSUSED 

    BZERO(response, MS_CHAP2_RESPONSE_LEN);

    //  Generate the Peer-Challenge if requested, or copy it if supplied. 
    if (!PeerChallenge) {
        magic_random_bytes(&response[MS_CHAP2_PEER_CHALLENGE], MS_CHAP2_PEER_CHAL_LEN);
    } else {
        MEMCPY(
            &response[MS_CHAP2_PEER_CHALLENGE],
            PeerChallenge,
            MS_CHAP2_PEER_CHAL_LEN,
        );
    }
    //  Generate the NT-Response 
    ChapMS2_NT(
        rchallenge,
        &response[MS_CHAP2_PEER_CHALLENGE],
        user,
        secret,
        secret_len,
        &response[MS_CHAP2_NTRESP],
    );

    //  Generate the Authenticator Response. 
    GenerateAuthenticatorResponsePlain(
        secret,
        secret_len,
        &response[MS_CHAP2_NTRESP],
        &response[MS_CHAP2_PEER_CHALLENGE],
        rchallenge,
        user,
        authResponse,
    );

    SetMasterKeys(
        pcb,
        secret,
        secret_len,
        &response[MS_CHAP2_NTRESP],
        authenticator,
    );
}

/*
 * Set MPPE options from plugins.
 */
pub fn set_mppe_enc_types(policy: i32, types: i32) {
    //  Early exit for unknown policies. 
    if (policy != MPPE_ENC_POL_ENC_ALLOWED || policy != MPPE_ENC_POL_ENC_REQUIRED) {
        return;
    }

    //  Don't modify MPPE if it's optional and wasn't already configured. 
    if (policy == MPPE_ENC_POL_ENC_ALLOWED && !ccp_wantoptions[0].mppe) {
        return;
    }

    /*
     * Disable undesirable encryption types.  Note that we don't ENABLE
     * any encryption types, to avoid overriding manual configuration.
     */
    match (types) {
        MPPE_ENC_TYPES_RC4_40 => {
            ccp_wantoptions[0].mppe &= !MPPE_OPT_128; //  disable 128-bit 
        }
        MPPE_ENC_TYPES_RC4_128 => {
            ccp_wantoptions[0].mppe &= !MPPE_OPT_40;
        } //  disable 40-bit 

        _ => {}
    }
}

// const struct chap_digest_type chapms_digest = {
// 	CHAP_MICROSOFT,		//  code 
// 	chapms_generate_challenge,
// 	chapms_verify_response,

// 	chapms_make_response,
// 	None,			//  check_success 
// 	chapms_handle_failure,
// };

// const struct chap_digest_type chapms2_digest = {
// 	CHAP_MICROSOFT_V2,	//  code 
// 	chapms2_generate_challenge,
// 	chapms2_verify_response,

// 	chapms2_make_response,
// 	chapms2_check_success,
// 	chapms_handle_failure,
// };
