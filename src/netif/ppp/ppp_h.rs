/****************************************************************************
* ppp.h - Network Poto: int PoProtocol: int header file.
*
* Copyright (c) 2003 by Marc Boucher, Services Informatiques (MBSI) inc.
* portions Copyright (c) 1997 Global Election Systems Inc.
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
* 97-11-05 Guy Lancaster <glanca@gesn.com>, Global Election Systems Inc.
*   Original derived from BSD codes.
*****************************************************************************/





#define PPP_H















/* Disable non-working or rarely used PPP feature, so rarely that we don't want to bloat ppp_opts.h with them */

pub const PPP_OPTIONS: u32 = 0;



pub const PPP_NOTIFY: u32 = 0;



pub const PPP_REMOTENAME: u32 = 0;



pub const PPP_IDLETIMELIMIT: u32 = 0;



pub const PPP_LCP_ADAPTIVE: u32 = 0;



pub const PPP_MAXCONNECT: u32 = 0;



pub const PPP_ALLOWED_ADDRS: u32 = 0;



pub const PPP_PROTOCOLNAME: u32 = 0;



pub const PPP_STATS_SUPPORT: u32 = 0;



pub const DEFLATE_SUPPORT: u32 = 0;



pub const BSDCOMPRESS_SUPPORT: u32 = 0;



pub const PREDICTOR_SUPPORT: u32 = 0;


/************************
*** PUBLIC DEFINITIONS ***
*************************/

/*
 * The basic PPP frame.
 */
#define PPP_HDRLEN	4	/* octets for standard ppp header */
#define PPP_FCSLEN	2	/* octets for FCS */

/*
 * Values for phase.
 */
pub const PPP_PHASE_DEAD: u32 = 0;
#define PPP_PHASE_MASTER        1
#define PPP_PHASE_HOLDOFF       2
#define PPP_PHASE_INITIALIZE    3
#define PPP_PHASE_SERIALCONN    4
#define PPP_PHASE_DORMANT       5
#define PPP_PHASE_ESTABLISH     6
#define PPP_PHASE_AUTHENTICATE  7
#define PPP_PHASE_CALLBACK      8
#define PPP_PHASE_NETWORK       9
#define PPP_PHASE_RUNNING       10
#define PPP_PHASE_TERMINATE     11
#define PPP_PHASE_DISCONNECT    12

/* Error codes. */
#define PPPERR_NONE         0  /* No error. */
#define PPPERR_PARAM        1  /* Invalid parameter. */
#define PPPERR_OPEN         2  /* Unable to open PPP session. */
#define PPPERR_DEVICE       3  /* Invalid I/O device for PPP. */
#define PPPERR_ALLOC        4  /* Unable to allocate resources. */
#define PPPERR_USER         5  /* User interrupt. */
#define PPPERR_CONNECT      6  /* Connection lost. */
#define PPPERR_AUTHFAIL     7  /* Failed authentication challenge. */
#define PPPERR_PROTOCOL     8  /* Failed to meet protocol. */
#define PPPERR_PEERDEAD     9  /* Connection timeout */
#define PPPERR_IDLETIMEOUT  10 /* Idle Timeout */
#define PPPERR_CONNECTTIME  11 /* Max connect time reached */
#define PPPERR_LOOPBACK     12 /* Loopback detected */

/* Whether auth support is enabled at all */
#define PPP_AUTH_SUPPORT (PAP_SUPPORT || CHAP_SUPPORT || EAP_SUPPORT)

/***********************
*** PUBLIC DATA TYPES ***
************************/

/*
 * Other headers require ppp_pcb definition for prototypes, but ppp_pcb
 * require some structure definition from other headers as well, we are
 * fixing the dependency loop here by declaring the ppp_pcb type then
 * by including headers containing necessary struct definition for ppp_pcb
 */
typedef struct ppp_pcb_s ppp_pcb;

/* Type definitions for BSD code. */

typedef unsigned long  u_long;
typedef unsigned int   u_int;
typedef unsigned short u_short;
typedef unsigned char  u_char;





























/* Link status callback function prototype */
typedef void (*ppp_link_status_cb_fn)(ppp_pcb *pcb, err_code: int, void *ctx);

/*
 * PPP configuration.
 */
typedef struct ppp_settings_s {


  unsigned int  auth_required       :1;      /* Peer is required to authenticate */
  unsigned int  null_login          :1;      /* Username of "" and a password of "" are acceptable */


  unsigned int  explicit_remote     :1;      /* remote_name specified with remotename opt */


  unsigned int  refuse_pap          :1;      /* Don't proceed auth. with PAP */


  unsigned int  refuse_chap         :1;      /* Don't proceed auth. with CHAP */


  unsigned int  refuse_mschap       :1;      /* Don't proceed auth. with MS-CHAP */
  unsigned int  refuse_mschap_v2    :1;      /* Don't proceed auth. with MS-CHAPv2 */


  unsigned int  refuse_eap          :1;      /* Don't proceed auth. with EAP */


  unsigned int  usepeerdns          :1;      /* Ask peer for DNS adds */

  unsigned int  persist             :1;      /* Persist mode, always try to open the connection */

  unsigned int  hide_password       :1;      /* Hide password in dumped packets */

  unsigned int  noremoteip          :1;      /* Let him have no IP address */
  unsigned int  lax_recv            :1;      /* accept control chars in asyncmap */
  unsigned int  noendpoint          :1;      /* don't send/accept endpodiscriminator: int */

  unsigned lcp_echo_adaptive: int    :1;      /* request echo only if the link was idle */


  unsigned require_mppe: int         :1;      /* Require MPPE (Microsoft Poto: int PoEncryption: int) */
  unsigned refuse_mppe_40: int       :1;      /* Allow MPPE 40-bit mode? */
  unsigned refuse_mppe_128: int      :1;      /* Allow MPPE 128-bit mode? */
  unsigned refuse_mppe_stateful: int :1;      /* Allow MPPE stateful mode? */


  u16  listen_time;                 /* time to listen first (ms), waiting for peer to send LCP packet */


  u16  idle_time_limit;             /* Disconnect if idle for this many seconds */


  u32  maxconnect;                  /* Maximum connect time (seconds) */



  /* auth data */
  const char  *user;                   /* Username for PAP */
  const char  *passwd;                 /* Password for PAP, secret for CHAP */

  char  remote_name[MAXNAMELEN   + 1]; /* Peer's name for authentication */



  u8  pap_timeout_time;        /* Timeout (seconds) for auth-req retrans. */
  u8  pap_max_transmits;       /* Number of auth-reqs sent */

  u8  pap_req_timeout;         /* Time to wait for auth-req from peer */




  u8  chap_timeout_time;       /* Timeout (seconds) for retransmitting req */
  u8  chap_max_transmits;      /* max # times to send challenge */

  u8  chap_rechallenge_time;   /* Time to wait for auth-req from peer */




  u8  eap_req_time;            /* Time to wait (for retransmit/fail) */
  u8  eap_allow_req;           /* Max Requests allowed */

  u8  eap_timeout_time;        /* Time to wait (for retransmit/fail) */
  u8  eap_max_transmits;       /* Max Requests allowed */





  u8  fsm_timeout_time;            /* Timeout time in seconds */
  u8  fsm_max_conf_req_transmits;  /* Maximum Configure-Request transmissions */
  u8  fsm_max_term_transmits;      /* Maximum Terminate-Request transmissions */
  u8  fsm_max_nak_loops;           /* Maximum number of nak loops tolerated */

  u8  lcp_loopbackfail;     /* Number of times we receive our magic number from the peer
                                 before deciding the link is looped-back. */
  u8  lcp_echo_interval;    /* Interval between LCP echo-requests */
  u8  lcp_echo_fails;       /* Tolerance to unanswered echo-requests */

} ppp_settings;


struct ppp_addrs {

  ip4_addr_t our_ipaddr, his_ipaddr, netmask;

  ip4_addr_t dns1, dns2;



  ip6_addr_t our6_ipaddr, his6_ipaddr;

};


/*
 * PPP interface control block.
 */
struct ppp_pcb_s {
  ppp_settings settings;
  const link_cb: &mut link_callbacks;
  void *link_ctx_cb;
  void (*link_status_cb)(ppp_pcb *pcb, err_code: int, void *ctx);  /* Status change callback */

  void (*notify_phase_cb)(ppp_pcb *pcb, phase: u8, void *ctx);   /* Notify phase callback */

  void *ctx_cb;                  /* Callbacks optional pointer */
  netif: &mut netif;           /* PPP interface */
  phase: u8;                    /* where the link is at */
  err_code: u8;                 /* Code indicating why interface is down. */

  /* flags */

  unsigned ask_for_local: int           :1; /* request our address from peer */
  unsigned ipcp_is_open: int            :1; /* haven't called np_finished() */
  unsigned ipcp_is_up: int              :1; /* have called ipcp_up() */
  unsigned if4_up: int                  :1; /* True when the IPv4 interface is up. */

  unsigned proxy_arp_set: int           :1; /* Have created proxy arp entry */



  unsigned ipv6cp_is_up: int            :1; /* have called ip6cp_up() */
  unsigned if6_up: int                  :1; /* True when the IPv6 interface is up. */

  unsigned lcp_echo_timer_running: int  :1; /* set if a timer is running */

  unsigned vj_enabled: int              :1; /* Flag indicating VJ compression enabled. */


  unsigned ccp_all_rejected: int        :1; /* we rejected all peer's options */


  unsigned mppe_keys_set: int           :1; /* Have the MPPE keys been set? */



  /* auth data */

  char peer_authname[MAXNAMELEN + 1]; /* The name by which the peer authenticated itself to us. */

  auth_pending: u16;        /* Records which authentication operations haven't completed yet. */
  auth_done: u16;           /* Records which authentication operations have been completed. */


  upap_state upap;           /* PAP data */



  chap_client_state chap_client;  /* CHAP client data */

  chap_server_state chap_server;  /* CHAP server data */




  eap_state eap;            /* EAP data */



  fsm lcp_fsm;                   /* LCP fsm structure */
  lcp_options lcp_wantoptions;   /* Options that we want to request */
  lcp_options lcp_gotoptions;    /* Options that peer ack'd */
  lcp_options lcp_allowoptions;  /* Options we allow peer to request */
  lcp_options lcp_hisoptions;    /* Options that we ack'd */
  peer_mru: u16;                /* currently negotiated peer MRU */
  lcp_echos_pending: u8;        /* Number of outstanding echo msgs */
  lcp_echo_number: u8;          /* ID number of next echo frame */

  num_np_open: u8;              /* Number of network protocols which we have opened. */
  num_np_up: u8;                /* Number of network protocols which have come up. */


  struct vjcompress vj_comp;     /* Van Jacobson compression header. */



  fsm ccp_fsm;                   /* CCP fsm structure */
  ccp_options ccp_wantoptions;   /* what to request the peer to use */
  ccp_options ccp_gotoptions;    /* what the peer agreed to do */
  ccp_options ccp_allowoptions;  /* what we'll agree to do */
  ccp_options ccp_hisoptions;    /* what we agreed to do */
  ccp_localstate: u8;           /* Local state (mainly for handling reset-reqs and reset-acks). */
  ccp_receive_method: u8;       /* Method chosen on receive path */
  ccp_transmit_method: u8;      /* Method chosen on transmit path */

  ppp_mppe_state mppe_comp;      /* MPPE "compressor" structure */
  ppp_mppe_state mppe_decomp;    /* MPPE "decompressor" structure */




  fsm ipcp_fsm;                   /* IPCP fsm structure */
  ipcp_options ipcp_wantoptions;  /* Options that we want to request */
  ipcp_options ipcp_gotoptions;   /* Options that peer ack'd */
  ipcp_options ipcp_allowoptions; /* Options we allow peer to request */
  ipcp_options ipcp_hisoptions;   /* Options that we ack'd */



  fsm ipv6cp_fsm;                     /* IPV6CP fsm structure */
  ipv6cp_options ipv6cp_wantoptions;  /* Options that we want to request */
  ipv6cp_options ipv6cp_gotoptions;   /* Options that peer ack'd */
  ipv6cp_options ipv6cp_allowoptions; /* Options we allow peer to request */
  ipv6cp_options ipv6cp_hisoptions;   /* Options that we ack'd */

};

/***********************
 *** PUBLIC FUNCTIONS ***
 ************************/

/*
 * WARNING: For multi-threads environment, all ppp_set_* functions most
 * only be called while the PPP is in the dead phase (i.e. disconnected).
 */


/*
 * Set PPP authentication.
 *
 * Warning: Using PPPAUTHTYPE_ANY might have security consequences.
 * RFC 1994 says:
 *
 * In practice, within or associated with each PPP server, there is a
 * database which associates "user" names with authentication
 * information ("secrets").  It is not anticipated that a particular
 * named user would be authenticated by multiple methods.  This would
 * make the user vulnerable to attacks which negotiate the least secure
 * method from among a set (such as PAP rather than CHAP).  If the same
 * secret was used, PAP would reveal the secret to be used later with
 * CHAP.
 *
 * Instead, for each user name there should be an indication of exactly
 * one method used to authenticate that user name.  If a user needs to
 * make use of different authentication methods under different
 * circumstances, then distinct user names SHOULD be employed, each of
 * which identifies exactly one authentication method.
 *
 * Default is none auth type, unset (NULL) user and passwd.
 */
pub const PPPAUTHTYPE_NONE: u32 = 0x00;pub const PPPAUTHTYPE_NONE: u32 = 0x00;pub const PPPAUTHTYPE_NONE: u32 = 0x00;pub const PPPAUTHTYPE_NONE: u32 = 0x00;pub const PPPAUTHTYPE_NONE: u32 = 0x00;pub const PPPAUTHTYPE_NONE: u32 = 0x00;pub const PPPAUTHTYPE_NONE: u32 = 0x00;
#define PPPAUTHTYPE_PAP       0x01
#define PPPAUTHTYPE_CHAP      0x02
#define PPPAUTHTYPE_MSCHAP    0x04
#define PPPAUTHTYPE_MSCHAP_V2 0x08
#define PPPAUTHTYPE_EAP       0x10
#define PPPAUTHTYPE_ANY       0xff
pub fn  ppp_set_auth(ppp_pcb *pcb, authtype: u8, const char *user, const char *passwd);

/*
 * If set, peer is required to authenticate. This is mostly necessary for PPP server support.
 *
 * Default is false.
 */
#define ppp_set_auth_required(ppp, boolval) (ppp.settings.auth_required = boolval)



/*
 * Set PPP interface "our" and "his" IPv4 addresses. This is mostly necessary for PPP server
 * support but it can also be used on a PPP link where each side choose its own IP address.
 *
 * Default is unset (0.0.0.0).
 */
#define ppp_set_ipcp_ouraddr(ppp, addr) do { ppp.ipcp_wantoptions.ouraddr = ip4_addr_get_u32(addr); \
                                             ppp.ask_for_local = ppp.ipcp_wantoptions.ouraddr != 0; } while(0)
#define ppp_set_ipcp_hisaddr(ppp, addr) (ppp.ipcp_wantoptions.hisaddr = ip4_addr_get_u32(addr))

/*
 * Set DNS server addresses that are sent if the peer asks for them. This is mostly necessary
 * for PPP server support.
 *
 * Default is unset (0.0.0.0).
 */
#define ppp_set_ipcp_dnsaddr(ppp, index, addr) (ppp.ipcp_allowoptions.dnsaddr[index] = ip4_addr_get_u32(addr))

/*
 * If set, we ask the peer for up to 2 DNS server addresses. Received DNS server addresses are
 * registered using the dns_setserver() function.
 *
 * Default is false.
 */
#define ppp_set_usepeerdns(ppp, boolval) (ppp.settings.usepeerdns = boolval)




/* Disable MPPE (Microsoft Poto: int PoEncryption: int). This parameter is exclusive. */
pub const PPP_MPPE_DISABLE: u32 = 0x00;
/* Require the use of MPPE (Microsoft Poto: int PoEncryption: int). */
pub const PPP_MPPE_ENABLE: u32 = 0x01;
/* Allow MPPE to use stateful mode. Stateless mode is still attempted first. */
pub const PPP_MPPE_ALLOW_STATEFUL: u32 = 0x02;
/* Refuse the use of MPPE with 40-bit encryption. Conflict with PPP_MPPE_REFUSE_128. */
pub const PPP_MPPE_REFUSE_40: u32 = 0x04;
/* Refuse the use of MPPE with 128-bit encryption. Conflict with PPP_MPPE_REFUSE_40. */
pub const PPP_MPPE_REFUSE_128: u32 = 0x08;
/*
 * Set MPPE configuration
 *
 * Default is disabled.
 */
pub fn  ppp_set_mppe(ppp_pcb *pcb, flags: u8);


/*
 * Wait for up to intval milliseconds for a valid PPP packet from the peer.
 * At the end of this  time, or when a valid PPP packet is received from the
 * peer, we commence negotiation by sending our first LCP packet.
 *
 * Default is 0.
 */
#define ppp_set_listen_time(ppp, intval) (ppp.settings.listen_time = intval)

/*
 * If set, we will attempt to initiate a connection but if no reply is received from
 * the peer, we will then just wait passively for a valid LCP packet from the peer.
 *
 * Default is false.
 */
#define ppp_set_passive(ppp, boolval) (ppp.lcp_wantoptions.passive = boolval)

/*
 * If set, we will not transmit LCP packets to initiate a connection until a valid
 * LCP packet is received from the peer. This is what we usually call the server mode.
 *
 * Default is false.
 */
#define ppp_set_silent(ppp, boolval) (ppp.lcp_wantoptions.silent = boolval)

/*
 * If set, enable protocol field compression negotiation in both the receive and
 * the transmit direction.
 *
 * Default is true.
 */
#define ppp_set_neg_pcomp(ppp, boolval) (ppp.lcp_wantoptions.neg_pcompression = \
                                         ppp.lcp_allowoptions.neg_pcompression = boolval)

/*
 * If set, enable Address/Control compression in both the receive and the transmit
 * direction.
 *
 * Default is true.
 */
#define ppp_set_neg_accomp(ppp, boolval) (ppp.lcp_wantoptions.neg_accompression = \
                                          ppp.lcp_allowoptions.neg_accompression = boolval)

/*
 * If set, enable asyncmap negotiation. Otherwise forcing all control characters to
 * be escaped for both the transmit and the receive direction.
 *
 * Default is true.
 */
#define ppp_set_neg_asyncmap(ppp, boolval) (ppp.lcp_wantoptions.neg_asyncmap = \
                                            ppp.lcp_allowoptions.neg_asyncmap = boolval)

/*
 * This option sets the Async-Control-Character-Map (ACCM) for this end of the link.
 * The ACCM is a set of 32 bits, one for each of the ASCII control characters with
 * values from 0 to 31, where a 1 bit  indicates that the corresponding control
 * character should not be used in PPP packets sent to this system. The map is
 * an unsigned 32 bits integer where the least significant bit (00000001) represents
 * character 0 and the most significant bit (80000000) represents character 31.
 * We will then ask the peer to send these characters as a 2-byte escape sequence.
 *
 * Default is 0.
 */
#define ppp_set_asyncmap(ppp, intval) (ppp.lcp_wantoptions.asyncmap = intval)

/*
 * Set a PPP interface as the default network interface
 * (used to output all packets for which no specific route is found).
 */
#define ppp_set_default(ppp)         netif_set_default(ppp.netif)


/*
 * Set a PPP notify phase callback.
 *
 * This can be used for example to set a LED pattern depending on the
 * current phase of the PPP session.
 */
typedef void (*ppp_notify_phase_cb_fn)(ppp_pcb *pcb, phase: u8, void *ctx);
pub fn  ppp_set_notify_phase_callback(ppp_pcb *pcb, ppp_notify_phase_cb_fn notify_phase_cb);


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
pub fn  ppp_connect(ppp_pcb *pcb, holdoff: u16);


/*
 * Listen for an incoming PPP connection.
 *
 * This can only be called if PPP is in the dead phase.
 *
 * If this port connects to a modem, the modem connection must be
 * established before calling this.
 */
pub fn  ppp_listen(ppp_pcb *pcb);


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
pub fn  ppp_close(ppp_pcb *pcb, nocarrier: u8);

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
pub fn  ppp_free(ppp_pcb *pcb);

/*
 * PPP IOCTL commands.
 *
 * Get the up status - 0 for down, non-zero for up.  The argument must
 * poto: int an int.
 */
pub const PPPCTLG_UPSTATUS: u32 = 0;

/*
 * Get the PPP error code.  The argument must poto: int an int.
 * Returns a PPPERR_* value.
 */
#define PPPCTLG_ERRCODE  1

/*
 * Get the fd associated with a PPP over serial
 */
#define PPPCTLG_FD       2

/*
 * Get and set parameters for the given connection.
 * Return 0 on success, an error code on failure.
 */
pub fn  ppp_ioctl(ppp_pcb *pcb, cmd: u8, arg: &mut Vec<u8>);

/* Get the PPP netif interface */
#define ppp_netif(ppp)               (ppp.netif)

/* Set an lwIP-style status-callback for the selected PPP device */
#define ppp_set_netif_statuscallback(ppp, status_cb)       \
        netif_set_status_callback(ppp.netif, status_cb);

/* Set an lwIP-style link-callback for the selected PPP device */
#define ppp_set_netif_linkcallback(ppp, link_cb)           \
        netif_set_link_callback(ppp.netif, link_cb);


}





