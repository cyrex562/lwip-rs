/*
 * @file
 * SNMP server MIB API to implement table nodes
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

//

//  default (customizable) read/write table
pub struct snmp_table_col_def {
    pub index: u32,
    pub asn1_type: u8,
    pub access: snmp_access_t,
}

pub impl snmp_table_col_def {
    fn new(index: u32, asn1_type: u8, access: snmp_access_t) -> snmp_table_col_def {
        snmp_table_col_def {
            index,
            asn1_type,
            access,
        }
    }
}

//  table node

// snmp_err_t (*get_cell_instance)( column: &mut u32,  row_oid: &mut u32, row_oid_len: u8, cell_instance: &mut snmp_node_instance);
pub type get_cell_instance = fn(
    column: &mut u32,
    row_oid: &mut u32,
    row_oid_len: u8,
    cell_instance: &mut snmp_node_instance,
);
// pub snmp_err_t (*get_next_cell_instance)( column: &mut u32, row_oid: &mut snmp_obj_id, cell_instance: &mut snmp_node_instance);
pub type get_nex_cell_instance =
    fn(column: &mut u32, row_oid: &mut snmp_obj_id, cell_instance: &mut snmp_node_instance);

pub struct snmp_table_node {
    //  inherited "base class" members
    pub node: snmp_leaf_node,
    pub column_count: u16,
    pub columns: snmp_table_col_def,
    pub get_cell_inst_func: get_cell_instance,
    pub get_next_cell_inst_func: get_next_cell_instance,
    //  returns object value for the given object identifier
    pub get_value: node_instance_get_value_method,
    //  tests length and/or range BEFORE setting
    pub set_test: node_instance_set_test_method,
    //  sets object value, only called when set_test() was successful
    pub set_value: node_instance_set_value_method,
}

// snmp_snmp_table_get_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);
// snmp_snmp_table_get_next_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);

// #define SNMP_TABLE_CREATE(oid, columns, get_cell_instance_method, get_next_cell_instance_method, get_value_method, set_test_method, set_value_method) \
//   {{{ SNMP_NODE_TABLE, (oid) }, \
//   snmp_table_get_instance, \
//   snmp_table_get_next_instance }, \
//   LWIP_ARRAYSIZE(columns), (columns), \
//   (get_cell_instance_method), (get_next_cell_instance_method), \
//   (get_value_method), (set_test_method), (set_value_method)}

// #define SNMP_TABLE_GET_COLUMN_FROM_OID(oid) ((oid)[1]) //  first array value is (fixed) row entry (fixed to 1) and 2nd value is column, follow3ed by instance
//  simple read-only table
pub enum snmp_table_column_data_type_t {
    SNMP_VARIANT_VALUE_TYPE_U32,
    SNMP_VARIANT_VALUE_TYPE_S32,
    SNMP_VARIANT_VALUE_TYPE_PTR,
    SNMP_VARIANT_VALUE_TYPE_CONST_PTR,
}

pub struct snmp_table_simple_col_def {
    pub index: u32,
    pub asn1_type: u8,
    pub data_type: snmp_table_column_data_type_t, //  depending of what union member is used to store the value
}

pub impl snmp_table_simple_col_def {
    fn new(
        index: u32,
        asn1_type: u8,
        data_type: snmp_table_column_data_type_t,
    ) -> snmp_table_simple_col_def {
        snmp_table_simple_col_def {
            index,
            asn1_type,
            data_type,
        }
    }
}

//  simple read-only table node
// snmp_err_t (*get_cell_value)( column: &mut u32,  row_oid: &mut u32, row_oid_len: u8, union snmp_variant_value* value, value_len: &mut u32);
pub type get_cell_value = fn(
    column: &mut u32,
    row_id: &mut u32,
    row_oid_len: u8,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) -> snmp_err_t;

// snmp_err_t (*get_next_cell_instance_and_value)( column: &mut u32, row_oid: &mut snmp_obj_id, union snmp_variant_value* value, value_len: &mut u32);
pub type get_next_cell_instance_and_value = fn(
    column: &mut u32,
    row_oid: &mut snmp_obj_id,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) -> snmp_err_t;

pub struct snmp_table_simple_node {
    //  inherited "base class" members
    pub node: snmp_leaf_node,
    pub column_count: u16,
    pub columns: snmp_table_simple_col_def,
}

// snmp_snmp_table_simple_get_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);
// snmp_snmp_table_simple_get_next_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);

// #define SNMP_TABLE_CREATE_SIMPLE(oid, columns, get_cell_value_method, get_next_cell_instance_and_value_method) \
//   {{{ SNMP_NODE_TABLE, (oid) }, \
//   snmp_table_simple_get_instance, \
//   snmp_table_simple_get_next_instance }, \
//   LWIP_ARRAYSIZE(columns), (columns), (get_cell_value_method), (get_next_cell_instance_and_value_method) }

// snmp_table_extract_value_from_s32ref: i16(instance: &mut snmp_node_instance, value: &mut Vec<u8>);
// snmp_table_extract_value_from_u32ref: i16(instance: &mut snmp_node_instance, value: &mut Vec<u8>);
// snmp_table_extract_value_from_refconstptr: i16(instance: &mut snmp_node_instance, value: &mut Vec<u8>);
