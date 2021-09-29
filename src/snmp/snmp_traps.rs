use crate::snmp::snmp_opts_h::SNMP_TRAP_DESTINATIONS;

/*
 * @file
 * SNMPv1 traps implementation.
 */

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
 * Author: Martin Hentschel
 *         Christiaan Simons <christiaan.simons@axon.tv>
 *
 */
















pub struct snmp_msg_trap {
  //  source enterprise ID (sysObjectID) 
 pub enterprise: snmp_obj_id,
  //  source IP address, raw network order format 
  pub sip: LwipAddr,
  //  generic trap code 
  pub gen_trap: u32,
  //  specific trap code 
  pub spc_trap: u32,
  //  timestamp 
  pub ts: u32,
  //  snmp_version 
  pub snmp_version: u32,

  //  output trap lengths used in ASN encoding 
  //  encoding pdu length 
  pub pdulen: u16,
  //  encoding community length 
  pub comlen: u16,
  //  encoding sequence length 
  pub seqlen: u16,
  //  encoding varbinds sequence length 
  pub vbseqlen: u16,
}

// static snmp_trap_varbind_sum: u16(trap: &mut snmp_msg_trap, varbinds: &mut snmp_varbind);
// static snmp_trap_header_sum: u16(trap: &mut snmp_msg_trap, vb_len: u16);
// pub fn snmp_trap_header_enc(trap: &mut snmp_msg_trap, pbuf_stream: &mut snmp_pbuf_stream) -> Result<(), LwipError>;pub fn snmp_trap_header_enc(trap: &mut snmp_msg_trap, pbuf_stream: &mut snmp_pbuf_stream) -> Result<(), LwipError>
// static snmp_trap_varbind_enc: err_t(trap: &mut snmp_msg_trap, pbuf_stream: &mut snmp_pbuf_stream, varbinds: &mut snmp_varbind);

// #define BUILD_EXEC(code) \
//   if ((code) != ERR_OK) { \
// //    LWIP_DEBUGF(SNMP_DEBUG, ("SNMP error during creation of outbound trap frame!")); \
//     return ERR_ARG; \
//   }

// //  Agent community string for sending traps 
// extern snmp_community_trap: String;

// pub fn  *snmp_traps_handle;

pub struct snmp_trap_dst {
  //  destination IP address in network order 
  pub dip: LwipAddr,
  //  set to 0 when disabled, >0 when enabled 
  pub enable: u8,
}
// static struct snmp_trap_dst trap_dst[SNMP_TRAP_DESTINATIONS];

// static snmp_auth_traps_enabled: u8 = 0;

/*
 * @ingroup snmp_traps
 * Sets enable match for this trap destination.
 * @param dst_idx index in 0 .. SNMP_TRAP_DESTINATIONS-1
 * @param enable match if 0 destination is disabled >0 enabled.
 */
pub fn 
snmp_trap_dst_enable(dst_idx: u8, enable: u8)
{
  LWIP_ASSERT_CORE_LOCKED();
  if (dst_idx < SNMP_TRAP_DESTINATIONS) {
    trap_dst[dst_idx].enable = enable;
  }
}

/*
 * @ingroup snmp_traps
 * Sets IPv4 address for this trap destination.
 * @param dst_idx index in 0 .. SNMP_TRAP_DESTINATIONS-1
 * @param dst IPv4 address in host order.
 */
pub fn 
snmp_trap_dst_ip_set(dst_idx: u8,  dst: &mut LwipAddr)
{
  LWIP_ASSERT_CORE_LOCKED();
  if (dst_idx < SNMP_TRAP_DESTINATIONS) {
    ip_addr_set(&trap_dst[dst_idx].dip, dst);
  }
}

/*
 * @ingroup snmp_traps
 * Enable/disable authentication traps
 */
pub fn 
snmp_set_auth_traps_enabled(enable: u8)
{
  snmp_auth_traps_enabled = enable;
}

/*
 * @ingroup snmp_traps
 * Get authentication traps enabled state
 */
pub fn snmp_get_auth_traps_enabled()
{
  return snmp_auth_traps_enabled;
}


/*
 * @ingroup snmp_traps
 * Sends a generic or enterprise specific trap message.
 *
 * @param eoid points to enterprise object identifier
 * @param generic_trap is the trap code
 * @param specific_trap used for enterprise traps when generic_trap == 6
 * @param varbinds linked list of varbinds to be sent
 * @return ERR_OK when success, ERR_MEM if we're out of memory
 *
 * @note the use of the enterprise identifier field
 * is per RFC1215.
 * Use .iso.org.dod.internet.mgmt.mib-2.snmp for generic traps
 * and .iso.org.dod.internet.private.enterprises.yourenterprise
 * (sysObjectID) for specific traps.
 */
pub fn 
snmp_send_trap( eoid: &mut snmp_obj_id, generic_trap: i32, specific_trap: i32, varbinds: &mut snmp_varbind)
{
  let trap_msg: snmp_msg_trap;
  let mut td: &mut snmp_trap_dst;
  let p: &mut PacketBuffer;
  let i: u16; let tot_len: u16;
  let err: err_t = ERR_OK;

  LWIP_ASSERT_CORE_LOCKED();

  trap_msg.snmp_version = 0;

  td = &trap_dst[0];
  for i in 0 .. SNMP_TRAP_DESTINATIONS {

  // for (i = 0, td = &trap_dst[0]; i < SNMP_TRAP_DESTINATIONS; i+= 1, td+= 1) {
    if ((td.enable != 0) && !ip_addr_isany(&td.dip)) {
      //  lookup current source address for this dst 
      if (snmp_get_local_ip_for_dst(snmp_traps_handle, &td.dip, &trap_msg.sip)) {
        if (eoid == None) {
          trap_msg.enterprise = snmp_get_device_enterprise_oid();
        } else {
          trap_msg.enterprise = eoid;
        }

        trap_msg.gen_trap = generic_trap;
        if (generic_trap == SNMP_GENTRAP_ENTERPRISE_SPECIFIC) {
          trap_msg.spc_trap = specific_trap;
        } else {
          trap_msg.spc_trap = 0;
        }

        MIB2_COPY_SYSUPTIME_TO(&trap_msg.ts);

        //  pass 0, calculate length fields 
        tot_len = snmp_trap_varbind_sum(&trap_msg, varbinds);
        tot_len = snmp_trap_header_sum(&trap_msg, tot_len);

        //  allocate pbuf(s) 
        p = pbuf_alloc(PBUF_TRANSPORT, tot_len, PBUF_RAM);
        if (p != None) {
          let pbuf_stream: snmp_pbuf_stream;
          snmp_pbuf_stream_init(&pbuf_stream, p, 0, tot_len);

          //  pass 1, encode packet into the pbuf(s) 
          snmp_trap_header_enc(&trap_msg, &pbuf_stream);
          snmp_trap_varbind_enc(&trap_msg, &pbuf_stream, varbinds);

          snmp_stats.outtraps+= 1;
          snmp_stats.outpkts+= 1;

          //  send to the TRAP destination 
          snmp_sendto(snmp_traps_handle, p, &td.dip, LWIP_IANA_PORT_SNMP_TRAP);
          pbuf_free(p);
        } else {
          err = ERR_MEM;
        }
      } else {
        //  routing error 
        err = ERR_RTE;
      }
    }
    td += 1;
  }
  return err;
}

/*
 * @ingroup snmp_traps
 * Send generic SNMP trap
 */
pub fn 
snmp_send_trap_generic( generic_trap: i32)
{
  let oid: snmp_obj_id = snmp_obj_id::new(7, [ 1, 3, 6, 1, 2, 1, 11 ]);
  return snmp_send_trap(&oid, generic_trap, 0, None);
}

/*
 * @ingroup snmp_traps
 * Send specific SNMP trap with variable bindings
 */
pub fn 
snmp_send_trap_specific( specific_trap: i32, varbinds: &mut snmp_varbind)
{
  return snmp_send_trap(None, SNMP_GENTRAP_ENTERPRISE_SPECIFIC, specific_trap, varbinds);
}

/*
 * @ingroup snmp_traps
 * Send coldstart trap
 */
pub fn 
snmp_coldstart_trap()
{
  snmp_send_trap_generic(SNMP_GENTRAP_COLDSTART);
}

/*
 * @ingroup snmp_traps
 * Send authentication failure trap (used internally by agent)
 */
pub fn 
snmp_authfail_trap()
{
  if (snmp_auth_traps_enabled != 0) {
    snmp_send_trap_generic(SNMP_GENTRAP_AUTH_FAILURE);
  }
}

pub fn snmp_trap_varbind_sum(trap: &mut snmp_msg_trap, varbinds: &mut snmp_varbind)
{
  let mut varbind: &mut snmp_varbind;
  let tot_len: u16;
  let tot_len_len: u8;

  tot_len = 0;
  varbind = varbinds;
  while (varbind != None) {
    let len: snmp_varbind_len;

    if (snmp_varbind_length(varbind, &len) == ERR_OK) {
      tot_len += 1 + len.vb_len_len + len.vb_value_len;
    }

    varbind = varbind.next;
  }

  trap.vbseqlen = tot_len;
  snmp_asn1_enc_length_cnt(trap.vbseqlen, &tot_len_len);
  tot_len += 1 + tot_len_len;

  return tot_len;
}

/*
 * Sums trap header field lengths from tail to head and
 * returns trap_header_lengths for second encoding pass.
 *
 * @param trap Trap message
 * @param vb_len varbind-list length
 * @return the required length for encoding the trap header
 */
pub fn snmp_trap_header_sum(trap: &mut snmp_msg_trap, vb_len: u16)
{
  let tot_len: u16;
  let len: usize;
  let lenlen: u8;

  tot_len = vb_len;

  snmp_asn1_enc_u32t_cnt(trap.ts, &len);
  snmp_asn1_enc_length_cnt(len, &lenlen);
  tot_len += 1 + len + lenlen;

  snmp_asn1_enc_s32t_cnt(trap.spc_trap, &len);
  snmp_asn1_enc_length_cnt(len, &lenlen);
  tot_len += 1 + len + lenlen;

  snmp_asn1_enc_s32t_cnt(trap.gen_trap, &len);
  snmp_asn1_enc_length_cnt(len, &lenlen);
  tot_len += 1 + len + lenlen;

  if (IP_IS_V6_VAL(trap.sip)) {

    len = sizeof(ip_2_ip6(&trap.sip).addr);

  } else {

    len = sizeof(ip_2_ip4(&trap.sip).addr);

  }
  snmp_asn1_enc_length_cnt(len, &lenlen);
  tot_len += 1 + len + lenlen;

  snmp_asn1_enc_oid_cnt(trap.enterprise.id, trap.enterprise.len, &len);
  snmp_asn1_enc_length_cnt(len, &lenlen);
  tot_len += 1 + len + lenlen;

  trap.pdulen = tot_len;
  snmp_asn1_enc_length_cnt(trap.pdulen, &lenlen);
  tot_len += 1 + lenlen;

  trap.comlen = LWIP_MIN(strlen(snmp_community_trap), 0xFFFF);
  snmp_asn1_enc_length_cnt(trap.comlen, &lenlen);
  tot_len += 1 + lenlen + trap.comlen;

  snmp_asn1_enc_s32t_cnt(trap.snmp_version, &len);
  snmp_asn1_enc_length_cnt(len, &lenlen);
  tot_len += 1 + len + lenlen;

  trap.seqlen = tot_len;
  snmp_asn1_enc_length_cnt(trap.seqlen, &lenlen);
  tot_len += 1 + lenlen;

  return tot_len;
}

pub fn snmp_trap_varbind_enc(trap: &mut snmp_msg_trap, pbuf_stream: &mut snmp_pbuf_stream, varbinds: &mut snmp_varbind) -> Result<(), LwipError>
{
  let tlv: snmp_asn1_tlv;
  let mut varbind: &mut snmp_varbind;

  varbind = varbinds;

  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 0, trap.vbseqlen);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );

  while (varbind != None) {
    BUILD_EXEC( snmp_append_outbound_varbind(pbuf_stream, varbind) );

    varbind = varbind.next;
  }

 return Ok(());
}

/*
 * Encodes trap header from head to tail.
 */
pub fn snmp_trap_header_enc(trap: &mut snmp_msg_trap, pbuf_stream: &mut snmp_pbuf_stream) -> Result<(), LwipError>
{
  let tlv: snmp_asn1_tlv;

  //  'Message' sequence 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 0, trap.seqlen);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );

  //  version 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
  snmp_asn1_enc_s32t_cnt(trap.snmp_version, &tlv.value_len);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
  BUILD_EXEC( snmp_asn1_enc_s32t(pbuf_stream, tlv.value_len, trap.snmp_version) );

  //  community 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OCTET_STRING, 0, trap.comlen);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
  BUILD_EXEC( snmp_asn1_enc_raw(pbuf_stream,  snmp_community_trap, trap.comlen) );

  //  'PDU' sequence 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, (SNMP_ASN1_CLASS_CONTEXT | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED | SNMP_ASN1_CONTEXT_PDU_TRAP), 0, trap.pdulen);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );

  //  object ID 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OBJECT_ID, 0, 0);
  snmp_asn1_enc_oid_cnt(trap.enterprise.id, trap.enterprise.len, &tlv.value_len);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
  BUILD_EXEC( snmp_asn1_enc_oid(pbuf_stream, trap.enterprise.id, trap.enterprise.len) );

  //  IP addr 
  if (IP_IS_V6_VAL(trap.sip)) {

    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_IPADDR, 0, sizeof(ip_2_ip6(&trap.sip).addr));
    BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
    BUILD_EXEC( snmp_asn1_enc_raw(pbuf_stream, &ip_2_ip6(&trap.sip).addr, sizeof(ip_2_ip6(&trap.sip).addr)) );

  } else {

    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_IPADDR, 0, sizeof(ip_2_ip4(&trap.sip).addr));
    BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
    BUILD_EXEC( snmp_asn1_enc_raw(pbuf_stream, &ip_2_ip4(&trap.sip).addr, sizeof(ip_2_ip4(&trap.sip).addr)) );

  }

  //  trap length 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
  snmp_asn1_enc_s32t_cnt(trap.gen_trap, &tlv.value_len);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
  BUILD_EXEC( snmp_asn1_enc_s32t(pbuf_stream, tlv.value_len, trap.gen_trap) );

  //  specific trap 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
  snmp_asn1_enc_s32t_cnt(trap.spc_trap, &tlv.value_len);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
  BUILD_EXEC( snmp_asn1_enc_s32t(pbuf_stream, tlv.value_len, trap.spc_trap) );

  //  timestamp 
  SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_TIMETICKS, 0, 0);
  snmp_asn1_enc_s32t_cnt(trap.ts, &tlv.value_len);
  BUILD_EXEC( snmp_ans1_enc_tlv(pbuf_stream, &tlv) );
  BUILD_EXEC( snmp_asn1_enc_s32t(pbuf_stream, tlv.value_len, trap.ts) );

 return Ok(());
}


