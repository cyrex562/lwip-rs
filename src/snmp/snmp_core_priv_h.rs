/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
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
 * Author: Martin Hentschel <info@cl-soft.de>
 *
 */


// #define LWIP_HDR_APPS_SNMP_CORE_PRIV_H












/* (outdated) SNMPv1 error codes
 * shall not be used by MIBS anymore, nevertheless required from core for properly answering a v1 request
 */
pub const SNMP_ERR_NOSUCHNAME: u32 = 2; 
pub const SNMP_ERR_BADVALUE: u32 = 3; 
pub const SNMP_ERR_READONLY: u32 = 4; 
/* error codes which are internal and shall not be used by MIBS
 * shall not be used by MIBS anymore, nevertheless required from core for properly answering a v1 request
 */
pub const SNMP_ERR_TOOBIG: u32 = 1; 
pub const SNMP_ERR_AUTHORIZATIONERROR: u32 = 16; 

pub const SNMP_ERR_UNKNOWN_ENGINEID: u32 = 30; 
pub const SNMP_ERR_UNKNOWN_SECURITYNAME: u32 = 31; 
pub const SNMP_ERR_UNSUPPORTED_SECLEVEL: u32 = 32; 
pub const SNMP_ERR_NOTINTIMEWINDOW: u32 = 33; 
pub const SNMP_ERR_DECRYIPTION_ERROR: u32 = 34; 

#define SNMP_ERR_NOSUCHOBJECT         SNMP_VARBIND_EXCEPTION_OFFSET + SNMP_ASN1_CONTEXT_VARBIND_NO_SUCH_OBJECT
#define SNMP_ERR_ENDOFMIBVIEW         SNMP_VARBIND_EXCEPTION_OFFSET + SNMP_ASN1_CONTEXT_VARBIND_END_OF_MIB_VIEW


const snmp_mib_tree_resolve_exact: &mut snmp_node(const mib: &mut snmp_mib,  u32 *oid, oid_len: u8, oid_instance_len: &mut Vec<u8>);
const snmp_mib_tree_resolve_next: &mut snmp_node(const mib: &mut snmp_mib,  u32 *oid, oid_len: u8, oidret: &mut snmp_obj_id);

typedef u8 (*snmp_validate_node_instance_method)(struct snmp_node_instance *, void *);

snmp_get_node_instance_from_oid: u8(const u32 *oid, oid_len: u8, node_instance: &mut snmp_node_instance);
snmp_get_next_node_instance_from_oid: u8(const u32 *oid, oid_len: u8, snmp_validate_node_instance_method validate_node_instance_method, validate_node_instance_arg: &mut (), node_oid: &mut snmp_obj_id, node_instance: &mut snmp_node_instance);


}





