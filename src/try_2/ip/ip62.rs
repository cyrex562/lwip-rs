/*
 * @file
 *
 * IPv6 layer.
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

/*
 * Finds the appropriate network interface for a given IPv6 address. It tries to select
 * a netif following a sequence of heuristics:
 * 1) if there is only 1 netif, return it
 * 2) if the destination is a zoned address, match its zone to a netif
 * 3) if the either the source or destination address is a scoped address,
 *    match the source address's zone (if set) or address (if not) to a netif
 * 4) tries to match the destination subnet to a configured address
 * 5) tries to find a router-announced route
 * 6) tries to match the (unscoped) source address to the netif
 * 7) returns the default netif, if configured
 *
 * Note that each of the two given addresses may or may not be properly zoned.
 *
 * @param src the source IPv6 address, if known
 * @param dest the destination IPv6 address for which to find the route
 * @return the netif on which to send to reach dest
 */
use crate::core::common::PP_HTONS;
use crate::core::defines::LwipAddr;
use crate::core::error::{ERR_BUF, ERR_RTE, ERR_VAL, LwipError};
use crate::icmp::icmp62::{icmp6_dest_unreach, icmp6_input, icmp6_packet_too_big, icmp6_param_problem, icmp6_time_exceeded};
use crate::ip::ip6_addr_h::{ip6_addr_copy, ip6_addr_copy_from_packed, ip6_addr_copy_to_packed, ip6_addr_isallnodes_iflocal, ip6_addr_isallnodes_linklocal, ip6_addr_isany, ip6_addr_isglobal, ip6_addr_isipv4mappedipv6, ip6_addr_islinklocal, ip6_addr_isloopback, ip6_addr_ismulticast, ip6_addr_ismulticast_iflocal, ip6_addr_ismulticast_linklocal, ip6_addr_issitelocal, ip6_addr_issolicitednode, ip6_addr_isuniquelocal, ip6_addr_multicast_scope, ip6_addr_set_zero, IP6_MULTICAST_SCOPE_GLOBAL, IP6_MULTICAST_SCOPE_LINK_LOCAL, IP6_MULTICAST_SCOPE_ORGANIZATION_LOCAL, IP6_MULTICAST_SCOPE_RESERVED, IP6_MULTICAST_SCOPE_SITE_LOCAL};
use crate::ip::ip6_frag::{ip6_frag, ip6_reass};
use crate::ip::ip6_h::{ip6_dest_hdr, IP6_DEST_HLEN, ip6_frag_hdr, IP6_FRAG_MORE_FLAG, IP6_FRAG_OFFSET_MASK, ip6_hbh_hdr, IP6_HBH_HLEN, ip6_hdr, IP6_HLEN, IP6_NEXTH_HOPBYHOP, IP6_NEXTH_ICMP6, IP6_NEXTH_NONE, ip6_opt_hdr, IP6_OPT_HLEN, IP6_PADN_OPTION, ip6_rout_hdr, IP6_ROUTER_ALERT_DLEN, IP6_ROUTER_ALERT_OPTION};
use crate::ip::ip6_zone_h::{ip6_addr_has_zone, ip6_addr_lacks_zone, ip6_addr_test_zone};
use crate::ip::ip6_zone_h::LwipIpv6ScopeType::{Ip6Unicast, Ip6Unknown};
use crate::mld6::mld62::mld6_lookfor_group;
use crate::nd6::nd62::{nd6_find_route, nd6_get_destination_mtu};
use crate::netif::defs::{NetifHint, NetworkInterfaceCtx};
use crate::netif::ops::{netif_is_link_up, netif_is_up, netif_loop_output};
use crate::packetbuffer::pbuf::{pbuf_add_header, pbuf_add_header_force, pbuf_free, pbuf_realloc, pbuf_remove_header};
use crate::packetbuffer::pbuf_h::PBUF_FLAG_MCASTLOOP;
use crate::raw::raw::raw_input;
use crate::raw::raw_priv_h::raw_input_state_t::{RAW_INPUT_DELIVERED, RAW_INPUT_EATEN};
use crate::raw::raw_priv_h::raw_input_state_t;
use crate::udp::udp2::udp_input;

pub fn ip6_route(net_ifc_coll: &mut Vec<NetworkInterfaceCtx>, src: &LwipAddr, dest: &LwipAddr) -> Result<NetworkInterfaceCtx, LwipError> {
    //  LWIP_SINGLE_NETIF 
    let netif: &mut NetworkInterfaceCtx;
    let i: i8;

    // LWIP_ASSERT_CORE_LOCKED();

    //  If single netif configuration, fast return. 
    if net_ifc_coll.len() != 0 && net_ifc_coll.len() == 1{
        if !netif_is_up(&net_ifc_coll[0])
            || !netif_is_link_up(&net_ifc_coll[0])
            || (ip6_addr_has_zone(dest) && !ip6_addr_test_zone(dest, &net_ifc_coll[0]))
        {
            return Err(LwipError::new(ERR_VAL, ""));
        }
        return netif_list;
    }

    /* Special processing for zoned destination addresses. This includes link-
     * local unicast addresses and interface/link-local multicast addresses. Use
     * the zone to find a matching netif. If the address is not zoned, then there
     * is technically no "wrong" netif to choose, and we leave routing to other
     * rules; in most cases this should be the scoped-source rule below. */
    if ip6_addr_has_zone(dest) {
        IP6_ADDR_ZONECHECK(dest);
        /* Find a netif based on the zone. For custom mappings, one zone may map
         * to multiple netifs, so find one that can actually send a packet. */
        // NETIF_FOREACH(netif) {
        //   if (ip6_addr_test_zone(dest, netif) &&
        //       netif_is_up(netif) && netif_is_link_up(netif)) {
        //     return netif;
        //   }
        // }
        /* No matching netif found. Do no try to route to a different netif,
         * as that would be a zone violation, resulting in any packets sent to
         * that netif being dropped on output. */
        return None;
    }

    /* Special processing for scoped source and destination addresses. If we get
     * here, the destination address does not have a zone, so either way we need
     * to look at the source address, which may or may not have a zone. If it
     * does, the zone is restrictive: there is (typically) only one matching
     * netif for it, and we should avoid routing to any other netif as that would
     * result in guaranteed zone violations. For scoped source addresses that do
     * not have a zone, use (only) a netif that has that source address locally
     * assigned. This case also applies to the loopback source address, which has
     * an implied link-local scope. If only the destination address is scoped
     * (but, again, not zoned), we still want to use only the source address to
     * determine its zone because that's most likely what the user/application
     * wants, regardless of whether the source address is scoped. Finally, some
     * of this story also applies if scoping is disabled altogether. */

    // if (ip6_addr_has_scope(dest, Ip6Unknown) ||
    //     ip6_addr_has_scope(src, Ip6Unicast) ||
    //  LWIP_IPV6_SCOPES 
    if (ip6_addr_islinklocal(dest)
        || ip6_addr_ismulticast_iflocal(dest)
        || ip6_addr_ismulticast_linklocal(dest)
        || ip6_addr_islinklocal(src)
        || ip6_addr_isloopback(src))
    {
        if (ip6_addr_has_zone(src)) {
            //  Find a netif matching the source zone (relatively cheap). 
            // NETIF_FOREACH(netif) {
            //   if (netif_is_up(netif) && netif_is_link_up(netif) &&
            //       ip6_addr_test_zone(src, netif)) {
            //     return netif;
            //   }
            // }
        } else {
            //  Find a netif matching the source address (relatively expensive). 
            // NETIF_FOREACH(netif) {
            //   if (!netif_is_up(netif) || !netif_is_link_up(netif)) {
            //     continue;
            //   }
            //   // for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
            //   //   if (ip6_addr_isvalid(netif_ip6_addr_state(netif, i)) &&
            //   //       ip6_addr_cmp_zoneless(src, netif_ip6_addr(netif, i))) {
            //   //     return netif;
            //   //   }
            //   // }
            // }
        }
        /* Again, do not use any other netif in this case, as that could result in
         * zone boundary violations. */
        return None;
    }

    //  We come here only if neither source nor destination is scoped. 
    IP6_ADDR_ZONECHECK(src);

    netif = LWIP_HOOK_IP6_ROUTE(src, dest);
    if (netif != None) {
        return netif;
    }

    /* See if the destination subnet matches a configured address. In accordance
     * with RFC 5942, dynamically configured addresses do not have an implied
     * local subnet, and thus should be considered /128 assignments. However, as
     * such, the destination address may still match a local address, and so we
     * still need to check for exact matches here. By (lwIP) policy, statically
     * configured addresses do always have an implied local /64 subnet. */
    // NETIF_FOREACH(netif) {
    //   if (!netif_is_up(netif) || !netif_is_link_up(netif)) {
    //     continue;
    //   }
    //   for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
    //     if (ip6_addr_isvalid(netif_ip6_addr_state(netif, i)) &&
    //         ip6_addr_netcmp(dest, netif_ip6_addr(netif, i)) &&
    //         (netif_ip6_addr_isstatic(netif, i) ||
    //         ip6_addr_nethostcmp(dest, netif_ip6_addr(netif, i)))) {
    //       return netif;
    //     }
    //   }
    // }

    //  Get the netif for a suitable router-announced route. 
    netif = nd6_find_route(dest);
    if (netif != None) {
        return netif;
    }

    /* Try with the netif that matches the source address. Given the earlier rule
     * for scoped source addresses, this applies to unscoped addresses only. */
    if (!ip6_addr_isany(src)) {
        // NETIF_FOREACH(netif) {
        //   if (!netif_is_up(netif) || !netif_is_link_up(netif)) {
        //     continue;
        //   }
        //   for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
        //     if (ip6_addr_isvalid(netif_ip6_addr_state(netif, i)) &&
        //         ip6_addr_cmp(src, netif_ip6_addr(netif, i))) {
        //       return netif;
        //     }
        //   }
        // }
    }

    //  loopif is disabled, loopback traffic is passed through any netif 
    if (ip6_addr_isloopback(dest)) {
        //  don't check for link on loopback traffic 
        if (netif_default != None && netif_is_up(netif_default)) {
            return netif_default;
        }
        //  default netif is not up, just use any netif for loopback traffic 
        // NETIF_FOREACH(netif) {
        //   if (netif_is_up(netif)) {
        //     return netif;
        //   }
        // }
        return None;
    }

    //  no matching netif found, use default netif, if up 
    if ((netif_default == None) || !netif_is_up(netif_default) || !netif_is_link_up(netif_default))
    {
        return None;
    }
    return netif_default;
}

/*
 * @ingroup ip6
 * Select the best IPv6 source address for a given destination IPv6 address.
 *
 * This implementation follows RFC 6724 Sec. 5 to the following extent:
 * - Rules 1, 2, 3: fully implemented
 * - Rules 4, 5, 5.5: not applicable
 * - Rule 6: not implemented
 * - Rule 7: not applicable
 * - Rule 8: limited to "prefer /64 subnet match over non-match"
 *
 * For Rule 2, we deliberately deviate from RFC 6724 Sec. 3.1 by considering
 * ULAs to be of smaller scope than global addresses, to avoid that a preferred
 * ULA is picked over a deprecated global address when given a global address
 * as destination, as that would likely result in broken two-way communication.
 *
 * As long as temporary addresses are not supported (as used in Rule 7), a
 * proper implementation of Rule 8 would obviate the need to implement Rule 6.
 *
 * @param netif the netif on which to send a packet
 * @param dest the destination we are trying to reach (possibly not properly
 *             zoned)
 * @return the most suitable source address to use, or NULL if no suitable
 *         source address is found
 */
pub fn ip6_select_source_address(netif: &mut NetworkInterfaceCtx, dest: &mut ip6_addr_t) -> LwipAddr {
    let best_addr: &mut LwipAddr;
    let cand_addr: &mut ip6_addr_t;
    let dest_scope: i8;
    let cand_scope: i8;
    let best_scope = IP6_MULTICAST_SCOPE_RESERVED;
    let i: u8;
    let cand_pref;
    let cand_bits;
    let best_pref: u8 = 0;
    let best_bits: u8 = 0;

    /* Start by determining the scope of the given destination address. These
     * tests are hopefully (roughly) in order of likeliness to match. */
    if (ip6_addr_isglobal(dest)) {
        dest_scope = IP6_MULTICAST_SCOPE_GLOBAL;
    } else if (ip6_addr_islinklocal(dest) || ip6_addr_isloopback(dest)) {
        dest_scope = IP6_MULTICAST_SCOPE_LINK_LOCAL;
    } else if (ip6_addr_isuniquelocal(dest)) {
        dest_scope = IP6_MULTICAST_SCOPE_ORGANIZATION_LOCAL;
    } else if (ip6_addr_ismulticast(dest)) {
        dest_scope = ip6_addr_multicast_scope(dest);
    } else if (ip6_addr_issitelocal(dest)) {
        dest_scope = IP6_MULTICAST_SCOPE_SITE_LOCAL;
    } else {
        //  no match, consider scope global 
        dest_scope = IP6_MULTICAST_SCOPE_GLOBAL;
    }

    best_addr = None;

    // for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
    //   //  Consider only valid (= preferred and deprecated) addresses. 
    //   if (!ip6_addr_isvalid(netif_ip6_addr_state(netif, i))) {
    //     continue;
    //   }
    //   //  Determine the scope of this candidate address. Same ordering idea. 
    //   cand_addr = netif_ip6_addr(netif, i);
    //   if (ip6_addr_isglobal(cand_addr)) {
    //     cand_scope = IP6_MULTICAST_SCOPE_GLOBAL;
    //   } else if (ip6_addr_islinklocal(cand_addr)) {
    //     cand_scope = IP6_MULTICAST_SCOPE_LINK_LOCAL;
    //   } else if (ip6_addr_isuniquelocal(cand_addr)) {
    //     cand_scope = IP6_MULTICAST_SCOPE_ORGANIZATION_LOCAL;
    //   } else if (ip6_addr_issitelocal(cand_addr)) {
    //     cand_scope = IP6_MULTICAST_SCOPE_SITE_LOCAL;
    //   } else {
    //     //  no match, treat as low-priority global scope 
    //     cand_scope = IP6_MULTICAST_SCOPE_RESERVEDF;
    //   }
    //   cand_pref = ip6_addr_ispreferred(netif_ip6_addr_state(netif, i));
    //   //  @todo compute the actual common bits, for longest matching prefix. 
    //   /* We cannot count on the destination address having a proper zone
    //    * assignment, so do not compare zones in this case. */
    //   cand_bits = ip6_addr_netcmp_zoneless(cand_addr, dest); //  just 1 or 0 for now 
    //   if (cand_bits && ip6_addr_nethostcmp(cand_addr, dest)) {
    //     return netif_ip_addr6(netif, i); //  Rule 1 
    //   }
    //   if ((best_addr == NULL) || //  no alternative yet 
    //       ((cand_scope < best_scope) && (cand_scope >= dest_scope)) ||
    //       ((cand_scope > best_scope) && (best_scope < dest_scope)) || //  Rule 2 
    //       ((cand_scope == best_scope) && ((cand_pref > best_pref) || //  Rule 3 
    //       ((cand_pref == best_pref) && (cand_bits > best_bits))))) { //  Rule 8 
    //     //  We found a new "winning" candidate. 
    //     best_addr = netif_ip_addr6(netif, i);
    //     best_scope = cand_scope;
    //     best_pref = cand_pref;
    //     best_bits = cand_bits;
    //   }
    // }

    return best_addr; //  may be NULL 
}

/*
 * Forwards an IPv6 packet. It finds an appropriate route for the
 * packet, decrements the HL value of the packet, and outputs
 * the packet on the appropriate interface.
 *
 * @param p the packet to forward (p.payload points to IP header)
 * @param iphdr the IPv6 header of the input packet
 * @param inp the netif on which this packet was received
 */
pub fn ip6_forward(p: &mut PacketBuffer, iphdr: &mut ip6_hdr, inp: &mut NetworkInterfaceCtx) {
    let netif: &mut NetworkInterfaceCtx;

    //  do not forward link-local or loopback addresses 
    if (ip6_addr_islinklocal(ip6_current_dest_addr())
        || ip6_addr_isloopback(ip6_current_dest_addr()))
    {
        //    LWIP_DEBUGF(IP6_DEBUG, ("ip6_forward: not forwarding link-local address.\n"));
        IP6_STATS_INC(ip6.rterr);
        IP6_STATS_INC(ip6.drop);
        return;
    }

    //  Find network interface where to forward this IP packet to. 
    netif = ip6_route(IP6_ADDR_ANY6, ip6_current_dest_addr());
    if (netif == None) {
        /*LWIP_DEBUGF(IP6_DEBUG, ("ip6_forward: no route for %"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F"\n",
        IP6_ADDR_BLOCK1(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK2(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK3(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK4(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK5(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK6(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK7(ip6_current_dest_addr()),
        IP6_ADDR_BLOCK8(ip6_current_dest_addr())));*/

        //  Don't send ICMP messages in response to ICMP messages 
        if (IP6H_NEXTH(iphdr) != IP6_NEXTH_ICMP6) {
            icmp6_dest_unreach(p, ICMP6_DUR_NO_ROUTE);
        }

        IP6_STATS_INC(ip6.rterr);
        IP6_STATS_INC(ip6.drop);
        return;
    }

    /* Do not forward packets with a zoned (e.g., link-local) source address
     * outside of their zone. We determined the zone a bit earlier, so we know
     * that the address is properly zoned here, so we can safely use has_zone.
     * Also skip packets with a loopback source address (link-local implied). */
    if ((ip6_addr_has_zone(ip6_current_src_addr())
        && !ip6_addr_test_zone(ip6_current_src_addr(), netif))
        || ip6_addr_isloopback(ip6_current_src_addr()))
    {
        //    LWIP_DEBUGF(IP6_DEBUG, ("ip6_forward: not forwarding packet beyond its source address zone.\n"));
        IP6_STATS_INC(ip6.rterr);
        IP6_STATS_INC(ip6.drop);
        return;
    }

    /* Do not forward packets onto the same network interface on which
     * they arrived. */
    if (netif == inp) {
        //    LWIP_DEBUGF(IP6_DEBUG, ("ip6_forward: not bouncing packets back on incoming interface.\n"));
        IP6_STATS_INC(ip6.rterr);
        IP6_STATS_INC(ip6.drop);
        return;
    }

    //  decrement HL 
    IP6H_HOPLIM_SET(iphdr, IP6H_HOPLIM(iphdr) - 1);
    //  send ICMP6 if HL == 0 
    if (IP6H_HOPLIM(iphdr) == 0) {
        //  Don't send ICMP messages in response to ICMP messages 
        if (IP6H_NEXTH(iphdr) != IP6_NEXTH_ICMP6) {
            icmp6_time_exceeded(p, ICMP6_TE_HL);
        }

        IP6_STATS_INC(ip6.drop);
        return;
    }

    if (netif.mtu && (p.tot_len > netif.mtu)) {
        //  Don't send ICMP messages in response to ICMP messages 
        if (IP6H_NEXTH(iphdr) != IP6_NEXTH_ICMP6) {
            icmp6_packet_too_big(p, netif.mtu);
        }

        IP6_STATS_INC(ip6.drop);
        return;
    }
    /*LWIP_DEBUGF(IP6_DEBUG, ("ip6_forward: forwarding packet to %"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F"\n",
    IP6_ADDR_BLOCK1(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK2(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK3(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK4(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK5(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK6(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK7(ip6_current_dest_addr()),
    IP6_ADDR_BLOCK8(ip6_current_dest_addr())));*/

    //  transmit pbuf on chosen interface 
    netif.output_ip6(netif, p, ip6_current_dest_addr());
    IP6_STATS_INC(ip6.fw);
    IP6_STATS_INC(ip6.xmit);
    return;
}

//  Return true if the current input packet should be accepted on this netif 
pub fn ip6_input_accept(netif: &mut NetworkInterfaceCtx) {
    //  interface is up? 
    if (netif_is_up(netif)) {
        let i: u8;
        //  unicast to this interface address? address configured? 
        /* If custom scopes are used, the destination zone will be tested as
         * part of the local-address comparison, but we need to test the source
         * scope as well (e.g., is this interface on the same link?). */
        // for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
        //   if (ip6_addr_isvalid(netif_ip6_addr_state(netif, i)) &&
        //       ip6_addr_cmp(ip6_current_dest_addr(), netif_ip6_addr(netif, i))

        //       && (!ip6_addr_has_zone(ip6_current_src_addr()) ||
        //           ip6_addr_test_zone(ip6_current_src_addr(), netif))

        //   ) {
        //     //  accept on this netif 
        //     return 1;
        //   }
        // }
    }
    return 0;
}

/*
 * This function is called by the network interface device driver when
 * an IPv6 packet is received. The function does the basic checks of the
 * IP header such as packet size being at least larger than the header
 * size etc. If the packet was not destined for us, the packet is
 * forwarded (using ip6_forward).
 *
 * Finally, the packet is sent to the upper layer protocol input function.
 *
 * @param p the received IPv6 packet (p.payload points to IPv6 header)
 * @param inp the netif on which this packet was received
 * @return ERR_OK if the packet was processed (could return ERR_* if it wasn't
 *         processed, but currently always returns ERR_OK)
 */
pub fn ip6_input(p: &mut PacketBuffer, inp: &mut NetworkInterfaceCtx) {
    let ip6hdr: &mut ip6_hdr;
    let netif: &mut NetworkInterfaceCtx;
    let nexth: &mut Vec<u8>;
    let hlen: u16;
    let hlen_tot; //  the current header length 
    let check_ip_src: i32 = 1;
    let raw_status: raw_input_state_t;

    // LWIP_ASSERT_CORE_LOCKED()

    IP6_STATS_INC(ip6.recv);

    //  identify the IP header 
    ip6hdr = p.payload;
    if (IP6H_V(ip6hdr) != 6) {
        /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_WARNING, ("IPv6 packet dropped due to bad version number %"U32_F"\n",
        IP6H_V(ip6hdr)));*/
        pbuf_free(p);
        IP6_STATS_INC(ip6.err);
        IP6_STATS_INC(ip6.drop);
       return Ok(());
    }

    if (LWIP_HOOK_IP6_INPUT(p, inp)) {
        //  the packet has been eaten 
       return Ok(());
    }

    //  header length exceeds first pbuf length, or ip length exceeds total pbuf length? 
    if ((IP6_HLEN > p.len) || (IP6H_PLEN(ip6hdr) > (p.tot_len - IP6_HLEN))) {
        if (IP6_HLEN > p.len) {
            /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
            ("IPv6 header (len %"U16_F") does not fit in first pbuf (len %"U16_F"), IP packet dropped.\n",
                IP6_HLEN, p.len));*/
        }
        if ((IP6H_PLEN(ip6hdr) + IP6_HLEN) > p.tot_len) {
            /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
            ("IPv6 (plen %"U16_F") is longer than pbuf (len %"U16_F"), IP packet dropped.\n",
                (IP6H_PLEN(ip6hdr) + IP6_HLEN), p.tot_len));*/
        }
        //  free (drop) packet pbufs 
        pbuf_free(p);
        IP6_STATS_INC(ip6.lenerr);
        IP6_STATS_INC(ip6.drop);
       return Ok(());
    }

    /* Trim pbuf. This should have been done at the netif layer,
     * but we'll do it anyway just to be sure that its done. */
    pbuf_realloc(p, (IP6_HLEN + IP6H_PLEN(ip6hdr)));

    //  copy IP addresses to aligned ip6_addr_t 
    ip_addr_copy_from_ip6_packed(ip_data.current_iphdr_dest, ip6hdr.dest);
    ip_addr_copy_from_ip6_packed(ip_data.current_iphdr_src, ip6hdr.src);

    /* Don't accept virtual IPv4 mapped IPv6 addresses.
     * Don't accept multicast source addresses. */
    if (ip6_addr_isipv4mappedipv6(ip_2_ip6(&ip_data.current_iphdr_dest))
        || ip6_addr_isipv4mappedipv6(ip_2_ip6(&ip_data.current_iphdr_src))
        || ip6_addr_ismulticast(ip_2_ip6(&ip_data.current_iphdr_src)))
    {
        //  free (drop) packet pbufs 
        pbuf_free(p);
        IP6_STATS_INC(ip6.err);
        IP6_STATS_INC(ip6.drop);
       return Ok(());
    }

    //  Set the appropriate zone identifier on the addresses. 
    ip6_addr_assign_zone(ip_2_ip6(&ip_data.current_iphdr_dest), Ip6Unknown, inp);
    ip6_addr_assign_zone(ip_2_ip6(&ip_data.current_iphdr_src), Ip6Unicast, inp);

    //  current header pointer. 
    ip_data.current_ip6_header = ip6hdr;

    //  In netif, used in case we need to send ICMPv6 packets back. 
    ip_data.current_netif = inp;
    ip_data.current_input_netif = inp;

    //  match packet against an interface, i.e. is this packet for us? 
    if (ip6_addr_ismulticast(ip6_current_dest_addr())) {
        //  Always joined to multicast if-local and link-local all-nodes group. 
        if (ip6_addr_isallnodes_iflocal(ip6_current_dest_addr())
            || ip6_addr_isallnodes_linklocal(ip6_current_dest_addr()))
        {
            netif = inp;
        } else if (mld6_lookfor_group(inp, ip6_current_dest_addr())) {
            netif = inp;
        }
        //  LWIP_IPV6_MLD 
        else if (ip6_addr_issolicitednode(ip6_current_dest_addr())) {
            let i: u8;
            /* Filter solicited node packets when MLD is not enabled
             * (for Neighbor discovery). */
            netif = None;
        //       for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
        //         if (ip6_addr_isvalid(netif_ip6_addr_state(inp, i)) &&
        //             ip6_addr_cmp_solicitednode(ip6_current_dest_addr(), netif_ip6_addr(inp, i))) {
        //           netif = inp;
        // /*LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: solicited node packet accepted on interface %c%c\n",
        //               netif.name[0], netif.name[1]));*/
        //           break;
        //         }
        //       }
        } else {
            netif = None;
        }
    } else {
        /* start trying with inp. if that's not acceptable, start walking the
        list of configured netifs. */
        if (ip6_input_accept(inp)) {
            netif = inp;
        } else {
            netif = None;

            /* Shortcut: stop looking for other interfaces if either the source or
             * the destination has a scope constrained to this interface. Custom
             * scopes may break the 1:1 link/interface mapping, however. */
            if (ip6_addr_islinklocal(ip6_current_dest_addr())
                || ip6_addr_islinklocal(ip6_current_src_addr()))
            {
                // goto netif_found;
            }

            /* The loopback address is to be considered link-local. Packets to it
             * should be dropped on other interfaces, as per RFC 4291 Sec. 2.5.3.
             * Its implied scope means packets *from* the loopback address should
             * not be accepted on other interfaces, either. These requirements
             * cannot be implemented in the case that loopback traffic is sent
             * across a non-loopback interface, however. */
            if (ip6_addr_isloopback(ip6_current_dest_addr())
                || ip6_addr_isloopback(ip6_current_src_addr()))
            {
                // goto netif_found;
            }

            // NETIF_FOREACH(netif) {
            //   if (netif == inp) {
            //     //  we checked that before already 
            //     continue;
            //   }
            //   if (ip6_input_accept(netif)) {
            //     break;
            //   }
            // }
        }
        // netif_found:
        /*LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet accepted on interface %c%c\n",
        netif ? netif.name[0] : 'X', netif? netif.name[1] : 'X'));*/
    }

    //  "::" packet source address? (used in duplicate address detection) 
    if (ip6_addr_isany(ip6_current_src_addr())
        && (!ip6_addr_issolicitednode(ip6_current_dest_addr())))
    {
        //  packet source is not valid 
        //  free (drop) packet pbufs 
        //    LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with src ANY_ADDRESS dropped\n"));
        pbuf_free(p);
        IP6_STATS_INC(ip6.drop);
        // goto ip6_input_cleanup;
    }

    //  packet not for us? 
    if (netif == None) {
        //  packet not for us, route or discard 
        //    LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_TRACE, ("ip6_input: packet not for us.\n"));

        //  non-multicast packet? 
        if (!ip6_addr_ismulticast(ip6_current_dest_addr())) {
            //  try to forward IP packet on (other) interfaces 
            ip6_forward(p, ip6hdr, inp);
        }

        pbuf_free(p);
        // goto ip6_input_cleanup;
    }

    //  current netif pointer. 
    ip_data.current_netif = netif;

    //  Save next header type. 
    nexth = &IP6H_NEXTH(ip6hdr);

    //  Init header length. 
    hlen = hlen_tot = IP6_HLEN;

    //  Move to payload. 
    pbuf_remove_header(p, IP6_HLEN);

    //  Process known option extension headers, if present. 
    while (*nexth != IP6_NEXTH_NONE) {
        match (*nexth) {
            IP6_NEXTH_HOPBYHOP => {
                let opt_offset: i32;
                let hbh_hdr: &mut ip6_hbh_hdr;
                let opt_hdr: &mut ip6_opt_hdr;
                //      LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with Hop-by-Hop options header\n"));

                //  Get and check the header length, while staying in packet bounds. 
                hbh_hdr = p.payload;

                //  Get next header type. 
                nexth = &IP6_HBH_NEXTH(hbh_hdr);

                //  Get the header length. 
                hlen = (8 * (1 + hbh_hdr._hlen));

                if ((p.len < 8) || (hlen > p.len)) {
                    /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                    ("IPv6 options header (hlen %"U16_F") does not fit in first pbuf (len %"U16_F"), IPv6 packet dropped.\n",
                        hlen, p.len));*/
                    //  free (drop) packet pbufs 
                    pbuf_free(p);
                    IP6_STATS_INC(ip6.lenerr);
                    IP6_STATS_INC(ip6.drop);
                    // goto ip6_input_cleanup;
                }

                hlen_tot = (hlen_tot + hlen);

                //  The extended option header starts right after Hop-by-Hop header. 
                opt_offset = IP6_HBH_HLEN;
                while (opt_offset < hlen) {
                    let opt_dlen = 0;

                    opt_hdr = (hbh_hdr + opt_offset);

                    match (IP6_OPT_TYPE(opt_hdr)) {
                        //  @todo: process IPV6 Hop-by-Hop option data 
                        IP6_PAD1_OPTION => opt_dlen = -1,
                        //  PAD1 option doesn't have length and value field 
                        IP6_PADN_OPTION => opt_dlen = IP6_OPT_DLEN(opt_hdr),

                        IP6_ROUTER_ALERT_OPTION => opt_dlen = IP6_OPT_DLEN(opt_hdr),

                        IP6_JUMBO_OPTION => opt_dlen = IP6_OPT_DLEN(opt_hdr),

                        _ => {
                            //  Check 2 MSB of Hop-by-Hop header type. 
                            match (IP6_OPT_TYPE_ACTION(opt_hdr)) {
                                1 => {
                                    //  Discard the packet. 
                                    //            LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid Hop-by-Hop option type dropped.\n"));
                                    pbuf_free(p);
                                    IP6_STATS_INC(ip6.drop);
                                }
                                // goto ip6_input_cleanup;
                                2 => {
                                    //  Send ICMP Parameter Problem 
                                    icmp6_param_problem(p, ICMP6_PP_OPTION, opt_hdr);
                                    //            LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid Hop-by-Hop option type dropped.\n"));
                                    pbuf_free(p);
                                    IP6_STATS_INC(ip6.drop);
                                }
                                // goto ip6_input_cleanup;
                                3 => {
                                    //  Send ICMP Parameter Problem if destination address is not a multicast address 
                                    if (!ip6_addr_ismulticast(ip6_current_dest_addr())) {
                                        icmp6_param_problem(p, ICMP6_PP_OPTION, opt_hdr);
                                    }
                                    //            LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid Hop-by-Hop option type dropped.\n"));
                                    pbuf_free(p);
                                    IP6_STATS_INC(ip6.drop);
                                }
                                // goto ip6_input_cleanup;
                                _ =>
                                //  Skip over this option. 
                                {
                                    opt_dlen = IP6_OPT_DLEN(opt_hdr);
                                }
                            }
                        }
                    }

                    //  Adjust the offset to move to the next extended option header 
                    opt_offset = opt_offset + IP6_OPT_HLEN + opt_dlen;
                }
                pbuf_remove_header(p, hlen);
            }
            IP6_NEXTH_DESTOPTS => {
                let opt_offset: i32;
                let dest_hdr: &mut ip6_dest_hdr;
                let opt_hdr: &mut ip6_opt_hdr;
                //      LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with Destination options header\n"));

                dest_hdr = p.payload;

                //  Get next header type. 
                nexth = &IP6_DEST_NEXTH(dest_hdr);

                //  Get the header length. 
                hlen = 8 * (1 + dest_hdr._hlen);
                if ((p.len < 8) || (hlen > p.len)) {
                    /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                    ("IPv6 options header (hlen %"U16_F") does not fit in first pbuf (len %"U16_F"), IPv6 packet dropped.\n",
                        hlen, p.len));*/
                    //  free (drop) packet pbufs 
                    pbuf_free(p);
                    IP6_STATS_INC(ip6.lenerr);
                    IP6_STATS_INC(ip6.drop);
                    // goto ip6_input_cleanup;
                }

                hlen_tot = (hlen_tot + hlen);

                //  The extended option header starts right after Destination header. 
                opt_offset = IP6_DEST_HLEN;
                while (opt_offset < hlen) {
                    let opt_dlen = 0;

                    opt_hdr = (dest_hdr + opt_offset);

                    match (IP6_OPT_TYPE(opt_hdr)) {
                        //  @todo: process IPV6 Destination option data 
                        IP6_PAD1_OPTION => {
                            //  PAD1 option deosn't have length and value field 
                            opt_dlen = -1;
                        }

                        IP6_PADN_OPTION => {
                            opt_dlen = IP6_OPT_DLEN(opt_hdr);
                        }

                        IP6_ROUTER_ALERT_OPTION => {
                            opt_dlen = IP6_OPT_DLEN(opt_hdr);
                        }

                        IP6_JUMBO_OPTION => {
                            opt_dlen = IP6_OPT_DLEN(opt_hdr);
                        }

                        IP6_HOME_ADDRESS_OPTION => {
                            opt_dlen = IP6_OPT_DLEN(opt_hdr);
                        }

                        _ => {
                            //  Check 2 MSB of Destination header type. 
                            match (IP6_OPT_TYPE_ACTION(opt_hdr)) {
                                1 => {
                                    //  Discard the packet. 
                                    //            LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid destination option type dropped.\n"));
                                    pbuf_free(p);
                                    IP6_STATS_INC(ip6.drop);
                                }
                                // goto ip6_input_cleanup;
                                2 => {
                                    //  Send ICMP Parameter Problem 
                                    icmp6_param_problem(p, ICMP6_PP_OPTION, opt_hdr);
                                    //            LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid destination option type dropped.\n"));
                                    pbuf_free(p);
                                    IP6_STATS_INC(ip6.drop);
                                }
                                // goto ip6_input_cleanup;
                                3 => {
                                    //  Send ICMP Parameter Problem if destination address is not a multicast address 
                                    if (!ip6_addr_ismulticast(ip6_current_dest_addr())) {
                                        icmp6_param_problem(p, ICMP6_PP_OPTION, opt_hdr);
                                    }
                                    //            LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid destination option type dropped.\n"));
                                    pbuf_free(p);
                                    IP6_STATS_INC(ip6.drop);
                                }
                                // goto ip6_input_cleanup;
                                _ => {
                                    //  Skip over this option. 
                                    opt_dlen = IP6_OPT_DLEN(opt_hdr);
                                }
                            }
                        }
                    }

                    //  Adjust the offset to move to the next extended option header 
                    opt_offset = opt_offset + IP6_OPT_HLEN + opt_dlen;
                }

                pbuf_remove_header(p, hlen);
            }
            IP6_NEXTH_ROUTING => {
                let rout_hdr: &mut ip6_rout_hdr;
                //      LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with Routing header\n"));

                rout_hdr = p.payload;

                //  Get next header type. 
                nexth = &IP6_ROUT_NEXTH(rout_hdr);

                //  Get the header length. 
                hlen = 8 * (1 + rout_hdr._hlen);

                if ((p.len < 8) || (hlen > p.len)) {
                    /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                    ("IPv6 options header (hlen %"U16_F") does not fit in first pbuf (len %"U16_F"), IPv6 packet dropped.\n",
                        hlen, p.len));*/
                    //  free (drop) packet pbufs 
                    pbuf_free(p);
                    IP6_STATS_INC(ip6.lenerr);
                    IP6_STATS_INC(ip6.drop);
                    // goto ip6_input_cleanup;
                }

                //  Skip over this header. 
                hlen_tot = (hlen_tot + hlen);

                //  if segment left value is 0 in routing header, ignore the option 
                if (IP6_ROUT_SEG_LEFT(rout_hdr)) {
                    //  The length field of routing option header must be even 
                    if (rout_hdr._hlen & 0x1) {
                        //  Discard and send parameter field error 
                        icmp6_param_problem(p, ICMP6_PP_FIELD, &rout_hdr._hlen);
                        //          LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid routing type dropped\n"));
                        pbuf_free(p);
                        IP6_STATS_INC(ip6.drop);
                        // goto ip6_input_cleanup;
                    }

                    match (IP6_ROUT_TYPE(rout_hdr)) {
                        //  TODO: process routing by the type 
                        IP6_ROUT_TYPE2 | IP6_ROUT_RPL | _ => {
                            //  Discard unrecognized routing type and send parameter field error 
                            icmp6_param_problem(p, ICMP6_PP_FIELD, &IP6_ROUT_TYPE(rout_hdr));
                            //          LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid routing type dropped\n"));
                            pbuf_free(p);
                            IP6_STATS_INC(ip6.drop);
                            // goto ip6_input_cleanup;
                        }
                    }
                }

                pbuf_remove_header(p, hlen);
            }
            IP6_NEXTH_FRAGMENT => {
                let frag_hdr: &mut ip6_frag_hdr;
                //      LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with Fragment header\n"));

                frag_hdr = p.payload;

                //  Get next header type. 
                nexth = &IP6_FRAG_NEXTH(frag_hdr);

                //  Fragment Header length. 
                hlen = 8;

                //  Make sure this header fits in current pbuf. 
                if (hlen > p.len) {
                    /*LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                    ("IPv6 options header (hlen %"U16_F") does not fit in first pbuf (len %"U16_F"), IPv6 packet dropped.\n",
                        hlen, p.len));*/
                    //  free (drop) packet pbufs 
                    pbuf_free(p);
                    IP6_FRAG_STATS_INC(ip6_frag.lenerr);
                    IP6_FRAG_STATS_INC(ip6_frag.drop);
                    // goto ip6_input_cleanup;
                }

                hlen_tot = (hlen_tot + hlen);

                //  check payload length is multiple of 8 octets when mbit is set 
                if (IP6_FRAG_MBIT(frag_hdr) && (IP6H_PLEN(ip6hdr) & 0x7)) {
                    //  ipv6 payload length is not multiple of 8 octets 
                    icmp6_param_problem(p, ICMP6_PP_FIELD, &ip6hdr._plen);
                    //        LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with invalid payload length dropped\n"));
                    pbuf_free(p);
                    IP6_STATS_INC(ip6.drop);
                    // goto ip6_input_cleanup;
                }

                //  Offset == 0 and more_fragments == 0? 
                if ((frag_hdr._fragment_offset
                    & PP_HTONS(IP6_FRAG_OFFSET_MASK | IP6_FRAG_MORE_FLAG))
                    == 0)
                {
                    //  This is a 1-fragment packet. Skip this header and continue. 
                    pbuf_remove_header(p, hlen);
                } else {
                    //  reassemble the packet 
                    ip_data.current_ip_header_tot_len = hlen_tot;
                    p = ip6_reass(p);
                    //  packet not fully reassembled yet? 
                    if (p == None) {
                        // goto ip6_input_cleanup;
                    }

                    /* Returned p poto: i32 IPv6 header.
                     * Update all our variables and pointers and continue. */
                    ip6hdr = p.payload;
                    nexth = &IP6H_NEXTH(ip6hdr);
                    hlen = hlen_tot = IP6_HLEN;
                    pbuf_remove_header(p, IP6_HLEN);

                    //  LWIP_IPV6_REASS 
                    //  free (drop) packet pbufs 
                    //        LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with Fragment header dropped (with LWIP_IPV6_REASS==0)\n"));
                    pbuf_free(p);
                    IP6_STATS_INC(ip6.opterr);
                    IP6_STATS_INC(ip6.drop);
                    // goto ip6_input_cleanup;
                }
            }
            _ => {} // goto options_done;
        }

        if (*nexth == IP6_NEXTH_HOPBYHOP) {
            //  Hop-by-Hop header comes only as a first option 
            icmp6_param_problem(p, ICMP6_PP_HEADER, nexth);
            //      LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: packet with Hop-by-Hop options header dropped (only valid as a first option)\n"));
            pbuf_free(p);
            IP6_STATS_INC(ip6.drop);
            // goto ip6_input_cleanup;
        }
    }

    // options_done:

    //  send to upper layers 
    //  LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: \n"));
    ip6_debug_print(p);
    //  LWIP_DEBUGF(IP6_DEBUG, ("ip6_input: p.len %"U16_F" p.tot_len %"U16_F"\n", p.len, p.tot_len));

    ip_data.current_ip_header_tot_len = hlen_tot;

    //  p points to IPv6 header again for raw_input. 
    pbuf_add_header_force(p, hlen_tot);
    //  raw input did not eat the packet? 
    raw_status = raw_input(p, inp);
    if (raw_status != RAW_INPUT_EATEN) {
        //  Poto: i32 payload. 
        pbuf_remove_header(p, hlen_tot);
        //  LWIP_RAW 
        // {

        match (*nexth) {
            IP6_NEXTH_NONE => {
                pbuf_free(p);
            }

            IP6_NEXTH_UDP | IP6_NEXTH_UDPLITE => {
                udp_input(p, inp);
            }

            IP6_NEXTH_TCP => {
                tcp_input(p, inp);
            }

            IP6_NEXTH_ICMP6 => {
                icmp6_input(p, inp);
            }

            _ => {
                if (raw_status == RAW_INPUT_DELIVERED) {
                    //  @todo: ipv6 mib in-delivers? 
                } else {
                    //  p points to IPv6 header again for raw_input. 
                    pbuf_add_header_force(p, hlen_tot);
                    //  send ICMP parameter problem unless it was a multicast or ICMPv6 
                    if ((!ip6_addr_ismulticast(ip6_current_dest_addr()))
                        && (IP6H_NEXTH(ip6hdr) != IP6_NEXTH_ICMP6))
                    {
                        icmp6_param_problem(p, ICMP6_PP_HEADER, nexth);
                    }

                    //        LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip6_input: Unsupported transport protocol %"U16_F"\n", IP6H_NEXTH(ip6hdr)));
                    IP6_STATS_INC(ip6.proterr);
                    IP6_STATS_INC(ip6.drop);
                }
                pbuf_free(p);
            }
        }
    }

    // ip6_input_cleanup:
    ip_data.current_netif = None;
    ip_data.current_input_netif = None;
    ip_data.current_ip6_header = None;
    ip_data.current_ip_header_tot_len = 0;
    ip6_addr_set_zero(ip6_current_src_addr());
    ip6_addr_set_zero(ip6_current_dest_addr());

   return Ok(());
}

/*
* Sends an IPv6 packet on a network interface. This function constructs
* the IPv6 header. If the source IPv6 address is NULL, the IPv6 "ANY" address is
* used as source (usually during network startup). If the source IPv6 address it
* IP6_ADDR_ANY, the most appropriate IPv6 address of the outgoing network
* interface is filled in as source address. If the destination IPv6 address is
* LWIP_IP_HDRINCL, p is assumed to already include an IPv6 header and
* p.payload points to it instead of the data.
*
* @param p the packet to send (p.payload points to the data, e.g. next
           protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
           IPv6 header and p.payload points to that IPv6 header)
* @param src the source IPv6 address to send from (if src == IP6_ADDR_ANY, an
*         IP address of the netif is selected and used as source address.
*         if src == NULL, IP6_ADDR_ANY is used as source) (src is possibly not
*         properly zoned)
* @param dest the destination IPv6 address to send the packet to (possibly not
*             properly zoned)
* @param hl the Hop Limit value to be set in the IPv6 header
* @param tc the Traffic Class value to be set in the IPv6 header
* @param nexth the Next Header to be set in the IPv6 header
* @param netif the netif on which to send this packet
* @return ERR_OK if the packet was sent OK
*         ERR_BUF if p doesn't have enough space for IPv6/LINK headers
*         returns errors returned by netif.output_ip6
*/
pub fn ip6_output_if(
    p: &mut PacketBuffer,
    src: &mut ip6_addr_t,
    dest: &mut ip6_addr_t,
    hl: u8,
    tc: u8,
    nexth: u8,
    netif: &mut NetworkInterfaceCtx,
) {
 let src_used: &mut ip6_addr_t = src;
    if (dest != LWIP_IP_HDRINCL) {
        if (src != None && ip6_addr_isany(src)) {
            src_used = ip_2_ip6(ip6_select_source_address(netif, dest));
            if ((src_used == None) || ip6_addr_isany(src_used)) {
                //  No appropriate source address was found for this packet. 
                //        LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip6_output: No suitable source address for packet.\n"));
                IP6_STATS_INC(ip6.rterr);
                return ERR_RTE;
            }
        }
    }
    return ip6_output_if_src(p, src_used, dest, hl, tc, nexth, netif);
}

/*
 * Same as ip6_output_if() but 'src' address is not replaced by netif address
 * when it is 'any'.
 */
pub fn ip6_output_if_src(
    p: &mut PacketBuffer,
    src: &mut ip6_addr_t,
    dest: &mut ip6_addr_t,
    hl: u8,
    tc: u8,
    nexth: u8,
    netif: &mut NetworkInterfaceCtx,
) {
    let ip6hdr: &mut ip6_hdr;
    let dest_addr: ip6_addr_t;

    // LWIP_ASSERT_CORE_LOCKED()
    LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);

    //  Should the IPv6 header be generated or is it already included in p? 
    if (dest != LWIP_IP_HDRINCL) {
        /* If the destination address is scoped but lacks a zone, add a zone now,
         * based on the outgoing interface. The lower layers (e.g., nd6) absolutely
         * require addresses to be properly zoned for correctness. In some cases,
         * earlier attempts will have been made to add a zone to the destination,
         * but this function is the only one that is called in all (other) cases,
         * so we must do this here. */
        if (ip6_addr_lacks_zone(dest, Ip6Unknown)) {
            ip6_addr_copy(dest_addr, *dest);
            ip6_addr_assign_zone(&dest_addr, Ip6Unknown, netif);
            dest = &dest_addr;
        }

        //  generate IPv6 header 
        if (pbuf_add_header(p, IP6_HLEN)) {
            //      LWIP_DEBUGF(IP6_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip6_output: not enough room for IPv6 header in pbuf\n"));
            IP6_STATS_INC(ip6.err);
            return ERR_BUF;
        }

        ip6hdr = p.payload;
        // LWIP_ASSERT(
            "check that first pbuf can hold struct ip6_hdr",
            (p.len >= sizeof(ip6_hdr)),
        );

        IP6H_HOPLIM_SET(ip6hdr, hl);
        IP6H_NEXTH_SET(ip6hdr, nexth);

        //  dest cannot be NULL here 
        ip6_addr_copy_to_packed(ip6hdr.dest, *dest);

        IP6H_VTCFL_SET(ip6hdr, 6, tc, 0);
        IP6H_PLEN_SET(ip6hdr, (p.tot_len - IP6_HLEN));

        if (src == None) {
            src = IP6_ADDR_ANY6;
        }
        //  src cannot be NULL here 
        ip6_addr_copy_to_packed(ip6hdr.src, *src);
    } else {
        //  IP header already included in p 
        ip6hdr = p.payload;
        ip6_addr_copy_from_packed(dest_addr, ip6hdr.dest);
        ip6_addr_assign_zone(&dest_addr, Ip6Unknown, netif);
        dest = &dest_addr;
    }

    IP6_STATS_INC(ip6.xmit);

    //  LWIP_DEBUGF(IP6_DEBUG, ("ip6_output_if: %c%c%"U16_F"\n", netif.name[0], netif.name[1], netif.num));
    ip6_debug_print(p);

    {
        let i: i32;

        if (ip6_addr_isloopback(dest)) {
            return netif_loop_output(netif, p);
        }

        //     for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
        //       if (ip6_addr_isvalid(netif_ip6_addr_state(netif, i)) &&
        //           ip6_addr_cmp(dest, netif_ip6_addr(netif, i))) {
        //         //  Packet to self, enqueue it for loopback 
        // //        LWIP_DEBUGF(IP6_DEBUG, ("netif_loop_output()\n"));
        //         return netif_loop_output(netif, p);
        //       }
        //     }
    }

    if ((p.flags & PBUF_FLAG_MCASTLOOP) != 0) {
        netif_loop_output(netif, p);
    }

    //  don't fragment if interface has mtu set to 0 [loopif] 
    if (netif_mtu6(netif) && (p.tot_len > nd6_get_destination_mtu(dest, netif))) {
        return ip6_frag(p, netif, dest);
    }

    //  LWIP_DEBUGF(IP6_DEBUG, ("netif.output_ip6()\n"));
    return netif.output_ip6(netif, p, dest);
}

/*
* Simple interface to ip6_output_if. It finds the outgoing network
* interface and calls upon ip6_output_if to do the actual work.
*
* @param p the packet to send (p.payload points to the data, e.g. next
           protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
           IPv6 header and p.payload points to that IPv6 header)
* @param src the source IPv6 address to send from (if src == IP6_ADDR_ANY, an
*         IP address of the netif is selected and used as source address.
*         if src == NULL, IP6_ADDR_ANY is used as source)
* @param dest the destination IPv6 address to send the packet to
* @param hl the Hop Limit value to be set in the IPv6 header
* @param tc the Traffic Class value to be set in the IPv6 header
* @param nexth the Next Header to be set in the IPv6 header
*
* @return ERR_RTE if no route is found
*         see ip_output_if() for more return values
*/
pub fn ip6_output(
    p: &mut PacketBuffer,
    src: &mut ip6_addr_t,
    dest: &mut ip6_addr_t,
    hl: u8,
    tc: u8,
    nexth: u8,
) {
    let netif: &mut NetworkInterfaceCtx;
    let ip6hdr: &mut ip6_hdr;
    let src_addr: ip6_addr_t;
    let dest_addr: ip6_addr_t;

    LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);

    if (dest != LWIP_IP_HDRINCL) {
        netif = ip6_route(src, dest);
    } else {
        //  IP header included in p, read addresses. 
        ip6hdr = p.payload;
        ip6_addr_copy_from_packed(src_addr, ip6hdr.src);
        ip6_addr_copy_from_packed(dest_addr, ip6hdr.dest);
        netif = ip6_route(&src_addr, &dest_addr);
    }

    if (netif == None) {
        /*LWIP_DEBUGF(IP6_DEBUG, ("ip6_output: no route for %"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F"\n",
        IP6_ADDR_BLOCK1(dest),
        IP6_ADDR_BLOCK2(dest),
        IP6_ADDR_BLOCK3(dest),
        IP6_ADDR_BLOCK4(dest),
        IP6_ADDR_BLOCK5(dest),
        IP6_ADDR_BLOCK6(dest),
        IP6_ADDR_BLOCK7(dest),
        IP6_ADDR_BLOCK8(dest)));*/
        IP6_STATS_INC(ip6.rterr);
        return ERR_RTE;
    }

    return ip6_output_if(p, src, dest, hl, tc, nexth, netif);
}

/* Like ip6_output, but takes and addr_hpointer: i32 that is passed on to netif.addr_hint
*  before calling ip6_output_if.
*
* @param p the packet to send (p.payload points to the data, e.g. next
           protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
           IPv6 header and p.payload points to that IPv6 header)
* @param src the source IPv6 address to send from (if src == IP6_ADDR_ANY, an
*         IP address of the netif is selected and used as source address.
*         if src == NULL, IP6_ADDR_ANY is used as source)
* @param dest the destination IPv6 address to send the packet to
* @param hl the Hop Limit value to be set in the IPv6 header
* @param tc the Traffic Class value to be set in the IPv6 header
* @param nexth the Next Header to be set in the IPv6 header
* @param netif_hnetif: i32 output hpointer: i32 set to netif.hbefore: i32
*        calling ip_output_if()
*
* @return ERR_RTE if no route is found
*         see ip_output_if() for more return values
*/
pub fn ip6_output_hinted(
    p: &mut PacketBuffer,
    src: &mut ip6_addr_t,
    dest: &mut ip6_addr_t,
    hl: u8,
    tc: u8,
    nexth: u8,
    netif_hint: &mut NetifHint,
) {
    let netif: &mut NetworkInterfaceCtx;
    let ip6hdr: &mut ip6_hdr;
    let src_addr: ip6_addr_t;
    let dest_addr: ip6_addr_t;
    let err: err_t;

    LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);

    if (dest != LWIP_IP_HDRINCL) {
        netif = ip6_route(src, dest);
    } else {
        //  IP header included in p, read addresses. 
        ip6hdr = p.payload;
        ip6_addr_copy_from_packed(src_addr, ip6hdr.src);
        ip6_addr_copy_from_packed(dest_addr, ip6hdr.dest);
        netif = ip6_route(&src_addr, &dest_addr);
    }

    if (netif == None) {
        /*LWIP_DEBUGF(IP6_DEBUG, ("ip6_output: no route for %"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F":%"X16_F"\n",
        IP6_ADDR_BLOCK1(dest),
        IP6_ADDR_BLOCK2(dest),
        IP6_ADDR_BLOCK3(dest),
        IP6_ADDR_BLOCK4(dest),
        IP6_ADDR_BLOCK5(dest),
        IP6_ADDR_BLOCK6(dest),
        IP6_ADDR_BLOCK7(dest),
        IP6_ADDR_BLOCK8(dest)));*/
        IP6_STATS_INC(ip6.rterr);
        return ERR_RTE;
    }

    NETIF_SET_HINTS(netif, netif_hint);
    err = ip6_output_if(p, src, dest, hl, tc, nexth, netif);
    NETIF_RESET_HINTS(netif);

    return err;
}

/*
 * Add a hop-by-hop options header with a router alert option and padding.
 *
 * Used by MLD when sending a Multicast listener report/done message.
 *
 * @param p the packet to which we will prepend the options header
 * @param nexth the next header protocol number (e.g. IP6_NEXTH_ICMP6)
 * @param value the value of the router alert option data (e.g. IP6_ROUTER_ALERT_VALUE_MLD)
 * @return ERR_OK if hop-by-hop header was added, ERR_* otherwise
 */
pub fn ip6_options_add_hbh_ra(p: &mut PacketBuffer, nexth: u8, value: u8) {
    let opt_data: &mut Vec<u8>;
    let offset: u32 = 0;
    let hbh_hdr: &mut ip6_hbh_hdr;
    let opt_hdr: &mut ip6_opt_hdr;

    //  fixed 4 bytes for router alert option and 2 bytes padding 
 let hlen: u8 = (sizeof(ip6_opt_hdr) * 2) + IP6_ROUTER_ALERT_DLEN;
    //  Move pointer to make room for hop-by-hop options header. 
    if (pbuf_add_header(p, sizeof(ip6_hbh_hdr) + hlen)) {
        //    LWIP_DEBUGF(IP6_DEBUG, ("ip6_options: no space for options header\n"));
        IP6_STATS_INC(ip6.err);
        return ERR_BUF;
    }

    //  Set fields of Hop-by-Hop header 
    hbh_hdr = p.payload;
    IP6_HBH_NEXTH(hbh_hdr) = nexth;
    hbh_hdr._hlen = 0;
    offset = IP6_HBH_HLEN;

    //  Set router alert options to Hop-by-Hop extended option header 
    opt_hdr = (hbh_hdr + offset);
    IP6_OPT_TYPE(opt_hdr) = IP6_ROUTER_ALERT_OPTION;
    IP6_OPT_DLEN(opt_hdr) = IP6_ROUTER_ALERT_DLEN;
    offset += IP6_OPT_HLEN;

    //  Set router alert option data 
    opt_data = hbh_hdr + offset;
    opt_data[0] = value;
    opt_data[1] = 0;
    offset += IP6_OPT_DLEN(opt_hdr);

    //  add 2 bytes padding to make 8 bytes Hop-by-Hop header length 
    opt_hdr = (hbh_hdr + offset);
    IP6_OPT_TYPE(opt_hdr) = IP6_PADN_OPTION;
    IP6_OPT_DLEN(opt_hdr) = 0;

   return Ok(());
}

/* Pran: i32 IPv6 header by using LWIP_DEBUGF
 * @param p an IPv6 packet, p.payload pointing to the IPv6 header
 */
pub fn ip6_debug_print(p: &mut PacketBuffer) {
    let ip6hdr: &mut ip6_hdr = p.payload;

    //  LWIP_DEBUGF(IP6_DEBUG, ("IPv6 header:\n"));
    //  LWIP_DEBUGF(IP6_DEBUG, ("+-------------------------------+\n"));
    /*LWIP_DEBUGF(IP6_DEBUG, ("| %2"U16_F" |  %3"U16_F"  |      %7"U32_F"     | (ver, class, flow)\n",
    IP6H_V(ip6hdr),
    IP6H_TC(ip6hdr),
    IP6H_FL(ip6hdr)));*/
    //  LWIP_DEBUGF(IP6_DEBUG, ("+-------------------------------+\n"));
    /*LWIP_DEBUGF(IP6_DEBUG, ("|     %5"U16_F"     |  %3"U16_F"  |  %3"U16_F"  | (plen, nexth, hopl)\n",
    IP6H_PLEN(ip6hdr),
    IP6H_NEXTH(ip6hdr),
    IP6H_HOPLIM(ip6hdr)));*/
    //  LWIP_DEBUGF(IP6_DEBUG, ("+-------------------------------+\n"));
    /*LWIP_DEBUGF(IP6_DEBUG, ("|  %4"X32_F" |  %4"X32_F" |  %4"X32_F" |  %4"X32_F" | (src)\n",
    IP6_ADDR_BLOCK1(&(ip6hdr.src)),
    IP6_ADDR_BLOCK2(&(ip6hdr.src)),
    IP6_ADDR_BLOCK3(&(ip6hdr.src)),
    IP6_ADDR_BLOCK4(&(ip6hdr.src))));*//*LWIP_DEBUGF(IP6_DEBUG, ("|  %4"X32_F" |  %4"X32_F" |  %4"X32_F" |  %4"X32_F" |\n",
    IP6_ADDR_BLOCK5(&(ip6hdr.src)),
    IP6_ADDR_BLOCK6(&(ip6hdr.src)),
    IP6_ADDR_BLOCK7(&(ip6hdr.src)),
    IP6_ADDR_BLOCK8(&(ip6hdr.src))));*/
    //  LWIP_DEBUGF(IP6_DEBUG, ("+-------------------------------+\n"));
    /*LWIP_DEBUGF(IP6_DEBUG, ("|  %4"X32_F" |  %4"X32_F" |  %4"X32_F" |  %4"X32_F" | (dest)\n",
    IP6_ADDR_BLOCK1(&(ip6hdr.dest)),
    IP6_ADDR_BLOCK2(&(ip6hdr.dest)),
    IP6_ADDR_BLOCK3(&(ip6hdr.dest)),
    IP6_ADDR_BLOCK4(&(ip6hdr.dest))));*//*LWIP_DEBUGF(IP6_DEBUG, ("|  %4"X32_F" |  %4"X32_F" |  %4"X32_F" |  %4"X32_F" |\n",
    IP6_ADDR_BLOCK5(&(ip6hdr.dest)),
    IP6_ADDR_BLOCK6(&(ip6hdr.dest)),
    IP6_ADDR_BLOCK7(&(ip6hdr.dest)),
    IP6_ADDR_BLOCK8(&(ip6hdr.dest))));*/
    //  LWIP_DEBUGF(IP6_DEBUG, ("+-------------------------------+\n"));
}
