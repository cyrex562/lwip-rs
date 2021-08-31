/*
 * @file
 * SNMP server MIB API to implement thread synchronization
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */


// #define LWIP_HDR_APPS_SNMP_THREADSYNC_H












typedef void (*snmp_threadsync_called_fn)(arg: &mut ());
typedef void (*snmp_threadsync_synchronizer_fn)(snmp_threadsync_called_fn fn, arg: &mut ());


/* Thread sync runtime data. For internal usage only. */
struct threadsync_data
{
  union {
    snmp_let err: err_t;
    let s16: i16;
  } retval;
  union {
    const u32 *root_oid;
    value: &mut ();
  } arg1;
  union {
    let root_oid_len: u8;
    let len: usize;
  } arg2;
  const threadsync_node: &mut snmp_threadsync_node;
  let proxy_instance: snmp_node_instance;
};

/* Thread sync instance. Needed EXCATLY once for every thread to be synced into. */
struct snmp_threadsync_instance
{
  sys_sem_t                       sem;
  sys_mutex_t                     sem_usage_mutex;
  snmp_threadsync_synchronizer_fn sync_fn;
  struct threadsync_data          data;
};

/* SNMP thread sync proxy leaf node */
struct snmp_threadsync_node
{
  /* inherited "base class" members */
  struct snmp_leaf_node           node;

  const struct snmp_leaf_node     *target;
  instance: &mut snmp_threadsync_instance;
};

snmp_snmp_threadsync_get_instance: err_t(const u32 *root_oid, root_oid_len: u8, struct snmp_node_instance* instance);
snmp_snmp_threadsync_get_next_instance: err_t(const u32 *root_oid, root_oid_len: u8, struct snmp_node_instance* instance);

/* Create thread sync proxy node */
#define SNMP_CREATE_THREAD_SYNC_NODE(oid, target_leaf_node, threadsync_instance) \
  {{{ SNMP_NODE_THREADSYNC, (oid) }, \
    snmp_threadsync_get_instance, \
    snmp_threadsync_get_next_instance }, \
    (target_leaf_node), \
    (threadsync_instance) }

/* Create thread sync instance data */
pub fn  snmp_threadsync_init(instance: &mut snmp_threadsync_instance, snmp_threadsync_synchronizer_fn sync_fn);




}



