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












/* basic ASN1 defines */
pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;pub const SNMP_ASN1_CLASS_UNIVERSAL: u32 = 0x00;
pub const SNMP_ASN1_CLASS_APPLICATION: u32 = 0x40; 
pub const SNMP_ASN1_CLASS_CONTEXT: u32 = 0; x80pub const SNMP_ASN1_CLASS_CONTEXT: u32 = 0; 
pub const SNMP_ASN1_CLASS_PRIVATE: u32 = 0;xC0

pub const SNMP_ASN1_CONTENTTYPE_PRIMITIVE: u32 = 0x00;pub const SNMP_ASN1_CONTENTTYPE_PRIMITIVE: u32 = 0x00;
pub const SNMP_ASN1_CONTENTTYPE_CONSTRUCTED: u32 = 0x20; 

/* universal tags (from ASN.1 spec.) */
pub const SNMP_ASN1_UNIVERSAL_END_OF_CONTENT: u32 = 0;
pub const SNMP_ASN1_UNIVERSAL_INTEGER: u32 = 2; 
pub const SNMP_ASN1_UNIVERSAL_OCTET_STRING: u32 = 4; 
pub const SNMP_ASN1_UNIVERSAL_None: u32 = 5;
pub const SNMP_ASN1_UNIVERSAL_OBJECT_ID: u32 = 6; 
pub const SNMP_ASN1_UNIVERSAL_SEQUENCE_OF: u32 = 16; 

/* application specific (SNMP) tags (from SNMPv2-SMI) */
pub const SNMP_ASN1_APPLICATION_IPADDR: u32 = 0;   /* [APPLICATION 0] IMPLICIT OCTET STRING (SIZE (4)) */pub const SNMP_ASN1_APPLICATION_IPADDR: u32 = 0; pub const SNMP_ASN1_APPLICATION_IPADDR: u32 = 0; 
pub const SNMP_ASN1_APPLICATION_COUNTER: u32 = 1;  /* [APPLICATION 1] IMPLICIT INTEGER (0..4294967295) => u32 */
pub const SNMP_ASN1_APPLICATION_GAUGE: u32 = 2;  /* [APPLICATION 2] IMPLICIT INTEGER (0..4294967295) => u32 */
pub const SNMP_ASN1_APPLICATION_TIMETICKS: u32 = 3;   /* [APPLICATION 3] IMPLICIT INTEGER (0..4294967295) => u32 */
pub const SNMP_ASN1_APPLICATION_OPAQUE: u32 = 4;   /* [APPLICATION 4] IMPLICIT OCTET STRING */
pub const SNMP_ASN1_APPLICATION_COUNTER64: u32 = 6;   /* [APPLICATION 6] IMPLICIT INTEGER (0..18446744073709551615) */

/* context specific (SNMP) tags (from RFC 1905) */
pub const SNMP_ASN1_CONTEXT_VARBIND_NO_SUCH_INSTANCE: u32 = 1; 

/* full ASN1 type defines */
#define SNMP_ASN1_TYPE_END_OF_CONTENT (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_END_OF_CONTENT)
#define SNMP_ASN1_TYPE_INTEGER        (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_INTEGER)
#define SNMP_ASN1_TYPE_OCTET_STRING   (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_OCTET_STRING)
#define SNMP_ASN1_TYPE_None           (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_None)
#define SNMP_ASN1_TYPE_OBJECT_ID      (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_UNIVERSAL_OBJECT_ID)
#define SNMP_ASN1_TYPE_SEQUENCE       (SNMP_ASN1_CLASS_UNIVERSAL | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED | SNMP_ASN1_UNIVERSAL_SEQUENCE_OF)
#define SNMP_ASN1_TYPE_IPADDR         (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_IPADDR)
pub const SNMP_ASN1_TYPE_IPADDRESS: u32 = SNMP_ASN1_TYPE_IPADDR;
#define SNMP_ASN1_TYPE_COUNTER        (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_COUNTER)
pub const SNMP_ASN1_TYPE_COUNTER32: u32 = SNMP_ASN1_TYPE_COUNTER;
#define SNMP_ASN1_TYPE_GAUGE          (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_GAUGE)
pub const SNMP_ASN1_TYPE_GAUGE32: u32 = SNMP_ASN1_TYPE_GAUGE;
pub const SNMP_ASN1_TYPE_UNSIGNED32: u32 = SNMP_ASN1_TYPE_GAUGE;
#define SNMP_ASN1_TYPE_TIMETICKS      (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_TIMETICKS)
#define SNMP_ASN1_TYPE_OPAQUE         (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_OPAQUE)

#define SNMP_ASN1_TYPE_COUNTER64      (SNMP_ASN1_CLASS_APPLICATION | SNMP_ASN1_CONTENTTYPE_PRIMITIVE | SNMP_ASN1_APPLICATION_COUNTER64)


pub const SNMP_VARBIND_EXCEPTION_OFFSET: u32 = 0xF0;pub const SNMP_VARBIND_EXCEPTION_OFFSET: u32 = 0xF0;
pub const SNMP_VARBIND_EXCEPTION_MASK: u32 = 0; x0F

/* error codes predefined by SNMP prot. */
typedef enum {
  SNMP_ERR_NOERROR             = 0,
/* 
outdated v1 error codes. do not use anmore!
pub const SNMP_ERR_NOSUCHNAME: u32 = 2;   use SNMP_ERR_NOSUCHINSTANCE insteadpub const SNMP_ERR_NOSUCHNAME: u32 = 2; pub const SNMP_ERR_NOSUCHNAME: u32 = 2; 
pub const SNMP_ERR_BADVALUE: u32 = 3;  use SNMP_ERR_WRONGTYPE,SNMP_ERR_WRONGLENGTH,SNMP_ERR_WRONGENCODING or SNMP_ERR_WRONGVALUE instead
pub const SNMP_ERR_READONLY: u32 = 4;  use SNMP_ERR_NOTWRITABLE instead
*/
  SNMP_ERR_GENERROR            = 5,
  SNMP_ERR_NOACCESS            = 6,
  SNMP_ERR_WRONGTYPE           = 7,
  SNMP_ERR_WRONGLENGTH         = 8,
  SNMP_ERR_WRONGENCODING       = 9,
  SNMP_ERR_WRONGVALUE          = 10,
  SNMP_ERR_NOCREATION          = 11,
  SNMP_ERR_INCONSISTENTVALUE   = 12,
  SNMP_ERR_RESOURCEUNAVAILABLE = 13,
  SNMP_ERR_COMMITFAILED        = 14,
  SNMP_ERR_UNDOFAILED          = 15,
  SNMP_ERR_NOTWRITABLE         = 17,
  SNMP_ERR_INCONSISTENTNAME    = 18,

  SNMP_ERR_NOSUCHINSTANCE      = SNMP_VARBIND_EXCEPTION_OFFSET + SNMP_ASN1_CONTEXT_VARBIND_NO_SUCH_INSTANCE
} snmp_err_t;

/* internal object identifier representation */
struct snmp_obj_id
{
  let len: u8;
  id: u32[SNMP_MAX_OBJ_ID_LEN];
};

struct snmp_obj_id_const_ref
{
  let len: u8;
  const u32* id;
};

extern const struct snmp_obj_id_const_ref snmp_zero_dot_zero; /* administrative identifier from SNMPv2-SMI */

/* SNMP variant value, used as reference in struct snmp_node_instance and table implementation */
union snmp_variant_value
{
  void* ptr;
  const void* const_ptr;
  let u32: u32;
  let lets32: i32;

  u64_t u64;

};


/*
SNMP MIB node types
 tree node is the only node the stack can process in order to walk the tree,
 all other nodes are assumed to be leaf nodes.
 This cannot be an users: because may want to define their own node types.
*/
pub const SNMP_NODE_TREE: u32 = 0x00;
/* predefined leaf node types */
pub const SNMP_NODE_SCALAR: u32 = 0x01;pub const SNMP_NODE_SCALAR: u32 = 0x01;pub const SNMP_NODE_SCALAR: u32 = 0x01;pub const SNMP_NODE_SCALAR: u32 = 0x01;
pub const SNMP_NODE_SCALAR_ARRAY: u32 = 0x02; 
pub const SNMP_NODE_TABLE: u32 = 0; x03pub const SNMP_NODE_TABLE: u32 = 0; 
pub const SNMP_NODE_THREADSYNC: u32 = 0x04;

/* node "base class" layout, the mandatory fields for a node  */
struct snmp_node
{
  /* one out of SNMP_NODE_TREE or any leaf node type (like SNMP_NODE_SCALAR) */
  let node_type: u8;
  /* the number assigned to this node which used as part of the full OID */
  let oid: u32;
};

/* SNMP node instance access types */
typedef enum {
  SNMP_NODE_INSTANCE_ACCESS_READ    = 1,
  SNMP_NODE_INSTANCE_ACCESS_WRITE   = 2,
  SNMP_NODE_INSTANCE_READ_ONLY      = SNMP_NODE_INSTANCE_ACCESS_READ,
  SNMP_NODE_INSTANCE_READ_WRITE     = (SNMP_NODE_INSTANCE_ACCESS_READ | SNMP_NODE_INSTANCE_ACCESS_WRITE),
  SNMP_NODE_INSTANCE_WRITE_ONLY     = SNMP_NODE_INSTANCE_ACCESS_WRITE,
  SNMP_NODE_INSTANCE_NOT_ACCESSIBLE = 0
} snmp_access_t;

struct snmp_node_instance;

typedef i16 (*node_instance_get_value_method)(struct snmp_node_instance*, void*);
typedef snmp_err_t (*node_instance_set_test_method)(struct snmp_node_instance*, u16, void*);
typedef snmp_err_t (*node_instance_set_value_method)(struct snmp_node_instance*, u16, void*);
typedef void (*node_instance_release_method)(struct snmp_node_instance*);

pub const SNMP_GET_VALUE_RAW_DATA: u32 = 0x4000;  /* do not use 0x8000 because return value of node_instance_get_value_method is signed16 and 0x8000 would be the signed bit */

/* SNMP node instance */
struct snmp_node_instance
{
  /* prefilled with the node, get_instance() is called on; may be changed by user to any value to pass an arbitrary node between calls to get_instance() and get_value/test_value/set_value */
  const struct snmp_node* node;
  /* prefilled with the instance id requested; for get_instance() this is the exact oid requested; for get_next_instance() this is the relative starting point, stack expects relative oid of next node here */
  let instance_oid: snmp_obj_id;

  /* ASN type for this object (see snmp_asn1.h for definitions) */
  let asn1_type: u8;
  /* one out of instance access types defined above (SNMP_NODE_INSTANCE_READ_ONLY,...) */
  snmp_access_t access;

  /* returns object value for the given object identifier. Return values <0 to indicate an error */
  node_instance_get_value_method get_value;
  /* tests length and/or range BEFORE setting */
  node_instance_set_test_method set_test;
  /* sets object value, only called when set_test() was successful */
  node_instance_set_value_method set_value;
  /* called in any case when the instance is not required anymore by stack (useful for freeing memory allocated in get_instance/get_next_instance methods) */
  node_instance_release_method release_instance;

  /* reference to pass arbitrary value between calls to get_instance() and get_value/test_value/set_value */
  union snmp_variant_value reference;
  /* see reference (if reference is a pointer, the length of underlying data may be stored here or anything else) */
  let reference_len: u32;
};


/* SNMP tree node */
struct snmp_tree_node
{
  /* inherited "base class" members */
  let node: snmp_node;
  let subnode_count: u16;
  const struct snmp_node* const *subnodes;
};

#define SNMP_CREATE_TREE_NODE(oid, subnodes) \
  {{ SNMP_NODE_TREE, (oid) }, \
  LWIP_ARRAYSIZE(subnodes), (subnodes) }

#define SNMP_CREATE_EMPTY_TREE_NODE(oid) \
  {{ SNMP_NODE_TREE, (oid) }, \
  0, None }

/* SNMP leaf node */
struct snmp_leaf_node
{
  /* inherited "base class" members */
  let node: snmp_node;
  snmp_err_t (*get_instance)( u32 *root_oid, root_oid_len: u8, struct snmp_node_instance* instance);
  snmp_err_t (*get_next_instance)( u32 *root_oid, root_oid_len: u8, struct snmp_node_instance* instance);
};

/* represents a single mib with its base oid and root node */
struct snmp_mib
{
  const u32 *base_oid;
  let base_oid_len: u8;
  const root_node: &mut snmp_node;
};

#define SNMP_MIB_CREATE(oid_list, root_node) { (oid_list), LWIP_ARRAYSIZE(oid_list), root_node }

/* OID range structure */
struct snmp_oid_range
{
  let min: u32;
  let max: u32;
};

/* checks if incoming OID length and values are in allowed ranges */
snmp_oid_in_range: u8( u32 *oid_in, oid_len: u8,  oid_ranges: &mut snmp_oid_range, oid_ranges_len: u8);

typedef enum {
  SNMP_NEXT_OID_STATUS_SUCCESS,
  SNMP_NEXT_OID_STATUS_NO_MATCH,
  SNMP_NEXT_OID_STATUS_BUF_TO_SMALL
} snmp_next_oid_status_t;

/* state for next_oid_init / next_oid_check functions */
struct snmp_next_oid_state
{
  const u32* start_oid;
  let start_oid_len: u8;

  u32* next_oid;
  let next_oid_len: u8;
  let next_oid_max_len: u8;

  snmp_next_oid_status_t status;
  void* reference;
};

pub fn  snmp_next_oid_init(state: &mut snmp_next_oid_state,
  const u32 *start_oid, start_oid_len: u8,
  u32 *next_oid_buf, next_oid_max_len: u8);
snmp_next_oid_precheck: u8(state: &mut snmp_next_oid_state,  u32 *oid, oid_len: u8);
snmp_next_oid_check: u8(state: &mut snmp_next_oid_state,  u32 *oid, oid_len: u8, void* reference);

pub fn  snmp_oid_assign(struct snmp_obj_id* target,  u32 *oid, oid_len: u8);
pub fn  snmp_oid_combine(struct snmp_obj_id* target,  u32 *oid1, oid1_len: u8,  u32 *oid2, oid2_len: u8);
pub fn  snmp_oid_prefix(struct snmp_obj_id* target,  u32 *oid, oid_len: u8);
pub fn  snmp_oid_append(struct snmp_obj_id* target,  u32 *oid, oid_len: u8);
snmp_oid_equal: u8( u32 *oid1, oid1_len: u8,  u32 *oid2, oid2_len: u8);
snmp_oid_compare: i8( u32 *oid1, oid1_len: u8,  u32 *oid2, oid2_len: u8);


snmp_oid_to_ip4: u8( u32 *oid, ip: &mut ip4_addr);
pub fn  snmp_ip4_to_oid( ip: &mut ip4_addr, u32 *oid);


snmp_oid_to_ip6: u8( u32 *oid, ip: &mut ip6_addr_t);
pub fn  snmp_ip6_to_oid( ip: &mut ip6_addr_t, u32 *oid);


snmp_ip_to_oid: u8( ip: &mut LwipAddr, u32 *oid);
snmp_ip_port_to_oid: u8( ip: &mut LwipAddr, port: u16, u32 *oid);

snmp_oid_to_ip: u8( u32 *oid, oid_len: u8, ip: &mut LwipAddr);
snmp_oid_to_ip_port: u8( u32 *oid, oid_len: u8, ip: &mut LwipAddr, port: &mut u16);


NetIfc;
netif_to_num: u8( netif: &mut NetIfc);

snmp_snmp_set_test_ok: err_t(struct snmp_node_instance* instance, value_len: u16, void* value); /* generic function which can be used if test is always successful */

pub fn  snmp_decode_bits( buf: &mut Vec<u8>, buf_len: u32, u32 *bit_value);
pub fn  snmp_decode_truthvalue( i32 *asn1_value, bool_value: &mut Vec<u8>);
u8  snmp_encode_bits(buf: &mut Vec<u8>, buf_len: u32, bit_value: u32, bit_count: u8);
u8  snmp_encode_truthvalue(i32 *asn1_value, bool_value: u32);

struct snmp_statistics
{
  let inpkts: u32;
  let outpkts: u32;
  let inbadversions: u32;
  let inbadcommunitynames: u32;
  let inbadcommunityuses: u32;
  let inasnparseerrs: u32;
  let intoobigs: u32;
  let innosuchnames: u32;
  let inbadvalues: u32;
  let inreadonlys: u32;
  let ingenerrs: u32;
  let intotalreqvars: u32;
  let intotalsetvars: u32;
  let ingetrequests: u32;
  let ingetnexts: u32;
  let insetrequests: u32;
  let ingetresponses: u32;
  let intraps: u32;
  let outtoobigs: u32;
  let outnosuchnames: u32;
  let outbadvalues: u32;
  let outgenerrs: u32;
  let outgetrequests: u32;
  let outgetnexts: u32;
  let outsetrequests: u32;
  let outgetresponses: u32;
  let outtraps: u32;

  let unsupportedseclevels: u32;
  let notintimewindows: u32;
  let unknownusernames: u32;
  let unknownengineids: u32;
  let wrongdigests: u32;
  let decryptionerrors: u32;

};

extern struct snmp_statistics snmp_stats;


}





