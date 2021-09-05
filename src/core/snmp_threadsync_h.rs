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

// typedef void (*snmp_threadsync_called_fn)(arg: &mut ());
type snmp_threadsync_called_fn = fn(arg: &mut Vec<u8>);

// typedef void (*snmp_threadsync_synchronizer_fn)(snmp_threadsync_called_fn fn, arg: &mut ());
type snmp_threadsync_synchronizer_fn = fn(func: snmp_threadsync_called_fn, arg: &mut Vec<u8>);

/* Thread sync runtime data. For internal usage only. */
pub struct threadsync_data {
    pub snmp_let_err: err_t,
    pub s16: i16,
    pub root_oid: u32,
    pub value: Vec<u8>,
    pub root_oid_len: u8,
    pub len: usize,
    pub threadsync_node: snmp_threadsync_node,
    pub proxy_instance: snmp_node_instance,
}

/* Thread sync instance. Needed EXCATLY once for every thread to be synced into. */
pub struct snmp_threadsync_instance {
    pub sem: sys_sem_t,
    pub sem_usage_mutex: sys_mutex_t,
    pub sync_fn: snmp_threadsync_synchronizer_fn,
    pub data: threadsync_data,
}

/* SNMP thread sync proxy leaf node */
pub struct snmp_threadsync_node {
    /* inherited "base class" members */
    pub node: snmp_leaf_node,

    pub target: snmp_leaf_node,
    pub instance: snmp_threadsync_instance,
}

// snmp_snmp_threadsync_get_instance: err_t( root_oid: &mut u32, root_oid_len: u8, struct snmp_node_instance* instance);
// snmp_snmp_threadsync_get_next_instance: err_t( root_oid: &mut u32, root_oid_len: u8, struct snmp_node_instance* instance);

/* Create thread sync proxy node */
// #define SNMP_CREATE_THREAD_SYNC_NODE(oid, target_leaf_node, threadsync_instance) \
//   {{{ SNMP_NODE_THREADSYNC, (oid) }, \
//     snmp_threadsync_get_instance, \
//     snmp_threadsync_get_next_instance }, \
//     (target_leaf_node), \
//     (threadsync_instance) }

/* Create thread sync instance data */
// pub fn  snmp_threadsync_init(instance: &mut snmp_threadsync_instance, snmp_threadsync_synchronizer_fn sync_fn);
