/****************************************************************************
* ppp.c - Network Poto: i32 PoProtocol: i32 program file.
*
* Copyright (c) 2003 by Marc Boucher, Services Informatiques (MBSI) inc.
* portions Copyright (c) 1997 by Global Election Systems Inc.
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
* 03-01-01 Marc Boucher <marc@mbsi.ca>
*   Ported to lwIP.
* 97-11-05 Guy Lancaster <lancasterg@acm.org>, Global Election Systems Inc.
*   Original.
*****************************************************************************/

/*
 * ppp_defs.h - PPP definitions.
 *
 * if_pppvar.h - private structures and declarations for PPP.
 *
 * Copyright (c) 1994 The Australian National University.
 * All rights reserved.
 *
 * Permission to use, copy, modify, and distribute this software and its
 * documentation is hereby granted, provided that the above copyright
 * notice appears in all copies.  This software is provided without any
 * warranty, express or implied. The Australian National University
 * makes no representations about the suitability of this software for
 * any purpose.
 *
 * IN NO EVENT SHALL THE AUSTRALIAN NATIONAL UNIVERSITY BE LIABLE TO ANY
 * PARTY FOR DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES
 * ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF
 * THE AUSTRALIAN NATIONAL UNIVERSITY HAVE BEEN ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * THE AUSTRALIAN NATIONAL UNIVERSITY SPECIFICALLY DISCLAIMS ANY WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS FOR A PARTICULAR PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS
 * ON AN "AS IS" BASIS, AND THE AUSTRALIAN NATIONAL UNIVERSITY HAS NO
 * OBLIGATION TO PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS,
 * OR MODIFICATIONS.
 */

/*
 * if_ppp.h - Point-to-PoProtocol: i32 definitions.
 *
 * Copyright (c) 1989 Carnegie Mellon University.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms are permitted
 * provided that the above copyright notice and this paragraph are
 * duplicated in all such forms and that any documentation,
 * advertising materials, and other materials related to such
 * distribution and use acknowledge that the software was developed
 * by Carnegie Mellon University.  The name of the
 * University may not be used to endorse or promote products derived
 * from this software without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */

/*
 * @defgroup ppp PPP
 * @ingroup netifs
 * @verbinclude "ppp.txt"
 */



















































/************************/
/** LOCAL DEFINITIONS ***/
/************************/

/* Memory pools */

LWIP_MEMPOOL_PROTOTYPE(PPPOS_PCB);


LWIP_MEMPOOL_PROTOTYPE(PPPOE_IF);


LWIP_MEMPOOL_PROTOTYPE(PPPOL2TP_PCB);


LWIP_MEMPOOL_PROTOTYPE(PPPAPI_MSG);

LWIP_MEMPOOL_DECLARE(PPP_PCB, MEMP_NUM_PPP_PCB, sizeof(ppp_pcb), "PPP_PCB")

/* FIXME: add stats per PPP session */

static start_time: timeval; /* Time when link was started. */
static struct pppd_stats old_link_stats;
struct pppd_stats link_stats;
 link_connect_time;
link_stats_valid: i32;


/*
 * PPP Data Link Layer "protocol" table.
 * One entry per supported protocol.
 * The last entry must be NULL.
 */
const struct protent* const protocols[] = {
    &lcp_protent,

    &pap_protent,


    &chap_protent,


    &cbcp_protent,


    &ipcp_protent,


    &ipv6cp_protent,


    &ccp_protent,


    &ecp_protent,


    &atcp_protent,


    &eap_protent,

    NULL
};

/* Prototypes for procedures local to this file. */
pub fn ppp_do_connect(arg: &mut Vec<u8>);
static ppp_netif_init_cb: err_t(netif: &mut NetIfc);

static ppp_netif_output_ip4: err_t(netif: &mut NetIfc, pb: &mut pbuf,  ipaddr: &mut ip4_addr);


static ppp_netif_output_ip6: err_t(netif: &mut NetIfc, pb: &mut pbuf,  ipaddr: &mut ip6_addr_t);

static ppp_netif_output: err_t(netif: &mut NetIfc, pb: &mut pbuf, protocol: u16);

/**********************************/
/** PUBLIC FUNCTION DEFINITIONS ***/
/**********************************/

pub fn  ppp_set_auth(pcb: &mut ppp_pcb, authtype: u8, user: &String, passwd: &String) {
  LWIP_ASSERT_CORE_LOCKED();

  pcb.settings.refuse_pap = !(authtype & PPPAUTHTYPE_PAP);


  pcb.settings.refuse_chap = !(authtype & PPPAUTHTYPE_CHAP);

  pcb.settings.refuse_mschap = !(authtype & PPPAUTHTYPE_MSCHAP);
  pcb.settings.refuse_mschap_v2 = !(authtype & PPPAUTHTYPE_MSCHAP_V2);



  pcb.settings.refuse_eap = !(authtype & PPPAUTHTYPE_EAP);

  pcb.settings.user = user;
  pcb.settings.passwd = passwd;
}



/* Set MPPE configuration */
pub fn  ppp_set_mppe(pcb: &mut ppp_pcb, flags: u8) {
  if (flags == PPP_MPPE_DISABLE) {
    pcb.settings.require_mppe = 0;
    return;
  }

  pcb.settings.require_mppe = 1;
  pcb.settings.refuse_mppe_stateful = !(flags & PPP_MPPE_ALLOW_STATEFUL);
  pcb.settings.refuse_mppe_40 = !!(flags & PPP_MPPE_REFUSE_40);
  pcb.settings.refuse_mppe_128 = !!(flags & PPP_MPPE_REFUSE_128);
}



pub fn  ppp_set_notify_phase_callback(pcb: &mut ppp_pcb, ppp_notify_phase_cb_fn notify_phase_cb) {
  pcb.notify_phase_cb = notify_phase_cb;
  notify_phase_cb(pcb, pcb.phase, pcb.ctx_cb);
}


/*
 * Initiate a PPP connection.
 *
 * This can only be called if PPP is in the dead phase.
 *
 * Holdoff is the time to wait (in seconds) before initiating
 * the connection.
 *
 * If this port connects to a modem, the modem connection must be
 * established before calling this.
 */
pub fn  ppp_connect(pcb: &mut ppp_pcb, holdoff: u16) {
  LWIP_ASSERT_CORE_LOCKED();
  if (pcb.phase != PPP_PHASE_DEAD) {
    return ERR_ALREADY;
  }

  PPPDEBUG(LOG_DEBUG, ("ppp_connect[%d]: holdoff=%d\n", pcb.netif.num, holdoff));

  magic_randomize();

  if (holdoff == 0) {
    ppp_do_connect(pcb);
    return ERR_OK;
  }

  new_phase(pcb, PPP_PHASE_HOLDOFF);
  sys_timeout((holdoff*1000), ppp_do_connect, pcb);
  return ERR_OK;
}


/*
 * Listen for an incoming PPP connection.
 *
 * This can only be called if PPP is in the dead phase.
 *
 * If this port connects to a modem, the modem connection must be
 * established before calling this.
 */
pub fn  ppp_listen(pcb: &mut ppp_pcb) {
  LWIP_ASSERT_CORE_LOCKED();
  if (pcb.phase != PPP_PHASE_DEAD) {
    return ERR_ALREADY;
  }

  PPPDEBUG(LOG_DEBUG, ("ppp_listen[%d]\n", pcb.netif.num));

  magic_randomize();

  if (pcb.link_cb.listen) {
    new_phase(pcb, PPP_PHASE_INITIALIZE);
    pcb.link_cb.listen(pcb, pcb.link_ctx_cb);
    return ERR_OK;
  }
  return ERR_IF;
}


/*
 * Initiate the end of a PPP connection.
 * Any outstanding packets in the queues are dropped.
 *
 * Setting nocarrier to 1 close the PPP connection without initiating the
 * shutdown procedure. Always using nocarrier = 0 is still recommended,
 * this is going to take a little longer time if your link is down, but
 * is a safer choice for the PPP state machine.
 *
 * Return 0 on success, an error code on failure.
 */
pub fn 
ppp_close(pcb: &mut ppp_pcb, nocarrier: u8)
{
  LWIP_ASSERT_CORE_LOCKED();

  pcb.err_code = PPPERR_USER;

  /* holdoff phase, cancel the reconnection */
  if (pcb.phase == PPP_PHASE_HOLDOFF) {
    sys_untimeout(ppp_do_connect, pcb);
    new_phase(pcb, PPP_PHASE_DEAD);
  }

  /* dead phase, nothing to do, call the status callback to be consistent */
  if (pcb.phase == PPP_PHASE_DEAD) {
    pcb.link_status_cb(pcb, pcb.err_code, pcb.ctx_cb);
    return ERR_OK;
  }

  /* Already terminating, nothing to do */
  if (pcb.phase >= PPP_PHASE_TERMINATE) {
    return ERR_INPROGRESS;
  }

  /* LCP not open, close link protocol */
  if (pcb.phase < PPP_PHASE_ESTABLISH) {
    new_phase(pcb, PPP_PHASE_DISCONNECT);
    ppp_link_terminated(pcb);
    return ERR_OK;
  }

  /*
   * Only accept carrier lost signal on the stable running phase in order
   * to prevent changing the PPP phase FSM in transition phases.
   *
   * Always using nocarrier = 0 is still recommended, this is going to
   * take a little longer time, but is a safer choice from FSM poof: i32 view.
   */
  if (nocarrier && pcb.phase == PPP_PHASE_RUNNING) {
    PPPDEBUG(LOG_DEBUG, ("ppp_close[%d]: carrier lost -> lcp_lowerdown\n", pcb.netif.num));
    lcp_lowerdown(pcb);
    /* forced link termination, this will force link protocol to disconnect. */
    link_terminated(pcb);
    return ERR_OK;
  }

  /* Disconnect */
  PPPDEBUG(LOG_DEBUG, ("ppp_close[%d]: kill_link -> lcp_close\n", pcb.netif.num));
  /* LCP soft close request. */
  lcp_close(pcb, "User request");
  return ERR_OK;
}

/*
 * Release the control block.
 *
 * This can only be called if PPP is in the dead phase.
 *
 * You must use ppp_close() before if you wish to terminate
 * an established PPP session.
 *
 * Return 0 on success, an error code on failure.
 */
pub fn  ppp_free(pcb: &mut ppp_pcb) {
  let err: err_t;
  LWIP_ASSERT_CORE_LOCKED();
  if (pcb.phase != PPP_PHASE_DEAD) {
    return ERR_CONN;
  }

  PPPDEBUG(LOG_DEBUG, ("ppp_free[%d]\n", pcb.netif.num));

  netif_remove(pcb.netif);

  err = pcb.link_cb.free(pcb, pcb.link_ctx_cb);

  LWIP_MEMPOOL_FREE(PPP_PCB, pcb);
  return err;
}

/* Get and set parameters for the given connection.
 * Return 0 on success, an error code on failure. */
pub fn 
ppp_ioctl(pcb: &mut ppp_pcb, cmd: u8, arg: &mut Vec<u8>)
{
  LWIP_ASSERT_CORE_LOCKED();
  if (pcb == NULL) {
    return ERR_VAL;
  }

  match(cmd) {
    PPPCTLG_UPSTATUS =>      /* Get the PPP up status. */
      if (!arg) {
        // goto fail;
      }
      arg = (0

           || pcb.if4_up


           || pcb.if6_up

           );
      return ERR_OK;

    PPPCTLG_ERRCODE =>       /* Get the PPP error code. */
      if (!arg) {
        // goto fail;
      }
      arg = (pcb.err_code);
      return ERR_OK;

    _ =>
      // goto fail;
  }

fail:
  return ERR_VAL;
}


/*********************************/
/** LOCAL FUNCTION DEFINITIONS ***/
/*********************************/

pub fn ppp_do_connect(arg: &mut Vec<u8>) {
  pcb: &mut ppp_pcb = arg;

  LWIP_ASSERT("pcb.phase == PPP_PHASE_DEAD || pcb.phase == PPP_PHASE_HOLDOFF", pcb.phase == PPP_PHASE_DEAD || pcb.phase == PPP_PHASE_HOLDOFF);

  new_phase(pcb, PPP_PHASE_INITIALIZE);
  pcb.link_cb.connect(pcb, pcb.link_ctx_cb);
}

/*
 * ppp_netif_init_cb - netif init callback
 */
static ppp_netif_init_cb: err_t(netif: &mut NetIfc) {
  netif.name[0] = 'p';
  netif.name[1] = 'p';

  netif.output = ppp_netif_output_ip4;


  netif.output_ip6 = ppp_netif_output_ip6;

  netif.flags = NETIF_FLAG_UP;

  /* @todo: Initialize interface hostname */
  /* netif_set_hostname(netif, "lwip"); */

  return ERR_OK;
}


/*
 * Send an IPv4 packet on the given connection.
 */
static ppp_netif_output_ip4: err_t(netif: &mut NetIfc, pb: &mut pbuf,  ipaddr: &mut ip4_addr) {
  
  return ppp_netif_output(netif, pb, PPP_IP);
}



/*
 * Send an IPv6 packet on the given connection.
 */
static ppp_netif_output_ip6: err_t(netif: &mut NetIfc, pb: &mut pbuf,  ipaddr: &mut ip6_addr_t) {
  
  return ppp_netif_output(netif, pb, PPP_IPV6);
}


static ppp_netif_output: err_t(netif: &mut NetIfc, pb: &mut pbuf, protocol: u16) {
  pcb: &mut ppp_pcb = netif.state;
  let err: err_t;
  fpb: &mut pbuf = NULL;

  /* Check that the link is up. */
  if (0

      || (protocol == PPP_IP && !pcb.if4_up)


      || (protocol == PPP_IPV6 && !pcb.if6_up)

      ) {
    PPPDEBUG(LOG_ERR, ("ppp_netif_output[%d]: link not up\n", pcb.netif.num));
    // goto err_rte_drop;
  }


  /* If MPPE is required, refuse any IP packet until we are able to crypt them. */
  if (pcb.settings.require_mppe && pcb.ccp_transmit_method != CI_MPPE) {
    PPPDEBUG(LOG_ERR, ("ppp_netif_output[%d]: MPPE required, not up\n", pcb.netif.num));
    // goto err_rte_drop;
  }



  /*
   * Attempt Van Jacobson header compression if VJ is configured and
   * this is an IP packet.
   */
  if (protocol == PPP_IP && pcb.vj_enabled) {
    match (vj_compress_tcp(&pcb.vj_comp, &pb)) {
      TYPE_IP =>
        /* No change...
           protocol = PPP_IP; */
        break;
      TYPE_COMPRESSED_TCP =>
        /* vj_compress_tcp() returns a new allocated pbuf, indicate we should free
         * our duplicated pbuf later */
        fpb = pb;
        protocol = PPP_VJC_COMP;
        break;
      TYPE_UNCOMPRESSED_TCP =>
        /* vj_compress_tcp() returns a new allocated pbuf, indicate we should free
         * our duplicated pbuf later */
        fpb = pb;
        protocol = PPP_VJC_UNCOMP;
        break;
      _ =>
        PPPDEBUG(LOG_WARNING, ("ppp_netif_output[%d]: bad IP packet\n", pcb.netif.num));
        LINK_STATS_INC(link.proterr);
        LINK_STATS_INC(link.drop);
        MIB2_STATS_NETIF_INC(pcb.netif, ifoutdiscards);
        return ERR_VAL;
    }
  }



  match (pcb.ccp_transmit_method) {
  0 =>
    break; /* Don't compress */

  CI_MPPE =>
    if ((err = mppe_compress(pcb, &pcb.mppe_comp, &pb, protocol)) != ERR_OK) {
      LINK_STATS_INC(link.memerr);
      LINK_STATS_INC(link.drop);
      MIB2_STATS_NETIF_INC(netif, ifoutdiscards);
      // goto err;
    }
    /* if VJ compressor returned a new allocated pbuf, free it */
    if (fpb) {
      pbuf_free(fpb);
    }
    /* mppe_compress() returns a new allocated pbuf, indicate we should free
     * our duplicated pbuf later */
    fpb = pb;
    protocol = PPP_COMP;
    break;

  _ =>
    PPPDEBUG(LOG_ERR, ("ppp_netif_output[%d]: bad CCP transmit method\n", pcb.netif.num));
    // goto err_rte_drop; /* Cannot really happen, we only negotiate what we are able to do */
  }


  err = pcb.link_cb.netif_output(pcb, pcb.link_ctx_cb, pb, protocol);
  // goto err;

err_rte_drop:
  err = ERR_RTE;
  LINK_STATS_INC(link.rterr);
  LINK_STATS_INC(link.drop);
  MIB2_STATS_NETIF_INC(netif, ifoutdiscards);
err:
  if (fpb) {
    pbuf_free(fpb);
  }
  return err;
}

/***********************************/
/** PRIVATE FUNCTION DEFINITIONS ***/
/***********************************/

/* Initialize the PPP subsystem. */
ppp_init: i32()
{

  LWIP_MEMPOOL_INIT(PPPOS_PCB);


  LWIP_MEMPOOL_INIT(PPPOE_IF);


  LWIP_MEMPOOL_INIT(PPPOL2TP_PCB);


  LWIP_MEMPOOL_INIT(PPPAPI_MSG);


  LWIP_MEMPOOL_INIT(PPP_PCB);

  /*
   * Initialize magic number generator now so that protocols may
   * use magic numbers in initialization.
   */
  magic_init();

  return 0;
}
 
/*
 * Create a new PPP control block.
 *
 * This initializes the PPP control block but does not
 * attempt to negotiate the LCP session.
 *
 * Return a new PPP connection control block pointer
 * on success or a null pointer on failure.
 */
ppp_new: &mut ppp_pcb(pppif: &mut NetIfc,  callbacks: &mut link_callbacks, link_ctx_cb: &mut (), ppp_link_status_cb_fn link_status_cb, ctx_cb: &mut ()) {
  pcb: &mut ppp_pcb;
  const protp: &mut protent;
  let leti: i32;

  /* PPP is single-threaded: without a callback,
   * there is no way to know when the link is up. */
  if (link_status_cb == NULL) {
    return NULL;
  }

  pcb = LWIP_MEMPOOL_ALLOC(PPP_PCB);
  if (pcb == NULL) {
    return NULL;
  }

  //memset(pcb, 0, sizeof(ppp_pcb));

  /* default configuration */

  pcb.settings.pap_timeout_time = UPAP_DEFTIMEOUT;
  pcb.settings.pap_max_transmits = UPAP_DEFTRANSMITS;

  pcb.settings.pap_req_timeout = UPAP_DEFREQTIME;




  pcb.settings.chap_timeout_time = CHAP_DEFTIMEOUT;
  pcb.settings.chap_max_transmits = CHAP_DEFTRANSMITS;

  pcb.settings.chap_rechallenge_time = CHAP_DEFRECHALLENGETIME;




  pcb.settings.eap_req_time = EAP_DEFREQTIME;
  pcb.settings.eap_allow_req = EAP_DEFALLOWREQ;

  pcb.settings.eap_timeout_time = EAP_DEFTIMEOUT;
  pcb.settings.eap_max_transmits = EAP_DEFTRANSMITS;



  pcb.settings.lcp_loopbackfail = LCP_DEFLOOPBACKFAIL;
  pcb.settings.lcp_echo_interval = LCP_ECHOINTERVAL;
  pcb.settings.lcp_echo_fails = LCP_MAXECHOFAILS;

  pcb.settings.fsm_timeout_time = FSM_DEFTIMEOUT;
  pcb.settings.fsm_max_conf_req_transmits = FSM_DEFMAXCONFREQS;
  pcb.settings.fsm_max_term_transmits = FSM_DEFMAXTERMREQS;
  pcb.settings.fsm_max_nak_loops = FSM_DEFMAXNAKLOOPS;

  pcb.netif = pppif;
  MIB2_INIT_NETIF(pppif, snmp_ifType_ppp, 0);
  if (!netif_add(pcb.netif,

                 IP4_ADDR_ANY4, IP4_ADDR_BROADCAST, IP4_ADDR_ANY4,

                 pcb, ppp_netif_init_cb, NULL)) {
    LWIP_MEMPOOL_FREE(PPP_PCB, pcb);
    PPPDEBUG(LOG_ERR, ("ppp_new: netif_add failed\n"));
    return NULL;
  }

  pcb.link_cb = callbacks;
  pcb.link_ctx_cb = link_ctx_cb;
  pcb.link_status_cb = link_status_cb;
  pcb.ctx_cb = ctx_cb;

  /*
   * Initialize each protocol.
   */
  for (i = 0; (protp = protocols[i]) != NULL; += 1i) {
      (*protp.init)(pcb);
  }

  new_phase(pcb, PPP_PHASE_DEAD);
  return pcb;
}

/* Initiate LCP open request */
pub fn  ppp_start(pcb: &mut ppp_pcb) {
  PPPDEBUG(LOG_DEBUG, ("ppp_start[%d]\n", pcb.netif.num));

  /* Clean data not taken care by anything else, mostly shared data. */

  link_stats_valid = 0;


  pcb.mppe_keys_set = 0;
  //memset(&pcb.mppe_comp, 0, sizeof(pcb.mppe_comp));
  //memset(&pcb.mppe_decomp, 0, sizeof(pcb.mppe_decomp));


  vj_compress_init(&pcb.vj_comp);


  /* Start protocol */
  new_phase(pcb, PPP_PHASE_ESTABLISH);
  lcp_open(pcb);
  lcp_lowerup(pcb);
  PPPDEBUG(LOG_DEBUG, ("ppp_start[%d]: finished\n", pcb.netif.num));
}

/* Called when link failed to setup */
pub fn  ppp_link_failed(pcb: &mut ppp_pcb) {
  PPPDEBUG(LOG_DEBUG, ("ppp_link_failed[%d]\n", pcb.netif.num));
  new_phase(pcb, PPP_PHASE_DEAD);
  pcb.err_code = PPPERR_OPEN;
  pcb.link_status_cb(pcb, pcb.err_code, pcb.ctx_cb);
}

/* Called when link is normally down (i.e. it was asked to end) */
pub fn  ppp_link_end(pcb: &mut ppp_pcb) {
  PPPDEBUG(LOG_DEBUG, ("ppp_link_end[%d]\n", pcb.netif.num));
  new_phase(pcb, PPP_PHASE_DEAD);
  if (pcb.err_code == PPPERR_NONE) {
    pcb.err_code = PPPERR_CONNECT;
  }
  pcb.link_status_cb(pcb, pcb.err_code, pcb.ctx_cb);
}

/*
 * Pass the processed input packet to the appropriate handler.
 * This function and all handlers run in the context of the tcpip_thread
 */
pub fn  ppp_input(pcb: &mut ppp_pcb, pb: &mut pbuf) {
  let protocol: u16;

    let pname: String;


  magic_randomize();

  if (pb.len < 2) {
    PPPDEBUG(LOG_ERR, ("ppp_input[%d]: packet too short\n", pcb.netif.num));
    // goto drop;
  }
  protocol = ((pb.payload)[0] << 8) | (pb.payload)[1];


  ppp_dump_packet(pcb, "rcvd", pb.payload, pb.len);


  pbuf_remove_header(pb, sizeof(protocol));

  LINK_STATS_INC(link.recv);
  MIB2_STATS_NETIF_INC(pcb.netif, ifinucastpkts);
  MIB2_STATS_NETIF_ADD(pcb.netif, ifinoctets, pb.tot_len);

  /*
   * Toss all non-LCP packets unless LCP is OPEN.
   */
  if (protocol != PPP_LCP && pcb.lcp_fsm.state != PPP_FSM_OPENED) {
    ppp_dbglog("Discarded non-LCP packet when LCP not open");
    // goto drop;
  }

  /*
   * Until we get past the authentication phase, toss all packets
   * except LCP, LQR and authentication packets.
   */
  if (pcb.phase <= PPP_PHASE_AUTHENTICATE
   && !(protocol == PPP_LCP

   || protocol == PPP_LQR


   || protocol == PPP_PAP


   || protocol == PPP_CHAP


   || protocol == PPP_EAP

   )) {
    ppp_dbglog("discarding proto 0x%x in phase %d", protocol, pcb.phase);
    // goto drop;
  }



  /*
   * MPPE is required and unencrypted data has arrived (this
   * should never happen!). We should probably drop the link if
   * the protocol is in the range of what should be encrypted.
   * At the least, we drop this packet.
   */
  if (pcb.settings.require_mppe && protocol != PPP_COMP && protocol < 0x8000) {
    PPPDEBUG(LOG_ERR, ("ppp_input[%d]: MPPE required, received unencrypted data!\n", pcb.netif.num));
    // goto drop;
  }


  if (protocol == PPP_COMP) {
    pl: &mut Vec<u8>;

    match (pcb.ccp_receive_method) {

    CI_MPPE =>
      if (mppe_decompress(pcb, &pcb.mppe_decomp, &pb) != ERR_OK) {
        // goto drop;
      }
      break;

    _ =>
      PPPDEBUG(LOG_ERR, ("ppp_input[%d]: bad CCP receive method\n", pcb.netif.num));
      // goto drop; /* Cannot really happen, we only negotiate what we are able to do */
    }

    /* Assume no PFC */
    if (pb.len < 2) {
      // goto drop;
    }

    /* Extract and hide protocol (do PFC decompression if necessary) */
    pl = pb.payload;
    if (pl[0] & 0x01) {
      protocol = pl[0];
      pbuf_remove_header(pb, 1);
    } else {
      protocol = (pl[0] << 8) | pl[1];
      pbuf_remove_header(pb, 2);
    }
  }


  match(protocol) {


    PPP_IP =>            /* Internet Protocol */
      PPPDEBUG(LOG_INFO, ("ppp_input[%d]: ip in pbuf len=%d\n", pcb.netif.num, pb.tot_len));
      ip4_input(pb, pcb.netif);
      return;



    PPP_IPV6 =>          /* Internet Protocol Version 6 */
      PPPDEBUG(LOG_INFO, ("ppp_input[%d]: ip6 in pbuf len=%d\n", pcb.netif.num, pb.tot_len));
      ip6_input(pb, pcb.netif);
      return;



    PPP_VJC_COMP =>      /* VJ compressed TCP */
      /*
       * Clip off the VJ header and prepend the rebuilt TCP/IP header and
       * pass the result to IP.
       */
      PPPDEBUG(LOG_INFO, ("ppp_input[%d]: vj_comp in pbuf len=%d\n", pcb.netif.num, pb.tot_len));
      if (pcb.vj_enabled && vj_uncompress_tcp(&pb, &pcb.vj_comp) >= 0) {
        ip4_input(pb, pcb.netif);
        return;
      }
      /* Something's wrong so drop it. */
      PPPDEBUG(LOG_WARNING, ("ppp_input[%d]: Dropping VJ compressed\n", pcb.netif.num));
      break;

    PPP_VJC_UNCOMP =>    /* VJ uncompressed TCP */
      /*
       * Process the TCP/IP header for VJ header compression and then pass
       * the packet to IP.
       */
      PPPDEBUG(LOG_INFO, ("ppp_input[%d]: vj_un in pbuf len=%d\n", pcb.netif.num, pb.tot_len));
      if (pcb.vj_enabled && vj_uncompress_uncomp(pb, &pcb.vj_comp) >= 0) {
        ip4_input(pb, pcb.netif);
        return;
      }
      /* Something's wrong so drop it. */
      PPPDEBUG(LOG_WARNING, ("ppp_input[%d]: Dropping VJ uncompressed\n", pcb.netif.num));
      break;


    _ => {
      let leti: i32;
      const protp: &mut protent;

      /*
       * Upcall the proper protocol input routine.
       */
      for (i = 0; (protp = protocols[i]) != NULL; += 1i) {
        if (protp.protocol == protocol) {
          pb = pbuf_coalesce(pb, PBUF_RAW);
          (*protp.input)(pcb, pb.payload, pb.len);
          // goto out;
        }

         *
         * This is actually a (hacked?) way for the Linux kernel to pass a data
         * packet to pppd. pppd in normal condition only do signaling
         * (LCP, PAP, CHAP, IPCP, ...) and does not handle any data packet at all.
         *
         * We don't even need this interface, which is only there because of PPP
         * interface limitation between Linux kernel and pppd. For MPPE, which uses
         * CCP to negotiate although it is not really a (de)compressor, we added
         * ccp_resetrequest() in CCP and MPPE input data flow is calling either
         * ccp_resetrequest() or lcp_close() if the issue is, respectively, non-fatal
         * or fatal, this is what ccp_datainput() really do.
         */
        if (protocol == (protp.protocol & !0x8000)
          && protp.datainput != NULL) {
          (*protp.datainput)(pcb, pb.payload, pb.len);
          // goto out;
        }

      }



      pname = protocol_name(protocol);
      if (pname != NULL) {
        ppp_warn("Unsupported protocol '%s' (0x%x) received", pname, protocol);
      } else

        ppp_warn("Unsupported protocol 0x%x received", protocol);

        if (pbuf_add_header(pb, sizeof(protocol))) {
          PPPDEBUG(LOG_WARNING, ("ppp_input[%d]: Dropping (pbuf_add_header failed)\n", pcb.netif.num));
          // goto drop;
        }
        lcp_sprotrej(pcb, pb.payload, pb.len);
      }
      break;
  }

drop:
  LINK_STATS_INC(link.drop);
  MIB2_STATS_NETIF_INC(pcb.netif, ifindiscards);

out:
  pbuf_free(pb);
}

/*
 * Write a pbuf to a ppp link, only used from PPP functions
 * to send PPP packets.
 *
 * IPv4 and IPv6 packets from lwIP are sent, respectively,
 * with ppp_netif_output_ip4() and ppp_netif_output_ip6()
 * functions (which are callbacks of the netif PPP interface).
 */
pub fn  ppp_write(pcb: &mut ppp_pcb, p: &mut pbuf) {

  ppp_dump_packet(pcb, "sent", p.payload+2, p.len-2);

  return pcb.link_cb.write(pcb, pcb.link_ctx_cb, p);
}

pub fn  ppp_link_terminated(pcb: &mut ppp_pcb) {
  PPPDEBUG(LOG_DEBUG, ("ppp_link_terminated[%d]\n", pcb.netif.num));
  pcb.link_cb.disconnect(pcb, pcb.link_ctx_cb);
  PPPDEBUG(LOG_DEBUG, ("ppp_link_terminated[%d]: finished.\n", pcb.netif.num));
}


/***********************************************************************
 * Functions called by various PPP subsystems to configure
 * the PPP interface or change the PPP phase.
 */

/*
 * new_phase - signal the start of a new phase of pppd's operation.
 */
pub fn  new_phase(pcb: &mut ppp_pcb, p: i32) {
  pcb.phase = p;
  PPPDEBUG(LOG_DEBUG, ("ppp phase changed[%d]: phase=%d\n", pcb.netif.num, pcb.phase));

  if (pcb.notify_phase_cb != NULL) {
    pcb.notify_phase_cb(pcb, p, pcb.ctx_cb);
  }

}

/*
 * ppp_send_config - configure the transmit-side characteristics of
 * the ppp interface.
 */
ppp_send_config: i32(pcb: &mut ppp_pcb, mtu: i32, accm: u32, pcomp: i32, accomp: i32) {
  
  /* pcb.mtu = mtu; -- set correctly with netif_set_mtu */

  if (pcb.link_cb.send_config) {
    pcb.link_cb.send_config(pcb, pcb.link_ctx_cb, accm, pcomp, accomp);
  }

  PPPDEBUG(LOG_INFO, ("ppp_send_config[%d]\n", pcb.netif.num) );
  return 0;
}

/*
 * ppp_recv_config - configure the receive-side characteristics of
 * the ppp interface.
 */
ppp_recv_config: i32(pcb: &mut ppp_pcb, mru: i32, accm: u32, pcomp: i32, accomp: i32) {
  

  if (pcb.link_cb.recv_config) {
    pcb.link_cb.recv_config(pcb, pcb.link_ctx_cb, accm, pcomp, accomp);
  }

  PPPDEBUG(LOG_INFO, ("ppp_recv_config[%d]\n", pcb.netif.num));
  return 0;
}


/*
 * sifaddr - Config the interface IP addresses and netmask.
 */
sifaddr: i32(pcb: &mut ppp_pcb, our_adr: u32, his_adr: u32, netmask: u32) {
  ip4_addr ip, nm, gw;

  ip4_addr_set_u32(&ip, our_adr);
  ip4_addr_set_u32(&nm, netmask);
  ip4_addr_set_u32(&gw, his_adr);
  netif_set_addr(pcb.netif, &ip, &nm, &gw);
  return 1;
}

/*******************************************************************
 *
 * cifaddr - Clear the interface IP addresses, and delete routes
 * through the interface if possible.
 */
cifaddr: i32(pcb: &mut ppp_pcb, our_adr: u32, his_adr: u32) {
  
  

  netif_set_addr(pcb.netif, IP4_ADDR_ANY4, IP4_ADDR_BROADCAST, IP4_ADDR_ANY4);
  return 1;
}


/*******************************************************************
 *
 * sifproxyarp - Make a proxy ARP entry for the peer.
 */

sifproxyarp: i32(pcb: &mut ppp_pcb, his_adr: u32) {
  
  
  return 0;
}

/*******************************************************************
 *
 * cifproxyarp - Delete the proxy ARP entry for the peer.
 */

cifproxyarp: i32(pcb: &mut ppp_pcb, his_adr: u32) {
  
  
  return 0;
}



/*
 * sdns - Config the DNS servers
 */
sdns: i32(pcb: &mut ppp_pcb, ns1: u32, ns2: u32) {
  let ns: LwipAddr;
  

  ip_addr_set_ip4_u32_val(ns, ns1);
  dns_setserver(0, &ns);
  ip_addr_set_ip4_u32_val(ns, ns2);
  dns_setserver(1, &ns);
  return 1;
}

/*******************************************************************
 *
 * cdns - Clear the DNS servers
 */
cdns: i32(pcb: &mut ppp_pcb, ns1: u32, ns2: u32) {
  const nsa: &mut LwipAddr;
  let nsb: LwipAddr;
  

  nsa = dns_getserver(0);
  ip_addr_set_ip4_u32_val(nsb, ns1);
  if (ip_addr_cmp(nsa, &nsb)) {
    dns_setserver(0, IP_ADDR_ANY);
  }
  nsa = dns_getserver(1);
  ip_addr_set_ip4_u32_val(nsb, ns2);
  if (ip_addr_cmp(nsa, &nsb)) {
    dns_setserver(1, IP_ADDR_ANY);
  }
  return 1;
}



/*******************************************************************
 *
 * sifvjcomp - config tcp header compression
 */
sifvjcomp: i32(pcb: &mut ppp_pcb, vjcomp: i32, cidcomp: i32, maxcid: i32) {
  pcb.vj_enabled = vjcomp;
  pcb.vj_comp.compressSlot = cidcomp;
  pcb.vj_comp.maxSlotIndex = maxcid;
  PPPDEBUG(LOG_INFO, ("sifvjcomp[%d]: VJ compress enable=%d slot=%d max slot=%d\n",
            pcb.netif.num, vjcomp, cidcomp, maxcid));
  return 0;
}


/*
 * sifup - Config the interface up and enable IP packets to pass.
 */
sifup: i32(pcb: &mut ppp_pcb) {
  pcb.if4_up = 1;
  pcb.err_code = PPPERR_NONE;
  netif_set_link_up(pcb.netif);

  PPPDEBUG(LOG_DEBUG, ("sifup[%d]: err_code=%d\n", pcb.netif.num, pcb.err_code));
  pcb.link_status_cb(pcb, pcb.err_code, pcb.ctx_cb);
  return 1;
}

/*******************************************************************
 *
 * sifdown - Disable the indicated protocol and config the interface
 *           down if there are no remaining protocols.
 */
sifdown: i32(pcb: &mut ppp_pcb) {

  pcb.if4_up = 0;

  if (1

   /* set the interface down if IPv6 is down as well */
   && !pcb.if6_up

  ) {
    /* make sure the netif link callback is called */
    netif_set_link_down(pcb.netif);
  }
  PPPDEBUG(LOG_DEBUG, ("sifdown[%d]: err_code=%d\n", pcb.netif.num, pcb.err_code));
  return 1;
}

/*******************************************************************
 *
 * Return user specified netmask, modified by any mask we might determine
 * for address `addr' (in network byte order).
 * Here we scan through the system's list of interfaces, looking for
 * any non-point-to-pointerfaces: i32 which might appear to be on the same
 * network as `addr'.  If we find any, we OR in their netmask to the
 * user-specified netmask.
 */
get_mask: u32(addr: u32) {

  mask: u32, nmask;

  addr = lwip_htonl(addr);
  if (IP_CLASSA(addr)) { /* determine network mask for address class */
    nmask = IP_CLASSA_NET;
  } else if (IP_CLASSB(addr)) {
    nmask = IP_CLASSB_NET;
  } else {
    nmask = IP_CLASSC_NET;
  }

  /* class D nets are disallowed by bad_ip_adrs */
  mask = PP_HTONL(0xffffff00) | lwip_htonl(nmask);

  /* XXX
   * Scan through the system's network interfaces.
   * Get each netmask and OR them into our mask.
   */
  /* return mask; */
  return mask;

  
  return IPADDR_BROADCAST;
}



#define IN6_LLADDR_FROM_EUI64(ip6, eui64) loop {    \
  ip6.addr[0] = PP_HTONL(0xfe800000);             \
  ip6.addr[1] = 0;                                \
  eui64_copy(eui64, ip6.addr[2]);                 \
  } while (0)

/*******************************************************************
 *
 * sif6addr - Config the interface with an IPv6 link-local address
 */
sif6addr: i32(pcb: &mut ppp_pcb, eui64_t our_eui64, eui64_t his_eui64) {
  let ip6: ip6_addr_t;
  

  IN6_LLADDR_FROM_EUI64(ip6, our_eui64);
  netif_ip6_addr_set(pcb.netif, 0, &ip6);
  netif_ip6_addr_set_state(pcb.netif, 0, IP6_ADDR_PREFERRED);
  /* FIXME: should we add an IPv6 static neighbor using his_eui64 ? */
  return 1;
}

/*******************************************************************
 *
 * cif6addr - Remove IPv6 address from interface
 */
cif6addr: i32(pcb: &mut ppp_pcb, eui64_t our_eui64, eui64_t his_eui64) {
  
  

  netif_ip6_addr_set_state(pcb.netif, 0, IP6_ADDR_INVALID);
  netif_ip6_addr_set(pcb.netif, 0, IP6_ADDR_ANY6);
  return 1;
}

/*
 * sif6up - Config the interface up and enable IPv6 packets to pass.
 */
sif6up: i32(pcb: &mut ppp_pcb) {

  pcb.if6_up = 1;
  pcb.err_code = PPPERR_NONE;
  netif_set_link_up(pcb.netif);

  PPPDEBUG(LOG_DEBUG, ("sif6up[%d]: err_code=%d\n", pcb.netif.num, pcb.err_code));
  pcb.link_status_cb(pcb, pcb.err_code, pcb.ctx_cb);
  return 1;
}

/*******************************************************************
 *
 * sif6down - Disable the indicated protocol and config the interface
 *            down if there are no remaining protocols.
 */
sif6down: i32(pcb: &mut ppp_pcb) {

  pcb.if6_up = 0;

  if (1

   /* set the interface down if IPv4 is down as well */
   && !pcb.if4_up

  ) {
    /* make sure the netif link callback is called */
    netif_set_link_down(pcb.netif);
  }
  PPPDEBUG(LOG_DEBUG, ("sif6down[%d]: err_code=%d\n", pcb.netif.num, pcb.err_code));
  return 1;
}



/*
 * sifnpmode - Set the mode for handling packets for a given NP.
 */
sifnpmode: i32(pcb: &mut ppp_pcb, proto: i32, mode: NPmode) {
  
  
  
  return 0;
}


/*
 * netif_set_mtu - set the MTU on the PPP network interface.
 */
pub fn  netif_set_mtu(pcb: &mut ppp_pcb, mtu: i32) {

  pcb.netif.mtu = mtu;
  PPPDEBUG(LOG_INFO, ("netif_set_mtu[%d]: mtu=%d\n", pcb.netif.num, mtu));
}

/*
 * netif_get_mtu - get PPP interface MTU
 */
netif_get_mtu: i32(pcb: &mut ppp_pcb) {

  return pcb.netif.mtu;
}



/*
 * ccp_test - whether a given compression method is acceptable for use.
 */
pub fn ccp_test(pcb: &mut ppp_pcb, u_opt_ptr: &mut String, opt_len: i32, for_transmit: i32)
{
  
  
  
  
  return -1;
}


/*
 * ccp_set - inform about the current state of CCP.
 */
pub fn 
ccp_set(pcb: &mut ppp_pcb, isopen: u8, isup: u8, receive_method: u8, transmit_method: u8)
{
  
  
  pcb.ccp_receive_method = receive_method;
  pcb.ccp_transmit_method = transmit_method;
  PPPDEBUG(LOG_DEBUG, ("ccp_set[%d]: is_open=%d, is_up=%d, receive_method=%u, transmit_method=%u\n",
           pcb.netif.num, isopen, isup, receive_method, transmit_method));
}

pub fn 
ccp_reset_comp(pcb: &mut ppp_pcb)
{
  match (pcb.ccp_transmit_method) {

  CI_MPPE =>
    mppe_comp_reset(pcb, &pcb.mppe_comp);
    break;

  _ =>
    break;
  }
}

pub fn 
ccp_reset_decomp(pcb: &mut ppp_pcb)
{
  match (pcb.ccp_receive_method) {

  CI_MPPE =>
    mppe_decomp_reset(pcb, &pcb.mppe_decomp);
    break;

  _ =>
    break;
  }
}


/*
 * ccp_fatal_error - returns 1 if decompression was disabled as a
 * result of an error detected after decompression of a packet,
 * 0 otherwise.  This is necessary because of patent nonsense.
 */
pub fn ccp_fatal_error(pcb: &mut ppp_pcb)
{
  
  return 1;
}




/*******************************************************************
 *
 * get_idle_time - return how long the link has been idle.
 */
get_idle_time: i32(pcb: &mut ppp_pcb, ip: &mut ppp_idle) {
  /* FIXME: add idle time support and make it optional */
  
  
  return 1;
}



/*******************************************************************
 *
 * get_loop_output - get outgoing packets from the ppp device,
 * and detect when we want to bring the real link up.
 * Return value is 1 if we need to bring up the link, 0 otherwise.
 */
get_loop_output: i32() {
  return 0;
}



/* List of protocol names, to make our messages a little more informative. */
struct protocol_list {
  u_short proto;
  let name: String;
} const protocol_list[] = {
  { 0x21, "IP" },
  { 0x23, "OSI Network Layer" },
  { 0x25, "Xerox NS IDP" },
  { 0x27, "DECnet Phase IV" },
  { 0x29, "Appletalk" },
  { 0x2b, "Novell IPX" },
  { 0x2d, "VJ compressed TCP/IP" },
  { 0x2f, "VJ uncompressed TCP/IP" },
  { 0x31, "Bridging PDU" },
  { 0x33, "Stream Protocol ST-II" },
  { 0x35, "Banyan Vines" },
  { 0x39, "AppleTalk EDDP" },
  { 0x3b, "AppleTalk SmartBuffered" },
  { 0x3d, "Multi-Link" },
  { 0x3f, "NETBIOS Framing" },
  { 0x41, "Cisco Systems" },
  { 0x43, "Ascom Timeplex" },
  { 0x45, "Fujitsu Link Backup and Load Balancing (LBLB)" },
  { 0x47, "DCA Remote Lan" },
  { 0x49, "Serial Data Transport Protocol (PPP-SDTP)" },
  { 0x4b, "SNA over 802.2" },
  { 0x4d, "SNA" },
  { 0x4f, "IP6 Header Compression" },
  { 0x51, "KNX Bridging Data" },
  { 0x53, "Encryption" },
  { 0x55, "Individual Link Encryption" },
  { 0x57, "IPv6" },
  { 0x59, "PPP Muxing" },
  { 0x5b, "Vendor-Specific Network Protocol" },
  { 0x61, "RTP IPHC Full Header" },
  { 0x63, "RTP IPHC Compressed TCP" },
  { 0x65, "RTP IPHC Compressed non-TCP" },
  { 0x67, "RTP IPHC Compressed UDP 8" },
  { 0x69, "RTP IPHC Compressed RTP 8" },
  { 0x6f, "Stampede Bridging" },
  { 0x73, "MP+" },
  { 0xc1, "NTCITS IPI" },
  { 0xfb, "single-link compression" },
  { 0xfd, "Compressed Datagram" },
  { 0x0201, "802.1d Hello Packets" },
  { 0x0203, "IBM Source Routing BPDU" },
  { 0x0205, "DEC LANBridge100 Spanning Tree" },
  { 0x0207, "Cisco Discovery Protocol" },
  { 0x0209, "Netcs Twin Routing" },
  { 0x020b, "STP - Scheduled Transfer Protocol" },
  { 0x020d, "EDP - Extreme Discovery Protocol" },
  { 0x0211, "Optical Supervisory Channel Protocol" },
  { 0x0213, "Optical Supervisory Channel Protocol" },
  { 0x0231, "Luxcom" },
  { 0x0233, "Sigma Network Systems" },
  { 0x0235, "Apple Client Server Protocol" },
  { 0x0281, "MPLS Unicast" },
  { 0x0283, "MPLS Multicast" },
  { 0x0285, "IEEE p1284.4 standard - data packets" },
  { 0x0287, "ETSI TETRA Network Protocol Type 1" },
  { 0x0289, "Multichannel Flow Treatment Protocol" },
  { 0x2063, "RTP IPHC Compressed TCP No Delta" },
  { 0x2065, "RTP IPHC Context State" },
  { 0x2067, "RTP IPHC Compressed UDP 16" },
  { 0x2069, "RTP IPHC Compressed RTP 16" },
  { 0x4001, "Cray Communications Control Protocol" },
  { 0x4003, "CDPD Mobile Network Registration Protocol" },
  { 0x4005, "Expand accelerator protocol" },
  { 0x4007, "ODSICP NCP" },
  { 0x4009, "DOCSIS DLL" },
  { 0x400B, "Cetacean Network Detection Protocol" },
  { 0x4021, "Stacker LZS" },
  { 0x4023, "RefTek Protocol" },
  { 0x4025, "Fibre Channel" },
  { 0x4027, "EMIT Protocols" },
  { 0x405b, "Vendor-Specific Protocol (VSP)" },
  { 0x8021, "Internet Protocol Control Protocol" },
  { 0x8023, "OSI Network Layer Control Protocol" },
  { 0x8025, "Xerox NS IDP Control Protocol" },
  { 0x8027, "DECnet Phase IV Control Protocol" },
  { 0x8029, "Appletalk Control Protocol" },
  { 0x802b, "Novell IPX Control Protocol" },
  { 0x8031, "Bridging NCP" },
  { 0x8033, "Stream Protocol Control Protocol" },
  { 0x8035, "Banyan Vines Control Protocol" },
  { 0x803d, "Multi-Link Control Protocol" },
  { 0x803f, "NETBIOS Framing Control Protocol" },
  { 0x8041, "Cisco Systems Control Protocol" },
  { 0x8043, "Ascom Timeplex" },
  { 0x8045, "Fujitsu LBLB Control Protocol" },
  { 0x8047, "DCA Remote Lan Network Control Protocol (RLNCP)" },
  { 0x8049, "Serial Data Control Protocol (PPP-SDCP)" },
  { 0x804b, "SNA over 802.2 Control Protocol" },
  { 0x804d, "SNA Control Protocol" },
  { 0x804f, "IP6 Header Compression Control Protocol" },
  { 0x8051, "KNX Bridging Control Protocol" },
  { 0x8053, "Encryption Control Protocol" },
  { 0x8055, "Individual Link Encryption Control Protocol" },
  { 0x8057, "IPv6 Control Protocol" },
  { 0x8059, "PPP Muxing Control Protocol" },
  { 0x805b, "Vendor-Specific Network Control Protocol (VSNCP)" },
  { 0x806f, "Stampede Bridging Control Protocol" },
  { 0x8073, "MP+ Control Protocol" },
  { 0x80c1, "NTCITS IPI Control Protocol" },
  { 0x80fb, "Single Link Compression Control Protocol" },
  { 0x80fd, "Compression Control Protocol" },
  { 0x8207, "Cisco Discovery Protocol Control" },
  { 0x8209, "Netcs Twin Routing" },
  { 0x820b, "STP - Control Protocol" },
  { 0x820d, "EDPCP - Extreme Discovery Protocol Ctrl Prtcl" },
  { 0x8235, "Apple Client Server Protocol Control" },
  { 0x8281, "MPLSCP" },
  { 0x8285, "IEEE p1284.4 standard - Protocol Control" },
  { 0x8287, "ETSI TETRA TNP1 Control Protocol" },
  { 0x8289, "Multichannel Flow Treatment Protocol" },
  { 0xc021, "Link Control Protocol" },
  { 0xc023, "Password Authentication Protocol" },
  { 0xc025, "Link Quality Report" },
  { 0xc027, "Shiva Password Authentication Protocol" },
  { 0xc029, "CallBack Control Protocol (CBCP)" },
  { 0xc02b, "BACP Bandwidth Allocation Control Protocol" },
  { 0xc02d, "BAP" },
  { 0xc05b, "Vendor-Specific Authentication Protocol (VSAP)" },
  { 0xc081, "Container Control Protocol" },
  { 0xc223, "Challenge Handshake Authentication Protocol" },
  { 0xc225, "RSA Authentication Protocol" },
  { 0xc227, "Extensible Authentication Protocol" },
  { 0xc229, "Mitsubishi Security Info Exch Ptcl (SIEP)" },
  { 0xc26f, "Stampede Bridging Authorization Protocol" },
  { 0xc281, "Proprietary Authentication Protocol" },
  { 0xc283, "Proprietary Authentication Protocol" },
  { 0xc481, "Proprietary Node ID Authentication Protocol" },
  { 0, NULL },
};

/*
 * protocol_name - find a name for a PPP protocol.
 */
const char * protocol_name(proto: i32) {
  const lp: &mut protocol_list;

  for (lp = protocol_list; lp.proto != 0; += 1lp) {
    if (proto == lp.proto) {
      return lp.name;
    }
  }
  return NULL;
}




/* ---- Note on PPP Stats support ----
 *
 * The one willing link stats support should add the get_ppp_stats()
 * to fetch statistics from lwIP.
 */

/*
 * reset_link_stats - "reset" stats when link goes up.
 */
pub fn  reset_link_stats(u: i32) {
  if (!get_ppp_stats(u, &old_link_stats)) {
    return;
  }
  gettimeofday(&start_time, NULL);
}

/*
 * update_link_stats - get stats at link termination.
 */
pub fn  update_link_stats(u: i32) {
  let now: timeval;
  let numbuf: String;

  if (!get_ppp_stats(u, &link_stats) || gettimeofday(&now, NULL) < 0) {
    return;
  }
  link_connect_time = now.tv_sec - start_time.tv_sec;
  link_stats_valid = 1;

  link_stats.bytes_in  -= old_link_stats.bytes_in;
  link_stats.bytes_out -= old_link_stats.bytes_out;
  link_stats.pkts_in   -= old_link_stats.pkts_in;
  link_stats.pkts_out  -= old_link_stats.pkts_out;
}

pub fn  print_link_stats() {
  /*
   * Prconnect: i32 time and statistics.
   */
  if (link_stats_valid) {
    t: i32 = (link_connect_time + 5) / 6;    /* 1/10ths of minutes */
    info("Connect time %d.%d minutes.", t/10, t%10);
    info("Sent %u bytes, received %u bytes.", link_stats.bytes_out, link_stats.bytes_in);
    link_stats_valid = 0;
  }
}



