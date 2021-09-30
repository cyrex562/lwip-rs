/*
 * @file
 * SNMP core API for implementing MIBs
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
 * Author: Christiaan Simons <christiaan.simons@axon.tv>
 *         Martin Hentschel <info@cl-soft.de>
 */

// #define LWIP_HDR_APPS_SNMP_CORE_H

//  basic ASN1 defines 
pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;
pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;
pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;
pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;
pub const SNMP_ASN1_CLASS_APPLICATION: u32 = 0x40;
pub const SNMP_ASN1_CLASS_CONTEXT: u32 = 0x80;
pub const SNMP_ASN1_CLASS_PRIVATE: u32 = 0xC0;

pub const SNMP_ASN1_CONTENTTYPE_PRIMITIVE: u32 = 0x00;
pub const SNMP_ASN1_CONTENTTYPE_PRIMITIVE: u32 = 0x00;
pub const SNMP_ASN1_CONTENTTYPE_CONSTRUCTED: u32 = 0x20;

//  universal tags (from ASN.1 spec.) 
pub const SNMP_ASN1_UNIVERSAL_END_OF_CONTENT: u32 = 0;
pub const SNMP_ASN1_UNIVERSAL_INTEGER: u32 = 2;
pub const SNMP_ASN1_UNIVERSAL_OCTET_STRING: u32 = 4;
pub const SNMP_ASN1_UNIVERSAL_None: u32 = 5;
pub const SNMP_ASN1_UNIVERSAL_OBJECT_ID: u32 = 6;
pub const SNMP_ASN1_UNIVERSAL_SEQUENCE_OF: u32 = 16;

//  application specific (SNMP) tags (from SNMPv2-SMI) 
pub const SNMP_ASN1_APPLICATION_IPADDR: u32 = 0; //  [APPLICATION 0] IMPLICIT OCTET STRING (SIZE (4)) 
pub const SNMP_ASN1_APPLICATION_IPADDR: u32 = 0;
pub const SNMP_ASN1_APPLICATION_IPADDR: u32 = 0;
pub const SNMP_ASN1_APPLICATION_COUNTER: u32 = 1; //  [APPLICATION 1] IMPLICIT INTEGER (0..4294967295) => u32 
pub const SNMP_ASN1_APPLICATION_GAUGE: u32 = 2; //  [APPLICATION 2] IMPLICIT INTEGER (0..4294967295) => u32 
pub const SNMP_ASN1_APPLICATION_TIMETICKS: u32 = 3; //  [APPLICATION 3] IMPLICIT INTEGER (0..4294967295) => u32 
pub const SNMP_ASN1_APPLICATION_OPAQUE: u32 = 4; //  [APPLICATION 4] IMPLICIT OCTET STRING 
pub const SNMP_ASN1_APPLICATION_COUNTER64: u32 = 6; //  [APPLICATION 6] IMPLICIT INTEGER (0..18446744073709551615) 

//  context specific (SNMP) tags (from RFC 1905) 
pub const SNMP_ASN1_CONTEXT_VARBIND_NO_SUCH_INSTANCE: u32 = 1;

//  full ASN1 type defines 
pub const SNMP_ASN1_TYPE_END_OF_CONTENT: u32 = (SNMP_ASN1_CLASS_UNIVERSAL
    | SNMP_ASN1_CONTENTTYPE_PRIMITIVE
    | SNMP_ASN1_UNIVERSAL_END_OF_CONTENT);
pub const SNMP_ASN1_TYPE_INTEGER: u32 =
    (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_INTEGER);
pub const SNMP_ASN1_TYPE_OCTET_STRING: u32 = (SNMP_ASN1_CLASS_UNIVERSAL
    | SNMP_ASN1_CONTENTTYPE_PRIMITIVE
    | SNMP_ASN1_UNIVERSAL_OCTET_STRING);
pub const SNMP_ASN1_TYPE_NULL: u32 =
    (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_NULL);
pub const SNMP_ASN1_TYPE_OBJECT_ID: u32 =
    (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_OBJECT_ID);
pub const SNMP_ASN1_TYPE_SEQUENCE: u32 = (SNMP_ASN1_CLASS_UNIVERSAL
    | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED
    | SNMP_ASN1_UNIVERSAL_SEQUENCE_OF);
pub const SNMP_ASN1_TYPE_IPADDR: u32 =
    (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_IPADDR);
pub const SNMP_ASN1_TYPE_IPADDRESS: u32 = SNMP_ASN1_TYPE_IPADDR;
pub const SNMP_ASN1_TYPE_COUNTER: u32 =
    (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_COUNTER);
pub const SNMP_ASN1_TYPE_COUNTER32: u32 = SNMP_ASN1_TYPE_COUNTER;
pub const SNMP_ASN1_TYPE_GAUGE: u32 =
    (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_GAUGE);
pub const SNMP_ASN1_TYPE_GAUGE32: u32 = SNMP_ASN1_TYPE_GAUGE;
pub const SNMP_ASN1_TYPE_UNSIGNED32: u32 = SNMP_ASN1_TYPE_GAUGE;
pub const SNMP_ASN1_TYPE_TIMETICKS: u32 = (SNMP_ASN1_CLASS_APPLICATION
    | SNMP_ASN1_CONTENTTYPE_PRIMITIVE
    | SNMP_ASN1_APPLICATION_TIMETICKS);
pub const SNMP_ASN1_TYPE_OPAQUE: u32 =
    (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_OPAQUE);

pub const SNMP_ASN1_TYPE_COUNTER64: u32 = (SNMP_ASN1_CLASS_APPLICATION
    | SNMP_ASN1_CONTENTTYPE_PRIMITIVE
    | SNMP_ASN1_APPLICATION_COUNTER64);

pub const SNMP_VARBIND_EXCEPTION_OFFSET: u32 = 0xF0;
pub const SNMP_VARBIND_EXCEPTION_MASK: u32 = 0x0F;

//  error codes predefined by SNMP prot. 
pub enum snmp_err_t {
    SNMP_ERR_NOERROR = 0,
    /*
    outdated v1 error codes. do not use anmore!
    pub const SNMP_ERR_NOSUCHNAME: u32 = 2;   use SNMP_ERR_NOSUCHINSTANCE insteadpub const SNMP_ERR_NOSUCHNAME: u32 = 2; pub const SNMP_ERR_NOSUCHNAME: u32 = 2;
    pub const SNMP_ERR_BADVALUE: u32 = 3;  use SNMP_ERR_WRONGTYPE,SNMP_ERR_WRONGLENGTH,SNMP_ERR_WRONGENCODING or SNMP_ERR_WRONGVALUE instead
    pub const SNMP_ERR_READONLY: u32 = 4;  use SNMP_ERR_NOTWRITABLE instead
    */
    SNMP_ERR_GENERROR = 5,
    SNMP_ERR_NOACCESS = 6,
    SNMP_ERR_WRONGTYPE = 7,
    SNMP_ERR_WRONGLENGTH = 8,
    SNMP_ERR_WRONGENCODING = 9,
    SNMP_ERR_WRONGVALUE = 10,
    SNMP_ERR_NOCREATION = 11,
    SNMP_ERR_INCONSISTENTVALUE = 12,
    SNMP_ERR_RESOURCEUNAVAILABLE = 13,
    SNMP_ERR_COMMITFAILED = 14,
    SNMP_ERR_UNDOFAILED = 15,
    SNMP_ERR_NOTWRITABLE = 17,
    SNMP_ERR_INCONSISTENTNAME = 18,

    SNMP_ERR_NOSUCHINSTANCE =
        SNMP_VARBIND_EXCEPTION_OFFSET + SNMP_ASN1_CONTEXT_VARBIND_NO_SUCH_INSTANCE,
}

//  internal object identifier representation 
pub struct snmp_obj_id {
    pub len: u8,
    pub id: [u32; SNMP_MAX_OBJ_ID_LEN],
}

pub impl snmp_obj_id {
    fn new(len: usize, id: [u32]) -> snmp_obj_id {
        snmp_obj_id { len: len, id: id }
    }
}

pub struct snmp_obj_id_const_ref {
    pub len: u8,
    // TODO: pub id: &mut u32;
}

// extern const struct snmp_obj_id_const_ref snmp_zero_dot_zero;
//  administrative identifier from SNMPv2-SMI 

//  SNMP variant value, used as reference in struct snmp_node_instance and table implementation 
pub struct snmp_variant_value {
    pub ptr: Vec<u8>,
    pub const_ptr: Vec<u8>,
    pub u32_val: u32,
    pub s32_val: i32,
    pub u64_val: u64,
}

/*
SNMP MIB node types
 tree node is the only node the stack can process in order to walk the tree,
 all other nodes are assumed to be leaf nodes.
 This cannot be an users: because may want to define their own node types.
*/
pub const SNMP_NODE_TREE: u32 = 0x00;
//  predefined leaf node types 
pub const SNMP_NODE_SCALAR: u32 = 0x01;
pub const SNMP_NODE_SCALAR: u32 = 0x01;
pub const SNMP_NODE_SCALAR: u32 = 0x01;
pub const SNMP_NODE_SCALAR: u32 = 0x01;
pub const SNMP_NODE_SCALAR_ARRAY: u32 = 0x02;
pub const SNMP_NODE_TABLE: u32 = 0x03;
pub const SNMP_NODE_THREADSYNC: u32 = 0x04;

//  node "base class" layout, the mandatory fields for a node  
pub struct snmp_node {
    //  one out of SNMP_NODE_TREE or any leaf node type (like SNMP_NODE_SCALAR) 
    pub node_type: u8,
    //  the number assigned to this node which used as part of the full OID 
    pub oid: u32,
}

//  SNMP node instance access types 
pub enum snmp_access_t {
    SNMP_NODE_INSTANCE_ACCESS_READ = 1,
    SNMP_NODE_INSTANCE_ACCESS_WRITE = 2,
    SNMP_NODE_INSTANCE_READ_ONLY = SNMP_NODE_INSTANCE_ACCESS_READ,
    SNMP_NODE_INSTANCE_READ_WRITE =
        (SNMP_NODE_INSTANCE_ACCESS_READ | SNMP_NODE_INSTANCE_ACCESS_WRITE),
    SNMP_NODE_INSTANCE_WRITE_ONLY = SNMP_NODE_INSTANCE_ACCESS_WRITE,
    SNMP_NODE_INSTANCE_NOT_ACCESSIBLE = 0,
}

// struct snmp_node_instance;

// typedef i16 (*node_instance_get_value_method)(struct snmp_node_instance*, void*);
type node_instance_get_value_method = fn(&mut snmp_node_instance, &mut Vec<u8>) -> i16;

// typedef snmp_err_t (*node_instance_set_test_method)(struct snmp_node_instance*, u16, void*);
type node_instance_set_test_method = fn(&mut snmp_node_instance, u16, &mut Vec<u8>) -> snmp_err_t;

// typedef snmp_err_t (*node_instance_set_value_method)(struct snmp_node_instance*, u16, void*);
type node_instance_set_value_method = fn(&mut snmp_node_instance, u16, &mut Vec<u8>) -> snmp_err_t;

// typedef void (*node_instance_release_method)(struct snmp_node_instance*);
type node_instance_release_method = fn(&mut snmp_node_instance);

pub const SNMP_GET_VALUE_RAW_DATA: u32 = 0x4000; //  do not use 0x8000 because return value of node_instance_get_value_method is signed16 and 0x8000 would be the signed bit 

//  SNMP node instance 
pub struct snmp_node_instance {
    //  prefilled with the node, get_instance() is called on; may be changed by user to any value to pass an arbitrary node between calls to get_instance() and get_value/test_value/set_value 
    pub node: snmp_node,
    //  prefilled with the instance id requested; for get_instance() this is the exact oid requested; for get_next_instance() this is the relative starting point, stack expects relative oid of next node here 
    pub instance_oid: snmp_obj_id,

    //  ASN type for this object (see snmp_asn1.h for definitions) 
    pub asn1_type: u8,
    //  one out of instance access types defined above (SNMP_NODE_INSTANCE_READ_ONLY,...) 
    pub access: snmp_access_t,

    //  returns object value for the given object identifier. Return values <0 to indicate an error 
    pub get_value: node_instance_get_value_method,
    //  tests length and/or range BEFORE setting 
    pub set_test: node_instance_set_test_method,
    //  sets object value, only called when set_test() was successful 
    pub set_value: node_instance_set_value_method,
    //  called in any case when the instance is not required anymore by stack (useful for freeing memory allocated in get_instance/get_next_instance methods) 
    pub release_instance: node_instance_release_method,

    //  reference to pass arbitrary value between calls to get_instance() and get_value/test_value/set_value 
    pub snmp_variant_value: reference,
    //  see reference (if reference is a pointer, the length of underlying data may be stored here or anything else) 
    pub reference_len: u32,
}

//  SNMP tree node 
pub struct snmp_tree_node {
    //  inherited "base class" members 
    pub node: snmp_node,
    pub subnode_count: u16,
    pub subnodes: Vec<snmp_node>,
}

// #define SNMP_CREATE_TREE_NODE(oid, subnodes) \
//   {{ SNMP_NODE_TREE, (oid) }, \
//   LWIP_ARRAYSIZE(subnodes), (subnodes) }

// #define SNMP_CREATE_EMPTY_TREE_NODE(oid) \
//   {{ SNMP_NODE_TREE, (oid) }, \
//   0, None }

//  SNMP leaf node 
pub type leaf_node_get_inst =
    fn(root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance) -> snmp_err_t;

pub type leaf_node_get_next_inst =
    fn(root_oid: &mut u32, root_oid_len: u8, instance: &mut snmp_node_instance) -> snmp_err_t;
pub struct snmp_leaf_node {
    //  inherited "base class" members 
    pub node: snmp_node,
    pub get_instance_func: leaf_node_get_inst,
    pub get_next_instance_func: leaf_node_get_next_inst,
}

//  represents a single mib with its base oid and root node 
pub struct snmp_mib {
    pub base_oid: Vec<u32>,
    pub base_oid_len: u8,
    pub root_node: snmp_node,
}

// #define SNMP_MIB_CREATE(oid_list, root_node) { (oid_list), LWIP_ARRAYSIZE(oid_list), root_node }

//  OID range structure 
pub struct snmp_oid_range {
    pub min: u32,
    pub max: u32,
}

pub impl snmp_oid_range {
    fn new(min: u32, max: u32) -> snmp_oid_range {
        snmp_oid_range { min, max }
    }
}

//  checks if incoming OID length and values are in allowed ranges 
// snmp_oid_in_range: u8( oid_in: &mut u32, oid_len: u8,  oid_ranges: &mut snmp_oid_range, oid_ranges_len: u8);

pub enum snmp_next_oid_status_t {
    SNMP_NEXT_OID_STATUS_SUCCESS,
    SNMP_NEXT_OID_STATUS_NO_MATCH,
    SNMP_NEXT_OID_STATUS_BUF_TO_SMALL,
}

//  state for next_oid_init / next_oid_check functions 
pub struct snmp_next_oid_state {
    pub start_oid: Vec<u32>,
    pub start_oid_len: u8,
    pub next_oid: u32,
    pub next_oid_len: u8,
    pub next_oid_max_len: u8,
    pub status: snmp_next_oid_status_t,
    pub reference: Vec<u8>,
}

// pub fn  snmp_next_oid_init(state: &mut snmp_next_oid_state,
//  start_oid: &mut u32, start_oid_len: u8,
//   next_oid_buf: &mut u32, next_oid_max_len: u8);
// snmp_next_oid_precheck: u8(state: &mut snmp_next_oid_state,  oid: &mut u32, oid_len: u8);
// snmp_next_oid_check: u8(state: &mut snmp_next_oid_state,  oid: &mut u32, oid_len: u8, reference: &mut Vec<u8>);

// pub fn  snmp_oid_assign(target: &mut snmp_obj_id,  oid: &mut u32, oid_len: u8);
// pub fn  snmp_oid_combine(target: &mut snmp_obj_id,  oid1: &mut u32, oid1_len: u8,  oid2: &mut u32, oid2_len: u8);
// pub fn  snmp_oid_prefix(target: &mut snmp_obj_id,  oid: &mut u32, oid_len: u8);
// pub fn  snmp_oid_append(target: &mut snmp_obj_id,  oid: &mut u32, oid_len: u8);
// snmp_oid_equal: u8( oid1: &mut u32, oid1_len: u8,  oid2: &mut u32, oid2_len: u8);
// snmp_oid_compare: i8( oid1: &mut u32, oid1_len: u8,  oid2: &mut u32, oid2_len: u8);

// snmp_oid_to_ip4: u8( oid: &mut u32, ip: &mut LwipAddr);
// pub fn  snmp_ip4_to_oid( ip: &mut LwipAddr, oid: &mut u32);

// snmp_oid_to_ip6: u8( oid: &mut u32, ip: &mut ip6_addr_t);
// pub fn  snmp_ip6_to_oid( ip: &mut ip6_addr_t, oid: &mut u32);

// snmp_ip_to_oid: u8( ip: &mut LwipAddr, oid: &mut u32);
// snmp_ip_port_to_oid: u8( ip: &mut LwipAddr, port: u16, oid: &mut u32);

// snmp_oid_to_ip: u8( oid: &mut u32, oid_len: u8, ip: &mut LwipAddr);
// snmp_oid_to_ip_port: u8( oid: &mut u32, oid_len: u8, ip: &mut LwipAddr, port: &mut u16);

// NetIfc;
// netif_to_num: u8( netif: &mut NetIfc);

// snmp_snmp_set_test_ok: err_t(instance: &mut snmp_node_instance, value_len: u16, value: &mut Vec<u8>); //  generic function which can be used if test is always successful 
// pub fn  snmp_decode_bits( buf: &mut Vec<u8>, buf_len: u32, bit_value: &mut u32);
// pub fn  snmp_decode_truthvalue( asn1_value: &mut i32, bool_value: &mut Vec<u8>);
// snmp_encode_bits: u8(buf: &mut Vec<u8>, buf_len: u32, bit_value: u32, bit_count: u8);
// snmp_encode_truthvalue: u8(asn1_value: &mut i32, bool_value: u32);

pub struct snmp_statistics {
    pub inpkts: u32,
    pub outpkts: u32,
    pub inbadversions: u32,
    pub inbadcommunitynames: u32,
    pub inbadcommunityuses: u32,
    pub inasnparseerrs: u32,
    pub intoobigs: u32,
    pub innosuchnames: u32,
    pub inbadvalues: u32,
    pub inreadonlys: u32,
    pub ingenerrs: u32,
    pub intotalreqvars: u32,
    pub intotalsetvars: u32,
    pub ingetrequests: u32,
    pub ingetnexts: u32,
    pub insetrequests: u32,
    pub ingetresponses: u32,
    pub intraps: u32,
    pub outtoobigs: u32,
    pub outnosuchnames: u32,
    pub outbadvalues: u32,
    pub outgenerrs: u32,
    pub outgetrequests: u32,
    pub outgetnexts: u32,
    pub outsetrequests: u32,
    pub outgetresponses: u32,
    pub outtraps: u32,

    pub unsupportedseclevels: u32,
    pub notintimewindows: u32,
    pub unknownusernames: u32,
    pub unknownengineids: u32,
    pub wrongdigests: u32,
    pub decryptionerrors: u32,
}
