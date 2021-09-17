/*
 * lcp.c - PPP Link Control Protocol.
 *
 * Copyright (c) 1984-2000 Carnegie Mellon University. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. The name "Carnegie Mellon University" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For permission or any legal
 *    details, please contact
 *      Office of Technology Transfer
 *      Carnegie Mellon University
 *      5000 Forbes Avenue
 *      Pittsburgh, PA  15213-3890
 *      (412) 268-4387, fax: (412) 268-7395
 *      tech-transfer@andrew.cmu.edu
 *
 * 4. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by Computing Services
 *     at Carnegie Mellon University (http://www.cmu.edu/computing/)."
 *
 * CARNEGIE MELLON UNIVERSITY DISCLAIMS ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL CARNEGIE MELLON UNIVERSITY BE LIABLE
 * FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */




/*
 * @todo:
 */
















/*
 * When the link comes up we want to be able to wait for a short while,
 * or until seeing some input from the peer, before starting to send
 * configure-requests.  We do this by delaying the fsm_lowerup call.
 */
/* steal a bit in fsm flags word */
pub const DELAYED_UP: u32 = 0x80;

pub fn lcp_delayed_up(arg: &mut Vec<u8>);

/*
 * LCP-related command-line options.
 */

int	lcp_echo_interval = 0; 	/* Interval between LCP echo-requests */
int	lcp_echo_fails = 0;	/* Tolerance to unanswered echo-requests */



/* options */
static u_lcp_echo_interval: i32      = LCP_ECHOINTERVAL; /* Interval between LCP echo-requests */
static u_lcp_echo_fails: i32         = LCP_MAXECHOFAILS; /* Tolerance to unanswered echo-requests */




bool	lcp_echo_adaptive = 0;	/* request echo only if the link was idle */

bool	lax_recv = 0;		/* accept control chars in asyncmap */
bool	noendpoint = 0;		/* don't send/accept endpodiscriminator: i32 */



static noopt: i32 ;



static setendpoint: i32 ;
pub fn printendpoint (option_t *, void (*)(void *, char *, ...),
			       void *);



static option_t lcp_option_list[] = {
    /* LCP options */
    { "-all", o_special_noarg, noopt,
      "Don't request/allow any LCP options" },

    { "noaccomp", o_bool, &lcp_wantoptions[0].neg_accompression,
      "Disable address/control compression",
      OPT_A2CLR, &lcp_allowoptions[0].neg_accompression },
    { "-ac", o_bool, &lcp_wantoptions[0].neg_accompression,
      "Disable address/control compression",
      OPT_ALIAS | OPT_A2CLR, &lcp_allowoptions[0].neg_accompression },

    { "asyncmap", o_uint32, &lcp_wantoptions[0].asyncmap,
      "Set asyncmap (for received packets)",
      OPT_OR, &lcp_wantoptions[0].neg_asyncmap },
    { "-as", o_uint32, &lcp_wantoptions[0].asyncmap,
      "Set asyncmap (for received packets)",
      OPT_ALIAS | OPT_OR, &lcp_wantoptions[0].neg_asyncmap },
    { "default-asyncmap", o_uint32, &lcp_wantoptions[0].asyncmap,
      "Disable asyncmap negotiation",
      OPT_OR | OPT_NOARG | OPT_VAL(!0) | OPT_A2CLR,
      &lcp_allowoptions[0].neg_asyncmap },
    { "-am", o_uint32, &lcp_wantoptions[0].asyncmap,
      "Disable asyncmap negotiation",
      OPT_ALIAS | OPT_OR | OPT_NOARG | OPT_VAL(!0) | OPT_A2CLR,
      &lcp_allowoptions[0].neg_asyncmap },

    { "nomagic", o_bool, &lcp_wantoptions[0].neg_magicnumber,
      "Disable magic number negotiation (looped-back line detection)",
      OPT_A2CLR, &lcp_allowoptions[0].neg_magicnumber },
    { "-mn", o_bool, &lcp_wantoptions[0].neg_magicnumber,
      "Disable magic number negotiation (looped-back line detection)",
      OPT_ALIAS | OPT_A2CLR, &lcp_allowoptions[0].neg_magicnumber },

    { "mru", o_int, &lcp_wantoptions[0].mru,
      "Set MRU (maximum received packet size) for negotiation",
      OPT_PRIO, &lcp_wantoptions[0].neg_mru },
    { "default-mru", o_bool, &lcp_wantoptions[0].neg_mru,
      "Disable MRU negotiation (use default 1500)",
      OPT_PRIOSUB | OPT_A2CLR, &lcp_allowoptions[0].neg_mru },
    { "-mru", o_bool, &lcp_wantoptions[0].neg_mru,
      "Disable MRU negotiation (use default 1500)",
      OPT_ALIAS | OPT_PRIOSUB | OPT_A2CLR, &lcp_allowoptions[0].neg_mru },

    { "mtu", o_int, &lcp_allowoptions[0].mru,
      "Set our MTU", OPT_LIMITS, None, MAXMRU, MINMRU },

    { "nopcomp", o_bool, &lcp_wantoptions[0].neg_pcompression,
      "Disable protocol field compression",
      OPT_A2CLR, &lcp_allowoptions[0].neg_pcompression },
    { "-pc", o_bool, &lcp_wantoptions[0].neg_pcompression,
      "Disable protocol field compression",
      OPT_ALIAS | OPT_A2CLR, &lcp_allowoptions[0].neg_pcompression },

    { "passive", o_bool, &lcp_wantoptions[0].passive,
      "Set passive mode", 1 },
    { "-p", o_bool, &lcp_wantoptions[0].passive,
      "Set passive mode", OPT_ALIAS | 1 },

    { "silent", o_bool, &lcp_wantoptions[0].silent,
      "Set silent mode", 1 },

    { "lcp-echo-failure", o_int, &lcp_echo_fails,
      "Set number of consecutive echo failures to indicate link failure",
      OPT_PRIO },
    { "lcp-echo-interval", o_int, &lcp_echo_interval,
      "Set time in seconds between LCP echo requests", OPT_PRIO },

    { "lcp-echo-adaptive", o_bool, &lcp_echo_adaptive,
      "Suppress LCP echo requests if traffic was received", 1 },

    { "lcp-restart", o_int, &lcp_fsm[0].timeouttime,
      "Set time in seconds between LCP retransmissions", OPT_PRIO },
    { "lcp-max-terminate", o_int, &lcp_fsm[0].maxtermtransmits,
      "Set maximum number of LCP terminate-request transmissions", OPT_PRIO },
    { "lcp-max-configure", o_int, &lcp_fsm[0].maxconfreqtransmits,
      "Set maximum number of LCP configure-request transmissions", OPT_PRIO },
    { "lcp-max-failure", o_int, &lcp_fsm[0].maxnakloops,
      "Set limit on number of LCP configure-naks", OPT_PRIO },

    { "receive-all", o_bool, &lax_recv,
      "Accept all received control characters", 1 },


    { "mrru", o_int, &lcp_wantoptions[0].mrru,
      "Maximum received packet size for multilink bundle",
      OPT_PRIO, &lcp_wantoptions[0].neg_mrru },

    { "mpshortseq", o_bool, &lcp_wantoptions[0].neg_ssnhf,
      "Use short sequence numbers in multilink headers",
      OPT_PRIO | 1, &lcp_allowoptions[0].neg_ssnhf },
    { "nompshortseq", o_bool, &lcp_wantoptions[0].neg_ssnhf,
      "Don't use short sequence numbers in multilink headers",
      OPT_PRIOSUB | OPT_A2CLR, &lcp_allowoptions[0].neg_ssnhf },

    { "endpoint", o_special,  setendpoint,
      "Endpodiscriminator: i32 for multilink",
      OPT_PRIO | OPT_A2PRINTER,  printendpoint },


    { "noendpoint", o_bool, &noendpoint,
      "Don't send or accept multilink endpodiscriminator: i32", 1 },

    {None}
};


/*
 * Callbacks for fsm code.  (CI = Configuration Information)
 */
pub fn lcp_resetci(f: &mut fsm);	/* Reset our CI */
static int  lcp_cilen(f: &mut fsm);		/* Return length of our CI */
pub fn lcp_addci(f: &mut fsm, u_ucp: &mut String, lenp: &mut i32); /* Add our CI to pkt */
static int  lcp_ackci(f: &mut fsm, u_p: &mut String, len: i32); /* Peer ack'd our CI */
static int  lcp_nakci(f: &mut fsm, u_p: &mut String, len: i32, treat_as_reject: i32); /* Peer nak'd our CI */
static int  lcp_rejci(f: &mut fsm, u_p: &mut String, len: i32); /* Peer rej'd our CI */
static int  lcp_reqci(f: &mut fsm, u_inp: &mut String, lenp: &mut i32, reject_if_disagree: i32); /* Rcv peer CI */
pub fn lcp_up(f: &mut fsm);		/* We're UP */
pub fn lcp_down(f: &mut fsm);		/* We're DOWN */
pub fn lcp_starting (fsm *);	/* We need lower layer up */
pub fn lcp_finished (fsm *);	/* We need lower layer down */
static int  lcp_extcode(f: &mut fsm, code: i32, id: i32, u_inp: &mut String, len: i32);
pub fn lcp_rprotrej(f: &mut fsm, u_inp: &mut String, len: i32);

/*
 * routines to send LCP echos to peer
 */

pub fn lcp_echo_lowerup(pcb: &mut ppp_pcb);
pub fn lcp_echo_lowerdown(pcb: &mut ppp_pcb);
pub fn LcpEchoTimeout(arg: &mut Vec<u8>);
pub fn lcp_received_echo_reply(f: &mut fsm, id: i32, u_inp: &mut String, len: i32);
pub fn LcpSendEchoRequest(f: &mut fsm);
pub fn LcpLinkFailure(f: &mut fsm);
pub fn LcpEchoCheck(f: &mut fsm);

static const fsm_callbacks lcp_callbacks = {	/* LCP callback routines */
    lcp_resetci,		/* Reset our Configuration Information */
    lcp_cilen,			/* Length of our Configuration Information */
    lcp_addci,			/* Add our Configuration Information */
    lcp_ackci,			/* ACK our Configuration Information */
    lcp_nakci,			/* NAK our Configuration Information */
    lcp_rejci,			/* Reject our Configuration Information */
    lcp_reqci,			/* Request peer's Configuration Information */
    lcp_up,			/* Called when fsm reaches OPENED state */
    lcp_down,			/* Called when fsm leaves OPENED state */
    lcp_starting,		/* Called when we want the lower layer up */
    lcp_finished,		/* Called when we want the lower layer down */
    None,			/* Called when Protocol-Reject received */
    None,			/* Retransmission is necessary */
    lcp_extcode,		/* Called to handle LCP-specific codes */
    "LCP"			/* String name of protocol */
};

/*
 * Protocol entry points.
 * Some of these are called directly.
 */

pub fn lcp_init(pcb: &mut ppp_pcb);
pub fn lcp_input(pcb: &mut ppp_pcb, u_p: &mut String, len: i32);
pub fn lcp_protrej(pcb: &mut ppp_pcb);

static lcp_printpkt: i32( u_p: &mut String, plen: i32,
		void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>);


const struct protent lcp_protent = {
    PPP_LCP,
    lcp_init,
    lcp_input,
    lcp_protrej,
    lcp_lowerup,
    lcp_lowerdown,
    lcp_open,
    lcp_close,

    lcp_printpkt,


    None,


    "LCP",
    None,


    lcp_option_list,
    None,


    None,
    None

};

/*
 * Length of each type of configuration option (in octets)
 */
pub const CILEN_VOID: u32 = 2; 
pub const CILEN_CHAR: u32 = 3; 
pub const CILEN_SHORT: u32 = 4; 	/* CILEN_VOID + 2 */pub const CILEN_VOID: u32 = 2; 
pub const CILEN_CHAP: u32 = 5; 	/* CILEN_VOID + 2 + 1 pub const CILEN_VOID: u32 = 2; 
pub const CILEN_LONG: u32 = 6; 	/* CILEN_VOID + 4 */

pub const CILEN_LQR: u32 = 8;	/* CILEN_VOID + 2 + 4 */

pub const CILEN_CBCP: u32 = 3;

#define CODENAME(x)	((x) == CONFACK ? "ACK" : \
			 (x) == CONFNAK ? "NAK" : "REJ")


/*
 * noopt - Disable all options (why?).
 */
pub fn noopt(argv)
    argv: &mut String;
{
    BZERO( &lcp_wantoptions[0], sizeof (struct lcp_options));
    BZERO( &lcp_allowoptions[0], sizeof (struct lcp_options));

    return (1);
}



pub fn setendpoint(argv)
    argv: &mut String;
{
    if (str_to_epdisc(&lcp_wantoptions[0].endpoint, *argv)) {
	lcp_wantoptions[0].neg_endpoint = 1;
	return 1;
    }
    option_error("Can't parse '%s' as an endpodiscriminator: i32", *argv);
    return 0;
}

pub fn
printendpoint(opt, printer, arg)
    option_t *opt;
    void (*printer) (void *, char *, ...);
    arg: &mut Vec<u8>;
{
	printer(arg, "%s", epdisc_to_str(&lcp_wantoptions[0].endpoint));
}


/*
 * lcp_init - Initialize LCP.
 */
pub fn lcp_init(pcb: &mut ppp_pcb) {
    f: &mut fsm = &pcb.lcp_fsm;
    lcp_options *wo = &pcb.lcp_wantoptions;
    lcp_options *ao = &pcb.lcp_allowoptions;

    f.pcb = pcb;
    f.protocol = PPP_LCP;
    f.callbacks = &lcp_callbacks;

    fsm_init(f);

    BZERO(wo, sizeof(*wo));
    wo.neg_mru = 1;
    wo.mru = PPP_DEFMRU;
    wo.neg_asyncmap = 1;
    wo.neg_magicnumber = 1;
    wo.neg_pcompression = 1;
    wo.neg_accompression = 1;

    BZERO(ao, sizeof(*ao));
    ao.neg_mru = 1;
    ao.mru = PPP_MAXMRU;
    ao.neg_asyncmap = 1;

    ao.neg_chap = 1;
    ao.chap_mdtype = CHAP_MDTYPE_SUPPORTED;


    ao.neg_upap = 1;


    ao.neg_eap = 1;

    ao.neg_magicnumber = 1;
    ao.neg_pcompression = 1;
    ao.neg_accompression = 1;
    ao.neg_endpoint = 1;
}


/*
 * lcp_open - LCP is allowed to come up.
 */
pub fn  lcp_open(pcb: &mut ppp_pcb) {
    f: &mut fsm = &pcb.lcp_fsm;
    lcp_options *wo = &pcb.lcp_wantoptions;

    f.flags &= !(OPT_PASSIVE | OPT_SILENT);
    if (wo.passive)
	f.flags |= OPT_PASSIVE;
    if (wo.silent)
	f.flags |= OPT_SILENT;
    fsm_open(f);
}


/*
 * lcp_close - Take LCP down.
 */
pub fn  lcp_close(pcb: &mut ppp_pcb, reason: &String) {
    f: &mut fsm = &pcb.lcp_fsm;
    let letoldstate: i32;

    if (pcb.phase != PPP_PHASE_DEAD

    && pcb.phase != PPP_PHASE_MASTER

    )
	new_phase(pcb, PPP_PHASE_TERMINATE);

    if (f.flags & DELAYED_UP) {
	UNTIMEOUT(lcp_delayed_up, f);
	f.state = PPP_FSM_STOPPED;
    }
    oldstate = f.state;

    fsm_close(f, reason);
    if (oldstate == PPP_FSM_STOPPED && (f.flags & (OPT_PASSIVE|OPT_SILENT|DELAYED_UP))) {
	/*
	 * This action is not strictly according to the FSM in RFC1548,
	 * but it does mean that the program terminates if you do a
	 * lcp_close() when a connection hasn't been established
	 * because we are in passive/silent mode or because we have
	 * delayed the fsm_lowerup() call and it hasn't happened yet.
	 */
	f.flags &= !DELAYED_UP;
	lcp_finished(f);
    }
}


/*
 * lcp_lowerup - The lower layer is up.
 */
pub fn  lcp_lowerup(pcb: &mut ppp_pcb) {
    lcp_options *wo = &pcb.lcp_wantoptions;
    f: &mut fsm = &pcb.lcp_fsm;
    /*
     * Don't use A/C or protocol compression on transmission,
     * but accept A/C and protocol compressed packets
     * if we are going to ask for A/C and protocol compression.
     */
    if (ppp_send_config(pcb, PPP_MRU, 0xffffffff, 0, 0) < 0
	|| ppp_recv_config(pcb, PPP_MRU, (pcb.settings.lax_recv? 0: 0xffffffff),
			   wo.neg_pcompression, wo.neg_accompression) < 0)
	    return;
    pcb.peer_mru = PPP_MRU;

    if (pcb.settings.listen_time != 0) {
	f.flags |= DELAYED_UP;
	TIMEOUTMS(lcp_delayed_up, f, pcb.settings.listen_time);
    } else
	fsm_lowerup(f);
}


/*
 * lcp_lowerdown - The lower layer is down.
 */
pub fn  lcp_lowerdown(pcb: &mut ppp_pcb) {
    f: &mut fsm = &pcb.lcp_fsm;

    if (f.flags & DELAYED_UP) {
	f.flags &= !DELAYED_UP;
	UNTIMEOUT(lcp_delayed_up, f);
    } else
	fsm_lowerdown(f);
}


/*
 * lcp_delayed_up - Bring the lower layer up now.
 */
pub fn lcp_delayed_up(arg: &mut Vec<u8>) {
    f: &mut fsm = arg;

    if (f.flags & DELAYED_UP) {
	f.flags &= !DELAYED_UP;
	fsm_lowerup(f);
    }
}


/*
 * lcp_input - Input LCP packet.
 */
pub fn lcp_input(pcb: &mut ppp_pcb, u_p: &mut String, len: i32) {
    f: &mut fsm = &pcb.lcp_fsm;

    if (f.flags & DELAYED_UP) {
	f.flags &= !DELAYED_UP;
	UNTIMEOUT(lcp_delayed_up, f);
	fsm_lowerup(f);
    }
    fsm_input(f, p, len);
}

/*
 * lcp_extcode - Handle a LCP-specific code.
 */
pub fn lcp_extcode(f: &mut fsm, code: i32, id: i32, u_inp: &mut String, len: i32)) -> i32 {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    let mut u_magp: &mut String;

    match( code ){
    PROTREJ =>
	lcp_rprotrej(f, inp, len);
	break;
    
    ECHOREQ =>
	if (f.state != PPP_FSM_OPENED)
	    break;
	magp = inp;
	PUTLONG(go.magicnumber, magp);
	fsm_sdata(f, ECHOREP, id, inp, len);
	break;
    
    ECHOREP =>
	lcp_received_echo_reply(f, id, inp, len);
	break;

    DISCREQ =>
    IDENTIF =>
    TIMEREM =>
	break;

    _ =>
	return 0;
    }
    return 1;
}

    
/*
 * lcp_rprotrej - Receive an Protocol-Reject.
 *
 * Figure out which protocol is rejected and inform it.
 */
pub fn lcp_rprotrej(f: &mut fsm, u_inp: &mut String, len: i32) {
    let leti: i32;
 let mut protp: &mut protent;
    prot: u16;

    let pname: String;


    if (len < 2) {
	LCPDEBUG(("lcp_rprotrej: Rcvd short Protocol-Reject packet!"));
	return;
    }

    GETSHORT(prot, inp);

    /*
     * Protocol-Reject packets received in any state other than the LCP
     * OPENED state SHOULD be silently discarded.
     */
    if( f.state != PPP_FSM_OPENED ){
	LCPDEBUG(("Protocol-Reject discarded: LCP in state %d", f.state));
	return;
    }


    pname = protocol_name(prot);


    /*
     * Upcall the proper Protocol-Reject routine.
     */
    for (i = 0; (protp = protocols[i]) != NULL; += 1i)
	if (protp.protocol == prot) {

	    if (pname != NULL)
		ppp_dbglog("Protocol-Reject for '%s' (0x%x) received", pname,
		       prot);
	    else

		ppp_dbglog("Protocol-Reject for 0x%x received", prot);
	    (*protp.protrej)(f.pcb);
	    return;
	}


    if (pname != NULL)
	ppp_warn("Protocol-Reject for unsupported protocol '%s' (0x%x)", pname,
	     prot);
    else

	ppp_warn("Protocol-Reject for unsupported protocol 0x%x", prot);
}


/*
 * lcp_protrej - A Protocol-Reject was received.
 */
/*ARGSUSED*/
pub fn lcp_protrej(pcb: &mut ppp_pcb) {
    /*
     * Can't reject LCP!
     */
    ppp_error("Received Protocol-Reject for LCP!");
    fsm_protreject(&pcb.lcp_fsm);
}


/*
 * lcp_sprotrej - Send a Protocol-Reject for some protocol.
 */
pub fn  lcp_sprotrej(pcb: &mut ppp_pcb, u_p: &mut String, len: i32) {
    f: &mut fsm = &pcb.lcp_fsm;
    /*
     * Send back the protocol and the information field of the
     * rejected packet.  We only get here if LCP is in the OPENED state.
     */

    p += 2;
    len -= 2;


    fsm_sdata(f, PROTREJ, += 1f.id,
	      p, len);
}


/*
 * lcp_resetci - Reset our CI.
 */
pub fn lcp_resetci(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *wo = &pcb.lcp_wantoptions;
    lcp_options *go = &pcb.lcp_// gotoptions;
    lcp_options *ao = &pcb.lcp_allowoptions;



    /* note: default value is true for allow options */
    if (pcb.settings.user && pcb.settings.passwd) {

      if (pcb.settings.refuse_pap) {
        ao.neg_upap = 0;
      }


      if (pcb.settings.refuse_chap) {
        ao.chap_mdtype &= !MDTYPE_MD5;
      }

      if (pcb.settings.refuse_mschap) {
        ao.chap_mdtype &= !MDTYPE_MICROSOFT;
      }
      if (pcb.settings.refuse_mschap_v2) {
        ao.chap_mdtype &= !MDTYPE_MICROSOFT_V2;
      }

      ao.neg_chap = (ao.chap_mdtype != MDTYPE_NONE);


      if (pcb.settings.refuse_eap) {
        ao.neg_eap = 0;
      }



      /* note: default value is false for wanted options */
      if (pcb.settings.auth_required) {

        if (!pcb.settings.refuse_pap) {
          wo.neg_upap = 1;
        }


        if (!pcb.settings.refuse_chap) {
          wo.chap_mdtype |= MDTYPE_MD5;
        }

        if (!pcb.settings.refuse_mschap) {
          wo.chap_mdtype |= MDTYPE_MICROSOFT;
        }
        if (!pcb.settings.refuse_mschap_v2) {
          wo.chap_mdtype |= MDTYPE_MICROSOFT_V2;
        }

        wo.neg_chap = (wo.chap_mdtype != MDTYPE_NONE);


        if (!pcb.settings.refuse_eap) {
          wo.neg_eap = 1;
        }

      }


    } else {

      ao.neg_upap = 0;


      ao.neg_chap = 0;
      ao.chap_mdtype = MDTYPE_NONE;


      ao.neg_eap = 0;

    }

    PPPDEBUG(LOG_DEBUG, ("ppp: auth protocols:"));

    PPPDEBUG(LOG_DEBUG, (" PAP=%d", ao.neg_upap));


    PPPDEBUG(LOG_DEBUG, (" CHAP=%d CHAP_MD5=%d", ao.neg_chap, !!(ao.chap_mdtype&MDTYPE_MD5)));

    PPPDEBUG(LOG_DEBUG, (" CHAP_MS=%d CHAP_MS2=%d", !!(ao.chap_mdtype&MDTYPE_MICROSOFT), !!(ao.chap_mdtype&MDTYPE_MICROSOFT_V2)));



    PPPDEBUG(LOG_DEBUG, (" EAP=%d", ao.neg_eap));

    PPPDEBUG(LOG_DEBUG, ("\n"));



    wo.magicnumber = magic();
    wo.numloops = 0;
    *go = *wo;

    if (!multilink) {
	go.neg_mrru = 0;

	go.neg_ssnhf = 0;
	go.neg_endpoint = 0;

    }

    if (pcb.settings.noendpoint)
	ao.neg_endpoint = 0;
    pcb.peer_mru = PPP_MRU;

    auth_reset(pcb);

}


/*
 * lcp_cilen - Return length of our CI.
 */
pub fn lcp_cilen(f: &mut fsm)) -> i32 {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;

#define LENCIVOID(neg)	((neg) ? CILEN_VOID : 0)

#define LENCICHAP(neg)	((neg) ? CILEN_CHAP : 0)

#define LENCISHORT(neg)	((neg) ? CILEN_SHORT : 0)
#define LENCILONG(neg)	((neg) ? CILEN_LONG : 0)

#define LENCILQR(neg)	((neg) ? CILEN_LQR: 0)

#define LENCICBCP(neg)	((neg) ? CILEN_CBCP: 0)
    /*
     * NB: we only ask for one of CHAP, UPAP, or EAP, even if we will
     * accept more than one.  We prefer EAP first, then CHAP, then
     * PAP.
     */
    return (LENCISHORT(go.neg_mru && go.mru != PPP_DEFMRU) +
	    LENCILONG(go.neg_asyncmap && go.asyncmap != 0xFFFFFFFF) +

	    LENCISHORT(go.neg_eap) +



	    LENCICHAP(!go.neg_eap && go.neg_chap) +


	    LENCICHAP(go.neg_chap) +




	    LENCISHORT(!go.neg_eap && !go.neg_chap && go.neg_upap) +


	    LENCISHORT(!go.neg_eap && go.neg_upap) +


	    LENCISHORT(!go.neg_chap && go.neg_upap) +


	    LENCISHORT(go.neg_upap) +



	    LENCILQR(go.neg_lqr) +

	    LENCICBCP(go.neg_cbcp) +
	    LENCILONG(go.neg_magicnumber) +
	    LENCIVOID(go.neg_pcompression) +
	    LENCIVOID(go.neg_accompression) +

	    LENCISHORT(go.neg_mrru) +

	    LENCIVOID(go.neg_ssnhf) +
	    (go.neg_endpoint? CILEN_CHAR + go.endpoint.length: 0));
}


/*
 * lcp_addci - Add our desired CIs to a packet.
 */
pub fn lcp_addci(f: &mut fsm, u_ucp: &mut String, lenp: &mut i32) {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    u_start_ucp: &mut String = ucp;

#define ADDCIVOID(opt, neg) \
    if (neg) { \
	PUTCHAR(opt, ucp); \
	PUTCHAR(CILEN_VOID, ucp); \
    }
#define ADDCISHORT(opt, neg, val) \
    if (neg) { \
	PUTCHAR(opt, ucp); \
	PUTCHAR(CILEN_SHORT, ucp); \
	PUTSHORT(val, ucp); \
    }

#define ADDCICHAP(opt, neg, val) \
    if (neg) { \
	PUTCHAR((opt), ucp); \
	PUTCHAR(CILEN_CHAP, ucp); \
	PUTSHORT(PPP_CHAP, ucp); \
	PUTCHAR((CHAP_DIGEST(val)), ucp); \
    }

#define ADDCILONG(opt, neg, val) \
    if (neg) { \
	PUTCHAR(opt, ucp); \
	PUTCHAR(CILEN_LONG, ucp); \
	PUTLONG(val, ucp); \
    }

#define ADDCILQR(opt, neg, val) \
    if (neg) { \
	PUTCHAR(opt, ucp); \
	PUTCHAR(CILEN_LQR, ucp); \
	PUTSHORT(PPP_LQR, ucp); \
	PUTLONG(val, ucp); \
    }

#define ADDCICHAR(opt, neg, val) \
    if (neg) { \
	PUTCHAR(opt, ucp); \
	PUTCHAR(CILEN_CHAR, ucp); \
	PUTCHAR(val, ucp); \
    }
#define ADDCIENDP(opt, neg, class, val, len) \
    if (neg) { \
	let leti: i32; \
	PUTCHAR(opt, ucp); \
	PUTCHAR(CILEN_CHAR + len, ucp); \
	PUTCHAR(class, ucp); \
	for (i = 0; i < len; += 1i) \
	    PUTCHAR(val[i], ucp); \
    }

    ADDCISHORT(CI_MRU, go.neg_mru && go.mru != PPP_DEFMRU, go.mru);
    ADDCILONG(CI_ASYNCMAP, go.neg_asyncmap && go.asyncmap != 0xFFFFFFFF,
	      go.asyncmap);

    ADDCISHORT(CI_AUTHTYPE, go.neg_eap, PPP_EAP);



    ADDCICHAP(CI_AUTHTYPE, !go.neg_eap && go.neg_chap, go.chap_mdtype);


    ADDCICHAP(CI_AUTHTYPE, go.neg_chap, go.chap_mdtype);




    ADDCISHORT(CI_AUTHTYPE, !go.neg_eap && !go.neg_chap && go.neg_upap, PPP_PAP);


    ADDCISHORT(CI_AUTHTYPE, !go.neg_eap && go.neg_upap, PPP_PAP);


    ADDCISHORT(CI_AUTHTYPE, !go.neg_chap && go.neg_upap, PPP_PAP);


    ADDCISHORT(CI_AUTHTYPE, go.neg_upap, PPP_PAP);



    ADDCILQR(CI_QUALITY, go.neg_lqr, go.lqr_period);

    ADDCICHAR(CI_CALLBACK, go.neg_cbcp, CBCP_OPT);
    ADDCILONG(CI_MAGICNUMBER, go.neg_magicnumber, go.magicnumber);
    ADDCIVOID(CI_PCOMPRESSION, go.neg_pcompression);
    ADDCIVOID(CI_ACCOMPRESSION, go.neg_accompression);

    ADDCISHORT(CI_MRRU, go.neg_mrru, go.mrru);

    ADDCIVOID(CI_SSNHF, go.neg_ssnhf);
    ADDCIENDP(CI_EPDISC, go.neg_endpoint, go.endpoint.class_,
	      go.endpoint.value, go.endpoint.length);

    if (ucp - start_ucp != *lenp) {
	/* this should never happen, because peer_mtu should be 1500 */
	ppp_error("Bug in lcp_addci: wrong length");
    }
}


/*
 * lcp_ackci - Ack our CIs.
 * This should not modify any state if the Ack is bad.
 *
 * Returns:
 *	0 - Ack was bad.
 *	1 - Ack was good.
 */
pub fn lcp_ackci(f: &mut fsm, u_p: &mut String, len: i32)) -> i32 {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    cilen: u8, citype, cichar;
    cishort: u16;
    let cilong: u32;

    /*
     * CIs must be in exactly the same order that we sent.
     * Check packet length and CI length at each step.
     * If we find any deviations, then this packet is bad.
     */
#define ACKCIVOID(opt, neg) \
    if (neg) { \
	if ((len -= CILEN_VOID) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_VOID || \
	    citype != opt) \
	    // goto bad; \
    }
#define ACKCISHORT(opt, neg, val) \
    if (neg) { \
	if ((len -= CILEN_SHORT) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_SHORT || \
	    citype != opt) \
	    // goto bad; \
	GETSHORT(cishort, p); \
	if (cishort != val) \
	    // goto bad; \
    }
#define ACKCICHAR(opt, neg, val) \
    if (neg) { \
	if ((len -= CILEN_CHAR) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_CHAR || \
	    citype != opt) \
	    // goto bad; \
	GETCHAR(cichar, p); \
	if (cichar != val) \
	    // goto bad; \
    }

#define ACKCICHAP(opt, neg, val) \
    if (neg) { \
	if ((len -= CILEN_CHAP) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_CHAP || \
	    citype != (opt)) \
	    // goto bad; \
	GETSHORT(cishort, p); \
	if (cishort != PPP_CHAP) \
	    // goto bad; \
	GETCHAR(cichar, p); \
	if (cichar != (CHAP_DIGEST(val))) \
	  // goto bad; \
    }

#define ACKCILONG(opt, neg, val) \
    if (neg) { \
	if ((len -= CILEN_LONG) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_LONG || \
	    citype != opt) \
	    // goto bad; \
	GETLONG(cilong, p); \
	if (cilong != val) \
	    // goto bad; \
    }

#define ACKCILQR(opt, neg, val) \
    if (neg) { \
	if ((len -= CILEN_LQR) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_LQR || \
	    citype != opt) \
	    // goto bad; \
	GETSHORT(cishort, p); \
	if (cishort != PPP_LQR) \
	    // goto bad; \
	GETLONG(cilong, p); \
	if (cilong != val) \
	  // goto bad; \
    }

#define ACKCIENDP(opt, neg, class, val, vlen) \
    if (neg) { \
	let leti: i32; \
	if ((len -= CILEN_CHAR + vlen) < 0) \
	    // goto bad; \
	GETCHAR(citype, p); \
	GETCHAR(cilen, p); \
	if (cilen != CILEN_CHAR + vlen || \
	    citype != opt) \
	    // goto bad; \
	GETCHAR(cichar, p); \
	if (cichar != class) \
	    // goto bad; \
	for (i = 0; i < vlen; += 1i) { \
	    GETCHAR(cichar, p); \
	    if (cichar != val[i]) \
		// goto bad; \
	} \
    }

    ACKCISHORT(CI_MRU, go.neg_mru && go.mru != PPP_DEFMRU, go.mru);
    ACKCILONG(CI_ASYNCMAP, go.neg_asyncmap && go.asyncmap != 0xFFFFFFFF,
	      go.asyncmap);

    ACKCISHORT(CI_AUTHTYPE, go.neg_eap, PPP_EAP);



    ACKCICHAP(CI_AUTHTYPE, !go.neg_eap && go.neg_chap, go.chap_mdtype);


    ACKCICHAP(CI_AUTHTYPE, go.neg_chap, go.chap_mdtype);




    ACKCISHORT(CI_AUTHTYPE, !go.neg_eap && !go.neg_chap && go.neg_upap, PPP_PAP);


    ACKCISHORT(CI_AUTHTYPE, !go.neg_eap && go.neg_upap, PPP_PAP);


    ACKCISHORT(CI_AUTHTYPE, !go.neg_chap && go.neg_upap, PPP_PAP);


    ACKCISHORT(CI_AUTHTYPE, go.neg_upap, PPP_PAP);



    ACKCILQR(CI_QUALITY, go.neg_lqr, go.lqr_period);

    ACKCICHAR(CI_CALLBACK, go.neg_cbcp, CBCP_OPT);
    ACKCILONG(CI_MAGICNUMBER, go.neg_magicnumber, go.magicnumber);
    ACKCIVOID(CI_PCOMPRESSION, go.neg_pcompression);
    ACKCIVOID(CI_ACCOMPRESSION, go.neg_accompression);

    ACKCISHORT(CI_MRRU, go.neg_mrru, go.mrru);

    ACKCIVOID(CI_SSNHF, go.neg_ssnhf);
    ACKCIENDP(CI_EPDISC, go.neg_endpoint, go.endpoint.class_,
	      go.endpoint.value, go.endpoint.length);

    /*
     * If there are any remaining CIs, then this packet is bad.
     */
    if (len != 0)
	// goto bad;
    return (1);
bad:
    LCPDEBUG(("lcp_acki: received bad Ack!"));
    return (0);
}


/*
 * lcp_nakci - Peer has sent a NAK for some of our CIs.
 * This should not modify any state if the Nak is bad
 * or if LCP is in the OPENED state.
 *
 * Returns:
 *	0 - Nak was bad.
 *	1 - Nak was good.
 */
pub fn lcp_nakci(f: &mut fsm, u_p: &mut String, len: i32, treat_as_reject: i32)) -> i32 {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    lcp_options *wo = &pcb.lcp_wantoptions;
    citype: u8, cichar, *next;
    cishort: u16;
    let cilong: u32;
    lcp_options no;		/* options we've seen Naks for */
    lcp_options try_;		/* options to request next time */
    looped_back: i32 = 0;
    let letcilen: i32;

    BZERO(&no, sizeof(no));
    try_ = *go;

    /*
     * Any Nak'd CIs must be in exactly the same order that we sent.
     * Check packet length and CI length at each step.
     * If we find any deviations, then this packet is bad.
     */
#define NAKCIVOID(opt, neg) \
    if (go.neg && \
	len >= CILEN_VOID && \
	p[1] == CILEN_VOID && \
	p[0] == opt) { \
	len -= CILEN_VOID; \
	INCPTR(CILEN_VOID, p); \
	no.neg = 1; \
	try_.neg = 0; \
    }

#define NAKCICHAP(opt, neg, code) \
    if (go.neg && \
	len >= CILEN_CHAP && \
	p[1] == CILEN_CHAP && \
	p[0] == opt) { \
	len -= CILEN_CHAP; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETCHAR(cichar, p); \
	no.neg = 1; \
	code \
    }

#define NAKCICHAR(opt, neg, code) \
    if (go.neg && \
	len >= CILEN_CHAR && \
	p[1] == CILEN_CHAR && \
	p[0] == opt) { \
	len -= CILEN_CHAR; \
	INCPTR(2, p); \
	GETCHAR(cichar, p); \
	no.neg = 1; \
	code \
    }
#define NAKCISHORT(opt, neg, code) \
    if (go.neg && \
	len >= CILEN_SHORT && \
	p[1] == CILEN_SHORT && \
	p[0] == opt) { \
	len -= CILEN_SHORT; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	no.neg = 1; \
	code \
    }
#define NAKCILONG(opt, neg, code) \
    if (go.neg && \
	len >= CILEN_LONG && \
	p[1] == CILEN_LONG && \
	p[0] == opt) { \
	len -= CILEN_LONG; \
	INCPTR(2, p); \
	GETLONG(cilong, p); \
	no.neg = 1; \
	code \
    }

#define NAKCILQR(opt, neg, code) \
    if (go.neg && \
	len >= CILEN_LQR && \
	p[1] == CILEN_LQR && \
	p[0] == opt) { \
	len -= CILEN_LQR; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETLONG(cilong, p); \
	no.neg = 1; \
	code \
    }

#define NAKCIENDP(opt, neg) \
    if (go.neg && \
	len >= CILEN_CHAR && \
	p[0] == opt && \
	p[1] >= CILEN_CHAR && \
	p[1] <= len) { \
	len -= p[1]; \
	INCPTR(p[1], p); \
	no.neg = 1; \
	try_.neg = 0; \
    }

    /*
     * NOTE!  There must be no assignments to individual fields of *go in
     * the code below.  Any such assignment is a BUG!
     */
    /*
     * We don't care if they want to send us smaller packets than
     * we want.  Therefore, accept any MRU less than what we asked for,
     * but then ignore the new value when setting the MRU in the kernel.
     * If they send us a bigger MRU than what we asked, accept it, up to
     * the limit of the default MRU we'd get if we didn't negotiate.
     */
    if (go.neg_mru && go.mru != PPP_DEFMRU) {
	NAKCISHORT(CI_MRU, neg_mru,
		   if (cishort <= wo.mru || cishort <= PPP_DEFMRU)
		       try_.mru = cishort;
		   );
    }

    /*
     * Add any characters they want to our (receive-side) asyncmap.
     */
    if (go.neg_asyncmap && go.asyncmap != 0xFFFFFFFF) {
	NAKCILONG(CI_ASYNCMAP, neg_asyncmap,
		  try_.asyncmap = go.asyncmap | cilong;
		  );
    }

    /*
     * If they've nak'd our authentication-protocol, check whether
     * they are proposing a different protocol, or a different
     * hash algorithm for CHAP.
     */
    if ((0

        || go.neg_chap


        || go.neg_upap


        || go.neg_eap

        )
	&& len >= CILEN_SHORT
	&& p[0] == CI_AUTHTYPE && p[1] >= CILEN_SHORT && p[1] <= len) {
	cilen = p[1];
	len -= cilen;

	no.neg_chap = go.neg_chap;


	no.neg_upap = go.neg_upap;


	no.neg_eap = go.neg_eap;

	INCPTR(2, p);
	GETSHORT(cishort, p);


	if (cishort == PPP_PAP && cilen == CILEN_SHORT) {

	    /* If we were asking for EAP, then we need to stop that. */
	    if (go.neg_eap)
		try_.neg_eap = 0;
	    else



	    /* If we were asking for CHAP, then we need to stop that. */
	    if (go.neg_chap)
		try_.neg_chap = 0;
	    else


	    /*
	     * If we weren't asking for CHAP or EAP, then we were asking for
	     * PAP, in which case this Nak is bad.
	     */
		// goto bad;
	} else



	if (cishort == PPP_CHAP && cilen == CILEN_CHAP) {
	    GETCHAR(cichar, p);

	    /* Stop asking for EAP, if we were. */
	    if (go.neg_eap) {
		try_.neg_eap = 0;
		/* Try to set up to use their suggestion, if possible */
		if (CHAP_CANDIGEST(go.chap_mdtype, cichar))
		    try_.chap_mdtype = CHAP_MDTYPE_D(cichar);
	    } else

	    if (go.neg_chap) {
		/*
		 * We were asking for our preferred algorithm, they must
		 * want something different.
		 */
		if (cichar != CHAP_DIGEST(go.chap_mdtype)) {
		    if (CHAP_CANDIGEST(go.chap_mdtype, cichar)) {
			/* Use their suggestion if we support it ... */
			try_.chap_mdtype = CHAP_MDTYPE_D(cichar);
		    } else {
			/* ... otherwise, try our next-preferred algorithm. */
			try_.chap_mdtype &= !(CHAP_MDTYPE(try_.chap_mdtype));
			if (try_.chap_mdtype == MDTYPE_NONE) /* out of algos */
			    try_.neg_chap = 0;
		    }
		} else {
		    /*
		     * Whoops, they Nak'd our algorithm of choice
		     * but then suggested it back to us.
		     */
		    // goto bad;
		}
	    } else {
		/*
		 * Stop asking for PAP if we were asking for it.
		 */

		try_.neg_upap = 0;

	    }

	} else

	{


	    /*
	     * If we were asking for EAP, and they're Conf-Naking EAP,
	     * well, that's just strange.  Nobody should do that.
	     */
	    if (cishort == PPP_EAP && cilen == CILEN_SHORT && go.neg_eap)
		ppp_dbglog("Unexpected Conf-Nak for EAP");

	    /*
	     * We don't recognize what they're suggesting.
	     * Stop asking for what we were asking for.
	     */
	    if (go.neg_eap)
		try_.neg_eap = 0;
	    else



	    if (go.neg_chap)
		try_.neg_chap = 0;
	    else



	    if(1)
		try_.neg_upap = 0;
	    else

	    {}

	    p += cilen - CILEN_SHORT;
	}
    }


    /*
     * If they can't cope with our link quality protocol, we'll have
     * to stop asking for LQR.  We haven't got any other protocol.
     * If they Nak the reporting period, take their value XXX ?
     */
    NAKCILQR(CI_QUALITY, neg_lqr,
	     if (cishort != PPP_LQR)
		 try_.neg_lqr = 0;
	     else
		 try_.lqr_period = cilong;
	     );


    /*
     * Only implementing CBCP...not the rest of the callback options
     */
    NAKCICHAR(CI_CALLBACK, neg_cbcp,
              try_.neg_cbcp = 0;
              ()cichar; /* if CHAP support is not compiled, cichar is set but not used, which makes some compilers complaining */
              );

    /*
     * Check for a looped-back line.
     */
    NAKCILONG(CI_MAGICNUMBER, neg_magicnumber,
	      try_.magicnumber = magic();
	      looped_back = 1;
	      );

    /*
     * Peer shouldn't send Nak for protocol compression or
     * address/control compression requests; they should send
     * a Reject instead.  If they send a Nak, treat it as a Reject.
     */
    NAKCIVOID(CI_PCOMPRESSION, neg_pcompression);
    NAKCIVOID(CI_ACCOMPRESSION, neg_accompression);


    /*
     * Nak for MRRU option - accept their value if it is smaller
     * than the one we want.
     */
    if (go.neg_mrru) {
	NAKCISHORT(CI_MRRU, neg_mrru,
		   if (treat_as_reject)
		       try_.neg_mrru = 0;
		   else if (cishort <= wo.mrru)
		       try_.mrru = cishort;
		   );
    }
 /* HAVE_MULTILINK */
    


    /*
     * Nak for short sequence numbers shouldn't be sent, treat it
     * like a reject.
     */
    NAKCIVOID(CI_SSNHF, neg_ssnhf);

    /*
     * Nak of the endpodiscriminator: i32 option is not permitted,
     * treat it like a reject.
     */
    NAKCIENDP(CI_EPDISC, neg_endpoint);

    /*
     * There may be remaining CIs, if the peer is requesting negotiation
     * on an option that we didn't include in our request packet.
     * If we see an option that we requested, or one we've already seen
     * in this packet, then this packet is bad.
     * If we wanted to respond by starting to negotiate on the requested
     * option(s), we could, but we don't, because except for the
     * authentication type and quality protocol, if we are not negotiating
     * an option, it is because we were told not to.
     * For the authentication type, the Nak from the peer means
     * `let me authenticate myself with you' which is a bit pointless.
     * For the quality protocol, the Nak means `ask me to send you quality
     * reports', but if we didn't ask for them, we don't want them.
     * An option we don't recognize represents the peer asking to
     * negotiate some option we don't support, so ignore it.
     */
    while (len >= CILEN_VOID) {
	GETCHAR(citype, p);
	GETCHAR(cilen, p);
	if (cilen < CILEN_VOID || (len -= cilen) < 0)
	    // goto bad;
	next = p + cilen - 2;

	match (citype) {
	CI_MRU =>
	    if ((go.neg_mru && go.mru != PPP_DEFMRU)
		|| no.neg_mru || cilen != CILEN_SHORT)
		// goto bad;
	    GETSHORT(cishort, p);
	    if (cishort < PPP_DEFMRU) {
		try_.neg_mru = 1;
		try_.mru = cishort;
	    }
	    break;
	CI_ASYNCMAP =>
	    if ((go.neg_asyncmap && go.asyncmap != 0xFFFFFFFF)
		|| no.neg_asyncmap || cilen != CILEN_LONG)
		// goto bad;
	    break;
	CI_AUTHTYPE =>
	    if (0

                || go.neg_chap || no.neg_chap


                || go.neg_upap || no.neg_upap


		|| go.neg_eap || no.neg_eap

		)
		// goto bad;
	    break;
	CI_MAGICNUMBER =>
	    if (go.neg_magicnumber || no.neg_magicnumber ||
		cilen != CILEN_LONG)
		// goto bad;
	    break;
	CI_PCOMPRESSION =>
	    if (go.neg_pcompression || no.neg_pcompression
		|| cilen != CILEN_VOID)
		// goto bad;
	    break;
	CI_ACCOMPRESSION =>
	    if (go.neg_accompression || no.neg_accompression
		|| cilen != CILEN_VOID)
		// goto bad;
	    break;

	CI_QUALITY =>
	    if (go.neg_lqr || no.neg_lqr || cilen != CILEN_LQR)
		// goto bad;
	    break;


	CI_MRRU =>
	    if (go.neg_mrru || no.neg_mrru || cilen != CILEN_SHORT)
		// goto bad;
	    break;

	CI_SSNHF =>
	    if (go.neg_ssnhf || no.neg_ssnhf || cilen != CILEN_VOID)
		// goto bad;
	    try_.neg_ssnhf = 1;
	    break;
	CI_EPDISC =>
	    if (go.neg_endpoint || no.neg_endpoint || cilen < CILEN_CHAR)
		// goto bad;
	    break;
	_ =>
	    break;
	}
	p = next;
    }

    /*
     * OK, the Nak is good.  Now we can update state.
     * If there are any options left we ignore them.
     */
    if (f.state != PPP_FSM_OPENED) {
	if (looped_back) {
	    if (+= 1try_.numloops >= pcb.settings.lcp_loopbackfail) {
		ppp_notice("Serial line is looped back.");
		pcb.err_code = PPPERR_LOOPBACK;
		lcp_close(f.pcb, "Loopback detected");
	    }
	} else
	    try_.numloops = 0;
	*go = try_;
    }

    return 1;

// bad:
    LCPDEBUG(("lcp_nakci: received bad Nak!"));
    return 0;
}


/*
 * lcp_rejci - Peer has Rejected some of our CIs.
 * This should not modify any state if the Reject is bad
 * or if LCP is in the OPENED state.
 *
 * Returns:
 *	0 - Reject was bad.
 *	1 - Reject was good.
 */
pub fn lcp_rejci(f: &mut fsm, u_p: &mut String, len: i32)) -> i32 {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    cichar: u8;
    cishort: u16;
    let cilong: u32;
    lcp_options try_;		/* options to request next time */

    try_ = *go;

    /*
     * Any Rejected CIs must be in exactly the same order that we sent.
     * Check packet length and CI length at each step.
     * If we find any deviations, then this packet is bad.
     */
#define REJCIVOID(opt, neg) \
    if (go.neg && \
	len >= CILEN_VOID && \
	p[1] == CILEN_VOID && \
	p[0] == opt) { \
	len -= CILEN_VOID; \
	INCPTR(CILEN_VOID, p); \
	try_.neg = 0; \
    }
#define REJCISHORT(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_SHORT && \
	p[1] == CILEN_SHORT && \
	p[0] == opt) { \
	len -= CILEN_SHORT; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	/* Check rejected value. */ \
	if (cishort != val) \
	    // goto bad; \
	try_.neg = 0; \
    }


#define REJCICHAP(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_CHAP && \
	p[1] == CILEN_CHAP && \
	p[0] == opt) { \
	len -= CILEN_CHAP; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETCHAR(cichar, p); \
	/* Check rejected value. */ \
	if ((cishort != PPP_CHAP) || (cichar != (CHAP_DIGEST(val)))) \
	    // goto bad; \
	try_.neg = 0; \
	try_.neg_eap = try_.neg_upap = 0; \
    }



#define REJCICHAP(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_CHAP && \
	p[1] == CILEN_CHAP && \
	p[0] == opt) { \
	len -= CILEN_CHAP; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETCHAR(cichar, p); \
	/* Check rejected value. */ \
	if ((cishort != PPP_CHAP) || (cichar != (CHAP_DIGEST(val)))) \
	    // goto bad; \
	try_.neg = 0; \
	try_.neg_upap = 0; \
    }



#define REJCICHAP(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_CHAP && \
	p[1] == CILEN_CHAP && \
	p[0] == opt) { \
	len -= CILEN_CHAP; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETCHAR(cichar, p); \
	/* Check rejected value. */ \
	if ((cishort != PPP_CHAP) || (cichar != (CHAP_DIGEST(val)))) \
	    // goto bad; \
	try_.neg = 0; \
	try_.neg_eap = 0; \
    }



#define REJCICHAP(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_CHAP && \
	p[1] == CILEN_CHAP && \
	p[0] == opt) { \
	len -= CILEN_CHAP; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETCHAR(cichar, p); \
	/* Check rejected value. */ \
	if ((cishort != PPP_CHAP) || (cichar != (CHAP_DIGEST(val)))) \
	    // goto bad; \
	try_.neg = 0; \
    }


#define REJCILONG(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_LONG && \
	p[1] == CILEN_LONG && \
	p[0] == opt) { \
	len -= CILEN_LONG; \
	INCPTR(2, p); \
	GETLONG(cilong, p); \
	/* Check rejected value. */ \
	if (cilong != val) \
	    // goto bad; \
	try_.neg = 0; \
    }

#define REJCILQR(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_LQR && \
	p[1] == CILEN_LQR && \
	p[0] == opt) { \
	len -= CILEN_LQR; \
	INCPTR(2, p); \
	GETSHORT(cishort, p); \
	GETLONG(cilong, p); \
	/* Check rejected value. */ \
	if (cishort != PPP_LQR || cilong != val) \
	    // goto bad; \
	try_.neg = 0; \
    }

#define REJCICBCP(opt, neg, val) \
    if (go.neg && \
	len >= CILEN_CBCP && \
	p[1] == CILEN_CBCP && \
	p[0] == opt) { \
	len -= CILEN_CBCP; \
	INCPTR(2, p); \
	GETCHAR(cichar, p); \
	/* Check rejected value. */ \
	if (cichar != val) \
	    // goto bad; \
	try_.neg = 0; \
    }
#define REJCIENDP(opt, neg, class, val, vlen) \
    if (go.neg && \
	len >= CILEN_CHAR + vlen && \
	p[0] == opt && \
	p[1] == CILEN_CHAR + vlen) { \
	let leti: i32; \
	len -= CILEN_CHAR + vlen; \
	INCPTR(2, p); \
	GETCHAR(cichar, p); \
	if (cichar != class) \
	    // goto bad; \
	for (i = 0; i < vlen; += 1i) { \
	    GETCHAR(cichar, p); \
	    if (cichar != val[i]) \
		// goto bad; \
	} \
	try_.neg = 0; \
    }

    REJCISHORT(CI_MRU, neg_mru, go.mru);
    REJCILONG(CI_ASYNCMAP, neg_asyncmap, go.asyncmap);

    REJCISHORT(CI_AUTHTYPE, neg_eap, PPP_EAP);
    if (!go.neg_eap) {


	REJCICHAP(CI_AUTHTYPE, neg_chap, go.chap_mdtype);
	if (!go.neg_chap) {


	    REJCISHORT(CI_AUTHTYPE, neg_upap, PPP_PAP);


	}


    }


    REJCILQR(CI_QUALITY, neg_lqr, go.lqr_period);

    REJCICBCP(CI_CALLBACK, neg_cbcp, CBCP_OPT);
    REJCILONG(CI_MAGICNUMBER, neg_magicnumber, go.magicnumber);
    REJCIVOID(CI_PCOMPRESSION, neg_pcompression);
    REJCIVOID(CI_ACCOMPRESSION, neg_accompression);

    REJCISHORT(CI_MRRU, neg_mrru, go.mrru);

    REJCIVOID(CI_SSNHF, neg_ssnhf);
    REJCIENDP(CI_EPDISC, neg_endpoint, go.endpoint.class_,
	      go.endpoint.value, go.endpoint.length);

    /*
     * If there are any remaining CIs, then this packet is bad.
     */
    if (len != 0)
	// goto bad;
    /*
     * Now we can update state.
     */
    if (f.state != PPP_FSM_OPENED)
	*go = try_;
    return 1;

// bad:
    LCPDEBUG(("lcp_rejci: received bad Reject!"));
    return 0;
}


/*
 * lcp_reqci - Check the peer's requested CIs and send appropriate response.
 *
 * Returns: CONFACK, CONFNAK or CONFREJ and input packet modified
 * appropriately.  If reject_if_disagree is non-zero, doesn't return
 * CONFNAK; returns CONFREJ if it can't return CONFACK.
 *
 * inp = Requested CIs
 * lenp = Length of requested CIs
 */
pub fn lcp_reqci(f: &mut fsm, u_inp: &mut String, lenp: &mut i32, reject_if_disagree: i32)) -> i32 {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    lcp_options *ho = &pcb.lcp_hisoptions;
    lcp_options *ao = &pcb.lcp_allowoptions;
    let u_cip: &mut String; let next: &mut String;		/* Pointer to current and next CIs */
    let cilen: i32; let citype: i32; let cichar: i32;	/* Parsed len, type, char value */
    cishort: u16;		/* Parsed short value */
    let cilong: u32;		/* Parse long value */
    rc: i32 = CONFACK;		/* Final packet return code */
    let letorc: i32;			/* Individual option return code */
    let mut u_p: &mut String;			/* Pointer to next char to parse */    let mut u_p: &mut String;
    let mut u_rejp: &mut String;		/* Pointer to next char in reject frame */
    let nakp: &mut pbuf;          /* Nak buffer */
    let mut u_nakoutp: &mut String;		/* Pointer to next char in Nak frame */
    l: i32 = *lenp;		/* Length left */

    /*
     * Reset all his options.
     */
    BZERO(ho, sizeof(*ho));

    /*
     * Process all his options.
     */
    next = inp;
    nakp = pbuf_alloc(PBUF_RAW, (PPP_CTRL_PBUF_MAX_SIZE), PPP_CTRL_PBUF_TYPE);
    if(NULL == nakp)
        return 0;
    if(nakp.tot_len != nakp.len) {
        pbuf_free(nakp);
        return 0;
    }

    nakoutp = nakp.payload;
    rejp = inp;
    while (l) {
	orc = CONFACK;			/* Assume success */
	cip = p = next;			/* Remember begining of CI */
	if (l < 2 ||			/* Not enough data for CI header or */
	    p[1] < 2 ||			/*  CI length too small or */
	    p[1] > l) {			/*  CI length too big? */
	    LCPDEBUG(("lcp_reqci: bad CI length!"));
	    orc = CONFREJ;		/* Reject bad CI */
	    cilen = l;			/* Reject till end of packet */
	    l = 0;			/* Don't loop again */
	    citype = 0;
	    // goto endmatch;
	}
	GETCHAR(citype, p);		/* Parse CI type */
	GETCHAR(cilen, p);		/* Parse CI length */
	l -= cilen;			/* Adjust remaining length */
	next += cilen;			/* Step to next CI */

	match (citype) {		/* Check CI type */
	CI_MRU =>
	    if (!ao.neg_mru ||		/* Allow option? */
		cilen != CILEN_SHORT) {	/* Check CI length */
		orc = CONFREJ;		/* Reject CI */
		break;
	    }
	    GETSHORT(cishort, p);	/* Parse MRU */

	    /*
	     * He must be able to receive at least our minimum.
	     * No need to check a maximum.  If he sends a large number,
	     * we'll just ignore it.
	     */
	    if (cishort < PPP_MINMRU) {
		orc = CONFNAK;		/* Nak CI */
		PUTCHAR(CI_MRU, nakoutp);
		PUTCHAR(CILEN_SHORT, nakoutp);
		PUTSHORT(PPP_MINMRU, nakoutp);	/* Give him a hint */
		break;
	    }
	    ho.neg_mru = 1;		/* Remember he sent MRU */
	    ho.mru = cishort;		/* And remember value */
	    break;

	CI_ASYNCMAP =>
	    if (!ao.neg_asyncmap ||
		cilen != CILEN_LONG) {
		orc = CONFREJ;
		break;
	    }
	    GETLONG(cilong, p);

	    /*
	     * Asyncmap must have set at least the bits
	     * which are set in lcp_allowoptions[unit].asyncmap.
	     */
	    if ((ao.asyncmap & !cilong) != 0) {
		orc = CONFNAK;
		PUTCHAR(CI_ASYNCMAP, nakoutp);
		PUTCHAR(CILEN_LONG, nakoutp);
		PUTLONG(ao.asyncmap | cilong, nakoutp);
		break;
	    }
	    ho.neg_asyncmap = 1;
	    ho.asyncmap = cilong;
	    break;

	CI_AUTHTYPE =>
	    if (cilen < CILEN_SHORT ||
		!(0

		|| ao.neg_upap


		|| ao.neg_chap


		|| ao.neg_eap

		)) {
		/*
		 * Reject the option if we're not willing to authenticate.
		 */
		ppp_dbglog("No auth is possible");
		orc = CONFREJ;
		break;
	    }
	    GETSHORT(cishort, p);

	    /*
	     * Authtype must be PAP, CHAP, or EAP.
	     *
	     * Note: if more than one of ao.neg_upap, ao.neg_chap, and
	     * ao.neg_eap are set, and the peer sends a Configure-Request
	     * with two or more authenticate-protocol requests, then we will
	     * reject the second request.
	     * Whether we end up doing CHAP, UPAP, or EAP depends then on
	     * the ordering of the CIs in the peer's Configure-Request.
             */


	    if (cishort == PPP_PAP) {
		/* we've already accepted CHAP or EAP */
		if (0

		    || ho.neg_chap


		    || ho.neg_eap

		    || cilen != CILEN_SHORT) {
		    LCPDEBUG(("lcp_reqci: rcvd AUTHTYPE PAP, rejecting..."));
		    orc = CONFREJ;
		    break;
		}
		if (!ao.neg_upap) {	/* we don't want to do PAP */
		    orc = CONFNAK;	/* NAK it and suggest CHAP or EAP */
		    PUTCHAR(CI_AUTHTYPE, nakoutp);

		    if (ao.neg_eap) {
			PUTCHAR(CILEN_SHORT, nakoutp);
			PUTSHORT(PPP_EAP, nakoutp);
		    } else {


			PUTCHAR(CILEN_CHAP, nakoutp);
			PUTSHORT(PPP_CHAP, nakoutp);
			PUTCHAR(CHAP_DIGEST(ao.chap_mdtype), nakoutp);


		    }

		    break;
		}
		ho.neg_upap = 1;
		break;
	    }


	    if (cishort == PPP_CHAP) {
		/* we've already accepted PAP or EAP */
		if (

		    ho.neg_upap ||


		    ho.neg_eap ||

		    cilen != CILEN_CHAP) {
		    LCPDEBUG(("lcp_reqci: rcvd AUTHTYPE CHAP, rejecting..."));
		    orc = CONFREJ;
		    break;
		}
		if (!ao.neg_chap) {	/* we don't want to do CHAP */
		    orc = CONFNAK;	/* NAK it and suggest EAP or PAP */
		    PUTCHAR(CI_AUTHTYPE, nakoutp);
		    PUTCHAR(CILEN_SHORT, nakoutp);

		    if (ao.neg_eap) {
			PUTSHORT(PPP_EAP, nakoutp);
		    } else


		    if(1) {
			PUTSHORT(PPP_PAP, nakoutp);
		    }
		    else

		    {}
		    break;
		}
		GETCHAR(cichar, p);	/* get digest type */
		if (!(CHAP_CANDIGEST(ao.chap_mdtype, cichar))) {
		    /*
		     * We can't/won't do the requested type,
		     * suggest something else.
		     */
		    orc = CONFNAK;
		    PUTCHAR(CI_AUTHTYPE, nakoutp);
		    PUTCHAR(CILEN_CHAP, nakoutp);
		    PUTSHORT(PPP_CHAP, nakoutp);
		    PUTCHAR(CHAP_DIGEST(ao.chap_mdtype), nakoutp);
		    break;
		}
		ho.chap_mdtype = CHAP_MDTYPE_D(cichar); /* save md type */
		ho.neg_chap = 1;
		break;
	    }


	    if (cishort == PPP_EAP) {
		/* we've already accepted CHAP or PAP */
		if (

		    ho.neg_chap ||


		    ho.neg_upap ||

		    cilen != CILEN_SHORT) {
		    LCPDEBUG(("lcp_reqci: rcvd AUTHTYPE EAP, rejecting..."));
		    orc = CONFREJ;
		    break;
		}
		if (!ao.neg_eap) {	/* we don't want to do EAP */
		    orc = CONFNAK;	/* NAK it and suggest CHAP or PAP */
		    PUTCHAR(CI_AUTHTYPE, nakoutp);

		    if (ao.neg_chap) {
			PUTCHAR(CILEN_CHAP, nakoutp);
			PUTSHORT(PPP_CHAP, nakoutp);
			PUTCHAR(CHAP_DIGEST(ao.chap_mdtype), nakoutp);
		    } else


		    if(1) {
			PUTCHAR(CILEN_SHORT, nakoutp);
			PUTSHORT(PPP_PAP, nakoutp);
		    } else

		    {}
		    break;
		}
		ho.neg_eap = 1;
		break;
	    }


	    /*
	     * We don't recognize the protocol they're asking for.
	     * Nak it with something we're willing to do.
	     * (At this powe: i32 know ao.neg_upap || ao.neg_chap ||
	     * ao.neg_eap.)
	     */
	    orc = CONFNAK;
	    PUTCHAR(CI_AUTHTYPE, nakoutp);


	    if (ao.neg_eap) {
		PUTCHAR(CILEN_SHORT, nakoutp);
		PUTSHORT(PPP_EAP, nakoutp);
	    } else


	    if (ao.neg_chap) {
		PUTCHAR(CILEN_CHAP, nakoutp);
		PUTSHORT(PPP_CHAP, nakoutp);
		PUTCHAR(CHAP_DIGEST(ao.chap_mdtype), nakoutp);
	    } else


	    if(1) {
		PUTCHAR(CILEN_SHORT, nakoutp);
		PUTSHORT(PPP_PAP, nakoutp);
	    } else

	    {}
	    break;


	CI_QUALITY =>
	    if (!ao.neg_lqr ||
		cilen != CILEN_LQR) {
		orc = CONFREJ;
		break;
	    }

	    GETSHORT(cishort, p);
	    GETLONG(cilong, p);

	    /*
	     * Check the protocol and the reporting period.
	     * XXX When should we Nak this, and what with?
	     */
	    if (cishort != PPP_LQR) {
		orc = CONFNAK;
		PUTCHAR(CI_QUALITY, nakoutp);
		PUTCHAR(CILEN_LQR, nakoutp);
		PUTSHORT(PPP_LQR, nakoutp);
		PUTLONG(ao.lqr_period, nakoutp);
		break;
	    }
	    break;


	CI_MAGICNUMBER =>
	    if (!(ao.neg_magicnumber || go.neg_magicnumber) ||
		cilen != CILEN_LONG) {
		orc = CONFREJ;
		break;
	    }
	    GETLONG(cilong, p);

	    /*
	     * He must have a different magic number.
	     */
	    if (go.neg_magicnumber &&
		cilong == go.magicnumber) {
		cilong = magic();	/* Don't put magic() inside macro! */
		orc = CONFNAK;
		PUTCHAR(CI_MAGICNUMBER, nakoutp);
		PUTCHAR(CILEN_LONG, nakoutp);
		PUTLONG(cilong, nakoutp);
		break;
	    }
	    ho.neg_magicnumber = 1;
	    ho.magicnumber = cilong;
	    break;


	CI_PCOMPRESSION =>
	    if (!ao.neg_pcompression ||
		cilen != CILEN_VOID) {
		orc = CONFREJ;
		break;
	    }
	    ho.neg_pcompression = 1;
	    break;

	CI_ACCOMPRESSION =>
	    if (!ao.neg_accompression ||
		cilen != CILEN_VOID) {
		orc = CONFREJ;
		break;
	    }
	    ho.neg_accompression = 1;
	    break;


	CI_MRRU =>
	    if (!ao.neg_mrru
		|| !multilink
		|| cilen != CILEN_SHORT) {
		orc = CONFREJ;
		break;
	    }

	    GETSHORT(cishort, p);
	    /* possibly should insist on a minimum/maximum MRRU here */
	    ho.neg_mrru = 1;
	    ho.mrru = cishort;
	    break;


	CI_SSNHF =>
	    if (!ao.neg_ssnhf

		|| !multilink

		|| cilen != CILEN_VOID) {
		orc = CONFREJ;
		break;
	    }
	    ho.neg_ssnhf = 1;
	    break;

	CI_EPDISC =>
	    if (!ao.neg_endpoint ||
		cilen < CILEN_CHAR ||
		cilen > CILEN_CHAR + MAX_ENDP_LEN) {
		orc = CONFREJ;
		break;
	    }
	    GETCHAR(cichar, p);
	    cilen -= CILEN_CHAR;
	    ho.neg_endpoint = 1;
	    ho.endpoint.class_ = cichar;
	    ho.endpoint.length = cilen;
	    MEMCPY(ho.endpoint.value, p, cilen);
	    INCPTR(cilen, p);
	    break;

	_ =>
	    LCPDEBUG(("lcp_reqci: rcvd unknown option %d", citype));
	    orc = CONFREJ;
	    break;
	}

// endmatch:
	if (orc == CONFACK &&		/* Good CI */
	    rc != CONFACK)		/*  but prior CI wasnt? */
	    continue;			/* Don't send this one */

	if (orc == CONFNAK) {		/* Nak this CI? */
	    if (reject_if_disagree	/* Getting fed up with sending NAKs? */
		&& citype != CI_MAGICNUMBER) {
		orc = CONFREJ;		/* Get tough if so */
	    } else {
		if (rc == CONFREJ)	/* Rejecting prior CI? */
		    continue;		/* Don't send this one */
		rc = CONFNAK;
	    }
	}
	if (orc == CONFREJ) {		/* Reject this CI */
	    rc = CONFREJ;
	    if (cip != rejp)		/* Need to move rejected CI? */
		MEMCPY(rejp, cip, cilen); /* Move it */
	    INCPTR(cilen, rejp);	/* Update output pointer */
	}
    }

    /*
     * If we wanted to send additional NAKs (for unsent CIs), the
     * code would go here.  The extra NAKs would go at *nakoutp.
     * At present there are no cases where we want to ask the
     * peer to negotiate an option.
     */

    match (rc) {
    CONFACK =>
	*lenp = next - inp;
	break;
    CONFNAK =>
	/*
	 * Copy the Nak'd options from the nak buffer to the caller's buffer.
	 */
	*lenp = nakoutp - nakp.payload;
	MEMCPY(inp, nakp.payload, *lenp);
	break;
    CONFREJ =>
	*lenp = rejp - inp;
	break;
    _ =>
	break;
    }

    pbuf_free(nakp);
    LCPDEBUG(("lcp_reqci: returning CONF%s.", CODENAME(rc)));
    return (rc);			/* Return final code */
}


/*
 * lcp_up - LCP has come UP.
 */
pub fn lcp_up(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *wo = &pcb.lcp_wantoptions;
    lcp_options *ho = &pcb.lcp_hisoptions;
    lcp_options *go = &pcb.lcp_// gotoptions;
    lcp_options *ao = &pcb.lcp_allowoptions;
    let mtu i32; let mru: i32;

    if (!go.neg_magicnumber)
	go.magicnumber = 0;
    if (!ho.neg_magicnumber)
	ho.magicnumber = 0;

    /*
     * Set our MTU to the smaller of the MTU we wanted and
     * the MRU our peer wanted.  If we negotiated an MRU,
     * set our MRU to the larger of value we wanted and
     * the value we got in the negotiation.
     * Note on the MTU: the link MTU can be the MRU the peer wanted,
     * the interface MTU is set to the lowest of that, the
     * MTU we want to use, and our link MRU.
     */
    mtu = ho.neg_mru? ho.mru: PPP_MRU;
    mru = go.neg_mru? LWIP_MAX(wo.mru, go.mru): PPP_MRU;

    if (!(multilink && go.neg_mrru && ho.neg_mrru))

	netif_set_mtu(pcb, LWIP_MIN(LWIP_MIN(mtu, mru), ao.mru));
    ppp_send_config(pcb, mtu,
		    (ho.neg_asyncmap? ho.asyncmap: 0xffffffff),
		    ho.neg_pcompression, ho.neg_accompression);
    ppp_recv_config(pcb, mru,
		    (pcb.settings.lax_recv? 0: go.neg_asyncmap? go.asyncmap: 0xffffffff),
		    go.neg_pcompression, go.neg_accompression);

    if (ho.neg_mru)
	pcb.peer_mru = ho.mru;

    lcp_echo_lowerup(f.pcb);  /* Enable echo messages */

    link_established(pcb);
}


/*
 * lcp_down - LCP has gone DOWN.
 *
 * Alert other protocols.
 */
pub fn lcp_down(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;

    lcp_echo_lowerdown(f.pcb);

    link_down(pcb);

    ppp_send_config(pcb, PPP_MRU, 0xffffffff, 0, 0);
    ppp_recv_config(pcb, PPP_MRU,
		    (go.neg_asyncmap? go.asyncmap: 0xffffffff),
		    go.neg_pcompression, go.neg_accompression);
    pcb.peer_mru = PPP_MRU;
}


/*
 * lcp_starting - LCP needs the lower layer up.
 */
pub fn lcp_starting(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    link_required(pcb);
}


/*
 * lcp_finished - LCP has finished with the lower layer.
 */
pub fn lcp_finished(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    link_terminated(pcb);
}



/*
 * lcp_printpkt - prthe: i32 contents of an LCP packet.
 */
static const const: &mut String lcp_codenames[] = {
    "ConfReq", "ConfAck", "ConfNak", "ConfRej",
    "TermReq", "TermAck", "CodeRej", "ProtRej",
    "EchoReq", "EchoRep", "DiscReq", "Ident",
    "TimeRem"
};

static lcp_printpkt: i32( u_p: &mut String, plen: i32,
		void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>) {
    code: i32, id, len, olen, i;
 let u_pstart: &mut String; let optend: &mut String;
    cishort: u16;
    let cilong: u32;

    if (plen < HEADERLEN)
	return 0;
    pstart = p;
    GETCHAR(code, p);
    GETCHAR(id, p);
    GETSHORT(len, p);
    if (len < HEADERLEN || len > plen)
	return 0;

   if (code >= 1 && code <= LWIP_ARRAYSIZE(lcp_codenames))
	printer(arg, " %s", lcp_codenames[code-1]);
    else
	printer(arg, " code=0x%x", code);
    printer(arg, " id=0x%x", id);
    len -= HEADERLEN;
    match (code) {
    CONFREQ =>
    CONFACK =>
    CONFNAK =>
    CONFREJ =>
	/* proption: i32 list */
	while (len >= 2) {
	    GETCHAR(code, p);
	    GETCHAR(olen, p);
	    p -= 2;
	    if (olen < 2 || olen > len) {
		break;
	    }
	    printer(arg, " <");
	    len -= olen;
	    optend = p + olen;
	    match (code) {
	    CI_MRU =>
		if (olen == CILEN_SHORT) {
		    p += 2;
		    GETSHORT(cishort, p);
		    printer(arg, "mru %d", cishort);
		}
		break;
	    CI_ASYNCMAP =>
		if (olen == CILEN_LONG) {
		    p += 2;
		    GETLONG(cilong, p);
		    printer(arg, "asyncmap 0x%x", cilong);
		}
		break;
	    CI_AUTHTYPE =>
		if (olen >= CILEN_SHORT) {
		    p += 2;
		    printer(arg, "auth ");
		    GETSHORT(cishort, p);
		    match (cishort) {

		    PPP_PAP =>
			printer(arg, "pap");
			break;


		    PPP_CHAP =>
			printer(arg, "chap");
			if (p < optend) {
			    match (*p) {
			    CHAP_MD5 =>
				printer(arg, " MD5");
				+= 1p;
				break;

			    CHAP_MICROSOFT =>
				printer(arg, " MS");
				+= 1p;
				break;

			    CHAP_MICROSOFT_V2 =>
				printer(arg, " MS-v2");
				+= 1p;
				break;

			    _ =>
				break;
			    }
			}
			break;


		    PPP_EAP =>
			printer(arg, "eap");
			break;

		    _ =>
			printer(arg, "0x%x", cishort);
		    }
		}
		break;

	    CI_QUALITY =>
		if (olen >= CILEN_SHORT) {
		    p += 2;
		    printer(arg, "quality ");
		    GETSHORT(cishort, p);
		    match (cishort) {
		    PPP_LQR =>
			printer(arg, "lqr");
			break;
		    _ =>
			printer(arg, "0x%x", cishort);
		    }
		}
		break;

	    CI_CALLBACK =>
		if (olen >= CILEN_CHAR) {
		    p += 2;
		    printer(arg, "callback ");
		    GETCHAR(cishort, p);
		    match (cishort) {
		    CBCP_OPT =>
			printer(arg, "CBCP");
			break;
		    _ =>
			printer(arg, "0x%x", cishort);
		    }
		}
		break;
	    CI_MAGICNUMBER =>
		if (olen == CILEN_LONG) {
		    p += 2;
		    GETLONG(cilong, p);
		    printer(arg, "magic 0x%x", cilong);
		}
		break;
	    CI_PCOMPRESSION =>
		if (olen == CILEN_VOID) {
		    p += 2;
		    printer(arg, "pcomp");
		}
		break;
	    CI_ACCOMPRESSION =>
		if (olen == CILEN_VOID) {
		    p += 2;
		    printer(arg, "accomp");
		}
		break;
	    CI_MRRU =>
		if (olen == CILEN_SHORT) {
		    p += 2;
		    GETSHORT(cishort, p);
		    printer(arg, "mrru %d", cishort);
		}
		break;
	    CI_SSNHF =>
		if (olen == CILEN_VOID) {
		    p += 2;
		    printer(arg, "ssnhf");
		}
		break;
	    CI_EPDISC =>

		if (olen >= CILEN_CHAR) {
		    let epd: epdisc;
		    p += 2;
		    GETCHAR(epd.class, p);
		    epd.length = olen - CILEN_CHAR;
		    if (epd.length > MAX_ENDP_LEN)
			epd.length = MAX_ENDP_LEN;
		    if (epd.length > 0) {
			MEMCPY(epd.value, p, epd.length);
			p += epd.length;
		    }
		    printer(arg, "endpoint [%s]", epdisc_to_str(&epd));
		}

		printer(arg, "endpoint");

		break;
	    _ =>
		break;
	    }
	    while (p < optend) {
		GETCHAR(code, p);
		printer(arg, " %.2x", code);
	    }
	    printer(arg, ">");
	}
	break;

    TERMACK =>
    TERMREQ =>
	if (len > 0 && *p >= ' ' && *p < 0x7f) {
	    printer(arg, " ");
	    ppp_print_string(p, len, printer, arg);
	    p += len;
	    len = 0;
	}
	break;

    ECHOREQ =>
    ECHOREP =>
    DISCREQ =>
	if (len >= 4) {
	    GETLONG(cilong, p);
	    printer(arg, " magic=0x%x", cilong);
	    len -= 4;
	}
	break;

    IDENTIF =>
    TIMEREM =>
	if (len >= 4) {
	    GETLONG(cilong, p);
	    printer(arg, " magic=0x%x", cilong);
	    len -= 4;
	}
	if (code == TIMEREM) {
	    if (len < 4)
		break;
	    GETLONG(cilong, p);
	    printer(arg, " seconds=%u", cilong);
	    len -= 4;
	}
	if (len > 0) {
	    printer(arg, " ");
	    ppp_print_string(p, len, printer, arg);
	    p += len;
	    len = 0;
	}
	break;
    _ =>
	break;
    }

    /* prthe: i32 rest of the bytes in the packet */
    for (i = 0; i < len && i < 32; += 1i) {
	GETCHAR(code, p);
	printer(arg, " %.2x", code);
    }
    if (i < len) {
	printer(arg, " ...");
	p += len - i;
    }

    return p - pstart;
}


/*
 * Time to shut down the link because there is nothing out there.
 */

pub fn LcpLinkFailure(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    if (f.state == PPP_FSM_OPENED) {
	ppp_info("No response to %d echo-requests", pcb.lcp_echos_pending);
        ppp_notice("Serial link appears to be disconnected.");
	pcb.err_code = PPPERR_PEERDEAD;
	lcp_close(pcb, "Peer not responding");
    }
}

/*
 * Timer expired for the LCP echo requests from this process.
 */

pub fn LcpEchoCheck(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;

    LcpSendEchoRequest (f);
    if (f.state != PPP_FSM_OPENED)
	return;

    /*
     * Start the timer for the next interval.
     */
    if (pcb.lcp_echo_timer_running)
	ppp_warn("assertion lcp_echo_timer_running==0 failed");
    TIMEOUT (LcpEchoTimeout, f, pcb.settings.lcp_echo_interval);
    pcb.lcp_echo_timer_running = 1;
}

/*
 * LcpEchoTimeout - Timer expired on the LCP echo
 */

pub fn LcpEchoTimeout(arg: &mut Vec<u8>) {
    f: &mut fsm = arg;
    pcb: &mut ppp_pcb = f.pcb;
    if (pcb.lcp_echo_timer_running != 0) {
        pcb.lcp_echo_timer_running = 0;
        LcpEchoCheck (arg);
    }
}

/*
 * LcpEchoReply - LCP has received a reply to the echo
 */

pub fn lcp_received_echo_reply(f: &mut fsm, id: i32, u_inp: &mut String, len: i32) {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    let magic_val: u32;
    

    /* Check the magic number - don't count replies from ourselves. */
    if (len < 4) {
	ppp_dbglog("lcp: received short Echo-Reply, length %d", len);
	return;
    }
    GETLONG(magic_val, inp);
    if (go.neg_magicnumber
	&& magic_val == go.magicnumber) {
	ppp_warn("appear to have received our own echo-reply!");
	return;
    }

    /* Reset the number of outstanding echo frames */
    pcb.lcp_echos_pending = 0;
}

/*
 * LcpSendEchoRequest - Send an echo request frame to the peer
 */

pub fn LcpSendEchoRequest(f: &mut fsm) {
    pcb: &mut ppp_pcb = f.pcb;
    lcp_options *go = &pcb.lcp_// gotoptions;
    let lcp_magic: u32;
    pkt: u8[4], *pktp;

    /*
     * Detect the failure of the peer at this point.
     */
    if (pcb.settings.lcp_echo_fails != 0) {
        if (pcb.lcp_echos_pending >= pcb.settings.lcp_echo_fails) {
            LcpLinkFailure(f);
            pcb.lcp_echos_pending = 0;
	}
    }


    /*
     * If adaptive echos have been enabled, only send the echo request if
     * no traffic was received since the last one.
     */
    if (pcb.settings.lcp_echo_adaptive) {
	static  last_pkts_in: i32 = 0;


	update_link_stats(f.unit);
	link_stats_valid = 0;


	if (link_stats.pkts_in != last_pkts_in) {
	    last_pkts_in = link_stats.pkts_in;
	    return;
	}
    }


    /*
     * Make and send the echo request frame.
     */
    if (f.state == PPP_FSM_OPENED) {
        lcp_magic = go.magicnumber;
	pktp = pkt;
	PUTLONG(lcp_magic, pktp);
        fsm_sdata(f, ECHOREQ, pcb.lcp_echo_number+= 1, pkt, pktp - pkt);
	+= 1pcb.lcp_echos_pending;
    }
}

/*
 * lcp_echo_lowerup - Start the timer for the LCP frame
 */

pub fn lcp_echo_lowerup(pcb: &mut ppp_pcb) {
    f: &mut fsm = &pcb.lcp_fsm;

    /* Clear the parameters for generating echo frames */
    pcb.lcp_echos_pending      = 0;
    pcb.lcp_echo_number        = 0;
    pcb.lcp_echo_timer_running = 0;
  
    /* If a timeout interval is specified then start the timer */
    if (pcb.settings.lcp_echo_interval != 0)
        LcpEchoCheck (f);
}

/*
 * lcp_echo_lowerdown - Stop the timer for the LCP frame
 */

pub fn lcp_echo_lowerdown(pcb: &mut ppp_pcb) {
    f: &mut fsm = &pcb.lcp_fsm;

    if (pcb.lcp_echo_timer_running != 0) {
        UNTIMEOUT (LcpEchoTimeout, f);
        pcb.lcp_echo_timer_running = 0;
    }
}


