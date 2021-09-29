/*
 * @file
 * ND6 protocol definitions
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

// #define LWIP_HDR_PROT_ND6_H

//  Neighbor solicitation message header. 

pub struct NeighSolicitHeader {
    pub msg_type: u8,
    code: u8,
    chksum: u16,
    reserved: u32,
    target_address: ip6_addr_p_t,
    //  Options follow. 
}

//  Neighbor advertisement message header. 

pub struct NeighAdvertHeader {
    pub msg_type: u8,
    pub code: u8,
    pub chksum: u16,
    pub flags: u8,
    pub reserved: [u8; 3],
    pub target_address: ip6_addr_p_t,
    //  Options follow. 
}

pub const ND6_FLAG_ROUTER: u8 = (0x80);
pub const ND6_FLAG_SOLICITED: u8 = (0x40);
pub const ND6_FLAG_OVERRIDE: u8 = (0x20);

//  Router solicitation message header. 

pub struct RtrSolicitHeader {
    pub msg_type: u8,
    pub code: u8,
    pub chksum: u16,
    pub reserved: u32,
    //  Options follow. 
}

//  Router advertisement message header. 
pub const ND6_RA_FLAG_MANAGED_ADDR_CONFIG: u8 = (0x80);
pub const ND6_RA_FLAG_OTHER_CONFIG: u8 = (0x40);
pub const ND6_RA_FLAG_HOME_AGENT: u8 = (0x20);
pub const ND6_RA_PREFERENCE_MASK: u8 = (0x18);
pub const ND6_RA_PREFERENCE_HIGH: u8 = (0x08);
pub const ND6_RA_PREFERENCE_MEDIUM: u8 = (0x00);
pub const ND6_RA_PREFERENCE_LOW: u8 = (0x18);
pub const ND6_RA_PREFERENCE_DISABLED: u8 = (0x10);

pub struct RtrAdvertHeader {
    pub msg_type: u8,
    pub code: u8,
    pub chksum: u16,
    pub current_hop_limit: u8,
    pub flags: u8,
    pub router_lifetime: u16,
    pub reachable_time: u32,
    pub retrans_timer: u32,
    //  Options follow. 
}

//  Redirect message header. 

pub struct RedirectHeader {
    pub msg_type: u8,
    pub code: u8,
    pub chksum: u16,
    pub reserved: u32,
    pub target_address: ip6_addr_p_t,
    pub destination_address: ip6_addr_p_t,
    //  Options follow. 
}

//  Link-layer address option. 
pub const ND6_OPTION_TYPE_SOURCE_LLADDR: u8 = (0x01);
pub const ND6_OPTION_TYPE_TARGET_LLADDR: u8 = (0x02);

pub struct LLAddrOption {
    pub msg_type: u8,
    pub length: u8,
    pub addr: [u8; NETIF_MAX_HWADDR_LEN],
}

//  Prefix information option. 
pub const ND6_OPTION_TYPE_PREFIX_INFO: u8 = (0x03);
pub const ND6_PREFIX_FLAG_ON_LINK: u8 = (0x80);
pub const ND6_PREFIX_FLAG_AUTONOMOUS: u8 = (0x40);
pub const ND6_PREFIX_FLAG_ROUTER_ADDRESS: u8 = (0x20);
pub const ND6_PREFIX_FLAG_SITE_PREFIX: u8 = (0x10);

pub struct PrefixOption {
    pub msg_type: u8,
    pub length: u8,
    pub prefix_length: u8,
    pub flags: u8,
    pub valid_lifetime: u32,
    pub preferred_lifetime: u32,
    pub reserved2: [u8; 3],
    pub site_prefix_length: u8,
    pub prefix: ip6_addr_p_t,
}

//  Redirected header option. 
pub const ND6_OPTION_TYPE_REDIR_HDR: u8 = (0x04);

pub struct RedirectedHeaderOption {
    pub msg_type: u8,
    pub length: u8,
    pub reserved: [u8; 6],
    //  Portion of redirected packet follows. 
    //  (redirected: [u8;8]); 
}

//  MTU option. 
pub const ND6_OPTION_TYPE_MTU: u8 = (0x05);

pub struct MtuOption {
    pub msg_type: u8,
    pub length: u8,
    pub reserved: u16,
    pub mtu: u32,
}

//  Route information option. 
pub const ND6_OPTION_TYPE_ROUTE_INFO: u8 = (24);

struct route_option {
    pub msg_type: u8,
    pub length: u8,
    pub prefix_length: u8,
    pub preference: u8,
    pub route_lifetime: u32,
    pub prefix: ip6_addr_p_t,
}

//  Recursive DNS Server Option. 
pub const ND6_OPTION_TYPE_RDNSS: u8 = (25);

pub struct RDNSSOption {
    pub msg_type: u8,
    pub length: u8,
    pub reserved: u16,
    pub lifetime: u32,
    pub rdnss_address: [ip6_addr_p_t; 1],
}

pub const SIZEOF_RDNSS_OPTION_BASE: u32 = 8; //  size without addresses 
