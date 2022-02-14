use crate::snmp::snmp_core_h::snmp_oid_range;
use crate::snmp::snmp_table_h::snmp_table_simple_col_def;

/*
 * @file
 * Management Information Base II (RFC1213) UDP objects and functions.
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

// #define SYNC_NODE_NAME(node_name) node_name ## _synced
// #define CREATE_LWIP_SYNC_NODE(oid, node_name) \
//    static const struct snmp_threadsync_node node_name ## _synced = SNMP_CREATE_THREAD_SYNC_NODE(oid, &node_name.node, &snmp_mib2_lwip_locks);

// #define SYNC_NODE_NAME(node_name) node_name
// #define CREATE_LWIP_SYNC_NODE(oid, node_name)

//  --- udp .1.3.6.1.2.1.7 ----------------------------------------------------- 

pub fn udp_get_value(instance: &mut snmp_node_instance, value: &mut Vec<u8>) {
    let uint_ptr: &mut u32 = value;

    match (instance.node.oid) {
        1 =>
        //  udpInDatagrams 
        {
            *uint_ptr = STATS_GET(mib2.udpindatagrams);
            return sizeof(*uint_ptr);
        }
        2 =>
        //  udpNoPorts 
        {
            *uint_ptr = STATS_GET(mib2.udpnoports);
            return sizeof(*uint_ptr);
        }
        3 =>
        //  udpInErrors 
        {
            *uint_ptr = STATS_GET(mib2.udpinerrors);
            return sizeof(*uint_ptr);
        }
        4 =>
        //  udpOutDatagrams 
        {
            *uint_ptr = STATS_GET(mib2.udpoutdatagrams);
            return sizeof(*uint_ptr);
        }

        8 => {
            //  udpHCInDatagrams 
            //  use the 32 bit counter for now... 
            let val64: u64 = STATS_GET(mib2.udpindatagrams);
            *value = val64;

            return sizeof;
        }
        9 => {
            //  udpHCOutDatagrams 
            //  use the 32 bit counter for now... 
            let val64: u64 = STATS_GET(mib2.udpoutdatagrams);
            *value = val64;

            return sizeof;
        }

        _ => {} //      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("udp_get_value(): unknown id: %"S32_F"\n", instance.node.oid));
    }

    return 0;
}

//  --- udpEndpointTable --- 

pub fn udp_endpointTable_get_cell_value_core(column: &mut u32, value: &mut snmp_variant_value) {
    //  all items except udpEndpointProcess are declared as not-accessible 
    match (*column) {
        8 => {
            //  udpEndpointProcess 
            value.u32 = 0;
        } //  not supported 

        _ => {
            return SNMP_ERR_NOSUCHINSTANCE;
        }
    }

    return SNMP_ERR_NOERROR;
}

pub fn udp_endpointTable_get_cell_value(
    column: &mut u32,
    row_oid: &mut u32,
    row_oid_len: u8,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) {
    let local_ip: LwipAddr;
    let remote_ip: LwipAddr;
    let local_port: u16;
    let remote_port: u16;
    let mut pcb: &mut udp_pcb;
    let idx: u8 = 0;

    //  udpEndpointLocalAddressType + udpEndpointLocalAddress + udpEndpointLocalPort 
    idx += snmp_oid_to_ip_port(&row_oid[idx], row_oid_len - idx, &local_ip, &local_port);
    if (idx == 0) {
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    //  udpEndpointRemoteAddressType + udpEndpointRemoteAddress + udpEndpointRemotePort 
    idx += snmp_oid_to_ip_port(&row_oid[idx], row_oid_len - idx, &remote_ip, &remote_port);
    if (idx == 0) {
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    //  udpEndpointInstance 
    if (row_oid_len < (idx + 1)) {
        return SNMP_ERR_NOSUCHINSTANCE;
    }
    if (row_oid[idx] != 0) {
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    //  find udp_pcb with requested ip and port
    pcb = udp_pcbs;
    while (pcb != None) {
        if (ip_addr_cmp(&local_ip, &pcb.local_ip)
            && (local_port == pcb.local_port)
            && ip_addr_cmp(&remote_ip, &pcb.remote_ip)
            && (remote_port == pcb.remote_port))
        {
            //  fill in object properties 
            return udp_endpointTable_get_cell_value_core(column, value);
        }
        pcb = pcb.next;
    }

    //  not found 
    return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn udp_endpointTable_get_next_cell_instance_and_value(
    column: &mut u32,
    row_oid: &mut snmp_obj_id,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) {
    let mut pcb: &mut udp_pcb;
    let state: snmp_next_oid_state;
    /* 1x udpEndpointLocalAddressType  + 1x OID len + 16x udpEndpointLocalAddress  + 1x udpEndpointLocalPort  +
     * 1x udpEndpointRemoteAddressType + 1x OID len + 16x udpEndpointRemoteAddress + 1x udpEndpointRemotePort +
     * 1x udpEndpointInstance = 39
     */
    let result_temp: [u32; 39];

    //  init struct to search next oid 
    snmp_next_oid_init(
        &state,
        row_oid.id,
        row_oid.len,
        result_temp,
        LWIP_ARRAYSIZE(result_temp),
    );

    //  iterate over all possible OIDs to find the next one 
    pcb = udp_pcbs;
    while (pcb != None) {
        let test_oid: [u32; LWIP_ARRAYSIZE(result_temp)];
        let idx: u8 = 0;

        //  udpEndpointLocalAddressType + udpEndpointLocalAddress + udpEndpointLocalPort 
        idx += snmp_ip_port_to_oid(&pcb.local_ip, pcb.local_port, &test_oid[idx]);

        //  udpEndpointRemoteAddressType + udpEndpointRemoteAddress + udpEndpointRemotePort 
        idx += snmp_ip_port_to_oid(&pcb.remote_ip, pcb.remote_port, &test_oid[idx]);

        test_oid[idx] = 0; //  udpEndpointInstance 
        idx += 1;

        //  check generated OID: is it a candidate for the next one? 
        snmp_next_oid_check(&state, test_oid, idx, None);

        pcb = pcb.next;
    }

    //  did we find a next one? 
    if (state.status == SNMP_NEXT_OID_STATUS_SUCCESS) {
        snmp_oid_assign(row_oid, state.next_oid, state.next_oid_len);
        //  fill in object properties 
        return udp_endpointTable_get_cell_value_core(column, value);
    } else {
        //  not found 
        return SNMP_ERR_NOSUCHINSTANCE;
    }
}

//  --- udpTable --- 

//  list of allowed value ranges for incoming OID 
pub const udp_Table_oid_ranges: [snmp_oid_range] = [
    snmp_oid_range::new(0, 0xff),   //  IP A        
    snmp_oid_range::new(0, 0xff),   //  IP B        
    snmp_oid_range::new(0, 0xff),   //  IP C        
    snmp_oid_range::new(0, 0xff),   //  IP D        
    snmp_oid_range::new(1, 0xffff), //  Port        
];

pub fn udp_Table_get_cell_value_core(
    pcb: &mut udp_pcb,
    column: &mut u32,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) {
    match (*column) {
        1 => {
            //  udpLocalAddress 
            //  set reference to PCB local IP and return a generic node that copies IP4 addresses 
            value.u32 = ip_2_ip4(&pcb.local_ip).addr;
        }

        2 => {
            //  udpLocalPort 
            //  set reference to PCB local port and return a generic node that copies values: u16 
            value.u32 = pcb.local_port;
        }

        _ => {
            return SNMP_ERR_NOSUCHINSTANCE;
        }
    }

    return SNMP_ERR_NOERROR;
}

pub fn udp_Table_get_cell_value(
    column: &mut u32,
    row_oid: &mut u32,
    row_oid_len: u8,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) {
    let mut if_addr: LwipAddr;
    let port: u16;
    let mut pcb: &mut udp_pcb;

    //  check if incoming OID length and if values are in plausible range 
    if (!snmp_oid_in_range(
        row_oid,
        row_oid_len,
        udp_Table_oid_ranges,
        LWIP_ARRAYSIZE(udp_Table_oid_ranges),
    )) {
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    //  get IP and port from incoming OID 
    snmp_oid_to_ip4(&row_oid[0], &ip); //  we know it succeeds because of oid_in_range check above 
    port = row_oid[4];

    //  find udp_pcb with requested ip and port
    pcb = udp_pcbs;
    while (pcb != None) {
        if (IP_IS_V4_VAL(pcb.local_ip)) {
            if (ip4_addr_cmp(&ip, ip_2_ip4(&pcb.local_ip)) && (port == pcb.local_port)) {
                //  fill in object properties 
                return udp_Table_get_cell_value_core(pcb, column, value, value_len);
            }
        }
        pcb = pcb.next;
    }

    //  not found 
    return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn udp_Table_get_next_cell_instance_and_value(
    column: &mut u32,
    row_oid: &mut snmp_obj_id,
    value: &mut snmp_variant_value,
    value_len: &mut u32,
) {
    let mut pcb: &mut udp_pcb;
    let state: snmp_next_oid_state;
    let result_temp: [u32; LWIP_ARRAYSIZE(udp_Table_oid_ranges)];

    //  init struct to search next oid 
    snmp_next_oid_init(
        &state,
        row_oid.id,
        row_oid.len,
        result_temp,
        LWIP_ARRAYSIZE(udp_Table_oid_ranges),
    );

    //  iterate over all possible OIDs to find the next one 
    pcb = udp_pcbs;
    while (pcb != None) {
        let test_oid: [u32; LWIP_ARRAYSIZE(udp_Table_oid_ranges)];

        if (IP_IS_V4_VAL(pcb.local_ip)) {
            snmp_ip4_to_oid(ip_2_ip4(&pcb.local_ip), &test_oid[0]);
            test_oid[4] = pcb.local_port;

            //  check generated OID: is it a candidate for the next one? 
            snmp_next_oid_check(&state, test_oid, LWIP_ARRAYSIZE(udp_Table_oid_ranges), pcb);
        }

        pcb = pcb.next;
    }

    //  did we find a next one? 
    if (state.status == SNMP_NEXT_OID_STATUS_SUCCESS) {
        snmp_oid_assign(row_oid, state.next_oid, state.next_oid_len);
        //  fill in object properties 
        return udp_Table_get_cell_value_core(state.reference, column, value, value_len);
    } else {
        //  not found 
        return SNMP_ERR_NOSUCHINSTANCE;
    }
}

pub const udp_inDatagrams: snmp_scalar_node =
    SNMP_SCALAR_CREATE_NODE_READONLY(1, SNMP_ASN1_TYPE_COUNTER, udp_get_value);
pub const udp_noPorts: snmp_scalar_node =
    SNMP_SCALAR_CREATE_NODE_READONLY(2, SNMP_ASN1_TYPE_COUNTER, udp_get_value);
pub const udp_inErrors: snmp_scalar_node =
    SNMP_SCALAR_CREATE_NODE_READONLY(3, SNMP_ASN1_TYPE_COUNTER, udp_get_value);
pub const udp_outDatagrams: snmp_scalar_node =
    SNMP_SCALAR_CREATE_NODE_READONLY(4, SNMP_ASN1_TYPE_COUNTER, udp_get_value);

pub const udp_HCInDatagrams: snmp_scalar_node =
    SNMP_SCALAR_CREATE_NODE_READONLY(8, SNMP_ASN1_TYPE_COUNTER64, udp_get_value);
pub const udp_HCOutDatagrams: snmp_scalar_node =
    SNMP_SCALAR_CREATE_NODE_READONLY(9, SNMP_ASN1_TYPE_COUNTER64, udp_get_value);

pub const udp_Table_columns: [snmp_table_simple_col_def] = [
    snmp_table_simple_col_def::new(1, SNMP_ASN1_TYPE_IPADDR, SNMP_VARIANT_VALUE_TYPE_U32), //  udpLocalAddress 
    snmp_table_simple_col_def::new(2, SNMP_ASN1_TYPE_INTEGER, SNMP_VARIANT_VALUE_TYPE_U32), //  udpLocalPort 
];
pub const udp_Table: snmp_table_simple_node = SNMP_TABLE_CREATE_SIMPLE(
    5,
    udp_Table_columns,
    udp_Table_get_cell_value,
    udp_Table_get_next_cell_instance_and_value,
);

pub const udp_endpointTable_columns: [snmp_table_simple_col_def] = [
  //  all items except udpEndpointProcess are declared as not-accessible 
  snmp_table_simple_col_def::new (8, SNMP_ASN1_TYPE_UNSIGNED32, SNMP_VARIANT_VALUE_TYPE_U32)]   //  udpEndpointProcess 
;

pub const udp_endpointTable: snmp_table_simple_node = SNMP_TABLE_CREATE_SIMPLE(
    7,
    udp_endpointTable_columns,
    udp_endpointTable_get_cell_value,
    udp_endpointTable_get_next_cell_instance_and_value,
);

//  the following nodes access variables in LWIP stack from SNMP worker thread and must therefore be synced to LWIP (TCPIP) thread 
// CREATE_LWIP_SYNC_NODE(1, udp_inDatagrams);
// CREATE_LWIP_SYNC_NODE(2, udp_noPorts);
// CREATE_LWIP_SYNC_NODE(3, udp_inErrors);
// CREATE_LWIP_SYNC_NODE(4, udp_outDatagrams);

// CREATE_LWIP_SYNC_NODE(5, udp_Table);

// CREATE_LWIP_SYNC_NODE(7, udp_endpointTable);

// CREATE_LWIP_SYNC_NODE(8, udp_HCInDatagrams);
// CREATE_LWIP_SYNC_NODE(9, udp_HCOutDatagrams);

// static const const: &mut snmp_node udp_nodes[] = {
//   &SYNC_NODE_NAMEudp_inDatagrams.node.node,
//   &SYNC_NODE_NAMEudp_noPorts.node.node,
//   &SYNC_NODE_NAMEudp_inErrors.node.node,
//   &SYNC_NODE_NAMEudp_outDatagrams.node.node,

//   &SYNC_NODE_NAMEudp_Table.node.node,

//   &SYNC_NODE_NAMEudp_endpointTable.node.node

//   ,
//   &SYNC_NODE_NAMEudp_HCInDatagrams.node.node,
//   &SYNC_NODE_NAMEudp_HCOutDatagrams.node.node

// };

pub const snmp_mib2_udp_root: snmp_tree_node = SNMP_CREATE_TREE_NODE(7, udp_nodes);
