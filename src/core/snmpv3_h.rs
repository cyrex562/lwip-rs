/*
 * @file
 * Additional SNMPv3 functionality RFC3414 and RFC3826.
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


#define LWIP_HDR_APPS_SNMP_V3_H





extern "C" {




typedef enum
{
  SNMP_V3_AUTH_ALGO_INVAL = 0,
  SNMP_V3_AUTH_ALGO_MD5   = 1,
  SNMP_V3_AUTH_ALGO_SHA   = 2
} snmpv3_auth_algo_t;

typedef enum
{
  SNMP_V3_PRIV_ALGO_INVAL = 0,
  SNMP_V3_PRIV_ALGO_DES   = 1,
  SNMP_V3_PRIV_ALGO_AES   = 2
} snmpv3_priv_algo_t;

typedef enum
{
  SNMP_V3_USER_STORAGETYPE_OTHER       = 1,
  SNMP_V3_USER_STORAGETYPE_VOLATILE    = 2,
  SNMP_V3_USER_STORAGETYPE_NONVOLATILE = 3,
  SNMP_V3_USER_STORAGETYPE_PERMANENT   = 4,
  SNMP_V3_USER_STORAGETYPE_READONLY    = 5
} snmpv3_user_storagetype_t;

/*
 * The following callback functions must be implemented by the application.
 * There is a dummy implementation in snmpv3_dummy.c.
 */

pub fn  snmpv3_get_engine_id(const char **id, u8 *len);
pub fn  snmpv3_set_engine_id(const char* id, len: u8);

u32 snmpv3_get_engine_boots(void);
pub fn  snmpv3_set_engine_boots(u32 boots);

u32 snmpv3_get_engine_time(void);
pub fn  snmpv3_reset_engine_time(void);

pub fn  snmpv3_get_user(const char* username, snmpv3_auth_algo_t *auth_algo, u8 *auth_key, snmpv3_priv_algo_t *priv_algo, u8 *priv_key);
snmpv3_get_amount_of_users: u8(void);
pub fn  snmpv3_get_user_storagetype(const char *username, snmpv3_user_storagetype_t *storagetype);
pub fn  snmpv3_get_username(char *username, index: u8);

/* The following functions are provided by the SNMPv3 agent */

pub fn  snmpv3_engine_id_changed(void);
i32 snmpv3_get_engine_time_internal(void);

pub fn  snmpv3_password_to_key_md5(
    const u8 *password,     /* IN */
    usize      passwordlen,  /* IN */
    const u8 *engineID,     /* IN  - pointer to snmpEngineID  */
    u8        engineLength, /* IN  - length of snmpEngineID */
    u8       *key);         /* OUT - pointer to caller 16-octet buffer */

pub fn  snmpv3_password_to_key_sha(
    const u8 *password,     /* IN */
    usize      passwordlen,  /* IN */
    const u8 *engineID,     /* IN  - pointer to snmpEngineID  */
    u8        engineLength, /* IN  - length of snmpEngineID */
    u8       *key);         /* OUT - pointer to caller 20-octet buffer */




}



