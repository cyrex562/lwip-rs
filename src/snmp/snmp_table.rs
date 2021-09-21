use crate::core::err_h::LwipError;

/*
 * @file
 * SNMP table support implementation.
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
 * Author: Martin Hentschel <info@cl-soft.de>
 *
 */









pub fn snmp_snmp_table_get_instance( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance) -> Result<(), LwipError>
{
  let snmp_ret: err_t = SNMP_ERR_NOSUCHINSTANCE;
 let table_node: &mut snmp_table_node = instance.node;

  
  

  /* check min. length (fixed row entry definition, column, row instance oid with at least one entry */
  /* fixed row entry always has oid 1 */
  if ((instance.instance_oid.len >= 3) && (instance.instance_oid.id[0] == 1)) {
    /* search column */
 let col_def: &mut snmp_table_col_def = table_node.columns;
    let i: u16 = table_node.column_count;
    while (i > 0) {
      if (col_def.index == instance.instance_oid.id[1]) {
        break;
      }

      col_def+= 1;
      i -= 1;
    }

    if (i > 0) {
      /* everything may be overwritten by get_cell_instance_method() in order to implement special handling for single columns/cells */
      instance.asn1_type = col_def.asn1_type;
      instance.access    = col_def.access;
      instance.get_value = table_node.get_value;
      instance.set_test  = table_node.set_test;
      instance.set_value = table_node.set_value;

      ret = table_node.get_cell_instance(
              &(instance.instance_oid.id[1]),
              &(instance.instance_oid.id[2]),
              instance.instance_oid.len - 2,
              instance);
    }
  }

  return ret;
}

pub fn snmp_snmp_table_get_next_instance( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance) -> Result<(), LwipError>
{
 let table_node: &mut snmp_table_node = instance.node;
 let mut col_def: &mut snmp_table_col_def;
  let row_oid: snmp_obj_id;
  let column: u32 = 0;
  let snmp_result: err_t;

  /* check that first part of id is 0 or 1, referencing fixed row entry */
  if ((instance.instance_oid.len > 0) && (instance.instance_oid.id[0] > 1)) {
    return SNMP_ERR_NOSUCHINSTANCE;
  }
  if (instance.instance_oid.len > 1) {
    column = instance.instance_oid.id[1];
  }
  if (instance.instance_oid.len > 2) {
    snmp_oid_assign(&row_oid, &(instance.instance_oid.id[2]), instance.instance_oid.len - 2);
  } else {
    row_oid.len = 0;
  }

  instance.get_value    = table_node.get_value;
  instance.set_test     = table_node.set_test;
  instance.set_value    = table_node.set_value;

  /* resolve column and value */
  loop {
    let i: u16;
 let next_col_def: &mut snmp_table_col_def = None;
    col_def = table_node.columns;

    for i in 0..table_node.column_count {
      if (col_def.index == column) {
        next_col_def = col_def;
        break;
      } else if ((col_def.index > column) && ((next_col_def == None) || (col_def.index < next_col_def.index))) {
        next_col_def = col_def;
      }
      col_def+= 1;
    }

    if (next_col_def == None) {
      /* no further column found */
      return SNMP_ERR_NOSUCHINSTANCE;
    }

    instance.asn1_type          = next_col_def.asn1_type;
    instance.access             = next_col_def.access;

    result = table_node.get_next_cell_instance(
               &next_col_def.index,
               &row_oid,
               instance);

    if (result == SNMP_ERR_NOERROR) {
      col_def = next_col_def;
      break;
    }

    row_oid.len = 0; /* reset row_oid because we match to next column and start with the first entry there */
    column = next_col_def.index + 1;
  } 

  /* build resulting oid */
  instance.instance_oid.len   = 2;
  instance.instance_oid.id[0] = 1;
  instance.instance_oid.id[1] = col_def.index;
  snmp_oid_append(&instance.instance_oid, row_oid.id, row_oid.len);

  return SNMP_ERR_NOERROR;
}


pub fn snmp_snmp_table_simple_get_instance( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance) -> Result<(), LwipErr>
{
  let snmp_ret: err_t = SNMP_ERR_NOSUCHINSTANCE;
 let table_node: &mut snmp_table_simple_node = instance.node;

  
  

  /* check min. length (fixed row entry definition, column, row instance oid with at least one entry */
  /* fixed row entry always has oid 1 */
  if ((instance.instance_oid.len >= 3) && (instance.instance_oid.id[0] == 1)) {
    ret = table_node.get_cell_value(
            &(instance.instance_oid.id[1]),
            &(instance.instance_oid.id[2]),
            instance.instance_oid.len - 2,
            &instance.reference,
            &instance.reference_len);

    if (ret == SNMP_ERR_NOERROR) {
      /* search column */
 let col_def: &mut snmp_table_simple_col_def = table_node.columns;
      let i: u32 = table_node.column_count;
      while (i > 0) {
        if (col_def.index == instance.instance_oid.id[1]) {
          break;
        }

        col_def+= 1;
        i -= 1;
      }

      if (i > 0) {
        instance.asn1_type = col_def.asn1_type;
        instance.access    = SNMP_NODE_INSTANCE_READ_ONLY;
        instance.set_test  = None;
        instance.set_value = None;

        match (col_def.data_type) {
          SNMP_VARIANT_VALUE_TYPE_U32 =>{
            instance.get_value = snmp_table_extract_value_from_u32ref;}
            
          SNMP_VARIANT_VALUE_TYPE_S32 =>{
            instance.get_value = snmp_table_extract_value_from_s32ref;}
            
          SNMP_VARIANT_VALUE_TYPE_PTR | /* fall through */
          SNMP_VARIANT_VALUE_TYPE_CONST_PTR =>{
            instance.get_value = snmp_table_extract_value_from_refconstptr;}
            
          _ =>{
//            LWIP_DEBUGF(SNMP_DEBUG, ("snmp_table_simple_get_instance(): unknown column data_type: %d\n", col_def.data_type));
            return SNMP_ERR_GENERROR;}
        }

        ret = SNMP_ERR_NOERROR;
      } else {
        ret = SNMP_ERR_NOSUCHINSTANCE;
      }
    }
  }

  return ret;
}

pub fn snmp_snmp_table_simple_get_next_instance( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance) -> Result<(), LwipError>
{
 let table_node: &mut snmp_table_simple_node = instance.node;
 let mut col_def: &mut snmp_table_simple_col_def;
  let row_oid: snmp_obj_id;
  let column: u32 = 0;
  let snmp_result: err_t;

  
  

  /* check that first part of id is 0 or 1, referencing fixed row entry */
  if ((instance.instance_oid.len > 0) && (instance.instance_oid.id[0] > 1)) {
    return SNMP_ERR_NOSUCHINSTANCE;
  }
  if (instance.instance_oid.len > 1) {
    column = instance.instance_oid.id[1];
  }
  if (instance.instance_oid.len > 2) {
    snmp_oid_assign(&row_oid, &(instance.instance_oid.id[2]), instance.instance_oid.len - 2);
  } else {
    row_oid.len = 0;
  }

  /* resolve column and value */
  loop {
    let i: u32;
 let next_col_def: &mut snmp_table_simple_col_def = None;
    col_def = table_node.columns;

    for i in 0 .. table_node.column_count {
      if (col_def.index == column) {
        next_col_def = col_def;
        break;
      } else if ((col_def.index > column) && ((next_col_def == None) ||
                 (col_def.index < next_col_def.index))) {
        next_col_def = col_def;
      }
      col_def+= 1;
    }

    if (next_col_def == None) {
      /* no further column found */
      return SNMP_ERR_NOSUCHINSTANCE;
    }

    result = table_node.get_next_cell_instance_and_value(
               &next_col_def.index,
               &row_oid,
               &instance.reference,
               &instance.reference_len);

    if (result == SNMP_ERR_NOERROR) {
      col_def = next_col_def;
      break;
    }

    row_oid.len = 0; /* reset row_oid because we match to next column and start with the first entry there */
    column = next_col_def.index + 1;
  } 

  instance.asn1_type = col_def.asn1_type;
  instance.access    = SNMP_NODE_INSTANCE_READ_ONLY;
  instance.set_test  = None;
  instance.set_value = None;

  match (col_def.data_type) {
    SNMP_VARIANT_VALUE_TYPE_U32 =>{
      instance.get_value = snmp_table_extract_value_from_u32ref;}
      
    SNMP_VARIANT_VALUE_TYPE_S32 =>{
      instance.get_value = snmp_table_extract_value_from_s32ref;}
      
    SNMP_VARIANT_VALUE_TYPE_PTR | /* fall through */
    SNMP_VARIANT_VALUE_TYPE_CONST_PTR =>{
      instance.get_value = snmp_table_extract_value_from_refconstptr;}
      
    _ =>{
//      LWIP_DEBUGF(SNMP_DEBUG, ("snmp_table_simple_get_instance(): unknown column data_type: %d\n", col_def.data_type));
      return SNMP_ERR_GENERROR;}
  }

  /* build resulting oid */
  instance.instance_oid.len   = 2;
  instance.instance_oid.id[0] = 1;
  instance.instance_oid.id[1] = col_def.index;
  snmp_oid_append(&instance.instance_oid, row_oid.id, row_oid.len);

  return SNMP_ERR_NOERROR;
}


pub fn snmp_table_extract_value_from_s32ref(instance: &mut snmp_node_instance, value: &mut Vec<u8>) -> i16
{
  let dst: &mut i32 = value;
  *dst = instance.reference.s32;
  return sizeof(*dst);
}

pub fn snmp_table_extract_value_from_u32ref(instance: &mut snmp_node_instance, value: &mut Vec<u8>) -> usize
{
  let dst: &mut u32 = value;
  *dst = instance.reference.u32;
  return sizeof(*dst);
}

pub fn snmp_table_extract_value_from_refconstptr(instance: &mut snmp_node_instance, value: &mut Vec<u8>) -> u16
{
  MEMCPY(value, instance.reference.const_ptr, instance.reference_len);
  return instance.reference_len;
}


