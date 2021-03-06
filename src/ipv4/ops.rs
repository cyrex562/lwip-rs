use crate::core::context::LwipContext;
use crate::core::errors::LwipErrorCode;
use crate::core::packet_buffer::PacketBuffer;
use crate::ip::proto::IpProto;
use crate::ipv4::addr::{ip4_addr_is_link_local, Ipv4Address};
use crate::ipv4::hdr::Ipv4Header;
use crate::LwipError;
use crate::netif::netif::NetworkInterface;

pub fn ip4_process_rx(pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    let ip4_hdr = Ipv4Header::from(pkt);
    let ip_proto = IpProto::from(ip4_hdr.proto);
    // TODO: handle ip options if present
    match ip_proto {
        IpProto::HopOpt => {
            return Err(LwipError::new(LwipErrorCode::InvalidData, "Ipv6 HOP OPT not supported for IPv4"));
        }
        IpProto::ICMP => {
            todo!()
        }
        IpProto::IGMP => {
            todo!()
        }
        IpProto::GGP => {
            unimplemented!()
        }
        IpProto::IPv4 => {
            unimplemented!()
        }
        IpProto::ST => {
            unimplemented!()
        }
        IpProto::TCP => {
            todo!()
        }
        IpProto::CBT => {
            unimplemented!()
        }
        IpProto::EGP => {
            unimplemented!()
        }
        IpProto::IGP => {
            unimplemented!()
        }
        IpProto::BBN_RCC_MON => {
            unimplemented!()
        }
        IpProto::NVP_II => {
            unimplemented!()
        }
        IpProto::PUP => {
            unimplemented!()
        }
        IpProto::EMCON => {
            unimplemented!()
        }
        IpProto::XNET => {
            unimplemented!()
        }
        IpProto::CHAOS => {
            unimplemented!()
        }
        IpProto::UDP => {
            todo!()
        }
        IpProto::MUX => {
            unimplemented!()
        }
        IpProto::DCN_MEAS => {
            unimplemented!()
        }
        IpProto::HMP => {
            unimplemented!()
        }
        IpProto::PRM => {
            unimplemented!()
        }
        IpProto::XNS_IDP => {
            unimplemented!()
        }
        IpProto::Trunk1 => {
            unimplemented!()
        }
        IpProto::Trunk2 => {
            unimplemented!()
        }
        IpProto::Leaf1 => {
            unimplemented!()
        }
        IpProto::Leaf2 => {
            unimplemented!()
        }
        IpProto::RDP => {
            unimplemented!()
        }
        IpProto::IRTP => {
            unimplemented!()
        }
        IpProto::ISO_TP4 => {
            unimplemented!()
        }
        IpProto::NETBLT => {
            unimplemented!()
        }
        IpProto::MFE_NSP => {
            unimplemented!()
        }
        IpProto::MERIT_INP => {
            unimplemented!()
        }
        IpProto::DCCP => {
            unimplemented!()
        }
        IpProto::IP_3PC => {
            unimplemented!()
        }
        IpProto::IDPR => {
            unimplemented!()
        }
        IpProto::XTP => {
            unimplemented!()
        }
        IpProto::DDP => {
            unimplemented!()
        }
        IpProto::IDPR_CMTP => {
            unimplemented!()
        }
        IpProto::TP_PP => {
            unimplemented!()
        }
        IpProto::IL => {
            unimplemented!()
        }
        IpProto::IPv6 => {
            unimplemented!()
        }
        IpProto::SDRP => {
            unimplemented!()
        }
        IpProto::IPV6_Route => {
            return Err(LwipError::new(LwipErrorCode::InvalidData, "IPv6 Route Opt not supported"));
        }
        IpProto::IPV6_Frag => {
            return Err(LwipError::new(LwipErrorCode::InvalidData, "IPv6 Frag Opt not supported"));
        }
        IpProto::IDRP => {
            unimplemented!()
        }
        IpProto::RSVP => {
            unimplemented!()
        }
        IpProto::GRE => {
            unimplemented!()
        }
        IpProto::DSR => {
            unimplemented!()
        }
        IpProto::BNA => {
            unimplemented!()
        }
        IpProto::ESP => {
            unimplemented!()
        }
        IpProto::AH => {
            unimplemented!()
        }
        IpProto::I_NLSP => {
            unimplemented!()
        }
        IpProto::NARP => {
            unimplemented!()
        }
        IpProto::MOBILE => {
            unimplemented!()
        }
        IpProto::TLSP => {
            unimplemented!()
        }
        IpProto::SKIP => {
            unimplemented!()
        }
        IpProto::IPv6_ICMP => {
            return Err(LwipError::new(LwipErrorCode::InvalidData, "IPv6 ICMP Opt not supported"));
        }
        IpProto::IPv6_NoNxt => {
            return Err(LwipError::new(LwipErrorCode::InvalidData, "IPv6 No-Next Opt not supported"));
        }
        IpProto::IPv6_Opts => {
            return Err(LwipError::new(LwipErrorCode::InvalidData, "IPv6 Options Opt not supported"));
        }
        IpProto::AnyHostInternal => {
            unimplemented!()
        }
        IpProto::CFTP => {
            unimplemented!()
        }
        IpProto::AnyLocalNet => {
            unimplemented!()
        }
        IpProto::SAT_EXPAK => {
            unimplemented!()
        }
        IpProto::KRYPTOLAN => {
            unimplemented!()
        }
        IpProto::RVD => {
            unimplemented!()
        }
        IpProto::IPPC => {
            unimplemented!()
        }
        IpProto::AnyDistFS => {
            unimplemented!()
        }
        IpProto::SAT_MON => {
            unimplemented!()
        }
        IpProto::VISA => unimplemented!(),
        IpProto::IPCV =>unimplemented!(),
        IpProto::CPNX => unimplemented!(),
        IpProto::CPHB => unimplemented!(),
        IpProto::WSN => unimplemented!(),
        IpProto::PVP => unimplemented!(),
        IpProto::BR_SAT_MON => unimplemented!(),
        IpProto::SUN_ND => unimplemented!(),
        IpProto::WB_MON => unimplemented!(),
        IpProto::WB_EXPAK => unimplemented!(),
        IpProto::ISO_IP => unimplemented!(),
        IpProto::VMTP => unimplemented!(),
        IpProto::SECURE_VMTP => unimplemented!(),
        IpProto::VINES => unimplemented!(),
        IpProto::TTP_IPTM => unimplemented!(),
        IpProto::NSFNET_IGP => unimplemented!(),
        IpProto::DGP => unimplemented!(),
        IpProto::TCF => unimplemented!(),
        IpProto::EIGRP => unimplemented!(),
        IpProto::OSPF_IGP => unimplemented!(),
        IpProto::SpriteRPC => unimplemented!(),
        IpProto::LARP => unimplemented!(),
        IpProto::MTP => unimplemented!(),
        IpProto::AX_25 => unimplemented!(),
        IpProto::IPIP => unimplemented!(),
        IpProto::SCC_SP => unimplemented!(),
        IpProto::ETHER_IP => unimplemented!(),
        IpProto::ENCAP => unimplemented!(),
        IpProto::AnyPrivEncrypt => unimplemented!(),
        IpProto::GMTP => unimplemented!(),
        IpProto::IFMP => unimplemented!(),
        IpProto::PNNI => unimplemented!(),
        IpProto::PIM => unimplemented!(),
        IpProto::ARIS => unimplemented!(),
        IpProto::SCPS => unimplemented!(),
        IpProto::QNX => unimplemented!(),
        IpProto::A_N => unimplemented!(),
        IpProto::IPComp => unimplemented!(),
        IpProto::SNP => unimplemented!(),
        IpProto::CompaqPeer => unimplemented!(),
        IpProto::IPXinIP => unimplemented!(),
        IpProto::VRRP => unimplemented!(),
        IpProto::PGM => unimplemented!(),
        IpProto::Any0Hop => unimplemented!(),
        IpProto::L2TP => unimplemented!(),
        IpProto::DDX => unimplemented!(),
        IpProto::IATP => unimplemented!(),
        IpProto::STP => unimplemented!(),
        IpProto::SRP => unimplemented!(),
        IpProto::UTI => unimplemented!(),
        IpProto::SMP => unimplemented!(),
        IpProto::PTP => unimplemented!(),
        IpProto::ISIS_IPV4 => unimplemented!(),
        IpProto::FIRE => unimplemented!(),
        IpProto::CRTP => unimplemented!(),
        IpProto::CRUDP => unimplemented!(),
        IpProto::SSCOPMCE => unimplemented!(),
        IpProto::IPLT => unimplemented!(),
        IpProto::SPS => unimplemented!(),
        IpProto::PIPE => unimplemented!(),
        IpProto::SCTP => unimplemented!(),
        IpProto::FC => unimplemented!(),
        IpProto::RSVP_E2E_IGNORE => unimplemented!(),
        IpProto::MobilityHeader => unimplemented!(),
        IpProto::UDPLite => unimplemented!(),
        IpProto::MPLS_IP => unimplemented!(),
        IpProto::MANET => unimplemented!(),
        IpProto::HIP => unimplemented!(),
        IpProto::Shim6 => unimplemented!(),
        IpProto::WESP => unimplemented!(),
        IpProto::ROHC => unimplemented!(),
        IpProto::Ethernet => unimplemented!(),
        IpProto::UnassignedStart => unimplemented!(),
        IpProto::UnassignedEnd => unimplemented!(),
        IpProto::Experimental1 => unimplemented!(),
        IpProto::Experimental2 => unimplemented!(),
        IpProto::Reserved => unimplemented!(),
    }
    Ok(())
}

/// Check if forwarding is allowed; check if the packet is allowed to be forwarded (ACL); attempt to find a route for the packet; forwarwd the packet based on its properties and available routes
pub fn ip4_can_forward(ctx: &mut LwipContext,
                       src_netif: &mut NetworkInterface,
                       pkt: &mut PacketBuffer) -> Result<bool, LwipError> {
    todo!()
}

/**
 * Forwards an IP packet. It finds an appropriate route for the
 * packet, decrements the TTL value of the packet, adjusts the
 * checksum and outputs the packet on the appropriate interface.
 *
 * @param p the packet to forward ( p.payload points to IP header)
 * @param iphdr the IP header of the input packet
 * @param inp the netif on which this packet was received
 */
pub fn ip4_forward(ctx: &mut LwipContext,
                   netif: &mut NetworkInterface,
                   pkt: &mut PacketBuffer) -> Result<(), LwipError> {
//     let mut hdr = Ipv4Header::from(pkt);
//     let dst_ip4_addr = Ipv4Address::from(hdr.dst_addr);
//     let src_ip4_addr = Ipv4Address::from(hdr.src_addr);
//     match ip4_can_forward(ctx, netif, pkt) {
//         Ok(can_forward) => {
//             if can_forward == false {
//                 return Err(LwipError::new(LwipErrorCode::InvalidOperation, "cant forward packet"));
//             }
//         }
//         Err(e) => {
//             return Err(LwipError::new(LwipErrorCode::OperationFailed, "ip4 can forward check op failed {}".format(e)));
//         }
//     }
//
//     // RFC3927 2.7: do not forward link-local addresses
//     if ip4_addr_is_link_local(&dst_ip4_addr) {
//         return Err(LwipError::new(LwipErrorCode::InvalidOperation, "cant forward link local address: {}".format(&hdr)));
//     }
//
//     let dest_netif = match ip4_route_src(ctx, &src_ip4_addr, &dst_ip4_addr, netif) {
//         Ok(x) => x,
//         Err(e) => {
//             return Err(LwipError::new(LwipErrorCode::OperationFailed, "could not get destination netif for packet: {}, {}".format(e, &hdr)));
//         }
//     };
//
//     if (dest_netif == src_netif) && ctx.options.ip_forward_allow_tx_on_rx_netif {
//         return Err(LwipError::new(LwipErrorCode::InvalidOperation, "cant forward packet with destination interface that is the same as its source interface"));
//     }
//
//     hdr.ttl = hdr.ttl - 1;
//     if hdr.ttl == 0 {
//         if hdr.proto != IP_PROTO_ICMP {
//             icmp_ttl_exceeded(p, ICMP_TE_TTL)?;
//         } else {
//             return Err(LwipError::new(LwipErrorCode::InvalidOperation, "send ICMP time exceeded msg in response to an ICMP message"));
//         }
//     }
//     pkt.update_header(pkt, hdr)?;
//
//     /* Incrementally update the IP checksum. */
//     if (IPH_CHKSUM(iphdr) >= PP_HTONS(0xffff - 0x100)) {
//         IPH_CHKSUM_SET(iphdr, (u16_t)(IPH_CHKSUM(iphdr) + PP_HTONS(0x100) + 1));
//     } else {
//         IPH_CHKSUM_SET(iphdr, (u16_t)(IPH_CHKSUM(iphdr) + PP_HTONS(0x100)));
//     }
//
//     /* Take care of setting checksums to 0 for checksum offload netifs */
//     if (CHECKSUM_GEN_IP || NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_GEN_IP)) {
//         IPH_CHKSUM_SET(iphdr, 0);
//     }
//     switch(IPH_PROTO(iphdr))
//     {
// // #if LWIP_UDP
// // #if LWIP_UDPLITE
//         case
//         IP_PROTO_UDPLITE:
// // #endif
//             case
//         IP_PROTO_UDP: if (CHECKSUM_GEN_UDP || NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_GEN_UDP)) {
//         ((struct udp_hdr
//         *)((u8_t *)
//         iphdr + IPH_HL_BYTES(iphdr))) -> chksum = 0;
//     }
//         break;
// // #endif
// // #if LWIP_TCP
//         case
//         IP_PROTO_TCP: if (CHECKSUM_GEN_TCP || NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_GEN_TCP)) {
//         ((struct tcp_hdr
//         *)((u8_t *)
//         iphdr + IPH_HL_BYTES(iphdr))) -> chksum = 0;
//     }
//         break;
// // #endif
// // #if LWIP_ICMP
//         case
//         IP_PROTO_ICMP: if (CHECKSUM_GEN_ICMP || NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_GEN_ICMP)) {
//         ((struct icmp_hdr
//         *)((u8_t *)
//         iphdr + IPH_HL_BYTES(iphdr))) -> chksum = 0;
//     }
//         break;
// // #endif
//         default: /* there's really nothing to do here other than satisfying 'switch-default' */
//         break;
//     }
//
//     LWIP_DEBUGF(IP_DEBUG, ("ip4_forward: forwarding packet to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
//                 ip4_addr1_16(ip4_current_dest_addr()), ip4_addr2_16(ip4_current_dest_addr()),
//                 ip4_addr3_16(ip4_current_dest_addr()), ip4_addr4_16(ip4_current_dest_addr())));
//
//     IP_STATS_INC(ip.fw);
//     MIB2_STATS_INC(mib2.ipforwdatagrams);
//     IP_STATS_INC(ip.xmit);
//
//     PERF_STOP("ip4_forward");
//     /* don't fragment if interface has mtu set to 0 [loopif] */
//     if (netif.mtu && (p.tot_len > netif.mtu)) {
//         if ((IPH_OFFSET(iphdr) & PP_NTOHS(IP_DF)) == 0) {
// // #if IP_FRAG
//             ip4_frag(p, netif, ip4_current_dest_addr()); # else /* IP_FRAG */
//             /* @todo: send ICMP Destination Unreachable code 13 "Communication administratively prohibited"? */
// // #endif /* IP_FRAG */
//         } else {
// // #if LWIP_ICMP
//             /* send ICMP Destination Unreachable code 4: "Fragmentation Needed and DF Set" */
//             icmp_dest_unreach(p, ICMP_DUR_FRAG);
// // #endif /* LWIP_ICMP */
//         }
//         return;
//     }
//     /* transmit pbuf on chosen interface */
//     netif.output(netif, p, ip4_current_dest_addr());
//     return;
//     return_noroute: MIB2_STATS_INC(mib2.ipoutnoroutes);
    todo!()
}
// #endif /* IP_FORWARD */

// /** Return true if the current input packet should be accepted on this netif */ static int
pub fn ip4_input_accept(netif: &mut NetworkInterface)
{
// LWIP_DEBUGF(IP_DEBUG, ("ip_input:  iphdr.dest 0x%"X32_F"  netif.ip_addr 0x%"X32_F" (0x%"X32_F", 0x%"X32_F", 0x%"X32_F")\n",
// ip4_addr_get_u32(ip4_current_dest_addr()), ip4_addr_get_u32(netif_ip4_addr(netif)),
// ip4_addr_get_u32(ip4_current_dest_addr()) & ip4_addr_get_u32(netif_ip4_netmask(netif)),
// ip4_addr_get_u32(netif_ip4_addr(netif)) & ip4_addr_get_u32(netif_ip4_netmask(netif)),
// ip4_addr_get_u32(ip4_current_dest_addr()) & ~ip4_addr_get_u32(netif_ip4_netmask(netif))));
//
// /* interface is up and configured? */ if ((netif_is_up(netif)) & & ( ! ip4_addr_isany_val(* netif_ip4_addr(netif)))) {
// /* unicast to this interface address? */ if (ip4_addr_eq(ip4_current_dest_addr(), netif_ip4_addr(netif)) | | /* or broadcast on this interface network address? */
// ip4_addr_isbroadcast(ip4_current_dest_addr(), netif)
// // #if LWIP_NETIF_LOOPBACK && !LWIP_HAVE_LOOPIF | | (ip4_addr_get_u32(ip4_current_dest_addr()) == PP_HTONL(IPADDR_LOOPBACK))
// // #endif /* LWIP_NETIF_LOOPBACK && !LWIP_HAVE_LOOPIF */
// ) {
// LWIP_DEBUGF(IP_DEBUG, ("ip4_input: packet accepted on interface %c%c\n",
// netif.name[0], netif.name[1])); /* accept on this netif */ return 1;
// }
// // #if LWIP_AUTOIP
// /* connections to link-local addresses must persist after changing
//     the netif's address (RFC3927 ch. 1.9) */ if (autoip_accept_packet(netif, ip4_current_dest_addr())) {
// LWIP_DEBUGF(IP_DEBUG, ("ip4_input: LLA packet accepted on interface %c%c\n",
// netif.name[0], netif.name[1])); /* accept on this netif */ return 1;
// }
// // #endif /* LWIP_AUTOIP */
// }
// return 0;
    todo!()
}

// /**
//  * This function is called by the network interface device driver when
//  * an IP packet is received. The function does the basic checks of the
//  * IP header such as packet size being at least larger than the header
//  * size etc. If the packet was not destined for us, the packet is
//  * forwarded (using ip_forward). The IP checksum is always checked.
//  *
//  * Finally, the packet is sent to the upper layer protocol input function.
//  *
//  * @param p the received IP packet ( p.payload points to IP header)
//  * @param inp the netif on which this packet was received
//  * @return ERR_OK if the packet was processed (could return ERR_* if it wasn't
//  *         processed, but currently always returns ERR_OK)
//  */
/// Called by the network interface when an IPv4 packet is received. checks the IP  header. If packet not destined for the host, then the forwarding function is called. If the pacekt is for the host, then an upper-layer recv function is called.
pub fn ip4_recv(ctx: &mut LwipContext, netif: &mut NetworkInterface, pkt: &mut PacketBuffer) -> Result<(), LwipError> {
//     const struct ip_hdr
//     *iphdr;
//     struct netif
//     *netif;
//     iphdr_hlen: u16;
//     iphdr_len: u16;
// // #if IP_ACCEPT_LINK_LAYER_ADDRESSING || LWIP_IGMP
//     int
//     check_ip_src = 1;
// // #endif /* IP_ACCEPT_LINK_LAYER_ADDRESSING || LWIP_IGMP */
// // #if LWIP_RAW
//     raw_input_state_t
//     raw_status;
// // #endif /* LWIP_RAW */
//
//     // LWIP_ASSERT_CORE_LOCKED()
//
//     IP_STATS_INC(ip.recv);
//     MIB2_STATS_INC(mib2.ipinreceives);
//
//     /* identify the IP header */
//     iphdr = (struct ip_hdr
//     *) p.payload;
//     if (IPH_V(iphdr) != 4) {
//         LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_WARNING, ("IP packet dropped due to bad version number %"U16_F"\n", (u16_t)IPH_V(iphdr)));
//         ip4_debug_print(p);
//         pbuf_free(p);
//         IP_STATS_INC(ip.err);
//         IP_STATS_INC(ip.drop);
//         MIB2_STATS_INC(mib2.ipinhdrerrors);
//         return ERR_OK;
//     }
//
//     # ifdef
//     LWIP_HOOK_IP4_INPUT
//     if (LWIP_HOOK_IP4_INPUT(p, inp)) {
//         /* the packet has been eaten */
//         return ERR_OK;
//     }
// // #endif
//
//     /* obtain IP header length in bytes */
//     iphdr_hlen = IPH_HL_BYTES(iphdr);
//     /* obtain ip length in bytes */
//     iphdr_len = lwip_ntohs(IPH_LEN(iphdr));
//
//     /* Trim pbuf. This is especially required for packets < 60 bytes. */
//     if (iphdr_len < p.tot_len) {
//         pbuf_realloc(p, iphdr_len);
//     }
//
//     /* header length exceeds first pbuf length, or ip length exceeds total pbuf length? */
//     if ((iphdr_hlen > p.len) || (iphdr_len > p.tot_len) || (iphdr_hlen < IP_HLEN)) {
//         if (iphdr_hlen < IP_HLEN) {
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
//                         ("ip4_input: short IP header (%"U16_F" bytes) received, IP packet dropped\n", iphdr_hlen));
//         }
//         if (iphdr_hlen > p.len) {
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
//                         ("IP header (len %"U16_F") does not fit in first pbuf (len %"U16_F"), IP packet dropped.\n",
//                         iphdr_hlen, p.len));
//         }
//         if (iphdr_len > p.tot_len) {
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
//                         ("IP (len %"U16_F") is longer than pbuf (len %"U16_F"), IP packet dropped.\n",
//                         iphdr_len, p.tot_len));
//         }
//         /* free (drop) packet pbufs */
//         pbuf_free(p);
//         IP_STATS_INC(ip.lenerr);
//         IP_STATS_INC(ip.drop);
//         MIB2_STATS_INC(mib2.ipindiscards);
//         return ERR_OK;
//     }
//
//     /* verify checksum */
// // #if CHECKSUM_CHECK_IP
//     IF__NETIF_CHECKSUM_ENABLED(inp, NETIF_CHECKSUM_CHECK_IP)
//     {
//         if (inet_chksum(iphdr, iphdr_hlen) != 0) {
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
//                         ("Checksum (0x%"X16_F") failed, IP packet dropped.\n", inet_chksum(iphdr, iphdr_hlen)));
//             ip4_debug_print(p);
//             pbuf_free(p);
//             IP_STATS_INC(ip.chkerr);
//             IP_STATS_INC(ip.drop);
//             MIB2_STATS_INC(mib2.ipinhdrerrors);
//             return ERR_OK;
//         }
//     }
// // #endif
//
//     /* copy IP addresses to aligned ip_addr_t */
//     ip_addr_copy_from_ip4(ip_data.current_iphdr_dest, iphdr.dest);
//     ip_addr_copy_from_ip4(ip_data.current_iphdr_src, iphdr.src);
//
//     /* match packet against an interface, i.e. is this packet for us? */
//     if (ip4_addr_ismulticast(ip4_current_dest_addr())) {
// // #if LWIP_IGMP
//         if ((inp.flags & NETIF_FLAG_IGMP) && (igmp_lookfor_group(inp, ip4_current_dest_addr()))) {
//             /* IGMP snooping switches need 0.0.0.0 to be allowed as source address (RFC 4541) */
//             allsystems: ip4_addr_t;
//             IP4_ADDR(&allsystems, 224, 0, 0, 1);
//             if (ip4_addr_eq(ip4_current_dest_addr(), &allsystems) && ip4_addr_isany(ip4_current_src_addr())) {
//                 check_ip_src = 0;
//             }
//             netif = inp;
//         } else {
//             netif = NULL;
//         } # else /* LWIP_IGMP */
//         if ((netif_is_up(inp)) && (!ip4_addr_isany_val(*netif_ip4_addr(inp)))) {
//             netif = inp;
//         } else {
//             netif = NULL;
//         }
// // #endif /* LWIP_IGMP */
//     } else {
//         /* start trying with inp. if that's not acceptable, start walking the
//            list of configured netifs. */
//         if (ip4_input_accept(inp)) {
//             netif = inp;
//         } else {
//             netif = NULL; # if !LWIP_NETIF_LOOPBACK || LWIP_HAVE_LOOPIF
//             /* Packets sent to the loopback address must not be accepted on an
//              * interface that does not have the loopback address assigned to it,
//              * unless a non-loopback interface is used for loopback traffic. */
//             if (!ip4_addr_isloopback(ip4_current_dest_addr()))
// // #endif /* !LWIP_NETIF_LOOPBACK || LWIP_HAVE_LOOPIF */ {
//                 # if !LWIP_SINGLE_NETIF
//                 NETIF_FOREACH(netif)
//                 {
//                     if (netif == inp) {
//                         /* we checked that before already */
//                         continue;
//                     }
//                     if (ip4_input_accept(netif)) {
//                         break;
//                     }
//                 }
// // #endif /* !LWIP_SINGLE_NETIF */
//             }
//         }
//     }
//
// // #if IP_ACCEPT_LINK_LAYER_ADDRESSING
//     /* Pass DHCP messages regardless of destination address. DHCP traffic is addressed
//      * using link layer addressing (such as Ethernet MAC) so we must not filter on IP.
//      * According to RFC 1542 section 3.1.1, referred by RFC 2131).
//      *
//      * If you want to accept private broadcast communication while a netif is down,
//      * define LWIP_IP_ACCEPT_UDP_PORT(dst_port), e.g.:
//      *
//      * #define LWIP_IP_ACCEPT_UDP_PORT(dst_port) ((dst_port) == PP_NTOHS(12345))
//      */
//     if (netif == NULL) {
//         /* remote port is DHCP server? */
//         if (IPH_PROTO(iphdr) == IP_PROTO_UDP) {
//             const struct udp_hdr
//             *udphdr = ( const struct udp_hdr
//             *)((const u8_t
//             *)iphdr + iphdr_hlen);
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE, ("ip4_input: UDP packet to DHCP client port %"U16_F"\n",
//                         lwip_ntohs(udphdr.dest)));
//             if (IP_ACCEPT_LINK_LAYER_ADDRESSED_PORT(udphdr.dest)) {
//                 LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE, ("ip4_input: DHCP packet accepted.\n"));
//                 netif = inp;
//                 check_ip_src = 0;
//             }
//         }
//     }
// // #endif /* IP_ACCEPT_LINK_LAYER_ADDRESSING */
//
//     /* broadcast or multicast packet source address? Compliant with RFC 1122: 3.2.1.3 */
// // #if LWIP_IGMP || IP_ACCEPT_LINK_LAYER_ADDRESSING
//     if (check_ip_src
// // #if IP_ACCEPT_LINK_LAYER_ADDRESSING
//         /* DHCP servers need 0.0.0.0 to be allowed as source address (RFC 1.1.2.2: 3.2.1.3/a) */ && !ip4_addr_isany_val(*ip4_current_src_addr())
// // #endif /* IP_ACCEPT_LINK_LAYER_ADDRESSING */
//     )
// // #endif /* LWIP_IGMP || IP_ACCEPT_LINK_LAYER_ADDRESSING */ {
//         if ((ip4_addr_isbroadcast(ip4_current_src_addr(), inp)) || (ip4_addr_ismulticast(ip4_current_src_addr()))) {
//             /* packet source is not valid */
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_LEVEL_WARNING, ("ip4_input: packet source is not valid.\n"));
//             /* free (drop) packet pbufs */
//             pbuf_free(p);
//             IP_STATS_INC(ip.drop);
//             MIB2_STATS_INC(mib2.ipinaddrerrors);
//             MIB2_STATS_INC(mib2.ipindiscards);
//             return ERR_OK;
//         }
//     }
//
//     /* packet not for us? */
//     if (netif == NULL) {
//         /* packet not for us, route or discard */
//         LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_TRACE, ("ip4_input: packet not for us.\n"));
// // #if IP_FORWARD
//         /* non-broadcast packet? */
//         if (!ip4_addr_isbroadcast(ip4_current_dest_addr(), inp)) {
//             /* try to forward IP packet on (other) interfaces */
//             ip4_forward(p, (struct ip_hdr
//             *) p.payload, inp);
//         } else
// // #endif /* IP_FORWARD */ {
//             IP_STATS_INC(ip.drop);
//             MIB2_STATS_INC(mib2.ipinaddrerrors);
//             MIB2_STATS_INC(mib2.ipindiscards);
//         }
//         pbuf_free(p);
//         return ERR_OK;
//     }
//     /* packet consists of multiple fragments? */
//     if ((IPH_OFFSET(iphdr) & PP_HTONS(IP_OFFMASK | IP_MF)) != 0) {
// // #if IP_REASSEMBLY /* packet fragment reassembly code present? */
//         LWIP_DEBUGF(IP_DEBUG, ("IP packet is a fragment (id=0x%04"X16_F" tot_len=%"U16_F" len=%"U16_F" MF=%"U16_F" offset=%"U16_F"), calling ip4_reass()\n",
//                     lwip_ntohs(IPH_ID(iphdr)), p.tot_len, lwip_ntohs(IPH_LEN(iphdr)), (u16_t)!!(IPH_OFFSET(iphdr) & PP_HTONS(IP_MF)), (u16_t)((lwip_ntohs(IPH_OFFSET(iphdr)) & IP_OFFMASK) * 8)));
//         /* reassemble the packet*/
//         p = ip4_reass(p);
//         /* packet not fully reassembled yet? */
//         if (p == NULL) {
//             return ERR_OK;
//         }
//         iphdr = ( const struct ip_hdr
//         *) p.payload; # else /* IP_REASSEMBLY == 0, no packet fragment reassembly code present */
//         pbuf_free(p);
//         LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("IP packet dropped since it was fragmented (0x%"X16_F") (while IP_REASSEMBLY == 0).\n",
//                     lwip_ntohs(IPH_OFFSET(iphdr))));
//         IP_STATS_INC(ip.opterr);
//         IP_STATS_INC(ip.drop);
//         /* unsupported protocol feature */
//         MIB2_STATS_INC(mib2.ipinunknownprotos);
//         return ERR_OK;
// // #endif /* IP_REASSEMBLY */
//     }
//
// // #if IP_OPTIONS_ALLOWED == 0 /* no support for IP options in the IP header? */
//
// // #if LWIP_IGMP
//     /* there is an extra "router alert" option in IGMP messages which we allow for but do not police */
//     if ((iphdr_hlen > IP_HLEN) && (IPH_PROTO(iphdr) != IP_PROTO_IGMP)) {
//         # else
//         if (iphdr_hlen > IP_HLEN) {
// // #endif /* LWIP_IGMP */
//             LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("IP packet dropped since there were IP options (while IP_OPTIONS_ALLOWED == 0).\n"));
//             pbuf_free(p);
//             IP_STATS_INC(ip.opterr);
//             IP_STATS_INC(ip.drop);
//             /* unsupported protocol feature */
//             MIB2_STATS_INC(mib2.ipinunknownprotos);
//             return ERR_OK;
//         }
// // #endif /* IP_OPTIONS_ALLOWED == 0 */
//
//         /* send to upper layers */
//         LWIP_DEBUGF(IP_DEBUG, ("ip4_input: \n"));
//         ip4_debug_print(p);
//         LWIP_DEBUGF(IP_DEBUG, ("ip4_input:  p.len %"U16_F"  p.tot_len %"U16_F"\n", p.len, p.tot_len));
//
//         ip_data.current_netif = netif;
//         ip_data.current_input_netif = inp;
//         ip_data.current_ip4_header = iphdr;
//         ip_data.current_ip_header_tot_len = IPH_HL_BYTES(iphdr);
//
// // #if LWIP_RAW
//         /* raw input did not eat the packet? */
//         raw_status = raw_input(p, inp);
//         if (raw_status != RAW_INPUT_EATEN)
// // #endif /* LWIP_RAW */ {
//             pbuf_remove_header(p, iphdr_hlen); /* Move to payload, no check necessary. */
//
//             switch(IPH_PROTO(iphdr))
//             {
// // #if LWIP_UDP
//                 case
//                 IP_PROTO_UDP:
// // #if LWIP_UDPLITE
//                     case
//                 IP_PROTO_UDPLITE:
// // #endif /* LWIP_UDPLITE */
//                     MIB2_STATS_INC(mib2.ipindelivers);
//                 udp_input(p, inp);
//                 break;
// // #endif /* LWIP_UDP */
// // #if LWIP_TCP
//                 case
//                 IP_PROTO_TCP: MIB2_STATS_INC(mib2.ipindelivers);
//                 tcp_input(p, inp);
//                 break;
// // #endif /* LWIP_TCP */
// // #if LWIP_ICMP
//                 case
//                 IP_PROTO_ICMP: MIB2_STATS_INC(mib2.ipindelivers);
//                 icmp_input(p, inp);
//                 break;
// // #endif /* LWIP_ICMP */
// // #if LWIP_IGMP
//                 case
//                 IP_PROTO_IGMP: igmp_input(p, inp, ip4_current_dest_addr());
//                 break;
// // #endif /* LWIP_IGMP */
//                 default:
// // #if LWIP_RAW
//                 if (raw_status == RAW_INPUT_DELIVERED) {
//                     MIB2_STATS_INC(mib2.ipindelivers);
//                 } else
// // #endif /* LWIP_RAW */ {
// // #if LWIP_ICMP
//                     /* send ICMP destination protocol unreachable unless is was a broadcast */
//                     if (!ip4_addr_isbroadcast(ip4_current_dest_addr(), netif) && !ip4_addr_ismulticast(ip4_current_dest_addr())) {
//                         pbuf_header_force(p, (s16_t)iphdr_hlen); /* Move to ip header, no check necessary. */
//                         icmp_dest_unreach(p, ICMP_DUR_PROTO);
//                     }
// // #endif /* LWIP_ICMP */
//
//                     LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("Unsupported transport protocol %"U16_F"\n", (u16_t)IPH_PROTO(iphdr)));
//
//                     IP_STATS_INC(ip.proterr);
//                     IP_STATS_INC(ip.drop);
//                     MIB2_STATS_INC(mib2.ipinunknownprotos);
//                 }
//                 pbuf_free(p);
//                 break;
//             }
//         }
//
//         /* @todo: this is not really necessary... */
//         ip_data.current_netif = NULL;
//         ip_data.current_input_netif = NULL;
//         ip_data.current_ip4_header = NULL;
//         ip_data.current_ip_header_tot_len = 0;
//         ip4_addr_set_any(ip4_current_src_addr());
//         ip4_addr_set_any(ip4_current_dest_addr());
//
//         return ERR_OK;
    todo!()
    }

    // /**
    // * Sends an IP packet on a network interface. This function constructs
    // * the IP header and calculates the IP header checksum. If the source
    // * IP address is NULL, the IP address of the outgoing network
    // * interface is filled in as source address.
    // * If the destination IP address is LWIP_IP_HDRINCL, p is assumed to already
    // * include an IP header and  p.payload points to it instead of the data.
    // *
    // * @param p the packet to send ( p.payload points to the data, e.g. next
    //            protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
    //            IP header and  p.payload points to that IP header)
    // * @param src the source IP address to send from (if src == IP4_ADDR_ANY, the
    // *         IP  address of the netif used to send is used as source address)
    // * @param dest the destination IP address to send the packet to
    // * @param ttl the TTL value to be set in the IP header
    // * @param tos the TOS value to be set in the IP header
    // * @param proto the PROTOCOL to be set in the IP header
    // * @param netif the netif on which to send this packet
    // * @return ERR_OK if the packet was sent OK
    // *         ERR_BUF if p doesn't have enough space for IP/LINK headers
    // *         returns errors returned by  netif.output
    // *
    // * @note ip_id: RFC791 "some host may be able to simply use
    // *  unique identifiers independent of destination"
    //  */
    pub fn ip4_output_if(
        pkt: &mut PacketBuffer,
        src_addr: &Ipv4Address,
        dst_addr: &Ipv4Addres,
        tll: u8,
        proto: u8,
        netif: &mut NetworkInterface) -> Result<(), LwipError>
    {
// // #if IP_OPTIONS_SEND
//         return ip4_output_if_opt(p, src, dest, ttl, tos, proto, netif, NULL, 0);
        todo!()
    }

    /**
     * Same as ip_output_if() but with the possibility to include IP options:
     *
     * @ param ip_options pointer to the IP options, copied into the IP header
     * @ param optlen length of ip_options
     */
   pub fn ip4_output_if_opt(
        pkt: &mut PacketBuffer,
        src_addr: &mut Ipv4Address,
        dst_addr: &mut Ipv4Address,
        ttl: u8,
        proto: u8,
        netif: &mut NetworkInterface,
        ) -> Result<(), LwipError>
    {
//         // #endif /* IP_OPTIONS_SEND */ const ip4_addr_t
//         *src_used = src;
//         if (dest != LWIP_IP_HDRINCL) {
//             if (ip4_addr_isany(src)) {
//                 src_used = netif_ip4_addr(netif);
//             }
//         }
//
// // #if IP_OPTIONS_SEND
//         return ip4_output_if_opt_src(p, src_used, dest, ttl, tos, proto, netif,
//                                      ip_options, optlen); # else /* IP_OPTIONS_SEND */
//         return ip4_output_if_src(p, src_used, dest, ttl, tos, proto, netif);
// // #endif /* IP_OPTIONS_SEND */
        todo!()
    }

    // /**
    //  * Same as ip_output_if() but 'src' address is not replaced by netif address
    //  * when it is 'any'.
    //  */
pub fn ip4_output_if_src(pkt: &mut PacketBuffer, src_addr: &Ipv4Address, dst_addr: &Ipv4Address, ttl: u8, tos: u8, proto: u8, netif: &mut NetworkInterface) -> Result<(), LwipError>
    {
// #if IP_OPTIONS_SEND
//         return ip4_output_if_opt_src(p, src, dest, ttl, tos, proto, netif, NULL, 0);
        todo!()
    }

    // /**
    //  * Same as ip_output_if_opt() but 'src' address is not replaced by netif address
    //  * when it is 'any'.
    //  */

pub fn ip4_output_if_opt_src(pkt: &mut PacketBuffer, src: &Ipv4Address, dst: &Ipv4Address, ttl: u8, tos: u8, proto: u8, netif: &mut NetworkInterface)
    {
//         // #endif /* IP_OPTIONS_SEND */ struct ip_hdr
//         *iphdr;
//         dest_addr: ip4_addr_t;
// // #if CHECKSUM_GEN_IP_INLINE
//         u32_t
//         chk_sum = 0;
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//
//         // LWIP_ASSERT_CORE_LOCKED()
//         LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);
//
//         MIB2_STATS_INC(mib2.ipoutrequests);
//
//         /* Should the IP header be generated or is it already included in p? */
//         if (dest != LWIP_IP_HDRINCL) {
//             u16_t
//             ip_hlen = IP_HLEN;
// // #if IP_OPTIONS_SEND
//             u16_t
//             optlen_aligned = 0;
//             if (optlen != 0) {
// // #if CHECKSUM_GEN_IP_INLINE
//                 int
//                 i;
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//                 if (optlen > (IP_HLEN_MAX - IP_HLEN)) {
//                     /* optlen too long */
//                     LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output_if_opt: optlen too long\n"));
//                     IP_STATS_INC(ip.err);
//                     MIB2_STATS_INC(mib2.ipoutdiscards);
//                     return ERR_VAL;
//                 }
//                 /* round up to a multiple of 4 */
//                 optlen_aligned = (u16_t)((optlen + 3) & ~3);
//                 ip_hlen = (u16_t)(ip_hlen + optlen_aligned);
//                 /* First write in the IP options */
//                 if (pbuf_add_header(p, optlen_aligned)) {
//                     LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output_if_opt: not enough room for IP options in pbuf\n"));
//                     IP_STATS_INC(ip.err);
//                     MIB2_STATS_INC(mib2.ipoutdiscards);
//                     return ERR_BUF;
//                 }
//                 MEMCPY(p.payload, ip_options, optlen);
//                 if (optlen < optlen_aligned) {
//                     /* zero the remaining bytes */
//                     memset(((char *) p.payload) + optlen, 0, (size_t)(optlen_aligned - optlen));
//                 }
// // #if CHECKSUM_GEN_IP_INLINE
//                 for (i = 0; i < optlen_aligned / 2; i+ +) {
//                     chk_sum += ((u16_t *)
//                     p.payload)[i];
//                 }
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//             }
// // #endif /* IP_OPTIONS_SEND */
//             /* generate IP header */
//             if (pbuf_add_header(p, IP_HLEN)) {
//                 LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output: not enough room for IP header in pbuf\n"));
//
//                 IP_STATS_INC(ip.err);
//                 MIB2_STATS_INC(mib2.ipoutdiscards);
//                 return ERR_BUF;
//             }
//
//             iphdr = (struct ip_hdr
//             *) p.payload;
//             // LWIP_ASSERT("check that first pbuf can hold struct ip_hdr",
//             (p.len >= sizeof(struct ip_hdr)));
//
//             IPH_TTL_SET(iphdr, ttl);
//             IPH_PROTO_SET(iphdr, proto);
// // #if CHECKSUM_GEN_IP_INLINE
//             chk_sum += PP_NTOHS(proto | (ttl << 8));
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//
//             /* dest cannot be NULL here */
//             ip4_addr_copy(iphdr.dest, *dest);
// // #if CHECKSUM_GEN_IP_INLINE
//             chk_sum += ip4_addr_get_u32(&iphdr.dest) & 0xFFFF;
//             chk_sum += ip4_addr_get_u32(&iphdr.dest) >> 16;
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//
//             IPH_VHL_SET(iphdr, 4, ip_hlen / 4);
//             IPH_TOS_SET(iphdr, tos);
// // #if CHECKSUM_GEN_IP_INLINE
//             chk_sum += PP_NTOHS(tos | (iphdr._v_hl << 8));
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//             IPH_LEN_SET(iphdr, lwip_htons(p.tot_len));
// // #if CHECKSUM_GEN_IP_INLINE
//             chk_sum += iphdr._len;
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//             IPH_OFFSET_SET(iphdr, 0);
//             IPH_ID_SET(iphdr, lwip_htons(ip_id));
// // #if CHECKSUM_GEN_IP_INLINE
//             chk_sum += iphdr._id;
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//             + + ip_id;
//
//             if (src == NULL) {
//                 ip4_addr_copy(iphdr.src, *IP4_ADDR_ANY4);
//             } else {
//                 /* src cannot be NULL here */
//                 ip4_addr_copy(iphdr.src, *src);
//             }
//
// // #if CHECKSUM_GEN_IP_INLINE
//             chk_sum += ip4_addr_get_u32(&iphdr.src) & 0xFFFF;
//             chk_sum += ip4_addr_get_u32(&iphdr.src) >> 16;
//             chk_sum = (chk_sum >> 16) + (chk_sum & 0xFFFF);
//             chk_sum = (chk_sum >> 16) + chk_sum;
//             chk_sum = ~chk_sum;
//             IF__NETIF_CHECKSUM_ENABLED(netif, NETIF_CHECKSUM_GEN_IP)
//             {
//                 iphdr._chksum = (u16_t)
//                 chk_sum; /* network order */
//             }
// // #if LWIP_CHECKSUM_CTRL_PER_NETIF
//             else {
//                 IPH_CHKSUM_SET(iphdr, 0);
//             }
// // #endif /* LWIP_CHECKSUM_CTRL_PER_NETIF*/ # else /* CHECKSUM_GEN_IP_INLINE */
//             IPH_CHKSUM_SET(iphdr, 0);
// // #if CHECKSUM_GEN_IP
//             IF__NETIF_CHECKSUM_ENABLED(netif, NETIF_CHECKSUM_GEN_IP)
//             {
//                 IPH_CHKSUM_SET(iphdr, inet_chksum(iphdr, ip_hlen));
//             }
// // #endif /* CHECKSUM_GEN_IP */
// // #endif /* CHECKSUM_GEN_IP_INLINE */
//         } else {
//             /* IP header already included in p */
//             if (p.len < IP_HLEN) {
//                 LWIP_DEBUGF(IP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("ip4_output: LWIP_IP_HDRINCL but pbuf is too short\n"));
//                 IP_STATS_INC(ip.err);
//                 MIB2_STATS_INC(mib2.ipoutdiscards);
//                 return ERR_BUF;
//             }
//             iphdr = (struct ip_hdr
//             *) p.payload;
//             ip4_addr_copy(dest_addr, iphdr.dest);
//             dest = &dest_addr;
//         }
//
//         IP_STATS_INC(ip.xmit);
//
//         LWIP_DEBUGF(IP_DEBUG, ("ip4_output_if: %c%c%"U16_F"\n", netif.name[0], netif.name[1], (u16_t) netif.num));
//         ip4_debug_print(p);
//
// // #if ENABLE_LOOPBACK
//         if (ip4_addr_eq(dest, netif_ip4_addr(netif)) # if !LWIP_HAVE_LOOPIF || ip4_addr_isloopback(dest)
// // #endif /* !LWIP_HAVE_LOOPIF */
//         ) {
//         /* Packet to self, enqueue it for loopback */
//         LWIP_DEBUGF(IP_DEBUG, ("netif_loop_output()"));
//         return netif_loop_output(netif, p);
//     }
// // #if LWIP_MULTICAST_TX_OPTIONS
//         if ((p.flags & PBUF_FLAG_MCASTLOOP) != 0) {
//             netif_loop_output(netif, p);
//         }
// // #endif /* LWIP_MULTICAST_TX_OPTIONS */
// // #endif /* ENABLE_LOOPBACK */
// // #if IP_FRAG
//         /* don't fragment if interface has mtu set to 0 [loopif] */
//         if (netif.mtu && (p.tot_len > netif.mtu)) {
//             return ip4_frag(p, netif, dest);
//         }
// // #endif /* IP_FRAG */
//
//         LWIP_DEBUGF(IP_DEBUG, ("ip4_output_if: call  netif.output()\n"));
//         return netif.output(netif, p, dest);
        todo!()
    }

    // /**
    // * Simple interface to ip_output_if. It finds the outgoing network
    // * interface and calls upon ip_output_if to do the actual work.
    // *
    // * @param p the packet to send ( p.payload points to the data, e.g. next
    //            protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
    //            IP header and  p.payload points to that IP header)
    // * @param src the source IP address to send from (if src == IP4_ADDR_ANY, the
    // *         IP  address of the netif used to send is used as source address)
    // * @param dest the destination IP address to send the packet to
    // * @param ttl the TTL value to be set in the IP header
    // * @param tos the TOS value to be set in the IP header
    // * @param proto the PROTOCOL to be set in the IP header
    // *
    // * @return ERR_RTE if no route is found
    // *         see ip_output_if() for more return values
    //  */

pub fn ip4_output(pkt: &mut PacketBuffer, src: &Ipv4Address, dst: &Ipv4Address, ttl: u8, tos: u8, proto: u8) -> Result<(), LwipError>
    {
        // struct netif
        // *netif;
        //
        // LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);
        //
        // if ((netif = ip4_route_src(src, dest)) == NULL) {
        //     LWIP_DEBUGF(IP_DEBUG, ("ip4_output: No route to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
        //                 ip4_addr1_16(dest), ip4_addr2_16(dest), ip4_addr3_16(dest), ip4_addr4_16(dest)));
        //     IP_STATS_INC(ip.rterr);
        //     return ERR_RTE;
        // }
        //
        // return ip4_output_if(p, src, dest, ttl, tos, proto, netif);
        todo!()
    }

// #if LWIP_NETIF_USE_HINTS
//     /** Like ip_output, but takes and addr_hint pointer that is passed on to  netif.addr_hint
//     *  before calling ip_output_if.
//     *
//     * @param p the packet to send ( p.payload points to the data, e.g. next
//                protocol header; if dest == LWIP_IP_HDRINCL, p already includes an
//                IP header and  p.payload points to that IP header)
//     * @param src the source IP address to send from (if src == IP4_ADDR_ANY, the
//     *         IP  address of the netif used to send is used as source address)
//     * @param dest the destination IP address to send the packet to
//     * @param ttl the TTL value to be set in the IP header
//     * @param tos the TOS value to be set in the IP header
//     * @param proto the PROTOCOL to be set in the IP header
//     * @param netif_hint netif output hint pointer set to  netif.hint before
//     *        calling ip_output_if()
//     *
//     * @return ERR_RTE if no route is found
//     *         see ip_output_if() for more return values
//      */
pub fn ip4_output_hinted(pkt: &mut PacketBuffer, src_addr: &Ipv4Address, dst_addr: &Ipv4Address, ttl: u8, tos: u8, proto: u8)
    {
        // struct netif
        // *netif;
        // err_t
        // err;
        //
        // LWIP_IP_CHECK_PBUF_REF_COUNT_FOR_TX(p);
        //
        // if ((netif = ip4_route_src(src, dest)) == NULL) {
        //     LWIP_DEBUGF(IP_DEBUG, ("ip4_output: No route to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
        //                 ip4_addr1_16(dest), ip4_addr2_16(dest), ip4_addr3_16(dest), ip4_addr4_16(dest)));
        //     IP_STATS_INC(ip.rterr);
        //     return ERR_RTE;
        // }
        //
        // NETIF_SET_HINTS(netif, netif_hint);
        // err = ip4_output_if(p, src, dest, ttl, tos, proto, netif);
        // NETIF_RESET_HINTS(netif);
        //
        // return err;
        todo!()
    }
// #endif /* LWIP_NETIF_USE_HINTS*/

// #if IP_DEBUG
//     /* Print an IP header by using LWIP_DEBUGF
//      * @param p an IP packet,  p.payload pointing to the IP header
//      */

pub fn ip4_debug_print(pkt: &mut PacketBuffer)
    {
        // struct ip_hdr
        // *iphdr = (struct ip_hdr
        // *) p.payload;
        //
        // LWIP_DEBUGF(IP_DEBUG, ("IP header:\n"));
        // LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
        // LWIP_DEBUGF(IP_DEBUG, ("|%2"S16_F" |%2"S16_F" |  0x%02"X16_F" |     %5"U16_F"     | (v, hl, tos, len)\n",
        //             (u16_t)IPH_V(iphdr),
        //             (u16_t)IPH_HL(iphdr),
        //             (u16_t)IPH_TOS(iphdr),
        //             lwip_ntohs(IPH_LEN(iphdr))));
        // LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
        // LWIP_DEBUGF(IP_DEBUG, ("|    %5"U16_F"      |%"U16_F"%"U16_F"%"U16_F"|    %4"U16_F"   | (id, flags, offset)\n",
        //             lwip_ntohs(IPH_ID(iphdr)),
        //             (u16_t)(lwip_ntohs(IPH_OFFSET(iphdr)) >> 15 & 1),
        //             (u16_t)(lwip_ntohs(IPH_OFFSET(iphdr)) >> 14 & 1),
        //             (u16_t)(lwip_ntohs(IPH_OFFSET(iphdr)) >> 13 & 1),
        //             (u16_t)(lwip_ntohs(IPH_OFFSET(iphdr)) & IP_OFFMASK)));
        // LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
        // LWIP_DEBUGF(IP_DEBUG, ("|  %3"U16_F"  |  %3"U16_F"  |    0x%04"X16_F"     | (ttl, proto, chksum)\n",
        //             (u16_t)IPH_TTL(iphdr),
        //             (u16_t)IPH_PROTO(iphdr),
        //             lwip_ntohs(IPH_CHKSUM(iphdr))));
        // LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
        // LWIP_DEBUGF(IP_DEBUG, ("|  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  | (src)\n",
        //             ip4_addr1_16_val(iphdr.src),
        //             ip4_addr2_16_val(iphdr.src),
        //             ip4_addr3_16_val(iphdr.src),
        //             ip4_addr4_16_val(iphdr.src)));
        // LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
        // LWIP_DEBUGF(IP_DEBUG, ("|  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  |  %3"U16_F"  | (dest)\n",
        //             ip4_addr1_16_val(iphdr.dest),
        //             ip4_addr2_16_val(iphdr.dest),
        //             ip4_addr3_16_val(iphdr.dest),
        //             ip4_addr4_16_val(iphdr.dest)));
        // LWIP_DEBUGF(IP_DEBUG, ("+-------------------------------+\n"));
    }
// #endif /* IP_DEBUG */

// #endif /* LWIP_IPV4 */
