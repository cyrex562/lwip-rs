/*
 * @file
 * MIB tree access/construction functions.
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

/*
 * @defgroup snmp SNMPv2c/v3 agent
 * @ingroup apps
 * SNMPv2c and SNMPv3 compatible agent\n
 * There is also a MIB compiler and a MIB viewer in lwIP contrib repository
 * (lwip-contrib/apps/LwipMibCompiler).\n
 * The agent implements the most important MIB2 MIBs including IPv6 support
 * (interfaces, UDP, TCP, SNMP, ICMP, SYSTEM). IP MIB is an older version
 * without IPv6 statistics (TODO).\n
 * Rewritten by Martin Hentschel <info@cl-soft.de> and
 * Dirk Ziegelmeier <dziegel@gmx.de>\n
 *
 * 0 Agent Capabilities
 * ====================
 *
 * Features:
 * ---------
 * - SNMPv2c support.
 * - SNMPv3 support (a port to ARM mbedtls is provided, LWIP_SNMP_V3_MBEDTLS option).
 * - Low RAM usage - no memory pools, stack only.
 * - MIB2 implementation is separated from SNMP stack.
 * - Support for multiple MIBs (snmp_set_mibs() call) - e.g. for private MIB.
 * - Simple and generic API for MIB implementation.
 * - Comfortable node types and helper functions for scalar arrays and tables.
 * - Counter64, bit and truthvalue datatype support.
 * - Callbacks for SNMP writes e.g. to implement persistency.
 * - Runs on two APIs: RAW and netconn.
 * - Async API is gone - the stack now supports netconn API instead,
 *   so blocking operations can be done in MIB calls.
 *   SNMP runs in a worker thread when netconn API is used.
 * - Simplified thread sync support for MIBs - useful when MIBs
 *   need to access variables shared with other threads where no locking is
 *   possible. Used in MIB2 to access lwIP stats from lwIP thread.
 *
 * MIB compiler (code generator):
 * ------------------------------
 * - Provided in lwIP contrib repository.
 * - Written in C#. MIB viewer used Windows Forms.
 * - Developed on Windows with Visual Studio 2010.
 * - Can be compiled and used on all platforms with http://www.monodevelop.com/.
 * - Based on a heavily modified version of of SharpSnmpLib (a4bd05c6afb4)
 *   (https://sharpsnmplib.codeplex.com/SourceControl/network/forks/Nemo157/MIBParserUpdate).
 * - MIB parser, C file generation framework and LWIP code generation are cleanly
 *   separated, which means the code may be useful as a base for code generation
 *   of other SNMP agents.
 *
 * Notes:
 * ------
 * - Stack and MIB compiler were used to implement a Profinet device.
 *   Compiled/implemented MIBs: LLDP-MIB, LLDP-EXT-DOT3-MIB, LLDP-EXT-PNO-MIB.
 *
 * SNMPv1 per RFC1157 and SNMPv2c per RFC 3416
 * -------------------------------------------
 *   Note the S in SNMP stands for "Simple". Note that "Simple" is
 *   relative. SNMP is simple compared to the complex ISO network
 *   management protocols CMIP (Common Management Information Protocol)
 *   and CMOT (CMip Over Tcp).
 *
 * SNMPv3
 * ------
 * When SNMPv3 is used, several functions from snmpv3.h must be implemented
 * by the user. This is mainly user management and persistence handling.
 * The sample provided in lwip-contrib is insecure, don't use it in production
 * systems, especially the missing persistence for engine boots variable
 * simplifies replay attacks.
 *
 * MIB II
 * ------
 *   The standard lwIP stack management information base.
 *   This is a required MIB, so this is always enabled.
 *   The groups EGP, CMOT and transmission are disabled by default.
 *
 *   Most mib-2 objects are not writable except:
 *   sysName, sysLocation, sysContact, snmpEnableAuthenTraps.
 *   Writing to or changing the ARP and IP address and route
 *   tables is not possible.
 *
 *   Note lwIP has a very limited notion of IP routing. It currently
 *   doen't have a route table and doesn't have a notion of the U,G,H flags.
 *   Instead lwIP uses the interface list with only one default interface
 *   acting as a single gateway interface (G) for the default route.
 *
 *   The agent returns a "virtual table" with the default route 0.0.0.0
 *   for the default interface and network routes (no H) for each
 *   network interface in the netif_list.
 *   All routes are considered to be up (U).
 *
 * Loading additional MIBs
 * -----------------------
 *   MIBs can only be added in compile-time, not in run-time.
 *
 *
 * 1 Building the Agent
 * ====================
 * First of all you'll need to add the following define
 * to your local lwipopts.h:
 * \// #define LWIP_SNMP               1
 *
 * and add the source files your makefile.
 *
 * Note you'll might need to adapt you network driver to update
 * the mib2 variables for your interface.
 *
 * 2 Running the Agent
 * ===================
 * The following function calls must be made in your program to
 * actually get the SNMP agent running.
 *
 * Before starting the agent you should supply pointers
 * for sysContact, sysLocation, and snmpEnableAuthenTraps.
 * You can do this by calling
 *
 * - snmp_mib2_set_syscontact()
 * - snmp_mib2_set_syslocation()
 * - snmp_set_auth_traps_enabled()
 *
 * You can register a callback which is called on successful write access:
 * snmp_set_write_callback().
 *
 * Additionally you may want to set
 *
 * - snmp_mib2_set_sysdescr()
 * - snmp_set_device_enterprise_oid()
 * - snmp_mib2_set_sysname()
 *
 * Also before starting the agent you need to setup
 * one or more trap destinations using these calls:
 *
 * - snmp_trap_dst_enable()
 * - snmp_trap_dst_ip_set()
 *
 * If you need more than MIB2, set the MIBs you want to use
 * by snmp_set_mibs().
 *
 * Finally, enable the agent by calling snmp_init()
 *
 * @defgroup snmp_core Core
 * @ingroup snmp
 *
 * @defgroup snmp_traps Traps
 * @ingroup snmp
 */

// #error "If you want to use SNMP, you have to define SNMP_TRAP_DESTINATIONS>=1 in your lwipopts.h"

// #error "If you want to use SNMP, you have to define LWIP_UDP=1 in your lwipopts.h"

// #error "SNMP_MAX_OBJ_ID_LEN must fit into an u8"

// let snmp_stats: snmp_statistics;
// static const struct snmp_obj_id  snmp_device_enterprise_oid_default = {SNMP_DEVICE_ENTERPRISE_OID_LEN, SNMP_DEVICE_ENTERPRISE_OID};
// static const snmp_device_enterprise_oid: &mut snmp_obj_id         = &snmp_device_enterprise_oid_default;

// // const snmp_zero_dot_zero_values: [u32;] = { 0, 0 };
// const struct snmp_obj_id_const_ref snmp_zero_dot_zero = { LWIP_ARRAYSIZE(snmp_zero_dot_zero_values), snmp_zero_dot_zero_values };

// static const const: &mut snmp_mib default_mibs[] = { &mib2, &snmpframeworkmib, &snmpusmmib };
// static snmp_num_mibs: u8                          = LWIP_ARRAYSIZE(default_mibs);
// #elif SNMP_LWIP_MIB2

// static const const: &mut snmp_mib default_mibs[] = { &mib2 };
// static snmp_num_mibs: u8                          = LWIP_ARRAYSIZE(default_mibs);

// static const const: &mut snmp_mib default_mibs[] = { None };
// static snmp_num_mibs: u8                          = 0;

/* List of known mibs */
// static struct snmp_mib const *const *snmp_mibs = default_mibs;

/*
 * @ingroup snmp_core
 * Sets the MIBs to use.
 * Example: call snmp_set_mibs() as follows:
 * static const my_snmp_mibs: &mut snmp_mib[] = {
 *   &mib2,
 *   &private_mib
 * };
 * snmp_set_mibs(my_snmp_mibs, LWIP_ARRAYSIZE(my_snmp_mibs));
 */
pub fn snmp_set_mibs(mibs: &mut Vec<snmp_mib>, num_mibs: u8) {
    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT("mibs pointer must be != NULL", (mibs != None));
    LWIP_ASSERT("num_mibs pointer must be != 0", (num_mibs != 0));
    snmp_mibs = mibs;
    snmp_num_mibs = num_mibs;
}

/*
 * @ingroup snmp_core
 * 'device enterprise oid' is used for 'device OID' field in trap PDU's (for identification of generating device)
 * as well as for value returned by MIB-2 'sysObjectID' field (if internal MIB2 implementation is used).
 * The 'device enterprise oid' shall poto: i32 an OID located under 'private-enterprises' branch (1.3.6.1.4.1.XXX). If a vendor
 * wants to provide a custom object there, he has to get its own enterprise oid from IANA (http://www.iana.org). It
 * is not allowed to use LWIP enterprise ID!
 * In order to identify a specific device it is recommended to create a dedicated OID for each device type under its own
 * enterprise oid.
 * e.g.
 * device a > 1.3.6.1.4.1.XXX(ent-oid).1(devices).1(device a)
 * device b > 1.3.6.1.4.1.XXX(ent-oid).1(devices).2(device b)
 * for more details see description of 'sysObjectID' field in RFC1213-MIB
 */
pub fn snmp_set_device_enterprise_oid(device_enterprise_oid: &mut snmp_obj_id) {
    LWIP_ASSERT_CORE_LOCKED();
    if (device_enterprise_oid == None) {
        snmp_device_enterprise_oid = &snmp_device_enterprise_oid_default;
    } else {
        snmp_device_enterprise_oid = device_enterprise_oid;
    }
}

/*
 * @ingroup snmp_core
 * Get 'device enterprise oid'
 */
pub fn snmp_get_device_enterprise_oid() -> snmp_obj_id {
    LWIP_ASSERT_CORE_LOCKED();
    return snmp_device_enterprise_oid;
}

/*
 * Conversion from InetAddressIPv4 oid to lwIP ip4_addr
 * @param oid points to u32 ident[4] input
 * @param ip points to output struct
 */
pub fn snmp_oid_to_ip4(oid: &mut u32, ip: &mut ip4_addr) -> u8 {
    if ((oid[0] > 0xFF) || (oid[1] > 0xFF) || (oid[2] > 0xFF) || (oid[3] > 0xFF)) {
        ip4_addr_copy(*ip, *IP4_ADDR_ANY4);
        return 0;
    }

    IP4_ADDR(ip, oid[0], oid[1], oid[2], oid[3]);
    return 1;
}

/*
 * Convert ip4_addr to InetAddressIPv4 (no InetAddressType)
 * @param ip points to input struct
 * @param oid points to u32 ident[4] output
 */
pub fn snmp_ip4_to_oid(ip: &mut ip4_addr, oid: &mut u32) {
    oid[0] = ip4_addr1(ip);
    oid[1] = ip4_addr2(ip);
    oid[2] = ip4_addr3(ip);
    oid[3] = ip4_addr4(ip);
}

/*
 * Conversion from InetAddressIPv6 oid to lwIP ip6_addr
 * @param oid points to u32 oid[16] input
 * @param ip points to output struct
 */
pub fn snmp_oid_to_ip6(oid: &mut u32, ip: &mut ip6_addr_t) -> u8 {
    if ((oid[0] > 0xFF)
        || (oid[1] > 0xFF)
        || (oid[2] > 0xFF)
        || (oid[3] > 0xFF)
        || (oid[4] > 0xFF)
        || (oid[5] > 0xFF)
        || (oid[6] > 0xFF)
        || (oid[7] > 0xFF)
        || (oid[8] > 0xFF)
        || (oid[9] > 0xFF)
        || (oid[10] > 0xFF)
        || (oid[11] > 0xFF)
        || (oid[12] > 0xFF)
        || (oid[13] > 0xFF)
        || (oid[14] > 0xFF)
        || (oid[15] > 0xFF))
    {
        ip6_addr_set_any(ip);
        return 0;
    }

    ip.addr[0] = (oid[0] << 24) | (oid[1] << 16) | (oid[2] << 8) | (oid[3] << 0);
    ip.addr[1] = (oid[4] << 24) | (oid[5] << 16) | (oid[6] << 8) | (oid[7] << 0);
    ip.addr[2] = (oid[8] << 24) | (oid[9] << 16) | (oid[10] << 8) | (oid[11] << 0);
    ip.addr[3] = (oid[12] << 24) | (oid[13] << 16) | (oid[14] << 8) | (oid[15] << 0);
    return 1;
}

/*
 * Convert ip6_addr to InetAddressIPv6 (no InetAddressType)
 * @param ip points to input struct
 * @param oid points to u32 ident[16] output
 */
pub fn snmp_ip6_to_oid(ip: &mut ip6_addr_t, oid: &mut u32) {
    oid[0] = (ip.addr[0] & 0xFF000000) >> 24;
    oid[1] = (ip.addr[0] & 0x00FF0000) >> 16;
    oid[2] = (ip.addr[0] & 0x0000FF00) >> 8;
    oid[3] = (ip.addr[0] & 0x000000FF) >> 0;
    oid[4] = (ip.addr[1] & 0xFF000000) >> 24;
    oid[5] = (ip.addr[1] & 0x00FF0000) >> 16;
    oid[6] = (ip.addr[1] & 0x0000FF00) >> 8;
    oid[7] = (ip.addr[1] & 0x000000FF) >> 0;
    oid[8] = (ip.addr[2] & 0xFF000000) >> 24;
    oid[9] = (ip.addr[2] & 0x00FF0000) >> 16;
    oid[10] = (ip.addr[2] & 0x0000FF00) >> 8;
    oid[11] = (ip.addr[2] & 0x000000FF) >> 0;
    oid[12] = (ip.addr[3] & 0xFF000000) >> 24;
    oid[13] = (ip.addr[3] & 0x00FF0000) >> 16;
    oid[14] = (ip.addr[3] & 0x0000FF00) >> 8;
    oid[15] = (ip.addr[3] & 0x000000FF) >> 0;
}

/*
 * Convert to InetAddressType+InetAddress+InetPortNumber
 * @param ip IP address
 * @param port Port
 * @param oid OID
 * @return OID length
 */
pub fn snmp_ip_port_to_oid(ip: &mut LwipAddr, port: u16, oid: &mut u32) -> u8 {
    let idx: u8;

    idx = snmp_ip_to_oid(ip, oid);
    oid[idx] = port;
    idx += 1;

    return idx;
}

/*
 * Convert to InetAddressType+InetAddress
 * @param ip IP address
 * @param oid OID
 * @return OID length
 */
pub fn snmp_ip_to_oid(ip: &mut LwipAddr, oid: &mut u32) -> u8 {
    if (IP_IS_ANY_TYPE_VAL(*ip)) {
        oid[0] = 0; /* any */
        oid[1] = 0; /* no IP OIDs follow */
        return 2;
    } else if (IP_IS_V6(ip)) {
        oid[0] = 2; /* ipv6 */
        oid[1] = 16; /* 16 InetAddressIPv6 OIDs follow */
        snmp_ip6_to_oid(ip_2_ip6(ip), &oid[2]);
        return 18;
        /* LWIP_IPV6 */
        return 0;
    } else {
        oid[0] = 1; /* ipv4 */
        oid[1] = 4; /* 4 InetAddressIPv4 OIDs follow */
        snmp_ip4_to_oid(ip_2_ip4(ip), &oid[2]);
        return 6;
        /* LWIP_IPV4 */
        return 0;
    }
}

/*
 * Convert from InetAddressType+InetAddress to LwipAddr
 * @param oid OID
 * @param oid_len OID length
 * @param ip IP address
 * @return Parsed OID length
 */
pub fn snmp_oid_to_ip(oid: &mut u32, oid_len: u8, ip: &mut LwipAddr) -> u8 {
    /* InetAddressType */
    if (oid_len < 1) {
        return 0;
    }

    if (oid[0] == 0) {
        /* any */
        /* 1x InetAddressType, 1x OID len */
        if (oid_len < 2) {
            return 0;
        }
        if (oid[1] != 0) {
            return 0;
        }

        //memset(ip, 0, sizeof(*ip));
        IP_SET_TYPE(ip, IPADDR_TYPE_ANY);

        return 2;
    } else if (oid[0] == 1) {
        /* ipv4 */

        /* 1x InetAddressType, 1x OID len, 4x InetAddressIPv4 */
        if (oid_len < 6) {
            return 0;
        }

        /* 4x ipv4 OID */
        if (oid[1] != 4) {
            return 0;
        }

        IP_SET_TYPE(ip, IPADDR_TYPE_V4);
        if (!snmp_oid_to_ip4(&oid[2], ip_2_ip4(ip))) {
            return 0;
        }

        return 6;
        /* LWIP_IPV4 */
        return 0;
    } else if (oid[0] == 2) {
        /* ipv6 */

        /* 1x InetAddressType, 1x OID len, 16x InetAddressIPv6 */
        if (oid_len < 18) {
            return 0;
        }

        /* 16x ipv6 OID */
        if (oid[1] != 16) {
            return 0;
        }

        IP_SET_TYPE(ip, IPADDR_TYPE_V6);
        if (!snmp_oid_to_ip6(&oid[2], ip_2_ip6(ip))) {
            return 0;
        }

        return 18;
        /* LWIP_IPV6 */
        return 0;
    } else {
        /* unsupported InetAddressType */
        return 0;
    }
}

/*
 * Convert from InetAddressType+InetAddress+InetPortNumber to LwipAddr and u16
 * @param oid OID
 * @param oid_len OID length
 * @param ip IP address
 * @param port Port
 * @return Parsed OID length
 */
pub fn snmp_oid_to_ip_port(oid: &mut u32, oid_len: u8, ip: &mut LwipAddr, port: &mut u16) -> u8 {
    let idx: u8;

    /* InetAddressType + InetAddress */
    idx = snmp_oid_to_ip(&oid[0], oid_len, ip);
    if (idx == 0) {
        return 0;
    }

    /* InetPortNumber */
    if (oid_len < (idx + 1)) {
        return 0;
    }
    if (oid[idx] > 0xffff) {
        return 0;
    }
    *port = oid[idx];
    idx += 1;

    return idx;
}

/*
 * Assign an OID to struct snmp_obj_id
 * @param target Assignment target
 * @param oid OID
 * @param oid_len OID length
 */
pub fn snmp_oid_assign(target: &mut snmp_obj_id, oid: &mut u32, oid_len: u8) {
    LWIP_ASSERT(
        "oid_len <= SNMP_MAX_OBJ_ID_LEN",
        oid_len <= SNMP_MAX_OBJ_ID_LEN,
    );

    target.len = oid_len;

    if (oid_len > 0) {
        MEMCPY(target.id, oid, oid_len * sizeof);
    }
}

/*
 * Prefix an OID to OID in struct snmp_obj_id
 * @param target Assignment target to prefix
 * @param oid OID
 * @param oid_len OID length
 */
pub fn snmp_oid_prefix(target: &mut snmp_obj_id, oid: &mut u32, oid_len: u8) {
    LWIP_ASSERT(
        "target.len + oid_len <= SNMP_MAX_OBJ_ID_LEN",
        (target.len + oid_len) <= SNMP_MAX_OBJ_ID_LEN,
    );

    if (oid_len > 0) {
        /* move existing OID to make room at the beginning for OID to insert */
        let leti: i32;
        // for (i = target.len - 1; i >= 0; i--) {
        //   target.id[i + oid_len] = target.id[i];
        // }

        /* paste oid at the beginning */
        MEMCPY(target.id, oid, oid_len * sizeof);
    }
}

/*
 * Combine two OIDs into struct snmp_obj_id
 * @param target Assignmet target
 * @param oid1 OID 1
 * @param oid1_len OID 1 length
 * @param oid2 OID 2
 * @param oid2_len OID 2 length
 */
pub fn snmp_oid_combine(
    target: &mut snmp_obj_id,
    oid1: &mut u32,
    oid1_len: u8,
    oid2: &mut u32,
    oid2_len: u8,
) {
    snmp_oid_assign(target, oid1, oid1_len);
    snmp_oid_append(target, oid2, oid2_len);
}

/*
 * Append OIDs to struct snmp_obj_id
 * @param target Assignment target to append to
 * @param oid OID
 * @param oid_len OID length
 */
pub fn snmp_oid_append(target: &mut snmp_obj_id, oid: &mut u32, oid_len: u8) {
    LWIP_ASSERT(
        "offset + oid_len <= SNMP_MAX_OBJ_ID_LEN",
        (target.len + oid_len) <= SNMP_MAX_OBJ_ID_LEN,
    );

    if (oid_len > 0) {
        MEMCPY(&target.id[target.len], oid, oid_len * sizeof);
        target.len = (target.len + oid_len);
    }
}

/*
 * Compare two OIDs
 * @param oid1 OID 1
 * @param oid1_len OID 1 length
 * @param oid2 OID 2
 * @param oid2_len OID 2 length
 * @return -1: OID1&lt;OID2  1: OID1 &gt;OID2 0: equal
 */
pub fn snmp_oid_compare(oid1: &mut u32, oid1_len: u8, oid2: &mut u32, oid2_len: u8) -> i8 {
    let level: u8 = 0;
    LWIP_ASSERT(
        "'oid1' param must not be NULL or 'oid1_len' param be 0!",
        (oid1 != None) || (oid1_len == 0),
    );
    LWIP_ASSERT(
        "'oid2' param must not be NULL or 'oid2_len' param be 0!",
        (oid2 != None) || (oid2_len == 0),
    );

    while ((level < oid1_len) && (level < oid2_len)) {
        if (*oid1 < *oid2) {
            return -1;
        }
        if (*oid1 > *oid2) {
            return 1;
        }

        level += 1;
        oid1 += 1;
        oid2 += 1;
    }

    /* common part of both OID's is equal, compare length */
    if (oid1_len < oid2_len) {
        return -1;
    }
    if (oid1_len > oid2_len) {
        return 1;
    }

    /* they are equal */
    return 0;
}

/*
 * Check of two OIDs are equal
 * @param oid1 OID 1
 * @param oid1_len OID 1 length
 * @param oid2 OID 2
 * @param oid2_len OID 2 length
 * @return 1: equal 0: non-equal
 */
pub fn snmp_oid_equal(oid1: &mut u32, oid1_len: u8, oid2: &mut u32, oid2_len: u8) -> u8 {
    return (snmp_oid_compare(oid1, oid1_len, oid2, oid2_len) == 0);
}

/*
 * Convert netif to interface index
 * @param netif netif
 * @return index
 */
pub fn netif_to_num(netif: &mut NetIfc) -> u8 {
    return netif_get_index(netif);
}

pub fn snmp_get_mib_from_oid(oid: &mut u32, oid_len: u8) -> snmp_mib {
    let list_oid: &mut u32;
    let searched_oid: &mut u32;
    let i: u8;
    let l;

    let max_match_len: u8 = 0;
    let matched_mib: &mut snmp_mib = None;

    LWIP_ASSERT("'oid' param must not be NULL!", (oid != None));

    if (oid_len == 0) {
        return None;
    }

    // for (i = 0; i < snmp_num_mibs; i+= 1) {
    //   LWIP_ASSERT("MIB array not initialized correctly", (snmp_mibs[i] != None));
    //   LWIP_ASSERT("MIB array not initialized correctly - base OID is NULL", (snmp_mibs[i].base_oid != None));

    //   if (oid_len >= snmp_mibs[i].base_oid_len) {
    //     l            = snmp_mibs[i].base_oid_len;
    //     list_oid     = snmp_mibs[i].base_oid;
    //     searched_oid = oid;

    //     while (l > 0) {
    //       if (*list_oid != *searched_oid) {
    //         break;
    //       }

    //       l -= 1;
    //       list_oid+= 1;
    //       searched_oid+= 1;
    //     }

    //     if ((l == 0) && (snmp_mibs[i].base_oid_len > max_match_len)) {
    //       max_match_len = snmp_mibs[i].base_oid_len;
    //       matched_mib = snmp_mibs[i];
    //     }
    //   }
    // }

    return matched_mib;
}

pub fn snmp_get_next_mib(oid: &mut u32, oid_len: u8) -> snmp_mib {
    let i: u8;
    let next_mib: &mut snmp_mib = None;

    LWIP_ASSERT("'oid' param must not be NULL!", (oid != None));

    if (oid_len == 0) {
        return None;
    }

    // for (i = 0; i < snmp_num_mibs; i+= 1) {
    //   if (snmp_mibs[i].base_oid != None) {
    //     /* check if mib is located behind starting point */
    //     if (snmp_oid_compare(snmp_mibs[i].base_oid, snmp_mibs[i].base_oid_len, oid, oid_len) > 0) {
    //       if ((next_mib == None) ||
    //           (snmp_oid_compare(snmp_mibs[i].base_oid, snmp_mibs[i].base_oid_len,
    //                             next_mib.base_oid, next_mib.base_oid_len) < 0)) {
    //         next_mib = snmp_mibs[i];
    //       }
    //     }
    //   }
    // }

    return next_mib;
}

pub fn snmp_get_mib_between(
    oid1: &mut u32,
    oid1_len: u8,
    oid2: &mut u32,
    oid2_len: u8,
) -> snmp_mib {
    let next_mib: &mut snmp_mib = snmp_get_next_mib(oid1, oid1_len);

    LWIP_ASSERT("'oid2' param must not be NULL!", (oid2 != None));
    LWIP_ASSERT("'oid2_len' param must be greater than 0!", (oid2_len > 0));

    if (next_mib != None) {
        if (snmp_oid_compare(next_mib.base_oid, next_mib.base_oid_len, oid2, oid2_len) < 0) {
            return next_mib;
        }
    }

    return None;
}

pub fn snmp_get_node_instance_from_oid(
    oid: &mut u32,
    oid_len: u8,
    node_instance: &mut snmp_node_instance,
) -> u8 {
    let result: u8 = SNMP_ERR_NOSUCHOBJECT;
    let mut mib: &mut snmp_mib;
    let mn: &mut snmp_node = None;

    mib = snmp_get_mib_from_oid(oid, oid_len);
    if (mib != None) {
        let oid_instance_len: u8;

        mn = snmp_mib_tree_resolve_exact(mib, oid, oid_len, &oid_instance_len);
        if ((mn != None) && (mn.node_type != SNMP_NODE_TREE)) {
            /* get instance */
            let leaf_node: &mut snmp_leaf_node = mn;

            node_instance.node = mn;
            snmp_oid_assign(
                &node_instance.instance_oid,
                oid + (oid_len - oid_instance_len),
                oid_instance_len,
            );

            result = leaf_node.get_instance(oid, oid_len - oid_instance_len, node_instance);

            if (result == SNMP_ERR_NOERROR) {
                if (((node_instance.access & SNMP_NODE_INSTANCE_ACCESS_READ) != 0)
                    && (node_instance.get_value == None))
                {
                    //          LWIP_DEBUGF(SNMP_DEBUG, ("SNMP inconsistent access: node is readable but no get_value function is specified\n"));
                }
                if (((node_instance.access & SNMP_NODE_INSTANCE_ACCESS_WRITE) != 0)
                    && (node_instance.set_value == None))
                {
                    //          LWIP_DEBUGF(SNMP_DEBUG, ("SNMP inconsistent access: node is writable but no set_value and/or set_test function is specified\n"));
                }
            }
        }
    }

    return result;
}

pub fn snmp_get_next_node_instance_from_oid(
    oid: &mut u32,
    oid_len: u8,
    validate_node_instance_method: snmp_validate_node_instance_method,
    validate_node_instance_arg: &mut Vec<u8>,
    node_oid: &mut snmp_obj_id,
    node_instance: &mut snmp_node_instance,
) -> u8 {
    let mib: &mut snmp_mib;
    let mn: &mut snmp_node = None;
    let start_oid: &mut u32 = None;
    let start_oid_len: u8 = 0;

    /* resolve target MIB from passed OID */
    mib = snmp_get_mib_from_oid(oid, oid_len);
    if (mib == None) {
        /* passed OID does not reference any known MIB, start at the next closest MIB */
        mib = snmp_get_next_mib(oid, oid_len);

        if (mib != None) {
            start_oid = mib.base_oid;
            start_oid_len = mib.base_oid_len;
        }
    } else {
        start_oid = oid;
        start_oid_len = oid_len;
    }

    /* resolve target node from MIB, skip to next MIB if no suitable node is found in current MIB */
    while ((mib != None) && (mn == None)) {
        let oid_instance_len: u8;

        /* check if OID directly references a node inside current MIB, in this case we have to ask this node for the next instance */
        mn = snmp_mib_tree_resolve_exact(mib, start_oid, start_oid_len, &oid_instance_len);
        if (mn != None) {
            snmp_oid_assign(node_oid, start_oid, start_oid_len - oid_instance_len); /* set oid to node */
            snmp_oid_assign(
                &node_instance.instance_oid,
                start_oid + (start_oid_len - oid_instance_len),
                oid_instance_len,
            ); /* set (relative) instance oid */
        } else {
            /* OID does not reference a node, search for the next closest node inside MIB; set instance_oid.len to zero because we want the first instance of this node */
            mn = snmp_mib_tree_resolve_next(mib, start_oid, start_oid_len, node_oid);
            node_instance.instance_oid.len = 0;
        }

        /* validate the node; if the node has no further instance or the returned instance is invalid, search for the next in MIB and validate again */
        node_instance.node = mn;
        while (mn != None) {
            let result: u8;

            /* clear fields which may have values from previous loops */
            node_instance.asn1_type = 0;
            node_instance.access = SNMP_NODE_INSTANCE_NOT_ACCESSIBLE;
            node_instance.get_value = None;
            node_instance.set_test = None;
            node_instance.set_value = None;
            node_instance.release_instance = None;
            node_instance.reference.ptr = None;
            node_instance.reference_len = 0;

            result = (mn).get_next_instance(node_oid.id, node_oid.len, node_instance);

            if (result == SNMP_ERR_NOERROR) {
                if (((node_instance.access & SNMP_NODE_INSTANCE_ACCESS_READ) != 0)
                    && (node_instance.get_value == None))
                {
                    //          LWIP_DEBUGF(SNMP_DEBUG, ("SNMP inconsistent access: node is readable but no get_value function is specified\n"));
                }
                if (((node_instance.access & SNMP_NODE_INSTANCE_ACCESS_WRITE) != 0)
                    && (node_instance.set_value == None))
                {
                    //          LWIP_DEBUGF(SNMP_DEBUG, ("SNMP inconsistent access: node is writable but no set_value function is specified\n"));
                }

                /* validate node because the node may be not accessible for example (but let the caller decide what is valid */
                if ((validate_node_instance_method == None)
                    || (validate_node_instance_method(node_instance, validate_node_instance_arg)
                        == SNMP_ERR_NOERROR))
                {
                    /* node_oid "returns" the full result OID (including the instance part) */
                    snmp_oid_append(
                        node_oid,
                        node_instance.instance_oid.id,
                        node_instance.instance_oid.len,
                    );
                    break;
                }

                if (node_instance.release_instance != None) {
                    node_instance.release_instance(node_instance);
                }
                /*
                the instance itself is not valid, ask for next instance from same node.
                we don't have to change any variables because node_instance.instance_oid is used as input (starting point)
                as well as output (resulting next OID), so we have to simply call get_next_instance method again
                */
            } else {
                if (node_instance.release_instance != None) {
                    node_instance.release_instance(node_instance);
                }

                /* the node has no further instance, skip to next node */
                mn = snmp_mib_tree_resolve_next(
                    mib,
                    node_oid.id,
                    node_oid.len,
                    &node_instance.instance_oid,
                ); /* misuse node_instance.instance_oid as tmp buffer */
                if (mn != None) {
                    /* prepare for next loop */
                    snmp_oid_assign(
                        node_oid,
                        node_instance.instance_oid.id,
                        node_instance.instance_oid.len,
                    );
                    node_instance.instance_oid.len = 0;
                    node_instance.node = mn;
                }
            }
        }

        if (mn != None) {
            /*
            we found a suitable next node,
            now we have to check if a inner MIB is located between the searched OID and the resulting OID.
            this is possible because MIB's may be located anywhere in the global tree, that means also in
            the subtree of another MIB (e.g. if searched OID is .2 and resulting OID is .4, then another
            MIB having .3 as root node may exist)
            */
            let mut intermediate_mib: &mut snmp_mib;
            intermediate_mib =
                snmp_get_mib_between(start_oid, start_oid_len, node_oid.id, node_oid.len);

            if (intermediate_mib != None) {
                /* search for first node inside intermediate mib in next loop */
                if (node_instance.release_instance != None) {
                    node_instance.release_instance(node_instance);
                }

                mn = None;
                mib = intermediate_mib;
                start_oid = mib.base_oid;
                start_oid_len = mib.base_oid_len;
            }
            /* else { we found out target node } */
        } else {
            /*
            there is no further (suitable) node inside this MIB, search for the next MIB with following priority
            1. search for inner MIB's (whose root is located inside tree of current MIB)
            2. search for surrouding MIB's (where the current MIB is the inner MIB) and continue there if any
            3. take the next closest MIB (not being related to the current MIB)
            */
            let mut next_mib: &mut snmp_mib;
            next_mib = snmp_get_next_mib(start_oid, start_oid_len); /* returns MIB's related to po1: i32 and 3 */

            /* is the found MIB an inner MIB? (po1: i32) */
            if ((next_mib != None)
                && (next_mib.base_oid_len > mib.base_oid_len)
                && (snmp_oid_compare(
                    next_mib.base_oid,
                    mib.base_oid_len,
                    mib.base_oid,
                    mib.base_oid_len,
                ) == 0))
            {
                /* yes it is -> continue at inner MIB */
                mib = next_mib;
                start_oid = mib.base_oid;
                start_oid_len = mib.base_oid_len;
            } else {
                /* check if there is a surrounding mib where to continue (po2: i32) (only possible if OID length > 1) */
                if (mib.base_oid_len > 1) {
                    mib = snmp_get_mib_from_oid(mib.base_oid, mib.base_oid_len - 1);

                    if (mib == None) {
                        /* no surrounding mib, use next mib encountered above (po3: i32) */
                        mib = next_mib;

                        if (mib != None) {
                            start_oid = mib.base_oid;
                            start_oid_len = mib.base_oid_len;
                        }
                    }
                    /* else { start_oid stays the same because we want to continue from current offset in surrounding mib (po2: i32) } */
                }
            }
        }
    }

    if (mib == None) {
        /* loop is only left when mib == null (error) or mib_node != NULL (success) */
        return SNMP_ERR_ENDOFMIBVIEW;
    }

    return SNMP_ERR_NOERROR;
}

/*
 * Searches tree for the supplied object identifier.
 *
 */
pub fn snmp_mib_tree_resolve_exact(
    mib: &mut snmp_mib,
    oid: &mut u32,
    oid_len: u8,
    oid_instance_len: &mut Vec<u8>,
) -> snmp_node {
    let node: &mut snmp_node = &mib.root_node;
    let oid_offset: u8 = mib.base_oid_len;

    while ((oid_offset < oid_len) && ((*node).node_type == SNMP_NODE_TREE)) {
        /* search for matching sub node */
        let subnode_oid: u32 = *(oid + oid_offset);

        let i: u32 = (node).subnode_count;
        node = (node).subnodes;
        while ((i > 0) && ((*node).oid != subnode_oid)) {
            node += 1;
            i -= 1;
        }

        if (i == 0) {
            /* no matching subnode found */
            return None;
        }

        oid_offset += 1;
    }

    if ((*node).node_type != SNMP_NODE_TREE) {
        /* we found a leaf node */
        *oid_instance_len = oid_len - oid_offset;
        return (*node);
    }

    return None;
}

pub fn snmp_mib_tree_resolve_next(
    mib: &mut snmp_mib,
    oid: &mut u32,
    oid_len: u8,
    oidret: &mut snmp_obj_id,
) -> snmp_node {
    let oid_offset: u8 = mib.base_oid_len;
    let node: &mut snmp_node;
    let node_stack: &mut Vec<snmp_tree_node>;
    let nsi = 0; /* NodeStackIndex */
    let subnode_oid: u32;

    if (mib.root_node.node_type != SNMP_NODE_TREE) {
        /* a next operation on a mib with only a leaf node will always return NULL because there is no other node */
        return None;
    }

    /* first build node stack related to passed oid (as far as possible), then go backwards to determine the next node */
    node_stack[nsi] = mib.root_node;
    while (oid_offset < oid_len) {
        /* search for matching sub node */
        let i: u32 = node_stack[nsi].subnode_count;
        node = node_stack[nsi].subnodes;

        subnode_oid = *(oid + oid_offset);

        while ((i > 0) && ((*node).oid != subnode_oid)) {
            node += 1;
            i -= 1;
        }

        if ((i == 0) || ((*node).node_type != SNMP_NODE_TREE)) {
            /* no (matching) tree-subnode found */
            break;
        }
        nsi += 1;
        node_stack[nsi] = (*node);

        oid_offset += 1;
    }

    if (oid_offset >= oid_len) {
        /* passed oid references a tree node -> return first useable sub node of it */
        subnode_oid = 0;
    } else {
        subnode_oid = *(oid + oid_offset) + 1;
    }

    while (nsi >= 0) {
        let subnode: &mut snmp_node = None;

        /* find next node on current level */
        let i = node_stack[nsi].subnode_count;
        node = node_stack[nsi].subnodes;
        while (i > 0) {
            if ((*node).oid == subnode_oid) {
                subnode = *node;
                break;
            } else if (((*node).oid > subnode_oid)
                && ((subnode == None) || ((*node).oid < subnode.oid)))
            {
                subnode = *node;
            }

            node += 1;
            i -= 1;
        }

        if (subnode == None) {
            /* no further node found on this level, go one level up and start searching with index of current node*/
            subnode_oid = node_stack[nsi].node.oid + 1;
            nsi -= 1;
        } else {
            if (subnode.node_type == SNMP_NODE_TREE) {
                /* next is a tree node, go into it and start searching */
                nsi += 1;
                node_stack[nsi] = subnode;
                subnode_oid = 0;
            } else {
                /* we found a leaf node -> fill oidret and return it */
                snmp_oid_assign(oidret, mib.base_oid, mib.base_oid_len);
                i = 1;
                while (i <= nsi) {
                    oidret.id[oidret.len] = node_stack[i].node.oid;
                    oidret.len += 1;
                    i += 1;
                }

                oidret.id[oidret.len] = subnode.oid;
                oidret.len += 1;

                return subnode;
            }
        }
    }

    return None;
}

/* initialize struct next_oid_state using this function before passing it to next_oid_check */
pub fn snmp_next_oid_init(
    state: &mut snmp_next_oid_state,
    start_oid: &mut u32,
    start_oid_len: u8,
    next_oid_buf: &mut u32,
    next_oid_max_len: u8,
) {
    state.start_oid = start_oid;
    state.start_oid_len = start_oid_len;
    state.next_oid = next_oid_buf;
    state.next_oid_len = 0;
    state.next_oid_max_len = next_oid_max_len;
    state.status = SNMP_NEXT_OID_STATUS_NO_MATCH;
}

/* checks if the passed incomplete OID may be a possible candidate for snmp_next_oid_check();
this methid is intended if the complete OID is not yet known but it is very expensive to build it up,
so it is possible to test the starting part before building up the complete oid and pass it to snmp_next_oid_check()*/
pub fn snmp_next_oid_precheck(state: &mut snmp_next_oid_state, oid: &mut u32, oid_len: u8) -> u8 {
    if (state.status != SNMP_NEXT_OID_STATUS_BUF_TO_SMALL) {
        // let start_oid_len: u8 = (oid_len < state.start_oid_len) ? oid_len : state.start_oid_len;

        /* check passed OID is located behind start offset */
        if (snmp_oid_compare(oid, oid_len, state.start_oid, start_oid_len) >= 0) {
            /* check if new oid is located closer to start oid than current closest oid */
            if ((state.status == SNMP_NEXT_OID_STATUS_NO_MATCH)
                || (snmp_oid_compare(oid, oid_len, state.next_oid, state.next_oid_len) < 0))
            {
                return 1;
            }
        }
    }

    return 0;
}

/* checks the passed OID if it is a candidate to be the next one (get_next); returns !=0 if passed oid is currently closest, otherwise 0 */
pub fn snmp_next_oid_check(
    state: &mut snmp_next_oid_state,
    oid: &mut u32,
    oid_len: u8,
    reference: &mut Vec<u8>,
) -> u8 {
    /* do not overwrite a fail result */
    if (state.status != SNMP_NEXT_OID_STATUS_BUF_TO_SMALL) {
        /* check passed OID is located behind start offset */
        if (snmp_oid_compare(oid, oid_len, state.start_oid, state.start_oid_len) > 0) {
            /* check if new oid is located closer to start oid than current closest oid */
            if ((state.status == SNMP_NEXT_OID_STATUS_NO_MATCH)
                || (snmp_oid_compare(oid, oid_len, state.next_oid, state.next_oid_len) < 0))
            {
                if (oid_len <= state.next_oid_max_len) {
                    MEMCPY(state.next_oid, oid, oid_len * sizeof);
                    state.next_oid_len = oid_len;
                    state.status = SNMP_NEXT_OID_STATUS_SUCCESS;
                    state.reference = reference;
                    return 1;
                } else {
                    state.status = SNMP_NEXT_OID_STATUS_BUF_TO_SMALL;
                }
            }
        }
    }

    return 0;
}

pub fn snmp_oid_in_range(
    oid_in: &mut u32,
    oid_len: u8,
    oid_ranges: &mut snmp_oid_range,
    oid_ranges_len: u8,
) -> u8 {
    let i: u8;

    if (oid_len != oid_ranges_len) {
        return 0;
    }

    // for (i = 0; i < oid_ranges_len; i+= 1) {
    //   if ((oid_in[i] < oid_ranges[i].min) || (oid_in[i] > oid_ranges[i].max)) {
    //     return 0;
    //   }
    // }

    return 1;
}

pub fn snmp_set_test_ok(
    instance: &mut snmp_node_instance,
    value_len: u16,
    value: &mut Vec<u8>,
) -> snmp_err_t {
    return SNMP_ERR_NOERROR;
}

/*
 * Decodes BITS pseudotype value from ASN.1 OctetString.
 *
 * @note Because BITS pseudo type is encoded as OCTET STRING, it cannot directly
 * be encoded/decoded by the agent. Instead call this function as required from
 * get/test/set methods.
 *
 * @param buf points to a buffer holding the ASN1 octet string
 * @param buf_len length of octet string
 * @param bit_value decoded Bit value with Bit0 == LSB
 * @return ERR_OK if successful, ERR_ARG if bit value contains more than 32 bit
 */
pub fn snmp_decode_bits(buf: &mut Vec<u8>, buf_len: u32, bit_value: &mut u32) {
    let b: u8;
    let bits_processed: u8 = 0;
    *bit_value = 0;

    while (buf_len > 0) {
        /* any bit set in this byte? */
        if (*buf != 0x00) {
            if (bits_processed >= 32) {
                /* accept more than 4 bytes, but only when no bits are set */
                return ERR_VAL;
            }

            b = *buf;
            loop {
                if (b & 0x80) {
                    *bit_value |= (1 << bits_processed);
                }
                bits_processed += 1;
                b <<= 1;
                if !((bits_processed & 0x07) != 0) {
                    break;
                }
            } /* &0x07 -> % 8 */
        } else {
            bits_processed += 8;
        }

        buf_len -= 1;
        buf += 1;
    }

    return Ok(());
}

pub fn snmp_decode_truthvalue(asn1_value: &mut i32, bool_value: &mut Vec<u8>) {
    /* defined by RFC1443:
     TruthValue ::= TEXTUAL-CONVENTION
      STATUS       current
      DESCRIPTION
       "Represents a boolean value."
      SYNTAX       INTEGER { true(1), false(2) }
    */

    if ((asn1_value == None) || (bool_value == None)) {
        return ERR_ARG;
    }

    if (*asn1_value == 1) {
        *bool_value = 1;
    } else if (*asn1_value == 2) {
        *bool_value = 0;
    } else {
        return ERR_VAL;
    }

    return Ok(());
}

/*
 * Encodes BITS pseudotype value into ASN.1 OctetString.
 *
 * @note Because BITS pseudo type is encoded as OCTET STRING, it cannot directly
 * be encoded/decoded by the agent. Instead call this function as required from
 * get/test/set methods.
 *
 * @param buf points to a buffer where the resulting ASN1 octet string is stored to
 * @param buf_len max length of the bufffer
 * @param bit_value Bit value to encode with Bit0 == LSB
 * @param bit_count Number of possible bits for the bit value (according to rfc we have to send all bits independant from their truth value)
 * @return number of bytes used from buffer to store the resulting OctetString
 */
pub fn snmp_encode_bits(buf: &mut Vec<u8>, buf_len: u32, bit_value: u32, bit_count: u8) -> u8 {
    let len: u8 = 0;
    let min_bytes: u8 = (bit_count + 7) >> 3; /* >>3 -> / 8 */

    while ((buf_len > 0) && (bit_value != 0x00)) {
        let i: i8 = 7;
        *buf = 0x00;
        while (i >= 0) {
            if (bit_value & 0x01) {
                *buf |= 0x01;
            }

            if (i > 0) {
                *buf <<= 1;
            }

            bit_value >>= 1;
            i -= 1;
        }

        buf += 1;
        buf_len -= 1;
        len += 1;
    }

    if (len < min_bytes) {
        buf += len;
        buf_len -= len;

        while ((len < min_bytes) && (buf_len > 0)) {
            *buf = 0x00;
            buf += 1;
            buf_len -= 1;
            len += 1;
        }
    }

    return len;
}

pub fn snmp_encode_truthvalue(asn1_value: &mut i32, bool_value: u32) -> u8 {
    /* defined by RFC1443:
     TruthValue ::= TEXTUAL-CONVENTION
      STATUS       current
      DESCRIPTION
       "Represents a boolean value."
      SYNTAX       INTEGER { true(1), false(2) }
    */

    if (asn1_value == None) {
        return 0;
    }

    if (bool_value) {
        *asn1_value = 1; /* defined by RFC1443 */
    } else {
        *asn1_value = 2; /* defined by RFC1443 */
    }

    return sizeof;
}
