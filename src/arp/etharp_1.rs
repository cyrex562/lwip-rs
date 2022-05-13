




/**
 * Resolve and fill-in Ethernet address header for outgoing IP packet.
 *
 * For IP multicast and broadcast, corresponding Ethernet addresses
 * are selected and the packet is transmitted on the link.
 *
 * For unicast addresses, the packet is submitted to etharp_query(). In
 * case the IP address is outside the local network, the IP address of
 * the gateway is used.
 *
 * @param netif The lwIP network interface which the IP packet will be sent on.
 * @param q The pbuf(s) containing the IP packet to be sent.
 * @param ipaddr The IP address of the packet destination.
 *
 * @return
 * - ERR_RTE No route to destination (no gateway to external networks),
 * or the return type of either etharp_query() or ethernet_output().
 */
err_t
etharp_output(struct netif *netif, struct pbuf *q, const ip4_addr_t *ipaddr)
{
  const struct eth_addr *dest;
  struct eth_addr mcastaddr;
  const ip4_addr_t *dst_addr = ipaddr;

  // LWIP_ASSERT_CORE_LOCKED()
  // LWIP_ASSERT("netif != NULL", netif != NULL);
  // LWIP_ASSERT("q != NULL", q != NULL);
  // LWIP_ASSERT("ipaddr != NULL", ipaddr != NULL);

  /* Determine on destination hardware address. Broadcasts and multicasts
   * are special, other IP addresses are looked up in the ARP table. */

  /* broadcast destination IP address? */
  if (ip4_addr_isbroadcast(ipaddr, netif)) {
    /* broadcast on Ethernet also */
    dest = (const struct eth_addr *)&ethbroadcast;
    /* multicast destination IP address? */
  } else if (ip4_addr_ismulticast(ipaddr)) {
    /* Hash IP multicast address to MAC address.*/
    mcastaddr.addr[0] = LL_IP4_MULTICAST_ADDR_0;
    mcastaddr.addr[1] = LL_IP4_MULTICAST_ADDR_1;
    mcastaddr.addr[2] = LL_IP4_MULTICAST_ADDR_2;
    mcastaddr.addr[3] = ip4_addr2(ipaddr) & 0x7f;
    mcastaddr.addr[4] = ip4_addr3(ipaddr);
    mcastaddr.addr[5] = ip4_addr4(ipaddr);
    /* destination Ethernet address is multicast */
    dest = &mcastaddr;
    /* unicast destination IP address? */
  } else {
    netif_addr_idx_t i;
    /* outside local network? if so, this can neither be a global broadcast nor
       a subnet broadcast. */
    if (!ip4_addr_net_eq(ipaddr, netif_ip4_addr(netif), netif_ip4_netmask(netif)) &&
        !ip4_addr_islinklocal(ipaddr)) {
// #if LWIP_AUTOIP
      struct ip_hdr *iphdr = LWIP_ALIGNMENT_CAST(struct ip_hdr *,  q.payload);
      /* According to RFC 3297, chapter 2.6.2 (Forwarding Rules), a packet with
         a link-local source address must always be "directly to its destination
         on the same physical link. The host MUST NOT send the packet to any
         router for forwarding". */
      if (!ip4_addr_islinklocal(& iphdr.src))
// #endif /* LWIP_AUTOIP */
      {
#ifdef LWIP_HOOK_ETHARP_GET_GW
        /* For advanced routing, a single default gateway might not be enough, so get
           the IP address of the gateway to handle the current destination address. */
        dst_addr = LWIP_HOOK_ETHARP_GET_GW(netif, ipaddr);
        if (dst_addr == NULL)
// #endif /* LWIP_HOOK_ETHARP_GET_GW */
        {
          /* interface has default gateway? */
          if (!ip4_addr_isany_val(*netif_ip4_gw(netif))) {
            /* send to hardware address of default gateway IP address */
            dst_addr = netif_ip4_gw(netif);
            /* no default gateway available */
          } else {
            /* no route to destination error (default gateway missing) */
            return ERR_RTE;
          }
        }
      }
    }
// #if LWIP_NETIF_HWADDRHINT
    if ( netif.hints != NULL) {
      /* per-pcb cached entry was given */
      netif_addr_idx_t etharp_cached_entry =  netif.hints->addr_hint;
      if (etharp_cached_entry < ARP_TABLE_SIZE) {
// #endif /* LWIP_NETIF_HWADDRHINT */
        if ((arp_table[etharp_cached_entry].state >= ETHARP_STATE_STABLE) &&
// #if ETHARP_TABLE_MATCH_NETIF
            (arp_table[etharp_cached_entry].netif == netif) &&
// #endif
            (ip4_addr_eq(dst_addr, &arp_table[etharp_cached_entry].ipaddr))) {
          /* the per-pcb-cached entry is stable and the right one! */
          ETHARP_STATS_INC(etharp.cachehit);
          return etharp_output_to_arp_index(netif, q, etharp_cached_entry);
        }
// #if LWIP_NETIF_HWADDRHINT
      }
    }
// #endif /* LWIP_NETIF_HWADDRHINT */

    /* find stable entry: do this here since this is a critical path for
       throughput and etharp_find_entry() is kind of slow */
    for (i = 0; i < ARP_TABLE_SIZE; i++) {
      if ((arp_table[i].state >= ETHARP_STATE_STABLE) &&
// #if ETHARP_TABLE_MATCH_NETIF
          (arp_table[i].netif == netif) &&
// #endif
          (ip4_addr_eq(dst_addr, &arp_table[i].ipaddr))) {
        /* found an existing, stable entry */
        ETHARP_SET_ADDRHINT(netif, i);
        return etharp_output_to_arp_index(netif, q, i);
      }
    }
    /* no stable entry found, use the (slower) query function:
       queue on destination Ethernet address belonging to ipaddr */
    return etharp_query(netif, dst_addr, q);
  }

  /* continuation for multicast/broadcast destinations */
  /* obtain source Ethernet address of the given interface */
  /* send packet directly on the link */
  return ethernet_output(netif, q, (struct eth_addr *)( netif.hwaddr), dest, ETHTYPE_IP);
}

/**
 * Send an ARP request for the given IP address and/or queue a packet.
 *
 * If the IP address was not yet in the cache, a pending ARP cache entry
 * is added and an ARP request is sent for the given address. The packet
 * is queued on this entry.
 *
 * If the IP address was already pending in the cache, a new ARP request
 * is sent for the given address. The packet is queued on this entry.
 *
 * If the IP address was already stable in the cache, and a packet is
 * given, it is directly sent and no ARP request is sent out.
 *
 * If the IP address was already stable in the cache, and no packet is
 * given, an ARP request is sent out.
 *
 * @param netif The lwIP network interface on which ipaddr
 * must be queried for.
 * @param ipaddr The IP address to be resolved.
 * @param q If non-NULL, a pbuf that must be delivered to the IP address.
 * q is not freed by this function.
 *
 * @note q must only be ONE packet, not a packet queue!
 *
 * @return
 * - ERR_BUF Could not make room for Ethernet header.
 * - ERR_MEM Hardware address unknown, and no more ARP entries available
 *   to query for address or queue the packet.
 * - ERR_MEM Could not queue packet due to memory shortage.
 * - ERR_RTE No route to destination (no gateway to external networks).
 * - ERR_ARG Non-unicast address given, those will not appear in ARP cache.
 *
 */
err_t
etharp_query(struct netif *netif, const ip4_addr_t *ipaddr, struct pbuf *q)
{
  struct eth_addr *srcaddr = (struct eth_addr *) netif.hwaddr;
  err_t result = ERR_MEM;
  int is_new_entry = 0;
  s16_t i_err;
  netif_addr_idx_t i;

  /* non-unicast address? */
  if (ip4_addr_isbroadcast(ipaddr, netif) ||
      ip4_addr_ismulticast(ipaddr) ||
      ip4_addr_isany(ipaddr)) {
    LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: will not add non-unicast IP address to ARP cache\n"));
    return ERR_ARG;
  }

  /* find entry in ARP cache, ask to create entry if queueing packet */
  i_err = etharp_find_entry(ipaddr, ETHARP_FLAG_TRY_HARD, netif);

  /* could not find or create entry? */
  if (i_err < 0) {
    LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: could not create ARP entry\n"));
    if (q) {
      LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: packet dropped\n"));
      ETHARP_STATS_INC(etharp.memerr);
    }
    return (err_t)i_err;
  }
  // LWIP_ASSERT("type overflow", (size_t)i_err < NETIF_ADDR_IDX_MAX);
  i = (netif_addr_idx_t)i_err;

  /* mark a fresh entry as pending (we just sent a request) */
  if (arp_table[i].state == ETHARP_STATE_EMPTY) {
    is_new_entry = 1;
    arp_table[i].state = ETHARP_STATE_PENDING;
    /* record network interface for re-sending arp request in etharp_tmr */
    arp_table[i].netif = netif;
  }

  /* { i is either a STABLE or (new or existing) PENDING entry } */
  // LWIP_ASSERT("arp_table[i].state == PENDING or STABLE",
              ((arp_table[i].state == ETHARP_STATE_PENDING) ||
               (arp_table[i].state >= ETHARP_STATE_STABLE)));

  /* do we have a new entry? or an implicit query request? */
  if (is_new_entry || (q == NULL)) {
    /* try to resolve it; send out ARP request */
    result = etharp_request(netif, ipaddr);
    if (result != ERR_OK) {
      /* ARP request couldn't be sent */
      /* We don't re-send arp request in etharp_tmr, but we still queue packets,
         since this failure could be temporary, and the next packet calling
         etharp_query again could lead to sending the queued packets. */
    } else {
      /* ARP request successfully sent */
      if ((arp_table[i].state == ETHARP_STATE_PENDING) && !is_new_entry) {
        /* A new ARP request has been sent for a pending entry. Reset the ctime to
           not let it expire too fast. */
        LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: reset ctime for entry %"S16_F"\n", (s16_t)i));
        arp_table[i].ctime = 0;
      }
    }
    if (q == NULL) {
      return result;
    }
  }

  /* packet given? */
  // LWIP_ASSERT("q != NULL", q != NULL);
  /* stable entry? */
  if (arp_table[i].state >= ETHARP_STATE_STABLE) {
    /* we have a valid  IP.Ethernet address mapping */
    ETHARP_SET_ADDRHINT(netif, i);
    /* send the packet */
    result = ethernet_output(netif, q, srcaddr, &(arp_table[i].ethaddr), ETHTYPE_IP);
    /* pending entry? (either just created or already pending */
  } else if (arp_table[i].state == ETHARP_STATE_PENDING) {
    /* entry is still pending, queue the given packet 'q' */
    struct pbuf *p;
    int copy_needed = 0;
    /* IF q includes a pbuf that must be copied, copy the whole chain into a
     * new PBUF_RAM. See the definition of PBUF_NEEDS_COPY for details. */
    p = q;
    while (p) {
      // LWIP_ASSERT("no packet queues allowed!", ( p.len !=  p.tot_len) || ( p.next == NULL));
      if (PBUF_NEEDS_COPY(p)) {
        copy_needed = 1;
        break;
      }
      p =  p.next;
    }
    if (copy_needed) {
      /* copy the whole packet into new pbufs */
      p = pbuf_clone(PBUF_LINK, PBUF_RAM, q);
    } else {
      /* referencing the old pbuf is enough */
      p = q;
      pbuf_ref(p);
    }
    /* packet could be taken over? */
    if (p != NULL) {
      /* queue packet ... */
// #if ARP_QUEUEING
      struct etharp_q_entry *new_entry;
      /* allocate a new arp queue entry */
      new_entry = (struct etharp_q_entry *)memp_malloc(MEMP_ARP_QUEUE);
      if (new_entry != NULL) {
        unsigned int qlen = 0;
         new_entry.next = NULL;
         new_entry.p = p;
        if (arp_table[i].q != NULL) {
          /* queue was already existent, append the new entry to the end */
          struct etharp_q_entry *r;
          r = arp_table[i].q;
          qlen++;
          while ( r.next != NULL) {
            r =  r.next;
            qlen++;
          }
           r.next = new_entry;
        } else {
          /* queue did not exist, first item in queue */
          arp_table[i].q = new_entry;
        }
// #if ARP_QUEUE_LEN
        if (qlen >= ARP_QUEUE_LEN) {
          struct etharp_q_entry *old;
          old = arp_table[i].q;
          arp_table[i].q = arp_table[i]. q.next;
          pbuf_free( old.p);
          memp_free(MEMP_ARP_QUEUE, old);
        }
// #endif
        LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: queued packet %p on ARP entry %"U16_F"\n", (void *)q, i));
        result = ERR_OK;
      } else {
        /* the pool MEMP_ARP_QUEUE is empty */
        pbuf_free(p);
        LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: could not queue a copy of PBUF_REF packet %p (out of memory)\n", (void *)q));
        result = ERR_MEM;
      }
#else /* ARP_QUEUEING */
      /* always queue one packet per ARP request only, freeing a previously queued packet */
      if (arp_table[i].q != NULL) {
        LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: dropped previously queued packet %p for ARP entry %"U16_F"\n", (void *)q, (u16_t)i));
        pbuf_free(arp_table[i].q);
      }
      arp_table[i].q = p;
      result = ERR_OK;
      LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: queued packet %p on ARP entry %"U16_F"\n", (void *)q, (u16_t)i));
// #endif /* ARP_QUEUEING */
    } else {
      ETHARP_STATS_INC(etharp.memerr);
      LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_query: could not queue a copy of PBUF_REF packet %p (out of memory)\n", (void *)q));
      result = ERR_MEM;
    }
  }
  return result;
}

/**
 * Send a raw ARP packet (opcode and all addresses can be modified)
 *
 * @param netif the lwip network interface on which to send the ARP packet
 * @param ethsrc_addr the source MAC address for the ethernet header
 * @param ethdst_addr the destination MAC address for the ethernet header
 * @param hwsrc_addr the source MAC address for the ARP protocol header
 * @param ipsrc_addr the source IP address for the ARP protocol header
 * @param hwdst_addr the destination MAC address for the ARP protocol header
 * @param ipdst_addr the destination IP address for the ARP protocol header
 * @param opcode the type of the ARP packet
 * @return ERR_OK if the ARP packet has been sent
 *         ERR_MEM if the ARP packet couldn't be allocated
 *         any other err_t on failure
 */
static err_t
etharp_raw(struct netif *netif, const struct eth_addr *ethsrc_addr,
           const struct eth_addr *ethdst_addr,
           const struct eth_addr *hwsrc_addr, const ip4_addr_t *ipsrc_addr,
           const struct eth_addr *hwdst_addr, const ip4_addr_t *ipdst_addr,
           const u16_t opcode)
{
  struct pbuf *p;
  err_t result = ERR_OK;
  struct etharp_hdr *hdr;

  // LWIP_ASSERT("netif != NULL", netif != NULL);

  /* allocate a pbuf for the outgoing ARP request packet */
  p = pbuf_alloc(PBUF_LINK, SIZEOF_ETHARP_HDR, PBUF_RAM);
  /* could allocate a pbuf for an ARP request? */
  if (p == NULL) {
    LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_LEVEL_SERIOUS,
                ("etharp_raw: could not allocate pbuf for ARP request.\n"));
    ETHARP_STATS_INC(etharp.memerr);
    return ERR_MEM;
  }
  // LWIP_ASSERT("check that first pbuf can hold struct etharp_hdr",
              ( p.len >= SIZEOF_ETHARP_HDR));

  hdr = (struct etharp_hdr *) p.payload;
  LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_raw: sending raw ARP packet.\n"));
   hdr.opcode = lwip_htons(opcode);

  // LWIP_ASSERT(" netif.hwaddr_len must be the same as ETH_HWADDR_LEN for etharp!",
              ( netif.hwaddr_len == ETH_HWADDR_LEN));

  /* Write the ARP MAC-Addresses */
  SMEMCPY(& hdr.shwaddr, hwsrc_addr, ETH_HWADDR_LEN);
  SMEMCPY(& hdr.dhwaddr, hwdst_addr, ETH_HWADDR_LEN);
  /* Copy struct ip4_addr_wordaligned to aligned ip4_addr, to support compilers without
   * structure packing. */
  IPADDR_WORDALIGNED_COPY_FROM_IP4_ADDR_T(& hdr.sipaddr, ipsrc_addr);
  IPADDR_WORDALIGNED_COPY_FROM_IP4_ADDR_T(& hdr.dipaddr, ipdst_addr);

   hdr.hwtype = PP_HTONS(LWIP_IANA_HWTYPE_ETHERNET);
   hdr.proto = PP_HTONS(ETHTYPE_IP);
  /* set hwlen and protolen */
   hdr.hwlen = ETH_HWADDR_LEN;
   hdr.protolen = sizeof(ip4_addr_t);

  /* send ARP query */
// #if LWIP_AUTOIP
  /* If we are using Link-Local, all ARP packets that contain a Link-Local
   * 'sender IP address' MUST be sent using link-layer broadcast instead of
   * link-layer unicast. (See RFC3927 Section 2.5, last paragraph) */
  if (ip4_addr_islinklocal(ipsrc_addr)) {
    ethernet_output(netif, p, ethsrc_addr, &ethbroadcast, ETHTYPE_ARP);
  } else
// #endif /* LWIP_AUTOIP */
  {
    ethernet_output(netif, p, ethsrc_addr, ethdst_addr, ETHTYPE_ARP);
  }

  ETHARP_STATS_INC(etharp.xmit);
  /* free ARP query packet */
  pbuf_free(p);
  p = NULL;
  /* could not allocate pbuf for ARP request */

  return result;
}

/**
 * Send an ARP request packet asking for ipaddr to a specific eth address.
 * Used to send unicast request to refresh the ARP table just before an entry
 * times out
 *
 * @param netif the lwip network interface on which to send the request
 * @param ipaddr the IP address for which to ask
 * @param hw_dst_addr the ethernet address to send this packet to
 * @return ERR_OK if the request has been sent
 *         ERR_MEM if the ARP packet couldn't be allocated
 *         any other err_t on failure
 */
static err_t
etharp_request_dst(struct netif *netif, const ip4_addr_t *ipaddr, const struct eth_addr *hw_dst_addr)
{
  return etharp_raw(netif, (struct eth_addr *) netif.hwaddr, hw_dst_addr,
                    (struct eth_addr *) netif.hwaddr, netif_ip4_addr(netif), &ethzero,
                    ipaddr, ARP_REQUEST);
}

/**
 * Send an ARP request packet asking for ipaddr.
 *
 * @param netif the lwip network interface on which to send the request
 * @param ipaddr the IP address for which to ask
 * @return ERR_OK if the request has been sent
 *         ERR_MEM if the ARP packet couldn't be allocated
 *         any other err_t on failure
 */
err_t
etharp_request(struct netif *netif, const ip4_addr_t *ipaddr)
{
  LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_request: sending ARP request.\n"));
  return etharp_request_dst(netif, ipaddr, &ethbroadcast);
}

// #if LWIP_ACD
/**
 * Send an ARP request packet probing for an ipaddr.
 * Used to send probe messages for address conflict detection.
 *
 * @param netif the lwip network interface on which to send the request
 * @param ipaddr the IP address to probe
 * @return ERR_OK if the request has been sent
 *         ERR_MEM if the ARP packet couldn't be allocated
 *         any other err_t on failure
 */
err_t
etharp_acd_probe(struct netif *netif, const ip4_addr_t *ipaddr)
{
  return etharp_raw(netif, (struct eth_addr *) netif.hwaddr, &ethbroadcast,
                    (struct eth_addr *) netif.hwaddr, IP4_ADDR_ANY4, &ethzero,
                    ipaddr, ARP_REQUEST);
}

/**
 * Send an ARP request packet announcing an ipaddr.
 * Used to send announce messages for address conflict detection.
 *
 * @param netif the lwip network interface on which to send the request
 * @param ipaddr the IP address to announce
 * @return ERR_OK if the request has been sent
 *         ERR_MEM if the ARP packet couldn't be allocated
 *         any other err_t on failure
 */
err_t
etharp_acd_announce(struct netif *netif, const ip4_addr_t *ipaddr)
{
  return etharp_raw(netif, (struct eth_addr *) netif.hwaddr, &ethbroadcast,
                    (struct eth_addr *) netif.hwaddr, ipaddr, &ethzero,
                    ipaddr, ARP_REQUEST);
}
// #endif /* LWIP_ACD */

// #endif /* LWIP_IPV4 && LWIP_ARP */
