/*
 * @file
 * Management Information Base II (RFC1213) SNMP objects and functions.
 */

/*
 * Copyright (c) 2006 Axon Digital Design B.V., The Netherlands.
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *         Christiaan Simons <christiaan.simons@axon.tv>
 */









pub const MIB2_AUTH_TRAPS_ENABLED: u32 = 1; 
pub const MIB2_AUTH_TRAPS_DISABLED: u32 = 2; 

/* --- snmp .1.3.6.1.2.1.11 ----------------------------------------------------- */
pub fn snmp_get_value( node: &mut snmp_scalar_array_node_def, value: &mut Vec<u8>)
{
  let uint_ptr: &mut u32 = value;
  match (node.oid) {
    1 => {/* snmpInPkts */
      *uint_ptr = snmp_stats.inpkts;}
      
    2 =>{ /* snmpOutPkts */
      *uint_ptr = snmp_stats.outpkts;}
      
    3 =>{ /* snmpInBadVersions */
      *uint_ptr = snmp_stats.inbadversions;}
      
    4 =>{ /* snmpInBadCommunityNames */
      *uint_ptr = snmp_stats.inbadcommunitynames;}
      
    5 =>{ /* snmpInBadCommunityUses */
      *uint_ptr = snmp_stats.inbadcommunityuses;}
      
    6 =>{ /* snmpInASNParseErrs */
      *uint_ptr = snmp_stats.inasnparseerrs;}
      
    8 =>{ /* snmpInTooBigs */
      *uint_ptr = snmp_stats.intoobigs;}
      
    9 =>{ /* snmpInNoSuchNames */
      *uint_ptr = snmp_stats.innosuchnames;}
      
    10 =>{ /* snmpInBadValues */
      *uint_ptr = snmp_stats.inbadvalues;}
      
    11 =>{ /* snmpInReadOnlys */
      *uint_ptr = snmp_stats.inreadonlys;
}      
    12 =>{ /* snmpInGenErrs */
      *uint_ptr = snmp_stats.ingenerrs;}
      
    13 =>{ /* snmpInTotalReqVars */
      *uint_ptr = snmp_stats.intotalreqvars;}
      
    14 =>{ /* snmpInTotalSetVars */
      *uint_ptr = snmp_stats.intotalsetvars;}
      
    15 =>{ /* snmpInGetRequests */
      *uint_ptr = snmp_stats.ingetrequests;}
      
    16 =>{ /* snmpInGetNexts */
      *uint_ptr = snmp_stats.ingetnexts;}
      
    17 =>{ /* snmpInSetRequests */
      *uint_ptr = snmp_stats.insetrequests;}
      
    18 =>{ /* snmpInGetResponses */
      *uint_ptr = snmp_stats.ingetresponses;}
      
    19 =>{ /* snmpInTraps */
      *uint_ptr = snmp_stats.intraps;}
      
    20 =>{ /* snmpOutTooBigs */
      *uint_ptr = snmp_stats.outtoobigs;}
      
    21 =>{ /* snmpOutNoSuchNames */
      *uint_ptr = snmp_stats.outnosuchnames;}
      
    22 =>{ /* snmpOutBadValues */
      *uint_ptr = snmp_stats.outbadvalues;}
      
    24 =>{ /* snmpOutGenErrs */
      *uint_ptr = snmp_stats.outgenerrs;}
      
    25 =>{ /* snmpOutGetRequests */
      *uint_ptr = snmp_stats.outgetrequests;
}      
    26 =>{ /* snmpOutGetNexts */
      *uint_ptr = snmp_stats.outgetnexts;}
      
    27 => {/* snmpOutSetRequests */
      *uint_ptr = snmp_stats.outsetrequests;}
      
    28 =>{ /* snmpOutGetResponses */
      *uint_ptr = snmp_stats.outgetresponses;}
      
    29 =>{ /* snmpOutTraps */
      *uint_ptr = snmp_stats.outtraps;}
      
    30 =>{ /* snmpEnableAuthenTraps */
      if (snmp_get_auth_traps_enabled() == SNMP_AUTH_TRAPS_DISABLED) {
        *uint_ptr = MIB2_AUTH_TRAPS_DISABLED;
      } else {
        *uint_ptr = MIB2_AUTH_TRAPS_ENABLED;
      }}
      
    31 =>{ /* snmpSilentDrops */
      *uint_ptr = 0;} /* not supported */
      
    32 =>{ /* snmpProxyDrops */
      *uint_ptr = 0;} /* not supported */
      
    _ =>{
//      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("snmp_get_value(): unknown id: %"S32_F"\n", node.oid));
      return 0;}
  }

  return sizeof(*uint_ptr);
}

pub fn snmp_set_test( node: &mut snmp_scalar_array_node_def, len: usize, value: &mut Vec<u8>)
{
  let snmp_ret: err_t = SNMP_ERR_WRONGVALUE;
  

  if (node.oid == 30) {
    /* snmpEnableAuthenTraps */
    let sint_ptr: &mut i32 = value;

    /* we should have writable non-volatile mem here */
    if ((*sint_ptr == MIB2_AUTH_TRAPS_DISABLED) || (*sint_ptr == MIB2_AUTH_TRAPS_ENABLED)) {
      ret = SNMP_ERR_NOERROR;
    }
  }
  return ret;
}

pub fn snmp_set_value( node: &mut snmp_scalar_array_node_def, len: usize, value: &mut Vec<u8>)
{
  

  if (node.oid == 30) {
    /* snmpEnableAuthenTraps */
    let sint_ptr: &mut i32 = value;
    if (*sint_ptr == MIB2_AUTH_TRAPS_DISABLED) {
      snmp_set_auth_traps_enabled(SNMP_AUTH_TRAPS_DISABLED);
    } else {
      snmp_set_auth_traps_enabled(SNMP_AUTH_TRAPS_ENABLED);
    }
  }

  return SNMP_ERR_NOERROR;
}

/* the following nodes access variables in SNMP stack (snmp_stats) from SNMP worker thread -> OK, no sync needed */
pub const  snmp_nodes: [snmp_scalar_array_node_def] = [
  snmp_scalar_array_node_def::new( 1, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInPkts */
  snmp_scalar_array_node_def::new( 2, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutPkts */
  snmp_scalar_array_node_def::new( 3, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInBadVersions */
  snmp_scalar_array_node_def::new( 4, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInBadCommunityNames */
  snmp_scalar_array_node_def::new( 5, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInBadCommunityUses */
  snmp_scalar_array_node_def::new( 6, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInASNParseErrs */
  snmp_scalar_array_node_def::new( 8, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInTooBigs */
  snmp_scalar_array_node_def::new( 9, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInNoSuchNames */
  snmp_scalar_array_node_def::new(10, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInBadValues */
  snmp_scalar_array_node_def::new(11, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInReadOnlys */
  snmp_scalar_array_node_def::new(12, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInGenErrs */
  snmp_scalar_array_node_def::new(13, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInTotalReqVars */
  snmp_scalar_array_node_def::new(14, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInTotalSetVars */
  snmp_scalar_array_node_def::new(15, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInGetRequests */
  snmp_scalar_array_node_def::new(16, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInGetNexts */
  snmp_scalar_array_node_def::new(17, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInSetRequests */
  snmp_scalar_array_node_def::new(18, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInGetResponses */
  snmp_scalar_array_node_def::new(19, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpInTraps */
  snmp_scalar_array_node_def::new(20, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutTooBigs */
  snmp_scalar_array_node_def::new(21, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutNoSuchNames */
  snmp_scalar_array_node_def::new(22, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutBadValues */
  snmp_scalar_array_node_def::new(24, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutGenErrs */
  snmp_scalar_array_node_def::new(25, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutGetRequests */
  snmp_scalar_array_node_def::new(26, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutGetNexts */
  snmp_scalar_array_node_def::new(27, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutSetRequests */
  snmp_scalar_array_node_def::new(28, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutGetResponses */
  snmp_scalar_array_node_def::new(29, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpOutTraps */
  snmp_scalar_array_node_def::new(30, SNMP_ASN1_TYPE_INTEGER, SNMP_NODE_INSTANCE_READ_WRITE), /* snmpEnableAuthenTraps */
  snmp_scalar_array_node_def::new(31, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY),  /* snmpSilentDrops */
  snmp_scalar_array_node_def::new(32, SNMP_ASN1_TYPE_COUNTER, SNMP_NODE_INSTANCE_READ_ONLY)   /* snmpProxyDrops */
];

// const struct snmp_scalar_array_node snmp_mib2_snmp_root = SNMP_SCALAR_CREATE_ARRAY_NODE(11, snmp_nodes, snmp_get_value, snmp_set_test, snmp_set_value);


