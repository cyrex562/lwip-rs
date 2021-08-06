/*
 * @file
 * Additional SNMPv3 functionality RFC3414 and RFC3826 (internal API, do not use in client code).
 */

/*
 * Copyright (c) 2016 Elias Oenal.
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
 */


// #define LWIP_HDR_APPS_SNMP_V3_PRIV_H








/* According to RFC 3411 */
#define SNMP_V3_MAX_ENGINE_ID_LENGTH  32
#define SNMP_V3_MAX_USER_LENGTH       32

#define SNMP_V3_MAX_AUTH_PARAM_LENGTH  12
#define SNMP_V3_MAX_PRIV_PARAM_LENGTH  8

#define SNMP_V3_MD5_LEN        16
#define SNMP_V3_SHA_LEN        20

typedef enum {
  SNMP_V3_PRIV_MODE_DECRYPT = 0,
  SNMP_V3_PRIV_MODE_ENCRYPT = 1
} snmpv3_priv_mode_t;

i32 snmpv3_get_engine_boots_internal();
pub fn  snmpv3_auth(stream: &mut snmp_pbuf_stream, length: u16,  u8 *key, snmpv3_auth_algo_t algo, u8 *hmac_out);
pub fn  snmpv3_crypt(stream: &mut snmp_pbuf_stream, length: u16,  u8 *key,
                   const u8 *priv_param,  engine_boots: u32,  engine_time: u32, snmpv3_priv_algo_t algo, snmpv3_priv_mode_t mode);
pub fn  snmpv3_build_priv_param(u8 *priv_param);
pub fn  snmpv3_enginetime_timer(arg: &mut Vec<u8>);




