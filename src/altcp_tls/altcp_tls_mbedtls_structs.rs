/*
 * @file
 * Application layered TCP/TLS connection API (to be used from TCPIP thread)
 *
 * This file contains structure definitions for a TLS layer using mbedTLS.
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt
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

// #define LWIP_HDR_ALTCP_MBEDTLS_STRUCTS_H







// TODO
pub struct mbedtls_ssl_context {

}









use crate::core::pbuf_h::PacketBuffer;

pub const ALTCP_MBEDTLS_FLAGS_HANDSHAKE_DONE: u8 = 0x01;
pub const ALTCP_MBEDTLS_FLAGS_UPPER_CALLED: u8 = 0x02;
pub const ALTCP_MBEDTLS_FLAGS_RX_CLOSE_QUEUED: u8 =  0x04;
pub const ALTCP_MBEDTLS_FLAGS_RX_CLOSED: u8 =        0x08;
pub const ALTCP_MBEDTLS_FLAGS_APPLDATA_SENT: u8 =    0x10;

pub struct altcp_mbedtls_state {
  pub conf: Vec<u8>,
  pub  ssl_context: mbedtls_ssl_context,
  /* chain of rx pbufs (before decryption) */
  pub rx: PacketBuffer,
  pub rx_app: Option<PacketBuffer>,
  pub flags: u8,
  pub rx_passed_unrecved: i32,
  pub bio_bytes_read: i32,
  pub bio_bytes_appl: i32,
}

impl altcp_mbedtls_state {
    pub fn new() -> altcp_mbedtls_state {
        altcp_mbedtls_state {
            conf: Vec::new(),
            ssl_context: mbedtls_ssl_context{},
            rx: PacketBuffer::new(),
            rx_app: PacketBuffer::new(),
            flags: 0,
            rx_passed_unrecved: 0,
            bio_bytes_read: 0,
            bio_bytes_appl: 0,
        }
    }
}






