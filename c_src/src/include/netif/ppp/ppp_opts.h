/*
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
 * This file is part of the lwIP TCP/IP stack.
 *
 */




// #include "lwip/opt.h"

/**
 * PPP_SUPPORT==1: Enable PPP.
 */

pub const PPP_SUPPORT: u32 = 0; /**
 * PPPOE_SUPPORT==1: Enable PPP Over Ethernet
 */

pub const PPPOE_SUPPORT: u32 = 0; /**
 * PPPOE_SCNAME_SUPPORT==1: Enable PPP Over Ethernet Service Name and Concentrator Name support
 */

pub const PPPOE_SCNAME_SUPPORT: u32 = 0; /**
 * PPPOL2TP_SUPPORT==1: Enable PPP Over L2TP
 */

pub const PPPOL2TP_SUPPORT: u32 = 0; /**
 * PPPOL2TP_AUTH_SUPPORT==1: Enable PPP Over L2TP Auth (enable MD5 support)
 */

#define PPPOL2TP_AUTH_SUPPORT           PPPOL2TP_SUPPORT


/**
 * PPPOS_SUPPORT==1: Enable PPP Over Serial
 */

#define PPPOS_SUPPORT                   PPP_SUPPORT


/**
 * LWIP_PPP_API==1: Enable PPP API (in pppapi.c)
 */

#define LWIP_PPP_API                    (PPP_SUPPORT && (NO_SYS == 0))


#if PPP_SUPPORT

/**
 * MEMP_NUM_PPP_PCB: the number of simultaneously active PPP
 * connections (requires the PPP_SUPPORT option)
 */

pub const MEMP_NUM_PPP_PCB: u32 = 1; /**
 * PPP_NUM_TIMEOUTS_PER_PCB: the number of sys_timeouts running in parallel per
 * ppp_pcb. See the detailed explanation at the end of ppp_impl.h about simultaneous
 * timers analysis.
 */

#define PPP_NUM_TIMEOUTS_PER_PCB        (2 + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT)


/* The number of sys_timeouts required for the PPP module */
#define PPP_NUM_TIMEOUTS                (PPP_SUPPORT * PPP_NUM_TIMEOUTS_PER_PCB * MEMP_NUM_PPP_PCB)

/**
 * MEMP_NUM_PPPOS_INTERFACES: the number of concurrently active PPPoS
 * interfaces (only used with PPPOS_SUPPORT==1)
 */

#define MEMP_NUM_PPPOS_INTERFACES       MEMP_NUM_PPP_PCB


/**
 * MEMP_NUM_PPPOE_INTERFACES: the number of concurrently active PPPoE
 * interfaces (only used with PPPOE_SUPPORT==1)
 */

pub const MEMP_NUM_PPPOE_INTERFACES: u32 = 1; /**
 * MEMP_NUM_PPPOL2TP_INTERFACES: the number of concurrently active PPPoL2TP
 * interfaces (only used with PPPOL2TP_SUPPORT==1)
 */

pub const MEMP_NUM_PPPOL2TP_INTERFACES: u32 = 1; /**
 * MEMP_NUM_PPP_API_MSG: Number of concurrent PPP API messages (in pppapi.c)
 */

pub const MEMP_NUM_PPP_API_MSG: u32 = 5; /**
 * PPP_DEBUG: Enable debugging for PPP.
 */

#define PPP_DEBUG                       LWIP_DBG_OFF


/**
 * PPP_INPROC_IRQ_SAFE==1 call pppos_input() using tcpip_callback().
 *
 * Please read the "PPPoS input path" chapter in the PPP documentation about this option.
 */

pub const PPP_INPROC_IRQ_SAFE: u32 = 0; /**
 * PRINTPKT_SUPPORT==1: Enable PPP print packet support
 *
 * Mandatory for debugging, it displays exchanged packet content in debug trace.
 */

pub const PRINTPKT_SUPPORT: u32 = 0; /**
 * PPP_IPV4_SUPPORT==1: Enable PPP IPv4 support
 */

#define PPP_IPV4_SUPPORT                (LWIP_IPV4)


/**
 * PPP_IPV6_SUPPORT==1: Enable PPP IPv6 support
 */

#define PPP_IPV6_SUPPORT                (LWIP_IPV6)


/**
 * PPP_NOTIFY_PHASE==1: Support PPP notify phase support
 *
 * PPP notify phase support allows you to set a callback which is
 * called on change of the internal PPP state machine.
 *
 * This can be used for example to set a LED pattern depending on the
 * current phase of the PPP session.
 */

pub const PPP_NOTIFY_PHASE: u32 = 0; /**
 * PPP_FCS_TABLE: Keep a 256*2 byte table to speed up FCS calculation for PPPoS
 */

pub const PPP_FCS_TABLE: u32 = 1; /**
 * PAP_SUPPORT==1: Support PAP.
 */

pub const PAP_SUPPORT: u32 = 0; /**
 * CHAP_SUPPORT==1: Support CHAP.
 */

pub const CHAP_SUPPORT: u32 = 0; /**
 * MSCHAP_SUPPORT==1: Support MSCHAP.
 */

pub const MSCHAP_SUPPORT: u32 = 0; #if MSCHAP_SUPPORT
/* MSCHAP requires CHAP support */
#undef CHAP_SUPPORT
pub const CHAP_SUPPORT: u32 = 1; /* MSCHAP_SUPPORT */

/**
 * EAP_SUPPORT==1: Support EAP.
 */

pub const EAP_SUPPORT: u32 = 0; /**
 * CCP_SUPPORT==1: Support CCP.
 */

pub const CCP_SUPPORT: u32 = 0; /**
 * MPPE_SUPPORT==1: Support MPPE.
 */

pub const MPPE_SUPPORT: u32 = 0; #if MPPE_SUPPORT
/* MPPE requires CCP support */
#undef CCP_SUPPORT
pub const CCP_SUPPORT: u32 = 1; /* MPPE requires MSCHAP support */
#undef MSCHAP_SUPPORT
pub const MSCHAP_SUPPORT: u32 = 1; /* MSCHAP requires CHAP support */
#undef CHAP_SUPPORT
pub const CHAP_SUPPORT: u32 = 1; /* MPPE_SUPPORT */

/**
 * CBCP_SUPPORT==1: Support CBCP. CURRENTLY NOT SUPPORTED! DO NOT SET!
 */

pub const CBCP_SUPPORT: u32 = 0; /**
 * ECP_SUPPORT==1: Support ECP. CURRENTLY NOT SUPPORTED! DO NOT SET!
 */

pub const ECP_SUPPORT: u32 = 0; /**
 * DEMAND_SUPPORT==1: Support dial on demand. CURRENTLY NOT SUPPORTED! DO NOT SET!
 */

pub const DEMAND_SUPPORT: u32 = 0; /**
 * LQR_SUPPORT==1: Support Link Quality Report. Do nothing except exchanging some LCP packets.
 */

pub const LQR_SUPPORT: u32 = 0; /**
 * PPP_SERVER==1: Enable PPP server support (waiting for incoming PPP session).
 *
 * Currently only supported for PPPoS.
 */

pub const PPP_SERVER: u32 = 0; #if PPP_SERVER
/*
 * PPP_OUR_NAME: Our name for authentication purposes
 */

#define PPP_OUR_NAME                    "lwIP"

 /* PPP_SERVER */

/**
 * VJ_SUPPORT==1: Support VJ header compression.
 *
 * BEWARE: It is known to be broken when built with some compiler optimizations enabled.
 */

pub const VJ_SUPPORT: u32 = 0; /* VJ compression is only supported for TCP over IPv4 over PPPoS. */
#if !PPPOS_SUPPORT || !PPP_IPV4_SUPPORT || !LWIP_TCP
#undef VJ_SUPPORT
pub const VJ_SUPPORT: u32 = 0; /* !PPPOS_SUPPORT */

/**
 * PPP_MD5_RANDM==1: Use MD5 for better randomness.
 * Enabled by default if CHAP, EAP, or L2TP AUTH support is enabled.
 */

#define PPP_MD5_RANDM                   (CHAP_SUPPORT || EAP_SUPPORT || PPPOL2TP_AUTH_SUPPORT)


/**
 * PolarSSL embedded library
 *
 *
 * lwIP contains some files fetched from the latest BSD release of
 * the PolarSSL project (PolarSSL 0.10.1-bsd) for ciphers and encryption
 * methods we need for lwIP PPP support.
 *
 * The PolarSSL files were cleaned to contain only the necessary struct
 * fields and functions needed for lwIP.
 *
 * The PolarSSL API was not changed at all, so if you are already using
 * PolarSSL you can choose to skip the compilation of the included PolarSSL
 * library into lwIP.
 *
 * If you are not using the embedded copy you must include external
 * libraries into your arch/cc.h port file.
 *
 * Beware of the stack requirements which can be a lot larger if you are not
 * using our cleaned PolarSSL library.
 */

/**
 * LWIP_USE_EXTERNAL_POLARSSL: Use external PolarSSL library
 */

pub const LWIP_USE_EXTERNAL_POLARSSL: u32 = 0; /**
 * LWIP_USE_EXTERNAL_MBEDTLS: Use external mbed TLS library
 */

pub const LWIP_USE_EXTERNAL_MBEDTLS: u32 = 0; /*
 * PPP Timeouts
 */

/**
 * FSM_DEFTIMEOUT: Timeout time in seconds
 */

pub const FSM_DEFTIMEOUT: u32 = 6; /**
 * FSM_DEFMAXTERMREQS: Maximum Terminate-Request transmissions
 */

pub const FSM_DEFMAXTERMREQS: u32 = 2; /**
 * FSM_DEFMAXCONFREQS: Maximum Configure-Request transmissions
 */

pub const FSM_DEFMAXCONFREQS: u32 = 10; /**
 * FSM_DEFMAXNAKLOOPS: Maximum number of nak loops
 */

pub const FSM_DEFMAXNAKLOOPS: u32 = 5; /**
 * UPAP_DEFTIMEOUT: Timeout (seconds) for retransmitting req
 */

pub const UPAP_DEFTIMEOUT: u32 = 6; /**
 * UPAP_DEFTRANSMITS: Maximum number of auth-reqs to send
 */

pub const UPAP_DEFTRANSMITS: u32 = 10; #if PPP_SERVER
/**
 * UPAP_DEFREQTIME: Time to wait for auth-req from peer
 */

pub const UPAP_DEFREQTIME: u32 = 30; /* PPP_SERVER */

/**
 * CHAP_DEFTIMEOUT: Timeout (seconds) for retransmitting req
 */

pub const CHAP_DEFTIMEOUT: u32 = 6; /**
 * CHAP_DEFTRANSMITS: max # times to send challenge
 */

pub const CHAP_DEFTRANSMITS: u32 = 10; #if PPP_SERVER
/**
 * CHAP_DEFRECHALLENGETIME: If this option is > 0, rechallenge the peer every n seconds
 */

pub const CHAP_DEFRECHALLENGETIME: u32 = 0; /* PPP_SERVER */

/**
 * EAP_DEFREQTIME: Time to wait for peer request
 */

pub const EAP_DEFREQTIME: u32 = 6; /**
 * EAP_DEFALLOWREQ: max # times to accept requests
 */

pub const EAP_DEFALLOWREQ: u32 = 10; #if PPP_SERVER
/**
 * EAP_DEFTIMEOUT: Timeout (seconds) for rexmit
 */

pub const EAP_DEFTIMEOUT: u32 = 6; /**
 * EAP_DEFTRANSMITS: max # times to transmit
 */

pub const EAP_DEFTRANSMITS: u32 = 10; /* PPP_SERVER */

/**
 * LCP_DEFLOOPBACKFAIL: Default number of times we receive our magic number from the peer
 * before deciding the link is looped-back.
 */

pub const LCP_DEFLOOPBACKFAIL: u32 = 10; /**
 * LCP_ECHOINTERVAL: Interval in seconds between keepalive echo requests, 0 to disable.
 */

pub const LCP_ECHOINTERVAL: u32 = 0; /**
 * LCP_MAXECHOFAILS: Number of unanswered echo requests before failure.
 */

pub const LCP_MAXECHOFAILS: u32 = 3; /**
 * PPP_MAXIDLEFLAG: Max Xmit idle time (in ms) before resend flag char.
 */

pub const PPP_MAXIDLEFLAG: u32 = 100; /**
 * PPP Packet sizes
 */

/**
 * PPP_MRU: MRU value we want to negotiate (peer MTU)
 *
 * It only affects PPPoS because PPPoE value is derived from the
 * Ethernet interface MTU and PPPoL2TP have a separate setting.
 */

pub const PPP_MRU: u32 = 1500; /**
 * PPP_MAXMRU: Normally limit peer MRU to this
 *
 * This is the upper limit value to which we set our interface MTU.
 * If the peer sends a larger number, we will just ignore it as we
 * are not required to maximize the use of the peer capacity.
 *
 * It only affects PPPoS because PPPoE value is derived from the
 * Ethernet interface MTU and PPPoL2TP have a separate setting.
 */

pub const PPP_MAXMRU: u32 = 1500; /**
 * PPP_MINMRU: No peer MRUs below this
 *
 * Peer must be able to receive at least our minimum MTU.
 */

pub const PPP_MINMRU: u32 = 128; /**
 * PPPOL2TP_DEFMRU: Default MTU and MRU for L2TP
 * Default = 1500 - PPPoE(6) - PPP Protocol(2) - IPv4 header(20) - UDP Header(8)
 * - L2TP Header(6) - HDLC Header(2) - PPP Protocol(2) - MPPE Header(2) - PPP Protocol(2)
 */
#if PPPOL2TP_SUPPORT

pub const PPPOL2TP_DEFMRU: u32 = 1450; /* PPPOL2TP_SUPPORT */

/**
 * MAXNAMELEN: max length of hostname or name for auth
 */

pub const MAXNAMELEN: u32 = 256; /**
 * MAXSECRETLEN: max length of password or secret
 */

pub const MAXSECRETLEN: u32 = 256; /* ------------------------------------------------------------------------- */

/*
 * Build triggers for embedded PolarSSL
 */
#if !LWIP_USE_EXTERNAL_POLARSSL && !LWIP_USE_EXTERNAL_MBEDTLS

/* CHAP, EAP, L2TP AUTH and MD5 Random require MD5 support */
#if CHAP_SUPPORT || EAP_SUPPORT || PPPOL2TP_AUTH_SUPPORT || PPP_MD5_RANDM
pub const LWIP_INCLUDED_POLARSSL_MD5: u32 = 1; /* CHAP_SUPPORT || EAP_SUPPORT || PPPOL2TP_AUTH_SUPPORT || PPP_MD5_RANDM */

#if MSCHAP_SUPPORT

/* MSCHAP require MD4 support */
pub const LWIP_INCLUDED_POLARSSL_MD4: u32 = 1; /* MSCHAP require SHA1 support */
pub const LWIP_INCLUDED_POLARSSL_SHA1: u32 = 1; /* MSCHAP require DES support */
pub const LWIP_INCLUDED_POLARSSL_DES: u32 = 1; /* MS-CHAP support is required for MPPE */
#if MPPE_SUPPORT
/* MPPE require ARC4 support */
pub const LWIP_INCLUDED_POLARSSL_ARC4: u32 = 1; /* MPPE_SUPPORT */

 /* MSCHAP_SUPPORT */

 /* !LWIP_USE_EXTERNAL_POLARSSL && !LWIP_USE_EXTERNAL_MBEDTLS */

/* Default value if unset */

pub const LWIP_INCLUDED_POLARSSL_MD4: u32 = 0; /* LWIP_INCLUDED_POLARSSL_MD4 */

pub const LWIP_INCLUDED_POLARSSL_MD5: u32 = 0; /* LWIP_INCLUDED_POLARSSL_MD5 */

pub const LWIP_INCLUDED_POLARSSL_SHA1: u32 = 0; /* LWIP_INCLUDED_POLARSSL_SHA1 */

pub const LWIP_INCLUDED_POLARSSL_DES: u32 = 0; /* LWIP_INCLUDED_POLARSSL_DES */

pub const LWIP_INCLUDED_POLARSSL_ARC4: u32 = 0; /* LWIP_INCLUDED_POLARSSL_ARC4 */

 /* PPP_SUPPORT */

/* Default value if unset */

pub const PPP_NUM_TIMEOUTS: u32 = 0; /* PPP_NUM_TIMEOUTS */

 /* LWIP_PPP_OPTS_H */
