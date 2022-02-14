/*
 * @file
 * TCP protocol definitions
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
 * Author: Adam Dunkels <adam@sics.se>
 *
 */

//

//  Length of the TCP header, excluding options.
pub const TCP_HLEN: u32 = 20;

/* Fields are (of course) in network byte order.
 * Some fields are converted to host byte order in tcp_input().
 */

struct tcp_hdr {
    src: u16,
    dest: u16,
    seqno: u32,
    ackno: u32,
    _hdrlen_rsvd_flags: u16,
    wnd: u16,
    chksum: u16,
    urgp: u16,
}

//  TCP header flags bits
pub const TCP_FIN: u32 = 0x01;
pub const TCP_SYN: u32 = 0x02;
pub const TCP_RST: u32 = 0x04;
pub const TCP_PSH: u32 = 0x08;
pub const TCP_ACK: u32 = 0x10;
pub const TCP_URG: u32 = 0x20;
pub const TCP_ECE: u32 = 0x40;
pub const TCP_CWR: u32 = 0x80;
//  Valid TCP header flags
pub const TCP_FLAGS: u32 = 0x3f;

pub const TCP_MAX_OPTION_BYTES: usize = 40;

pub fn TCPH_HDRLEN(phdr: &tcp_hdr) -> usize {
    (lwip_ntohs((phdr)._hdrlen_rsvd_flags) >> 12)
}
pub fn TCPH_HDRLEN_BYTES(phdr: &tcp_hdr) -> usize {
    (TCPH_HDRLEN(phdr) << 2)
}
pub fn TCPH_FLAGS(phdr: &tcp_hdr) -> u8 {
    (lwip_ntohs((phdr)._hdrlen_rsvd_flags) & TCP_FLAGS)
}

pub fn TCPH_HDRLEN_SET(phdr: &mut tcp_hdr, len: usize) {
    (phdr)._hdrlen_rsvd_flags = lwip_htons(((len) << 12) | TCPH_FLAGS(phdr))
}
pub fn TCPH_FLAGS_SET(phdr: &mut tcp_hdr, flags: u16) {
    (phdr)._hdrlen_rsvd_flags =
        (((phdr)._hdrlen_rsvd_flags & PP_HTONS(!TCP_FLAGS)) | lwip_htons(flags))
}
pub fn TCPH_HDRLEN_FLAGS_SET(phdr: &mut tcp_hdr, len: usize, flags: u16) {
    (phdr)._hdrlen_rsvd_flags = (lwip_htons(((len) << 12) | (flags)))
}

pub fn TCPH_SET_FLAG(phdr: &mut tcp_hdr, flags: u16) {
    (phdr)._hdrlen_rsvd_flags = ((phdr)._hdrlen_rsvd_flags | lwip_htons(flags))
}
pub fn TCPH_UNSET_FLAG(phdr: &mut tcp_hdr, flags: u16) {
    (phdr)._hdrlen_rsvd_flags = ((phdr)._hdrlen_rsvd_flags & !lwip_htons(flags))
}
