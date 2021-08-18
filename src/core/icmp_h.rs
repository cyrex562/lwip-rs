/*
 * @file
 * ICMP protocol definitions
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

// #define LWIP_HDR_PROT_ICMP_H

pub const ICMP_ER: u32 = 0; /* echo reply */
pub const ICMP_DUR: u32 = 3; /* destination unreachable */
pub const ICMP_SQ: u32 = 4; /* source quench */
pub const ICMP_RD: u32 = 5; /* redirect */
pub const ICMP_ECHO: u32 = 8; /* echo */
pub const ICMP_TE: u32 = 11; /* time exceeded */
pub const ICMP_PP: u32 = 12; /* parameter problem */
pub const ICMP_TS: u32 = 13; /* timestamp */
pub const ICMP_TSR: u32 = 14; /* timestamp reply */
pub const ICMP_IRQ: u32 = 15; /* information request */
pub const ICMP_IR: u32 = 16; /* information reply */
pub const ICMP_AM: u32 = 17; /* address mask request */
pub const ICMP_AMR: u32 = 18; /* address mask reply */

/* This is the standard ICMP header only that the u32 data
 *  is split to two like: u16 ICMP echo needs it.
 *  This header is also used for other ICMP types that do not
 *  use the data part.
 */

struct icmp_echo_hdr {
    echo_type: u8,
    code: u8,
    chksum: u16,
    id: u16,
    seqno: u16,
}

impl icmp_echo_hdr {
    pub fn ICMPH_TYPE(self: &Self) -> u8 {
        self.echo_type
    }

    pub fn ICMP_CODE(self: &Self) -> u8 {
        self.code
    }

    pub fn ICMPH_TYPE_SET(self: &Self, t: u8) {
        self.echo_type = t
    }

    pub fn ICMPH_CODE_SET(self: &Self, c: u8) {
        self.code = c
    }
}
