/*
 * @file
 * MQTT client (private interface)
 */

/*
 * Copyright (c) 2016 Erik Andersson
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
 * Author: Erik Andersson
 *
 */

//

//  Pending request item, binds application callback to pending server requests
pub struct MqttRequest {
    /* Next item in list, NULL means this is the last in chain,
    next pointing at itself means request is unallocated */
    // next: &mut mqtt_request_t;
    //  Callback to upper layer
    pub cb: mqtt_request_cb_t,
    pub arg: Vec<u8>,
    //  MQTT packet identifier
    pub pkt_id: u16,
    //  Expire time relative to element before this
    pub timeout_diff: u16,
}

//  Ring buffer
pub struct MqttRingbuf {
    pub put: u16,
    pub get: u16,
    buf: Vec<u8>,
}

//  MQTT client
pub struct MqttClient {
    //  Timers and timeouts
    pub cyclic_tick: u16,
    pub keep_alive: u16,
    pub server_watchdog: u16,
    //  Packet identifier generator
    pub pkt_id_seq: u16,
    //  Packet identifier of pending incoming publish
    pub inpub_pkt_id: u16,
    //  Connection state
    pub conn_state: u8,
    pub conn: &mut AlTcpPcb,
    //  Connection callback
    pub connect_arg: Vec<u8>,
    pub connect_cb: mqtt_connection_cb_t,
    //  Pending requests to server
    pub pend_req_queue: &mut MqttRequest,
    pub req_list: Vec<MqttRequest>,
    inpub_arg: Vec<u8>,
    //  Incoming data callback
    pub data_cb: mqtt_incoming_data_cb_t,
    pub pub_cb: mqtt_incoming_publish_cb_t,
    //  Input
    pub msg_idx: u32,
    pub rx_buffer: Vec<u8>,
    //  Output ring-buffer
    pub output: MqttRingbuf,
}
