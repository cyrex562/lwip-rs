/**
 * @file
 * MQTT client options
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



// #include "lwip/opt.h"




/**
 * @defgroup mqtt_opts Options
 * @ingroup mqtt
 * @{
 */

/**
 * Output ring-buffer size, must be able to fit largest outgoing publish message topic+payloads
 */

pub const MQTT_OUTPUT_RINGBUF_SIZE: u32 = 256; /**
 * Number of bytes in receive buffer, must be at least the size of the longest incoming topic + 8
 * If one wants to avoid fragmented incoming publish, set length to max incoming topic length + max payload length + 8
 */

pub const MQTT_VAR_HEADER_BUFFER_LEN: u32 = 128; /**
 * Maximum number of pending subscribe, unsubscribe and publish requests to server .
 */

pub const MQTT_REQ_MAX_IN_FLIGHT: u32 = 4; /**
 * Seconds between each cyclic timer call.
 */

pub const MQTT_CYCLIC_TIMER_INTERVAL: u32 = 5; /**
 * Publish, subscribe and unsubscribe request timeout in seconds.
 */

pub const MQTT_REQ_TIMEOUT: u32 = 30; /**
 * Seconds for MQTT connect response timeout after sending connect request
 */

pub const MQTT_CONNECT_TIMOUT: u32 = 100; /**
 * @}
 */




 /* LWIP_HDR_APPS_MQTT_OPTS_H */
