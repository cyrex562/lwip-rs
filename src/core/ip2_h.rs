use super::netif_h::NetIfc;
use crate::defines::LwipAddr;
use crate::defines::LwipAddrType::ADDR_TYPE_IPV6;
use crate::core::ip62::ip6_route;
use crate::core::err_h::LwipError;
use crate::core::ip42::ip4_route_src;

/*
 * @file
 * IP API
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

// #define LWIP_HDR_IP_H

/* This is passed as the destination address to ip_output_if (not
to ip_output), meaning that an IP header already is constructed
in the pbuf. This is used when TCP retransmits. */
// #define LWIP_IP_HDRINCL  NULL

/* pbufs passed to IP must have a ref-count of 1 as their payload pointer
gets altered as the packet is passed down the stack */

// #define LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p) LWIP_ASSERT("p.ref == 1", (p).ref == 1)

// #define IP_PCB_NETIFHINT ;NetIfc_hnetif_hints: i32
/* LWIP_NETIF_USE_HINTS */
// #define IP_PCB_NETIFHINT

/* This is the common part of all PCB types. It needs to be at the
beginning of a PCB type definition. It is located here so that
changes to this common part are made in one location instead of
having to change all PCB structs. */
// pub struct IpPcbCommon {
//     pub local_ip: LwipAddr,
//     pub remote_ip: LwipAddr,
//     pub netif_idx: usize,
//     pub so_options: u8,
//     pub tos: u8,
//     pub ttl: u8,
// }

// #define IP_PCB                             \
//   /* ip addresses in network byte order */ \
//   let local_ip: LwipAddr;                      \
//   let remote_ip: LwipAddr;                     \
//   /* Bound netif index */                  \
//   let netif_idx: u8;                          \
//   /* Socket options */                     \
//   let so_options: u8;                         \
//   /* Type Of Service */                    \
//   let tos: u8;                                \
//   /* Time To Live */                       \
//   ttl: u8                                 \
//   /* link layer address resolution hint */ \
//   IP_PCB_NETIFHINT

#[derive(Default,Clone,Debug)]
pub struct IpContext {
    pub local_ip: LwipAddr,
    pub remote_ip: LwipAddr,
    pub netif_idx: usize,
    pub so_options: u32,
    pub tos: u8,
    pub ttl: u8,
}

impl IpContext {
    pub fn new() -> IpContext{
        IpContext{
            ..Default::default()
        }
    }
}


/*
 * Option flags per-socket. These are the same like SO_XXX in sockets.h
 */
pub const SOF_REUSEADDR: u32 = 0x04; /* allow local address reuse */
pub const SOF_KEEPALIVE: u32 = 0x08; /* keep connections alive */
pub const SOF_BROADCAST: u32 = 0x20; /* permit to send and to receive broadcast messages (see IP_SOF_BROADCAST option) */

/* These flags are inherited (e.g. from a listen-pcb to a connection-pcb): */
pub const SOF_INHERITED: u32 = (SOF_REUSEADDR | SOF_KEEPALIVE);

// /* Global variables of this module, kept in a struct for efficient access using base+index. */
// pub struct

/* Get the interface that accepted the current packet.
 * This may or may not be the receiving netif, depending on your netif/network setup.
 * This function must only be called from a receive callback (udp_recv,
 * raw_recv, tcp_accept). It will return NULL otherwise. */
// #define ip_current_netif()      (ip_data.current_netif)
/* Get the interface that received the current packet.
 * This function must only be called from a receive callback (udp_recv,
 * raw_recv, tcp_accept). It will return NULL otherwise. */
// #define ip_current_input_netif() (ip_data.current_input_netif)
/* Total header length of ip(6)_current_header() (i.e. after this, the UDP/TCP header starts) */
// #define ip_current_header_tot_len() (ip_data.current_ip_header_tot_len)
/* Source IP address of current_header */
// #define ip_current_src_addr()   (&ip_data.current_iphdr_src)
/* Destination IP address of current_header */
// #define ip_current_dest_addr()  (&ip_data.current_iphdr_dest)

/* Get the IPv4 header of the current packet.
 * This function must only be called from a receive callback (udp_recv,
 * raw_recv, tcp_accept). It will return NULL otherwise. */
// #define ip4_current_header()     ip_data.current_ip4_header
/* Get the IPv6 header of the current packet.
 * This function must only be called from a receive callback (udp_recv,
 * raw_recv, tcp_accept). It will return NULL otherwise. */
// #define ip6_current_header()      (( struct ip6_hdr*)(ip_data.current_ip6_header))
/* Returns TRUE if the current IP input packet is IPv6, FALSE if it is IPv4 */
// #define ip_current_is_v6()        (ip6_current_header() != NULL)
/* Source IPv6 address of current_header */
// #define ip6_current_src_addr()    (ip_2_ip6(&ip_data.current_iphdr_src))
/* Destination IPv6 address of current_header */
// #define ip6_current_dest_addr()   (ip_2_ip6(&ip_data.current_iphdr_dest))
/* Get the transport layer protocol */
// #define ip_current_header_proto() (ip_current_is_v6() ? \
//    IP6H_NEXTH(ip6_current_header()) :\
//    IPH_PROTO(ip4_current_header()))
/* Get the transport layer header */
// #define ip_next_header_ptr()     (( void*)((ip_current_is_v6() ? \
//   ip6_current_header() : ip4_current_header())  + ip_current_header_tot_len()))

/* Source IP4 address of current_header */
// #define ip4_current_src_addr()     (ip_2_ip4(&ip_data.current_iphdr_src))
/* Destination IP4 address of current_header */
// #define ip4_current_dest_addr()    (ip_2_ip4(&ip_data.current_iphdr_dest))

// #elif LWIP_IPV4 /* LWIP_IPV4 && LWIP_IPV6 */
/* Get the IPv4 header of the current packet.
 * This function must only be called from a receive callback (udp_recv,
 * raw_recv, tcp_accept). It will return NULL otherwise. */
// #define ip4_current_header()     ip_data.current_ip4_header
/* Always returns FALSE when only supporting IPv4 only */
// #define ip_current_is_v6()        0
/* Get the transport layer protocol */
// #define ip_current_header_proto() IPH_PROTO(ip4_current_header())
/* Get the transport layer header */
// #define ip_next_header_ptr()     (( void*)(ip4_current_header() + ip_current_header_tot_len()))
/* Source IP4 address of current_header */
// #define ip4_current_src_addr()     (&ip_data.current_iphdr_src)
/* Destination IP4 address of current_header */
// #define ip4_current_dest_addr()    (&ip_data.current_iphdr_dest)

// #elif LWIP_IPV6 /* LWIP_IPV4 && LWIP_IPV6 */
/* Get the IPv6 header of the current packet.
 * This function must only be called from a receive callback (udp_recv,
 * raw_recv, tcp_accept). It will return NULL otherwise. */
// #define ip6_current_header()      (( struct ip6_hdr*)(ip_data.current_ip6_header))
/* Always returns TRUE when only supporting IPv6 only */
// #define ip_current_is_v6()        1
/* Get the transport layer protocol */
// #define ip_current_header_proto() IP6H_NEXTH(ip6_current_header())
/* Get the transport layer header */
// #define ip_next_header_ptr()     (( void*)((ip6_current_header()) + ip_current_header_tot_len()))
/* Source IP6 address of current_header */
// #define ip6_current_src_addr()    (&ip_data.current_iphdr_src)
/* Destination IP6 address of current_header */
// #define ip6_current_dest_addr()   (&ip_data.current_iphdr_dest)

/* Union source address of current_header */
// #define ip_current_src_addr()    (&ip_data.current_iphdr_src)
/* Union destination address of current_header */
// #define ip_current_dest_addr()   (&ip_data.current_iphdr_dest)

/* Gets an IP pcb option (SOF_* flags) */
// #define ip_get_option(pcb, opt)   ((pcb).so_options & (opt))
/* Sets an IP pcb option (SOF_* flags) */
// #define ip_set_option(pcb, opt)   ((pcb).so_options = ((pcb).so_options | (opt)))
/* Resets an IP pcb option (SOF_* flags) */
// #define ip_reset_option(pcb, opt) ((pcb).so_options = ((pcb).so_options & !(opt)))

/*
 * @ingroup ip
 * Output IP packet, netif is selected by source address
 */
// #define ip_output(p, src, dest, ttl, tos, proto) \
//         (IP_IS_V6(dest) ? \
//         ip6_output(p, ip_2_ip6(src), ip_2_ip6(dest), ttl, tos, proto) : \
//         ip4_output(p, ip_2_ip4(src), ip_2_ip4(dest), ttl, tos, proto))
/*
 * @ingroup ip
 * Output IP packet to specified interface
 */
// #define ip_output_if(p, src, dest, ttl, tos, proto, netif) \
//         (IP_IS_V6(dest) ? \
//         ip6_output_if(p, ip_2_ip6(src), ip_2_ip6(dest), ttl, tos, proto, netif) : \
//         ip4_output_if(p, ip_2_ip4(src), ip_2_ip4(dest), ttl, tos, proto, netif))
/*
 * @ingroup ip
 * Output IP packet to interface specifying source address
 */
// #define ip_output_if_src(p, src, dest, ttl, tos, proto, netif) \
//         (IP_IS_V6(dest) ? \
//         ip6_output_if_src(p, ip_2_ip6(src), ip_2_ip6(dest), ttl, tos, proto, netif) : \
//         ip4_output_if_src(p, ip_2_ip4(src), ip_2_ip4(dest), ttl, tos, proto, netif))
/* Output IP packet that already includes an IP header. */
// #define ip_output_if_hdrincl(p, src, dest, netif) \
//         (IP_IS_V6(dest) ? \
//         ip6_output_if(p, ip_2_ip6(src), LWIP_IP_HDRINCL, 0, 0, 0, netif) : \
//         ip4_output_if(p, ip_2_ip4(src), LWIP_IP_HDRINCL, 0, 0, 0, netif))
/* Output IP packet with netif_hint */
// #define ip_output_hinted(p, src, dest, ttl, tos, proto, netif_hint) \
//         (IP_IS_V6(dest) ? \
//         ip6_output_hinted(p, ip_2_ip6(src), ip_2_ip6(dest), ttl, tos, proto, netif_hint) : \
//         ip4_output_hinted(p, ip_2_ip4(src), ip_2_ip4(dest), ttl, tos, proto, netif_hint))
/*
 * @ingroup ip
 * Get netif for address combination. See \ref ip6_route and \ref ip4_route
 */
// #define ip_route(src, dest) \
//         (IP_IS_V6(dest) ? \
//         ip6_route(ip_2_ip6(src), ip_2_ip6(dest)) : \
//         ip4_route_src(ip_2_ip4(src), ip_2_ip4(dest)))
pub fn ip_route(src: &mut LwipAddr, dst: &mut LwipAddr) -> Result<(), LwipError> {
    if dst.addr_type == ADDR_TYPE_IPV6 {
        ip6_route(src, dst)
    } else {
        ip4_route_src(src, dst)
    }
}

/*
 * @ingroup ip
 * Get netif for IP.
 */
// #define ip_netif_get_local_ip(netif, dest) (IP_IS_V6(dest) ? \
//         ip6_netif_get_local_ip(netif, ip_2_ip6(dest)) : \
//         ip4_netif_get_local_ip(netif))
// #define ip_debug_print(is_ipv6, p) ((is_ipv6) ? ip6_debug_print(p) : ip4_debug_print(p))

// pub fn  ip_input(p: &mut pbuf, inp: &mut NetIfc);

// #elif LWIP_IPV4 /* LWIP_IPV4 && LWIP_IPV6 */
// #define ip_output(p, src, dest, ttl, tos, proto) \
//         ip4_output(p, src, dest, ttl, tos, proto)
// #define ip_output_if(p, src, dest, ttl, tos, proto, netif) \
//         ip4_output_if(p, src, dest, ttl, tos, proto, netif)
// #define ip_output_if_src(p, src, dest, ttl, tos, proto, netif) \
//         ip4_output_if_src(p, src, dest, ttl, tos, proto, netif)
// #define ip_output_hinted(p, src, dest, ttl, tos, proto, netif_hint) \
//         ip4_output_hinted(p, src, dest, ttl, tos, proto, netif_hint)
// #define ip_output_if_hdrincl(p, src, dest, netif) \
//         ip4_output_if(p, src, LWIP_IP_HDRINCL, 0, 0, 0, netif)
// #define ip_route(src, dest) \
//         ip4_route_src(src, dest)
// #define ip_netif_get_local_ip(netif, dest) \
//         ip4_netif_get_local_ip(netif)
// #define ip_debug_print(is_ipv6, p) ip4_debug_print(p)

// #define ip_input ip4_input

// #elif LWIP_IPV6 /* LWIP_IPV4 && LWIP_IPV6 */
// #define ip_output(p, src, dest, ttl, tos, proto) \
//         ip6_output(p, src, dest, ttl, tos, proto)
// #define ip_output_if(p, src, dest, ttl, tos, proto, netif) \
//         ip6_output_if(p, src, dest, ttl, tos, proto, netif)
// #define ip_output_if_src(p, src, dest, ttl, tos, proto, netif) \
//         ip6_output_if_src(p, src, dest, ttl, tos, proto, netif)
// #define ip_output_hinted(p, src, dest, ttl, tos, proto, netif_hint) \
//         ip6_output_hinted(p, src, dest, ttl, tos, proto, netif_hint)
// #define ip_output_if_hdrincl(p, src, dest, netif) \
//         ip6_output_if(p, src, LWIP_IP_HDRINCL, 0, 0, 0, netif)
// #define ip_route(src, dest) \
//         ip6_route(src, dest)
// #define ip_netif_get_local_ip(netif, dest) \
//         ip6_netif_get_local_ip(netif, dest)
// #define ip_debug_print(is_ipv6, p) ip6_debug_print(p)

// #define ip_input ip6_input

pub fn ip_route_get_local_ip(
    src: &LwipAddr,
    dest: &LwipAddr,
    netif: &NetIfc,
    ipaddr: &mut LwipAddr,
) {
    (netif) = ip_route(src, dest);
    (ipaddr) = ip_netif_get_local_ip(netif, dest);
}
