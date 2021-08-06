/*
 * @file
 * MQTT client
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

// #define LWIP_HDR_APPS_MQTT_CLIENT_H

// typedef struct mqtt_client_s mqtt_client_t;

// struct altcp_tls_config;

/* @ingroup mqtt
 * Default MQTT port (non-TLS) */
pub const MQTT_PORT: u16 = LWIP_IANA_PORT_MQTT;
/* @ingroup mqtt
 * Default MQTT TLS port */
pub const MQTT_TLS_PORT: u16 = LWIP_IANA_PORT_SECURE_MQTT;

/*---------------------------------------------------------------------------------------------- */
/* Connection with server */

/*
 * @ingroup mqtt
 * Client information and connection parameters */
pub struct MqttConnectClientInfo {
    /* Client identifier, must be set by caller */
    client_id: String,
    /* User name, set to NULL if not used */
    client_user: String,
    /* Password, set to NULL if not used */
    client_pass: String,
    /* keep alive time in seconds, 0 to disable keep alive functionality*/
    keep_alive: u16,
    /* will topic, set to NULL if will is not to be used,
    will_msg, will_qos and will retain are then ignored */
    will_topic: String,
    /* will_msg, see will_topic */
    will_msg: String,
    /* will_qos, see will_topic */
    will_qos: u8,
    /* will_retain, see will_topic */
    will_retain: u8,

    /* TLS configuration for secure connections */
    tls_config: altcp_tls_config,
}

/*
 * @ingroup mqtt
 * Connection status codes */
// typedef enum
// {
//   /* Accepted */
//   MQTT_CONNECT_ACCEPTED                 = 0,
//   /* Refused protocol version */
//   MQTT_CONNECT_REFUSED_PROTOCOL_VERSION = 1,
//   /* Refused identifier */
//   MQTT_CONNECT_REFUSED_IDENTIFIER       = 2,
//   /* Refused server */
//   MQTT_CONNECT_REFUSED_SERVER           = 3,
//   /* Refused user credentials */
//   MQTT_CONNECT_REFUSED_USERNAME_PASS    = 4,
//   /* Refused not authorized */
//   MQTT_CONNECT_REFUSED_NOT_AUTHORIZED_  = 5,
//   /* Disconnected */
//   MQTT_CONNECT_DISCONNECTED             = 256,
//   /* Timeout */
//   MQTT_CONNECT_TIMEOUT                  = 257
// } mqtt_connection_status_t;

/* Accepted */
pub const MQTT_CONNECT_ACCEPTED: u16 = 0;
/* Refused protocol version */
pub const MQTT_CONNECT_REFUSED_PROTOCOL_VERSION: u16 = 1;
/* Refused identifier */
pub const MQTT_CONNECT_REFUSED_IDENTIFIER: u16 = 2;
/* Refused server */
pub const MQTT_CONNECT_REFUSED_SERVER: u16 = 3;
/* Refused user credentials */
pub const MQTT_CONNECT_REFUSED_USERNAME_PASS: u16 = 4;
/* Refused not authorized */
pub const MQTT_CONNECT_REFUSED_NOT_AUTHORIZED_: u16 = 5;
/* Disconnected */
pub const MQTT_CONNECT_DISCONNECTED: u16 = 256;
/* Timeout */
pub const MQTT_CONNECT_TIMEOUT: u16 = 257;

/*
 * @ingroup mqtt
 * Function prototype for mqtt connection status callback. Called when
 * client has connected to the server after initiating a mqtt connection attempt by
 * calling mqtt_client_connect() or when connection is closed by server or an error
 *
 * @param client MQTT client itself
 * @param arg Additional argument to pass to the callback function
 * @param status Connect result code or disconnection notification @see mqtt_connection_status_t
 *
 */
// typedef void (*mqtt_connection_cb_t)(client: &mut mqtt_client_t, arg: &mut Vec<u8>, mqtt_connection_status_t status);
type mqtt_connection_cb_t =
    fn(client: &mut mqtt_client_t, arg: &mut Vec<u8>, status: mqtt_status_t);

/*
 * @ingroup mqtt
 * Data callback flags */
// enum {
//   /* Flag set when last fragment of data arrives in data callback */
//   MQTT_DATA_FLAG_LAST = 1
// };

pub const MQTT_DATA_FLAG_LAST: u32 = 1;

/*
* @ingroup mqtt
* Function prototype for MQTT incoming publish data callback function. Called when data
* arrives to a subscribed topic @see mqtt_subscribe
*
* @param arg Additional argument to pass to the callback function
* @param data User data, pointed object, data may not be referenced after callback return,
         NULL is passed when all publish data are delivered
* @param len Length of publish data fragment
* @param flags MQTT_DATA_FLAG_LAST set when this call contains the last part of data from publish message
*
*/
// typedef void (*mqtt_incoming_data_cb_t)(arg: &mut Vec<u8>, const u8 *data, len: u16, flags: u8);
type mqtt_incoming_data_cb_t = fn(arg: &mut Vec<u8>, data: &Vec<u8>, len: u16, flags: u8);

/*
 * @ingroup mqtt
 * Function prototype for MQTT incoming publish function. Called when an incoming publish
 * arrives to a subscribed topic @see mqtt_subscribe
 *
 * @param arg Additional argument to pass to the callback function
 * @param topic Zero terminated Topic text string, topic may not be referenced after callback return
 * @param tot_len Total length of publish data, if set to 0 (no publish payload) data callback will not be invoked
 */
// typedef void (*mqtt_incoming_publish_cb_t)(arg: &mut Vec<u8>, topic: &String, tot_len: u32);
type mqtt_incoming_publish_cb_t = fn(arg: &mut Vec<u8>, topic: &String, tot_len: u32);

/*
 * @ingroup mqtt
 * Function prototype for mqtt request callback. Called when a subscribe, unsubscribe
 * or publish request has completed
 * @param arg Pointer to user data supplied when invoking request
 * @param err ERR_OK on success
 *            ERR_TIMEOUT if no response was received within timeout,
 *            ERR_ABRT if (un)subscribe was denied
 */
// typedef void (*mqtt_request_cb_t)(arg: &mut Vec<u8>, err: err_t);
type mqtt_request_cb_t = fn(arg: &mut Vec<u8>, err: err_t);

// pub fn  mqtt_client_connect(client: &mut mqtt_client_t, const ipaddr: &mut ip_addr_t, port: u16, mqtt_connection_cb_t cb, arg: &mut Vec<u8>,
//                    const client_info: &mut MqttConnectClientInfo);

//pub fn  mqtt_disconnect(client: &mut mqtt_client_t);

// mqtt_client_new: &mut mqtt_client_t();
// pub fn  mqtt_client_free(mqtt_client_t* client);

// mqtt_client_is_connected: u8(client: &mut mqtt_client_t);

// pub fn  mqtt_set_inpub_callback(client: &mut mqtt_client_t, mqtt_incoming_publish_cb_t,
//                              mqtt_incoming_data_cb_t data_cb, arg: &mut Vec<u8>);

// pub fn  mqtt_sub_unsub(client: &mut mqtt_client_t, topic: &String, qos: u8, mqtt_request_cb_t cb, arg: &mut Vec<u8>, sub: u8);

/* @ingroup mqtt
 *Subscribe to topic */
// #define mqtt_subscribe(client, topic, qos, cb, arg) mqtt_sub_unsub(client, topic, qos, cb, arg, 1)
// type mqtt_unsubscribe = mqtt_sub_unsub;
/* @ingroup mqtt
 *  Unsubscribe to topic */
// #define mqtt_unsubscribe(client, topic, cb, arg) mqtt_sub_unsub(client, topic, 0, cb, arg, 0)
// type mqtt_subscribe = mqtt_sub_unsub;

// pub fn  mqtt_publish(client: &mut mqtt_client_t, topic: &String, payload: &Vec<u8>, payload_length: u16, qos: u8, retain: u8,
//                                     mqtt_request_cb_t cb, arg: &mut Vec<u8>);

// }
