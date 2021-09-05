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

// #define LWIP_HDR_APPS_SNMP_V3_H

pub enum snmpv3_auth_algo_t {
    SNMP_V3_AUTH_ALGO_INVAL = 0,
    SNMP_V3_AUTH_ALGO_MD5 = 1,
    SNMP_V3_AUTH_ALGO_SHA = 2,
}

pub enum snmpv3_priv_algo_t {
    SNMP_V3_PRIV_ALGO_INVAL = 0,
    SNMP_V3_PRIV_ALGO_DES = 1,
    SNMP_V3_PRIV_ALGO_AES = 2,
}

pub enum snmpv3_user_storagetype_t {
    SNMP_V3_USER_STORAGETYPE_OTHER = 1,
    SNMP_V3_USER_STORAGETYPE_VOLATILE = 2,
    SNMP_V3_USER_STORAGETYPE_NONVOLATILE = 3,
    SNMP_V3_USER_STORAGETYPE_PERMANENT = 4,
    SNMP_V3_USER_STORAGETYPE_READONLY = 5,
}

/*
 * The following callback functions must be implemented by the application.
 * There is a dummy implementation in snmpv3_dummy.c.
 */

// pub fn  snmpv3_get_engine_id( id: &mut String, len: &mut Vec<u8>);
// pub fn  snmpv3_set_engine_id( id: &mut String, len: u8);

// snmpv3_get_engine_boots: u32();
// pub fn  snmpv3_set_engine_boots(boots: u32);

// snmpv3_get_engine_time: u32();
// pub fn  snmpv3_reset_engine_time();

// pub fn  snmpv3_get_user( username: &mut String, snmpv3_auth_algo_t *auth_algo, auth_key: &mut Vec<u8>, snmpv3_priv_algo_t *priv_algo, priv_key: &mut Vec<u8>);
// snmpv3_get_amount_of_users: u8();
// pub fn  snmpv3_get_user_storagetype(username: &String, snmpv3_user_storagetype_t *storagetype);
// pub fn  snmpv3_get_username(username: &mut String, index: u8);

/* The following functions are provided by the SNMPv3 agent */

// pub fn  snmpv3_engine_id_changed();
// i32 snmpv3_get_engine_time_internal();

// pub fn  snmpv3_password_to_key_md5(
//  password: &mut Vec<u8>,     /* IN */
//     usize      passwordlen,  /* IN */
//  engineID: &mut Vec<u8>,     /* IN  - pointer to snmpEngineID  */
//     u8        engineLength, /* IN  - length of snmpEngineID */
//     u8       *key);         /* OUT - pointer to caller 16-octet buffer */
// pub fn  snmpv3_password_to_key_sha(
//  password: &mut Vec<u8>,     /* IN */
//     usize      passwordlen,  /* IN */
//  engineID: &mut Vec<u8>,     /* IN  - pointer to snmpEngineID  */
//     u8        engineLength, /* IN  - length of snmpEngineID */
//     u8       *key);         /* OUT - pointer to caller 20-octet buffer */
