use super::{icmp2::icmp_dest_unreach, icmp62::icmp6_dest_unreach};

/*
 * @file
 * ICMP API
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

// #define LWIP_HDR_ICMP_H

/* ICMP destination unreachable codes */
pub enum icmp_dur_type {
    /* net unreachable */
    ICMP_DUR_NET = 0,
    /* host unreachable */
    ICMP_DUR_HOST = 1,
    /* protocol unreachable */
    ICMP_DUR_PROTO = 2,
    /* port unreachable */
    ICMP_DUR_PORT = 3,
    /* fragmentation needed and DF set */
    ICMP_DUR_FRAG = 4,
    /* source route failed */
    ICMP_DUR_SR = 5,
}

/* ICMP time exceeded codes */
pub enum icmp_te_type {
    /* time to live exceeded in transit */
    ICMP_TE_TTL = 0,
    /* fragment reassembly time exceeded */
    ICMP_TE_FRAG = 1,
}

// pub fn  icmp_input(p: &mut PacketBuffer, inp: &mut NetIfc);
// pub fn  icmp_dest_unreach(p: &mut PacketBuffer, t: icmp_dur_type);
// pub fn  icmp_time_exceeded(p: &mut PacketBuffer, t: icmp_te_type);

pub fn icmp_port_unreach(isipv6: bool, pbuf: &mut PacketBuffer) -> bool {
    if isipv6 {
        icmp6_dest_unreach(pbuf, ICMP6_DUR_PORT)
    } else {
        icmp_dest_unreach(pbuf, ICMP_DUR_PORT)
    }
}
