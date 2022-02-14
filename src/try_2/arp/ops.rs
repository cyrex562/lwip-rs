use crate::arp::defs::{ARP_AGE_REREQUEST_USED_BROADCAST, ARP_AGE_REREQUEST_USED_UNICAST, ARP_MAXPENDING, ArpEntry, ArpOpcode, ArpState, ETHARP_FLAG_FIND_ONLY, ETHARP_FLAG_TRY_HARD, etharp_free_entry, ArpHeader, EtharpQEntry, SIZEOF_ETHARP_HDR};
use crate::arp::defs;
use crate::arp::defs::ArpOpcode::{Reply, Request};
use crate::arp::defs::ArpState::{Empty, EtharpStatePending, EtharpStateStable, EtharpStateStableRerequesting1, EtharpStateStableRerequesting2, Static};
use crate::arp::etharp_h::{IPADDR_WORDALIGNED_COPY_FROM_ip4_addr, IPADDR_WORDALIGNED_COPY_TO_ip4_addr};
use crate::autoip::autoip2::autoip_arp_reply;
use crate::core::common::PP_HTONS;
use crate::core::context::LwipContext;
use crate::core::defines::LwipAddr;
use crate::core::error::{ERR_ARG, ERR_MEM, ERR_OK, ERR_RTE, LwipError};
use crate::core::error::LwipErrorCodes::ERR_MEM;
use crate::dhcp::dhcp2::dhcp_arp_reply;
use crate::ethernet::defs::{ETH_HWADDR_LEN, MacAddress};
use crate::ethernet::ops::ethernet_output;
use crate::ip::ip4_addr::{ip4_addr2, ip4_addr3, ip4_addr4, ip4_addr_isany, ip4_addr_isany_val, ip4_addr_islinklocal, ip4_addr_ismulticast, ip4_addr_netcmp, Ipv4Address};
use crate::ip::ip4_addr_h::{ip4_addr, ip4_addr_cmp};
use crate::netif::defs::NetworkInterfaceCtx;
use crate::netif::netif_h::{netif_ip4_addr, netif_ip4_gw, netif_ip4_netmask};
use crate::packetbuffer::pbuf::{pbuf_alloc, pbuf_clone, pbuf_free, pbuf_ref};
use crate::packetbuffer::pbuf_h::{PacketBuffer, PBUF_LINK, PBUF_NEEDS_COPY, PBUF_RAM};
use crate::tcp::port_numbers::lwip_iana_hwtype::LWIP_IANA_HWTYPE_ETHERNET;
use log::{debug, info, warn, error};
use crate::ethernet::multicast_addresses::{ETHER_BCAST_ADDR, IPV4_MCAST_MAC_ADDR_RANGE};

/// Removes expired timers from the ARP table
pub fn etharp_tmr(ctx: &mut LwipContext) {
    let i: i32;

    for i in 0..ctx.arp_table.len() {
        let mut entry = &mut ctx.arp_table[i];
      if entry.state != ArpState::Empty && (state != ArpState::Static) {
        entry.ctime += 1;
        if (entry.ctime >= arp_maxage) || ((entry.state == ArpState.EtharpStatePending)  && (entry.ctime >= ARP_MAXPENDING)) {
          //  pending or stable entry has become old!
          log::debug!("etharp_timer: expired {:?} entry {}.", entry.state, i);
          //  clean up entries that have just been expired
          etharp_free_entry(ctx, i as isize);
        } else if entry.state == EtharpStateStableRerequesting1 {
          //  Don't send more than one request every 2 seconds.
          entry.state = EtharpStateStableRerequesting2;
        } else if entry.state == EtharpStateStableRerequesting2 {
          /* Reset state to stable, so that the next transmitted packet will
             re-send an ARP request. */
          entry.state = EtharpStateStable;
        } else if entry.state == EtharpStatePending {
          //  still pending, resend an ARP query
          etharp_request(entry.netif, &entry.ipaddr);
        }
      }
    }
}

pub fn etharp_input(p: &mut PacketBuffer, netif: &mut NetworkInterfaceCtx) {
    let hdr: &mut ArpHeader;
    let src_ip4_addr: Ipv4Address;
    let dst_ip4_addr: Ipv4Address;
    let for_us: u8;

    hdr = p.payload;

    //  RFC 826 "Packet Reception": 
    if (hdr.hw_type != PP_HTONS(LWIP_IANA_HWTYPE_ETHERNET))
        || (hdr.hw_len != ETH_HWADDR_LEN as u8)
        || (hdr.protolen != sizeof(ip4_addr))
        || (hdr.proto != PP_HTONS(ETHTYPE_IP))
    {
        // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_LEVEL_WARNING,
        //             ("etharp_input: packet dropped, wrong hw type, hwlen, proto, protolen or ethernet type (%"U16_F"/%"U16_F"/%"U16_F"/%"U16_F")\n",
        //              hdr.hwtype, hdr.hwlen, hdr.proto, hdr.protolen));
        ETHARP_STATS_INC(etharp.proterr);
        ETHARP_STATS_INC(etharp.drop);
        pbuf_free(p);
        return;
    }
    ETHARP_STATS_INC(etharp.recv);

    /* We have to check if a host already has configured our random
     * created link local address and continuously check if there is
     * a host with this IP-address so we can detect collisions */
    autoip_arp_reply(netif, hdr);

    /* Copy struct ip4_addr_wordaligned to aligned ip4_addr, to support compilers without
     * structure packing (not using structure copy which breaks strict-aliasing rules). */
    IPADDR_WORDALIGNED_COPY_TO_ip4_addr(&src_ip4_addr, &hdr.src_ip_addr);
    IPADDR_WORDALIGNED_COPY_TO_ip4_addr(&dst_ip4_addr, &hdr.dst_ip_addr);

    //  this interface is not configured? 
    if (ip4_addr_isany_val(*netif_ip4_addr(netif))) {
        for_us = 0;
    } else {
        //  ARP packet directed to us? 
        for_us = ip4_addr_cmp(&dst_ip4_addr, netif_ip4_addr(netif));
    }

    /* ARP message directed to us?
     -> add IP address in ARP cache; assume requester wants to talk to us,
        can result in directly sending the queued packets for this host.
    ARP message not directed to us?
     ->  update the source IP address in the cache, if present */
    let mut a = ETHARP_FLAG_FIND_ONLY;
    if for_us {
        a = ETHARP_FLAG_TRY_HARD;
    }
    let mac_addr = MacAddress::from_slice(&hdr.src_hw_addr);
    defs::etharp_update_arp_entry(ctx, netif, &src_ip4_addr, &mac_addr, a);

    //  now act on the message itself 
    match hdr.op_code {
        //  ARP request? 
        PP_HTONS(Request) => {
            /* ARP request. If it asked for our address, we send out a
             * reply. In any case, we time-stamp any existing ARP entry,
             * and possibly send out an IP packet that was queued on it. */
            log::debug!("etharp input: incoming ARP request");
            //  ARP request for our address? 
            if for_us {
                //  send ARP response 
                etharp_raw(
                    netif,
                    &netif.hwaddr,
                    &MacAddress::from_slice(&hdr.src_hw_addr),
                    &netif.hwaddr,
                    netif_ip4_addr(netif),
                    &MacAddress::from_slice(&hdr.src_hw_addr),
                    &src_ip4_addr,
                    Reply,
                );
                //  we are not configured? 
            } else if ip4_addr_isany_val(*netif_ip4_addr(netif)) {

                //  { for_us == 0 and netif.ip_addr.addr == 0 } 
                /*LWIP_DEBUGF(
                    ETHARP_DEBUG | LWIP_DBG_TRACE,
                    ("etharp_input: we are unconfigured, ARP request ignored.\n"),
                );*/
                //  request was not directed to us 
            } else {
                //  { for_us == 0 and netif.ip_addr.addr != 0 } 
                /*LWIP_DEBUGF(
                    ETHARP_DEBUG | LWIP_DBG_TRACE,
                    ("etharp_input: ARP request was not for us.\n"),
                );*/
            }
        }

        PP_HTONS(Reply) => {
            //  ARP reply. We already updated the ARP cache earlier. 
            /*LWIP_DEBUGF(
                ETHARP_DEBUG | LWIP_DBG_TRACE,
                ("etharp_input: incoming ARP reply\n"),
            );*/

            /* DHCP wants to know about ARP replies from any host with an
             * IP address also offered to us by the DHCP server. We do not
             * want to take a duplicate IP address on a single network.
             * @todo How should we handle redundant (fail-over) interfaces? */
            dhcp_arp_reply(netif, &src_ip4_addr);
        }
        _ => {
            // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_input: ARP unknown opcode type %"S16_F"\n", lwip_htons(hdr.opcode)));
            ETHARP_STATS_INC(etharp.err);
        }
    }
    //  free ARP packet 
    pbuf_free(p);
}

pub fn etharp_output_to_arp_index(
    netif: &mut NetworkInterfaceCtx,
    q: &mut PacketBuffer,
    arp_idx: netif_addr_idx_t,
) -> Result<(), LwipError> {
    LWIP_ASSERT(
        "arp_table[arp_idx].state >= EtharpStateStable",
        arp_table[arp_idx].state >= ETHARP_STATE_STABLE,
    );
    /* if arp table entry is about to expire: re-request it,
    but only if its state is EtharpStateStable to prevent flooding the
    network with ARP requests if this address is used frequently. */
    if (arp_table[arp_idx].state == ETHARP_STATE_STABLE) {
        if (arp_table[arp_idx].ctime >= ARP_AGE_REREQUEST_USED_BROADCAST) {
            //  issue a standard request using broadcast 
            if (etharp_request(netif, &arp_table[arp_idx].ipaddr) == ERR_OK) {
                arp_table[arp_idx].state = ETHARP_STATE_STABLE_REREQUESTING_1;
            }
        } else if (arp_table[arp_idx].ctime >= ARP_AGE_REREQUEST_USED_UNICAST) {
            //  issue a unicast request (for 15 seconds) to prevent unnecessary broadcast 
            if (etharp_request_dst(
                netif,
                &arp_table[arp_idx].ipaddr,
                &arp_table[arp_idx].ethaddr,
            ) == ERR_OK)
            {
                arp_table[arp_idx].state = ETHARP_STATE_STABLE_REREQUESTING_1;
            }
        }
    }

    return ethernet_output(
        netif,
        q,
        (&netif.hwaddr),
        &arp_table[arp_idx].ethaddr,
        ETHTYPE_IP,
        None,
    );
}

pub fn etharp_output(netif: &mut NetworkInterfaceCtx, q: &mut PacketBuffer, ipaddr: &mut Ipv4Address) -> Result<(), LwipError> {
    let mut dest: MacAddress;
    let mut mcastaddr: MacAddress;
    let mut dst_addr: Ipv4Address = ipaddr.clone();

    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT("netif != NULL", netif != None);
    LWIP_ASSERT("q != NULL", q != None);
    LWIP_ASSERT("ipaddr != NULL", ipaddr != None);

    /* Determine on destination hardware address. Broadcasts and multicasts
     * are special, other IP addresses are looked up in the ARP table. */

    //  broadcast destination IP address? 
    if ip4_addr_isbroadcast(ipaddr, netif) {
        //  broadcast on Ethernet also 
        dest = ETHER_BCAST_ADDR.clone();
        //  multicast destination IP address? 
    } else if ip4_addr_ismulticast(ipaddr) {
        mcastaddr =  MacAddress::from_slice(&IPV4_MCAST_MAC_ADDR_RANGE.start);
        mcastaddr.octets[3] = ipaddr.octets[2] & 0x7f;
        mcastaddr.octets[4] = ipaddr.octets[3];
        mcastaddr.octets[5] = ipaddr.octets[4];
        //  destination Ethernet address is multicast 
        dest = mcastaddr.clone();
        //  unicast destination IP address? 
    } else {
        let i: netif_addr_idx_t;
        /* outside local network? if so, this can neither be a global broadcast nor
        a subnet broadcast. */
        if !ip4_addr_netcmp(ipaddr, netif_ip4_addr(netif), netif_ip4_netmask(netif))
            && !ip4_addr_islinklocal(ipaddr)
        {
            let iphdr: &mut ip_hdr = q.payload;
            /* According to RFC 3297, chapter 2.6.2 (Forwarding Rules), a packet with
            a link-local source address must always be "directly to its destination
            on the same physical link. The host MUST NOT send the packet to any
            router for forwarding". */
            if (!ip4_addr_islinklocal(&iphdr.src)) {
                /* For advanced routing, a single default gateway might not be enough, so get
                the IP address of the gateway to handle the current destination address. */
                dst_addr = LWIP_HOOK_ETHARP_GET_GW(netif, ipaddr);
                if (dst_addr == None) {
                    //  interface has default gateway? 
                    if (!ip4_addr_isany_val(*netif_ip4_gw(netif))) {
                        //  send to hardware address of default gateway IP address 
                        dst_addr = netif_ip4_gw(netif);
                        //  no default gateway available 
                    } else {
                        //  no route to destination error (default gateway missing) 
                        return ERR_RTE;
                    }
                }
            }
        }

        if (netif.hints != None) {
            //  per-pcb cached entry was given 
            let etharp_cached_entry: netif_addr_idx_t = netif.hints.addr_hint;
            if (etharp_cached_entry < ARP_TABLE_SIZE) {
                if ((arp_table[etharp_cached_entry].state >= ETHARP_STATE_STABLE)
                    && (arp_table[etharp_cached_entry].netif == netif)
                    && (ip4_addr_cmp(dst_addr, &arp_table[etharp_cached_entry].ipaddr)))
                {
                    //  the per-pcb-cached entry is stable and the right one! 
                    ETHARP_STATS_INC(etharp.cachehit);
                    return etharp_output_to_arp_index(netif, q, etharp_cached_entry);
                }
            }
        }

        /* find stable entry: do this here since this is a critical path for
        throughput and etharp_find_entry() is kind of slow */
        // for (i = 0; i < ARP_TABLE_SIZE; i+= 1) {
        //   if ((arp_table[i].state >= EtharpStateStable) &&

        //       (arp_table[i].netif == netif) &&

        //       (ip4_addr_cmp(dst_addr, &arp_table[i].ipaddr))) {
        //     //  found an existing, stable entry 
        //     etharp_set_addrhint(netif, i);
        //     return etharp_output_to_arp_index(netif, q, i);
        //   }
        // }
        /* no stable entry found, use the (slower) query function:
        queue on destination Ethernet address belonging to ipaddr */
        return etharp_query(ctx, netif, &dst_addr, q);
    }

    //  continuation for multicast/broadcast destinations 
    //  obtain source Ethernet address of the given interface 
    //  send packet directly on the link 
    return ethernet_output(netif, q, &netif.hwaddr, &dest, ETHTYPE_IP, None);
}

pub fn etharp_query(ctx: &mut LwipContext, netif: &mut NetworkInterfaceCtx, ipaddr: &Ipv4Address, q: &mut PacketBuffer) -> Result<(), LwipError> {
    let srcaddr: MacAddress = netif.hwaddr.clone();
    let result: err_t = ERR_MEM;
    let is_new_entry: i32 = 0;
    let i_err: i16;
    let i: netif_addr_idx_t;

    //  non-unicast address? 
    if (ip4_addr_isbroadcast(ipaddr, netif)
        || ip4_addr_ismulticast(ipaddr)
        || ip4_addr_isany(ipaddr))
    {
        /*LWIP_DEBUGF(
            ETHARP_DEBUG | LWIP_DBG_TRACE,
            ("etharp_query: will not add non-unicast IP address to ARP cache\n"),
        );*/
        return ERR_ARG;
    }

    //  find entry in ARP cache, ask to create entry if queueing packet 
    i_err = defs::etharp_find_entry(ctx, Some(ipaddr), ETHARP_FLAG_TRY_HARD, Some(netif));

    //  could not find or create entry? 
    if (i_err < 0) {
        /*LWIP_DEBUGF(
            ETHARP_DEBUG | LWIP_DBG_TRACE,
            ("etharp_query: could not create ARP entry\n"),
        );*/
        if (q) {
            /*LWIP_DEBUGF(
                ETHARP_DEBUG | LWIP_DBG_TRACE,
                ("etharp_query: packet dropped\n"),
            );*/
            ETHARP_STATS_INC(etharp.memerr);
        }
        return i_err;
    }
    LWIP_ASSERT("type overflow", i_err < NETIF_ADDR_IDX_MAX);
    i = i_err;

    //  mark a fresh entry as pending (we just sent a request) 
    if (arp_table[i].state == ETHARP_STATE_EMPTY) {
        is_new_entry = 1;
        arp_table[i].state = ETHARP_STATE_PENDING;
        //  record network interface for re-sending arp request in etharp_tmr 
        arp_table[i].netif = netif;
    }

    //  { i is either a STABLE or (new or existing) PENDING entry } 
    LWIP_ASSERT(
        "arp_table[i].state == PENDING or STABLE",
        ((arp_table[i].state == ETHARP_STATE_PENDING)
            || (arp_table[i].state >= ETHARP_STATE_STABLE)),
    );

    //  do we have a new entry? or an implicit query request? 
    if (is_new_entry || (q == None)) {
        //  try to resolve it; send out ARP request 
        result = etharp_request(netif, ipaddr);
        if (result != ERR_OK) {
            //  ARP request couldn't be sent 
            /* We don't re-send arp request in etharp_tmr, but we still queue packets,
            since this failure could be temporary, and the next packet calling
            etharp_query again could lead to sending the queued packets. */
        }
        if (q == None) {
            return result;
        }
    }

    //  packet given? 
    LWIP_ASSERT("q != NULL", q != None);
    //  stable entry? 
    if (arp_table[i].state >= ETHARP_STATE_STABLE) {
        //  we have a valid IP.Ethernet address mapping 
        etharp_set_addrhint(netif, i);
        //  send the packet 
        result = ethernet_output(netif, q, srcaddr, &(arp_table[i].ethaddr), ETHTYPE_IP);
        //  pending entry? (either just created or already pending 
    } else if (arp_table[i].state == ETHARP_STATE_PENDING) {
        //  entry is still pending, queue the given packet 'q' 
        let p: &mut PacketBuffer;
        let copy_needed: i32 = 0;
        /* IF q includes a pbuf that must be copied, copy the whole chain into a
         * new PBUF_RAM. See the definition of PBUF_NEEDS_COPY for details. */
        p = q;
        while (p) {
            LWIP_ASSERT(
                "no packet queues allowed!",
                (p.len != p.tot_len) || (p.next == 0),
            );
            if (PBUF_NEEDS_COPY(p)) {
                copy_needed = 1;
                break;
            }
            p = p.next;
        }
        if (copy_needed) {
            //  copy the whole packet into new pbufs 
            p = pbuf_clone(PBUF_LINK, PBUF_RAM, q);
        } else {
            //  referencing the old pbuf is enough 
            p = q;
            pbuf_ref(p);
        }
        //  packet could be taken over? 
        if (p != None) {
            //  queue packet ... 

            let new_entry: &mut EtharpQEntry;
            //  allocate a new arp queue entry 
            new_entry = memp_malloc(MEMP_ARP_QUEUE);
            if (new_entry != None) {
                let qlen: i32 = 0;
                new_entry.next = 0;
                new_entry.p = p;
                if (arp_table[i].q != None) {
                    //  queue was already existent, append the new entry to the end 
                    let r: &mut EtharpQEntry;
                    r = arp_table[i].q;
                    qlen += 1;
                    while (r.next != None) {
                        r = r.next;
                        qlen += 1;
                    }
                    r.next = new_entry;
                } else {
                    //  queue did not exist, first item in queue 
                    arp_table[i].q = new_entry;
                }

                if (qlen >= ARP_QUEUE_LEN) {
                    let old: &mut EtharpQEntry;
                    old = arp_table[i].q;
                    arp_table[i].q = arp_table[i].q.next;
                    pbuf_free(old.p);
                    memp_free(MEMP_ARP_QUEUE, old);
                }

                // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: queued packet %p on ARP entry %"U16_F"\n", q, i));
                result = ERR_OK;
            } else {
                //  the pool MEMP_ARP_QUEUE is empty 
                pbuf_free(p);
                //                LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: could not queue a copy of PBUF_REF packet %p (out of memory)\n", q));
                result = ERR_MEM;
            }
            //  arp_queueing 
            //  always queue one packet per ARP request only, freeing a previously queued packet 
            if (arp_table[i].q != None) {
                // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: dropped previously queued packet %p for ARP entry %"U16_F"\n", q, i));
                pbuf_free(arp_table[i].q);
            }
            arp_table[i].q = p;
            result = ERR_OK;
            // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: queued packet %p on ARP entry %"U16_F"\n", q, i));
        } else {
            ETHARP_STATS_INC(etharp.memerr);
            /*LWIP_DEBUGF(
                ETHARP_DEBUG | LWIP_DBG_TRACE,
                (
                    "etharp_query: could not queue a copy of PBUF_REF packet %p (out of memory)\n",
                    q,
                ),
            );*/
            result = ERR_MEM;
        }
    }
    return result;
}

pub fn etharp_raw(
    netif: &NetworkInterfaceCtx,
    ethsrc_addr: &MacAddress,
    ethdst_addr: &MacAddress,
    hwsrc_addr: &MacAddress,
    ipsrc_addr: &Ipv4Address,
    hwdst_addr: &MacAddress,
    ipdst_addr: &Ipv4Address,
    opcode: ArpOpcode,
) -> Result<(), LwipError> {
    let result: err_t = ERR_OK;
    let hdr: &mut ArpHeader;
    //  allocate a pbuf for the outgoing ARP request packet
    let mut p = pbuf_alloc(PBUF_LINK, SIZEOF_ETHARP_HDR, PBUF_RAM);
    //  could allocate a pbuf for an ARP request? 

    hdr = p.payload;
    hdr.op_code = lwip_htons(opcode);

    LWIP_ASSERT(
        "netif.hwaddr_len must be the same as ETH_HWADDR_LEN for etharp!",
        (netif.hwaddr_len == ETH_HWADDR_LEN),
    );

    //  Write the ARP MAC-Addresses 
    SMEMCPY(&hdr.src_hw_addr, hwsrc_addr, ETH_HWADDR_LEN);
    SMEMCPY(&hdr.dst_hw_addr, hwdst_addr, ETH_HWADDR_LEN);
    /* Copy struct ip4_addr_wordaligned to aligned ip4_addr, to support compilers without
     * structure packing. */
    IPADDR_WORDALIGNED_COPY_FROM_ip4_addr(&hdr.src_ip_addr, ipsrc_addr);
    IPADDR_WORDALIGNED_COPY_FROM_ip4_addr(&hdr.dst_ip_addr, ipdst_addr);

    hdr.hw_type = PP_HTONS(LWIP_IANA_HWTYPE_ETHERNET);
    hdr.proto = PP_HTONS(ETHTYPE_IP);
    //  set hwlen and protolen 
    hdr.hw_len = ETH_HWADDR_LEN;
    hdr.protolen = sizeof(ip4_addr);

    //  send ARP query 

    /* If we are using Link-Local, all ARP packets that contain a Link-Local
     * 'sender IP address' MUST be sent using link-layer broadcast instead of
     * link-layer unicast. (See RFC3927 Section 2.5, last paragraph) */
    if (ip4_addr_islinklocal(ipsrc_addr)) {
        ethernet_output(netif, p, ethsrc_addr, &ethbroadcast, ETHTYPE_ARP);
    } else {
        ethernet_output(netif, p, ethsrc_addr, ethdst_addr, ETHTYPE_ARP);
    }

    ETHARP_STATS_INC(etharp.xmit);
    //  free ARP query packet 
    pbuf_free(p);
    p = None;
    //  could not allocate pbuf for ARP request 

    return result;
}

pub fn etharp_request_dst(
    netif: &NetworkInterfaceCtx,
    ipaddr: &Ipv4Address,
    hw_dst_addr: &MacAddress,
) -> Result<(), LwipError> {
    return etharp_raw(
        netif,
        netif.hwaddr,
        hw_dst_addr,
        netif.hwaddr,
        netif_ip4_addr(netif),
        &ethzero,
        ipaddr,
        ArpOpcode::Request,
    );
}

pub fn etharp_request(netif: &mut NetworkInterfaceCtx, ipaddr: &Ipv4Address) -> Result<(), LwipError>{
    debug!("sending ARP request");
     etharp_request_dst(netif, ipaddr, &ETHER_BCAST_ADDR)
}
