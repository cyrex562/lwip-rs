/*
 * upap.c - User/Password Authentication Protocol.
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
 * Command-line options.
 */
static option_t pap_option_list[] = {
    { "hide-password", o_bool, &hide_password,
      "Don't output passwords to log", OPT_PRIO | 1 },
    { "show-password", o_bool, &hide_password,
      "Show password string in debug log messages", OPT_PRIOSUB | 0 },

    { "pap-restart", o_int, &upap[0].us_timeouttime,
      "Set retransmit timeout for PAP", OPT_PRIO },
    { "pap-max-authreq", o_int, &upap[0].us_maxtransmits,
      "Set max number of transmissions for auth-reqs", OPT_PRIO },
    { "pap-timeout", o_int, &upap[0].us_reqtimeout,
      "Set time limit for peer PAP authentication", OPT_PRIO },

    { None }
};


/*
 * Protocol entry points.
 */
pub fn upap_init(pcb: &mut ppp_pcb);
pub fn upap_lowerup(pcb: &mut ppp_pcb);
pub fn upap_lowerdown(pcb: &mut ppp_pcb);
pub fn upap_input(pcb: &mut ppp_pcb, u_inpacket: &mut String, l: i32);
pub fn upap_protrej(pcb: &mut ppp_pcb);

pub fn upap_printpkt( u_p: &mut String, plen: i32, void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>)) -> i32;


const struct protent pap_protent = {
    PPP_PAP,
    upap_init,
    upap_input,
    upap_protrej,
    upap_lowerup,
    upap_lowerdown,
    None,
    None,

    upap_printpkt,


    None,


    "PAP",
    None,


    pap_option_list,
    None,


    None,
    None

};

pub fn upap_timeout(arg: &mut Vec<u8>);

pub fn upap_reqtimeout(arg: &mut Vec<u8>);
pub fn upap_rauthreq(pcb: &mut ppp_pcb, u_inp: &mut String, id: i32, len: i32);

pub fn upap_rauthack(pcb: &mut ppp_pcb, u_inp: &mut String, id: i32, len: i32);
pub fn upap_rauthnak(pcb: &mut ppp_pcb, u_inp: &mut String, id: i32, len: i32);
pub fn upap_sauthreq(pcb: &mut ppp_pcb);

pub fn upap_sresp(pcb: &mut ppp_pcb, u_char code, u_char id, msg: &String, msglen: i32);



/*
 * upap_init - Initialize a UPAP unit.
 */
pub fn upap_init(pcb: &mut ppp_pcb) {
    pcb.upap.us_user = None;
    pcb.upap.us_userlen = 0;
    pcb.upap.us_passwd = None;
    pcb.upap.us_passwdlen = 0;
    pcb.upap.us_clientstate = UPAPCS_INITIAL;

    pcb.upap.us_serverstate = UPAPSS_INITIAL;

    pcb.upap.us_id = 0;
}


/*
 * upap_authwithpeer - Authenticate us with our peer (start client).
 *
 * Set new state and send authenticate's.
 */
pub fn  upap_authwithpeer(pcb: &mut ppp_pcb, user: &String, password: &String) {

    if(!user || !password)
        return;

    /* Save the username and password we're given */
    pcb.upap.us_user = user;
    pcb.upap.us_userlen = LWIP_MIN(strlen(user), 0xff);
    pcb.upap.us_passwd = password;
    pcb.upap.us_passwdlen = LWIP_MIN(strlen(password), 0xff);
    pcb.upap.us_transmits = 0;

    /* Lower layer up yet? */
    if (pcb.upap.us_clientstate == UPAPCS_INITIAL ||
	pcb.upap.us_clientstate == UPAPCS_PENDING) {
	pcb.upap.us_clientstate = UPAPCS_PENDING;
	return;
    }

    upap_sauthreq(pcb);		/* Start protocol */
}


/*
 * upap_authpeer - Authenticate our peer (start server).
 *
 * Set new state.
 */
pub fn  upap_authpeer(pcb: &mut ppp_pcb) {

    /* Lower layer up yet? */
    if (pcb.upap.us_serverstate == UPAPSS_INITIAL ||
	pcb.upap.us_serverstate == UPAPSS_PENDING) {
	pcb.upap.us_serverstate = UPAPSS_PENDING;
	return;
    }

    pcb.upap.us_serverstate = UPAPSS_LISTEN;
    if (pcb.settings.pap_req_timeout > 0)
	TIMEOUT(upap_reqtimeout, pcb, pcb.settings.pap_req_timeout);
}


/*
 * upap_timeout - Retransmission timer for sending auth-reqs expired.
 */
pub fn upap_timeout(arg: &mut Vec<u8>) {
    pcb: &mut ppp_pcb = arg;

    if (pcb.upap.us_clientstate != UPAPCS_AUTHREQ)
	return;

    if (pcb.upap.us_transmits >= pcb.settings.pap_max_transmits) {
	/* give up in disgust */
	ppp_error("No response to PAP authenticate-requests");
	pcb.upap.us_clientstate = UPAPCS_BADAUTH;
	auth_withpeer_fail(pcb, PPP_PAP);
	return;
    }

    upap_sauthreq(pcb);		/* Send Authenticate-Request */
}



/*
 * upap_reqtimeout - Give up waiting for the peer to send an auth-req.
 */
pub fn upap_reqtimeout(arg: &mut Vec<u8>) {
    pcb: &mut ppp_pcb = arg;

    if (pcb.upap.us_serverstate != UPAPSS_LISTEN)
	return;			/* huh?? */

    auth_peer_fail(pcb, PPP_PAP);
    pcb.upap.us_serverstate = UPAPSS_BADAUTH;
}



/*
 * upap_lowerup - The lower layer is up.
 *
 * Start authenticating if pending.
 */
pub fn upap_lowerup(pcb: &mut ppp_pcb) {

    if (pcb.upap.us_clientstate == UPAPCS_INITIAL)
	pcb.upap.us_clientstate = UPAPCS_CLOSED;
    else if (pcb.upap.us_clientstate == UPAPCS_PENDING) {
	upap_sauthreq(pcb);	/* send an auth-request */
    }


    if (pcb.upap.us_serverstate == UPAPSS_INITIAL)
	pcb.upap.us_serverstate = UPAPSS_CLOSED;
    else if (pcb.upap.us_serverstate == UPAPSS_PENDING) {
	pcb.upap.us_serverstate = UPAPSS_LISTEN;
	if (pcb.settings.pap_req_timeout > 0)
	    TIMEOUT(upap_reqtimeout, pcb, pcb.settings.pap_req_timeout);
    }

}


/*
 * upap_lowerdown - The lower layer is down.
 *
 * Cancel all timeouts.
 */
pub fn upap_lowerdown(pcb: &mut ppp_pcb) {

    if (pcb.upap.us_clientstate == UPAPCS_AUTHREQ)	/* Timeout pending? */
	UNTIMEOUT(upap_timeout, pcb);		/* Cancel timeout */

    if (pcb.upap.us_serverstate == UPAPSS_LISTEN && pcb.settings.pap_req_timeout > 0)
	UNTIMEOUT(upap_reqtimeout, pcb);


    pcb.upap.us_clientstate = UPAPCS_INITIAL;

    pcb.upap.us_serverstate = UPAPSS_INITIAL;

}


/*
 * upap_protrej - Peer doesn't speak this protocol.
 *
 * This shouldn't happen.  In any case, pretend lower layer went down.
 */
pub fn upap_protrej(pcb: &mut ppp_pcb) {

    if (pcb.upap.us_clientstate == UPAPCS_AUTHREQ) {
	ppp_error("PAP authentication failed due to protocol-reject");
	auth_withpeer_fail(pcb, PPP_PAP);
    }

    if (pcb.upap.us_serverstate == UPAPSS_LISTEN) {
	ppp_error("PAP authentication of peer failed (protocol-reject)");
	auth_peer_fail(pcb, PPP_PAP);
    }

    upap_lowerdown(pcb);
}


/*
 * upap_input - Input UPAP packet.
 */
pub fn upap_input(pcb: &mut ppp_pcb, u_inpacket: &mut String, l: i32) {
    let mut u_inp: &mut String;
    u_char code, id;
    let letlen: i32;

    /*
     * Parse header (code, id and length).
     * If packet too short, drop it.
     */
    inp = inpacket;
    if (l < UPAP_HEADERLEN) {
	UPAPDEBUG(("pap_input: rcvd short header."));
	return;
    }
    GETCHAR(code, inp);
    GETCHAR(id, inp);
    GETSHORT(len, inp);
    if (len < UPAP_HEADERLEN) {
	UPAPDEBUG(("pap_input: rcvd illegal length."));
	return;
    }
    if (len > l) {
	UPAPDEBUG(("pap_input: rcvd short packet."));
	return;
    }
    len -= UPAP_HEADERLEN;

    /*
     * Action depends on code.
     */
    match (code) {
    UPAP_AUTHREQ =>

	upap_rauthreq(pcb, inp, id, len);

	break;

    UPAP_AUTHACK =>
	upap_rauthack(pcb, inp, id, len);
	break;

    UPAP_AUTHNAK =>
	upap_rauthnak(pcb, inp, id, len);
	break;

    _ =>				/* XXX Need code reject */
	break;
    }
}


/*
 * upap_rauth - Receive Authenticate.
 */
pub fn upap_rauthreq(pcb: &mut ppp_pcb, u_inp: &mut String, id: i32, len: i32) {
    u_char ruserlen, rpasswdlen;
    let mut ruser: &mut String;
    let mut rpasswd: &mut String;
    let rhostname: String;
    let letretcode: i32;
     let msg: String;
    let letmsglen: i32;

    if (pcb.upap.us_serverstate < UPAPSS_LISTEN)
	return;

    /*
     * If we receive a duplicate authenticate-request, we are
     * supposed to return the same status as for the first request.
     */
    if (pcb.upap.us_serverstate == UPAPSS_OPEN) {
	upap_sresp(pcb, UPAP_AUTHACK, id, "", 0);	/* return auth-ack */
	return;
    }
    if (pcb.upap.us_serverstate == UPAPSS_BADAUTH) {
	upap_sresp(pcb, UPAP_AUTHNAK, id, "", 0);	/* return auth-nak */
	return;
    }

    /*
     * Parse user/passwd.
     */
    if (len < 1) {
	UPAPDEBUG(("pap_rauth: rcvd short packet."));
	return;
    }
    GETCHAR(ruserlen, inp);
    len -= sizeof (u_char) + ruserlen + sizeof (u_char);
    if (len < 0) {
	UPAPDEBUG(("pap_rauth: rcvd short packet."));
	return;
    }
    ruser =  inp;
    INCPTR(ruserlen, inp);
    GETCHAR(rpasswdlen, inp);
    if (len < rpasswdlen) {
	UPAPDEBUG(("pap_rauth: rcvd short packet."));
	return;
    }

    rpasswd =  inp;

    /*
     * Check the username and password given.
     */
    retcode = UPAP_AUTHNAK;
    if (auth_check_passwd(pcb, ruser, ruserlen, rpasswd, rpasswdlen, &msg, &msglen)) {
      retcode = UPAP_AUTHACK;
    }
    BZERO(rpasswd, rpasswdlen);


    /*
     * Check remote number authorization.  A plugin may have filled in
     * the remote number or added an allowed number, and rather than
     * return an authenticate failure, is leaving it for us to verify.
     */
    if (retcode == UPAP_AUTHACK) {
	if (!auth_number()) {
	    /* We do not want to leak info about the pap result. */
	    retcode = UPAP_AUTHNAK; /* XXX exit value will be "wrong" */
	    warn("calling number %q is not authorized", remote_number);
	}
    }

    msglen = strlen(msg);
    if (msglen > 255)
	msglen = 255;


    upap_sresp(pcb, retcode, id, msg, msglen);

    /* Null terminate and clean remote name. */
    ppp_slprintf(rhostname, sizeof(rhostname), "%.*v", ruserlen, ruser);

    if (retcode == UPAP_AUTHACK) {
	pcb.upap.us_serverstate = UPAPSS_OPEN;
	ppp_notice("PAP peer authentication succeeded for %q", rhostname);
	auth_peer_success(pcb, PPP_PAP, 0, ruser, ruserlen);
    } else {
	pcb.upap.us_serverstate = UPAPSS_BADAUTH;
	ppp_warn("PAP peer authentication failed for %q", rhostname);
	auth_peer_fail(pcb, PPP_PAP);
    }

    if (pcb.settings.pap_req_timeout > 0)
	UNTIMEOUT(upap_reqtimeout, pcb);
}


/*
 * upap_rauthack - Receive Authenticate-Ack.
 */
pub fn upap_rauthack(pcb: &mut ppp_pcb, u_inp: &mut String, id: i32, len: i32) {
    u_char msglen;
     let msg: &mut String;
    

    if (pcb.upap.us_clientstate != UPAPCS_AUTHREQ) /* XXX */
	return;

    /*
     * Parse message.
     */
    if (len < 1) {
	UPAPDEBUG(("pap_rauthack: ignoring missing msg-length."));
    } else {
	GETCHAR(msglen, inp);
	if (msglen > 0) {
	    len -= sizeof (u_char);
	    if (len < msglen) {
		UPAPDEBUG(("pap_rauthack: rcvd short packet."));
		return;
	    }
	    msg =  inp;
	    PRINTMSG(msg, msglen);
	}
    }

    pcb.upap.us_clientstate = UPAPCS_OPEN;

    auth_withpeer_success(pcb, PPP_PAP, 0);
}


/*
 * upap_rauthnak - Receive Authenticate-Nak.
 */
pub fn upap_rauthnak(pcb: &mut ppp_pcb, u_inp: &mut String, id: i32, len: i32) {
    u_char msglen;
     let msg: &mut String;
    

    if (pcb.upap.us_clientstate != UPAPCS_AUTHREQ) /* XXX */
	return;

    /*
     * Parse message.
     */
    if (len < 1) {
	UPAPDEBUG(("pap_rauthnak: ignoring missing msg-length."));
    } else {
	GETCHAR(msglen, inp);
	if (msglen > 0) {
	    len -= sizeof (u_char);
	    if (len < msglen) {
		UPAPDEBUG(("pap_rauthnak: rcvd short packet."));
		return;
	    }
	    msg =  inp;
	    PRINTMSG(msg, msglen);
	}
    }

    pcb.upap.us_clientstate = UPAPCS_BADAUTH;

    ppp_error("PAP authentication failed");
    auth_withpeer_fail(pcb, PPP_PAP);
}


/*
 * upap_sauthreq - Send an Authenticate-Request.
 */
pub fn upap_sauthreq(pcb: &mut ppp_pcb) {
    let p: &mut pbuf;
    let mut u_outp: &mut String;
    let letoutlen: i32;

    outlen = UPAP_HEADERLEN + 2 * sizeof (u_char) +
	pcb.upap.us_userlen + pcb.upap.us_passwdlen;
    p = pbuf_alloc(PBUF_RAW, (PPP_HDRLEN +outlen), PPP_CTRL_PBUF_TYPE);
    if(None == p)
        return;
    if(p.tot_len != p.len) {
        pbuf_free(p);
        return;
    }

    outp = p.payload;
    MAKEHEADER(outp, PPP_PAP);

    PUTCHAR(UPAP_AUTHREQ, outp);
    PUTCHAR(+= 1pcb.upap.us_id, outp);
    PUTSHORT(outlen, outp);
    PUTCHAR(pcb.upap.us_userlen, outp);
    MEMCPY(outp, pcb.upap.us_user, pcb.upap.us_userlen);
    INCPTR(pcb.upap.us_userlen, outp);
    PUTCHAR(pcb.upap.us_passwdlen, outp);
    MEMCPY(outp, pcb.upap.us_passwd, pcb.upap.us_passwdlen);

    ppp_write(pcb, p);

    TIMEOUT(upap_timeout, pcb, pcb.settings.pap_timeout_time);
    += 1pcb.upap.us_transmits;
    pcb.upap.us_clientstate = UPAPCS_AUTHREQ;
}


/*
 * upap_sresp - Send a response (ack or nak).
 */
pub fn upap_sresp(pcb: &mut ppp_pcb, u_char code, u_char id, msg: &String, msglen: i32) {
    let p: &mut pbuf;
    let mut u_outp: &mut String;
    let letoutlen: i32;

    outlen = UPAP_HEADERLEN + sizeof (u_char) + msglen;
    p = pbuf_alloc(PBUF_RAW, (PPP_HDRLEN +outlen), PPP_CTRL_PBUF_TYPE);
    if(None == p)
        return;
    if(p.tot_len != p.len) {
        pbuf_free(p);
        return;
    }

    outp = p.payload;
    MAKEHEADER(outp, PPP_PAP);

    PUTCHAR(code, outp);
    PUTCHAR(id, outp);
    PUTSHORT(outlen, outp);
    PUTCHAR(msglen, outp);
    MEMCPY(outp, msg, msglen);

    ppp_write(pcb, p);
}



/*
 * upap_printpkt - prthe: i32 contents of a PAP packet.
 */
static const const: &mut String upap_codenames[] = {
    "AuthReq", "AuthAck", "AuthNak"
};

pub fn upap_printpkt( u_p: &mut String, plen: i32, void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>)) -> i32 {
    let code: i32; let id: i32; let len: i32;
    let mlen: i32; let ulen: i32; let wlen: i32;
 u_user: &mut String, *pwd, *msg;
 let mut u_pstart: &mut String;

    if (plen < UPAP_HEADERLEN)
	return 0;
    pstart = p;
    GETCHAR(code, p);
    GETCHAR(id, p);
    GETSHORT(len, p);
    if (len < UPAP_HEADERLEN || len > plen)
	return 0;

    if (code >= 1 && code <= LWIP_ARRAYSIZE(upap_codenames))
	printer(arg, " %s", upap_codenames[code-1]);
    else
	printer(arg, " code=0x%x", code);
    printer(arg, " id=0x%x", id);
    len -= UPAP_HEADERLEN;
    match (code) {
    UPAP_AUTHREQ =>
	if (len < 1)
	    break;
	ulen = p[0];
	if (len < ulen + 2)
	    break;
	wlen = p[ulen + 1];
	if (len < ulen + wlen + 2)
	    break;
	user =  (p + 1);
	pwd =  (p + ulen + 2);
	p += ulen + wlen + 2;
	len -= ulen + wlen + 2;
	printer(arg, " user=");
	ppp_print_string(user, ulen, printer, arg);
	printer(arg, " password=");
/* FIXME: require ppp_pcb struct as printpkt() argument */

	if (!pcb.settings.hide_password)

	    ppp_print_string(pwd, wlen, printer, arg);

	else
	    printer(arg, "<hidden>");

	break;
    UPAP_AUTHACK =>
    UPAP_AUTHNAK =>
	if (len < 1)
	    break;
	mlen = p[0];
	if (len < mlen + 1)
	    break;
	msg =  (p + 1);
	p += mlen + 1;
	len -= mlen + 1;
	printer(arg, " ");
	ppp_print_string(msg, mlen, printer, arg);
	break;
    _ =>
	break;
    }

    /* prthe: i32 rest of the bytes in the packet */
    for (; len > 0; --len) {
	GETCHAR(code, p);
	printer(arg, " %.2x", code);
    }

    return p - pstart;
}



