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

// #define LWIP_HDR_APPS_MQTT_PRIV_H








/* Pending request item, binds application callback to pending server requests */
struct mqtt_request_t
{
  /* Next item in list, NULL means this is the last in chain,
      next pointing at itself means request is unallocated */
  next: &mut mqtt_request_t;
  /* Callback to upper layer */
  mqtt_request_cb_t cb;
  arg: &mut Vec<u8>;
  /* MQTT packet identifier */
  let pkt_id: u16;
  /* Expire time relative to element before this  */
  let timeout_diff: u16;
};

/* Ring buffer */
struct mqtt_ringbuf_t {
  let put: u16;
  let get: u16;
  buf: [u8;MQTT_OUTPUT_RINGBUF_SIZE];
};

/* MQTT client */
struct mqtt_client_s
{
  /* Timers and timeouts */
  let cyclic_tick: u16;
  let keep_alive: u16;
  let server_watchdog: u16;
  /* Packet identifier generator*/
  let pkt_id_seq: u16;
  /* Packet identifier of pending incoming publish */
  let inpub_pkt_id: u16;
  /* Connection state */
  let conn_state: u8;
   let conn: &mut AlTcpPcb;
  /* Connection callback */
  connect_arg: &mut ();
  mqtt_connection_cb_t connect_cb;
  /* Pending requests to server */
  pend_req_queue: &mut mqtt_request_t;
  struct mqtt_request_t req_list[MQTT_REQ_MAX_IN_FLIGHT];
  inpub_arg: &mut ();
  /* Incoming data callback */
  mqtt_incoming_data_cb_t data_cb;
  mqtt_incoming_publish_cb_t pub_cb;
  /* Input */
  let msg_idx: u32;
  rx_buffer: [u8;MQTT_VAR_HEADER_BUFFER_LEN];
  /* Output ring-buffer */
  struct mqtt_ringbuf_t output;
};


}



