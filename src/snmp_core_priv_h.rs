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




// #include "lwip/apps/snmp_opts.h"

#if LWIP_SNMP /* don't build if not configured for use in lwipopts.h */

// #include "lwip/apps/snmp_core.h"
// #include "snmp_asn1.h"




#if !defined LWIP_ASSERT_SNMP_LOCKED
#if SNMP_USE_RAW
#define LWIP_ASSERT_SNMP_LOCKED() LWIP_ASSERT_CORE_LOCKED()
#else
#define LWIP_ASSERT_SNMP_LOCKED()



/* (outdated) SNMPv1 error codes
 * shall not be used by MIBS anymore, nevertheless required from core for properly answering a v1 request
 */
pub const SNMP_ERR_NOSUCHNAME: u32 = 2; #define SNMP_ERR_BADVALUE   3
pub const SNMP_ERR_READONLY: u32 = 4; /* error codes which are internal and shall not be used by MIBS
 * shall not be used by MIBS anymore, nevertheless required from core for properly answering a v1 request
 */
pub const SNMP_ERR_TOOBIG: u32 = 1; #define SNMP_ERR_AUTHORIZATIONERROR   16

pub const SNMP_ERR_UNKNOWN_ENGINEID: u32 = 30; #define SNMP_ERR_UNKNOWN_SECURITYNAME 31
pub const SNMP_ERR_UNSUPPORTED_SECLEVEL: u32 = 32; #define SNMP_ERR_NOTINTIMEWINDOW      33
pub const SNMP_ERR_DECRYIPTION_ERROR: u32 = 34; #define SNMP_ERR_NOSUCHOBJECT         SNMP_VARBIND_EXCEPTION_OFFSET + SNMP_ASN1_CONTEXT_VARBIND_NO_SUCH_OBJECT
#define SNMP_ERR_ENDOFMIBVIEW         SNMP_VARBIND_EXCEPTION_OFFSET + SNMP_ASN1_CONTEXT_VARBIND_END_OF_MIB_VIEW


const struct snmp_node *snmp_mib_tree_resolve_exact(const struct snmp_mib *mib, const u32_t *oid, u8_t oid_len, u8_t *oid_instance_len);
const struct snmp_node *snmp_mib_tree_resolve_next(const struct snmp_mib *mib, const u32_t *oid, u8_t oid_len, struct snmp_obj_id *oidret);

typedef u8_t (*snmp_validate_node_instance_method)(struct snmp_node_instance *, void *);

u8_t snmp_get_node_instance_from_oid(const u32_t *oid, u8_t oid_len, struct snmp_node_instance *node_instance);
u8_t snmp_get_next_node_instance_from_oid(const u32_t *oid, u8_t oid_len, snmp_validate_node_instance_method validate_node_instance_method, void *validate_node_instance_arg, struct snmp_obj_id *node_oid, struct snmp_node_instance *node_instance);




 /* LWIP_SNMP */

 /* LWIP_HDR_APPS_SNMP_CORE_PRIV_H */
