use crate::ppp::ppp_opts_h::{CHAP_SUPPORT, EAP_SUPPORT, PAP_SUPPORT};
use super::{eap_h::eap_state, lcp_h::lcp_options, upap_h::upap_state, vj_h::vjcompress};

/****************************************************************************
* ppp.h - Network Poto: i32 PoProtocol: i32 header file.
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

//  Disable non-working or rarely used PPP feature, so rarely that we don't want to bloat ppp_opts.h with them 

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


// The basic PPP frame.
pub const PPP_HDRLEN: u32 = 4; //  octets for standard ppp header
pub const PPP_FCSLEN: u32 = 2; //  octets for FCS 

// Values for phase.
pub const PPP_PHASE_DEAD: u32 = 0;
pub const PPP_PHASE_MASTER: u32 = 1;
pub const PPP_PHASE_HOLDOFF: u32 = 2;
pub const PPP_PHASE_INITIALIZE: u32 = 3;
pub const PPP_PHASE_SERIALCONN: u32 = 4;
pub const PPP_PHASE_DORMANT: u32 = 5;
pub const PPP_PHASE_ESTABLISH: u32 = 6;
pub const PPP_PHASE_AUTHENTICATE: u32 = 7;
pub const PPP_PHASE_CALLBACK: u32 = 8;
pub const PPP_PHASE_NETWORK: u32 = 9;
pub const PPP_PHASE_RUNNING: u32 = 10;
pub const PPP_PHASE_TERMINATE: u32 = 11;
pub const PPP_PHASE_DISCONNECT: u32 = 12;

//  Error codes. 
pub const PPPERR_NONE: u32 = 0; //  No error.
pub const PPPERR_PARAM: u32 = 1; //  Invalid parameter. 
pub const PPPERR_OPEN: u32 = 2; //  Unable to open PPP session. 
pub const PPPERR_DEVICE: u32 = 3; //  Invalid I/O device for PPP. 
pub const PPPERR_ALLOC: u32 = 4; //  Unable to allocate resources. 
pub const PPPERR_USER: u32 = 5; //  User interrupt. 
pub const PPPERR_CONNECT: u32 = 6; //  Connection lost. 
pub const PPPERR_AUTHFAIL: u32 = 7; //  Failed authentication challenge. 
pub const PPPERR_PROTOCOL: u32 = 8; //  Failed to meet protocol. 
pub const PPPERR_PEERDEAD: u32 = 9; //  Connection timeout 
pub const PPPERR_IDLETIMEOUT: u32 = 10; //  Idle Timeout 
pub const PPPERR_CONNECTTIME: u32 = 11; //  Max connect time reached 
pub const PPPERR_LOOPBACK: u32 = 12; //  Loopback detected 

pub const PPP_AUTH_SUPPORT: u32 = (PAP_SUPPORT || CHAP_SUPPORT || EAP_SUPPORT); //  Whether auth support is enabled at all

// PUBLIC DATA TYPES


/*
 * Other headers require ppp_pcb definition for prototypes, but ppp_pcb
 * require some structure definition from other headers as well, we are
 * fixing the dependency loop here by declaring the ppp_pcb type then
 * by including headers containing necessary struct definition for ppp_pcb
 */

//  Link status callback function prototype 
// typedef void (*ppp_link_status_cb_fn)(pcb: &mut ppp_pcb, err_code: i32, ctx: &mut Vec<u8>);
type ppp_link_status_cb_fn = fn(pcb: &mut PppCtx, err_code: i32, ctx: &mut Vec<u8>);

/*
 * PPP configuration.
 */
pub struct ppp_settings {
    auth_required: bool, //  Peer is required to authenticate 
    None_login: bool,    //  Username of "" and a password of "" are acceptable 

    explicit_remote: bool, //  remote_name specified with remotename opt 

    refuse_pap: bool, //  Don't proceed auth. with PAP 

    refuse_chap: bool, //  Don't proceed auth. with CHAP 

    refuse_mschap: bool,    //  Don't proceed auth. with MS-CHAP 
    refuse_mschap_v2: bool, //  Don't proceed auth. with MS-CHAPv2 

    refuse_eap: bool, //  Don't proceed auth. with EAP 

    usepeerdns: bool, //  Ask peer for DNS adds 

    persist: bool, //  Persist mode, always try to open the connection 

    hide_password: bool, //  Hide password in dumped packets 

    noremoteip: bool, //  Let him have no IP address 
    lax_recv: bool,   //  accept control chars in asyncmap 
    noendpoint: bool, //  don't send/accept endpodiscriminator: i32 

    lcp_echo_adaptive: bool, //  request echo only if the link was idle 

    require_mppe: bool, //  Require MPPE (Microsoft Poto: i32 PoEncryption: i32) 
    refuse_mppe_40: bool, //  Allow MPPE 40-bit mode? 
    refuse_mppe_128: bool, //  Allow MPPE 128-bit mode? 
    refuse_mppe_stateful: bool, //  Allow MPPE stateful mode? 

    listen_time: u16, //  time to listen first (ms), waiting for peer to send LCP packet 

    idle_time_limit: u16, //  Disconnect if idle for this many seconds 

    maxconnect: u32, //  Maximum connect time (seconds) 

    //  auth data 
    user: String,   //  Username for PAP 
    passwd: String, //  Password for PAP, secret for CHAP 

    pub remote_name: String, //  Peer's name for authentication 

    pap_timeout_time: u8,  //  Timeout (seconds) for auth-req retrans. 
    pap_max_transmits: u8, //  Number of auth-reqs sent 

    pap_req_timeout: u8, //  Time to wait for auth-req from peer 

    chap_timeout_time: u8,  //  Timeout (seconds) for retransmitting req 
    chap_max_transmits: u8, //  max # times to send challenge 

    chap_rechallenge_time: u8, //  Time to wait for auth-req from peer 

    eap_req_time: u8,  //  Time to wait (for retransmit/fail) 
    eap_allow_req: u8, //  Max Requests allowed 

    eap_timeout_time: u8,  //  Time to wait (for retransmit/fail) 
    eap_max_transmits: u8, //  Max Requests allowed 

    fsm_timeout_time: u8,           //  Timeout time in seconds 
    fsm_max_conf_req_transmits: u8, //  Maximum Configure-Request transmissions 
    fsm_max_term_transmits: u8,     //  Maximum Terminate-Request transmissions 
    fsm_max_nak_loops: u8,          //  Maximum number of nak loops tolerated 

    lcp_loopbackfail: u8, /* Number of times we receive our magic number from the peer
                          before deciding the link is looped-back. */
    lcp_echo_interval: u8, //  Interval between LCP echo-requests 
    lcp_echo_fails: u8,    //  Tolerance to unanswered echo-requests 
}

pub struct ppp_addrs {
    pub our_ipaddr: LwipAddr,
    pub his_ipaddr: LwipAddr,
    pub netmask: LwipAddr,
    pub dns1: LwipAddr,
    pub dns2: LwipAddr,
    pub our6_ipaddr: LwipAddr,
    pub his6_ipaddr: LwipAddr,
}

// void (*link_status_cb)(pcb: &mut ppp_pcb, err_code: i32, ctx: &mut Vec<u8>),  //  Status change callback 
// type link_status_cb = fn(pcb: &mut ppp_pcb, err_code: i32, ctx: &mut Vec<u8>);

// void (*notify_phase_cb)(pcb: &mut ppp_pcb, phase: u8, ctx: &mut Vec<u8>),   //  Notify phase callback 
// type notify_phase_cb = fn(pcb: &mut ppp_pcb, phase: u8, ctx: &mut Vec<u8>);

/*
 * PPP interface control block.
 */
pub struct PppCtx {
    pub settings: ppp_settings,
    pub link_cb: link_callbacks,
    pub link_ctx_cb: Vec<u8>,
    pub ctx_cb: Vec<u8>, //  Callbacks optional pointer 
    pub netif: NetIfc,   //  PPP interface 
    pub phase: u8,       //  where the link is at 
    pub err_code: u8,    //  Code indicating why interface is down.
    //  flags 
    pub ask_for_local: bool, //  request our address from peer 
    pub ipcp_is_open: bool,  //  haven't called np_finished() 
    pub ipcp_is_up: bool,    //  have called ipcp_up() 
    pub if4_up: bool,        //  True when the IPv4 interface is up.
    pub proxy_arp_set: bool, //  Have created proxy arp entry
    pub ipv6cp_is_up: bool, //  have called ip6cp_up() 
    pub if6_up: bool,       //  True when the IPv6 interface is up.
    pub lcp_echo_timer_running: bool, //  set if a timer is running
    pub vj_enabled: bool, //  Flag indicating VJ compression enabled. 

    pub ccp_all_rejected: bool, //  we rejected all peer's options 

    pub mppe_keys_set: bool, //  Have the MPPE keys been set? 

    //  auth data 
    pub peer_authname: String, //  The name by which the peer authenticated itself to us. 

    pub auth_pending: u16, //  Records which authentication operations haven't completed yet. 
    pub auth_pending: u16,
    pub auth_done: u16, //  Records which authentication operations have been completed. 

    pub upap: upap_state, //  PAP data 

    pub chap_client: chap_client_state, //  CHAP client data 

    pub chap_server: chap_server_state, //  CHAP server data 

    pub eap: eap_state, //  EAP data 

    pub lcp_fsm: fsm,                  //  LCP fsm structure 
    pub lcp_wantoptions: lcp_options,  //  Options that we want to request 
    pub lcp_gotoptions: lcp_options,   //  Options that peer ack'd 
    pub lcp_allowoptions: lcp_options, //  Options we allow peer to request 
    pub lcp_hisoptions: lcp_options,   //  Options that we ack'd 
    pub peer_mru: u16,                 //  currently negotiated peer MRU 
    pub lcp_echos_pending: u8,         //  Number of outstanding echo msgs 

    pub num_np_open: u8, //  Number of network protocols which we have opened. 
    pub num_np_open: u8,
    pub num_np_up: u8, //  Number of network protocols which have come up. 

    pub vj_comp: vjcompress, //  Van Jacobson compression header. 

    ccp_fsm: fsm,                  //  CCP fsm structure 
    ccp_wantoptions: ccp_options,  //  what to request the peer to use 
    ccp_gotoptions: ccp_options,   // ,    //  what the peer agreed to do 
    ccp_allowoptions: ccp_options, //  what we'll agree to do 
    ccp_hisoptions: ccp_options,   //  what we agreed to do 
    ccp_localstate: u8, //  Local state (mainly for handling reset-reqs and reset-acks). 
    ccp_receive_method: u8, //  Method chosen on receive path 
    ccp_transmit_method: u8, //  Method chosen on transmit path 

    mppe_comp: ppp_mppe_state,   //  MPPE "compressor" structure 
    mppe_decomp: ppp_mppe_state, //  MPPE "decompressor" structure 

    ipcp_fsm: fsm,                   //  IPCP fsm structure 
    ipcp_wantoptions: ipcp_options,  //  Options that we want to request 
    ipcp_gotoptions: ipcp_options,   // ,   //  Options that peer ack'd 
    ipcp_allowoptions: ipcp_options, //  Options we allow peer to request 
    ipcp_hisoptions: ipcp_options,   //  Options that we ack'd 

    ipv6cp_fsm: fsm,                         //  IPV6CP fsm structure 
    pub ipv6cp_wantoptions: ipv6cp_options,  //  Options that we want to request 
    ipv6cp_gotoptions: ipv6cp_options,       //  Options that peer ack'd 
    pub ipv6cp_allowoptions: ipv6cp_options, //  Options we allow peer to request 
    pub ipv6cp_hisoptions: ipv6cp_options,   //  Options that we ack'd 
}

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
pub const PPPAUTHTYPE_NONE: u32 = 0x00;
pub const PPPAUTHTYPE_PAP: u32 = 001;
pub const PPPAUTHTYPE_CHAP: u32 = 0x02;
pub const PPPAUTHTYPE_MSCHAP: u32 = 0x04;
pub const PPPAUTHTYPE_MSCHAP_V2: u32 = 0x08;
pub const PPPAUTHTYPE_EAP: u32 = 0x10;
pub const PPPAUTHTYPE_ANY: u32 = 0xff;
// pub fn  ppp_set_auth(pcb: &mut ppp_pcb, authtype: u8, user: &String, passwd: &String);

/*
 * If set, peer is required to authenticate. This is mostly necessary for PPP server support.
 *
 * Default is false.
 */
// #define ppp_set_auth_required(ppp, boolval) (ppp.settings.auth_required = boolval)

/*
 * Set PPP interface "our" and "his" IPv4 addresses. This is mostly necessary for PPP server
 * support but it can also be used on a PPP link where each side choose its own IP address.
 *
 * Default is unset (0.0.0.0).
 */
// #define ppp_set_ipcp_ouraddr(ppp, addr) loop { ppp.ipcp_wantoptions.ouraddr = ip4_addr_get_u32(addr); \
//                                              ppp.ask_for_local = ppp.ipcp_wantoptions.ouraddr != 0; } while(0)
// #define ppp_set_ipcp_hisaddr(ppp, addr) (ppp.ipcp_wantoptions.hisaddr = ip4_addr_get_u32(addr))

/*
 * Set DNS server addresses that are sent if the peer asks for them. This is mostly necessary
 * for PPP server support.
 *
 * Default is unset (0.0.0.0).
 */
// #define ppp_set_ipcp_dnsaddr(ppp, index, addr) (ppp.ipcp_allowoptions.dnsaddr[index] = ip4_addr_get_u32(addr))

/*
 * If set, we ask the peer for up to 2 DNS server addresses. Received DNS server addresses are
 * registered using the dns_setserver() function.
 *
 * Default is false.
 */
// #define ppp_set_usepeerdns(ppp, boolval) (ppp.settings.usepeerdns = boolval)

//  Disable MPPE (Microsoft Poto: i32 PoEncryption: i32). This parameter is exclusive. 
pub const PPP_MPPE_DISABLE: u32 = 0x00;
//  Require the use of MPPE (Microsoft Poto: i32 PoEncryption: i32). 
pub const PPP_MPPE_ENABLE: u32 = 0x01;
//  Allow MPPE to use stateful mode. Stateless mode is still attempted first. 
pub const PPP_MPPE_ALLOW_STATEFUL: u32 = 0x02;
//  Refuse the use of MPPE with 40-bit encryption. Conflict with PPP_MPPE_REFUSE_128. 
pub const PPP_MPPE_REFUSE_40: u32 = 0x04;
//  Refuse the use of MPPE with 128-bit encryption. Conflict with PPP_MPPE_REFUSE_40. 
pub const PPP_MPPE_REFUSE_128: u32 = 0x08;
/*
 * Set MPPE configuration
 *
 * Default is disabled.
 */
// pub fn  ppp_set_mppe(pcb: &mut ppp_pcb, flags: u8);

/*
 * Wait for up to intval milliseconds for a valid PPP packet from the peer.
 * At the end of this  time, or when a valid PPP packet is received from the
 * peer, we commence negotiation by sending our first LCP packet.
 *
 * Default is 0.
 */
// #define ppp_set_listen_time(ppp, intval) (ppp.settings.listen_time = intval)

/*
 * If set, we will attempt to initiate a connection but if no reply is received from
 * the peer, we will then just wait passively for a valid LCP packet from the peer.
 *
 * Default is false.
 */
// #define ppp_set_passive(ppp, boolval) (ppp.lcp_wantoptions.passive = boolval)

/*
 * If set, we will not transmit LCP packets to initiate a connection until a valid
 * LCP packet is received from the peer. This is what we usually call the server mode.
 *
 * Default is false.
 */
// #define ppp_set_silent(ppp, boolval) (ppp.lcp_wantoptions.silent = boolval)

/*
 * If set, enable protocol field compression negotiation in both the receive and
 * the transmit direction.
 *
 * Default is true.
 */
// #define ppp_set_neg_pcomp(ppp, boolval) (ppp.lcp_wantoptions.neg_pcompression = \
//                                          ppp.lcp_allowoptions.neg_pcompression = boolval)

/*
 * If set, enable Address/Control compression in both the receive and the transmit
 * direction.
 *
 * Default is true.
 */
// #define ppp_set_neg_accomp(ppp, boolval) (ppp.lcp_wantoptions.neg_accompression = \
//                                           ppp.lcp_allowoptions.neg_accompression = boolval)

/*
 * If set, enable asyncmap negotiation. Otherwise forcing all control characters to
 * be escaped for both the transmit and the receive direction.
 *
 * Default is true.
 */
// #define ppp_set_neg_asyncmap(ppp, boolval) (ppp.lcp_wantoptions.neg_asyncmap = \
//                                             ppp.lcp_allowoptions.neg_asyncmap = boolval)

/*
 * This option sets the Async-Control-Character-Map (ACCM) for this end of the link.
 * The ACCM is a set of 32 bits, one for each of the ASCII control characters with
 * values from 0 to 31, where a 1 bit  indicates that the corresponding control
 * character should not be used in PPP packets sent to this system. The map is
 * an  32 bits integer where the least significant bit (00000001) represents
 * character 0 and the most significant bit (80000000) represents character 31.
 * We will then ask the peer to send these characters as a 2-byte escape sequence.
 *
 * Default is 0.
 */
// #define ppp_set_asyncmap(ppp, intval) (ppp.lcp_wantoptions.asyncmap = intval)

/*
 * Set a PPP interface as the default network interface
 * (used to output all packets for which no specific route is found).
 */
// #define ppp_set_default(ppp)         netif_set_default(ppp.netif)

/*
 * Set a PPP notify phase callback.
 *
 * This can be used for example to set a LED pattern depending on the
 * current phase of the PPP session.
 */
type ppp_notify_phase_cb_fn = fn(pcb: &mut PppCtx, phase: u8, ctx: &mut Vec<u8>);
// pub fn  ppp_set_notify_phase_callback(pcb: &mut ppp_pcb, ppp_notify_phase_cb_fn notify_phase_cb);

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
// pub fn  ppp_connect(pcb: &mut ppp_pcb, holdoff: u16);

/*
 * Listen for an incoming PPP connection.
 *
 * This can only be called if PPP is in the dead phase.
 *
 * If this port connects to a modem, the modem connection must be
 * established before calling this.
 */
// pub fn  ppp_listen(pcb: &mut ppp_pcb);

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
// pub fn  ppp_close(pcb: &mut ppp_pcb, nocarrier: u8);

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
// pub fn  ppp_free(pcb: &mut ppp_pcb);

/*
 * PPP IOCTL commands.
 *
 * Get the up status - 0 for down, non-zero for up.  The argument must
 * poto: i32 an int.
 */
// pub const PPPCTLG_UPSTATUS: u32 = 0;

/*
 * Get the PPP error code.  The argument must poto: i32 an int.
 * Returns a PPPERR_* value.
 */
pub const PPPCTLG_ERRCODE: u32 = 1;

/*
 * Get the fd associated with a PPP over serial
 */
pub const PPPCTLG_FD: u32 = 2;

/*
 * Get and set parameters for the given connection.
 * Return 0 on success, an error code on failure.
 */
// pub fn  ppp_ioctl(pcb: &mut ppp_pcb, cmd: u8, arg: &mut Vec<u8>);

//  Get the PPP netif interface 
// #define ppp_netif(ppp)               (ppp.netif)

//  Set an lwIP-style status-callback for the selected PPP device 
// #define ppp_set_netif_statuscallback(ppp, status_cb)       \
//         netif_set_status_callback(ppp.netif, status_cb);

//  Set an lwIP-style link-callback for the selected PPP device 
// #define ppp_set_netif_linkcallback(ppp, link_cb)           \
//         netif_set_link_callback(ppp.netif, link_cb);
