/*
 * @file
 * Management Information Base II (RFC1213) objects and functions.
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

/*
 * @defgroup snmp_mib2 MIB2
 * @ingroup snmp
 */

// #error LWIP_SNMP MIB2 needs LWIP_STATS (for MIB2)

// #error LWIP_SNMP MIB2 needs MIB2_STATS (for MIB2)

pub fn snmp_mib2_lwip_synchronizer(func: snmp_threadsync_called_fn, arg: &mut Vec<u8>) {
    LOCK_TCPIP_CORE();
    func(arg);
    UNLOCK_TCPIP_CORE();

    tcpip_callback(func, arg);
}

// let snmp_mib2_lwip_locks: snmp_threadsync_instance;

/* dot3 and EtherLike MIB not planned. (transmission .1.3.6.1.2.1.10) */
/* historical (some say hysterical). (cmot .1.3.6.1.2.1.9) */
/* lwIP has no EGP, thus may not implement it. (egp .1.3.6.1.2.1.8) */

/* --- mib-2 .1.3.6.1.2.1 ----------------------------------------------------- */
// extern const struct snmp_scalar_array_node snmp_mib2_snmp_root;
// extern pub const snmp_mib2_udp_root: snmp_tree_node;
// extern pub const snmp_mib2_tcp_root: snmp_tree_node;
// extern const struct snmp_scalar_array_node snmp_mib2_icmp_root;
// extern pub const snmp_mib2_interface_root: snmp_tree_node;
// extern const struct snmp_scalar_array_node snmp_mib2_system_node;
// extern pub const snmp_mib2_at_root: snmp_tree_node;
// extern pub const snmp_mib2_ip_root: snmp_tree_node;

// static const const: &mut snmp_node mib2_nodes[] = {
//   &snmp_mib2_system_node.node.node,
//   &snmp_mib2_interface_root.node,

//   &snmp_mib2_at_root.node,

//   &snmp_mib2_ip_root.node,

//   &snmp_mib2_icmp_root.node.node,

//   &snmp_mib2_tcp_root.node,

//   &snmp_mib2_udp_root.node,

//   &snmp_mib2_snmp_root.node.node
// };

// static pub const mib2_root: snmp_tree_node = SNMP_CREATE_TREE_NODE(1, mib2_nodes);

// static const u32  mib2_base_oid_arr[] = { 1, 3, 6, 1, 2, 1 };
// const struct snmp_mib mib2 = SNMP_MIB_CREATE(mib2_base_oid_arr, &mib2_root.node);
