/**
 * @file
 * SNMP Agent message handling structures (internal API, do not use in client code).
 */

/*
 * Copyright (c) 2006 Axon Digital Design B.V., The Netherlands.
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
 * Author: Christiaan Simons <christiaan.simons@axon.tv>
 *         Martin Hentschel <info@cl-soft.de>
 *         Elias Oenal <lwip@eliasoenal.com>
 */


#define LWIP_HDR_APPS_SNMP_MSG_H

















extern "C" {


/* version defines used in PDU */
pub const SNMP_VERSION_1: u32 = 0;
#define SNMP_VERSION_2c 1
#define SNMP_VERSION_3  3

struct snmp_varbind_enumerator {
  struct snmp_pbuf_stream pbuf_stream;
  varbind_count: u16;
};

typedef enum {
  SNMP_VB_ENUMERATOR_ERR_OK            = 0,
  SNMP_VB_ENUMERATOR_ERR_EOVB          = 1,
  SNMP_VB_ENUMERATOR_ERR_ASN1ERROR     = 2,
  SNMP_VB_ENUMERATOR_ERR_INVALIDLENGTH = 3
} snmp_vb_enumerator_err_t;

pub fn  snmp_vb_enumerator_init(enumerator: &mut snmp_varbind_enumerator, p: &mut pbuf, offset: u16, length: u16);
snmp_vb_enumerator_err_t snmp_vb_enumerator_get_next(enumerator: &mut snmp_varbind_enumerator, varbind: &mut snmp_varbind);

struct snmp_request {
  /* Communication handle */
  void *handle;
  /* source IP address */
  const source_ip: &mut ip_addr_t;
  /* source UDP port */
  source_port: u16;
  /* incoming snmp version */
  version: u8;
  /* community name (zero terminated) */
  community: u8[SNMP_MAX_COMMUNITY_STR_LEN + 1];
  /* community string length (exclusive zero term) */
  community_strlen: u16;
  /* request type */
  request_type: u8;
  /* request ID */
  request_id: i32;
  /* error status */
  error_status: i32;
  /* error index */
  error_index: i32;
  /* non-repeaters (getBulkRequest (SNMPv2c)) */
  non_repeaters: i32;
  /* max-repetitions (getBulkRequest (SNMPv2c)) */
  max_repetitions: i32;

  /* Usually response-pdu (2). When snmpv3 errors are detected report-pdu(8) */
  request_out_type: u8;


  msg_id: i32;
  msg_max_size: i32;
  u8  msg_flags;
  msg_security_model: i32;
  u8  msg_authoritative_engine_id[SNMP_V3_MAX_ENGINE_ID_LENGTH];
  u8  msg_authoritative_engine_id_len;
  msg_authoritative_engine_boots: i32;
  msg_authoritative_engine_time: i32;
  u8  msg_user_name[SNMP_V3_MAX_USER_LENGTH];
  u8  msg_user_name_len;
  u8  msg_authentication_parameters[SNMP_V3_MAX_AUTH_PARAM_LENGTH];
  u8  msg_authentication_parameters_len;
  u8  msg_privacy_parameters[SNMP_V3_MAX_PRIV_PARAM_LENGTH];
  u8  msg_privacy_parameters_len;
  u8  context_engine_id[SNMP_V3_MAX_ENGINE_ID_LENGTH];
  u8  context_engine_id_len;
  u8  context_name[SNMP_V3_MAX_ENGINE_ID_LENGTH];
  u8  context_name_len;


  inbound_pbuf: &mut pbuf;
  struct snmp_varbind_enumerator inbound_varbind_enumerator;
  inbound_varbind_offset: u16;
  inbound_varbind_len: u16;
  inbound_padding_len: u16;

  outbound_pbuf: &mut pbuf;
  struct snmp_pbuf_stream outbound_pbuf_stream;
  outbound_pdu_offset: u16;
  outbound_error_status_offset: u16;
  outbound_error_index_offset: u16;
  outbound_varbind_offset: u16;

  outbound_msg_global_data_offset: u16;
  outbound_msg_global_data_end: u16;
  outbound_msg_security_parameters_str_offset: u16;
  outbound_msg_security_parameters_seq_offset: u16;
  outbound_msg_security_parameters_end: u16;
  outbound_msg_authentication_parameters_offset: u16;
  outbound_scoped_pdu_seq_offset: u16;
  outbound_scoped_pdu_string_offset: u16;


  value_buffer: u8[SNMP_MAX_VALUE_SIZE];
};

/** A helper struct keeping length information about varbinds */
struct snmp_varbind_len {
  u8  vb_len_len;
  vb_value_len: u16;
  u8  oid_len_len;
  oid_value_len: u16;
  u8  value_len_len;
  value_value_len: u16;
};

/** Agent community string */
extern snmp_community: String;
/** Agent community string for write access */
extern snmp_community_write: String;
/** handle for sending traps */
extern void *snmp_traps_handle;

pub fn  snmp_receive(void *handle, p: &mut pbuf, const source_ip: &mut ip_addr_t, port: u16);
pub fn  snmp_sendto(void *handle, p: &mut pbuf, const dst: &mut ip_addr_t, port: u16);
snmp_get_local_ip_for_dst: u8(void *handle, const dst: &mut ip_addr_t, result: &mut ip_addr_t);
pub fn  snmp_varbind_length(varbind: &mut snmp_varbind, len: &mut snmp_varbind_len);
pub fn  snmp_append_outbound_varbind(pbuf_stream: &mut snmp_pbuf_stream, varbind: &mut snmp_varbind);


}





