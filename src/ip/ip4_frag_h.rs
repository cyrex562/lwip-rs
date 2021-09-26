/*
 * @file
 * IP fragmentation/reassembly
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
 * Author: Jani Monoses <jani@iv.ro>
 *
 */

use crate::packetbuffer::pbuf_h::{pbuf_custom, PacketBuffer};
use crate::ip::ip4_h::Ip4Header;

/// The IP reassembly timer interval in milliseconds.
pub const IP_TMR_INTERVAL: u64 = 1000;

/// IP reassembly helper struct.
/// This is exported because memp needs to know the size.
#[derive(Clone, Debug, Default)]
pub struct Ip4ReassemblyData {
    pkt_buf: PacketBuffer,
    ip4_hdr: Ip4Header,
    datagram_len: usize,
    flags: u8,
    pub(crate) timer: u64,
}

impl Ip4ReassemblyData {
    pub fn new() -> Ip4ReassemblyData {
        Ip4ReassemblyData {
            pkt_buf: PacketBuffer::default(),
            ip4_hdr: Default::default(),
            datagram_len: 0,
            flags: 0,
            timer: 0
        }
    }
}

// ip4_reass: &mut PacketBuffer(p: &mut PacketBuffer);

/// A custom pbuf that holds a reference to another pbuf, which is freed
/// when this custom pbuf is freed. This is used to create a custom PBUF_REF
/// that points into the original pbuf. */
pub struct PbufCustomRef {
    /// 'base class'
    pub pc: PacketBuffer_custom,
    /// pointer to the original pbuf that is referenced
    pub original: PacketBuffer,
}
