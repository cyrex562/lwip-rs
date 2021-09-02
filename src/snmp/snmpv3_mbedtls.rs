/*
 * @file
 * SNMPv3 crypto/auth functions implemented for ARM mbedtls.
 */

/*
 * Copyright (c) 2016 Elias Oenal and Dirk Ziegelmeier.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * Author: Elias Oenal <lwip@eliasoenal.com>
 *         Dirk Ziegelmeier <dirk@ziegelmeier.net>
 */
















pub fn 
snmpv3_auth(stream: &mut snmp_pbuf_stream, length: u16,
 key: &mut Vec<u8>, snmpv3_auth_algo_t algo, hmac_out: &mut Vec<u8>)
{
  let i: u32;
  let key_len: u8;
 mbedtls_md_info_t *md_info;
  mbedtls_md_context_t ctx;
  let read_stream: snmp_pbuf_stream;
  snmp_pbuf_stream_init(&read_stream, stream.pbuf, stream.offset, stream.length);

  if (algo == SNMP_V3_AUTH_ALGO_MD5) {
    md_info = mbedtls_md_info_from_type(MBEDTLS_MD_MD5);
    key_len = SNMP_V3_MD5_LEN;
  } else if (algo == SNMP_V3_AUTH_ALGO_SHA) {
    md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA1);
    key_len = SNMP_V3_SHA_LEN;
  } else {
    return ERR_ARG;
  }

  mbedtls_md_init(&ctx);
  if (mbedtls_md_setup(&ctx, md_info, 1) != 0) {
    return ERR_ARG;
  }

  if (mbedtls_md_hmac_starts(&ctx, key, key_len) != 0) {
    // goto free_md;
  }

  for (i = 0; i < length; i+= 1) {
    let byte: u8;

    if (snmp_pbuf_stream_read(&read_stream, &byte)) {
      // goto free_md;
    }

    if (mbedtls_md_hmac_update(&ctx, &byte, 1) != 0) {
      // goto free_md;
    }
  }

  if (mbedtls_md_hmac_finish(&ctx, hmac_out) != 0) {
    // goto free_md;
  }

  mbedtls_md_free(&ctx);
 return Ok(());

// free_md:
  mbedtls_md_free(&ctx);
  return ERR_ARG;
}



pub fn 
snmpv3_crypt(stream: &mut snmp_pbuf_stream, length: u16,
 key: &mut Vec<u8>,  priv_param: &mut Vec<u8>,  engine_boots: u32,
 engine_time: u32, snmpv3_priv_algo_t algo, snmpv3_priv_mode_t mode)
{
  let i: usize;
  mbedtls_cipher_context_t ctx;
 mbedtls_cipher_info_t *cipher_info;

  let read_stream: snmp_pbuf_stream;
  let write_stream: snmp_pbuf_stream;
  snmp_pbuf_stream_init(&read_stream, stream.pbuf, stream.offset, stream.length);
  snmp_pbuf_stream_init(&write_stream, stream.pbuf, stream.offset, stream.length);
  mbedtls_cipher_init(&ctx);

  if (algo == SNMP_V3_PRIV_ALGO_DES) {
    iv_local: [u8;8];
    out_bytes: [u8;8];
    let out_len: usize;

    /* RFC 3414 mandates padding for DES */
    if ((length & 0x07) != 0) {
      return ERR_ARG;
    }

    cipher_info = mbedtls_cipher_info_from_type(MBEDTLS_CIPHER_DES_CBC);
    if (mbedtls_cipher_setup(&ctx, cipher_info) != 0) {
      return ERR_ARG;
    }
    if (mbedtls_cipher_set_padding_mode(&ctx, MBEDTLS_PADDING_NONE) != 0) {
      return ERR_ARG;
    }
    if (mbedtls_cipher_setkey(&ctx, key, 8 * 8, (mode == SNMP_V3_PRIV_MODE_ENCRYPT) ? MBEDTLS_ENCRYPT : MBEDTLS_DECRYPT) != 0) {
      // goto error;
    }

    /* Prepare IV */
    for (i = 0; i < LWIP_ARRAYSIZE(iv_local); i+= 1) {
      iv_local[i] = priv_param[i] ^ key[i + 8];
    }
    if (mbedtls_cipher_set_iv(&ctx, iv_local, LWIP_ARRAYSIZE(iv_local)) != 0) {
      // goto error;
    }

    for (i = 0; i < length; i += 8) {
      let j: usize;
      in_bytes: [u8;8];
      out_len = LWIP_ARRAYSIZE(out_bytes) ;

      for (j = 0; j < LWIP_ARRAYSIZE(in_bytes); j+= 1) {
        if (snmp_pbuf_stream_read(&read_stream, &in_bytes[j]) != ERR_OK) {
          // goto error;
        }
      }

      if (mbedtls_cipher_update(&ctx, in_bytes, LWIP_ARRAYSIZE(in_bytes), out_bytes, &out_len) != 0) {
        // goto error;
      }

      if (snmp_pbuf_stream_writebuf(&write_stream, out_bytes, out_len) != ERR_OK) {
        // goto error;
      }
    }

    out_len = LWIP_ARRAYSIZE(out_bytes);
    if (mbedtls_cipher_finish(&ctx, out_bytes, &out_len) != 0) {
      // goto error;
    }

    if (snmp_pbuf_stream_writebuf(&write_stream, out_bytes, out_len) != ERR_OK) {
      // goto error;
    }
  } else if (algo == SNMP_V3_PRIV_ALGO_AES) {
    iv_local: [u8;16];

    cipher_info = mbedtls_cipher_info_from_type(MBEDTLS_CIPHER_AES_128_CFB128);
    if (mbedtls_cipher_setup(&ctx, cipher_info) != 0) {
      return ERR_ARG;
    }
    if (mbedtls_cipher_setkey(&ctx, key, 16 * 8, (mode == SNMP_V3_PRIV_MODE_ENCRYPT) ? MBEDTLS_ENCRYPT : MBEDTLS_DECRYPT) != 0) {
      // goto error;
    }

    /*
     * IV is the big endian concatenation of boots,
     * uptime and priv param - see RFC3826.
     */
    iv_local[0 + 0] = (engine_boots >> 24) & 0xFF;
    iv_local[0 + 1] = (engine_boots >> 16) & 0xFF;
    iv_local[0 + 2] = (engine_boots >>  8) & 0xFF;
    iv_local[0 + 3] = (engine_boots >>  0) & 0xFF;
    iv_local[4 + 0] = (engine_time  >> 24) & 0xFF;
    iv_local[4 + 1] = (engine_time  >> 16) & 0xFF;
    iv_local[4 + 2] = (engine_time  >>  8) & 0xFF;
    iv_local[4 + 3] = (engine_time  >>  0) & 0xFF;
    SMEMCPY(iv_local + 8, priv_param, 8);
    if (mbedtls_cipher_set_iv(&ctx, iv_local, LWIP_ARRAYSIZE(iv_local)) != 0) {
      // goto error;
    }

    for (i = 0; i < length; i+= 1) {
      let in_byte: u8;
      let out_byte: u8;
      out_len: usize = sizeof(out_byte);

      if (snmp_pbuf_stream_read(&read_stream, &in_byte) != ERR_OK) {
        // goto error;
      }
      if (mbedtls_cipher_update(&ctx, &in_byte, sizeof(in_byte), &out_byte, &out_len) != 0) {
        // goto error;
      }
      if (snmp_pbuf_stream_write(&write_stream, out_byte) != ERR_OK) {
        // goto error;
      }
    }
  } else {
    return ERR_ARG;
  }

  mbedtls_cipher_free(&ctx);
 return Ok(());

// error:
  mbedtls_cipher_free(&ctx);
 return Ok(());
}



/* A.2.1. Password to Key Sample Code for MD5 */
pub fn 
snmpv3_password_to_key_md5(
 password: &mut Vec<u8>,    /* IN */
  usize      passwordlen, /* IN */
 engineID: &mut Vec<u8>,    /* IN  - pointer to snmpEngineID  */
  u8        engineLength,/* IN  - length of snmpEngineID */
  u8       *key)         /* OUT - pointer to caller 16-octet buffer */
{
  mbedtls_md5_context MD;
  cp: &mut Vec<u8>, password_buf[64];
  password_index: u32 = 0;
  let i: u8;
  count: u32 = 0;

  mbedtls_md5_init(&MD); /* initialize MD5 */
  mbedtls_md5_starts(&MD);

  /*********************************************/
  /* Use while loop until we've done 1 Megabyte */
  /*********************************************/
  while (count < 1048576) {
    cp = password_buf;
    for (i = 0; i < 64; i+= 1) {
      /************************************************/
      /* Take the next octet of the password, wrapping */
      /* to the beginning of the password as necessary.*/
      /************************************************/
      *cp+= 1 = password[password_index+= 1 % passwordlen];
    }
    mbedtls_md5_update(&MD, password_buf, 64);
    count += 64;
  }
  mbedtls_md5_finish(&MD, key); /* tell MD5 we're done */

  /****************************************************/
  /* Now localize the key with the engineID and pass   */
  /* through MD5 to produce final key                  */
  /* May want to ensure that engineLength <= 32,       */
  /* otherwise need to use a buffer larger than 64     */
  /****************************************************/
  SMEMCPY(password_buf, key, 16);
  MEMCPY(password_buf + 16, engineID, engineLength);
  SMEMCPY(password_buf + 16 + engineLength, key, 16);

  mbedtls_md5_starts(&MD);
  mbedtls_md5_update(&MD, password_buf, 32 + engineLength);
  mbedtls_md5_finish(&MD, key);

  mbedtls_md5_free(&MD);
  return;
}

/* A.2.2. Password to Key Sample Code for SHA */
pub fn 
snmpv3_password_to_key_sha(
 password: &mut Vec<u8>,    /* IN */
  usize      passwordlen, /* IN */
 engineID: &mut Vec<u8>,    /* IN  - pointer to snmpEngineID  */
  u8        engineLength,/* IN  - length of snmpEngineID */
  u8       *key)         /* OUT - pointer to caller 20-octet buffer */
{
  mbedtls_sha1_context SH;
  cp: &mut Vec<u8>, password_buf[72];
  password_index: u32 = 0;
  let i: u8;
  count: u32 = 0;

  mbedtls_sha1_init(&SH); /* initialize SHA */
  mbedtls_sha1_starts(&SH);

  /*********************************************/
  /* Use while loop until we've done 1 Megabyte */
  /*********************************************/
  while (count < 1048576) {
    cp = password_buf;
    for (i = 0; i < 64; i+= 1) {
      /************************************************/
      /* Take the next octet of the password, wrapping */
      /* to the beginning of the password as necessary.*/
      /************************************************/
      *cp+= 1 = password[password_index+= 1 % passwordlen];
    }
    mbedtls_sha1_update(&SH, password_buf, 64);
    count += 64;
  }
  mbedtls_sha1_finish(&SH, key); /* tell SHA we're done */

  /****************************************************/
  /* Now localize the key with the engineID and pass   */
  /* through SHA to produce final key                  */
  /* May want to ensure that engineLength <= 32,       */
  /* otherwise need to use a buffer larger than 72     */
  /****************************************************/
  SMEMCPY(password_buf, key, 20);
  MEMCPY(password_buf + 20, engineID, engineLength);
  SMEMCPY(password_buf + 20 + engineLength, key, 20);

  mbedtls_sha1_starts(&SH);
  mbedtls_sha1_update(&SH, password_buf, 40 + engineLength);
  mbedtls_sha1_finish(&SH, key);

  mbedtls_sha1_free(&SH);
  return;
}


