/*
 * @file
 * DHCP protocol definitions
 */

/*
 * Copyright (c) 2001-2004 Leon Woestenberg <leon.woestenberg@gmx.net>
 * Copyright (c) 2001-2004 Axon Digital Design B.V., The Netherlands.
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
 * Author: Leon Woestenberg <leon.woestenberg@gmx.net>
 *
 */

// #define LWIP_HDR_PROT_DHCP_H

/* DHCP message item offsets and length */
pub const DHCP_CHADDR_LEN: usize = 16;
pub const DHCP_SNAME_OFS: usize = 44;
pub const DHCP_SNAME_LEN: usize = 64;
pub const DHCP_FILE_OFS: usize = 108;
pub const DHCP_FILE_LEN: usize = 128;
pub const DHCP_MSG_LEN: usize = 236;
pub const DHCP_OPTIONS_OFS: usize = (DHCP_MSG_LEN + 4); /* 4 byte: cookie */

pub const DHCP_MIN_OPTIONS_LEN: usize = 68;
pub const DHCP_OPTIONS_LEN: usize = DHCP_MIN_OPTIONS_LEN;

/* minimum set of fields of any DHCP message */
struct dhcp_msg {
    pub op: u8,
    pub htype: u8,
    pub hlen: u8,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: u16,
    pub ciaddr: ip4_addr_p_t,
    pub yiaddr: ip4_addr_p_t,
    pub siaddr: ip4_addr_p_t,
    pub giaddr: ip4_addr_p_t,
    pub chaddr: [u8; DHCP_CHADDR_LEN],
    pub sname: [u8; DHCP_SNAME_LEN],
    pub file: [u8; DHCP_FILE_LEN],
    pub cookie: u32,

    /* make sure user does not configure this too small */

    // #  undef DHCP_OPTIONS_LEN

    /* allow this to be configured in lwipopts.h, but not too small */

    /* set this to be sufficient for your options in outgoing DHCP msgs */
    pub options: [u8; DHCP_OPTIONS_LEN],
}

/* DHCP client states */
pub enum DhcpState {
    DhcpStateOff = 0,
    DhcpStateRequesting = 1,
    DhcpStateInit = 2,
    DhcpStateRebooting = 3,
    DhcpStateRebinding = 4,
    DhcpStateRenewing = 5,
    DhcpStateSelecting = 6,
    DhcpStateInforming = 7,
    DhcpStateChecking = 8,
    DhcpStatePermanent = 9, /* not yet implemented */
    DhcpStateBound = 10,
    DhcpStateReleasing = 11, /* not yet implemented */
    DhcpStateBackingOff = 12,
}

/* DHCP op codes */
pub const DHCP_BOOTREQUEST: u32 = 1;
pub const DHCP_BOOTREPLY: u32 = 2;

/* DHCP message types */
pub const DHCP_DISCOVER: u32 = 1;
pub const DHCP_OFFER: u32 = 2;
pub const DHCP_REQUEST: u32 = 3;
pub const DHCP_DECLINE: u32 = 4;
pub const DHCP_ACK: u32 = 5;
pub const DHCP_NAK: u32 = 6;
pub const DHCP_RELEASE: u32 = 7;
pub const DHCP_INFORM: u32 = 8;

pub const DHCP_MAGIC_COOKIE: u32 = 0x63825363;

/* This is a list of options for BOOTP and DHCP, see RFC 2132 for descriptions */

/* BootP options */
pub const DHCP_OPTION_PAD: u32 = 0;
pub const DHCP_OPTION_SUBNET_MASK: u32 = 1; /* RFC 2132 3.3 */
pub const DHCP_OPTION_ROUTER: u32 = 3;
pub const DHCP_OPTION_DNS_SERVER: u32 = 6;
pub const DHCP_OPTION_HOSTNAME: u32 = 12;
pub const DHCP_OPTION_IP_TTL: u32 = 23;
pub const DHCP_OPTION_MTU: u32 = 26;
pub const DHCP_OPTION_BROADCAST: u32 = 28;
pub const DHCP_OPTION_TCP_TTL: u32 = 37;
pub const DHCP_OPTION_NTP: u32 = 42;
pub const DHCP_OPTION_END: u32 = 255;

/* DHCP options */
pub const DHCP_OPTION_REQUESTED_IP: u32 = 50; /* RFC 2132 9.1, requested IP address */
pub const DHCP_OPTION_LEASE_TIME: u64 = 51; /* RFC 2132 9.2, time in seconds, in 4 bytes */
pub const DHCP_OPTION_OVERLOAD: u32 = 52; /* RFC2132 9.3, use file and/or sname field for options */

pub const DHCP_OPTION_MESSAGE_TYPE: u32 = 53; /* RFC 2132 9.6, important for DHCP */
pub const DHCP_OPTION_MESSAGE_TYPE_LEN: usize = 1;

pub const DHCP_OPTION_SERVER_ID: u32 = 54; /* RFC 2132 9.7, server IP address */
pub const DHCP_OPTION_PARAMETER_REQUEST_LIST: u32 = 55; /* RFC 2132 9.8, requested option types */

pub const DHCP_OPTION_MAX_MSG_SIZE: usize = 57; /* RFC 2132 9.10, message size accepted >= 576 */
pub const DHCP_OPTION_MAX_MSG_SIZE_LEN: usize = 2;

pub const DHCP_OPTION_T1: u64 = 58; /* T1 renewal time */
pub const DHCP_OPTION_T2: u64 = 59; /* T2 rebinding time */
pub const DHCP_OPTION_US: u64 = 60;
pub const DHCP_OPTION_CLIENT_ID: u64 = 61;
pub const DHCP_OPTION_TFTP_SERVERNAME: u64 = 66;
pub const DHCP_OPTION_BOOTFILE: u64 = 67;

/* possible combinations of overloading the file and sname fields with options */
pub const DHCP_OVERLOAD_NONE: u32 = 0;
pub const DHCP_OVERLOAD_FILE: u32 = 1;
pub const DHCP_OVERLOAD_SNAME: u32 = 2;
pub const DHCP_OVERLOAD_SNAME_FILE: u32 = 3;
