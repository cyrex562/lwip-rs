/*
 * @file
 * SNMP server MIB API to implement scalar nodes
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

// #define LWIP_HDR_APPS_SNMP_SCALAR_H

/* basic scalar node */
pub struct snmp_scalar_node {
    /* inherited "base class" members */
    pub node: snmp_leaf_node,
    pub asn1_type: u8,
    pub access: snmp_access_t,
    pub get_value: node_instance_get_value_method,
    pub set_test: node_instance_set_test_method,
    pub set_value: node_instance_set_value_method,
}

// snmp_snmp_scalar_get_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);
// snmp_snmp_scalar_get_next_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);

// #define SNMP_SCALAR_CREATE_NODE(oid, access, asn1_type, get_value_method, set_test_method, set_value_method) \
//   {{{ SNMP_NODE_SCALAR, (oid) }, \
//     snmp_scalar_get_instance, \
//     snmp_scalar_get_next_instance }, \
//     (asn1_type), (access), (get_value_method), (set_test_method), (set_value_method) }

// #define SNMP_SCALAR_CREATE_NODE_READONLY(oid, asn1_type, get_value_method) SNMP_SCALAR_CREATE_NODE(oid, SNMP_NODE_INSTANCE_READ_ONLY, asn1_type, get_value_method, None, None)

/* scalar array node - a tree node which contains scalars only as children */
pub struct snmp_scalar_array_node_def {
    pub oid: u32,
    pub asn1_type: u8,
    pub access: snmp_access_t,
}

// typedef i16 (*snmp_scalar_array_get_value_method)( struct snmp_scalar_array_node_def*, void*);
// typedef snmp_err_t (*snmp_scalar_array_set_test_method)( struct snmp_scalar_array_node_def*, u16, void*);
// typedef snmp_err_t (*snmp_scalar_array_set_value_method)( struct snmp_scalar_array_node_def*, u16, void*);

/* basic scalar array node */
pub struct snmp_scalar_array_node {
    /* inherited "base class" members */
    pub node: snmp_leaf_node,
    pub array_node_count: u16,
    pub array_nodes: snmp_scalar_array_node_def,
    pub get_value: snmp_scalar_array_get_value_method,
    pub set_test: snmp_scalar_array_set_test_method,
    pub set_value: snmp_scalar_array_set_value_method,
}

// snmp_snmp_scalar_array_get_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);
// snmp_snmp_scalar_array_get_next_instance: err_t( root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance);

// #define SNMP_SCALAR_CREATE_ARRAY_NODE(oid, array_nodes, get_value_method, set_test_method, set_value_method) \
//   {{{ SNMP_NODE_SCALAR_ARRAY, (oid) }, \
//     snmp_scalar_array_get_instance, \
//     snmp_scalar_array_get_next_instance }, \
//     LWIP_ARRAYSIZE(array_nodes), (array_nodes), (get_value_method), (set_test_method), (set_value_method) }
