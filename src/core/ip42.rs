/*
 * @file
 * This is the IPv4 layer implementation for incoming and outgoing IP traffic.
 *
 * @see ip_frag.c
 *
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


























/* Set this to 0 in the rare case of wanting to call an extra function to
 * generate the IP checksum (in contrast to calculating it on-the-fly). */


pub const LWIP_INLINE_IP_CHKSUM: u32 = 0;
 /* LWIP_CHECKSUM_CTRL_PER_NETIF */
// #define LWIP_INLINE_IP_CHKSUM   1




pub const CHECKSUM_GEN_IP_INLINE: u32 =  1;

pub const CHECKSUM_GEN_IP_INLINE: u32 = 0;



pub const IP_ACCEPT_LINK_LAYER_ADDRESSING: u32 = 1;

/* Some defines for DHCP to let link-layer-addressed packets through while the
 * netif is down.
 * To use this in your own application/protocol, define LWIP_IP_ACCEPT_UDP_PORT(port)
 * to return 1 if the port is accepted and 0 if the port is not accepted.
 */

/* accept DHCP client port and custom port */
pub fn IP_ACCEPT_LINK_LAYER_ADDRESSED_PORT(port: u16) {(((port) == PP_NTOHS(LWIP_IANA_PORT_DHCP_CLIENT)) 
         || (LWIP_IP_ACCEPT_UDP_PORT(port)))}
// #elif defined(LWIP_IP_ACCEPT_UDP_PORT) /* LWIP_DHCP && defined(LWIP_IP_ACCEPT_UDP_PORT) */
/* accept custom port only */
// pbuf fnIP_ACCEPT_LINK_LAYER_ADDRESSED_PORT(port) (LWIP_IP_ACCEPT_UDP_PORT(port))
 /* LWIP_DHCP && defined(LWIP_IP_ACCEPT_UDP_PORT) */
/* accept DHCP client port only */
// #define IP_ACCEPT_LINK_LAYER_ADDRESSED_PORT(port) ((port) == PP_NTOHS(LWIP_IANA_PORT_DHCP_CLIENT))


 /* LWIP_DHCP */
pub const IP_ACCEPT_LINK_LAYER_ADDRESSING: u32 = 0;


/* The IP header ID of the next outgoing IP packet */
static ip_id: u16;


/* The default netif used for multicast */
static ip4_default_multicast_netif: &mut NetIfc;

/*
 * @ingroup ip4
 * Set a default netif for IPv4 multicast. */
pub fn 
ip4_set_default_multicast_netif(default_multicast_netif: &mut NetIfc)
{
  ip4_default_multicast_netif = default_multicast_netif;
}



/*
 * Source based IPv4 routing must be fully implemented in
 * LWIP_HOOK_IP4_ROUTE_SRC(). This function only provides the parameters.
 */
pub fn ip4_route_src(src: &mut ip4_addr,  dest: &mut ip4_addr) -> NetIfc
{
  if (src != NULL) {
    /* when src==NULL, the hook is called from ip4_route(dest) */
    let netif: &mut NetIfc = LWIP_HOOK_IP4_ROUTE_SRC(src, dest);
    if (netif != NULL) {
      return netif;
    }
  }
  return ip4_route(dest);
}


/*
 * Finds the appropriate network interface for a given IP address. It
 * searches the list of network interfaces linearly. A match is found
 * if the masked IP address of the network interface equals the masked
 * IP address given to the function.
 *
 * @param dest the destination IP address for which to find the route
 * @return the netif on which to send to reach dest
 */
pub fn ip4_route(dest: &mut ip4_addr) -> NetIfc
{

  let netif: &mut NetIfc;

  LWIP_ASSERT_CORE_LOCKED();


  /* Use administratively selected interface for multicast by default */
  if (ip4_addr_ismulticast(dest) && ip4_default_multicast_netif) {
    return ip4_default_multicast_netif;
  }


  /* bug #54569: in case LWIP_SINGLE_NETIF=1 and LWIP_DEBUGF() disabled, the following loop is optimized away */
  

  /* iterate through netifs */
  // NETIF_FOREACH(netif) {
  //   /* is the netif up, does it have a link and a valid address? */
  //   if (netif_is_up(netif) && netif_is_link_up(netif) && !ip4_addr_isany_val(*netif_ip4_addr(netif))) {
  //     /* network mask matches? */
  //     if (ip4_addr_netcmp(dest, netif_ip4_addr(netif), netif_ip4_netmask(netif))) {
  //       /* return netif on which to forward IP packet */
  //       return netif;
  //     }
  //     /* gateway matches on a non broadcast interface? (i.e. peer in a poto: i32 pointerface: i32) */
  //     if (((netif.flags & NETIF_FLAG_BROADCAST) == 0) && ip4_addr_cmp(dest, netif_ip4_gw(netif))) {
  //       /* return netif on which to forward IP packet */
  //       return netif;
  //     }
  //   }
  // }


  /* loopif is disabled, looopback traffic is passed through any netif */
  if (ip4_addr_isloopback(dest)) {
    /* don't check for link on loopback traffic */
    if (netif_default != NULL && netif_is_up(netif_default)) {
      return netif_default;
    }
    /* default netif is not up, just use any netif for loopback traffic */
    // NETIF_FOREACH(netif) {
    //   if (netif_is_up(netif)) {
    //     return netif;
    //   }
    // }
    return NULL;
  }



  netif = LWIP_HOOK_IP4_ROUTE_SRC(NULL, dest);
  if (netif != NULL) {
    return netif;
  }
// #elif defined(LWIP_HOOK_IP4_ROUTE)
//   netif = LWIP_HOOK_IP4_ROUTE(dest);
//   if (netif != NULL) {
//     return netif;
//   }



  if ((netif_default == NULL) || !netif_is_up(netif_default) || !netif_is_link_up(netif_default) ||
      ip4_addr_isany_val(*netif_ip4_addr(netif_default)) || ip4_addr_isloopback(dest)) {
    /* No matching netif found and default netif is not usable.
       If this is not good enough for you, use LWIP_HOOK_IP4_ROUTE() */
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_route: No route to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                ip4_addr1_16(dest), ip4_addr2_16(dest), ip4_addr3_16(dest), ip4_addr4_16(dest)));*/
    IP_STATS_INC(ip.rterr);
    MIB2_STATS_INC(mib2.ipoutnoroutes);
    return NULL;
  }

  return netif_default;
}


/*
 * Determine whether an IP address is in a reserved set of addresses
 * that may not be forwarded, or whether datagrams to that destination
 * may be forwarded.
 * @param p the packet to forward
 * @return 1: can forward 0: discard
 */
pub fn ip4_canforward(p: &mut pbuf)
{
  let addr: u32 = lwip_htonl(ip4_addr_get_u32(ip4_current_dest_addr()));


  let ret: i32 = LWIP_HOOK_IP4_CANFORWARD(p, addr);
  if (ret >= 0) {
    return ret;
  }


  if (p.flags & PBUF_FLAG_LLBCAST) {
    /* don't route link-layer broadcasts */
    return 0;
  }
  if ((p.flags & PBUF_FLAG_LLMCAST) || IP_MULTICAST(addr)) {
    /* don't route link-layer multicasts (use LWIP_HOOK_IP4_CANFORWARD instead) */
    return 0;
  }
  if (IP_EXPERIMENTAL(addr)) {
    return 0;
  }
  if (IP_CLASSA(addr)) {
    let net: u32 = addr & IP_CLASSA_NET;
    if ((net == 0) || (net == (IP_LOOPBACKNET << IP_CLASSA_NSHIFT))) {
      /* don't route loopback packets */
      return 0;
    }
  }
  return 1;
}

/*
 * Forwards an IP packet. It finds an appropriate route for the
 * packet, decrements the TTL value of the packet, adjusts the
 * checksum and outputs the packet on the appropriate interface.
 *
 * @param p the packet to forward (p.payload points to IP header)
 * @param iphdr the IP header of the input packet
 * @param inp the netif on which this packet was received
 */
pub fn
ip4_forward(p: &mut pbuf, iphdr: &mut ip_hdr, inp: &mut NetIfc)
{
  let netif: &mut NetIfc;

  PERF_START;
  

  if (!ip4_canforward(p)) {
    // goto return_noroute;
  }

  /* RFC3927 2.7: do not forward link-local addresses */
  if (ip4_addr_islinklocal(ip4_current_dest_addr())) {
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_forward: not forwarding LLA %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                           ip4_addr1_16(ip4_current_dest_addr()), ip4_addr2_16(ip4_current_dest_addr()),
                           ip4_addr3_16(ip4_current_dest_addr()), ip4_addr4_16(ip4_current_dest_addr())));*/
    // goto return_noroute;
  }

  /* Find network interface where to forward this IP packet to. */
  netif = ip4_route_src(ip4_current_src_addr(), ip4_current_dest_addr());
  if (netif == NULL) {
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_forward: no forwarding route for %"U16_F".%"U16_F".%"U16_F".%"U16_F" found\n",
                           ip4_addr1_16(ip4_current_dest_addr()), ip4_addr2_16(ip4_current_dest_addr()),
                           ip4_addr3_16(ip4_current_dest_addr()), ip4_addr4_16(ip4_current_dest_addr())));*/
    /* @todo: send ICMP_DUR_NET? */
    // goto return_noroute;
  }

  /* Do not forward packets onto the same network interface on which
   * they arrived. */
  if (netif == inp) {
//    LWIP_DEBUGF(IP_DEBUG, ("ip4_forward: not bouncing packets back on incoming interface.\n"));
    // goto return_noroute;
  }


  /* decrement TTL */
  IPH_TTL_SET(iphdr, IPH_TTL(iphdr) - 1);
  /* send ICMP if TTL == 0 */
  if (IPH_TTL(iphdr) == 0) {
    MIB2_STATS_INC(mib2.ipinhdrerrors);

    /* Don't send ICMP messages in response to ICMP messages */
    if (IPH_PROTO(iphdr) != IP_PROTO_ICMP) {
      icmp_time_exceeded(p, ICMP_TE_TTL);
    }

    return;
  }

  /* Incrementally update the IP checksum. */
  if (IPH_CHKSUM(iphdr) >= PP_HTONS(0xffff - 0x100)) {
    IPH_CHKSUM_SET(iphdr, (IPH_CHKSUM(iphdr) + PP_HTONS(0x100) + 1));
  } else {
    IPH_CHKSUM_SET(iphdr, (IPH_CHKSUM(iphdr) + PP_HTONS(0x100)));
  }
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_forward: forwarding packet to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                         ip4_addr1_16(ip4_current_dest_addr()), ip4_addr2_16(ip4_current_dest_addr()),
                         ip4_addr3_16(ip4_current_dest_addr()), ip4_addr4_16(ip4_current_dest_addr())));*/

  IP_STATS_INC(ip.fw);
  MIB2_STATS_INC(mib2.ipforwdatagrams);
  IP_STATS_INC(ip.xmit);

  PERF_STOP("ip4_forward");
  /* don't fragment if interface has mtu set to 0 [loopif] */
  if (netif.mtu && (p.tot_len > netif.mtu)) {
    if ((IPH_OFFSET(iphdr) & PP_NTOHS(IP_DF)) == 0) {

      ip4_frag(p, netif, ip4_current_dest_addr());
 /* IP_FRAG */
      /* @todo: send ICMP Destination Unreachable code 13 "Communication administratively prohibited"? */

    } else {

      /* send ICMP Destination Unreachable code 4: "Fragmentation Needed and DF Set" */
      icmp_dest_unreach(p, ICMP_DUR_FRAG);

    }
    return;
  }
  /* transmit pbuf on chosen interface */
  netif.output(netif, p, ip4_current_dest_addr());
  return;
// return_noroute:
  MIB2_STATS_INC(mib2.ipoutnoroutes);
}


/* Return true if the current input packet should be accepted on this netif */
pub fn ip4_input_accept(netif: &mut NetIfc)
{
/*LWIP_DEBUGF(IP_DEBUG, ("ip_input: iphdr.dest 0x%"X32_F" netif.ip_addr 0x%"X32_F" (0x%"X32_F", 0x%"X32_F", 0x%"X32_F")\n",
                         ip4_addr_get_u32(ip4_current_dest_addr()), ip4_addr_get_u32(netif_ip4_addr(netif)),
                         ip4_addr_get_u32(ip4_current_dest_addr()) & ip4_addr_get_u32(netif_ip4_netmask(netif)),
                         ip4_addr_get_u32(netif_ip4_addr(netif)) & ip4_addr_get_u32(netif_ip4_netmask(netif)),
                         ip4_addr_get_u32(ip4_current_dest_addr()) & !ip4_addr_get_u32(netif_ip4_netmask(netif))));*/

  /* interface is up and configured? */
  if ((netif_is_up(netif)) && (!ip4_addr_isany_val(*netif_ip4_addr(netif)))) {
    /* unicast to this interface address? */
    if (ip4_addr_cmp(ip4_current_dest_addr(), netif_ip4_addr(netif)) ||
        /* or broadcast on this interface network address? */
        ip4_addr_isbroadcast(ip4_current_dest_addr(), netif)

        || (ip4_addr_get_u32(ip4_current_dest_addr()) == PP_HTONL(IPADDR_LOOPBACK))

       ) {
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_input: packet accepted on interface %c%c\n",
                             netif.name[0], netif.name[1]));*/
      /* accept on this netif */
      return 1;
    }

    /* connections to link-local addresses must persist after changing
        the netif's address (RFC3927 ch. 1.9) */
    if (autoip_accept_packet(netif, ip4_current_dest_addr())) {
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_input: LLA packet accepted on interface %c%c\n",
                             netif.name[0], netif.name[1]));*/
      /* accept on this netif */
      return 1;
    }

  }
  return 0;
}

/*
 * This function is called by the network interface device driver when
 * an IP packet is received. The function does the basic checks of the
 * IP header such as packet size being at least larger than the header
 * size etc. If the packet was not destined for us, the packet is
 * forwarded (using ip_forward). The IP checksum is always checked.
 *
 * Finally, the packet is sent to the upper layer protocol input function.
 *
 * @param p the received IP packet (p.payload points to IP header)
 * @param inp the netif on which this packet was received
 * @return ERR_OK if the packet was processed (could return ERR_* if it wasn't
 *         processed, but currently always returns ERR_OK)
 */
pub fn 
ip4_input(p: &mut pbuf, inp: &mut NetIfc)
{
  const iphdr: &mut ip_hdr;
  let netif: &mut NetIfc;
  let iphdr_hlen: u16;
  let iphdr_len: u16;
  let check_ip_src: i32 = 1;
  let raw_status: raw_input_state_t;

  LWIP_ASSERT_CORE_LOCKED();

  IP_STATS_INC(ip.recv);
  MIB2_STATS_INC(mib2.ipinreceives);

  /* identify the IP header */
  iphdr = p.payload;
  if (IPH_V(iphdr) != 4) {
//    LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_WARNING, ("IP packet dropped due to bad version number %"U16_F"\n", IPH_V(iphdr)));
    ip4_debug_print(p);
    pbuf_free(p);
    IP_STATS_INC(ip.err);
    IP_STATS_INC(ip.drop);
    MIB2_STATS_INC(mib2.ipinhdrerrors);
    return ERR_OK;
  }


  if (LWIP_HOOK_IP4_INPUT(p, inp)) {
    /* the packet has been eaten */
    return ERR_OK;
  }


  /* obtain IP header length in bytes */
  iphdr_hlen = IPH_HL_BYTES(iphdr);
  /* obtain ip length in bytes */
  iphdr_len = lwip_ntohs(IPH_LEN(iphdr));

  /* Trim pbuf. This is especially required for packets < 60 bytes. */
  if (iphdr_len < p.tot_len) {
    pbuf_realloc(p, iphdr_len);
  }

  /* header length exceeds first pbuf length, or ip length exceeds total pbuf length? */
  if ((iphdr_hlen > p.len) || (iphdr_len > p.tot_len) || (iphdr_hlen < IP_HLEN)) {
    if (iphdr_hlen < IP_HLEN) {
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                  ("ip4_input: short IP header (%"U16_F" bytes) received, IP packet dropped\n", iphdr_hlen));*/
    }
    if (iphdr_hlen > p.len) {
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                  ("IP header (len %"U16_F") does not fit in first pbuf (len %"U16_F"), IP packet dropped.\n",
                   iphdr_hlen, p.len));*/
    }
    if (iphdr_len > p.tot_len) {
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                  ("IP (len %"U16_F") is longer than pbuf (len %"U16_F"), IP packet dropped.\n",
                   iphdr_len, p.tot_len));*/
    }
    /* free (drop) packet pbufs */
    pbuf_free(p);
    IP_STATS_INC(ip.lenerr);
    IP_STATS_INC(ip.drop);
    MIB2_STATS_INC(mib2.ipindiscards);
    return ERR_OK;
  }

  /* verify checksum */

  // IF__NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_CHECK_IP) {
    if inp::CHECKSUM_ENABLED(NETIF_CHECKSUM_CHECK_IP) {
    if (inet_chksum(iphdr, iphdr_hlen) != 0) {
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
                  ("Checksum (0x%"X16_F") failed, IP packet dropped.\n", inet_chksum(iphdr, iphdr_hlen)));*/
      ip4_debug_print(p);
      pbuf_free(p);
      IP_STATS_INC(ip.chkerr);
      IP_STATS_INC(ip.drop);
      MIB2_STATS_INC(mib2.ipinhdrerrors);
      return ERR_OK;
    }
  }


  /* copy IP addresses to aligned ip_addr_t */
  ip_addr_copy_from_ip4(ip_data.current_iphdr_dest, iphdr.dest);
  ip_addr_copy_from_ip4(ip_data.current_iphdr_src, iphdr.src);

  /* match packet against an interface, i.e. is this packet for us? */
  if (ip4_addr_ismulticast(ip4_current_dest_addr())) {

    if ((inp.flags & NETIF_FLAG_IGMP) && (igmp_lookfor_group(inp, ip4_current_dest_addr()))) {
      /* IGMP snooping matches need 0.0.0.0 to be allowed as source address (RFC 4541) */
      let mut if_addr: LwipAddr;
      IP4_ADDR(&allsystems, 224, 0, 0, 1);
      if (ip4_addr_cmp(ip4_current_dest_addr(), &allsystems) &&
          ip4_addr_isany(ip4_current_src_addr())) {
        check_ip_src = 0;
      }
      netif = inp;
    } else {
      netif = NULL;
    }
 /* LWIP_IGMP */
    if ((netif_is_up(inp)) && (!ip4_addr_isany_val(*netif_ip4_addr(inp)))) {
      netif = inp;
    } else {
      netif = NULL;
    }

  } else {
    /* start trying with inp. if that's not acceptable, start walking the
       list of configured netifs. */
    if (ip4_input_accept(inp)) {
      netif = inp;
    } else {
      netif = NULL;

      /* Packets sent to the loopback address must not be accepted on an
       * interface that does not have the loopback address assigned to it,
       * unless a non-loopback interface is used for loopback traffic. */
      if (!ip4_addr_isloopback(ip4_current_dest_addr()))

      {

        // NETIF_FOREACH(netif) {
        //   if (netif == inp) {
        //     /* we checked that before already */
        //     continue;
        //   }
        //   if (ip4_input_accept(netif)) {
        //     break;
        //   }
        // }

      }
    }
  }


  /* Pass DHCP messages regardless of destination address. DHCP traffic is addressed
   * using link layer addressing (such as Ethernet MAC) so we must not filter on IP.
   * According to RFC 1542 section 3.1.1, referred by RFC 2131).
   *
   * If you want to accept private broadcast communication while a netif is down,
   * define LWIP_IP_ACCEPT_UDP_PORT(dst_port), e.g.:
   *
   * // #define LWIP_IP_ACCEPT_UDP_PORT(dst_port) ((dst_port) == PP_NTOHS(12345))
   */
  if (netif == NULL) {
    /* remote port is DHCP server? */
    if (IPH_PROTO(iphdr) == IP_PROTO_UDP) {
      const udphdr: &mut udp_hdr = (iphdr + iphdr_hlen);
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE, ("ip4_input: UDP packet to DHCP client port %"U16_F"\n",
                                              lwip_ntohs(udphdr.dest)));*/
      if (IP_ACCEPT_LINK_LAYER_ADDRESSED_PORT(udphdr.dest)) {
//        LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE, ("ip4_input: DHCP packet accepted.\n"));
        netif = inp;
        check_ip_src = 0;
      }
    }
  }


  /* broadcast or multicast packet source address? Compliant with RFC 1122: 3.2.1.3 */

  if (check_ip_src

      /* DHCP servers need 0.0.0.0 to be allowed as source address (RFC 1.1.2.2: 3.2.1.3/a) */
      && !ip4_addr_isany_val(*ip4_current_src_addr())

     )

  {
    if ((ip4_addr_isbroadcast(ip4_current_src_addr(), inp)) ||
        (ip4_addr_ismulticast(ip4_current_src_addr()))) {
      /* packet source is not valid */
//      LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_LEVEL_WARNING, ("ip4_input: packet source is not valid.\n"));
      /* free (drop) packet pbufs */
      pbuf_free(p);
      IP_STATS_INC(ip.drop);
      MIB2_STATS_INC(mib2.ipinaddrerrors);
      MIB2_STATS_INC(mib2.ipindiscards);
      return ERR_OK;
    }
  }

  /* packet not for us? */
  if (netif == NULL) {
    /* packet not for us, route or discard */
//    LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE, ("ip4_input: packet not for us.\n"));

    /* non-broadcast packet? */
    if (!ip4_addr_isbroadcast(ip4_current_dest_addr(), inp)) {
      /* try to forward IP packet on (other) interfaces */
      ip4_forward(p, p.payload, inp);
    } else

    {
      IP_STATS_INC(ip.drop);
      MIB2_STATS_INC(mib2.ipinaddrerrors);
      MIB2_STATS_INC(mib2.ipindiscards);
    }
    pbuf_free(p);
    return ERR_OK;
  }
  /* packet consists of multiple fragments? */
  if ((IPH_OFFSET(iphdr) & PP_HTONS(IP_OFFMASK | IP_MF)) != 0) {
/*LWIP_DEBUGF(IP_DEBUG, ("IP packet is a fragment (id=0x%04"X16_F" tot_len=%"U16_F" len=%"U16_F" MF=%"U16_F" offset=%"U16_F"), calling ip4_reass()\n",
                           lwip_ntohs(IPH_ID(iphdr)), p.tot_len, lwip_ntohs(IPH_LEN(iphdr)), !!(IPH_OFFSET(iphdr) & PP_HTONS(IP_MF)), ((lwip_ntohs(IPH_OFFSET(iphdr)) & IP_OFFMASK) * 8)));*/
    /* reassemble the packet*/
    p = ip4_reass(p);
    /* packet not fully reassembled yet? */
    if (p == NULL) {
      return ERR_OK;
    }
    iphdr = p.payload;
 /* IP_REASSEMBLY == 0, no packet fragment reassembly code present */
    pbuf_free(p);
/*LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("IP packet dropped since it was fragmented (0x%"X16_F") (while IP_REASSEMBLY == 0).\n",
                lwip_ntohs(IPH_OFFSET(iphdr))));*/
    IP_STATS_INC(ip.opterr);
    IP_STATS_INC(ip.drop);
    /* unsupported protocol feature */
    MIB2_STATS_INC(mib2.ipinunknownprotos);
    return ERR_OK;

  }




  /* there is an extra "router alert" option in IGMP messages which we allow for but do not police */
  if ((iphdr_hlen > IP_HLEN) &&  (IPH_PROTO(iphdr) != IP_PROTO_IGMP)) {

  if (iphdr_hlen > IP_HLEN) {

//    LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("IP packet dropped since there were IP options (while IP_OPTIONS_ALLOWED == 0).\n"));
    pbuf_free(p);
    IP_STATS_INC(ip.opterr);
    IP_STATS_INC(ip.drop);
    /* unsupported protocol feature */
    MIB2_STATS_INC(mib2.ipinunknownprotos);
    return ERR_OK;
  }


  /* send to upper layers */
//  LWIP_DEBUGF(IP_DEBUG, ("ip4_input: \n"));
  ip4_debug_print(p);
//  LWIP_DEBUGF(IP_DEBUG, ("ip4_input: p.len %"U16_F" p.tot_len %"U16_F"\n", p.len, p.tot_len));

  ip_data.current_netif = netif;
  ip_data.current_input_netif = inp;
  ip_data.current_ip4_header = iphdr;
  ip_data.current_ip_header_tot_len = IPH_HL_BYTES(iphdr);


  /* raw input did not eat the packet? */
  raw_status = raw_input(p, inp);
  if (raw_status != RAW_INPUT_EATEN)

  {
    pbuf_remove_header(p, iphdr_hlen); /* Move to payload, no check necessary. */

    match (IPH_PROTO(iphdr)) {

      IP_PROTO_UDP |

      IP_PROTO_UDPLITE =>{

        MIB2_STATS_INC(mib2.ipindelivers);
        udp_input(p, inp);}
        


      IP_PROTO_TCP =>{
        MIB2_STATS_INC(mib2.ipindelivers);
        tcp_input(p, inp);}
        


      IP_PROTO_ICMP =>{
        MIB2_STATS_INC(mib2.ipindelivers);
        icmp_input(p, inp);}
        


      IP_PROTO_IGMP =>{
        igmp_input(p, inp, ip4_current_dest_addr());}
        

      _ =>{

        if (raw_status == RAW_INPUT_DELIVERED) {
          MIB2_STATS_INC(mib2.ipindelivers);
        } else

        {

          /* send ICMP destination protocol unreachable unless is was a broadcast */
          if (!ip4_addr_isbroadcast(ip4_current_dest_addr(), netif) &&
              !ip4_addr_ismulticast(ip4_current_dest_addr())) {
            pbuf_header_force(p, iphdr_hlen); /* Move to ip header, no check necessary. */
            icmp_dest_unreach(p, ICMP_DUR_PROTO);
          }


//          LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("Unsupported transport protocol %"U16_F"\n", IPH_PROTO(iphdr)));

          IP_STATS_INC(ip.proterr);
          IP_STATS_INC(ip.drop);
          MIB2_STATS_INC(mib2.ipinunknownprotos);
        }
        pbuf_free(p);}
    }
  }
  }

  /* @todo: this is not really necessary... */
  ip_data.current_netif = NULL;
  ip_data.current_input_netif = NULL;
  ip_data.current_ip4_header = NULL;
  ip_data.current_ip_header_tot_len = 0;
  ip4_addr_set_any(ip4_current_src_addr());
  ip4_addr_set_any(ip4_current_dest_addr());

  return ERR_OK;
}

/*
 * Sends an IP packet on a network interface. This function constructs
 * the IP header and calculates the IP header checksum. If the source
 * IP address is NULL, the IP address of the outgoing network
 * interface is filled in as source address.
 * If the destination IP address is LWIP_IP_HDRINCL, p is assumed to already
 * include an IP header and p.payload points to it instead of the data.
 *
 * @param p the packet to send (p.payload points to the data, e.g. next
            protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
            IP header and p.payload points to that IP header)
 * @param src the source IP address to send from (if src == IP4_ADDR_ANY, the
 *         IP  address of the netif used to send is used as source address)
 * @param dest the destination IP address to send the packet to
 * @param ttl the TTL value to be set in the IP header
 * @param tos the TOS value to be set in the IP header
 * @param proto the PROTOCOL to be set in the IP header
 * @param netif the netif on which to send this packet
 * @return ERR_OK if the packet was sent OK
 *         ERR_BUF if p doesn't have enough space for IP/LINK headers
 *         returns errors returned by netif.output
 *
 * @note ip_id: RFC791 "some host may be able to simply use
 *  unique identifiers independent of destination"
 */
pub fn 
ip4_output_if(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
              ttl: u8, tos: u8,
              proto: u8, netif: &mut NetIfc)
{

  return ip4_output_if_opt(p, src, dest, ttl, tos, proto, netif, NULL, 0);
}

/*
 * Same as ip_output_if() but with the possibility to include IP options:
 *
 * @ param ip_options pointer to the IP options, copied into the IP header
 * @ param optlen length of ip_options
 */
pub fn 
ip4_output_if_opt(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
                  ttl: u8, tos: u8, proto: u8, netif: &mut NetIfc, ip_options: &mut (),
                  optlen: u16)
{

  const src_used: &mut ip4_addr = src;
  if (dest != LWIP_IP_HDRINCL) {
    if (ip4_addr_isany(src)) {
      src_used = netif_ip4_addr(netif);
    }
  }


  return ip4_output_if_opt_src(p, src_used, dest, ttl, tos, proto, netif,
                               ip_options, optlen);
 /* IP_OPTIONS_SEND */
  return ip4_output_if_src(p, src_used, dest, ttl, tos, proto, netif);

}

/*
 * Same as ip_output_if() but 'src' address is not replaced by netif address
 * when it is 'any'.
 */
pub fn 
ip4_output_if_src(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
                  ttl: u8, tos: u8,
                  proto: u8, netif: &mut NetIfc)
{

  return ip4_output_if_opt_src(p, src, dest, ttl, tos, proto, netif, NULL, 0);
}

/*
 * Same as ip_output_if_opt() but 'src' address is not replaced by netif address
 * when it is 'any'.
 */
pub fn 
ip4_output_if_opt_src(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
                      ttl: u8, tos: u8, proto: u8, netif: &mut NetIfc, ip_options: &mut (),
                      optlen: u16)
{

  let iphdr: &mut ip_hdr;
  let mut if_addr: LwipAddr;
  let chk_sum: u32 = 0;

  LWIP_ASSERT_CORE_LOCKED();
  LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);

  MIB2_STATS_INC(mib2.ipoutrequests);

  /* Should the IP header be generated or is it already included in p? */
  if (dest != LWIP_IP_HDRINCL) {
    let ip_hlen: u16 = IP_HLEN;
    let optlen_aligned: u16 = 0;
    if (optlen != 0) {

      let leti: i32;

      if (optlen > (IP_HLEN_MAX - IP_HLEN)) {
        /* optlen too long */
//        LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output_if_opt: optlen too long\n"));
        IP_STATS_INC(ip.err);
        MIB2_STATS_INC(mib2.ipoutdiscards);
        return ERR_VAL;
      }
      /* round up to a multiple of 4 */
      optlen_aligned = ((optlen + 3) & !3);
      ip_hlen = (ip_hlen + optlen_aligned);
      /* First write in the IP options */
      if (pbuf_add_header(p, optlen_aligned)) {
//        LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output_if_opt: not enough room for IP options in pbuf\n"));
        IP_STATS_INC(ip.err);
        MIB2_STATS_INC(mib2.ipoutdiscards);
        return ERR_BUF;
      }
      MEMCPY(p.payload, ip_options, optlen);
      if (optlen < optlen_aligned) {
        /* zero the remaining bytes */
        memset((p.payload) + optlen, 0, (optlen_aligned - optlen));
      }

      // for (i = 0; i < optlen_aligned / 2; i+= 1) {
      //   chk_sum += ((u16 *)p.payload)[i];
      // }

    }

    /* generate IP header */
    if (pbuf_add_header(p, IP_HLEN)) {
//      LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output: not enough room for IP header in pbuf\n"));

      IP_STATS_INC(ip.err);
      MIB2_STATS_INC(mib2.ipoutdiscards);
      return ERR_BUF;
    }

    iphdr = p.payload;
    // LWIP_ASSERT("check that first pbuf can hold struct ip_hdr",
    //             (p.len >= sizeof(struct ip_hdr)));

    IPH_TTL_SET(iphdr, ttl);
    IPH_PROTO_SET(iphdr, proto);

    chk_sum += PP_NTOHS(proto | (ttl << 8));


    /* dest cannot be NULL here */
    ip4_addr_copy(iphdr.dest, *dest);

    chk_sum += ip4_addr_get_u32(&iphdr.dest) & 0xFFFF;
    chk_sum += ip4_addr_get_u32(&iphdr.dest) >> 16;


    IPH_VHL_SET(iphdr, 4, ip_hlen / 4);
    IPH_TOS_SET(iphdr, tos);

    chk_sum += PP_NTOHS(tos | (iphdr._v_hl << 8));

    IPH_LEN_SET(iphdr, lwip_htons(p.tot_len));

    chk_sum += iphdr._len;

    IPH_OFFSET_SET(iphdr, 0);
    IPH_ID_SET(iphdr, lwip_htons(ip_id));

    chk_sum += iphdr._id;

    ip_id += 1;

    if (src == NULL) {
      ip4_addr_copy(iphdr.src, *IP4_ADDR_ANY4);
    } else {
      /* src cannot be NULL here */
      ip4_addr_copy(iphdr.src, *src);
    }


    chk_sum += ip4_addr_get_u32(&iphdr.src) & 0xFFFF;
    chk_sum += ip4_addr_get_u32(&iphdr.src) >> 16;
    chk_sum = (chk_sum >> 16) + (chk_sum & 0xFFFF);
    chk_sum = (chk_sum >> 16) + chk_sum;
    chk_sum = !chk_sum;
    // IF__NETIF_CHECKSUM_ENABLED(netif, NETIF_CHECKSUM_GEN_IP) {
      if netif::CHECKSUM_ENABLED(NETIF_CHECKSUM_GEN_IP) {
      iphdr._chksum = chk_sum; /* network order */
    }

    else {
      IPH_CHKSUM_SET(iphdr, 0);
    }

 /* CHECKSUM_GEN_IP_INLINE */
    IPH_CHKSUM_SET(iphdr, 0);

    // IF__NETIF_CHECKSUM_ENABLED(netif, NETIF_CHECKSUM_GEN_IP) {
      if netif::CHECKSUM_ENABLED(NETIF_CHECKSUM_GEN_IP) {
      IPH_CHKSUM_SET(iphdr, inet_chksum(iphdr, ip_hlen));
    }


  } else {
    /* IP header already included in p */
    if (p.len < IP_HLEN) {
//      LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output: LWIP_IP_HDRINCL but pbuf is too short\n"));
      IP_STATS_INC(ip.err);
      MIB2_STATS_INC(mib2.ipoutdiscards);
      return ERR_BUF;
    }
    iphdr = p.payload;
    ip4_addr_copy(dest_addr, iphdr.dest);
    dest = &dest_addr;
  }

  IP_STATS_INC(ip.xmit);

//  LWIP_DEBUGF(IP_DEBUG, ("ip4_output_if: %c%c%"U16_F"\n", netif.name[0], netif.name[1], netif.num));
  ip4_debug_print(p);


  if (ip4_addr_cmp(dest, netif_ip4_addr(netif))

      || ip4_addr_isloopback(dest)

     ) {
    /* Packet to self, enqueue it for loopback */
//    LWIP_DEBUGF(IP_DEBUG, ("netif_loop_output()"));
    return netif_loop_output(netif, p);
  }

  if ((p.flags & PBUF_FLAG_MCASTLOOP) != 0) {
    netif_loop_output(netif, p);
  }



  /* don't fragment if interface has mtu set to 0 [loopif] */
  if (netif.mtu && (p.tot_len > netif.mtu)) {
    return ip4_frag(p, netif, dest);
  }


//  LWIP_DEBUGF(IP_DEBUG, ("ip4_output_if: call netif.output()\n"));
  return netif.output(netif, p, dest);
}

/*
 * Simple interface to ip_output_if. It finds the outgoing network
 * interface and calls upon ip_output_if to do the actual work.
 *
 * @param p the packet to send (p.payload points to the data, e.g. next
            protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
            IP header and p.payload points to that IP header)
 * @param src the source IP address to send from (if src == IP4_ADDR_ANY, the
 *         IP  address of the netif used to send is used as source address)
 * @param dest the destination IP address to send the packet to
 * @param ttl the TTL value to be set in the IP header
 * @param tos the TOS value to be set in the IP header
 * @param proto the PROTOCOL to be set in the IP header
 *
 * @return ERR_RTE if no route is found
 *         see ip_output_if() for more return values
 */
pub fn 
ip4_output(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
           ttl: u8, tos: u8, proto: u8)
{
  let netif: &mut NetIfc;

  LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);

  if ((netif = ip4_route_src(src, dest)) == NULL) {
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_output: No route to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                           ip4_addr1_16(dest), ip4_addr2_16(dest), ip4_addr3_16(dest), ip4_addr4_16(dest)));*/
    IP_STATS_INC(ip.rterr);
    return ERR_RTE;
  }

  return ip4_output_if(p, src, dest, ttl, tos, proto, netif);
}


/* Like ip_output, but takes and addr_hpointer: i32 that is passed on to netif.addr_hint
 *  before calling ip_output_if.
 *
 * @param p the packet to send (p.payload points to the data, e.g. next
            protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
            IP header and p.payload points to that IP header)
 * @param src the source IP address to send from (if src == IP4_ADDR_ANY, the
 *         IP  address of the netif used to send is used as source address)
 * @param dest the destination IP address to send the packet to
 * @param ttl the TTL value to be set in the IP header
 * @param tos the TOS value to be set in the IP header
 * @param proto the PROTOCOL to be set in the IP header
 * @param netif_hnetif: i32 output hpointer: i32 set to netif.hbefore: i32
 *        calling ip_output_if()
 *
 * @return ERR_RTE if no route is found
 *         see ip_output_if() for more return values
 */
pub fn 
ip4_output_hinted(p: &mut pbuf,  src: &mut ip4_addr,  dest: &mut ip4_addr,
                  ttl: u8, tos: u8, proto: u8, netif_hint: &mut netif_hint)
{
  let netif: &mut NetIfc;
  let err: err_t;

  LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);

  if ((netif = ip4_route_src(src, dest)) == NULL) {
/*LWIP_DEBUGF(IP_DEBUG, ("ip4_output: No route to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                           ip4_addr1_16(dest), ip4_addr2_16(dest), ip4_addr3_16(dest), ip4_addr4_16(dest)));*/
    IP_STATS_INC(ip.rterr);
    return ERR_RTE;
  }

  NETIF_SET_HINTS(netif, netif_hint);
  err = ip4_output_if(p, src, dest, ttl, tos, proto, netif);
  NETIF_RESET_HINTS(netif);

  return err;
}



/* Pran: i32 IP header by using LWIP_DEBUGF
 * @param p an IP packet, p.payload pointing to the IP header
 */
pub fn 
ip4_debug_print(p: &mut pbuf)
{
  let iphdr: &mut ip_hdr = p.payload;

//  LWIP_DEBUGF(IP_DEBUG, ("IP header:\n"));
//  LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
/*LWIP_DEBUGF(IP_DEBUG, ("|%2"S16_F" |%2"S16_F" |  0x%02"X16_F" |     %5"U16_F"     | (v, hl, tos, len)\n",
                         IPH_V(iphdr),
                         IPH_HL(iphdr),
                         IPH_TOS(iphdr),
                         lwip_ntohs(IPH_LEN(iphdr))));*/
//  LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
/*LWIP_DEBUGF(IP_DEBUG, ("|    %5"U16_F"      |%"U16_F"%"U16_F"%"U16_F"|    %4"U16_F"   | (id, flags, offset)\n",
                         lwip_ntohs(IPH_ID(iphdr)),
                         (lwip_ntohs(IPH_OFFSET(iphdr)) >> 15 & 1),
                         (lwip_ntohs(IPH_OFFSET(iphdr)) >> 14 & 1),
                         (lwip_ntohs(IPH_OFFSET(iphdr)) >> 13 & 1),
                         (lwip_ntohs(IPH_OFFSET(iphdr)) & IP_OFFMASK)));*/
//  LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
/*LWIP_DEBUGF(IP_DEBUG, ("|  %3"U16_F"  |  %3"U16_F"  |    0x%04"X16_F"     | (ttl, proto, chksum)\n",
                         IPH_TTL(iphdr),
                         IPH_PROTO(iphdr),
                         lwip_ntohs(IPH_CHKSUM(iphdr))));*/
//  LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
/*LWIP_DEBUGF(IP_DEBUG, ("|  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  | (src)\n",
                         ip4_addr1_16_val(iphdr.src),
                         ip4_addr2_16_val(iphdr.src),
                         ip4_addr3_16_val(iphdr.src),
                         ip4_addr4_16_val(iphdr.src)));*/
//  LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
/*LWIP_DEBUGF(IP_DEBUG, ("|  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  | (dest)\n",
                         ip4_addr1_16_val(iphdr.dest),
                         ip4_addr2_16_val(iphdr.dest),
                         ip4_addr3_16_val(iphdr.dest),
                         ip4_addr4_16_val(iphdr.dest)));*/
//  LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
}



