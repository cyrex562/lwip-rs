/****************************************************************************
* pppoe.h - PPP Over Ethernet implementation for lwIP.
*
* Copyright (c) 2006 by Marc Boucher, Services Informatiques (MBSI) inc.
*
* The authors hereby grant permission to use, copy, modify, distribute,
* and license this software and its documentation for any purpose, provided
* that existing copyright notices are retained in all copies and that this
* notice and the following disclaimer are included verbatim in any
* distributions. No written agreement, license, or royalty fee is required
* for any of the authorized uses.
*
* THIS SOFTWARE IS PROVIDED BY THE CONTRIBUTORS *AS IS* AND ANY EXPRESS OR
* IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
* OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
* IN NO EVENT SHALL THE CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
* INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
* NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
* DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
* THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
* (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
* THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*
******************************************************************************
* REVISION HISTORY
*
* 06-01-01 Marc Boucher <marc@mbsi.ca>
*   Ported to lwIP.
*****************************************************************************/

//  based on NetBSD: if_pppoe.c,v 1.64 2006/01/31 23:50:15 martin Exp 

/*-
 * Copyright (c) 2002 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Martin Husemann <martin@NetBSD.org>.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *        This product includes software developed by the NetBSD
 *        Foundation, Inc. and its contributors.
 * 4. Neither the name of The NetBSD Foundation nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// #define PPP_OE_H

pub struct pppoehdr {
    pub vertype: u8,
    pub code: u8,
    pub session: u16,
    pub plen: u16,
}

pub struct pppoetag {
    pub tag: u16,
    pub len: usize,
}

pub const PPPOE_STATE_INITIAL: u32 = 0;
pub const PPPOE_STATE_PADI_SENT: u32 = 1;
pub const PPPOE_STATE_PADR_SENT: u32 = 2;
pub const PPPOE_STATE_SESSION: u32 = 3;
//  passive 
pub const PPPOE_STATE_PADO_SENT: u32 = 1;

// #define PPPOE_HEADERLEN       sizeof(pppoehdr)
pub const PPPOE_VERTYPE: u32 = 0x11; //  VER=1, TYPE = 1 

pub const PPPOE_TAG_EOL: u32 = 0x0000; //  end of list 
pub const PPPOE_TAG_SNAME: u32 = 0x0101; //  service name 
pub const PPPOE_TAG_ACNAME: u32 = 0x0102; //  access concentrator name 
pub const PPPOE_TAG_HUNIQUE: u32 = 0x0103; //  host unique 
pub const PPPOE_TAG_ACCOOKIE: u32 = 0x0104; //  AC cookie 
pub const PPPOE_TAG_VENDOR: u32 = 0x0105; //  vendor specific 
pub const PPPOE_TAG_RELAYSID: u32 = 0x0110; //  relay session id 
pub const PPPOE_TAG_SNAME_ERR: u32 = 0x0201; //  service name error 
pub const PPPOE_TAG_ACSYS_ERR: u32 = 0x0202; //  AC system error 
pub const PPPOE_TAG_GENERIC_ERR: u32 = 0x0203; //  gerneric error 

pub const PPPOE_CODE_PADI: u32 = 0x09; //  Active Discovery Initiation 
pub const PPPOE_CODE_PADO: u32 = 0x07; //  Active Discovery Offer 
pub const PPPOE_CODE_PADR: u32 = 0; //  Active Discovery Session confirmation 
pub const PPPOE_CODE_PADT: u32 = 0xA7; //  Active Discovery Terminate 

pub const PPPOE_MAX_AC_COOKIE_LEN: u32 = 64;

pub struct pppoe_softc {
    // pub next: &mut pppoe_softc;
    pub sc_ethif: NetIfc,              //  ethernet interface we are using 
    pub pcb: ppp_pcb,                  //  PPP PCB 
    pub sc_dest: eth_addr,             //  hardware address of concentrator 
    pub sc_session: u16,               //  PPPoE session id 
    pub sc_state: u8,                  //  discovery phase or session connected 
    pub sc_service_name: Vec<u8>,      //  if != NULL: requested name of service 
    pub sc_concentrator_name: Vec<u8>, //  if != NULL: requested concentrator id 
    pub sc_ac_cookie: [u8; PPPOE_MAX_AC_COOKIE_LEN], //  content of AC cookie we must echo back 
    pub sc_ac_cookie_len: u8,          //  length of cookie data 
    pub sc_hunique: &mut Vec<u8>,      //  content of host unique we must echo back 
    pub sc_hunique_len: u8,            //  length of host unique 
    pub sc_padi_retried: u8,           //  number of PADI retries already done 
    pub sc_padr_retried: u8,           //  number of PADR retries already done 
}

// #define pppoe_init() //  compatibility define, no initialization needed 
// pppoe_create: &mut ppp_pcb(pppif: &mut NetIfc,
//        ethif: &mut NetIfc,
//        service_name: &String, concentrator_name: &String,
//        ppp_link_status_cb_fn link_status_cb, ctx_cb: &mut Vec<u8>);

/*
 * Functions called from lwIP
 * DO NOT CALL FROM lwIP USER APPLICATION.
 */
// pub fn  pppoe_disc_input(netif: &mut NetIfc, p: &mut PacketBuffer);
// pub fn  pppoe_data_input(netif: &mut NetIfc, p: &mut PacketBuffer);
