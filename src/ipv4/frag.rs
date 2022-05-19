use crate::errors::{LwipError, LwipErrorCode};
use crate::ipv4::hdr::Ipv4Header;
use crate::packet_buffer::PacketBuffer;

/* The IP reassembly timer interval in milliseconds. */
pub const IP_TMR_INTERVAL: u32 = 1000;

#[derive(Clone,Debug,Default)]
pub struct Ipv4ReassemblyHelper {
    next_pbuf: Option<PacketBuffer>,
    start: isize,
    end: isize
}

impl Ipv4ReassemblyHelper {
    pub fn new() -> Self {
        Self {
            next_pbuf: None,
            start: -1,
            end: -1,
        }
    }
}

#[derive(Clone,Debug,Default)]
pub struct Ipv4ReassemblyData {
    pub pkt: Option<PacketBuffer>,
    pub ip4_hdr: Option<Ipv4Header>,
    pub datagram_len: isize,
    pub timer: i16,
    pub flags: i16,
}

impl Ipv4ReassemblyData {
    pub fn new() -> Self {
        Struct {
            pkt: None,
            ip4_hdr: None,
            datagram_len: -1,
            timer: -1,
            flags: -1,
        }
    }
}

pub fn ipv4_addresses_and_id_match(hdr1: &Ipv4Header, hdr2: &Ipv4Header) -> bool {
    (hdr1.src_addr == hdr2.src_addr) && (hdr1.dst_addr == hdr2.dst_addr) && (hdr1.id == hdr2.id)
}


/// Reassembly timer base function; Should be called every 1000 ms
pub fn ipv4_reassembly_base_timer(reassdatagrams: &mut Vec<Ipv4ReassemblyData>)
{
    let mut prev_rdgram : &mut Ipv4ReassemblyData;
    let mut dgrams_to_free: Vec<&mut Ipv4ReassemblyData> = Vec::new();
    for mut rdgram in reassdatagrams {
        if rdgram.timer > 0 {
            rdgram.timer -= 1;
            prev_rdgram = rdgram;
        } else {
            dgrams_to_free.push(rdgram);
        }
    }

    for rdgram in dgrams_to_free {
        let idx = reassdatagrams.iter().position(|&d| d == rdgram);
        match idx {
            Some(val) => reassdatagrams.remove(val),
            None() => {}
        }
    }
}

pub fn ipv4_free_oldest_reass_dgram(reassdatagrams: &mut Vec<Ipv4ReassemblyData>, ip4_hdr: &Ipv4Header) -> Result<(), LwipError> {
    let mut oldest_dgram: Option<&mut Ipv4ReassemblyData> = None;
    for dgram in reassdatagrams {
        if dgram.ip4_hdr == ip4_hdr {
            if oldest_dgram.is_none() {
                oldest_dgram = Some(dgram);
            } else {
                if dgram.timer > oldest_dgram.timer {
                    oldest_dgram = Some(dgram);
                }
            }
        }
    }

    if oldest_dgram.is_some() {
        let idx = reassdatagrams.iter().position(|&x| x == oldest_dgram.some());
        if idx.is_some() {
            reassdatagrams.remove(idx.some());
        }
    }

    Ok(())
}

/// Enqueues a new fragment into the fragment queue.
pub fn ipv4_enqueue_reass_dgram(rdgrams: &mut Vec<Ipv4ReassemblyData>, ip4_hdr: &Ipv4Header) -> Result<(), LwipError> {
    let mut rdata = Ipv4ReassemblyData::new();
    rdata.ip4_hdr = Some(ip4_hdr.clone());
    rdata.timer = IP_REASS_MAXAGE;
    rdgrams.push(rdata);
    Ok(())
}

/// Deqeueus a datagram form the fragment queue
pub fn ipv4_dequeue_reass_dgram(rdgrams: &mut Vec<Ipv4ReassemblyData>) -> Result<Ipv4ReassemblyData, LwipError> {
    return match rdgrams.pop() {
        Some(val) => Ok(val),
        Err(_) => Err(LwipError::new(LwipErrorCode::InvalidData, "failed to pop reassembly datagram from queue"))
    }
}

/// /**
///   * Chain a new pbuf into the pbuf list that composes the datagram.  The pbuf list
///   * will grow over time as  new pbufs are rx.
///   * Also checks that the datagram passes basic continuity checks (if the last
///   * fragment was received at least once).
///   * @param ipr points to the reassembly state
///   * @param new_p points to the pbuf for the current fragment
///   * @param is_last is 1 if this pbuf has MF==0 ( ipr.flags not updated yet)
///   * @return see IP_REASS_VALIDATE_* defines
///   */
pub fn ipv4_reass_chain_frag_into_datagram_and_validate(ipr: &mut Vec<Ipv4ReassemblyData>, new_p: &mut PacketBuffer, is_last: bool) -> Result<(), LwipError>
{
    let mut iprh: &mut Ipv4ReassemblyHelper;
    let mut iprh_tmp: &mut Ipv4ReassemblyHelper;
    let mut iprh_prev: &mut Ipv4ReassemblyHelper;
    let mut q: &mut PacketBuffer;
    let mut len: isize;
    let mut offset: isize;
    let mut hlen: u8;
    let mut frag_hdr: &mut Ipv4Header;
    let mut valid = false;

  /* Extract length and fragment offset from current fragment */
  fraghdr = (struct ip_hdr *) new_p.payload;
  len = lwip_ntohs(IPH_LEN(fraghdr));
  hlen = IPH_HL_BYTES(fraghdr);
  if (hlen > len) {
    /* invalid datagram */
    return IP_REASS_VALIDATE_PBUF_DROPPED;
  }
  len = (u16_t)(len - hlen);
  offset = IPH_OFFSET_BYTES(fraghdr);

  /* overwrite the fragment's ip header from the pbuf with our helper struct,
///     * and setup the embedded helper structure. */
  /* make sure the struct ip_reass_helper fits into the IP header */
  // LWIP_ASSERT("sizeof(struct ip_reass_helper) <= IP_HLEN",
              sizeof(struct ip_reass_helper) <= IP_HLEN);
  iprh = (struct ip_reass_helper *) new_p.payload;
   iprh.next_pbuf = NULL;
   iprh.start = offset;
   iprh.end = (u16_t)(offset + len);
  if ( iprh.end < offset) {
    /* u16_t overflow, cannot handle this */
    return IP_REASS_VALIDATE_PBUF_DROPPED;
  }

  /* Iterate through until we either get to the end of the list (append),
///     * or we find one with a larger offset (insert). */
  for (q =  ipr.p; q != NULL;) {
    iprh_tmp = (struct ip_reass_helper *) q.payload;
    if ( iprh.start <  iprh_tmp.start) {
      /* the new pbuf should be inserted before this */
       iprh.next_pbuf = q;
      if (iprh_prev != NULL) {
        /* not the fragment with the lowest offset */
// #if IP_REASS_CHECK_OVERLAP
        if (( iprh.start <  iprh_prev.end) || ( iprh.end >  iprh_tmp.start)) {
          /* fragment overlaps with previous or following, throw away */
          return IP_REASS_VALIDATE_PBUF_DROPPED;
        }
// #endif /* IP_REASS_CHECK_OVERLAP */
         iprh_prev.next_pbuf = new_p;
        if ( iprh_prev.end !=  iprh.start) {
          /* There is a fragment missing between the current
///             * and the previous fragment */
          valid = 0;
        }
      } else {
// #if IP_REASS_CHECK_OVERLAP
        if ( iprh.end >  iprh_tmp.start) {
          /* fragment overlaps with following, throw away */
          return IP_REASS_VALIDATE_PBUF_DROPPED;
        }
// #endif /* IP_REASS_CHECK_OVERLAP */
        /* fragment with the lowest offset */
         ipr.p = new_p;
      }
      break;
    } else if ( iprh.start ==  iprh_tmp.start) {
      /* received the same datagram twice: no need to keep the datagram */
      return IP_REASS_VALIDATE_PBUF_DROPPED;
// #if IP_REASS_CHECK_OVERLAP
    } else if ( iprh.start <  iprh_tmp.end) {
      /* overlap: no need to keep the new datagram */
      return IP_REASS_VALIDATE_PBUF_DROPPED;
// #endif /* IP_REASS_CHECK_OVERLAP */
    } else {
      /* Check if the fragments received so far have no holes. */
      if (iprh_prev != NULL) {
        if ( iprh_prev.end !=  iprh_tmp.start) {
          /* There is a fragment missing between the current
///             * and the previous fragment */
          valid = 0;
        }
      }
    }
    q =  iprh_tmp.next_pbuf;
    iprh_prev = iprh_tmp;
  }

  /* If q is NULL, then we made it to the end of the list. Determine what to do now */
  if (q == NULL) {
    if (iprh_prev != NULL) {
      /* this is (for now), the fragment with the highest offset:
///         * chain it to the last fragment */
// #if IP_REASS_CHECK_OVERLAP
      // LWIP_ASSERT("check fragments don't overlap",  iprh_prev.end <=  iprh.start);
// #endif /* IP_REASS_CHECK_OVERLAP */
       iprh_prev.next_pbuf = new_p;
      if ( iprh_prev.end !=  iprh.start) {
        valid = 0;
      }
    } else {
// #if IP_REASS_CHECK_OVERLAP
      // LWIP_ASSERT("no previous fragment, this must be the first fragment!",
                   ipr.p == NULL);
// #endif /* IP_REASS_CHECK_OVERLAP */
      /* this is the first fragment we ever received for this ip datagram */
       ipr.p = new_p;
    }
  }

  /* At this point, the validation part begins: */
  /* If we already received the last fragment */
  if (is_last || (( ipr.flags & IP_REASS_FLAG_LASTFRAG) != 0)) {
    /* and had no holes so far */
    if (valid) {
      /* then check if the rest of the fragments is here */
      /* Check if the queue starts with the first datagram */
      if (( ipr.p == NULL) || (((struct ip_reass_helper *) ipr.p->payload)->start != 0)) {
        valid = 0;
      } else {
        /* and check that there are no holes after this datagram */
        iprh_prev = iprh;
        q =  iprh.next_pbuf;
        while (q != NULL) {
          iprh = (struct ip_reass_helper *) q.payload;
          if ( iprh_prev.end !=  iprh.start) {
            valid = 0;
            break;
          }
          iprh_prev = iprh;
          q =  iprh.next_pbuf;
        }
        /* if still valid, all fragments are received
///           * (because to the MF==0 already arrived */
        if (valid) {
          // LWIP_ASSERT("sanity check",  ipr.p != NULL);
          // LWIP_ASSERT("sanity check",
                      ((struct ip_reass_helper *) ipr.p->payload) != iprh);
          // LWIP_ASSERT("validate_datagram:next_pbuf!=NULL",
                       iprh.next_pbuf == NULL);
        }
      }
    }
    /* If valid is 0 here, there are some fragments missing in the middle
///       * (since MF == 0 has already arrived). Such datagrams simply time out if
///       * no more fragments are received... */
    return valid ? IP_REASS_VALIDATE_TELEGRAM_FINISHED : IP_REASS_VALIDATE_PBUF_QUEUED;
  }
  /* If we come here, not all fragments were received, yet! */
  return IP_REASS_VALIDATE_PBUF_QUEUED; /* not yet valid! */
}

/// /**
///   * Reassembles incoming IP fragments into an IP datagram.
///   *
///   * @param p points to a pbuf chain of the fragment
///   * @return NULL if reassembly is incomplete, ? otherwise
///   */
struct pbuf *
ip4_reass(struct pbuf *p)
{
  struct pbuf *r;
  struct ip_hdr *fraghdr;
  struct ip_reassdata *ipr;
  struct ip_reass_helper *iprh;
  u16_t offset, len, clen;
  hlen: u8;
  int valid;
  int is_last;

  IPFRAG_STATS_INC(ip_frag.recv);
  MIB2_STATS_INC(mib2.ipreasmreqds);

  fraghdr = (struct ip_hdr *) p.payload;

  if (IPH_HL_BYTES(fraghdr) != IP_HLEN) {
    LWIP_DEBUGF(IP_REASS_DEBUG, ("ip4_reass: IP options currently not supported!\n"));
    IPFRAG_STATS_INC(ip_frag.err);
    goto nullreturn;
  }

  offset = IPH_OFFSET_BYTES(fraghdr);
  len = lwip_ntohs(IPH_LEN(fraghdr));
  hlen = IPH_HL_BYTES(fraghdr);
  if (hlen > len) {
    /* invalid datagram */
    goto nullreturn;
  }
  len = (u16_t)(len - hlen);

  /* Check if we are allowed to enqueue more datagrams. */
  clen = pbuf_clen(p);
  if ((ip_reass_pbufcount + clen) > IP_REASS_MAX_PBUFS) {
// #if IP_REASS_FREE_OLDEST
    if (!ip_reass_remove_oldest_datagram(fraghdr, clen) ||
        ((ip_reass_pbufcount + clen) > IP_REASS_MAX_PBUFS))
// #endif /* IP_REASS_FREE_OLDEST */
    {
      /* No datagram could be freed and still too many pbufs enqueued */
      LWIP_DEBUGF(IP_REASS_DEBUG, ("ip4_reass: Overflow condition: pbufct=%d, clen=%d, MAX=%d\n",
                                   ip_reass_pbufcount, clen, IP_REASS_MAX_PBUFS));
      IPFRAG_STATS_INC(ip_frag.memerr);
      /* @todo: send ICMP time exceeded here? */
      /* drop this pbuf */
      goto nullreturn;
    }
  }

  /* Look for the datagram the fragment belongs to in the current datagram queue,
///     * remembering the previous in the queue for later dequeueing. */
  for (ipr = reassdatagrams; ipr != NULL; ipr =  ipr.next) {
    /* Check if the incoming fragment matches the one currently present
       in the reassembly buffer. If so, we proceed with copying the
       fragment into the buffer. */
    if (IP_ADDRESSES_AND_ID_MATCH(& ipr.iphdr, fraghdr)) {
      LWIP_DEBUGF(IP_REASS_DEBUG, ("ip4_reass: matching previous fragment ID=%"X16_F"\n",
                                   lwip_ntohs(IPH_ID(fraghdr))));
      IPFRAG_STATS_INC(ip_frag.cachehit);
      break;
    }
  }

  if (ipr == NULL) {
    /* Enqueue a new datagram into the datagram queue */
    ipr = ip_reass_enqueue_new_datagram(fraghdr, clen);
    /* Bail if unable to enqueue */
    if (ipr == NULL) {
      goto nullreturn;
    }
  } else {
    if (((lwip_ntohs(IPH_OFFSET(fraghdr)) & IP_OFFMASK) == 0) &&
        ((lwip_ntohs(IPH_OFFSET(& ipr.iphdr)) & IP_OFFMASK) != 0)) {
      /*  ipr.iphdr is not the header from the first fragment, but fraghdr is
///         * -> copy fraghdr into  ipr.iphdr since we want to have the header
///         * of the first fragment (for ICMP time exceeded and later, for copying
///         * all options, if supported)*/
      SMEMCPY(& ipr.iphdr, fraghdr, IP_HLEN);
    }
  }

  /* At this point, we have either created a new entry or pointing
///     * to an existing one */

  /* check for 'no more fragments', and update queue entry*/
  is_last = (IPH_OFFSET(fraghdr) & PP_NTOHS(IP_MF)) == 0;
  if (is_last) {
    u16_t datagram_len = (u16_t)(offset + len);
    if ((datagram_len < offset) || (datagram_len > (0xFFFF - IP_HLEN))) {
      /* u16_t overflow, cannot handle this */
      goto nullreturn_ipr;
    }
  }
  /* find the right place to insert this pbuf */
  /* @todo: trim pbufs if fragments are overlapping */
  valid = ip_reass_chain_frag_into_datagram_and_validate(ipr, p, is_last);
  if (valid == IP_REASS_VALIDATE_PBUF_DROPPED) {
    goto nullreturn_ipr;
  }
  /* if we come here, the pbuf has been enqueued */

  /* Track the current number of pbufs current 'in-flight', in order to limit
     the number of fragments that may be enqueued at any one time
     (overflow checked by testing against IP_REASS_MAX_PBUFS) */
  ip_reass_pbufcount = (u16_t)(ip_reass_pbufcount + clen);
  if (is_last) {
    u16_t datagram_len = (u16_t)(offset + len);
     ipr.datagram_len = datagram_len;
     ipr.flags |= IP_REASS_FLAG_LASTFRAG;
    LWIP_DEBUGF(IP_REASS_DEBUG,
                ("ip4_reass: last fragment seen, total len %"S16_F"\n",
                  ipr.datagram_len));
  }

  if (valid == IP_REASS_VALIDATE_TELEGRAM_FINISHED) {
    struct ip_reassdata *ipr_prev;
    /* the totally last fragment (flag more fragments = 0) was received at least
///       * once AND all fragments are received */
    u16_t datagram_len = (u16_t)( ipr.datagram_len + IP_HLEN);

    /* save the second pbuf before copying the header over the pointer */
    r = ((struct ip_reass_helper *) ipr.p->payload)->next_pbuf;

    /* copy the original ip header back to the first pbuf */
    fraghdr = (struct ip_hdr *)( ipr.p->payload);
    SMEMCPY(fraghdr, & ipr.iphdr, IP_HLEN);
    IPH_LEN_SET(fraghdr, lwip_htons(datagram_len));
    IPH_OFFSET_SET(fraghdr, 0);
    IPH_CHKSUM_SET(fraghdr, 0);
    /* @todo: do we need to set/calculate the correct checksum? */
// #if CHECKSUM_GEN_IP
    IF__NETIF_CHECKSUM_ENABLED(ip_current_input_netif(), NETIF_CHECKSUM_GEN_IP) {
      IPH_CHKSUM_SET(fraghdr, inet_chksum(fraghdr, IP_HLEN));
    }
// #endif /* CHECKSUM_GEN_IP */

    p =  ipr.p;

    /* chain together the pbufs contained within the reass_data list. */
    while (r != NULL) {
      iprh = (struct ip_reass_helper *) r.payload;

      /* hide the ip header for every succeeding fragment */
      pbuf_remove_header(r, IP_HLEN);
      pbuf_cat(p, r);
      r =  iprh.next_pbuf;
    }

    /* find the previous entry in the linked list */
    if (ipr == reassdatagrams) {
      ipr_prev = NULL;
    } else {
      for (ipr_prev = reassdatagrams; ipr_prev != NULL; ipr_prev =  ipr_prev.next) {
        if ( ipr_prev.next == ipr) {
          break;
        }
      }
    }

    /* release the sources allocate for the fragment queue entry */
    ip_reass_dequeue_datagram(ipr, ipr_prev);

    /* and adjust the number of pbufs currently queued for reassembly. */
    clen = pbuf_clen(p);
    // LWIP_ASSERT("ip_reass_pbufcount >= clen", ip_reass_pbufcount >= clen);
    ip_reass_pbufcount = (u16_t)(ip_reass_pbufcount - clen);

    MIB2_STATS_INC(mib2.ipreasmoks);

    /* Return the pbuf chain */
    return p;
  }
  /* the datagram is not (yet?) reassembled completely */
  LWIP_DEBUGF(IP_REASS_DEBUG, ("ip_reass_pbufcount: %d out\n", ip_reass_pbufcount));
  return NULL;

nullreturn_ipr:
  // LWIP_ASSERT("ipr != NULL", ipr != NULL);
  if ( ipr.p == NULL) {
    /* dropped pbuf after creating a new datagram entry: remove the entry, too */
    // LWIP_ASSERT("not firstalthough just enqueued", ipr == reassdatagrams);
    ip_reass_dequeue_datagram(ipr, NULL);
  }

nullreturn:
  LWIP_DEBUGF(IP_REASS_DEBUG, ("ip4_reass: nullreturn\n"));
  IPFRAG_STATS_INC(ip_frag.drop);
  pbuf_free(p);
  return NULL;
}

/// /**
///   * Fragment an IP datagram if too large for the netif.
///   *
///   * Chop the datagram in MTU sized chunks and send them in order
///   * by pointing PBUF_REFs into p.
///   *
///   * @param p ip packet to send
///   * @param netif the netif on which to send
///   * @param dest destination ip address to which to send
///   *
///   * @return ERR_OK if sent successfully, err_t otherwise
///   */
err_t
ip4_frag(struct pbuf *p, struct netif *netif, const ip4_addr_t *dest)
{
  struct pbuf *rambuf;
#if !LWIP_NETIF_TX_SINGLE_PBUF
  struct pbuf *newpbuf;
  u16_t newpbuflen = 0;
  left_to_copy: u16;
// #endif
  struct ip_hdr *original_iphdr;
  struct ip_hdr *iphdr;
  const u16_t nfb = (u16_t)(( netif.mtu - IP_HLEN) / 8);
  u16_t left, fragsize;
  ofo: u16;
  int last;
  u16_t poff = IP_HLEN;
  tmp: u16;
  int mf_set;

  original_iphdr = (struct ip_hdr *) p.payload;
  iphdr = original_iphdr;
  if (IPH_HL_BYTES(iphdr) != IP_HLEN) {
    /* ip4_frag() does not support IP options */
    return ERR_VAL;
  }
  LWIP_ERROR("ip4_frag(): pbuf too short",  p.len >= IP_HLEN, return ERR_VAL);

  /* Save original offset */
  tmp = lwip_ntohs(IPH_OFFSET(iphdr));
  ofo = tmp & IP_OFFMASK;
  /* already fragmented? if so, the last fragment we create must have MF, too */
  mf_set = tmp & IP_MF;

  left = (u16_t)( p.tot_len - IP_HLEN);

  while (left) {
    /* Fill this fragment */
    fragsize = LWIP_MIN(left, (u16_t)(nfb * 8));

// #if LWIP_NETIF_TX_SINGLE_PBUF
    rambuf = pbuf_alloc(PBUF_IP, fragsize, PBUF_RAM);
    if (rambuf == NULL) {
      goto memerr;
    }
    // LWIP_ASSERT("this needs a pbuf in one piece!",
                ( rambuf.len ==  rambuf.tot_len) && ( rambuf.next == NULL));
    poff += pbuf_copy_partial(p,  rambuf.payload, fragsize, poff);
    /* make room for the IP header */
    if (pbuf_add_header(rambuf, IP_HLEN)) {
      pbuf_free(rambuf);
      goto memerr;
    }
    /* fill in the IP header */
    SMEMCPY( rambuf.payload, original_iphdr, IP_HLEN);
    iphdr = (struct ip_hdr *) rambuf.payload;
    /* When not using a static buffer, create a chain of pbufs.
///       * The first will be a PBUF_RAM holding the link and IP header.
///       * The rest will be PBUF_REFs mirroring the pbuf chain to be fragged,
///       * but limited to the size of an mtu.
///       */
    rambuf = pbuf_alloc(PBUF_LINK, IP_HLEN, PBUF_RAM);
    if (rambuf == NULL) {
      goto memerr;
    }
    // LWIP_ASSERT("this needs a pbuf in one piece!",
                ( rambuf.len >= (IP_HLEN)));
    SMEMCPY( rambuf.payload, original_iphdr, IP_HLEN);
    iphdr = (struct ip_hdr *) rambuf.payload;

    left_to_copy = fragsize;
    while (left_to_copy) {
      struct pbuf_custom_ref *pcr;
      u16_t plen = (u16_t)( p.len - poff);
      // LWIP_ASSERT(" p.len >= poff",  p.len >= poff);
      newpbuflen = LWIP_MIN(left_to_copy, plen);
      /* Is this pbuf already empty? */
      if (!newpbuflen) {
        poff = 0;
        p =  p.next;
        continue;
      }
      pcr = ip_frag_alloc_pbuf_custom_ref();
      if (pcr == NULL) {
        pbuf_free(rambuf);
        goto memerr;
      }
      /* Mirror this pbuf, although we might not need all of it. */
      newpbuf = pbuf_alloced_custom(PBUF_RAW, newpbuflen, PBUF_REF, & pcr.pc,
                                    (u8_t *) p.payload + poff, newpbuflen);
      if (newpbuf == NULL) {
        ip_frag_free_pbuf_custom_ref(pcr);
        pbuf_free(rambuf);
        goto memerr;
      }
      pbuf_ref(p);
       pcr.original = p;
       pcr.pc.custom_free_function = ipfrag_free_pbuf_custom;

      /* Add it to end of rambuf's chain, but using pbuf_cat, not pbuf_chain
///         * so that it is removed when pbuf_dechain is later called on rambuf.
///         */
      pbuf_cat(rambuf, newpbuf);
      left_to_copy = (u16_t)(left_to_copy - newpbuflen);
      if (left_to_copy) {
        poff = 0;
        p =  p.next;
      }
    }
    poff = (u16_t)(poff + newpbuflen);

    /* Correct header */
    last = (left <=  netif.mtu - IP_HLEN);

    /* Set new offset and MF flag */
    tmp = (IP_OFFMASK & (ofo));
    if (!last || mf_set) {
      /* the last fragment has MF set if the input frame had it */
      tmp = tmp | IP_MF;
    }
    IPH_OFFSET_SET(iphdr, lwip_htons(tmp));
    IPH_LEN_SET(iphdr, lwip_htons((u16_t)(fragsize + IP_HLEN)));
    IPH_CHKSUM_SET(iphdr, 0);

    IF__NETIF_CHECKSUM_ENABLED(netif, NETIF_CHECKSUM_GEN_IP) {
      IPH_CHKSUM_SET(iphdr, inet_chksum(iphdr, IP_HLEN));
    }

    /* No need for separate header pbuf - we allowed room for it in rambuf
///       * when allocated.
///       */
     netif.output(netif, rambuf, dest);
    IPFRAG_STATS_INC(ip_frag.xmit);

    /* Unfortunately we can't reuse rambuf - the hardware may still be
///       * using the buffer. Instead we free it (and the ensuing chain) and
///       * recreate it next time round the loop. If we're lucky the hardware
///       * will have already sent the packet, the free will really free, and
///       * there will be zero memory penalty.
///       */

    pbuf_free(rambuf);
    left = (u16_t)(left - fragsize);
    ofo = (u16_t)(ofo + nfb);
  }
  MIB2_STATS_INC(mib2.ipfragoks);
  return ERR_OK;
memerr:
  MIB2_STATS_INC(mib2.ipfragfails);
  return ERR_MEM;
}
