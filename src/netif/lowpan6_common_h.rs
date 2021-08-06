/*
 * @file
 *
 * Common 6LowPAN routines for IPv6. Uses ND tables for link-layer addressing. Fragments packets to 6LowPAN units.
 */

/*
 * Copyright (c) 2015 Inico Technologies Ltd.
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
 * Author: Ivan Delamer <delamer@inicotech.com>
 *
 *
 * Please coordinate changes and requests with Ivan Delamer
 * <delamer@inicotech.com>
 */


// #define LWIP_HDR_LOWPAN6_COMMON_H














/* Helper define for a link layer address, which can be encoded as 0, 2 or 8 bytes */
struct lowpan6_link_addr {
  /* encoded length of the address */
  addr_len: u8;
  /* address bytes */
  addr: u8[8];
};

s8_t lowpan6_get_address_mode(const ip6addr: &mut ip6_addr_t,  mac_addr: &mut lowpan6_link_addr);


pub fn  lowpan6_compress_headers(netif: &mut netif, u8 *inbuf, inbuf_size: usize, u8 *outbuf, outbuf_size: usize,
                               u8 *lowpan6_header_len_out, u8 *hidden_header_len_out, lowpan6_contexts: &mut ip6_addr_t,
                               const src: &mut lowpan6_link_addr,  dst: &mut lowpan6_link_addr);
lowpan6_decompress: &mut pbuf(p: &mut pbuf, datagram_size: u16, lowpan6_contexts: &mut ip6_addr_t,
                                src: &mut lowpan6_link_addr, dest: &mut lowpan6_link_addr);



}





