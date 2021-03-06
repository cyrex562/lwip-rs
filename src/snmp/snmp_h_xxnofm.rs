/**
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



// #include "lwip/apps/snmp_opts.h"




// #if LWIP_SNMP /* don't build if not configured for use in lwipopts.h */

// #include "lwip/err.h"
// #include "lwip/apps/snmp_core.h"

/** SNMP variable binding descriptor (publicly needed for traps) */
struct snmp_varbind
{
  /** pointer to next varbind, NULL for last in list */
  struct snmp_varbind *next;
  /** pointer to previous varbind, NULL for first in list */
  struct snmp_varbind *prev;

  /** object identifier */
  struct snmp_obj_id oid;

  /** value ASN1 type */
  type: u8;
  /** object value length */
  value_len: u16;
  /** object value */
  void *value;
};

/**
 * @ingroup snmp_core
 * Agent setup, start listening to port 161.
 */
void snmp_init();
void snmp_set_mibs(const struct snmp_mib **mibs, u8_t num_mibs);

void snmp_set_device_enterprise_oid(const struct snmp_obj_id* device_enterprise_oid);
const struct snmp_obj_id* snmp_get_device_enterprise_oid();

void snmp_trap_dst_enable(u8_t dst_idx, u8_t enable);
void snmp_trap_dst_ip_set(u8_t dst_idx, const ip_addr_t *dst);

/** Generic trap: cold start */
pub const SNMP_GENTRAP_COLDSTART: u32 = 0; /** Generic trap: warm start */
pub const SNMP_GENTRAP_WARMSTART: u32 = 1; /** Generic trap: link down */
pub const SNMP_GENTRAP_LINKDOWN: u32 = 2; /** Generic trap: link up */
pub const SNMP_GENTRAP_LINKUP: u32 = 3; /** Generic trap: authentication failure */
pub const SNMP_GENTRAP_AUTH_FAILURE: u32 = 4; /** Generic trap: EGP neighbor lost */
pub const SNMP_GENTRAP_EGP_NEIGHBOR_LOSS: u32 = 5; /** Generic trap: enterprise specific */
pub const SNMP_GENTRAP_ENTERPRISE_SPECIFIC: u32 = 6; err_t snmp_send_trap_generic(s32_t generic_trap);
err_t snmp_send_trap_specific(s32_t specific_trap, struct snmp_varbind *varbinds);
err_t snmp_send_trap(const struct snmp_obj_id* oid, s32_t generic_trap, s32_t specific_trap, struct snmp_varbind *varbinds);

err_t snmp_send_inform_generic(s32_t generic_trap, struct snmp_varbind *varbinds, s32_t *ptr_request_id);
err_t snmp_send_inform_specific(s32_t specific_trap, struct snmp_varbind *varbinds, s32_t *ptr_request_id);
err_t snmp_send_inform(const struct snmp_obj_id* oid, s32_t generic_trap, s32_t specific_trap, struct snmp_varbind *varbinds, s32_t *ptr_request_id);
struct snmp_request;
typedef void (*snmp_inform_callback_fct)(struct snmp_request *request, void* callback_arg);
void snmp_set_inform_callback(snmp_inform_callback_fct inform_callback, void* callback_arg);

void snmp_set_default_trap_version(u8_t snmp_version);
u8_t snmp_get_default_trap_version();

pub const SNMP_AUTH_TRAPS_DISABLED: u32 = 0; #define SNMP_AUTH_TRAPS_ENABLED  1
void snmp_set_auth_traps_enabled(u8_t enable);
u8_t snmp_get_auth_traps_enabled();

u8_t snmp_v1_enabled();
u8_t snmp_v2c_enabled();
u8_t snmp_v3_enabled();
void snmp_v1_enable(u8_t enable);
void snmp_v2c_enable(u8_t enable);
void snmp_v3_enable(u8_t enable);

const char * snmp_get_community();
const char * snmp_get_community_write();
const char * snmp_get_community_trap();
void snmp_set_community(const char * const community);
void snmp_set_community_write(const char * const community);
void snmp_set_community_trap(const char * const community);

void snmp_coldstart_trap();
void snmp_authfail_trap();

typedef void (*snmp_write_callback_fct)(const u32_t* oid, u8_t oid_len, void* callback_arg);
void snmp_set_write_callback(snmp_write_callback_fct write_callback, void* callback_arg);

 /* LWIP_SNMP */




 /* LWIP_HDR_APPS_SNMP_H */
