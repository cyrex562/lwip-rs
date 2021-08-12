/*
 * @file
 * SNMP pbuf stream wrapper (internal API, do not use in client code).
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
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Martin Hentschel <info@cl-soft.de>
 *
 */


// #define LWIP_HDR_APPS_SNMP_PBUF_STREAM_H












struct snmp_pbuf_stream {
  pbuf: &mut pbuf;
  offset: u16;
  length: u16;
};

pub fn  snmp_pbuf_stream_init(pbuf_stream: &mut snmp_pbuf_stream, p: &mut pbuf, offset: u16, length: u16);
pub fn  snmp_pbuf_stream_read(pbuf_stream: &mut snmp_pbuf_stream, data: &mut Vec<u8>);
pub fn  snmp_pbuf_stream_write(pbuf_stream: &mut snmp_pbuf_stream, data: u8);
pub fn  snmp_pbuf_stream_writebuf(pbuf_stream: &mut snmp_pbuf_stream, buf: &Vec<u8>, buf_len: u16);
pub fn  snmp_pbuf_stream_writeto(pbuf_stream: &mut snmp_pbuf_stream, target_pbuf_stream: &mut snmp_pbuf_stream, len: u16);
pub fn  snmp_pbuf_stream_seek(pbuf_stream: &mut snmp_pbuf_stream, i32 offset);
pub fn  snmp_pbuf_stream_seek_abs(pbuf_stream: &mut snmp_pbuf_stream, offset: u32);


}





