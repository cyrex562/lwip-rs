/*
 * @file
 * IGMP protocol definitions
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

// #define LWIP_HDR_PROT_IGMP_H

/*
 * IGMP constants
 */
pub const IGMP_TTL: u8 = 1;
pub const IGMP_MINLEN: usize = 8;
pub const ROUTER_ALERT: u32 = 0x9404;
pub const ROUTER_ALERTLEN: usize = 4;

/*
 * IGMP message types, including version number.
 */
pub const IGMP_MEMB_QUERY: u32 = 0x11; //  Membership query         
pub const IGMP_V1_MEMB_REPORT: u32 = 0x12; //  Ver. 1 membership report 
pub const IGMP_V2_MEMB_REPORT: u32 = 0x16; //  Ver. 2 membership report 
pub const IGMP_LEAVE_GROUP: u32 = 0x17; //  Leave-group message      

//  Group  membership states 
pub const IGMP_GROUP_NON_MEMBER: u32 = 0;
pub const IGMP_GROUP_DELAYING_MEMBER: u32 = 1;
pub const IGMP_GROUP_IDLE_MEMBER: u32 = 2;

/*
 * IGMP packet format.
 */

pub struct igmp_msg {
    pub msg_type: u8,
    pub max_resp: u8,
    pub checksum: u16,
    pub group_addr: u32,
}
