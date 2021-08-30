/*
 * @file
 *
 * IPv6 version of ICMP, as per RFC 4443.
 */

/*
 * Copyright (c) 2010 Inico Technologies Ltd.
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

// #undef LWIP_ICMP6_DATASIZE
// #define LWIP_ICMP6_DATASIZE   8

/* Forward declarations */
// pub fn icmp6_send_response(p: &mut pbuf, code: u8, data: u32, type: u8);
// pub fn icmp6_send_response_with_addrs(p: &mut pbuf, code: u8, data: u32,
//     type: u8,  src_addr: &mut ip6_addr_t,  dest_addr: &mut ip6_addr_t);
// pub fn icmp6_send_response_with_addrs_and_netif(p: &mut pbuf, code: u8, data: u32,
//     type: u8,  src_addr: &mut ip6_addr_t,  dest_addr: &mut ip6_addr_t, netif: &mut NetIfc);

/*
 * Process an input ICMPv6 message. Called by ip6_input.
 *
 * Will generate a reply for echo requests. Other messages are forwarded
 * to nd6_input, or mld6_input.
 *
 * @param p the mld packet, p.payload pointing to the icmpv6 header
 * @param inp the netif on which this packet was received
 */
pub fn icmp6_input(p: &mut pbuf, inp: &mut NetIfc) {
    let icmp6hdr: &mut icmp6_hdr;
    let r: &mut pbuf;
    const reply_src: &mut ip6_addr_t;

    ICMP6_STATS_INC(icmp6.recv);

    /* Check that ICMPv6 header fits in payload */
    if (p.len < sizeof(icmp6_hdr)) {
        /* drop short packets */
        pbuf_free(p);
        ICMP6_STATS_INC(icmp6.lenerr);
        ICMP6_STATS_INC(icmp6.drop);
        return;
    }

    icmp6hdr = p.payload;

    // IF__NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_CHECK_ICMP6) {
    if inp::CHECKSUM_ENABLED(NETIF_CHECKSUM_CHECK_ICMP6) {
        if (ip6_chksum_pseudo(
            p,
            IP6_NEXTH_ICMP6,
            p.tot_len,
            ip6_current_src_addr(),
            ip6_current_dest_addr(),
        ) != 0)
        {
            /* Checksum failed */
            pbuf_free(p);
            ICMP6_STATS_INC(icmp6.chkerr);
            ICMP6_STATS_INC(icmp6.drop);
            return;
        }
    }

    match (icmp6hdr.msg_type) {
        ICMP6_TYPE_NA => {} /* Neighbor advertisement */
        ICMP6_TYPE_NS => {} /* Neighbor solicitation */
        ICMP6_TYPE_RA => {} /* Router advertisement */
        ICMP6_TYPE_RD => {} /* Redirect */
        ICMP6_TYPE_PTB => {
            /* Packet too big */
            nd6_input(p, inp);
            return;
        }
        ICMP6_TYPE_RS => {}

        /* @todo implement router functionality */
        ICMP6_TYPE_MLQ => {}
        ICMP6_TYPE_MLR => {}
        ICMP6_TYPE_MLD => {
            mld6_input(p, inp);
            return;
        }
        ICMP6_TYPE_EREQ => {
            /* multicast destination address? */
            if (ip6_addr_ismulticast(ip6_current_dest_addr())) {
                /* drop */
                pbuf_free(p);
                ICMP6_STATS_INC(icmp6.drop);
                return;
            }

            /* Allocate reply. */
            r = pbuf_alloc(PBUF_IP, p.tot_len, PBUF_RAM);
            if (r == None) {
                /* drop */
                pbuf_free(p);
                ICMP6_STATS_INC(icmp6.memerr);
                return;
            }

            /* Copy echo request. */
            if (pbuf_copy(r, p) != ERR_OK) {
                /* drop */
                pbuf_free(p);
                pbuf_free(r);
                ICMP6_STATS_INC(icmp6.err);
                return;
            }

            /* Determine reply source IPv6 address. */

            if (ip6_addr_ismulticast(ip6_current_dest_addr())) {
                reply_src = ip_2_ip6(ip6_select_source_address(inp, ip6_current_src_addr()));
                if (reply_src == None) {
                    /* drop */
                    pbuf_free(p);
                    pbuf_free(r);
                    ICMP6_STATS_INC(icmp6.rterr);
                    return;
                }
            } else {
                reply_src = ip6_current_dest_addr();
            }

            /* Set fields in reply. */
            (r.payload).msg_type = ICMP6_TYPE_EREP;
            (r.payload).chksum = 0;

            // IF__NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_GEN_ICMP6) {
            if inp::NETIF_CHECKSUM_ENABLED(NETIF_CHECKSUM_GEN_ICMP6) {
                (r.payload).chksum = ip6_chksum_pseudo(
                    r,
                    IP6_NEXTH_ICMP6,
                    r.tot_len,
                    reply_src,
                    ip6_current_src_addr(),
                );
            }

            /* Send reply. */
            ICMP6_STATS_INC(icmp6.xmit);
            ip6_output_if(
                r,
                reply_src,
                ip6_current_src_addr(),
                LWIP_ICMP6_HL,
                0,
                IP6_NEXTH_ICMP6,
                inp,
            );
            pbuf_free(r);
        }
        _ => {
            ICMP6_STATS_INC(icmp6.proterr);
            ICMP6_STATS_INC(icmp6.drop);
        }
    }

    pbuf_free(p);
}

/*
 * Send an icmpv6 'destination unreachable' packet.
 *
 * This function must be used only in direct response to a packet that is being
 * received right now. Otherwise, address zones would be lost.
 *
 * @param p the input packet for which the 'unreachable' should be sent,
 *          p.payload pointing to the IPv6 header
 * @param c ICMPv6 code for the unreachable type
 */
pub fn icmp6_dest_unreach(p: &mut pbuf, c: icmp6_dur_code) {
    icmp6_send_response(p, c, 0, ICMP6_TYPE_DUR);
}

/*
 * Send an icmpv6 'packet too big' packet.
 *
 * This function must be used only in direct response to a packet that is being
 * received right now. Otherwise, address zones would be lost.
 *
 * @param p the input packet for which the 'packet too big' should be sent,
 *          p.payload pointing to the IPv6 header
 * @param mtu the maximum mtu that we can accept
 */
pub fn icmp6_packet_too_big(p: &mut pbuf, mtu: u32) {
    icmp6_send_response(p, 0, mtu, ICMP6_TYPE_PTB);
}

/*
 * Send an icmpv6 'time exceeded' packet.
 *
 * This function must be used only in direct response to a packet that is being
 * received right now. Otherwise, address zones would be lost.
 *
 * @param p the input packet for which the 'time exceeded' should be sent,
 *          p.payload pointing to the IPv6 header
 * @param c ICMPv6 code for the time exceeded type
 */
pub fn icmp6_time_exceeded(p: &mut pbuf, c: icmp6_te_code) {
    icmp6_send_response(p, c, 0, ICMP6_TYPE_TE);
}

/*
 * Send an icmpv6 'time exceeded' packet, with explicit source and destination
 * addresses.
 *
 * This function may be used to send a response sometime after receiving the
 * packet for which this response is meant. The provided source and destination
 * addresses are used primarily to retain their zone information.
 *
 * @param p the input packet for which the 'time exceeded' should be sent,
 *          p.payload pointing to the IPv6 header
 * @param c ICMPv6 code for the time exceeded type
 * @param src_addr source address of the original packet, with zone information
 * @param dest_addr destination address of the original packet, with zone
 *                  information
 */
pub fn icmp6_time_exceeded_with_addrs(
    p: &mut pbuf,
    c: icmp6_te_code,
    src_addr: &mut ip6_addr_t,
    dest_addr: &mut ip6_addr_t,
) {
    icmp6_send_response_with_addrs(p, c, 0, ICMP6_TYPE_TE, src_addr, dest_addr);
}

/*
 * Send an icmpv6 'parameter problem' packet.
 *
 * This function must be used only in direct response to a packet that is being
 * received right now. Otherwise, address zones would be lost and the calculated
 * offset would be wrong (calculated against ip6_current_header()).
 *
 * @param p the input packet for which the 'param problem' should be sent,
 *          p.payload pointing to the IP header
 * @param c ICMPv6 code for the param problem type
 * @param pointer the pointer to the byte where the parameter is found
 */
pub fn icmp6_param_problem(p: &mut pbuf, c: icmp6_pp_code, pointer: &Vec<u8>) {
    let pointer_u32: u32 = (pointer - ip6_current_header());
    icmp6_send_response(p, c, pointer_u32, ICMP6_TYPE_PP);
}

/*
 * Send an ICMPv6 packet in response to an incoming packet.
 * The packet is sent *to* ip_current_src_addr() on ip_current_netif().
 *
 * @param p the input packet for which the response should be sent,
 *          p.payload pointing to the IPv6 header
 * @param code Code of the ICMPv6 header
 * @param data Additional 32-bit parameter in the ICMPv6 header
 * @param type Type of the ICMPv6 header
 */
pub fn icmp6_send_response(p: &mut pbuf, code: u8, data: u32, msg_type: u8) {
    let reply_src: &mut ip6_addr;
    let reply_dest: &mut ip6_addr;
    let netif: &mut NetIfc = ip_current_netif();

    LWIP_ASSERT("icmpv6 packet not a direct response", netif != None);
    reply_dest = ip6_current_src_addr();

    /* Select an address to use as source. */
    reply_src = ip_2_ip6(ip6_select_source_address(netif, reply_dest));
    if (reply_src == None) {
        ICMP6_STATS_INC(icmp6.rterr);
        return;
    }
    icmp6_send_response_with_addrs_and_netif(p, code, data, msg_type, reply_src, reply_dest, netif);
}

/*
 * Send an ICMPv6 packet in response to an incoming packet.
 *
 * Call this function if the packet is NOT sent as a direct response to an
 * incoming packet, but rather sometime later (e.g. for a fragment reassembly
 * timeout). The caller must provide the zoned source and destination addresses
 * from the original packet with the src_addr and dest_addr parameters. The
 * reason for this approach is that while the addresses themselves are part of
 * the original packet, their zone information is not, thus possibly resulting
 * in a link-local response being sent over the wrong link.
 *
 * @param p the input packet for which the response should be sent,
 *          p.payload pointing to the IPv6 header
 * @param code Code of the ICMPv6 header
 * @param data Additional 32-bit parameter in the ICMPv6 header
 * @param type Type of the ICMPv6 header
 * @param src_addr original source address
 * @param dest_addr original destination address
 */
pub fn icmp6_send_response_with_addrs(
    p: &mut pbuf,
    code: u8,
    data: u32,
    msg_type: u8,
    src_addr: &mut ip6_addr_t,
    dest_addr: &mut ip6_addr_t,
) {
    let reply_src: &mut ip6_addr;
    let reply_dest: &mut ip6_addr;
    let netif: &mut NetIfc;

    /* Get the destination address and netif for this ICMP message. */
    LWIP_ASSERT("must provide both source and destination", src_addr != None);
    LWIP_ASSERT(
        "must provide both source and destination",
        dest_addr != None,
    );

    /* Special case, as ip6_current_xxx is either NULL, or points
    to a different packet than the one that expired. */
    IP6_ADDR_ZONECHECK(src_addr);
    IP6_ADDR_ZONECHECK(dest_addr);
    /* Swap source and destination for the reply. */
    reply_dest = src_addr;
    reply_src = dest_addr;
    netif = ip6_route(reply_src, reply_dest);
    if (netif == None) {
        ICMP6_STATS_INC(icmp6.rterr);
        return;
    }
    icmp6_send_response_with_addrs_and_netif(p, code, data, msg_type, reply_src, reply_dest, netif);
}

/*
 * Send an ICMPv6 packet (with srd/dst address and netif given).
 *
 * @param p the input packet for which the response should be sent,
 *          p.payload pointing to the IPv6 header
 * @param code Code of the ICMPv6 header
 * @param data Additional 32-bit parameter in the ICMPv6 header
 * @param type Type of the ICMPv6 header
 * @param reply_src source address of the packet to send
 * @param reply_dest destination address of the packet to send
 * @param netif netif to send the packet
 */
pub fn icmp6_send_response_with_addrs_and_netif(
    p: &mut pbuf,
    code: u8,
    data: u32,
    msg_type: u8,
    reply_src: &mut ip6_addr_t,
    reply_dest: &mut ip6_addr_t,
    netif: &mut NetIfc,
) {
    let q: &mut pbuf;
    let icmp6hdr: &mut icmp6_hdr;

    /* ICMPv6 header + IPv6 header + data */
    // q = pbuf_alloc(PBUF_IP, sizeof(icmp6_hdr) + IP6_HLEN + LWIP_ICMP6_DATASIZE,
    //                PBUF_RAM);
    // if (q == NULL) {
    //   LWIP_DEBUGF(ICMP_DEBUG, ("icmp_time_exceeded: failed to allocate pbuf for ICMPv6 packet.\n"));
    //   ICMP6_STATS_INC(icmp6.memerr);
    //   return;
    // }
    LWIP_ASSERT(
        "check that first pbuf can hold icmp 6message",
        (q.len >= (sizeof(icmp6_hdr) + IP6_HLEN + LWIP_ICMP6_DATASIZE)),
    );

    icmp6hdr = q.payload;
    icmp6hdr.msg_type = msg_type;
    icmp6hdr.code = code;
    icmp6hdr.data = lwip_htonl(data);

    /* copy fields from original packet */
    // SMEMCPY(q.payload + sizeof( icmp6_hdr), p.payload,
    //         IP6_HLEN + LWIP_ICMP6_DATASIZE);

    /* calculate checksum */
    icmp6hdr.chksum = 0;

    // IF__NETIF_CHECKSUM_ENABLED(netif, NETIF_CHECKSUM_GEN_ICMP6) {
    if netif::CHECKSUM_ENABLED(NETIF_CHECKSUM_GEN_ICMP6) {
        icmp6hdr.chksum = ip6_chksum_pseudo(q, IP6_NEXTH_ICMP6, q.tot_len, reply_src, reply_dest);
    }

    ICMP6_STATS_INC(icmp6.xmit);
    ip6_output_if(
        q,
        reply_src,
        reply_dest,
        LWIP_ICMP6_HL,
        0,
        IP6_NEXTH_ICMP6,
        netif,
    );
    pbuf_free(q);
}
