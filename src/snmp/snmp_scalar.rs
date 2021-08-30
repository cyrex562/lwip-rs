/*
 * @file
 * SNMP scalar node support implementation.
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








static i16 snmp_scalar_array_get_value(instance: &mut snmp_node_instance, value: &mut ());
static snmp_err_t  snmp_scalar_array_set_test(instance: &mut snmp_node_instance, value_len: u16, value: &mut ());
static snmp_err_t  snmp_scalar_array_set_value(instance: &mut snmp_node_instance, value_len: u16, value: &mut ());

snmp_err_t
snmp_scalar_get_instance(const u32 *root_oid, root_oid_len: u8, instance: &mut snmp_node_instance)
{
  const scalar_node: &mut snmp_scalar_node = (const struct snmp_scalar_node *)instance.node;

  
  

  /* scalar only has one dedicated instance: .0 */
  if ((instance.instance_oid.len != 1) || (instance.instance_oid.id[0] != 0)) {
    return SNMP_ERR_NOSUCHINSTANCE;
  }

  instance.access    = scalar_node.access;
  instance.asn1_type = scalar_node.asn1_type;
  instance.get_value = scalar_node.get_value;
  instance.set_test  = scalar_node.set_test;
  instance.set_value = scalar_node.set_value;
  return SNMP_ERR_NOERROR;
}

snmp_err_t
snmp_scalar_get_next_instance(const u32 *root_oid, root_oid_len: u8, instance: &mut snmp_node_instance)
{
  /* because our only instance is .0 we can only return a next instance if no instance oid is passed */
  if (instance.instance_oid.len == 0) {
    instance.instance_oid.len   = 1;
    instance.instance_oid.id[0] = 0;

    return snmp_scalar_get_instance(root_oid, root_oid_len, instance);
  }

  return SNMP_ERR_NOSUCHINSTANCE;
}


snmp_err_t
snmp_scalar_array_get_instance(const u32 *root_oid, root_oid_len: u8, instance: &mut snmp_node_instance)
{
  
  

  if ((instance.instance_oid.len == 2) && (instance.instance_oid.id[1] == 0)) {
    const array_node: &mut snmp_scalar_array_node = (const struct snmp_scalar_array_node *)instance.node;
    const array_node_def: &mut snmp_scalar_array_node_def = array_node.array_nodes;
    i: u32 = 0;

    while (i < array_node.array_node_count) {
      if (array_node_def.oid == instance.instance_oid.id[0]) {
        break;
      }

      array_node_def+= 1;
      i+= 1;
    }

    if (i < array_node.array_node_count) {
      instance.access              = array_node_def.access;
      instance.asn1_type           = array_node_def.asn1_type;
      instance.get_value           = snmp_scalar_array_get_value;
      instance.set_test            = snmp_scalar_array_set_test;
      instance.set_value           = snmp_scalar_array_set_value;
      instance.reference.const_ptr = array_node_def;

      return SNMP_ERR_NOERROR;
    }
  }

  return SNMP_ERR_NOSUCHINSTANCE;
}

snmp_err_t
snmp_scalar_array_get_next_instance(const u32 *root_oid, root_oid_len: u8, instance: &mut snmp_node_instance)
{
  const array_node: &mut snmp_scalar_array_node = (const struct snmp_scalar_array_node *)instance.node;
  const array_node_def: &mut snmp_scalar_array_node_def = array_node.array_nodes;
  const result: &mut snmp_scalar_array_node_def = None;

  
  

  if ((instance.instance_oid.len == 0) && (array_node.array_node_count > 0)) {
    /* return node with lowest OID */
let     i: u16 = 0;

    result = array_node_def;
    array_node_def+= 1;

    for (i = 1; i < array_node.array_node_count; i+= 1) {
      if (array_node_def.oid < result.oid) {
        result = array_node_def;
      }
      array_node_def+= 1;
    }
  } else if (instance.instance_oid.len >= 1) {
    if (instance.instance_oid.len == 1) {
      /* if we have the requested OID we return its instance, otherwise we search for the next available */
let       i: u16 = 0;
      while (i < array_node.array_node_count) {
        if (array_node_def.oid == instance.instance_oid.id[0]) {
          result = array_node_def;
          break;
        }

        array_node_def+= 1;
        i+= 1;
      }
    }
    if (result == None) {
      oid_dist: u32 = 0xFFFFFFFFUL;
      i: u16        = 0;
      array_node_def = array_node.array_nodes; /* may be already at the end when if case before was executed without result -> reinitialize to start */
      while (i < array_node.array_node_count) {
        if ((array_node_def.oid > instance.instance_oid.id[0]) &&
            ((array_node_def.oid - instance.instance_oid.id[0]) < oid_dist)) {
          result   = array_node_def;
          oid_dist = array_node_def.oid - instance.instance_oid.id[0];
        }

        array_node_def+= 1;
        i+= 1;
      }
    }
  }

  if (result == None) {
    /* nothing to return */
    return SNMP_ERR_NOSUCHINSTANCE;
  }

  instance.instance_oid.len   = 2;
  instance.instance_oid.id[0] = result.oid;
  instance.instance_oid.id[1] = 0;

  instance.access              = result.access;
  instance.asn1_type           = result.asn1_type;
  instance.get_value           = snmp_scalar_array_get_value;
  instance.set_test            = snmp_scalar_array_set_test;
  instance.set_value           = snmp_scalar_array_set_value;
  instance.reference.const_ptr = result;

  return SNMP_ERR_NOERROR;
}

pub fn snmp_scalar_array_get_value(instance: &mut snmp_node_instance, value: &mut ())
{
  i16 result = -1;
  const array_node: &mut snmp_scalar_array_node = (const struct snmp_scalar_array_node *)instance.node;
  const array_node_def: &mut snmp_scalar_array_node_def = (const struct snmp_scalar_array_node_def *)instance.reference.const_ptr;

  if (array_node.get_value != None) {
    result = array_node.get_value(array_node_def, value);
  }
  return result;
}

pub fn snmp_scalar_array_set_test(instance: &mut snmp_node_instance, value_len: u16, value: &mut ())
{
  snmp_result: err_t = SNMP_ERR_NOTWRITABLE;
  const array_node: &mut snmp_scalar_array_node = (const struct snmp_scalar_array_node *)instance.node;
  const array_node_def: &mut snmp_scalar_array_node_def = (const struct snmp_scalar_array_node_def *)instance.reference.const_ptr;

  if (array_node.set_test != None) {
    result = array_node.set_test(array_node_def, value_len, value);
  }
  return result;
}

pub fn snmp_scalar_array_set_value(instance: &mut snmp_node_instance, value_len: u16, value: &mut ())
{
  snmp_result: err_t = SNMP_ERR_NOTWRITABLE;
  const array_node: &mut snmp_scalar_array_node = (const struct snmp_scalar_array_node *)instance.node;
  const array_node_def: &mut snmp_scalar_array_node_def = (const struct snmp_scalar_array_node_def *)instance.reference.const_ptr;

  if (array_node.set_value != None) {
    result = array_node.set_value(array_node_def, value_len, value);
  }
  return result;
}


