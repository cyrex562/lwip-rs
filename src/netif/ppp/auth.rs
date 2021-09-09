/*
 * auth.c - PPP authentication and phase control.
 *
 * Copyright (c) 1993-2002 Paul Mackerras. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * 3. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by Paul Mackerras
 *     <paulus@samba.org>".
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * Derived from main.c, which is:
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

pub const PW_PPP: u32 = PW_LOGIN;

/* Bits in scan_authfile return value */
pub const NONWILD_SERVER: u32 = 1;
pub const NONWILD_CLIENT: u32 = 2;

// #define ISWILD(word)	(word[0] == '*' && word[1] == 0)
pub fn ISWILD(word: &[char; 2]) -> bool {
    word[0] == '*' && word[1] == 0
}

/* List of addresses which the peer may use. */
// static addresses: &mut permitted_ip[NUM_PPP];

/* Wordlist giving addresses which the peer may use
without authenticating itself. */
// static noauth_addrs: &mut wordlist;

/* Remote telephone number, if available */
// char remote_number[MAXNAMELEN];

/* Wordlist giving remote telephone numbers which may connect. */
// static permitted_numbers: &mut wordlist;

/* Extra options to apply, from the secrets file entry for the peer. */
// static extra_options: &mut wordlist;

/* Set if we require authentication only because we have a default route. */
// static bool default_auth;

/* Hook to enable a plugin to control the idle time limit */
// int (*idle_time_hook) (struct ppp_idle *) = None;

/* Hook for a plugin to say whether we can possibly authenticate any peer */
// int (*pap_check_hook) () = None;

/* Hook for a plugin to check the PAP user and password */
// int (*pap_auth_hook) (user: &mut String, passwd: &mut String, msgp: &mut String,
// 			  struct wordlist **paddrs,
// 			  struct wordlist **popts) = None;

/* Hook for a plugin to know about the PAP user logout */
// pub fn  (*pap_logout_hook) () = None;

/* Hook for a plugin to get the PAP password for authenticating us */
// int (*pap_passwd_hook) (user: &mut String, passwd: &mut String) = None;

/* Hook for a plugin to say if we can possibly authenticate a peer using CHAP */
// int (*chap_check_hook) () = None;

/* Hook for a plugin to get the CHAP password for authenticating us */
// int (*chap_passwd_hook) (user: &mut String, passwd: &mut String) = None;

/* Hook for a plugin to say whether it is OK if the peer
refuses to authenticate. */
// int (*None_auth_hook) (struct wordlist **paddrs,
// 			   struct wordlist **popts) = None;

// int (*allowed_address_hook) (addr: u32) = None;

/* Hook for plugin to hear when an interface joins a multilink bundle */
// pub fn  (*multilink_join_hook) () = None;

/* A notifier for when the peer has authenticated itself,
and we are proceeding to the network phase. */
// auth_up_notifier: &mut notifier = None;

/* A notifier for when the link goes down. */
// link_down_notifier: &mut notifier = None;

/*
 * Option variables.
 */

pub const uselogin: bool = 0; /* Use /etc/passwd for checking PAP */
pub const session_mgmt: bool = 0; /* Do session management (login records) */
pub const cryptpap: bool = 0; /* Passwords in pap-secrets are encrypted */
pub const refuse_pap: bool = 0; /* Don't wanna auth. ourselves with PAP */
pub const refuse_chap: bool = 0; /* Don't wanna auth. ourselves with CHAP */
pub const refuse_eap: bool = 0; /* Don't wanna auth. ourselves with EAP */

pub const refuse_mschap: bool = 0; /* Don't wanna auth. ourselves with MS-CHAP */
pub const refuse_mschap_v2: bool = 0; /* Don't wanna auth. ourselves with MS-CHAPv2 */
/* MSCHAP_SUPPORT */
pub const refuse_mschap: bool = 1; /* Don't wanna auth. ourselves with MS-CHAP */
pub const refuse_mschap_v2: bool = 1; /* Don't wanna auth. ourselves with MS-CHAPv2 */

pub const usehostname: bool = 0; /* Use hostname for our_name */
pub const auth_required: bool = 0; /* Always require authentication from peer */
pub const allow_any_ip: bool = 0; /* Allow peer to use any IP address */
pub const explicit_remote: bool = 0; /* User specified explicit remote name */
pub const explicit_user: bool = 0; /* Set if "user" option supplied */
pub const explicit_passwd: bool = 0; /* Set if "password" option supplied */
// char remote_name[MAXNAMELEN];	/* Peer's name for authentication */
// static uafname: &mut String;		/* name of most recent +ua file */
// extern crypt: &mut String ( char *,  char *);

/* Prototypes for procedures local to this file. */

// pub fn network_phase(pcb: &mut ppp_pcb);

// pub fn check_idle(arg: &mut Vec<u8>);

// pub fn connect_time_expired(arg: &mut Vec<u8>);

// static int  None_login ;
/* static int  get_pap_passwd ; */
// static int  have_pap_secret ;
// static int  have_chap_secret (char *, char *, int, int *);
// static int  have_srp_secret (client: &mut String, server: &mut String, need_ip: i32,
//     lacks_ipp: &mut i32);
// static int  ip_addr_check (u32, struct permitted_ip *);
// static int  scan_authfile (FILE *, char *, char *, char *,
// 			       struct wordlist **, struct wordlist **,
// 			       char *, int);
// pub fn free_wordlist ;
// pub fn set_allowed_addrs (int, struct wordlist *, struct wordlist *);
// static int  some_ip_ok ;
// static int  setupapfile ;
// static int  privgroup ;
// static int  set_noauth_addr ;
// static int  set_permitted_number ;
// pub fn check_access (FILE *, char *);
// static int  wordlist_count ;

// pub fn check_maxoctets ;

/*
 * Authentication-related options.
 */
// option_t auth_options[] = {
//     { "auth", o_bool, &auth_required,
//       "Require authentication from peer", OPT_PRIO | 1 },
//     { "noauth", o_bool, &auth_required,
//       "Don't require peer to authenticate", OPT_PRIOSUB | OPT_PRIV,
//       &allow_any_ip },
//     { "require-pap", o_bool, &lcp_wantoptions[0].neg_upap,
//       "Require PAP authentication from peer",
//       OPT_PRIOSUB | 1, &auth_required },
//     { "+pap", o_bool, &lcp_wantoptions[0].neg_upap,
//       "Require PAP authentication from peer",
//       OPT_ALIAS | OPT_PRIOSUB | 1, &auth_required },
//     { "require-chap", o_bool, &auth_required,
//       "Require CHAP authentication from peer",
//       OPT_PRIOSUB | OPT_A2OR | MDTYPE_MD5,
//       &lcp_wantoptions[0].chap_mdtype },
//     { "+chap", o_bool, &auth_required,
//       "Require CHAP authentication from peer",
//       OPT_ALIAS | OPT_PRIOSUB | OPT_A2OR | MDTYPE_MD5,
//       &lcp_wantoptions[0].chap_mdtype },

//     { "require-mschap", o_bool, &auth_required,
//       "Require MS-CHAP authentication from peer",
//       OPT_PRIOSUB | OPT_A2OR | MDTYPE_MICROSOFT,
//       &lcp_wantoptions[0].chap_mdtype },
//     { "+mschap", o_bool, &auth_required,
//       "Require MS-CHAP authentication from peer",
//       OPT_ALIAS | OPT_PRIOSUB | OPT_A2OR | MDTYPE_MICROSOFT,
//       &lcp_wantoptions[0].chap_mdtype },
//     { "require-mschap-v2", o_bool, &auth_required,
//       "Require MS-CHAPv2 authentication from peer",
//       OPT_PRIOSUB | OPT_A2OR | MDTYPE_MICROSOFT_V2,
//       &lcp_wantoptions[0].chap_mdtype },
//     { "+mschap-v2", o_bool, &auth_required,
//       "Require MS-CHAPv2 authentication from peer",
//       OPT_ALIAS | OPT_PRIOSUB | OPT_A2OR | MDTYPE_MICROSOFT_V2,
//       &lcp_wantoptions[0].chap_mdtype },

//     { "refuse-pap", o_bool, &refuse_pap,
//       "Don't agree to auth to peer with PAP", 1 },
//     { "-pap", o_bool, &refuse_pap,
//       "Don't allow PAP authentication with peer", OPT_ALIAS | 1 },
//     { "refuse-chap", o_bool, &refuse_chap,
//       "Don't agree to auth to peer with CHAP",
//       OPT_A2CLRB | MDTYPE_MD5,
//       &lcp_allowoptions[0].chap_mdtype },
//     { "-chap", o_bool, &refuse_chap,
//       "Don't allow CHAP authentication with peer",
//       OPT_ALIAS | OPT_A2CLRB | MDTYPE_MD5,
//       &lcp_allowoptions[0].chap_mdtype },

//     { "refuse-mschap", o_bool, &refuse_mschap,
//       "Don't agree to auth to peer with MS-CHAP",
//       OPT_A2CLRB | MDTYPE_MICROSOFT,
//       &lcp_allowoptions[0].chap_mdtype },
//     { "-mschap", o_bool, &refuse_mschap,
//       "Don't allow MS-CHAP authentication with peer",
//       OPT_ALIAS | OPT_A2CLRB | MDTYPE_MICROSOFT,
//       &lcp_allowoptions[0].chap_mdtype },
//     { "refuse-mschap-v2", o_bool, &refuse_mschap_v2,
//       "Don't agree to auth to peer with MS-CHAPv2",
//       OPT_A2CLRB | MDTYPE_MICROSOFT_V2,
//       &lcp_allowoptions[0].chap_mdtype },
//     { "-mschap-v2", o_bool, &refuse_mschap_v2,
//       "Don't allow MS-CHAPv2 authentication with peer",
//       OPT_ALIAS | OPT_A2CLRB | MDTYPE_MICROSOFT_V2,
//       &lcp_allowoptions[0].chap_mdtype },

//     { "require-eap", o_bool, &lcp_wantoptions[0].neg_eap,
//       "Require EAP authentication from peer", OPT_PRIOSUB | 1,
//       &auth_required },

//     { "refuse-eap", o_bool, &refuse_eap,
//       "Don't agree to authenticate to peer with EAP", 1 },

//     { "name", o_string, our_name,
//       "Set local name for authentication",
//       OPT_PRIO | OPT_PRIV | OPT_STATIC, None, MAXNAMELEN },

//     { "+ua", o_special, setupapfile,
//       "Get PAP user and password from file",
//       OPT_PRIO | OPT_A2STRVAL, &uafname },

//     { "user", o_string, user,
//       "Set name for auth with peer", OPT_PRIO | OPT_STATIC,
//       &explicit_user, MAXNAMELEN },

//     { "password", o_string, passwd,
//       "Password for authenticating us to the peer",
//       OPT_PRIO | OPT_STATIC | OPT_HIDE,
//       &explicit_passwd, MAXSECRETLEN },

//     { "usehostname", o_bool, &usehostname,
//       "Must use hostname for authentication", 1 },

//     { "remotename", o_string, remote_name,
//       "Set remote name for authentication", OPT_PRIO | OPT_STATIC,
//       &explicit_remote, MAXNAMELEN },

//     { "login", o_bool, &uselogin,
//       "Use system password database for PAP", OPT_A2COPY | 1 ,
//       &session_mgmt },
//     { "enable-session", o_bool, &session_mgmt,
//       "Enable session accounting for remote peers", OPT_PRIV | 1 },

//     { "papcrypt", o_bool, &cryptpap,
//       "PAP passwords are encrypted", 1 },

//     { "privgroup", o_special, privgroup,
//       "Allow group members to use privileged options", OPT_PRIV | OPT_A2LIST },

//     { "allow-ip", o_special, set_noauth_addr,
//       "Set IP address(es) which can be used without authentication",
//       OPT_PRIV | OPT_A2LIST },

//     { "remotenumber", o_string, remote_number,
//       "Set remote telephone number for authentication", OPT_PRIO | OPT_STATIC,
//       None, MAXNAMELEN },

//     { "allow-number", o_special, set_permitted_number,
//       "Set telephone number(s) which are allowed to connect",
//       OPT_PRIV | OPT_A2LIST },

//     { None }
// };

/*
 * setupapfile - specifies UPAP info for authenticating with peer.
 */
pub fn setupapfile(argv: &mut Vec<String>) {
    let ufile: FILE;
    let letl: i32;
    let euid: uid_t;
    // char u[MAXNAMELEN], p[MAXSECRETLEN];
    let u: String;
    let p: String;
    let mut fname: &mut String;

    lcp_allowoptions[0].neg_upap = 1;

    /* open user info file */
    fname = strdup(*argv);
    if (fname == None) {
        novm("+ua file name");
    }
    euid = geteuid();
    if (seteuid(getuid()) == -1) {
        option_error("unable to reset uid before opening %s: %m", fname);
        return 0;
    }
    ufile = fopen(fname, "r");
    if (seteuid(euid) == -1) {
        fatal("unable to regain privileges: %m");
    }
    if (ufile == None) {
        option_error("unable to open user login data file %s", fname);
        return 0;
    }
    check_access(ufile, fname);
    uafname = fname;

    /* get username */
    if (fgets(u, MAXNAMELEN - 1, ufile) == None || fgets(p, MAXSECRETLEN - 1, ufile) == None) {
        fclose(ufile);
        option_error("unable to read user login data file %s", fname);
        return 0;
    }
    fclose(ufile);

    /* get rid of newlines */
    l = strlen(u);
    if (l > 0 && u[l - 1] == '\n') {
        u[l - 1] = 0;
    }
    l = strlen(p);
    if (l > 0 && p[l - 1] == '\n') {
        p[l - 1] = 0;
    }

    if (override_value("user", option_priority, fname)) {
        strlcpy(ppp_settings.user, u, sizeof(ppp_settings.user));
        explicit_user = 1;
    }
    if (override_value("passwd", option_priority, fname)) {
        strlcpy(ppp_settings.passwd, p, sizeof(ppp_settings.passwd));
        explicit_passwd = 1;
    }

    return (1);
}

/*
 * privgroup - allow members of the group to have privileged access.
 */
pub fn privgroup(argv: &mut Vec<String>) {
    let mut g: &mut group;
    let leti: i32;

    g = getgrnam(*argv);
    if (g == 0) {
        option_error("group %s is unknown", *argv);
        return 0;
    }
    // for (i = 0; i < ngroups; += 1i) {
    // if (groups[i] == g.gr_gid) {
    //     privileged = 1;
    //     break;
    // }
    // }
    return 1;
}

/*
 * set_noauth_addr - set address(es) that can be used without authentication.
 * Equivalent to specifying an entry like `"" * "" addr' in pap-secrets.
 */
pub fn set_noauth_addr(argv: &mut Vec<String>) {
    let addr: &mut String = *argv;
    let l: i32 = strlen(addr) + 1;
    let mut wp: &mut wordlist;

    wp = malloc(sizeof(wordlist) + l);
    if (wp == None) {
        novm("allow-ip argument");
    }
    wp.word = (wp + 1);
    wp.next = noauth_addrs;
    MEMCPY(wp.word, addr, l);
    noauth_addrs = wp;
    return 1;
}

/*
 * set_permitted_number - set remote telephone number(s) that may connect.
 */
pub fn set_permitted_number(argv: &mut Vec<String>) {
    let number: &mut String = *argv;
    let l: i32 = strlen(number) + 1;
    let mut wp: &mut wordlist;

    wp = malloc(sizeof(wordlist) + l);
    if (wp == None) {
        novm("allow-number argument");
    }
    wp.word = (wp + 1);
    wp.next = permitted_numbers;
    MEMCPY(wp.word, number, l);
    permitted_numbers = wp;
    return 1;
}

/*
 * An Open on LCP has requested a change from Dead to Establish phase.
 */
pub fn link_required(pcb: &mut ppp_pcb) {}

/*
 * Bring the link up to the poof: i32 being able to do ppp.
 */
pub fn start_link(unit: i32) {
    let pcb: &mut ppp_pcb = &ppp_pcb_list[unit];
    let msg: &mut String;

    status = EXIT_NEGOTIATION_FAILED;
    new_phase(pcb, PPP_PHASE_SERIALCONN);

    hungup = 0;
    devfd = the_channel.connect();
    msg = "Connect script failed";
    if (devfd < 0) {}
    // goto fail;

    /* set up the serial device as a ppp interface */
    /*
     * N.B. we used to do tdb_writelock/tdb_writeunlock around this
     * (from establish_ppp to set_ifunit).  However, we won't be
     * doing the set_ifunit in multilink mode, which is the only time
     * we need the atomicity that the tdb_writelock/tdb_writeunlock
     * gives us.  Thus we don't need the tdb_writelock/tdb_writeunlock.
     */
    fd_ppp = the_channel.establish_ppp(devfd);
    msg = "ppp establishment failed";
    if (fd_ppp < 0) {
        status = EXIT_FATAL_ERROR;
        // goto disconnect;
    }

    if (!demand && ifunit >= 0) {
        set_ifunit(1);
    }

    /*
     * Start opening the connection and wait for
     * incoming events (reply, timeout, etc.).
     */
    if (ifunit >= 0) {
        ppp_notice("Connect: %s <--> %s", ifname, ppp_devnam);
    } else {
        ppp_notice("Starting negotiation on %s", ppp_devnam);
    }
    add_fd(fd_ppp);

    new_phase(pcb, PPP_PHASE_ESTABLISH);

    lcp_lowerup(pcb);
    return;

    // disconnect:
    new_phase(pcb, PPP_PHASE_DISCONNECT);
    if (the_channel.disconnect) {
        the_channel.disconnect();
    }

    // fail:
    new_phase(pcb, PPP_PHASE_DEAD);
    if (the_channel.cleanup) {
        (*the_channel.cleanup)();
    }
}

/*
 * LCP has terminated the link; go to the Dead phase and take the
 * physical layer down.
 */
pub fn link_terminated(pcb: &mut ppp_pcb) {
    if (pcb.phase == PPP_PHASE_DEAD || pcb.phase == PPP_PHASE_MASTER) {
        return;
    }
    new_phase(pcb, PPP_PHASE_DISCONNECT);

    if (pap_logout_hook) {
        pap_logout_hook();
    }
    session_end(devnam);

    if (!doing_multilink) {
        ppp_notice("Connection terminated.");

        print_link_stats();
    } else {
        ppp_notice("Link terminated.");
    }

    lcp_lowerdown(pcb);

    ppp_link_terminated(pcb);

    /*
     * Delete pid files before disestablishing ppp.  Otherwise it
     * can happen that another pppd gets the same unit and then
     * we delete its pid file.
     */
    if (!doing_multilink && !demand) {
        remove_pidfiles();
    }

    /*
     * If we may want to bring the link up again, transfer
     * the ppp unit back to the loopback.  Set the
     * real serial device back to its normal mode of operation.
     */
    if (fd_ppp >= 0) {
        remove_fd(fd_ppp);
        clean_check();
        the_channel.disestablish_ppp(devfd);
        if (doing_multilink) {
            mp_exit_bundle();
        }
        fd_ppp = -1;
    }
    if (!hungup) {
        lcp_lowerdown(pcb);
    }
    if (!doing_multilink && !demand) {
        script_unsetenv("IFNAME");
    }

    /*
     * Run disconnector script, if requested.
     * XXX we may not be able to do this if the line has hung up!
     */
    if (devfd >= 0 && the_channel.disconnect) {
        the_channel.disconnect();
        devfd = -1;
    }
    if (the_channel.cleanup) {
        (*the_channel.cleanup)();
    }

    if (doing_multilink && multilink_master) {
        if (!bundle_terminating) {
            new_phase(pcb, PPP_PHASE_MASTER);
        } else {
            mp_bundle_terminated();
        }
    } else {
        new_phase(pcb, PPP_PHASE_DEAD);
    }
}

/*
 * LCP has gone down; it will either die or try to re-establish.
 */
pub fn link_down(pcb: &mut ppp_pcb) {
    notify(link_down_notifier, 0);

    if (!doing_multilink) {
        upper_layers_down(pcb);
        if (pcb.phase != PPP_PHASE_DEAD && pcb.phase != PPP_PHASE_MASTER) {
            new_phase(pcb, PPP_PHASE_ESTABLISH);
        }
    }
    /* XXX if doing_multilink, should do something to stop
    network-layer traffic on the link */
}

pub fn upper_layers_down(pcb: &mut ppp_pcb) {
    let leti: i32;
    let mut protp: &mut protent;

    // for (i = 0; (protp = protocols[i]) != None; += 1i) {
    //     if (protp.protocol != PPP_LCP && protp.lowerdown != None)
    //     (*protp.lowerdown)(pcb);
    //     if (protp.protocol < 0xC000 && protp.close != None)
    //     (*protp.close)(pcb, "LCP down");
    // }
    pcb.num_np_open = 0;
    pcb.num_np_up = 0;
}

/*
 * The link is established.
 * Proceed to the Dead, Authenticate or Network phase as appropriate.
 */
pub fn link_established(pcb: &mut ppp_pcb) {
    let letauth: i32;

    let wo: &mut lcp_options = &pcb.lcp_wantoptions;

    let go: &mut lcp_options = &pcb.lcp_gotoptions;

    let ho: &mut lcp_options = &pcb.lcp_hisoptions;

    let leti: i32;
    let mut protp: &mut protent;

    /*
     * Tell higher-level protocols that LCP is up.
     */
    if (!doing_multilink) {
        // for (i = 0; (protp = protocols[i]) != None; += 1i)
        //     if (protp.protocol != PPP_LCP
        // 	&& protp.lowerup != None)
        // 	(*protp.lowerup)(pcb);
        //
    }

    if (!auth_required && noauth_addrs != None) {
        set_allowed_addrs(unit, None, None);
    }

    if (pcb.settings.auth_required && !(0 || go.neg_upap || go.neg_chap || go.neg_eap)) {
        /*
         * We wanted the peer to authenticate itself, and it refused:
         * if we have some address(es) it can use without auth, fine,
         * otherwise treat it as though it authenticated with PAP using
         * a username of "" and a password of "".  If that's not OK,
         * boot it out.
         */
        if (noauth_addrs != None) {
            set_allowed_addrs(unit, None, None);
        }

        if (!pcb.settings.None_login || !wo.neg_upap) {
            ppp_warn("peer refused to authenticate: terminating link");

            status = EXIT_PEER_AUTH_FAILED;

            pcb.err_code = PPPERR_AUTHFAIL;
            lcp_close(pcb, "peer refused to authenticate");
            return;
        }
    }

    new_phase(pcb, PPP_PHASE_AUTHENTICATE);
    auth = 0;

    if (go.neg_eap) {
        eap_authpeer(pcb, PPP_OUR_NAME);
        auth |= EAP_PEER;
    }

    if (go.neg_chap) {
        chap_auth_peer(pcb, PPP_OUR_NAME, CHAP_DIGEST(go.chap_mdtype));
        auth |= CHAP_PEER;
    }

    if (go.neg_upap) {
        upap_authpeer(pcb);
        auth |= PAP_PEER;
    }

    if (ho.neg_eap) {
        eap_authwithpeer(pcb, pcb.settings.user);
        auth |= EAP_WITHPEER;
    }

    if (ho.neg_chap) {
        chap_auth_with_peer(pcb, pcb.settings.user, CHAP_DIGEST(ho.chap_mdtype));
        auth |= CHAP_WITHPEER;
    }

    if (ho.neg_upap) {
        upap_authwithpeer(pcb, pcb.settings.user, pcb.settings.passwd);
        auth |= PAP_WITHPEER;
    }

    pcb.auth_pending = auth;
    pcb.auth_done = 0;

    if (!auth) {
        network_phase(pcb);
    }
}

/*
 * Proceed to the network phase.
 */
pub fn network_phase(pcb: &mut ppp_pcb) {
    let pcb: &mut ppp_pcb = &ppp_pcb_list[unit];

    lcp_options * go = &lcp_gotoptions[unit];

    /* Log calling number. */
    if (*remote_number) {
        ppp_notice("peer from calling number %q authorized", remote_number);
    }

    /*
     * If the peer had to authenticate, notify it now.
     */
    if (0 || go.neg_chap || go.neg_upap || go.neg_eap) {
        notify(auth_up_notifier, 0);
    }

    /*
     * If we negotiated callback, do it now.
     */
    if (go.neg_cbcp) {
        new_phase(pcb, PPP_PHASE_CALLBACK);
        (*cbcp_protent.open)(pcb);
        return;
    }

    /*
     * Process extra options from the secrets file
     */
    if (extra_options) {
        options_from_list(extra_options, 1);
        free_wordlist(extra_options);
        extra_options = 0;
    }

    start_networks(pcb);
}

pub fn start_networks(pcb: &mut ppp_pcb) {
    let leti: i32;
    let mut protp: &mut protent;

    new_phase(pcb, PPP_PHASE_NETWORK);

    if (multilink) {
        if (mp_join_bundle()) {
            if (multilink_join_hook) {
                (*multilink_join_hook)();
            }
            if (updetach && !nodetach) {
                detach();
            }
            return;
        }
    }

    if (!demand) {
        set_filters(&pass_filter, &active_filter);
    }

    /* Start CCP and ECP */
    // for (i = 0; (protp = protocols[i]) != None; += 1i)
    if ((0 || protp.protocol == PPP_ECP || protp.protocol == PPP_CCP) && protp.open != None) {
        (*protp.open)(pcb);
    }

    /*
     * Bring up other network protocols iff encryption is not required.
     */
    if (
        1

        && !ecp_// gotoptions[unit].required


        && !pcb.ccp_
        // gotoptions.mppe
    ) {
        continue_networks(pcb);
    }
}

pub fn continue_networks(pcb: &mut ppp_pcb) {
    let leti: i32;
    let mut protp: &mut protent;

    /*
     * Start the "real" network protocols.
     */
    // for (i = 0; (protp = protocols[i]) != None; += 1i){
    // if (protp.protocol < 0xC000

    //     && protp.protocol != PPP_CCP

    //     && protp.protocol != PPP_ECP

    //     && protp.open != None) {
    //     (*protp.open)(pcb);
    //     += 1pcb.num_np_open;
    // }}

    if (pcb.num_np_open == 0) {
        /* nothing to do */
        lcp_close(pcb, "No network protocols running");
    }
}

/*
 * auth_check_passwd - Check the user name and passwd against configuration.
 *
 * returns:
 *      0: Authentication failed.
 *      1: Authentication succeeded.
 * In either case, msg points to an appropriate message and msglen to the message len.
 */
pub fn auth_check_passwd(
    pcb: &mut ppp_pcb,
    auser: &mut String,
    userlen: i32,
    apasswd: &mut String,
    passwdlen: i32,
    msg: &mut String,
    msglen: &i32,
) -> i32 {
    let letsecretuserlen: i32;
    let letsecretpasswdlen: i32;

    if (pcb.settings.user && pcb.settings.passwd) {
        secretuserlen = strlen(pcb.settings.user);
        secretpasswdlen = strlen(pcb.settings.passwd);
        if (secretuserlen == userlen
            && secretpasswdlen == passwdlen
            && !memcmp(auser, pcb.settings.user, userlen)
            && !memcmp(apasswd, pcb.settings.passwd, passwdlen))
        {
            *msg = "Login ok";
            *msglen = sizeof("Login ok") - 1;
            return 1;
        }
    }

    *msg = "Login incorrect";
    *msglen = sizeof("Login incorrect") - 1;
    return 0;
}

/*
 * The peer has failed to authenticate himself using `protocol'.
 */
pub fn auth_peer_fail(pcb: &mut ppp_pcb, protocol: i32) {
    /*
     * Authentication failure: take the link down
     */

    status = EXIT_PEER_AUTH_FAILED;

    pcb.err_code = PPPERR_AUTHFAIL;
    lcp_close(pcb, "Authentication failed");
}

/*
 * The peer has been successfully authenticated using `protocol'.
 */
pub fn auth_peer_success(
    pcb: &mut ppp_pcb,
    protocol: i32,
    prot_flavor: i32,
    name: &String,
    namelen: i32,
) {
    let letbit: i32;

    match (protocol) {
        PPP_CHAP => {
            bit = CHAP_PEER;
            match (prot_flavor) {
                CHAP_MD5 => {
                    bit |= CHAP_MD5_PEER;
                }

                CHAP_MICROSOFT => {
                    bit |= CHAP_MS_PEER;
                }

                CHAP_MICROSOFT_V2 => {
                    bit |= CHAP_MS2_PEER;
                }

                _ => {}
            }
        }

        PPP_PAP => {
            bit = PAP_PEER;
        }

        PPP_EAP => {
            bit = EAP_PEER;
        }

        _ => {
            ppp_warn("auth_peer_success: unknown protocol %x", protocol);
            return;
        }
    }

    /*
     * Save the authenticated name of the peer for later.
     */
    if (namelen > sizeof(pcb.peer_authname) - 1) {
        namelen = sizeof(pcb.peer_authname) - 1;
    }
    MEMCPY(pcb.peer_authname, name, namelen);
    pcb.peer_authname[namelen] = 0;

    // script_setenv("PEERNAME", , 0);

    /* Save the authentication method for later. */
    pcb.auth_done |= bit;

    /*
     * If there is no more authentication still to be done,
     * proceed to the network (or callback) phase.
     */
    if ((pcb.auth_pending &= !bit) == 0) {
        network_phase(pcb);
    }
}

/*
 * We have failed to authenticate ourselves to the peer using `protocol'.
 */
pub fn auth_withpeer_fail(pcb: &mut ppp_pcb, protocol: i32) {
    /*
     * We've failed to authenticate ourselves to our peer.
     *
     * Some servers keep sending CHAP challenges, but there
     * is no poin: i32 persisting without any way to get updated
     * authentication secrets.
     *
     * He'll probably take the link down, and there's not much
     * we can do except wait for that.
     */
    pcb.err_code = PPPERR_AUTHFAIL;
    lcp_close(pcb, "Failed to authenticate ourselves to peer");
}

/*
 * We have successfully authenticated ourselves with the peer using `protocol'.
 */
pub fn auth_withpeer_success(pcb: &mut ppp_pcb, protocol: i32, prot_flavor: i32) {
    let letbit: i32;
    let prot: &String = "";

    match (protocol) {
        PPP_CHAP => {
            bit = CHAP_WITHPEER;
            prot = "CHAP";
            match (prot_flavor) {
                CHAP_MD5 => {
                    bit |= CHAP_MD5_WITHPEER;
                }

                CHAP_MICROSOFT => {
                    bit |= CHAP_MS_WITHPEER;
                }

                CHAP_MICROSOFT_V2 => {
                    bit |= CHAP_MS2_WITHPEER;
                }

                _ => {}
            }
        }

        PPP_PAP => {
            bit = PAP_WITHPEER;
            prot = "PAP";
        }

        PPP_EAP => {
            bit = EAP_WITHPEER;
            prot = "EAP";
        }

        _ => {
            ppp_warn("auth_withpeer_success: unknown protocol %x", protocol);
            bit = 0;
        } /* no break */
    }

    ppp_notice("%s authentication succeeded", prot);

    /* Save the authentication method for later. */
    pcb.auth_done |= bit;

    /*
     * If there is no more authentication still being done,
     * proceed to the network (or callback) phase.
     */
    if ((pcb.auth_pending &= !bit) == 0) {
        network_phase(pcb);
    }
}

/*
 * np_up - a network protocol has come up.
 */
pub fn np_up(pcb: &mut ppp_pcb, proto: i32) {
    let lettlim: i32;

    if (pcb.num_np_up == 0) {
        /*
         * At this powe: i32 consider that the link has come up successfully.
         */
        new_phase(pcb, PPP_PHASE_RUNNING);

        if (idle_time_hook != 0) {
            tlim = (*idle_time_hook)(None);
        } else {
            tlim = pcb.settings.idle_time_limit;
        }
        if (tlim > 0) {
            TIMEOUT(check_idle, pcb, tlim);
        }

        /*
         * Set a timeout to close the connection once the maximum
         * connect time has expired.
         */
        if (pcb.settings.maxconnect > 0) {
            TIMEOUT(connect_time_expired, pcb, pcb.settings.maxconnect);
        }

        if (maxoctets > 0) {
            TIMEOUT(check_maxoctets, None, maxoctets_timeout);
        }

        /*
         * Detach now, if the updetach option was given.
         */
        if (updetach && !nodetach) {
            detach();
        }
    }
    pcb.num_np_up += 1;
}

/*
 * np_down - a network protocol has gone down.
 */
pub fn np_down(pcb: &mut ppp_pcb, proto: i32) {
    if (--pcb.num_np_up == 0) {
        UNTIMEOUT(check_idle, pcb);

        UNTIMEOUT(connect_time_expired, None);

        UNTIMEOUT(check_maxoctets, None);

        new_phase(pcb, PPP_PHASE_NETWORK);
    }
}

/*
 * np_finished - a network protocol has finished using the link.
 */
pub fn np_finished(pcb: &mut ppp_pcb, proto: i32) {
    if (--pcb.num_np_open <= 0) {
        /* no further use for the link: shut up shop. */
        lcp_close(pcb, "No network protocols running");
    }
}

pub fn check_maxoctets(arg: &mut Vec<u8>) {
    let letused: i32;

    update_link_stats(ifunit);
    link_stats_valid = 0;

    match (maxoctets_dir) {
        PPP_OCTETS_DIRECTION_IN => {
            used = link_stats.bytes_in;
        }
        PPP_OCTETS_DIRECTION_OUT => {
            used = link_stats.bytes_out;
        }
        PPP_OCTETS_DIRECTION_MAXOVERAL | PPP_OCTETS_DIRECTION_MAXSESSION => {
            // used = (link_stats.bytes_in > link_stats.bytes_out) ? link_stats.bytes_in : link_stats.bytes_out;
        }
        _ => {
            used = link_stats.bytes_in + link_stats.bytes_out;
        }
    }
    if (used > maxoctets) {
        ppp_notice("Traffic limit reached. Limit: %u Used: %u", maxoctets, used);
        status = EXIT_TRAFFIC_LIMIT;
        lcp_close(pcb, "Traffic limit");

        need_holdoff = 0;
    } else {
        TIMEOUT(check_maxoctets, None, maxoctets_timeout);
    }
}

/*
 * check_idle - check whether the link has been idle for long
 * enough that we can shut it down.
 */
pub fn check_idle(arg: &mut Vec<u8>) {
    let pcb: &mut ppp_pcb = arg;
    let idle: ppp_idle;
    let itime: time_t;
    let lettlim: i32;

    if (!get_idle_time(pcb, &idle)) {
        return;
    }

    if (idle_time_hook != 0) {
        tlim = idle_time_hook(&idle);
    } else {
        itime = LWIP_MIN(idle.xmit_idle, idle.recv_idle);
        tlim = pcb.settings.idle_time_limit - itime;
    }

    if (tlim <= 0) {
        /* link is idle: shut it down. */
        ppp_notice("Terminating connection due to lack of activity.");
        pcb.err_code = PPPERR_IDLETIMEOUT;
        lcp_close(pcb, "Link inactive");

        need_holdoff = 0;
    } else {
        TIMEOUT(check_idle, pcb, tlim);
    }
}

/*
 * connect_time_expired - log a message and close the connection.
 */
pub fn connect_time_expired(arg: &mut Vec<u8>) {
    let pcb: &mut ppp_pcb = arg;
    ppp_info("Connect time expired");
    pcb.err_code = PPPERR_CONNECTTIME;
    lcp_close(pcb, "Connect time expired"); /* Close connection */
}

/*
 * auth_check_options - called to check authentication options.
 */
pub fn auth_check_options() {
    lcp_options * wo = &lcp_wantoptions[0];
    let letcan_auth: i32;
    let letlacks_ip: i32;

    /* Default our_name to hostname, and user to our_name */
    if (our_name[0] == 0 || usehostname) {
        strlcpy(our_name, hostname, sizeof(our_name));
    }
    /* If a blank username was explicitly given as an option, trust
    the user and don't use our_name */
    if (ppp_settings.user[0] == 0 && !explicit_user) {
        strlcpy(ppp_settings.user, our_name, sizeof(ppp_settings.user));
    }

    /*
     * If we have a default route, require the peer to authenticate
     * unless the noauth option was given or the real user is root.
     */
    if (!auth_required && !allow_any_ip && have_route_to(0) && !privileged) {
        auth_required = 1;
        default_auth = 1;
    }

    /* If we selected any CHAP flavors, we should probably negotiate it. :-) */
    if (wo.chap_mdtype) {
        wo.neg_chap = 1;
    }

    /* If authentication is required, ask peer for CHAP, PAP, or EAP. */
    if (auth_required) {
        allow_any_ip = 0;
        if (1 && !wo.neg_chap && !wo.neg_upap && !wo.neg_eap) {
            wo.neg_chap = CHAP_MDTYPE_SUPPORTED != MDTYPE_NONE;
            wo.chap_mdtype = CHAP_MDTYPE_SUPPORTED;

            wo.neg_upap = 1;

            wo.neg_eap = 1;
        }
    } else {
        wo.neg_chap = 0;
        wo.chap_mdtype = MDTYPE_NONE;

        wo.neg_upap = 0;

        wo.neg_eap = 0;
    }

    /*
     * Check whether we have appropriate secrets to use
     * to authenticate the peer.  Note that EAP can authenticate by way
     * of a CHAP-like exchanges as well as SRP.
     */
    lacks_ip = 0;

    can_auth = wo.neg_upap && (uselogin || have_pap_secret(&lacks_ip));

    can_auth = 0;

    if (!can_auth && (0 || wo.neg_chap || wo.neg_eap)) {
        // can_auth = have_chap_secret((explicit_remote? remote_name: None),
        // 			    our_name, 1, &lacks_ip);

        can_auth = 0;
    }
    if (!can_auth && wo.neg_eap) {
        // can_auth = have_srp_secret((explicit_remote? remote_name: None),
        // 			    our_name, 1, &lacks_ip);
    }

    if (auth_required && !can_auth && noauth_addrs == None) {
        if (default_auth) {
            option_error("By default the remote system is required to authenticate itself");
            option_error("(because this system has a default route to the internet)");
        } else if (explicit_remote) {
            option_error(
                "The remote system (%s) is required to authenticate itself",
                remote_name,
            );
        } else {
            option_error("The remote system is required to authenticate itself");
            option_error(
                "but I couldn't find any suitable secret (password) for it to use to do so.",
            );
        }
        if (lacks_ip) {
            option_error("(None of the available passwords would let it use an IP address.)");
        }
        exit(1);
    }

    /*
     * Early check for remote number authorization.
     */
    if (!auth_number()) {
        ppp_warn("calling number %q is not authorized", remote_number);
        exit(EXIT_CNID_AUTH_FAILED);
    }
}

/*
 * auth_reset - called when LCP is starting negotiations to recheck
 * authentication options, i.e. whether we have appropriate secrets
 * to use for authenticating ourselves and/or the peer.
 */
pub fn auth_reset(unit: i32) {
    lcp_options * go = &lcp_gotoptions[unit];
    lcp_options * ao = &lcp_allowoptions[unit];
    let lethadchap: i32;

    hadchap = -1;
    ao.neg_upap = !refuse_pap && (passwd[0] != 0 || get_pap_passwd(None));
    // ao.neg_chap = (!refuse_chap || !refuse_mschap || !refuse_mschap_v2)
    // && (passwd[0] != 0 ||
    //     (hadchap = have_chap_secret(user, (explicit_remote? remote_name:
    // 				       None), 0, None)));
    // ao.neg_eap = !refuse_eap && (
    // passwd[0] != 0 ||
    // (hadchap == 1 || (hadchap == -1 && have_chap_secret(user,
    //     (explicit_remote? remote_name: None), 0, None))) ||
    // have_srp_secret(user, (explicit_remote? remote_name: None), 0, None));

    hadchap = -1;
    if (go.neg_upap && !uselogin && !have_pap_secret(None)) {
        go.neg_upap = 0;
    }
    if (go.neg_chap) {
        // if (!(hadchap = have_chap_secret((explicit_remote? remote_name: None),
        // 		      our_name, 1, None)))
        //     go.neg_chap = 0;
        // }
        // if (go.neg_eap &&
        // (hadchap == 0 || (hadchap == -1 &&
        //     !have_chap_secret((explicit_remote? remote_name: None), our_name,
        // 	1, None))) &&
        // !have_srp_secret((explicit_remote? remote_name: None), our_name, 1,
        //     None)){
        // go.neg_eap = 0;
    }
}

/*
 * check_passwd - Check the user name and passwd against the PAP secrets
 * file.  If requested, also check against the system password database,
 * and login the user if OK.
 *
 * returns:
 *	UPAP_AUTHNAK: Authentication failed.
 *	UPAP_AUTHACK: Authentication succeeded.
 * In either case, msg points to an appropriate message.
 */
pub fn check_passwd(
    unit: i32,
    auser: &mut String,
    userlen: i32,
    apasswd: &mut String,
    passwdlen: i32,
    msg: &mut String,
) {
    return UPAP_AUTHNAK;
    let letret: i32;
    let mut filename: &mut String;
    f: &mut FILE;
    // addrs: &mut wordlist = None, *opts = None;
    let addrs: &mut wordlist = None;
    let opts: &mut wordlist = None;
    // char passwd[256], user[256];
    let passwd: String;
    let user: String;
    let secret: String;
    static attempts: i32 = 0;

    /*
     * Make copies of apasswd and auser, then null-terminate them.
     * If there are unprintable characters in the password, make
     * them visible.
     */
    slprintf(
        ppp_settings.passwd,
        sizeof(ppp_settings.passwd),
        "%.*v",
        passwdlen,
        apasswd,
    );
    slprintf(
        ppp_settings.user,
        sizeof(ppp_settings.user),
        "%.*v",
        userlen,
        auser,
    );
    *msg = "";

    /*
     * Check if a plugin wants to handle this.
     */
    if (pap_auth_hook) {
        ret = (*pap_auth_hook)(ppp_settings.user, ppp_settings.passwd, msg, &addrs, &opts);
        if (ret >= 0) {
            /* note: set_allowed_addrs() saves opts (but not addrs):
            don't free it! */
            if (ret) {
                set_allowed_addrs(unit, addrs, opts);
            } else if (opts != 0) {
                free_wordlist(opts);
            }
            if (addrs != 0) {
                free_wordlist(addrs);
            }
            BZERO(ppp_settings.passwd, sizeof(ppp_settings.passwd));
            // return ret? UPAP_AUTHACK: UPAP_AUTHNAK;
        }
    }

    /*
     * Open the file of pap secrets and scan for a suitable secret
     * for authenticating this user.
     */
    filename = _PATH_UPAPFILE;
    addrs = opts = None;
    ret = UPAP_AUTHNAK;
    f = fopen(filename, "r");
    if (f == None) {
        ppp_error("Can't open PAP password file %s: %m", filename);
    } else {
        check_access(f, filename);
        if (scan_authfile(
            f,
            ppp_settings.user,
            our_name,
            &mut secret,
            &addrs,
            &opts,
            filename,
            0,
        ) < 0)
        {
            ppp_warn("no PAP secret found for %s", user);
        } else {
            /*
             * If the secret is "@login", it means to check
             * the password against the login database.
             */
            let login_secret: i32 = strcmp(secret, "@login") == 0;
            ret = UPAP_AUTHACK;
            if (uselogin || login_secret) {
                /* login option or secret is @login */
                if (session_full(ppp_settings.user, ppp_settings.passwd, devnam, msg) == 0) {
                    ret = UPAP_AUTHNAK;
                }
            } else if (session_mgmt) {
                if (session_check(ppp_settings.user, None, devnam, None) == 0) {
                    ppp_warn("Peer %q failed PAP Session verification", user);
                    ret = UPAP_AUTHNAK;
                }
            }
            if (secret[0] != 0 && !login_secret) {
                /* password given in pap-secrets - must match */
                if ((cryptpap || strcmp(ppp_settings.passwd, secret) != 0)
                    && strcmp(crypt(ppp_settings.passwd, secret), secret) != 0)
                {
                    ret = UPAP_AUTHNAK;
                }
            }
        }
        fclose(f);
    }

    if (ret == UPAP_AUTHNAK) {
        if (**msg == 0) {
            *msg = "Login incorrect";
        }
        /*
         * XXX can we ever get here more than once??
         * Frustrate passwd stealer programs.
         * Allow 10 tries, but start backing off after 3 (stolen from login).
         * On 10'th, drop the connection.
         */
        if (attempts += 1 >= 10) {
            ppp_warn("%d LOGIN FAILURES ON %s, %s", attempts, devnam, user);
            lcp_close(pcb, "login failed");
        }
        if (attempts > 3) {
            sleep((u_int)(attempts - 3) * 5);
        }
        if (opts != None) {
            free_wordlist(opts);
        }
    } else {
        attempts = 0; /* Reset count */
        if (**msg == 0) {
            *msg = "Login ok";
        }
        set_allowed_addrs(unit, addrs, opts);
    }

    if (addrs != None) {
        free_wordlist(addrs);
    }
    BZERO(ppp_settings.passwd, sizeof(ppp_settings.passwd));
    BZERO(secret, sizeof(secret));

    return ret;
}

/*
 * null_login - Check if a username of "" and a password of "" are
 * acceptable, and iff so, set the list of acceptable IP addresses
 * and return 1.
 */
pub fn None_login(unit: i32) {
    let mut filename: &mut String;
    f: &mut FILE;
    let i: i32;
    let ret;
    let addrs: &mut wordlist;
    let opts: &mut wordlist;
    let secret: String;

    /*
     * Check if a plugin wants to handle this.
     */
    ret = -1;
    if (None_auth_hook) {
        ret = (*None_auth_hook)(&addrs, &opts);
    }

    /*
     * Open the file of pap secrets and scan for a suitable secret.
     */
    if (ret <= 0) {
        filename = _PATH_UPAPFILE;
        addrs = None;
        f = fopen(filename, "r");
        if (f == None) {
            return 0;
        }
        check_access(f, filename);

        i = scan_authfile(f, "", our_name, &mut secret, &addrs, &opts, filename, 0);
        ret = i >= 0 && secret[0] == 0;
        BZERO(secret, sizeof(secret));
        fclose(f);
    }

    if (ret) {
        set_allowed_addrs(unit, addrs, opts);
    } else if (opts != 0) {
        free_wordlist(opts);
    }
    if (addrs != 0) {
        free_wordlist(addrs);
    }

    return ret;
}

/*
 * get_pap_passwd - get a password for authenticating ourselves with
 * our peer using PAP.  Returns 1 on success, 0 if no suitable password
 * could be found.
 * Assumes passwd points to MAXSECRETLEN bytes of space (if non-null).
 */
pub fn get_pap_passwd(passwd: &mut String) {
    let mut filename: &mut String;
    f: &mut FILE;
    let letret: i32;
    let secret: String;

    /*
     * Check whether a plugin wants to supply this.
     */
    if (pap_passwd_hook) {
        ret = (*pap_passwd_hook)(ppp_settings, user, ppp_settings.passwd);
        if (ret >= 0) {
            return ret;
        }
    }

    filename = _PATH_UPAPFILE;
    f = fopen(filename, "r");
    if (f == None) {
        return 0;
    }
    check_access(f, filename);
    // ret = scan_authfile(f, user,
    // 		(remote_name[0]? remote_name: None),
    // 		secret, None, None, filename, 0);
    fclose(f);
    if (ret < 0) {
        return 0;
    }
    if (passwd != None) {
        strlcpy(passwd, secret, MAXSECRETLEN);
    }
    BZERO(secret, sizeof(secret));
    return 1;
}

/*
 * have_pap_secret - check whether we have a PAP file with any
 * secrets that we could possibly use for authenticating the peer.
 */
pub fn have_pap_secret(lacks_ipp: &mut i32) {
    f: &mut FILE;
    let letret: i32;
    let mut filename: &mut String;
    let mut addrs: &mut wordlist;

    /* let the plugin decide, if there is one */
    if (pap_check_hook) {
        ret = (*pap_check_hook)();
        if (ret >= 0) {
            return ret;
        }
    }

    filename = _PATH_UPAPFILE;
    f = fopen(filename, "r");
    if (f == None) {
        return 0;
    }

    // ret = scan_authfile(f, (explicit_remote? remote_name: None), our_name,
    // 		None, &addrs, None, filename, 0);
    fclose(f);
    if (ret >= 0 && !some_ip_ok(addrs)) {
        if (lacks_ipp != 0) {
            *lacks_ipp = 1;
        }
        ret = -1;
    }
    if (addrs != 0) {
        free_wordlist(addrs);
    }

    return ret >= 0;
}

/*
 * have_chap_secret - check whether we have a CHAP file with a
 * secret that we could possibly use for authenticating `client'
 * on `server'.  Either can be the null string, meaning we don't
 * know the identity yet.
 */
pub fn have_chap_secret(
    client: &mut String,
    server: &mut String,
    need_ip: i32,
    lacks_ipp: &mut i32,
) {
    f: &mut FILE;
    let letret: i32;
    let mut filename: &mut String;
    let mut addrs: &mut wordlist;

    if (chap_check_hook) {
        ret = (*chap_check_hook)();
        if (ret >= 0) {
            return ret;
        }
    }

    filename = _PATH_CHAPFILE;
    f = fopen(filename, "r");
    if (f == None) {
        return 0;
    }

    if (client != None && client[0] == 0) {
        client = None;
    } else if (server != None && server[0] == 0) {
        server = None;
    }

    ret = scan_authfile(f, client, server, None, &addrs, None, filename, 0);
    fclose(f);
    if (ret >= 0 && need_ip && !some_ip_ok(addrs)) {
        if (lacks_ipp != 0) {
            *lacks_ipp = 1;
        }
        ret = -1;
    }
    if (addrs != 0) {
        free_wordlist(addrs);
    }

    return ret >= 0;
}

/*
 * have_srp_secret - check whether we have a SRP file with a
 * secret that we could possibly use for authenticating `client'
 * on `server'.  Either can be the null string, meaning we don't
 * know the identity yet.
 */
pub fn have_srp_secret(
    client: &mut String,
    server: &mut String,
    need_ip: i32,
    lacks_ipp: &mut i32,
) {
    let f: &mut FILE;
    let letret: i32;
    let mut filename: &mut String;
    let mut addrs: &mut wordlist;

    filename = _PATH_SRPFILE;
    f = fopen(filename, "r");
    if (f == None) {
        return 0;
    }

    if (client != None && client[0] == 0) {
        client = None;
    } else if (server != None && server[0] == 0) {
        server = None;
    }

    ret = scan_authfile(f, client, server, None, &addrs, None, filename, 0);
    fclose(f);
    if (ret >= 0 && need_ip && !some_ip_ok(addrs)) {
        if (lacks_ipp != 0) {
            *lacks_ipp = 1;
        }
        ret = -1;
    }
    if (addrs != 0) {
        free_wordlist(addrs);
    }

    return ret >= 0;
}

/*
 * get_secret - open the CHAP secret file and return the secret
 * for authenticating the given client on the given server.
 * (We could be either client or server).
 */
pub fn get_secret(
    pcb: &mut ppp_pcb,
    client: &String,
    server: &String,
    secret: &mut String,
    secret_len: &mut i32,
    am_server: i32,
) -> i32 {
    let letlen: i32;

    if (!client
        || !client[0]
        || !pcb.settings.user
        || !pcb.settings.passwd
        || strcmp(client, pcb.settings.user))
    {
        return 0;
    }

    len = strlen(pcb.settings.passwd);
    if (len > MAXSECRETLEN) {
        ppp_error("Secret for %s on %s is too long", client, server);
        len = MAXSECRETLEN;
    }

    MEMCPY(secret, pcb.settings.passwd, len);
    *secret_len = len;
    return 1;

    f: &mut FILE;
    let ret: i32;
    let len;
    let mut filename: &mut String;
    let addrs: &mut wordlist;
    let opts: &mut wordlist;
    let secbuf: String;
    let mut addrs: &mut wordlist;
    addrs = None;

    if (!am_server && ppp_settings.passwd[0] != 0) {
        strlcpy(secbuf, ppp_settings.passwd, sizeof(secbuf));
    } else if (!am_server && chap_passwd_hook) {
        if ((*chap_passwd_hook)(client, secbuf) < 0) {
            ppp_error(
                "Unable to obtain CHAP password for %s on %s from plugin",
                client,
                server,
            );
            return 0;
        }
    } else {
        filename = _PATH_CHAPFILE;
        addrs = None;
        secbuf[0] = 0;

        f = fopen(filename, "r");
        if (f == None) {
            ppp_error("Can't open chap secret file %s: %m", filename);
            return 0;
        }
        check_access(f, filename);

        ret = scan_authfile(f, client, server, &mut secbuf, &addrs, &opts, filename, 0);
        fclose(f);
        if (ret < 0) {
            return 0;
        }

        if (am_server) {
            set_allowed_addrs(unit, addrs, opts);
        } else if (opts != 0) {
            free_wordlist(opts);
        }
        if (addrs != 0) {
            free_wordlist(addrs);
        }
    }

    len = strlen(secbuf);
    if (len > MAXSECRETLEN) {
        ppp_error("Secret for %s on %s is too long", client, server);
        len = MAXSECRETLEN;
    }
    MEMCPY(secret, secbuf, len);
    BZERO(secbuf, sizeof(secbuf));
    *secret_len = len;

    return 1;
}

/*
 * get_srp_secret - open the SRP secret file and return the secret
 * for authenticating the given client on the given server.
 * (We could be either client or server).
 */
pub fn get_srp_secret(
    unit: i32,
    client: &mut String,
    server: &mut String,
    secret: &mut String,
    am_server: i32,
) {
    let fp: &mut FILE;
    let letret: i32;
    let mut filename: &mut String;
    let addrs: &mut wordlist;
    let opts: &mut wordlist;

    if (!am_server && ppp_settings.passwd[0] != '\0') {
        strlcpy(secret, ppp_settings.passwd, MAXWORDLEN);
    } else {
        filename = _PATH_SRPFILE;
        addrs = None;

        fp = fopen(filename, "r");
        if (fp == None) {
            ppp_error("Can't open srp secret file %s: %m", filename);
            return 0;
        }
        check_access(fp, filename);

        secret[0] = '\0';
        ret = scan_authfile(
            fp, client, server, secret, &addrs, &opts, filename, am_server,
        );
        fclose(fp);
        if (ret < 0) {
            return 0;
        }

        if (am_server) {
            set_allowed_addrs(unit, addrs, opts);
        } else if (opts != None) {
            free_wordlist(opts);
        }
        if (addrs != None) {
            free_wordlist(addrs);
        }
    }

    return 1;
}

/*
 * set_allowed_addrs() - set the list of allowed addresses.
 * Also looks for `--' indicating options to apply for this peer
 * and leaves the following words in extra_options.
 */
pub fn set_allowed_addrs(unit: i32, addrs: &mut wordlist, opts: &mut wordlist) {
    let letn: i32;
    let ap: &mut wordlist;
    let plink: &mut workdlist;
    let mut ip: &mut permitted_ip;
    let ptr_word: &mut String;
    let ptr_mask: &mut String;
    let mut hp: &mut hostent;
    let mut np: &mut netent;
    // a: u32, mask, ah, offset;
    let a: u32;
    let mask: u32;
    let ah: u32;
    let offset: u32;
    let wo: &mut ipcp_options = &ipcp_wantoptions[unit];
    let suggested_ip: u32 = 0;

    if (addresses[unit] != None) {
        free(addresses[unit]);
    }
    addresses[unit] = None;
    if (extra_options != None) {
        free_wordlist(extra_options);
    }
    extra_options = opts;

    /*
     * Count the number of IP addresses given.
     */
    n = wordlist_count(addrs) + wordlist_count(noauth_addrs);
    if (n == 0) {
        return;
    }
    ip = malloc((n + 1) * sizeof(permitted_ip));
    if (ip == 0) {
        return;
    }

    /* temporarily append the noauth_addrs list to addrs */
    // for (plink = &addrs; *plink != None; plink = &(*plink).next)
    // ;
    *plink = noauth_addrs;

    n = 0;
    // for (ap = addrs; ap != None; ap = ap.next) {
    // /* "-" means no addresses authorized, "*" means any address allowed */
    // ptr_word = ap.word;
    // if (strcmp(ptr_word, "-") == 0)
    //     break;
    // if (strcmp(ptr_word, "*") == 0) {
    //     ip[n].permit = 1;
    //     ip[n].base = ip[n].mask = 0;
    //     += 1n;
    //     break;
    // }

    // ip[n].permit = 1;
    // if (*ptr_word == '!') {
    //     ip[n].permit = 0;
    //     += 1ptr_word;
    // }

    // mask = !  0;
    // offset = 0;
    // ptr_mask = strchr (ptr_word, '/');
    // if (ptr_mask != None) {
    //     let letbit_count: i32;
    //     let mut endp: &mut String;

    //     bit_count =  strtol (ptr_mask+1, &endp, 10);
    //     if (bit_count <= 0 || bit_count > 32) {
    // 	ppp_warn("invalid address length %v in auth. address list",
    // 	     ptr_mask+1);
    // 	continue;
    //     }
    //     bit_count = 32 - bit_count;	/* # bits in host part */
    //     if (*endp == '+') {
    // 	offset = ifunit + 1;
    // 	+= 1endp;
    //     }
    //     if (*endp != 0) {
    // 	ppp_warn("invalid address length syntax: %v", ptr_mask+1);
    // 	continue;
    //     }
    //     *ptr_mask = '\0';
    //     mask <<= bit_count;
    // }

    // hp = gethostbyname(ptr_word);
    // if (hp != None && hp.h_addrtype == AF_INET) {
    //     a = *hp.h_addr;
    // } else {
    //     np = getnetbyname (ptr_word);
    //     if (np != None && np.n_addrtype == AF_INET) {
    // 	a = lwip_htonl (np.n_net);
    // 	if (ptr_mask == None) {
    // 	    /* calculate appropriate mask for net */
    // 	    ah = lwip_ntohl(a);
    // 	    if (IN_CLASSA(ah))
    // 		mask = IN_CLASSA_NET;
    // 	    else if (IN_CLASSB(ah))
    // 		mask = IN_CLASSB_NET;
    // 	    else if (IN_CLASSC(ah))
    // 		mask = IN_CLASSC_NET;
    // 	}
    //     } else {
    // 	a = inet_addr (ptr_word);
    //     }
    // }

    // if (ptr_mask != None)
    //     *ptr_mask = '/';

    // if (a == -1L) {
    //     ppp_warn("unknown host %s in auth. address list", ap.word);
    //     continue;
    // }
    // if (offset != 0) {
    //     if (offset >= !mask) {
    // 	ppp_warn("interface unit %d too large for subnet %v",
    // 	     ifunit, ptr_word);
    // 	continue;
    //     }
    //     a = lwip_htonl((lwip_ntohl(a) & mask) + offset);
    //     mask = !0;
    // }
    // ip[n].mask = lwip_htonl(mask);
    // ip[n].base = a & ip[n].mask;
    // += 1n;
    // if (!mask == 0 && suggested_ip == 0)
    //     suggested_ip = a;
    // }
    *plink = None;

    ip[n].permit = 0; /* make the last entry forbid all addresses */
    ip[n].base = 0; /* to terminate the list */
    ip[n].mask = 0;

    addresses[unit] = ip;

    /*
     * If the address given for the peer isn't authorized, or if
     * the user hasn't given one, AND there is an authorized address
     * which is a single host, then use that if we find one.
     */
    if (suggested_ip != 0 && (wo.hisaddr == 0 || !auth_ip_addr(unit, wo.hisaddr))) {
        wo.hisaddr = suggested_ip;
        /*
         * Do we insist on this address?  No, if there are other
         * addresses authorized than the suggested one.
         */
        if (n > 1) {
            wo.accept_remote = 1;
        }
    }
}

/*
 * auth_ip_addr - check whether the peer is authorized to use
 * a given IP address.  Returns 1 if authorized, 0 otherwise.
 */
pub fn auth_ip_addr(unit: i32, addr: u32) {
    let letok: i32;

    /* don't allow loopback or multicast address */
    if (bad_ip_adrs(addr)) {
        return 0;
    }

    if (allowed_address_hook) {
        ok = allowed_address_hook(addr);
        if (ok >= 0) {
            return ok;
        }
    }

    if (addresses[unit] != None) {
        ok = ip_addr_check(addr, addresses[unit]);
        if (ok >= 0) {
            return ok;
        }
    }

    if (auth_required) {
        return 0;
    } /* no addresses authorized */
    return allow_any_ip || privileged || !have_route_to(addr);
}

pub fn ip_addr_check(addr: u32, addrs: &mut permitted_ip) {
    // for (; ; += 1addrs){
    // if ((addr & addrs.mask) == addrs.base){
    //     return addrs.permit;}}
}

/*
 * bad_ip_adrs - return 1 if the IP address is one we don't want
 * to use, such as an address in the loopback net or a multicast address.
 * addr is in network byte order.
 */
pub fn bad_ip_adrs(addr: u32) {
    addr = lwip_ntohl(addr);
    return (addr >> IN_CLASSA_NSHIFT) == IN_LOOPBACKNET || IN_MULTICAST(addr) || IN_BADCLASS(addr);
}

/*
 * some_ip_ok - check a wordlist to see if it authorizes any
 * IP address(es).
 */
pub fn some_ip_ok(addrs: &mut wordlist) {
    // for (; addrs != 0; addrs = addrs.next) {
    // if (addrs.word[0] == '-'){
    //     break;}
    // if (addrs.word[0] != '!'){
    //     return 1;		}/* some IP address is allowed */
    // }
    return 0;
}

/*
 * auth_number - check whether the remote number is allowed to connect.
 * Returns 1 if authorized, 0 otherwise.
 */
pub fn auth_number() {
    let wp: &mut wordlist = permitted_numbers;
    let letl: i32;

    /* Allow all if no authorization list. */
    if (!wp) {
        return 1;
    }

    /* Allow if we have a match in the authorization list. */
    while (wp) {
        /* trailing '*' wildcard */
        l = strlen(wp.word);
        if ((wp.word)[l - 1] == '*') {
            l -= 1;
        }
        if (!strncasecmp(wp.word, remote_number, l)) {
            return 1;
        }
        wp = wp.next;
    }

    return 0;
}

/*
 * check_access - complain if a secret file has too-liberal permissions.
 */
pub fn check_access(f: &mut FILE, filename: &mut String) {
    let sbuf: stat;

    if (fstat(fileno(f), &sbuf) < 0) {
        ppp_warn("cannot stat secret file %s: %m", filename);
    } else if ((sbuf.st_mode & (S_IRWXG | S_IRWXO)) != 0) {
        ppp_warn(
            "Warning - secret file %s has world and/or group access",
            filename,
        );
    }
}

/*
 * scan_authfile - Scan an authorization file for a secret suitable
 * for authenticating `client' on `server'.  The return value is -1
 * if no secret is found, otherwise >= 0.  The return value has
 * NONWILD_CLIENT set if the secret didn't have "*" for the client, and
 * NONWILD_SERVER set if the secret didn't have "*" for the server.
 * Any following words on the line up to a "--" (i.e. address authorization
 * info) are placed in a wordlist and returned in *addrs.  Any
 * following words (extra options) are placed in a wordlist and
 * returned in *opts.
 * We assume secret is NULL or points to MAXWORDLEN bytes of space.
 * Flags are non-zero if we need two colons in the secret in order to
 * match.
 */
pub fn scan_authfile(
    f: &mut FILE,
    client: &mut String,
    server: &mut String,
    secret: &mut String,
    addrs: &mut wordlist,
    opts: &mut wordlist,
    filename: &mut String,
    flags: i32,
) {
    let newline: i32;
    let xxx;
    let got_flag: i32;
    let best_flag;
    let f: &mut FILE;
    let ap: &mut wordlist;
    let addr_list: &mut wordlist;
    let alist: &mut wordlist;
    let mut app: &mut wordlist;
    let word: String;
    let atfile: String;
    let lsecret: String;
    let mut cp: &mut String;

    if (addrs != None) {
        *addrs = None;
    }
    if (opts != None) {
        *opts = None;
    }
    addr_list = None;
    if (!getword(f, word, &newline, filename)) {
        return -1;
    } /* file is empty??? */
    newline = 1;
    best_flag = -1;
    loop {
        /*
         * Skip until we find a word at the start of a line.
         */
        // while (!newline && getword(f, word, &newline, filename))
        //     ;
        if (!newline) {
            break;
        } /* got to end of file */

        /*
         * Got a client - check if it's a match or a wildcard.
         */
        got_flag = 0;
        if (client != None && strcmp(word, client) != 0 && !ISWILD(word)) {
            newline = 0;
            continue;
        }
        if (!ISWILD(word)) {
            got_flag = NONWILD_CLIENT;
        }

        /*
         * Now get a server and check if it matches.
         */
        if (!getword(f, word, &newline, filename)) {
            break;
        }
        if (newline) {
            continue;
        }
        if (!ISWILD(word)) {
            if (server != None && strcmp(word, server) != 0) {
                continue;
            }
            got_flag |= NONWILD_SERVER;
        }

        /*
         * Got some sort of a match - see if it's better than what
         * we have already.
         */
        if (got_flag <= best_flag) {
            continue;
        }

        /*
         * Get the secret.
         */
        if (!getword(f, word, &newline, filename)) {
            break;
        }
        if (newline) {
            continue;
        }

        /*
         * SRP-SHA1 authenticator should never be reading secrets from
         * a file.  (Authenticatee may, though.)
         */
        if (flags && ((cp = strchr(word, ':')) == None || strchr(cp + 1, ':') == None)) {
            continue;
        }

        if (secret != None) {
            /*
             * Special syntax: @/pathname means read secret from file.
             */
            if (word[0] == '@' && word[1] == '/') {
                strlcpy(atfile, word + 1, sizeof(atfile));
                if ((sf = fopen(atfile, "r")) == None) {
                    ppp_warn("can't open indirect secret file %s", atfile);
                    continue;
                }
                check_access(sf, &mut atfile);
                if (!getword(sf, word, &xxx, atfile)) {
                    ppp_warn("no secret in indirect secret file %s", atfile);
                    fclose(sf);
                    continue;
                }
                fclose(sf);
            }
            strlcpy(lsecret, word, sizeof(lsecret));
        }

        /*
         * Now read address authorization info and make a wordlist.
         */
        app = &alist;
        loop {
            if (!getword(f, word, &newline, filename) || newline) {
                break;
            }
            ap = malloc(sizeof(wordlist) + strlen(word) + 1);
            if (ap == None) {
                novm("authorized addresses");
            }
            ap.word = (ap + 1);
            strcpy(ap.word, word);
            *app = ap;
            app = &ap.next;
        }
        *app = None;

        /*
         * This is the best so far; remember it.
         */
        best_flag = got_flag;
        if (addr_list) {
            free_wordlist(addr_list);
        }
        addr_list = alist;
        if (secret != None) {
            strlcpy(secret, lsecret, MAXWORDLEN);
        }

        if (!newline) {
            break;
        }
    }

    /* scan for a -- word indicating the start of options */
    // for (app = &addr_list; (ap = *app) != None; app = &ap.next)
    // if (strcmp(ap.word, "--") == 0)
    //     break;
    /* ap = start of options */
    if (ap != None) {
        ap = ap.next; /* first option */
        free(*app); /* free the "--" word */
        *app = None; /* terminate addr list */
    }
    if (opts != None) {
        *opts = ap;
    } else if (ap != None) {
        free_wordlist(ap);
    }
    if (addrs != None) {
        *addrs = addr_list;
    } else if (addr_list != None) {
        free_wordlist(addr_list);
    }

    return best_flag;
}

/*
 * wordlist_count - return the number of items in a wordlist
 */
pub fn wordlist_count(wp: &mut wordlist) {
    let letn: i32;

    // for (n = 0; wp != None; wp = wp.next)
    // += 1n;
    return n;
}

/*
 * free_wordlist - release memory allocated for a wordlist.
 */
pub fn free_wordlist(wp: &mut wordlist) {
    let mut next: &mut wordlist;

    while (wp != None) {
        next = wp.next;
        free(wp);
        wp = next;
    }
}
