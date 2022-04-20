use std::net::Ipv4Addr;

use log::{debug, warn};
///
/// @file
///
/// ACD IPv4 Address Conflict Detection
///
/// This is an IPv4 address conflict detection implementation for the lwIP TCP/IP
/// stack. It aims to be conform to RFC5227.
///
/// @defgroup acd ACD
/// @ingroup ip4
/// ACD related functions
/// USAGE:
///
/// define @ref LWIP_ACD 1 in your lwipopts.h
/// Options:
/// ACD_TMR_INTERVAL msecs,
///   I recommend a value of 100. The value must divide 1000 with a remainder almost 0.
///   Possible values are 1000, 500, 333, 250, 200, 166, 142, 125, 111, 100 ....
///
/// For fixed IP:
/// - call acd_start after selecting an IP address. The caller will be informed
///   on conflict status via the callback function.
///
/// With AUTOIP:
/// - will be called from the autoip module. No extra's needed.
///
/// With DHCP:
/// - enable LWIP_DHCP_DOES_ACD_CHECK. Then it will be called from the dhcp module.
///   No extra's needed.
////

/*
///
/// Copyright (c) 2007 Dominik Spies <kontakt@dspies.de>
/// Copyright (c) 2018 Jasper Verschueren <jasper.verschueren@apart-audio.com>
/// All rights reserved.
///
/// Redistribution and use in source and binary forms, with or without modification,
/// are permitted provided that the following conditions are met:
///
/// 1. Redistributions of source code must retain the above copyright notice,
///    this list of conditions and the following disclaimer.
/// 2. Redistributions in binary form must reproduce the above copyright notice,
///    this list of conditions and the following disclaimer in the documentation
///    and/or other materials provided with the distribution.
/// 3. The name of the author may not be used to endorse or promote products
///    derived from this software without specific prior written permission.
///
/// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
/// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
/// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
/// SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
/// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
/// OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
/// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
/// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
/// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
/// OF SUCH DAMAGE.
///
/// Author: Jasper Verschueren <jasper.verschueren@apart-audio.com>
/// Author: Dominik Spies <kontakt@dspies.de>
////

use crate::mac_address::MacAddress;
use crate::utils::LWIP_RAND;
use rnd::Rng;
use crate::acd::AcdState::ProbeWait;
use crate::errors::{LwipError, LwipErrorCode};
use crate::ip_address::IpAddress;
use crate::network_interface::NetworkInterface;

/* don't build if not configured for use in lwipopts.h */



/* RFC 5227 and RFC 3927 Constants */
pub const PROBE_WAIT: usize = 1; /* second  (initial random delay)                    */
pub const PROBE_MIN: usize = 1; /* second  (minimum delay till repeated probe)       */
pub const PROBE_MAX: usize = 2; /* seconds (maximum delay till repeated probe)       */
pub const PROBE_NUM: usize = 3; /*         (number of probe packets)                 */
pub const ANNOUNCE_NUM: usize = 2; /*         (number of announcement packets)          */
pub const ANNOUNCE_INTERVAL: usize = 2; /* seconds (time between announcement packets)       */
pub const ANNOUNCE_WAIT: usize = 2; /* seconds (delay before announcing)                 */
pub const MAX_CONFLICTS: usize = 10; /*         (max conflicts before rate limiting)      */
pub const RATE_LIMIT_INTERVAL: usize = 60; /* seconds (delay between successive attempts)       */
pub const DEFEND_INTERVAL: usize = 10; /* seconds (minimum interval between defensive ARPs) */

/* ACD states */
#[derive(Debug,Clone)]
pub enum AcdState {
  /* ACD is module is off */
    Off,
  /* Waiting before probing can be started */
    ProbeWait,
  /* Probing the ipaddr */
    Probing,
  /* Waiting before announcing the probed ipaddr */
    AnnounceWait,
  /* Announcing the new ipaddr */
    Announcing,
  /* Performing ongoing conflict detection with one defend within defend inferval */
    Ongoing,
  /* Performing ongoing conflict detection but immediately back off and Release
   * the address when a conflict occurs. This state is used for LL addresses
   * that stay active even if the netif has a routable address selected.
   * In such a case, we cannot defend our address */
    PassiveOngoing,
  /* To many conflicts occurred, we need to wait before restarting the selection
   * process */
    RateLimit
}

pub enum AcdCallbackResult {
    AcdIpOk,            /* IP address is good, no conflicts found in checking state */
    AcdRestartClient,   /* Conflict found -> the client should try again */
    AcdDecline           /* Decline the received IP address (rate limiting)*/
}



// /**
//  * Callback function: Handle conflict information from ACD module
//  *
//  * @param netif   network interface to handle conflict information on
//  * @param state   acd_callback_enum_t
//  */
// typedef void (*acd_conflict_callback_t)(netif: &mut NetworkInterface, acd_callback_enum_t state);
type acd_conflict_callback = fn(netif: &NetworkInterface, state: AcdCallbackResult);

/// ACD state information per netif */
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AcdStateInfo {
    /** next acd module */
    // struct acd *next;
    next_id: u32,
    /** the currently selected, probed, announced or used IP-Address */
    ipaddr: IpAddress,
    /** current ACD state machine state */
    state: AcdState,
    /** sent number of probes or announces, dependent on state */
    sent_num: usize,
    /** ticks to wait, tick is ACD_TMR_INTERVAL long */
    ticks_to_wait: u32,
    /** ticks until a conflict can again be solved by defending */
    lastconflict: u8,
    /** total number of probed/used IP-Addresses that resulted in a conflict */
    num_conflicts: usize,
    /** callback function -> let's the acd user know if the address is good or
         if a conflict is detected */
    callback: acd_conflict_callback,
}

impl AcdStateInfo {
    pub fn new() -> Self {
        Default::default()
    }
}


// ACD Timing
// ACD_TMR_INTERVAL msecs, I recommend a value of 100.
// The value must divide 1000 with a remainder almost 0. Possible values are
// 1000, 500, 333, 250, 200, 166, 142, 125, 111, 100 ....
//
pub const ACD_TMR_INTERVAL: u32 = 100; 




// #define ACD_FOREACH(acd, acd_list) for ((acd) = acd_list; (acd) != NULL; (acd) = (acd)->next)

pub const ACD_TICKS_PER_SECOND: u32 =  (1000 / ACD_TMR_INTERVAL);

// pub fn LWIP_ACD_RAND(netif: NetworkInterface, acd: Acd) -> u32 {
//     utils::LWIP_RAND()
// }




// #define ACD_RANDOM_PROBE_WAIT(netif, acd) (LWIP_ACD_RAND(netif, acd) % \
//                                     (PROBE_WAIT * ACD_TICKS_PER_SECOND))
pub fn ACD_RANDOM_PROBE_WAIT(netif: &NetworkInterface, acd: Acd) -> u32 {
    LWIP_RAND() % (PROBE_WAIT * ACD_TICKS_PER_SECOND)
}


// #define ACD_RANDOM_PROBE_INTERVAL(netif, acd) ((LWIP_ACD_RAND(netif, acd) % \
//                                     ((PROBE_MAX - PROBE_MIN) * ACD_TICKS_PER_SECOND)) + \
//                                     (PROBE_MIN * ACD_TICKS_PER_SECOND ))
pub fn ACD_RANDOM_PROBE_INTERVAL(netif: &NetworkInterface, acd: ACD) -> u32 {
    LWIP_RAND() &  (((PROBE_MAX - PROBE_MIN) * ACD_TICKS_PER_SECOND) + (PROBE_MIN * ACD_TICKS_PER_SECOND ))
}

/* Function definitions */
// static void acd_restart(netif: &mut NetworkInterface, acd: &mut AcdStateInfo);
// static void acd_handle_arp_conflict(netif: &mut NetworkInterface, acd: &mut AcdStateInfo);
// static void acd_put_in_passive_mode(netif: &mut NetworkInterface, acd: &mut AcdStateInfo);


pub fn acd_add_to_list(netif: &mut NetworkInterface, acd: &mut AcdContext,
                       callback: acd_conflict_callback) -> Result<(), LwipError> {
    let acd2: AcdStateInfo = AcdStateInfo::new();
    /* Set callback */
    // LWIP_ASSERT_CORE_LOCKED()
    // LWIP_ASSERT("acd_conflict_callback != NULL", acd_conflict_callback != NULL);
    acd.acd_conflict_callback = callback;

    /* Check if the acd struct is already added */
    for acd2 in netif.acd_list {
        if acd2 == acd {
            debug!("acd already added to list");
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "acd already exists in list"));
        }
    }

    return ERR_OK;
}


pub fn acd_remove_from_list(netif: &mut NetworkInterface, acd: &mut AcdStateInfo) -> Result<(), LwipError> {
    let mut item_in_list = false;
    let mut idx = 0usize;
    for acd2 in netif.acd_list {
        if acd2 == acd {
            item_in_list = true;
            break;
        }
        idx += 1;
    }
    if item_in_list {
        netif.acd_list.remove(idx);
        Ok(())
    }
    Err(LwipError::new(LwipErrorCode::InvalidArgument, "acd not in netif acd list"))
}

pub fn acd_start_client(netif: &mut NetworkInterface, acd: &mut AcdStateInfo, addr: &IpAddress) -> Result<(), LwipError> {
    acd.sent_num = 0;
    acd.lastconflict = 0;
    acd.ipaddr = addr.clone();
    acd.state = AcdState::ProbeWait;
    acd.ticks_to_wait = ACD_RANDOM_PROBE_WAIT(netif, acd);

    Ok(())
}


pub fn acd_stop_client(acd: &mut AcdStateInfo) -> Result<(), LwipError> {
    acd.state = AcdState::Off;
    Ok(())
}


pub fn acd_network_changed_link_down(netif: &mut NetworkInterface)
{
    for acd in netif.acd_list.iter_mut() {
        acd_stop_client(acd);
    }

}

///
/// Has to be called in loop every ACD_TMR_INTERVAL milliseconds
////
pub fn acd_tmr(netif_list: &mut Vec<NetworkInterface>) {
    for netif in netif_list.iter_mut() {
        for acd in netif.acd_list.iter_mut() {
            if acd.lastconflict > 0 {
                acd.lastconflict -= 1;
            }

            debug!("acd state: {:?}, ttw: {}", &acd.state, &acd.ticks_to_wait);
            if acd.ttw > 0 {
                acd.ttw -= 1;
            }

            match &acd.state {
                AcdState::ProbeWait => {},
                AcdState::Probing => {
                    if acd.ttw == 0 {
                        acd.state = AcdState::Probing;
                        etharp_acd_probe(netif, &acd.ipaddr);
                        debug!("PROBING sent probe");
                        acd.sent_num += 1;

                        if acd.sent_num >= PROBE_NUM {
                            acd.state = AcdState::AnnounceWait;
                            acd.sent_num = 0;
                            acd.ttw = (ANNOUNCE_WAIT * ACD_TICKS_PER_SECOND)
                        } else {
                            acd.ttw = ACD_RANDOM_PROBE_INTERVAL(netif, acd);
                        }
                    }
                },
                AcdState::AnnounceWait => {},
                AcdState::Announcing => {
                    if acd.ttw == 0 {
                        if acd.sent_num == 0 {
                            acd.state = AcdState::Announcing;
                            acd.num_conflicts = 0;
                            debug!("changing state to announcing");
                        }

                        etharp_acd_announce(netif, &acd, ipaddr);
                        debug!("sent announce");
                        acd.ttw = ANNOUNCE_INTERVAL * ACD_TICKS_PER_SECOND;
                        acd.sent_num += 1;

                        if acd.sent_num >= ANNOUNCE_NUM {
                            acd.state = AcdState::Ongoing;
                            acd.sent_num = 0;
                            acd.ttw = 0;
                            debug!("changing state to ongoing");
                            acd.acd_conflict_callback(netif, ACD_IP_OK);
                        }
                    }
                },
                AcdState::RateLimit => {
                    if acd.ttw == 0 {
                        acd_stop_client(acd);
                        acd.acd_conflict_callback(netif, ACD_RESTART_CLIENT);
                    }
                },
                _ => {}
            }
        }
    }
}

pub fn acd_restart(netif: &mut NetworkInterface, acd: &mut AcdStateInfo) {
    /* increase conflict counter. */
    acd.num_conflicts += 1;

    /* Decline the address */
    acd.acd_conflict_callback(netif, ACD_DECLINE);

    /* if we tried more then MAX_CONFLICTS we must limit our rate for
     * acquiring and probing addresses. compliant to RFC 5227 Section 2.1.1 */
    if acd.num_conflicts >= MAX_CONFLICTS {
        acd.state = ACD_STATE_RATE_LIMIT;
        acd.ttw = (RATE_LIMIT_INTERVAL * ACD_TICKS_PER_SECOND);
        debug!("rate limiting initiating. too many conflicts");
    } else {
        /* acd should be stopped because ipaddr isn't valid any more */
        acd_stop(acd);
        /* let the acd user know right away that their is a conflict detected.
         * So it can restart the address acquiring process. */
        acd.acd_conflict_callback(netif, ACD_RESTART_CLIENT);
    }
}

///
/// Handles every incoming ARP Packet, called by etharp_input().
///
/// @param netif network interface to use for acd processing
/// @param hdr   Incoming ARP packet
////
pub fn acd_arp_reply(netif: &mut NetworkInterface, hdr: &mut etharp_hdr)
{
  let mut acd: AcdStateInfo;
  let mut sipaddr: Ipv4Addr;
  let mut dipaddr: Ipv4Addr;
  let mut netifaddr: MacAddress;

  netifaddr.address = netif.mac_address;

  /* Copy struct ip4_addr_wordaligned to aligned ip4_addr, to support
   * compilers without structure packing (not using structure copy which
   * breaks strict-aliasing rules).
   */
    sipaddr = hdr.sipaddr.clone();
    dipaddr = hdr.dipaddr.clone();

  /* loop over the acd's*/
    for acd in netif.acd_list.iter_mut()  {
    match &acd.state {
        AcdState::Off => {},
        AcdState::RateLimit => {},
        AcdState::ProbeWait | AcdState::Probing | AcdState::AnnounceWait => {
            /* RFC 5227 Section 2.1.1:
             * from beginning to after ANNOUNCE_WAIT seconds we have a conflict if
             * ip.src == ipaddr (someone is already using the address)
             * OR
             * ip.dst == ipaddr && hw.src != own hwaddr (someone else is probing it)
             */
            if ((sipaddr == acd.ipaddr) || (sipaddr == IPV4_ADDRESS_ANY && dipaddr == acd.ipaddr && netifaddr != hdr.shwaddr)) {
              warn!("probe conflict detected");
                acd_restart(netif, acd);
            }
        }
        AcdState::Announcing | AcdState::Ongoing | AcdState::PassiveOngoing =>{
        /* RFC 5227 Section 2.4:
         * in any state we have a conflict if
         * ip.src == ipaddr && hw.src != own hwaddr (someone is using our address)
         */
          if sipaddr == acd.ipaddr && netifaddr == hdr.shwadr {
            warn!("conflicting arp packet detected");
            acd_handle_arp_conflict(netifc, acd);
          }}
        
    }
  }
}

/// Handle a IP address conflict after an ARP conflict detection
///  RFC5227, 2.4 "Ongoing Address Conflict Detection and Address Defense"
/// allows three options where:
/// a) means retreat on the first conflict,
/// b) allows to keep an already configured address when having only one
///    conflict in DEFEND_INTERVAL seconds and
/// c) the host will not give up it's address and defend it indefinitely
/// 
/// We use option b) when the acd module represents the netif address, since it
/// helps to improve the chance that one of the two conflicting hosts may be
/// able to retain its address. while we are flexible enough to help network
/// performance
/// 
/// We use option a) when the acd module does not represent the netif address,
/// since we cannot have the acd module announcing or restarting. This
/// situation occurs for the LL acd module when a routable address is used on
///     the netif but the LL address is still open in the background
pub fn acd_handle_arp_conflict(netif: &mut NetworkInterface, acd: &mut AcdStateInfo)
{
  if ( acd.state == ACD_STATE_PASSIVE_ONGOING) {
    // Immediately back off on a conflict
    debug!("conflict when we are in passive mode -> back off");
    acd_stop(acd);
     acd.acd_conflict_callback(netif, ACD_DECLINE);
  }
  else {
    if ( acd.lastconflict > 0) {
      // retreat, there was a conflicting ARP in the last DEFEND_INTERVAL seconds
      debug!("conflict withing DEFEND INTERVAL: retreating");

      // Active TCP sessions are aborted when removing the ip address but a bad
      // connection was inevitable anyway with conflicting hosts
       acd_restart(netif, acd);
    } else {
     debug!("we are defending, send ARP Announce");
      etharp_acd_announce(netif, & acd.ipaddr);
       acd.lastconflict = DEFEND_INTERVAL * ACD_TICKS_PER_SECOND;
    }
  }
}

/// Put the acd module in passive ongoing conflict detection.
pub fn acd_put_in_passive_mode(netif: &mut NetworkInterface, acd: &mut AcdStateInfo)
{
  match acd.state {
    AcdState::Off | AcdState::PassiveOngoing => {}
    AcdState::ProbeWait | AcdState::Probing | AcdState::AnnounceWait | AcdState::RateLimit => {
      acd_stop(acd);
      acd.acd_conflict_callback(netif, AcdCallbackResult::AcdDecline);
    }
    AcdState::Announcing | AcdState::Ongoing => {
      acd.state = AcdState::PassiveOngoing;
      debug!("acd put in passive mode");
    }
  }
}

///
/// @ingroup acd
/// Inform the ACD modules of address changes
///
/// @param netif     network interface on which the address is changing
/// @param old_addr  old ip address
/// @param new_addr  new ip address
///
pub fn acd_netif_ip_addr_changed(netif: &mut NetworkInterface, const ip_addr_t *old_addr,
                          const ip_addr_t *new_addr)
{
  let mut acd: &mut AcdStateInfo;

  LWIP_DEBUGF(ACD_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
    ("acd_netif_ip_addr_changed(): Address changed\n"));

  LWIP_DEBUGF(ACD_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
    ("acd_netif_ip_addr_changed(): old address = %s\n", ipaddr_ntoa(old_addr)));
  LWIP_DEBUGF(ACD_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
    ("acd_netif_ip_addr_changed(): new address = %s\n", ipaddr_ntoa(new_addr)));

  /* If we change from ANY to an IP or from an IP to ANY we do nothing */
  if (ip_addr_isany(old_addr) || ip_addr_isany(new_addr)) {
    return;
  }

  ACD_FOREACH(acd,  netif.acd_list) {
    /* Find ACD module of old address */
    if(ip4_addr_eq(& acd.ipaddr, ip_2_ip4(old_addr))) {
      /* Did we change from a LL address to a routable address? */
      if (ip_addr_islinklocal(old_addr) && !ip_addr_islinklocal(new_addr)) {
        LWIP_DEBUGF(ACD_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE,
          ("acd_netif_ip_addr_changed(): changed from LL to routable address\n"));
        /* Put the module in passive conflict detection mode */
        acd_put_in_passive_mode(netif, acd);
      }
    }
  }
}

// #endif /* LWIP_IPV4 && LWIP_ACD */
