use crate::snmp::snmp_core_h::snmp_obj_id;

/*
 * @file
 * SNMP server main API - start and basic configuration
 */

/*
 * Copyright (c) 2001, 2002 Leon Woestenberg <leon.woestenberg@axon.tv>
 * Copyright (c) 2001, 2002 Axon Digital Design B.V., The Netherlands.
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
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Leon Woestenberg <leon.woestenberg@axon.tv>
 *         Martin Hentschel <info@cl-soft.de>
 *
 */

// #define LWIP_HDR_APPS_SNMP_H

//  SNMP variable binding descriptor (publically needed for traps) 
pub struct snmp_varbind {
    //  pointer to next varbind, NULL for last in list 
    // pub mut next: &mut snmp_varbind,
    //  pointer to previous varbind, NULL for first in list 
    // pub mut prev: &mut snmp_varbind,

    //  object identifier 
    pub oid: snmp_obj_id,

    //  value ASN1 type 
    pub asn1_type: u8,
    //  object value length 
    pub value_len: u16,
    //  object value 
    pub value: Vec<u8>,
}

/*
 * @ingroup snmp_core
 * Agent setup, start listening to port 161.
 */
// pub fn  snmp_init();
// pub fn  snmp_set_mibs(mibs: &mut Vec<snmp_mib>, num_mibs: u8);

// pub fn  snmp_set_device_enterprise_oid( device_enterprise_oid: &mut snmp_obj_id);
// const snmp_get_device_enterprise_oid: &mut snmp_obj_id();

// pub fn  snmp_trap_dst_enable(dst_idx: u8, enable: u8);
// pub fn  snmp_trap_dst_ip_set(dst_idx: u8,  dst: &mut LwipAddr);

//  Generic trap: cold start 
pub const SNMP_GENTRAP_COLDSTART: u32 = 0;
//  Generic trap: warm start 
pub const SNMP_GENTRAP_WARMSTART: u32 = 1;
//  Generic trap: link down 
pub const SNMP_GENTRAP_LINKDOWN: u32 = 2;
//  Generic trap: link up 
pub const SNMP_GENTRAP_LINKUP: u32 = 3;
//  Generic trap: authentication failure 
pub const SNMP_GENTRAP_AUTH_FAILURE: u32 = 4;
//  Generic trap: EGP neighbor lost 
pub const SNMP_GENTRAP_EGP_NEIGHBOR_LOSS: u32 = 5;
//  Generic trap: enterprise specific 
pub const SNMP_GENTRAP_ENTERPRISE_SPECIFIC: u32 = 6;

// pub fn  snmp_send_trap_generic( generic_trap: i32);
// pub fn  snmp_send_trap_specific( specific_trap: i32, varbinds: &mut snmp_varbind);
// pub fn  snmp_send_trap( oid: &mut snmp_obj_id, generic_trap: i32, specific_trap: i32, varbinds: &mut snmp_varbind);

pub const SNMP_AUTH_TRAPS_DISABLED: u32 = 0;
pub const SNMP_AUTH_TRAPS_ENABLED: u32 = 1;
// pub fn  snmp_set_auth_traps_enabled(enable: u8);
// snmp_get_auth_traps_enabled: u8();

// snmp_v1_enabled: u8();
// snmp_v2c_enabled: u8();
// snmp_v3_enabled: u8();
// pub fn  snmp_v1_enable(enable: u8);
// pub fn  snmp_v2c_enable(enable: u8);
// pub fn  snmp_v3_enable(enable: u8);

// const char * snmp_get_community();
// const char * snmp_get_community_write();
// const char * snmp_get_community_trap();
// pub fn  snmp_set_community( char * const community);
// pub fn  snmp_set_community_write( char * const community);
// pub fn  snmp_set_community_trap( char * const community);

// pub fn  snmp_coldstart_trap();
// pub fn  snmp_authfail_trap();

// typedef void (*snmp_write_callback_fct)( oid: &mut u32, oid_len: u8, callback_arg: &mut Vec<u8>);
type snmp_write_callback_fct = fn(oid: &mut u32, oid_len: u8, callback_arg: &mut Vec<u8>);

// pub fn  snmp_set_write_callback(snmp_write_callback_fct write_callback, callback_arg: &mut Vec<u8>);
