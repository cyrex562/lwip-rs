/*
 * @file
 * SNMP RAW API frontend.
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 */

//  lwIP UDP receive callback function 
pub fn snmp_recv(
    arg: &mut Vec<u8>,
    pcb: &mut udp_pcb,
    p: &mut PacketBuffer,
    addr: &mut LwipAddr,
    port: u16,
) {
    snmp_receive(pcb, p, addr, port);

    pbuf_free(p);
}

pub fn snmp_sendto(handle: &mut Vec<u8>, p: &mut PacketBuffer, dst: &mut LwipAddr, port: u16) {
    return udp_sendto(handle, p, dst, port);
}

pub fn snmp_get_local_ip_for_dst(
    handle: &mut Vec<u8>,
    dst: &mut LwipAddr,
    result: &mut LwipAddr,
) -> u8 {
    let udp_pcb: &mut udp_pcb = handle;
    let mut dst_if: &mut NetIfc;
    let mut dst_ip: &mut LwipAddr;

    //  unused in case of IPV4 only configuration 

    ip_route_get_local_ip(&udp_pcb.local_ip, dst, dst_if, dst_ip);

    if ((dst_if != None) && (dst_ip != None)) {
        ip_addr_copy(*result, *dst_ip);
        return 1;
    } else {
        return 0;
    }
}

/*
 * @ingroup snmp_core
 * Starts SNMP Agent.
 * Allocates UDP pcb and binds it to IP_ANY_TYPE port 161.
 */
pub fn snmp_init() {
    let err: err_t;
    let snmp_pcb: &mut udp_pcb = udp_new_ip_type(IPADDR_TYPE_ANY);
    // LWIP_ERROR("snmp_raw: no PCB", (snmp_pcb != None), return;);

    LWIP_ASSERT_CORE_LOCKED();

    snmp_traps_handle = snmp_pcb;

    udp_recv(snmp_pcb, snmp_recv, None);
    err = udp_bind(snmp_pcb, IP_ANY_TYPE, LWIP_IANA_PORT_SNMP);
    // LWIP_ERROR("snmp_raw: Unable to bind PCB", (err == ERR_OK), return;);
}
