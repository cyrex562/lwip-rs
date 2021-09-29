/*
 * @file
 *
 * AutoIP Automatic LinkLocal IP Configuration
 */

/*
 *
 * Copyright (c) 2007 Dominik Spies <kontakt@dspies.de>
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
 * Author: Dominik Spies <kontakt@dspies.de>
 *
 * This is a AutoIP implementation for the lwIP TCP/IP stack. It aims to conform
 * with RFC 3927.
 *
 */

//  AutoIP Timing 
// #define AUTOIP_TMR_INTERVAL      100
pub const AUTOIP_TMR_INTERVAL: u32 = 100;
// #define AUTOIP_TICKS_PER_SECOND (1000 / AUTOIP_TMR_INTERVAL)
pub const AUTOIP_TICKS_PER_SECOND: u32 = 1000 / AUTOIP_TMR_INTERVAL;

//  AutoIP state information per netif 
pub struct autoip {
    //  the currently selected, probed, announced or used LL IP-Address 
    // let mut if_addr: LwipAddr;
    llipaddr: ip4_addr,
    //  current AutoIP state machine state 
    state: u8,
    //  sent number of probes or announces, dependent on state 
    sent_num: u8,
    //  ticks to wait, tick is AUTOIP_TMR_INTERVAL long 
    ttw: u16,
    //  ticks until a conflict can be solved by defending 
    lastconflict: u8,
    //  total number of probed/used Link Local IP-Addresses 
    tried_llipaddr: u8,
}

impl autoip {
    pub fn clear(&mut self) {
        self.state = 0;
        self.sent_num = 0;
        self.ttw = 0;
        self.lastconflict = 0;
        self.tried_llipaddr = 0;
    }

    pub fn new() -> autoip {
        autoip {
            llipaddr: ip4_addr::new(),
            state: (),
            sent_num: (),
            ttw: (),
            lastconflict: (),
            tried_llipaddr: (),
        }
    }
}

// pub fn  autoip_set_struct(netif: &mut NetIfc, autoip: &mut autoip);
//  Remove a struct autoip previously set to the netif using autoip_set_struct() 
// TODO: #define autoip_remove_struct(netif) loop { (netif).autoip = NULL; } while (0)
// pub fn  autoip_start(netif: &mut NetIfc);
// pub fn  autoip_stop(netif: &mut NetIfc);
// pub fn  autoip_arp_reply(netif: &mut NetIfc, hdr: &mut etharp_hdr);
// pub fn  autoip_tmr();
// pub fn  autoip_network_changed(netif: &mut NetIfc);
// autoip_supplied_address: u8( netif: &mut NetIfc);

//  for lwIP internal use by ip4.c 
// autoip_accept_packet: u8(netif: &mut NetIfc,  addr: &mut ip4_addr);

// TODO: #define netif_autoip_data(netif) ((struct autoip*)netif_get_client_data(netif, LWIP_NETIF_CLIENT_DATA_INDEX_AUTOIP))
pub fn netif_autoip_data(netif: &mut NetIfc) -> &mut autoip {
    netif_get_client_data::<&mut autoip>(netif, LWIP_NETIF_CLIENT_DATA_INDEX)
}
