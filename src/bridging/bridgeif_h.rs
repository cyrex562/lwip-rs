/*
 * @file
 * lwIP netif implementing an IEEE 802.1D MAC Bridge
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt.
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 */

// #define LWIP_HDR_NETIF_BRIDGEIF_H

// #error BRIDGEIF_MAX_PORTS must be [1..63]
// #elif BRIDGEIF_MAX_PORTS < 8
// typedef bridgeif_portmask_t: u8;
// #elif BRIDGEIF_MAX_PORTS < 16
// typedef bridgeif_portmask_t: u16;
// #elif BRIDGEIF_MAX_PORTS < 32
// typedef bridgeif_portmask_t: u32;
// #elif BRIDGEIF_MAX_PORTS < 64
// typedef bridgeif_portmask_t: u64;

pub const BR_FLOOD: u32 = ((bridgeif_portmask_t) - 1);

/* @ingroup bridgeif
 * Initialisation data for @ref bridgeif_init.
 * An instance of this type must be passed as parameter 'state' to @ref netif_add
 * when the bridge is added.
 */
pub struct bridgeif_initdata_t {
    //  MAC address of the bridge (cannot use the netif's addresses) 
    pub ethaddr: LwipAddr,
    /* Maximum number of ports in the bridge (ports are stored in an array, this
    influences memory allocated for netif.state of the bridge netif). */
    pub max_ports: usize,
    /* Maximum number of dynamic/learning entries in the bridge's forwarding database.
    In the default implementation, this controls memory consumption only. */
    pub max_fdb_dynamic_entries: usize,
    //  Maximum number of static forwarding entries. Influences memory consumption! 
    pub max_fdb_static_entries: usize,
}

/* @ingroup bridgeif
 * Use this for constant initialization of a bridgeif_initdat_t
 * (ethaddr must be passed as ETH_ADDR())
 */
// #define BRIDGEIF_INITDATA1(max_ports, max_fdb_dynamic_entries, max_fdb_static_entries, ethaddr) {ethaddr, max_ports, max_fdb_dynamic_entries, max_fdb_static_entries}
/* @ingroup bridgeif
 * Use this for constant initialization of a bridgeif_initdat_t
 * (each byte of ethaddr must be passed)
 */
// #define BRIDGEIF_INITDATA2(max_ports, max_fdb_dynamic_entries, max_fdb_static_entries, e0, e1, e2, e3, e4, e5) {{e0, e1, e2, e3, e4, e5}, max_ports, max_fdb_dynamic_entries, max_fdb_static_entries}

// pub fn  bridgeif_init(netif: &mut NetIfc);
// pub fn  bridgeif_add_port(bridgeif: &mut NetIfc, portif: &mut NetIfc);
// pub fn  bridgeif_fdb_add(bridgeif: &mut NetIfc,  addr: &mut MacAddress, bridgeif_portmask_t ports);
// pub fn  bridgeif_fdb_remove(bridgeif: &mut NetIfc,  addr: &mut MacAddress);

//  FDB interface, can be replaced by own implementation 
// pub fn                 bridgeif_fdb_update_src(fdb_ptr: &mut Vec<u8>, src_addr: &mut MacAddress, port_idx: u8);
// bridgeif_portmask_t bridgeif_fdb_get_dst_ports(fdb_ptr: &mut Vec<u8>, dst_addr: &mut MacAddress);
// pub fn *               bridgeif_fdb_init(max_fdb_entries: u16);

//  define bridgeif protection to sys_arch_protect... 

// #define BRIDGEIF_DECL_PROTECT(lev)    SYS_ARCH_DECL_PROTECT(lev)
// #define BRIDGEIF_READ_PROTECT(lev)    SYS_ARCH_PROTECT(lev)
// #define BRIDGEIF_READ_UNPROTECT(lev)  SYS_ARCH_UNPROTECT(lev)
// #define BRIDGEIF_WRITE_PROTECT(lev)
// #define BRIDGEIF_WRITE_UNPROTECT(lev)

//  BRIDGEIF_PORT_NETIFS_OUTPUT_DIRECT 

// #define BRIDGEIF_DECL_PROTECT(lev)
// #define BRIDGEIF_READ_PROTECT(lev)
// #define BRIDGEIF_READ_UNPROTECT(lev)
// #define BRIDGEIF_WRITE_PROTECT(lev)
// #define BRIDGEIF_WRITE_UNPROTECT(lev)
