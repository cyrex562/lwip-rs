use super::ppp_h::PppCtx;

/****************************************************************************
* pppoe.c - PPP Over Ethernet implementation for lwIP.
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

//  Memory pool 
// LWIP_MEMPOOL_DECLARE(PPPOE_IF, MEMP_NUM_PPPOE_INTERFACES, sizeof(pppoe_softc), "PPPOE_IF")

//  Add a 16 bit  value to a buffer pointed to by PTR 
// pub fn PPPOE_ADD_16(PTR, VAL) \
//     *(PTR)+= 1 = ((VAL) / 256);    \
//     *(PTR)+= 1 = ((VAL) % 256)

//  Add a complete PPPoE header to the buffer pointed to by PTR 
// #define PPPOE_ADD_HEADER(PTR, CODE, SESS, LEN)  \
//     *(PTR)+= 1 = PPPOE_VERTYPE;  \
//     *(PTR)+= 1 = (CODE);         \
//     PPPOE_ADD_16(PTR, SESS);   \
//     PPPOE_ADD_16(PTR, LEN)

pub const PPPOE_DISC_TIMEOUT: u64 = (5 * 1000); //  base for quick timeout calculation 
pub const PPPOE_SLOW_RETRY: u64 = (60 * 1000); //  persistent retry interval 
pub const PPPOE_DISC_MAXPADI: u32 = 4; //  retry PADI four times (quickly) 
pub const PPPOE_DISC_MAXPADI: u32 = 4;
pub const PPPOE_DISC_MAXPADR: u32 = 2; //  retry PADR twice 

// #error "PPPOE_SERVER is not yet supported under lwIP!"
//  from if_spppsubr.c 
pub const IFF_PASSIVE: u32 = IFF_LINK0; //  wait passively for connection 

pub const PPPOE_ERRORSTRING_LEN: u32 = 64;

//  callbacks called from PPP core 
// static pppoe_write: err_t(ppp: &mut ppp_pcb, ctx: &mut Vec<u8>, p: &mut PacketBuffer);
// static pppoe_netif_output: err_t(ppp: &mut ppp_pcb, ctx: &mut Vec<u8>, p: &mut PacketBuffer, protocol: u16);
// pub fn pppoe_connect(ppp: &mut ppp_pcb, ctx: &mut Vec<u8>);
// pub fn pppoe_disconnect(ppp: &mut ppp_pcb, ctx: &mut Vec<u8>);
// static pppoe_destroy: err_t(ppp: &mut ppp_pcb, ctx: &mut Vec<u8>);

//  management routines 
// pub fn pppoe_abort_connect;

// pub fn pppoe_clear_softc(struct pppoe_softc *,  char *);

//  internal timeout handling 
// pub fn pppoe_timeout;

//  sending actual protocol controll packets 
// static pppoe_send_padi: err_t;
// static pppoe_send_padr: err_t;

// static pppoe_send_pado: err_t;
// static pppoe_send_pads: err_t;

// static pppoe_send_padt: err_t(NetIfc *, u_int,  u8 *);

//  internal helper functions 
// static pppoe_xmit: err_t(sc: &mut pppoe_softc, pb: &mut PacketBuffer);
// static struct pppoe_softc* pppoe_find_softc_by_session(u_session: i32, rcvif: &mut NetIfc);
// static struct pppoe_softc* pppoe_find_softc_by_hunique(token: &mut Vec<u8>, len: usize, rcvif: &mut NetIfc);

//  linked list of created pppoe interfaces 
// static pppoe_softc_list: &mut pppoe_softc;

//  Callbacks structure for PPP core 
// static const struct link_callbacks pppoe_callbacks = {
//   pppoe_connect,

//   None,

//   pppoe_disconnect,
//   pppoe_destroy,
//   pppoe_write,
//   pppoe_netif_output,
//   None,
//   None
// };

/*
 * Create a new PPP Over Ethernet (PPPoE) connection.
 *
 * Return 0 on success, an error code on failure.
 */
pub fn pppoe_create(
    pppif: &mut NetIfc,
    ethif: &mut NetIfc,
    service_name: &String,
    concentrator_name: &String,
    link_status_cb: ppp_link_status_cb_fn,
    ctx_cb: &mut Vec<u8>,
) -> PppCtx {
    let mut ppp: &mut PppCtx;
    let mut sc: &mut pppoe_softc;

    // LWIP_ASSERT_CORE_LOCKED()

    sc = LWIP_MEMPOOL_ALLOC(PPPOE_IF);
    if (sc == None) {
        return None;
    }

    ppp = ppp_new(pppif, &pppoe_callbacks, sc, link_status_cb, ctx_cb);
    if (ppp == None) {
        LWIP_MEMPOOL_FREE(PPPOE_IF, sc);
        return None;
    }

    //memset(sc, 0, sizeof(pppoe_softc));
    sc.pcb = ppp;
    sc.sc_ethif = ethif;
    //  put the new interface at the head of the list 
    sc.next = pppoe_softc_list;
    pppoe_softc_list = sc;
    return ppp;
}

//  Called by PPP core 
pub fn pppoe_write(ppp: &mut PppCtx, ctx: &mut Vec<u8>, p: &mut PacketBuffer) -> Result<(), LwipStatus> {
    let sc: &mut pppoe_softc = ctx;
    let ph: &mut PacketBuffer; //  Ethernet + PPPoE header 
    let ret: err_t;

    let tot_len: u16;
    //  MIB2_STATS 

    //  skip address & flags 
    pbuf_remove_header(p, 2);

    ph = pbuf_alloc(PBUF_LINK, (PPPOE_HEADERLEN), PBUF_RAM);
    if (!ph) {
        LINK_STATS_INC(link.memerr);
        LINK_STATS_INC(link.proterr);
        MIB2_STATS_NETIF_INC(ppp.netif, ifoutdiscards);
        pbuf_free(p);
        return ERR_MEM;
    }

    pbuf_remove_header(ph, PPPOE_HEADERLEN); //  hide PPPoE header 
    pbuf_cat(ph, p);

    tot_len = ph.tot_len;

    ret = pppoe_xmit(sc, ph);
    if (ret != ERR_OK) {
        LINK_STATS_INC(link.err);
        MIB2_STATS_NETIF_INC(ppp.netif, ifoutdiscards);
        return ret;
    }

    MIB2_STATS_NETIF_ADD(ppp.netif, ifoutoctets, tot_len);
    MIB2_STATS_NETIF_INC(ppp.netif, ifoutucastpkts);
    LINK_STATS_INC(link.xmit);
    return Ok(());
}

//  Called by PPP core 
pub fn pppoe_netif_output(
    ppp: &mut PppCtx,
    ctx: &mut Vec<u8>,
    p: &mut PacketBuffer,
    protocol: u16,
) -> Result<(), LwipError> {
    let sc: &mut pppoe_softc = ctx;
    let pb: &mut PacketBuffer;
    let pl: &mut Vec<u8>;
    let err: err_t;
    let tot_len: u16;
    //  MIB2_STATS 
    //  @todo: try to use pbuf_header() here! 
    pb = pbuf_alloc(PBUF_LINK, PPPOE_HEADERLEN + sizeof(protocol), PBUF_RAM);
    if (!pb) {
        LINK_STATS_INC(link.memerr);
        LINK_STATS_INC(link.proterr);
        MIB2_STATS_NETIF_INC(ppp.netif, ifoutdiscards);
        return ERR_MEM;
    }

    pbuf_remove_header(pb, PPPOE_HEADERLEN);

    pl = pb.payload;
    PUTSHORT(protocol, pl);

    pbuf_chain(pb, p);

    tot_len = pb.tot_len;

    if ((err = pppoe_xmit(sc, pb)) != ERR_OK) {
        LINK_STATS_INC(link.err);
        MIB2_STATS_NETIF_INC(ppp.netif, ifoutdiscards);
        return err;
    }

    MIB2_STATS_NETIF_ADD(ppp.netif, ifoutoctets, tot_len);
    MIB2_STATS_NETIF_INC(ppp.netif, ifoutucastpkts);
    LINK_STATS_INC(link.xmit);
    return Ok(());
}

pub fn pppoe_destroy(ppp: &mut PppCtx, ctx: &mut Vec<u8>) -> Result<(), LwipError> {
    let sc: &mut pppoe_softc = ctx;
    // struct pppoe_softc **copp, *freep;
    let copp: &mut pppoe_softc;
    let freep: &mut pppoe_softc;

    sys_untimeout(pppoe_timeout, sc);

    //  remove interface from list 
    // for (copp = &pppoe_softc_list; (freep = *copp); copp = &freep.next) {
    //   if (freep == sc) {
    //      *copp = freep.next;
    //      break;
    //   }
    // }

    if (sc.sc_concentrator_name) {
        mem_free(sc.sc_concentrator_name);
    }
    if (sc.sc_service_name) {
        mem_free(sc.sc_service_name);
    }

    LWIP_MEMPOOL_FREE(PPPOE_IF, sc);

    return Ok(());
}

/*
 * Find the interface handling the specified session.
 * Note: O(number of sessions open), this is a client-side only, mean
 * and lean implementation, so number of open sessions typically should
 * be 1.
 */
pub fn pppoe_find_softc_by_session(u_session: i32, rcvif: &mut NetIfc) -> pppoe_softc {
    let mut sc: &mut pppoe_softc;

    // for (sc = pppoe_softc_list; sc != None; sc = sc.next) {
    //   if (sc.sc_state == PPPOE_STATE_SESSION
    //       && sc.sc_session == session
    //        && sc.sc_ethif == rcvif) {
    //          return sc;
    //     }
    // }
    return None;
}

/* Check host unique token passed and return appropriate softc pointer,
 * or NULL if token is bogus. */
pub fn pppoe_find_softc_by_hunique(
    token: &mut Vec<u8>,
    len: usize,
    rcvif: &mut NetIfc,
) -> pppoe_softc {
    let sc: &mut pppoe_softc;
    let t;

    if (len != std::mem::sizeof(sc)) {
        return None;
    }
    MEMCPY(&t, token, len);

    // for (sc = pppoe_softc_list; sc != None; sc = sc.next) {
    //   if (sc == t) {
    //     break;
    //   }
    // }

    if (sc == None) {
        PPPDEBUG(
            LOG_DEBUG,
            ("pppoe: alien host unique tag, no session found\n"),
        );
        return None;
    }

    //  should be safe to access *sc now 
    if (sc.sc_state < PPPOE_STATE_PADI_SENT || sc.sc_state >= PPPOE_STATE_SESSION) {
        // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": host unique tag found, but it belongs to a connection in state %d\n",
        //   sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, sc.sc_state));
        return None;
    }
    if (sc.sc_ethif != rcvif) {
        // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": wrong interface, not accepting host unique\n",
        //   sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
        return None;
    }
    return sc;
}

//  analyze and handle a single received packet while not in session state 
pub fn pppoe_disc_input(netif: &mut NetIfc, pb: &mut PacketBuffer) {
    let tag: u16;
    let len;
    let off;
    let session: u16;
    let plen: u16;
    let mut sc: &mut pppoe_softc;
    let err_msg: &String = None;
    let ac_cookie: &mut Vec<u8>;
    let ac_cookie_len: u16;
    let hunique: &mut Vec<u8>;
    let hunique_len: usize;
    let mut ph: &mut pppoehdr;
    let pt: pppoetag;
    let err: i32;
    let mut ethhdr: &mut eth_hdr;

    //  don't do anything if there is not a single PPPoE instance 
    if (pppoe_softc_list == None) {
        pbuf_free(pb);
        return;
    }

    pb = pbuf_coalesce(pb, PBUF_RAW);

    ethhdr = pb.payload;

    ac_cookie = None;
    ac_cookie_len = 0;

    hunique = None;
    hunique_len = 0;

    session = 0;
    off = sizeof(eth_hdr) + sizeof(pppoehdr);
    if (pb.len < off) {
        PPPDEBUG(LOG_DEBUG, ("pppoe: packet too short: %d\n", pb.len));
        // goto done;
    }

    ph = (ethhdr + 1);
    if (ph.vertype != PPPOE_VERTYPE) {
        PPPDEBUG(
            LOG_DEBUG,
            ("pppoe: unknown version/type packet: 0x%x\n", ph.vertype),
        );
        // goto done;
    }
    session = lwip_ntohs(ph.session);
    plen = lwip_ntohs(ph.plen);

    if (plen > (pb.len - off)) {
        PPPDEBUG(
            LOG_DEBUG,
            (
                "pppoe: packet content does not fit: data available = %d, packet size = %u\n",
                pb.len - off,
                plen,
            ),
        );
        // goto done;
    }
    if (pb.tot_len == pb.len) {
        let framelen: u16 = off + plen;
        if (framelen < pb.len) {
            //  ignore trailing garbage 
            pb.tot_len = pb.len = framelen;
        }
    }
    tag = 0;
    len = 0;
    sc = None;
    while (off + sizeof(pt) <= pb.len) {
        MEMCPY(&pt, pb.payload + off, sizeof(pt));
        tag = lwip_ntohs(pt.tag);
        len = lwip_ntohs(pt.len);
        if (off + sizeof(pt) + len > pb.len) {
            PPPDEBUG(
                LOG_DEBUG,
                ("pppoe: tag 0x%x len 0x%x is too long\n", tag, len),
            );
            // goto done;
        }
        match (tag) {
            PPPOE_TAG_EOL => {}
            // goto breakbreak;
            PPPOE_TAG_SNAME => {}
            // break;  //  ignored 
            PPPOE_TAG_ACNAME => {}
            // break;  //  ignored 
            PPPOE_TAG_HUNIQUE => {
                if (sc != None) {
                    // break;
                }

                hunique = pb.payload + off + sizeof(pt);
                hunique_len = len;

                sc = pppoe_find_softc_by_hunique(pb.payload + off + sizeof(pt), len, netif);
                // break;
            }
            PPPOE_TAG_ACCOOKIE => {
                if (ac_cookie == None) {
                    if (len > PPPOE_MAX_AC_COOKIE_LEN) {
                        PPPDEBUG(
                            LOG_DEBUG,
                            (
                                "pppoe: AC cookie is too long: len = %d, max = %d\n",
                                len,
                                PPPOE_MAX_AC_COOKIE_LEN,
                            ),
                        );
                        // goto done;
                    }
                    ac_cookie = pb.payload + off + sizeof(pt);
                    ac_cookie_len = len;
                }
                // break;
            }
            PPPOE_TAG_SNAME_ERR => {
                err_msg = "SERVICE NAME ERROR";
            }

            PPPOE_TAG_ACSYS_ERR => {
                err_msg = "AC SYSTEM ERROR";
            }

            PPPOE_TAG_GENERIC_ERR => {
                err_msg = "GENERIC ERROR";
            }
            _ => {}
        }

        if (err_msg != None) {
            let error_tmp: String;
            let error_len: u16 = LWIP_MIN(len, sizeof(error_tmp) - 1);
            strncpy(error_tmp, pb.payload + off + sizeof(pt), error_len);
            error_tmp[error_len] = '\0';
            if (sc) {
                // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": %s: %s\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, err_msg, error_tmp));
            } else {
                PPPDEBUG(LOG_DEBUG, ("pppoe: %s: %s\n", err_msg, error_tmp));
            }
        }

        off += sizeof(pt) + len;
    }

    // breakbreak:;
    match (ph.code) {
        PPPOE_CODE_PADI => {
            /*
             * got service name, concentrator name, and/or host unique.
             * ignore if we have no interfaces with IFF_PASSIVE|IFF_UP.
             */
            if (LIST_EMPTY(&pppoe_softc_list)) {
                // goto done;
            }
            // LIST_FOREACH(sc, &pppoe_softc_list, sc_list) {
            //   if (!(sc.sc_sppp.pp_if.if_flags & IFF_UP)) {
            //     continue;
            //   }
            //   if (!(sc.sc_sppp.pp_if.if_flags & IFF_PASSIVE)) {
            //     continue;
            //   }
            //   if (sc.sc_state == PPPOE_STATE_INITIAL) {
            //     break;
            //   }
            // }
            if (sc == None) {
                //  PPPDEBUG(LOG_DEBUG, ("pppoe: free passive interface is not found\n")); 
                // goto done;
            }
            if (hunique) {
                if (sc.sc_hunique) {
                    mem_free(sc.sc_hunique);
                }
                sc.sc_hunique = mem_malloc(hunique_len);
                if (sc.sc_hunique == None) {
                    // goto done;
                }
                sc.sc_hunique_len = hunique_len;
                MEMCPY(sc.sc_hunique, hunique, hunique_len);
            }
            // MEMCPY(&sc.sc_dest, eh.ether_shost, sizeof sc.sc_dest);
            sc.sc_state = PPPOE_STATE_PADO_SENT;
            pppoe_send_pado(sc);
            // break;
        }
        PPPOE_CODE_PADR => {
            /*
             * get sc from ac_cookie if IFF_PASSIVE
             */
            if (ac_cookie == None) {
                //  be quiet if there is not a single pppoe instance 
                PPPDEBUG(
                    LOG_DEBUG,
                    ("pppoe: received PADR but not includes ac_cookie\n"),
                );
                // goto done;
            }
            sc = pppoe_find_softc_by_hunique(ac_cookie, ac_cookie_len, netif);
            if (sc == None) {
                //  be quiet if there is not a single pppoe instance 
                if (!LIST_EMPTY(&pppoe_softc_list)) {
                    PPPDEBUG(
                        LOG_DEBUG,
                        ("pppoe: received PADR but could not find request for it\n"),
                    );
                }
                // goto done;
            }
            if (sc.sc_state != PPPOE_STATE_PADO_SENT) {
                // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": received unexpected PADR\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
                // goto done;
            }
            if (hunique) {
                if (sc.sc_hunique) {
                    mem_free(sc.sc_hunique);
                }
                sc.sc_hunique = mem_malloc(hunique_len);
                if (sc.sc_hunique == None) {
                    // goto done;
                }
                sc.sc_hunique_len = hunique_len;
                MEMCPY(sc.sc_hunique, hunique, hunique_len);
            }
            pppoe_send_pads(sc);
            sc.sc_state = PPPOE_STATE_SESSION;
            ppp_start(sc.pcb); //  notify upper layers 
            // break;
        }
        //  ignore, we are no access concentrator 
        // goto done;
        PPPOE_CODE_PADO => {
            if (sc == None) {
                //  be quiet if there is not a single pppoe instance 
                if (pppoe_softc_list != None) {
                    PPPDEBUG(
                        LOG_DEBUG,
                        ("pppoe: received PADO but could not find request for it\n"),
                    );
                }
                // goto done;
            }
            if (sc.sc_state != PPPOE_STATE_PADI_SENT) {
                // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": received unexpected PADO\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
                // goto done;
            }
            if (ac_cookie) {
                sc.sc_ac_cookie_len = ac_cookie_len;
                MEMCPY(sc.sc_ac_cookie, ac_cookie, ac_cookie_len);
            }
            MEMCPY(&sc.sc_dest, ethhdr.src.addr, sizeof(sc.sc_dest.addr));
            sys_untimeout(pppoe_timeout, sc);
            sc.sc_padr_retried = 0;
            sc.sc_state = PPPOE_STATE_PADR_SENT;
            if ((err = pppoe_send_padr(sc)) != 0) {
                // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": failed to send PADR, error=%d\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, err));
                //  if PPPDEBUG is disabled 
            }
            sys_timeout(
                PPPOE_DISC_TIMEOUT * (1 + sc.sc_padr_retried),
                pppoe_timeout,
                sc,
            );
            // break;
        }
        PPPOE_CODE_PADS => {
            if (sc == None) {
                // goto done;
            }
            sc.sc_session = session;
            sys_untimeout(pppoe_timeout, sc);
            // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": session 0x%x connected\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, session));
            sc.sc_state = PPPOE_STATE_SESSION;
            ppp_start(sc.pcb); //  notify upper layers 
            // break;
        }
        PPPOE_CODE_PADT => {
            /* Don't disconnect here, we let the LCP Echo/Reply find the fact
             * that PPP session is down. Asking the PPP stack to end the session
             * require strict checking about the PPP phase to prevent endless
             * disconnection loops.
             */

            if (sc == None) { //  PADT frames are rarely sent with a hunique tag, this is actually almost always true 
                // goto done;
            }
            pppoe_clear_softc(sc, "received PADT");

            // break;
        }
        _ => {
            if (sc) {
                // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": unknown code (0x%"X16_F") session = 0x%"X16_F"\n",
                //     sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num,
                //     ph.code, session));
            } else {
                // PPPDEBUG(LOG_DEBUG, ("pppoe: unknown code (0x%"X16_F") session = 0x%"X16_F"\n", ph.code, session));
            }
        } // break;
    }

    // done:
    pbuf_free(pb);
    return;
}

pub fn pppoe_data_input(netif: &mut NetIfc, pb: &mut PacketBuffer) {
    let session: u16;
    let plen: u16;
    let mut sc: &mut pppoe_softc;
    let mut ph: &mut pppoehdr;
    let shost: [u8; ETHER_ADDR_LEN];

    MEMCPY(shost, (pb.payload).src.addr, sizeof(shost));

    if (pbuf_remove_header(pb, sizeof(eth_hdr)) != 0) {
        //  bail out 
        PPPDEBUG(LOG_ERR, ("pppoe_data_input: PacketBuffer_remove_header failed\n"));
        LINK_STATS_INC(link.lenerr);
        // goto drop;
    }

    if (pb.len < sizeof(*ph)) {
        PPPDEBUG(
            LOG_DEBUG,
            ("pppoe_data_input: could not get PPPoE header\n"),
        );
        // goto drop;
    }
    ph = pb.payload;

    if (ph.vertype != PPPOE_VERTYPE) {
        PPPDEBUG(
            LOG_DEBUG,
            (
                "pppoe (data): unknown version/type packet: 0x%x\n",
                ph.vertype,
            ),
        );
        // goto drop;
    }
    if (ph.code != 0) {
        // goto drop;
    }

    session = lwip_ntohs(ph.session);
    sc = pppoe_find_softc_by_session(session, netif);
    if (sc == None) {
        PPPDEBUG(
            LOG_DEBUG,
            (
                "pppoe: input for unknown session 0x%x, sending PADT\n",
                session,
            ),
        );
        pppoe_send_padt(netif, session, shost);

        // goto drop;
    }

    plen = lwip_ntohs(ph.plen);

    if (pbuf_remove_header(pb, PPPOE_HEADERLEN) != 0) {
        //  bail out 
        PPPDEBUG(
            LOG_ERR,
            ("pppoe_data_input: PacketBuffer_remove_header PPPOE_HEADERLEN failed\n"),
        );
        LINK_STATS_INC(link.lenerr);
        // goto drop;
    }

    // PPPDEBUG(LOG_DEBUG, ("pppoe_data_input: %c%c%"U16_F": pkthdr.len=%d, pppoe.len=%d\n",
    //       sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num,
    //       pb.len, plen));

    if (pb.tot_len < plen) {
        // goto drop;
    }

    //  Dispatch the packet thereby consuming it. 
    ppp_input(sc.pcb, pb);
    return;

    // drop:
    pbuf_free(pb);
}

pub fn pppoe_output(sc: &mut pppoe_softc, pb: &mut PacketBuffer) -> Result<(), LwipError> {
    let mut ethhdr: &mut eth_hdr;
    let etype: u16;
    let res: err_t;

    //  make room for Ethernet header - should not fail 
    if (pbuf_add_header(pb, sizeof(eth_hdr)) != 0) {
        //  bail out 
        // PPPDEBUG(LOG_ERR, ("pppoe: %c%c%"U16_F": pppoe_output: could not allocate room for Ethernet header\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
        LINK_STATS_INC(link.lenerr);
        pbuf_free(pb);
        return ERR_BUF;
    }
    ethhdr = pb.payload;
    // etype = sc.sc_state == PPPOE_STATE_SESSION ? ETHTYPE_PPPOE : ETHTYPE_PPPOEDISC;
    ethhdr.hdr_type = lwip_htons(etype);
    MEMCPY(
        &ethhdr.dest.addr,
        &sc.sc_dest.addr,
        sizeof(ethhdr.dest.addr),
    );
    MEMCPY(
        &ethhdr.src.addr,
        &sc.sc_ethif.hwaddr,
        sizeof(ethhdr.src.addr),
    );

    // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F" (%x) state=%d, session=0x%x output -> %02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F":%02"X16_F", len=%d\n",
    //     sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, etype,
    //     sc.sc_state, sc.sc_session,
    //     sc.sc_dest.addr[0], sc.sc_dest.addr[1], sc.sc_dest.addr[2], sc.sc_dest.addr[3], sc.sc_dest.addr[4], sc.sc_dest.addr[5],
    //     pb.tot_len));

    res = sc.sc_ethif.linkoutput(sc.sc_ethif, pb);

    pbuf_free(pb);

    return res;
}

pub fn pppoe_send_padi(sc: &mut pppoe_softc) -> Result<(), LwipError> {
    let pb: &mut PacketBuffer;
    let p: &mut Vec<u8>;
    let len: i32;
    let l1: i32 = 0;
    let l2 = 0; //  XXX: gcc 

    //  calculate length of frame (excluding ethernet header + pppoe header) 
    len = 2 + 2 + 2 + 2 + sizeof(sc); //  service name tag is required, host unique is send too 

    if (sc.sc_service_name != None) {
        l1 = strlen(sc.sc_service_name);
        len += l1;
    }
    if (sc.sc_concentrator_name != None) {
        l2 = strlen(sc.sc_concentrator_name);
        len += 2 + 2 + l2;
    }

    // LWIP_ASSERT(
        "sizeof(EthHdr) + PPPOE_HEADERLEN + len <= 0xffff",
        sizeof(eth_hdr) + PPPOE_HEADERLEN + len <= 0xffff,
    );

    //  allocate a buffer 
    pb = pbuf_alloc(PBUF_LINK, (PPPOE_HEADERLEN + len), PBUF_RAM);
    if (!pb) {
        return ERR_MEM;
    }
    // LWIP_ASSERT("pb.tot_len == pb.len", pb.tot_len == pb.len);

    p = pb.payload;
    //  fill in pkt 
    PPPOE_ADD_HEADER(p, PPPOE_CODE_PADI, 0, len);
    PPPOE_ADD_16(p, PPPOE_TAG_SNAME);

    if (sc.sc_service_name != None) {
        PPPOE_ADD_16(p, l1);
        MEMCPY(p, sc.sc_service_name, l1);
        p += l1;
    } else {
        PPPOE_ADD_16(p, 0);
    }

    if (sc.sc_concentrator_name != None) {
        PPPOE_ADD_16(p, PPPOE_TAG_ACNAME);
        PPPOE_ADD_16(p, l2);
        MEMCPY(p, sc.sc_concentrator_name, l2);
        p += l2;
    }

    PPPOE_ADD_16(p, PPPOE_TAG_HUNIQUE);
    PPPOE_ADD_16(p, sizeof(sc));
    MEMCPY(p, &sc, sizeof(sc));

    //  send pkt 
    return pppoe_output(sc, pb);
}

pub fn pppoe_timeout(arg: &mut Vec<u8>) {
    let retry_wait: u32;
    let err: i32;
    let sc: &mut pppoe_softc = arg;

    // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": timeout\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));

    match (sc.sc_state) {
        PPPOE_STATE_PADI_SENT => {
            /*
             * We have two basic ways of retrying:
             *  - Quick retry mode: try a few times in short sequence
             *  - Slow retry mode: we already had a connection successfully
             *    established and will try infinitely (without user
             *    intervention)
             * We only enter slow retry mode if IFF_LINK1 (aka autodial)
             * is not set.
             */
            if (sc.sc_padi_retried < 0xff) {
                sc.sc_padi_retried += 1;
            }
            if (!sc.pcb.settings.persist && sc.sc_padi_retried >= PPPOE_DISC_MAXPADI) {
                if ((sc.sc_sppp.pp_if.if_flags & IFF_LINK1) == 0) {
                    //  slow retry mode 
                    retry_wait = PPPOE_SLOW_RETRY;
                } else {
                    pppoe_abort_connect(sc);
                    return;
                }
            }
            //  initialize for quick retry mode 
            retry_wait = LWIP_MIN(PPPOE_DISC_TIMEOUT * sc.sc_padi_retried, PPPOE_SLOW_RETRY);
            if ((err = pppoe_send_padi(sc)) != 0) {
                sc.sc_padi_retried -= 1;
                // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": failed to transmit PADI, error=%d\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, err));
                //  if PPPDEBUG is disabled 
            }
            sys_timeout(retry_wait, pppoe_timeout, sc);
            // break;
        }
        PPPOE_STATE_PADR_SENT => {
            sc.sc_padr_retried += 1;
            if (sc.sc_padr_retried >= PPPOE_DISC_MAXPADR) {
                MEMCPY(&sc.sc_dest, ethbroadcast.addr, sizeof(sc.sc_dest));
                sc.sc_state = PPPOE_STATE_PADI_SENT;
                sc.sc_padr_retried = 0;
                if ((err = pppoe_send_padi(sc)) != 0) {
                    // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": failed to send PADI, error=%d\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, err));
                    //  if PPPDEBUG is disabled 
                }
                sys_timeout(
                    PPPOE_DISC_TIMEOUT * (1 + sc.sc_padi_retried),
                    pppoe_timeout,
                    sc,
                );
                return;
            }
            if ((err = pppoe_send_padr(sc)) != 0) {
                sc.sc_padr_retried -= 1;
                // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": failed to send PADR, error=%d\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, err));
                //  if PPPDEBUG is disabled 
            }
            sys_timeout(
                PPPOE_DISC_TIMEOUT * (1 + sc.sc_padr_retried),
                pppoe_timeout,
                sc,
            );
            // break;
        }
        _ => {
            return;
        } //  all done, work in peace 
    }
}

//  Start a connection (i.e. initiate discovery phase) 
pub fn pppoe_connect(ppp: &mut PppCtx, ctx: &mut Vec<u8>) {
    let err: err_t;
    let sc: &mut pppoe_softc = ctx;
    let lcp_wo: &mut lcp_options;
    let lcp_ao: &mut lcp_options;
    let ipcp_wo: &mut ipcp_options;
    let ipcp_ao: &mut ipcp_options;

    sc.sc_session = 0;
    sc.sc_ac_cookie_len = 0;
    sc.sc_padi_retried = 0;
    sc.sc_padr_retried = 0;
    //  changed to real address later 
    MEMCPY(&sc.sc_dest, ethbroadcast.addr, sizeof(sc.sc_dest));

    //  wait PADI if IFF_PASSIVE 
    if (sc.sc_sppp.pp_if.if_flags & IFF_PASSIVE) {
        return 0;
    }

    lcp_wo = &ppp.lcp_wantoptions;
    lcp_wo.mru = sc.sc_ethif.mtu - PPPOE_HEADERLEN - 2; //  two byte PPP protocol discriminator, then IP data 
    lcp_wo.neg_asyncmap = 0;
    lcp_wo.neg_pcompression = 0;
    lcp_wo.neg_accompression = 0;
    lcp_wo.passive = 0;
    lcp_wo.silent = 0;

    lcp_ao = &ppp.lcp_allowoptions;
    lcp_ao.mru = sc.sc_ethif.mtu - PPPOE_HEADERLEN - 2; //  two byte PPP protocol discriminator, then IP data 
    lcp_ao.neg_asyncmap = 0;
    lcp_ao.neg_pcompression = 0;
    lcp_ao.neg_accompression = 0;

    ipcp_wo = &ppp.ipcp_wantoptions;
    ipcp_wo.neg_vj = 0;
    ipcp_wo.old_vj = 0;

    ipcp_ao = &ppp.ipcp_allowoptions;
    ipcp_ao.neg_vj = 0;
    ipcp_ao.old_vj = 0;

    //  save state, in case we fail to send PADI 
    sc.sc_state = PPPOE_STATE_PADI_SENT;
    if ((err = pppoe_send_padi(sc)) != 0) {
        // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": failed to send PADI, error=%d\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, err));
    }
    sys_timeout(PPPOE_DISC_TIMEOUT, pppoe_timeout, sc);
}

//  disconnect 
pub fn pppoe_disconnect(ppp: &mut PppCtx, ctx: &mut Vec<u8>) {
    let sc: &mut pppoe_softc = ctx;

    // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": disconnecting\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
    if (sc.sc_state == PPPOE_STATE_SESSION) {
        pppoe_send_padt(sc.sc_ethif, sc.sc_session, &sc.sc_dest);
    }

    //  stop any timer, disconnect can be called while initiating is in progress 
    sys_untimeout(pppoe_timeout, sc);
    sc.sc_state = PPPOE_STATE_INITIAL;

    if (sc.sc_hunique) {
        mem_free(sc.sc_hunique);
        sc.sc_hunique = None; //  probably not necessary, if state is initial we shouldn't have to access hunique anyway  
    }
    sc.sc_hunique_len = 0; //  probably not necessary, if state is initial we shouldn't have to access hunique anyway  

    ppp_link_end(ppp); //  notify upper layers 
    return;
}

//  Connection attempt aborted 
pub fn pppoe_abort_connect(sc: &mut pppoe_softc) {
    // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": could not establish connection\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
    sc.sc_state = PPPOE_STATE_INITIAL;
    ppp_link_failed(sc.pcb); //  notify upper layers 
}

//  Send a PADR packet 
pub fn pppoe_send_padr(sc: &mut pppoe_softc) -> Result<(), LwipError> {
    let pb: &mut PacketBuffer;
    let p: &mut Vec<u8>;
    let len: usize;
    let l1: usize = 0; //  XXX: gcc 

    len = 2 + 2 + 2 + 2 + sizeof(sc); //  service name, host unique 

    if (sc.sc_service_name != None) {
        //  service name tag maybe empty 
        l1 = strlen(sc.sc_service_name);
        len += l1;
    }

    if (sc.sc_ac_cookie_len > 0) {
        len += 2 + 2 + sc.sc_ac_cookie_len; //  AC cookie 
    }
    // LWIP_ASSERT(
        "sizeof(EthHdr) + PPPOE_HEADERLEN + len <= 0xffff",
        sizeof(eth_hdr) + PPPOE_HEADERLEN + len <= 0xffff,
    );
    pb = pbuf_alloc(PBUF_LINK, (PPPOE_HEADERLEN + len), PBUF_RAM);
    if (!pb) {
        return ERR_MEM;
    }
    // LWIP_ASSERT("pb.tot_len == pb.len", pb.tot_len == pb.len);
    p = pb.payload;
    PPPOE_ADD_HEADER(p, PPPOE_CODE_PADR, 0, len);
    PPPOE_ADD_16(p, PPPOE_TAG_SNAME);

    if (sc.sc_service_name != None) {
        PPPOE_ADD_16(p, l1);
        MEMCPY(p, sc.sc_service_name, l1);
        p += l1;
    } else {
        PPPOE_ADD_16(p, 0);
    }
    if (sc.sc_ac_cookie_len > 0) {
        PPPOE_ADD_16(p, PPPOE_TAG_ACCOOKIE);
        PPPOE_ADD_16(p, sc.sc_ac_cookie_len);
        MEMCPY(p, sc.sc_ac_cookie, sc.sc_ac_cookie_len);
        p += sc.sc_ac_cookie_len;
    }
    PPPOE_ADD_16(p, PPPOE_TAG_HUNIQUE);
    PPPOE_ADD_16(p, sizeof(sc));
    MEMCPY(p, &sc, sizeof(sc));

    return pppoe_output(sc, pb);
}

//  send a PADT packet 
pub fn pppoe_send_padt(
    outgoing_if: &mut NetIfc,
    u_session: i32,
    dest: &mut Vec<u8>,
) -> Result<(), LwipError> {
    let pb: &mut PacketBuffer;
    let mut ethhdr: &mut eth_hdr;
    let res: err_t;
    let p: &mut Vec<u8>;

    pb = pbuf_alloc(PBUF_LINK, (PPPOE_HEADERLEN), PBUF_RAM);
    if (!pb) {
        return ERR_MEM;
    }
    // LWIP_ASSERT("pb.tot_len == pb.len", pb.tot_len == pb.len);

    if (pbuf_add_header(pb, sizeof(eth_hdr))) {
        PPPDEBUG(
            LOG_ERR,
            ("pppoe: pppoe_send_padt: could not allocate room for PPPoE header\n"),
        );
        LINK_STATS_INC(link.lenerr);
        pbuf_free(pb);
        return ERR_BUF;
    }
    ethhdr = pb.payload;
    ethhdr.ether_type = PP_HTONS(ETHTYPE_PPPOEDISC);
    MEMCPY(&ethhdr.dest.addr, dest, sizeof(ethhdr.dest.addr));
    MEMCPY(
        &ethhdr.src.addr,
        &outgoing_if.hwaddr,
        sizeof(ethhdr.src.addr),
    );

    p = (ethhdr + 1);
    PPPOE_ADD_HEADER(p, PPPOE_CODE_PADT, session, 0);

    res = outgoing_if.linkoutput(outgoing_if, pb);

    pbuf_free(pb);

    return res;
}

pub fn pppoe_send_pado(sc: &mut pppoe_softc) -> Result<(), LwipError> {
    let pb: &mut PacketBuffer;
    let p: &mut Vec<u8>;
    let len: usize;

    //  calc length 
    len = 0;
    //  include ac_cookie 
    len += 2 + 2 + sizeof(sc);
    //  include hunique 
    len += 2 + 2 + sc.sc_hunique_len;
    pb = pbuf_alloc(PBUF_LINK, (PPPOE_HEADERLEN + len), PBUF_RAM);
    if (!pb) {
        return ERR_MEM;
    }
    // LWIP_ASSERT("pb.tot_len == pb.len", pb.tot_len == pb.len);
    p = pb.payload;
    PPPOE_ADD_HEADER(p, PPPOE_CODE_PADO, 0, len);
    PPPOE_ADD_16(p, PPPOE_TAG_ACCOOKIE);
    PPPOE_ADD_16(p, sizeof(sc));
    MEMCPY(p, &sc, sizeof(sc));
    p += sizeof(sc);
    PPPOE_ADD_16(p, PPPOE_TAG_HUNIQUE);
    PPPOE_ADD_16(p, sc.sc_hunique_len);
    MEMCPY(p, sc.sc_hunique, sc.sc_hunique_len);
    return pppoe_output(sc, pb);
}

pub fn pppoe_send_pads(sc: &mut pppoe_softc) -> Result<(), LwipError> {
    let pb: &mut PacketBuffer;
    let p: &mut Vec<u8>;
    let len: usize;
    let l1 = 0; //  XXX: gcc 

    sc.sc_session = mono_time.tv_sec % 0xff + 1;
    //  calc length 
    len = 0;
    //  include hunique 
    len += 2 + 2 + 2 + 2 + sc.sc_hunique_len; //  service name, host unique
    if (sc.sc_service_name != None) {
        //  service name tag maybe empty 
        l1 = strlen(sc.sc_service_name);
        len += l1;
    }
    pb = pbuf_alloc(PBUF_LINK, (PPPOE_HEADERLEN + len), PBUF_RAM);
    if (!pb) {
        return ERR_MEM;
    }
    // LWIP_ASSERT("pb.tot_len == pb.len", pb.tot_len == pb.len);
    p = pb.payload;
    PPPOE_ADD_HEADER(p, PPPOE_CODE_PADS, sc.sc_session, len);
    PPPOE_ADD_16(p, PPPOE_TAG_SNAME);
    if (sc.sc_service_name != None) {
        PPPOE_ADD_16(p, l1);
        MEMCPY(p, sc.sc_service_name, l1);
        p += l1;
    } else {
        PPPOE_ADD_16(p, 0);
    }
    PPPOE_ADD_16(p, PPPOE_TAG_HUNIQUE);
    PPPOE_ADD_16(p, sc.sc_hunique_len);
    MEMCPY(p, sc.sc_hunique, sc.sc_hunique_len);
    return pppoe_output(sc, pb);
}

pub fn pppoe_xmit(sc: &mut pppoe_softc, pb: &mut PacketBuffer) -> Result<(), LwipError> {
    let p: &mut Vec<u8>;
    let len: usize;

    len = pb.tot_len;

    //  make room for PPPoE header - should not fail 
    if (pbuf_add_header(pb, PPPOE_HEADERLEN) != 0) {
        //  bail out 
        // PPPDEBUG(LOG_ERR, ("pppoe: %c%c%"U16_F": pppoe_xmit: could not allocate room for PPPoE header\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
        // LINK_STATS_INC(link.lenerr);
        pbuf_free(pb);
        return ERR_BUF;
    }

    p = pb.payload;
    PPPOE_ADD_HEADER(p, 0, sc.sc_session, len);

    return pppoe_output(sc, pb);
}

pub fn pppoe_ifattach_hook(
    arg: &mut Vec<u8>,
    mp: &mut Vec<PacketBuffer>,
    ifp: &mut NetIfc,
    dir: i32,
) {
    let mut sc: &mut pppoe_softc;
    let s: i32;

    if (mp != PFIL_IFNET_DETACH) {
        return 0;
    }

    // LIST_FOREACH(sc, &pppoe_softc_list, sc_list) {
    //   if (sc.sc_ethif != ifp) {
    //     continue;
    //   }
    //   if (sc.sc_sppp.pp_if.if_flags & IFF_UP) {
    //     sc.sc_sppp.pp_if.if_flags &= !(IFF_UP|IFF_RUNNING);
    //     // PPPDEBUG(LOG_DEBUG, ("%c%c%"U16_F": ethernet interface detached, going down\n",
    //     //     sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num));
    //   }
    //   sc.sc_ethif = None;
    //   pppoe_clear_softc(sc, "ethernet interface detached");
    // }

    return 0;
}

pub fn pppoe_clear_softc(sc: &mut pppoe_softc, message: &String) {
    //  stop timer 
    sys_untimeout(pppoe_timeout, sc);
    // PPPDEBUG(LOG_DEBUG, ("pppoe: %c%c%"U16_F": session 0x%x terminated, %s\n", sc.sc_ethif.name[0], sc.sc_ethif.name[1], sc.sc_ethif.num, sc.sc_session, message));
    sc.sc_state = PPPOE_STATE_INITIAL;
    ppp_link_end(sc.pcb); //  notify upper layers - /!\ dangerous /!\ - see pppoe_disc_input() 
}
