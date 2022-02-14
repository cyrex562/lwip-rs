/*
 * pppcrypt.c - PPP/DES linkage for MS-CHAP and EAP SRP-SHA1
 *
 * Extracted from chap_ms.c by James Carlson.
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




/* This header file is included in all PPP modules needing hashes and/or ciphers */




/*
 * If included PolarSSL copy is not used, user is expected to include
 * external libraries in arch/cc.h (which is included by lwip/arch.h).
 */





/*
 * Map hashes and ciphers functions to PolarSSL
 */



pub const lwip_md4_context: u32 = md4_context;
#define lwip_md4_init(context)
pub const lwip_md4_starts: u32 = md4_starts;
pub const lwip_md4_update: u32 = md4_update;
pub const lwip_md4_finish: u32 = md4_finish;
#define lwip_md4_free(context)


pub const lwip_md5_context: u32 = md5_context;
#define lwip_md5_init(context)
pub const lwip_md5_starts: u32 = md5_starts;
pub const lwip_md5_update: u32 = md5_update;
pub const lwip_md5_finish: u32 = md5_finish;
#define lwip_md5_free(context)


pub const lwip_sha1_context: u32 = sha1_context;
#define lwip_sha1_init(context)
pub const lwip_sha1_starts: u32 = sha1_starts;
pub const lwip_sha1_update: u32 = sha1_update;
pub const lwip_sha1_finish: u32 = sha1_finish;
#define lwip_sha1_free(context)


pub const lwip_des_context: u32 = des_context;
#define lwip_des_init(context)
pub const lwip_des_setkey_enc: u32 = des_setkey_enc;
pub const lwip_des_crypt_ecb: u32 = des_crypt_ecb;
#define lwip_des_free(context)


pub const lwip_arc4_context: u32 = arc4_context;
#define lwip_arc4_init(context)
pub const lwip_arc4_setup: u32 = arc4_setup;
pub const lwip_arc4_crypt: u32 = arc4_crypt;
#define lwip_arc4_free(context)

 /* !LWIP_USE_EXTERNAL_MBEDTLS */

/*
 * Map hashes and ciphers functions to mbed TLS
 */


pub const lwip_md4_context: u32 = mbedtls_md4_context;
pub const lwip_md4_init: u32 = mbedtls_md4_init;
pub const lwip_md4_starts: u32 = mbedtls_md4_starts;
pub const lwip_md4_update: u32 = mbedtls_md4_update;
pub const lwip_md4_finish: u32 = mbedtls_md4_finish;
pub const lwip_md4_free: u32 = mbedtls_md4_free;

pub const lwip_md5_context: u32 = mbedtls_md5_context;
pub const lwip_md5_init: u32 = mbedtls_md5_init;
pub const lwip_md5_starts: u32 = mbedtls_md5_starts;
pub const lwip_md5_update: u32 = mbedtls_md5_update;
pub const lwip_md5_finish: u32 = mbedtls_md5_finish;
pub const lwip_md5_free: u32 = mbedtls_md5_free;

pub const lwip_sha1_context: u32 = mbedtls_sha1_context;
pub const lwip_sha1_init: u32 = mbedtls_sha1_init;
pub const lwip_sha1_starts: u32 = mbedtls_sha1_starts;
pub const lwip_sha1_update: u32 = mbedtls_sha1_update;
pub const lwip_sha1_finish: u32 = mbedtls_sha1_finish;
pub const lwip_sha1_free: u32 = mbedtls_sha1_free;

pub const lwip_des_context: u32 = mbedtls_des_context;
pub const lwip_des_init: u32 = mbedtls_des_init;
pub const lwip_des_setkey_enc: u32 = mbedtls_des_setkey_enc;
pub const lwip_des_crypt_ecb: u32 = mbedtls_des_crypt_ecb;
pub const lwip_des_free: u32 = mbedtls_des_free;

pub const lwip_arc4_context: u32 = mbedtls_arc4_context;
pub const lwip_arc4_init: u32 = mbedtls_arc4_init;
pub const lwip_arc4_setup: u32 = mbedtls_arc4_setup;
#define lwip_arc4_crypt(context, buffer, length) mbedtls_arc4_crypt(context, length, buffer, buffer)
pub const lwip_arc4_free: u32 = mbedtls_arc4_free;

 /* LWIP_USE_EXTERNAL_MBEDTLS */

void pppcrypt_56_to_64_bit_key(u_char *key, u_char *des_key);


}


 /* PPPCRYPT_H */

 /* PPP_SUPPORT */
