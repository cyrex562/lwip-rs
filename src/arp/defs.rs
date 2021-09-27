use crate::arp::etharp_h::ip4_addr_wordaligned;
use crate::core::error::{ERR_ARG, ERR_MEM, ERR_RTE, ERR_VAL, LwipError};
use crate::core::defines::LwipAddr;
use crate::ethernet::ethernet_h::ETH_HWADDR_LEN;
use crate::ip::ip42::ip4_route;
use crate::ip::ip4_addr_h::{ip4_addr, ip4_addr_copy, ip4_addr_isany, ip4_addr_ismulticast, ip4_addr_set_zero, ip4_addr_cmp};
use crate::netif::netif_h::NetIfc;
use crate::packetbuffer::pbuf::pbuf_free;
use crate::packetbuffer::pbuf_h::PacketBuffer;
use crate::snmp::snmp2_h::mib2_remove_arp_entry;
use crate::context::LwipContext;
use crate::arp::defs::ArpState::{EtharpStateEmpty, EtharpStatePending, EtharpStateStatic, EtharpStateStable};
use crate::core::debug_h::LWIP_DBG_TRACE;

pub const ARP_AGE_REREQUEST_USED_UNICAST: u32 = (ARP_MAXAGE - 30);
pub const ARP_AGE_REREQUEST_USED_BROADCAST: u32 = (ARP_MAXAGE - 15);
pub const ARP_MAXPENDING: u32 = 5;

pub enum ArpState {
    EtharpStateEmpty = 0,
    EtharpStatePending,
    EtharpStateStable,
    EtharpStateStableRerequesting1,
    EtharpStateStableRerequesting2,
    EtharpStateStatic,
}

#[derive(Clone, Debug, Default)]
pub struct ArpEntry {
    pub pkt_q: Vec<PacketBuffer>,
    pub ip_addr: LwipAddr,
    pub net_ifc: NetIfc,
    pub eth_addr: LwipAddr,
    pub ctime: u64,
    pub state: ArpState,
}

impl ArpEntry {
    pub fn new() -> ArpEntry {
        ArpEntry {
            ..Default::default()
        }
    }
}

pub const ETHARP_FLAG_TRY_HARD: u8 = 1;
pub const ETHARP_FLAG_FIND_ONLY: u8 = 2;
pub const ETHARP_FLAG_STATIC_ENTRY: u8 = 4;
pub const ARP_TMR_INTERVAL: u32 = 1000;


pub struct EtharpQEntry {
    // next: &mut EtharpQEntry;
    pkt_buf: PacketBuffer,
}

pub fn free_etharp_q(entries: &mut Vec<EtharpQEntry>) {
    entries.clear()
}

pub fn etharp_free_entry(arp_table: &mut Vec<ArpEntry>, i: i32) {
    /* remove from SNMP ARP index tree */
    // TODO: figure out why this isnt showign up properly
    // mib2_remove_arp_entry(&arp_table[i].net_ifc, &arp_table[i].ip_addr);
    /* and empty packet queue */
    arp_table[i].pkt_q.clear();
    /* recycle entry for re-use */
    arp_table[i].state = ETHARP_STATE_EMPTY;
    /* for debugging, clean out the complete entry */
    arp_table[i].ctime = 0;
    arp_table[i].netif = None;
    ip4_addr_set_zero(&mut arp_table[i].ip_addr);
    arp_table[i].ethaddr = ethzero;
}

pub fn etharp_find_entry(
    ctx: &mut LwipContext,
    ipaddr: &mut ip4_addr,
    flags: u8,
    netif: &mut NetIfc
) -> Result<i16, LwipError> {
    let old_pending: usize = ctx.options.arp_options.ARP_TABLE_SIZE;
    let old_stable: usize = ctx.options.arp_options.ARP_TABLE_SIZE;
    let mut empty: usize = ctx.options.arp_options.ARP_TABLE_SIZE;
    let i: i16 = 0;
    /* oldest entry with packets on queue */
    let old_queue = ctx.options.arp_options.ARP_TABLE_SIZE;
    /* its age */
    let age_queue: u16 = 0;
    let age_pending = 0;
    let age_stable = 0;
    /*
     * a) do a search through the cache, remember candidates
     * b) select candidate entry
     * c) create new entry
     */

    /* a) in a single search sweep, do all of this
     * 1) remember the first empty entry (if any)
     * 2) remember the oldest stable entry (if any)
     * 3) remember the oldest pending entry without queued packets (if any)
     * 4) remember the oldest pending entry with queued packets (if any)
     * 5) search for a matching IP entry, either pending or stable
     *    until 5 matches, or all entries are searched for.
     */

    for i in 0 .. ctx.options.arp_options.ARP_TABLE_SIZE {
      state: u8 = arp_table[i].state;
      /* no empty entry found yet and now we do find one? */
      if (empty == ARP_TABLE_SIZE) && (state == EtharpStateEmpty) {
        LWIP_DEBUGF(ETHARP_DEBUG, ("etharp_find_entry: found empty entry %d\n", i));
        /* remember first empty entry */
        empty = i;
      } else if state != EtharpStateEmpty {
        LWIP_ASSERT("state == EtharpStatePending || state >= EtharpStateStable",
                    state == EtharpStatePending || state >= EtharpStateStable);
        /* if given, does IP address match IP address in ARP entry? */
        if ipaddr && ip4_addr_cmp(ipaddr, &arp_table[i].ipaddr)

            && ((netif == NULL) || (netif == arp_table[i].netif)) {
          LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_find_entry: found matching entry %d\n", i));
          /* found exact IP address match, simply bail out */
          return i;
        }
        /* pending entry? */
        if (state == EtharpStatePending) {
          /* pending with queued packets? */
          if (arp_table[i].q != NULL) {
            if (arp_table[i].ctime >= age_queue) {
              old_queue = i;
              age_queue = arp_table[i].ctime;
            }
          } else
            /* pending without queued packets? */
          {
            if (arp_table[i].ctime >= age_pending) {
              old_pending = i;
              age_pending = arp_table[i].ctime;
            }
          }
          /* stable entry? */
        } else if (state >= EtharpStateStable) {

          /* don't record old_stable for static entries since they never expire */
          if (state < EtharpStateStatic)

          {
            /* remember entry with oldest stable entry in oldest, its age in maxtime */
            if (arp_table[i].ctime >= age_stable) {
              old_stable = i;
              age_stable = arp_table[i].ctime;
            }
          }
        }
      }
    }
    /* { we have no match } => try to create a new entry */

    /* don't create new entry, only search? */
    if ((flags & ETHARP_FLAG_FIND_ONLY) != 0) ||
      /* or no empty entry found and not allowed to recycle? */
      ((empty == ARP_TABLE_SIZE) && ((flags & ETHARP_FLAG_TRY_HARD) == 0))
    {
        /*LWIP_DEBUGF(
            ETHARP_DEBUG | LWIP_DBG_TRACE,
            ("etharp_find_entry: no empty entry found and not allowed to recycle\n"),
        );*/
        return Err(LwipError::new(ERR_MEM, ""));
    }

    /* b) choose the least destructive entry to recycle:
     * 1) empty entry
     * 2) oldest stable entry
     * 3) oldest pending entry without queued packets
     * 4) oldest pending entry with queued packets
     *
     * { ETHARP_FLAG_TRY_HARD is set at this point }
     */

    /* 1) empty entry available? */
    if empty < ARP_TABLE_SIZE {
        i = empty;
    /*LWIP_DEBUGF(
        ETHARP_DEBUG | LWIP_DBG_TRACE,
        ("etharp_find_entry: selecting empty entry %d\n", i),
    );*/
    } else {
        /* 2) found recyclable stable entry? */
        if old_stable < ARP_TABLE_SIZE {
            /* recycle oldest stable*/
            i = old_stable;
            /*LWIP_DEBUGF(
                ETHARP_DEBUG | LWIP_DBG_TRACE,
                ("etharp_find_entry: selecting oldest stable entry %d\n", i),
            );*/
            /* no queued packets should exist on stable entries */
            LWIP_ASSERT("arp_table[i].q == NULL", arp_table[i].q == None);
            /* 3) found recyclable pending entry without queued packets? */
        } else if old_pending < ARP_TABLE_SIZE {
            /* recycle oldest pending */
            i = old_pending;
        /*LWIP_DEBUGF(
            ETHARP_DEBUG | LWIP_DBG_TRACE,
            (
                "etharp_find_entry: selecting oldest pending entry %d (without queue)\n",
                i,
            ),
        );*/
        /* 4) found recyclable pending entry with queued packets? */
        } else if old_queue < ARP_TABLE_SIZE {
            /* recycle oldest pending (queued packets are free in etharp_free_entry) */
            i = old_queue;
        //            LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_find_entry: selecting oldest pending entry %d, freeing packet queue %p\n", i, (arp_table[i].q)));
        /* no empty or recyclable entries found */
        } else {
            /*LWIP_DEBUGF(
                ETHARP_DEBUG | LWIP_DBG_TRACE,
                ("etharp_find_entry: no empty or recyclable entries found\n"),
            );*/
            return ERR_MEM;
        }

        /* { empty or recyclable entry found } */
        LWIP_ASSERT("i < ARP_TABLE_SIZE", i < ARP_TABLE_SIZE);
        etharp_free_entry(i);
    }

    LWIP_ASSERT("i < ARP_TABLE_SIZE", i < ARP_TABLE_SIZE);
    LWIP_ASSERT(
        "arp_table[i].state == EtharpStateEmpty",
        arp_table[i].state == ETHARP_STATE_EMPTY,
    );

    /* IP address given? */
    if ipaddr != None {
        /* set IP address */
        ip4_addr_copy(arp_table[i].ipaddr, *ipaddr);
    }
    arp_table[i].ctime = 0;

    arp_table[i].netif = netif;

    return i;
}

pub fn etharp_update_arp_entry(
    netif: &mut NetIfc,
    ipaddr: &mut ip4_addr,
    ethaddr: &mut eth_addr,
    flags: u8,
) -> Result<(), LwipError> {
    let i: i16;
    // LWIP_ASSERT("netif.hwaddr_len == ETH_HWADDR_LEN", netif.hwaddr_len == ETH_HWADDR_LEN);
    // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_update_arp_entry: %"U16_F".%"U16_F".%"U16_F".%"U16_F" - %02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F"\n",
    // ip4_addr1_16(ipaddr), ip4_addr2_16(ipaddr), ip4_addr3_16(ipaddr), ip4_addr4_16(ipaddr),
    // ethaddr.addr[0], ethaddr.addr[1], ethaddr.addr[2],
    // ethaddr.addr[3], ethaddr.addr[4], ethaddr.addr[5]));
    /* non-unicast address? */
    if (ip4_addr_isany(ipaddr)
        || ip4_addr_isbroadcast(ipaddr, netif)
        || ip4_addr_ismulticast(ipaddr))
    {
        /*LWIP_DEBUGF(
            ETHARP_DEBUG | LWIP_DBG_TRACE,
            ("etharp_update_arp_entry: will not add non-unicast IP address to ARP cache\n"),
        );*/
        return ERR_ARG;
    }
    /* find or create ARP entry */
    i = etharp_find_entry(, ipaddr, flags, netif);
    /* bail out if no entry could be found */
    if (i < 0) {
        return i;
    }

    if (flags & ETHARP_FLAG_STATIC_ENTRY) {
        /* record static type */
        arp_table[i].state = ETHARP_STATE_STATIC;
    } else if (arp_table[i].state == ETHARP_STATE_STATIC) {
        /* found entry is a static type, don't overwrite it */
        return ERR_VAL;
    } else {
        /* mark it stable */
        arp_table[i].state = ETHARP_STATE_STABLE;
    }

    /* record network interface */
    arp_table[i].netif = netif;
    /* insert in SNMP ARP index tree */
    mib2_add_arp_entry(netif, &arp_table[i].ipaddr);

    // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_update_arp_entry: updating stable entry %"S16_F"\n", i));
    /* update address */
    SMEMCPY(&arp_table[i].ethaddr, ethaddr, ETH_HWADDR_LEN);
    /* reset time stamp */
    arp_table[i].ctime = 0;
    /* this is where we will send out queued packets! */

    while (arp_table[i].q != None) {
        let p: &mut PacketBuffer;
        /* remember remainder of queue */
        let q: &mut EtharpQEntry = arp_table[i].q;
        /* pop first item off the queue */
        arp_table[i].q = q.next;
        /* get the packet pointer */
        p = q.pkt_buf;
        /* now queue entry can be freed */
        memp_free(MEMP_ARP_QUEUE, q);
        /* arp_queueing */
        if (arp_table[i].q != None) {
            let p: &mut PacketBuffer = arp_table[i].q;
            arp_table[i].q = None;

            /* send the queued IP packet */
            ethernet_output(netif, p, (netif.hwaddr), ethaddr, ETHTYPE_IP);
            /* free the queued IP packet */
            pbuf_free(p);
        }
    }
    Ok(())
}

pub fn etharp_add_static_entry(ipaddr: &mut ip4_addr, ethaddr: &mut eth_addr) {
    let netif: &mut NetIfc;
    LWIP_ASSERT_CORE_LOCKED();
    // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_add_static_entry: %"U16_F".%"U16_F".%"U16_F".%"U16_F" - %02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F"\n",
    //             ip4_addr1_16(ipaddr), ip4_addr2_16(ipaddr), ip4_addr3_16(ipaddr), ip4_addr4_16(ipaddr),
    //             ethaddr.addr[0], ethaddr.addr[1], ethaddr.addr[2],
    //             ethaddr.addr[3], ethaddr.addr[4], ethaddr.addr[5]));

    netif = ip4_route(ipaddr);
    if (netif == None) {
        return ERR_RTE;
    }

    return etharp_update_arp_entry(
        netif,
        ipaddr,
        ethaddr,
        ETHARP_FLAG_TRY_HARD | ETHARP_FLAG_STATIC_ENTRY,
    );
}

pub fn etharp_remove_static_entry(ipaddr: &mut ip4_addr) {
    let i: i16;
    LWIP_ASSERT_CORE_LOCKED();
    // LWIP_DEBUGF(ETHARP_DEBUG | LWIP_DBG_TRACE, ("etharp_remove_static_entry: %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
    //             ip4_addr1_16(ipaddr), ip4_addr2_16(ipaddr), ip4_addr3_16(ipaddr), ip4_addr4_16(ipaddr)));

    /* find or create ARP entry */
    i = etharp_find_entry(, ipaddr, ETHARP_FLAG_FIND_ONLY, None);
    /* bail out if no entry could be found */
    if (i < 0) {
        return i;
    }

    if (arp_table[i].state != ETHARP_STATE_STATIC) {
        /* entry wasn't a static entry, cannot remove it */
        return ERR_ARG;
    }
    /* entry found, free it */
    etharp_free_entry(i);
   return Ok(());
}

pub fn etharp_cleanup_netif(netif: &mut NetIfc) {
    let i: i32;

    // for (i = 0; i < ARP_TABLE_SIZE; += 1i) {
    //   state: u8 = arp_table[i].state;
    //   if ((state != EtharpStateEmpty) && (arp_table[i].netif == netif)) {
    //     etharp_free_entry(i);
    //   }
    // }
}

pub fn etharp_find_addr(
    netif: &mut NetIfc,
    ipaddr: &mut ip4_addr,
    eth_ret: &mut eth_addr,
    ip_ret: ip4_addr,
) -> isize {
    let i: i16;

    LWIP_ASSERT(
        "eth_ret != NULL && ip_ret != NULL",
        eth_ret != None && ip_ret != None,
    );

    i = etharp_find_entry(, ipaddr, ETHARP_FLAG_FIND_ONLY, netif);
    if ((i >= 0) && (arp_table[i].state >= ETHARP_STATE_STABLE)) {
        *eth_ret = &arp_table[i].ethaddr;
        *ip_ret = &arp_table[i].ipaddr;
        return i;
    }
    return -1;
}

pub fn etharp_get_entry(i: usize, ipaddr: &mut ip4_addr, netif: netif, eth_ret: eth_addr) {
    LWIP_ASSERT("ipaddr != NULL", ipaddr != None);
    LWIP_ASSERT("netif != NULL", netif != None);
    LWIP_ASSERT("eth_ret != NULL", eth_ret != None);

    if ((i < ARP_TABLE_SIZE) && (arp_table[i].state >= ETHARP_STATE_STABLE)) {
        *ipaddr = &arp_table[i].ipaddr;
        *netif = arp_table[i].netif;
        *eth_ret = &arp_table[i].ethaddr;
        return 1;
    } else {
        return 0;
    }
}

pub const ETHARP_HWADDR_LEN: usize = ETH_HWADDR_LEN;

pub struct etharp_hdr {
    pub hwtype: u16,
    pub proto: u16,
    pub hwlen: u8,
    pub protolen: u8,
    pub opcode: u16,
    pub shwaddr: eth_addr,
    pub sipaddr: ip4_addr_wordaligned,
    pub dhwaddr: eth_addr,
    pub dipaddr: ip4_addr_wordaligned,
}

pub const SIZEOF_ETHARP_HDR: usize = 28;

pub enum etharp_opcode {
    ARP_REQUEST = 1,
    ARP_REPLY = 2,
}
