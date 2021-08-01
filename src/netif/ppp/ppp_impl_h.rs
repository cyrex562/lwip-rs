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

// #define LWIP_HDR_PPP_IMPL_H

























/*
 * Memory used for control packets.
 *
 * PPP_CTRL_PBUF_MAX_SIZE is the amount of memory we allocate when we
 * cannot figure out how much we are going to use before filling the buffer.
 */

#define PPP_CTRL_PBUF_TYPE       PBUF_RAM
#define PPP_CTRL_PBUF_MAX_SIZE   512
#else /* PPP_USE_PBUF_RAM */
#define PPP_CTRL_PBUF_TYPE       PBUF_POOL
#define PPP_CTRL_PBUF_MAX_SIZE   PBUF_POOL_BUFSIZE


/*
 * The basic PPP frame.
 */
#define PPP_ADDRESS(p)	(((u_char *)(p))[0])
#define PPP_CONTROL(p)	(((u_char *)(p))[1])
#define PPP_PROTOCOL(p)	((((u_char *)(p))[2] << 8) + ((u_char *)(p))[3])

/*
 * Significant octet values.
 */
pub const PPP_ALLSTATIONS: u32 = 0xff;	/* All-Stations broadcast address */pub const PPP_ALLSTATIONS: u32 = 0xff;pub const PPP_ALLSTATIONS: u32 = 0xff;pub const PPP_ALLSTATIONS: u32 = 0xff;pub const PPP_ALLSTATIONS: u32 = 0xff;
#define	PPP_UI		0x03	/* Unnumbered Information */
#define	PPP_FLAG	0x7e	/* Flag Sequence */
#define	PPP_ESCAPE	0x7d	/* Asynchronous Control Escape */
#define	PPP_TRANS	0x20	/* Asynchronous transparency modifier */

/*
 * Protocol field values.
 */
pub const PPP_IP: u32 = 0x21;	/* Internet Protocol */

pub const PPP_AT: u32 = 0x29;	/* AppleTalk Protocol */pub const PPP_AT: u32 = 0x29;
#define PPP_IPX		0x2b	/* IPX protocol */


pub const PPP_VJC_COMP: u32 = 0x2d;	/* VJ compressed TCP */pub const PPP_VJC_COMP: u32 = 0x2d;
#define	PPP_VJC_UNCOMP	0x2f	/* VJ uncompressed TCP */


pub const PPP_IPV6: u32 = 0x57;	/* Internet Protocol Version 6 */


pub const PPP_COMP: u32 = 0xfd;	/* compressed packet */

pub const PPP_IPCP: u32 = 0x8021;	/* IP Control Protocol */

pub const PPP_ATCP: u32 = 0x8029;	/* AppleTalk Control Protocol */pub const PPP_ATCP: u32 = 0x8029;
#define PPP_IPXCP	0x802b	/* IPX Control Protocol */


pub const PPP_IPV6CP: u32 = 0x8057;	/* IPv6 Control Protocol */


pub const PPP_CCP: u32 = 0x80fd;	/* Compression Control Protocol */


pub const PPP_ECP: u32 = 0x8053;	/* Encryption Control Protocol */

pub const PPP_LCP: u32 = 0xc021;	/* Link Control Protocol */

pub const PPP_PAP: u32 = 0xc023;	/* Password Authentication Protocol */


pub const PPP_LQR: u32 = 0xc025;	/* Link Quality Report protocol */


pub const PPP_CHAP: u32 = 0xc223;	/* Cryptographic Handshake Auth. Protocol */


pub const PPP_CBCP: u32 = 0xc029;	/* Callback Control Protocol */


pub const PPP_EAP: u32 = 0xc227;	/* Extensible Authentication Protocol */


/*
 * The following struct gives the addresses of procedures to call
 * for a particular lower link level protocol.
 */
struct link_callbacks {
  /* Start a connection (e.g. Initiate discovery phase) */
  void (*connect) (ppp_pcb *pcb, void *ctx);

  /* Listen for an incoming connection (Passive mode) */
  void (*listen) (ppp_pcb *pcb, void *ctx);

  /* End a connection (i.e. initiate disconnect phase) */
  void (*disconnect) (ppp_pcb *pcb, void *ctx);
  /* Free lower protocol control block */
  err_t (*free) (ppp_pcb *pcb, void *ctx);
  /* Write a pbuf to a ppp link, only used from PPP functions to send PPP packets. */
  err_t (*write)(ppp_pcb *pcb, void *ctx, p: &mut pbuf);
  /* Send a packet from lwIP core (IPv4 or IPv6) */
  err_t (*netif_output)(ppp_pcb *pcb, void *ctx, p: &mut pbuf, u_short protocol);
  /* configure the transmit-side characteristics of the PPP interface */
  void (*send_config)(ppp_pcb *pcb, void *ctx, accm: u32, pcomp: int, accomp: int);
  /* confire the receive-side characteristics of the PPP interface */
  void (*recv_config)(ppp_pcb *pcb, void *ctx, accm: u32, pcomp: int, accomp: int);
};

/*
 * What to do with network protocol (NP) packets.
 */
enum NPmode {
    NPMODE_PASS,		/* pass the packet through */
    NPMODE_DROP,		/* silently drop the packet */
    NPMODE_ERROR,		/* return an error */
    NPMODE_QUEUE		/* save it up for later. */
};

/*
 * Statistics.
 */

struct pppstat	{
    unsigned ppp_ibytes: int;	/* bytes received */
    unsigned ppp_ipackets: int;	/* packets received */
    unsigned ppp_ierrors: int;	/* receive errors */
    unsigned ppp_obytes: int;	/* bytes sent */
    unsigned ppp_opackets: int;	/* packets sent */
    unsigned ppp_oerrors: int;	/* transmit errors */
};


struct vjstat {
    unsigned vjs_packets: int;	/* outbound packets */
    unsigned vjs_compressed: int; /* outbound compressed packets */
    unsigned vjs_searches: int;	/* searches for connection state */
    unsigned vjs_misses: int;	/* times couldn't find conn. state */
    unsigned vjs_uncompressedin: int; /* inbound uncompressed packets */
    unsigned vjs_compressedin: int; /* inbound compressed packets */
    unsigned vjs_errorin: int;	/* inbound unknown type packets */
    unsigned vjs_tossed: int;	/* inbound packets tossed because of error */
};


struct ppp_stats {
    struct pppstat p;		/* basic PPP statistics */

    struct vjstat vj;		/* VJ header compression statistics */

};


struct compstat {
    unsigned unc_bytes: int;	/* total uncompressed bytes */
    unsigned unc_packets: int;	/* total uncompressed packets */
    unsigned comp_bytes: int;	/* compressed bytes */
    unsigned comp_packets: int;	/* compressed packets */
    unsigned inc_bytes: int;	/* incompressible bytes */
    unsigned inc_packets: int;	/* incompressible packets */
    unsigned ratio: int;		/* recent compression ratio << 8 */
};

struct ppp_comp_stats {
    struct compstat c;		/* packet compression statistics */
    struct compstat d;		/* packet decompression statistics */
};





/*
 * The following structure records the time in seconds since
 * the last NP packet was sent or received.
 */
struct ppp_idle {
    time_t xmit_idle;		/* time since last NP packet sent */
    time_t recv_idle;		/* time since last NP packet received */
};


/* values for epdisc.class */
#define EPD_NULL	0	/* null discriminator, no data */
#define EPD_LOCAL	1
#define EPD_IP		2
#define EPD_MAC		3
#define EPD_MAGIC	4
#define EPD_PHONENUM	5

/*
 * Global variables.
 */

extern u8	multilink;	/* enable multilink operation */
extern u8	doing_multilink;
extern u8	multilink_master;
extern u8	bundle_eof;
extern u8	bundle_terminating;



extern unsigned maxoctets: int;	     /* Maximum octetes per session (in bytes) */
extern int       maxoctets_dir;      /* Direction :
				      0 - in+out (default)
				      1 - in
				      2 - out
				      3 - max(in,out) */
extern int       maxoctets_timeout;  /* Timeout for check of octets limit */
pub const PPP_OCTETS_DIRECTION_SUM: u32 = 0;
#define PPP_OCTETS_DIRECTION_IN         1
#define PPP_OCTETS_DIRECTION_OUT        2
#define PPP_OCTETS_DIRECTION_MAXOVERAL  3
/* same as previos, but little different on RADIUS side */
#define PPP_OCTETS_DIRECTION_MAXSESSION 4


/* Data input may be used by CCP and ECP, remove this entry
 * from struct protent to save some flash
 */
pub const PPP_DATAINPUT: u32 = 0;

/*
 * The following struct gives the addresses of procedures to call
 * for a particular protocol.
 */
struct protent {
    u_short protocol;		/* PPP protocol number */
    /* Initialization procedure */
    void (*init) (ppp_pcb *pcb);
    /* Process a received packet */
    void (*input) (ppp_pcb *pcb, u_char *pkt, len: int);
    /* Process a received protocol-reject */
    void (*protrej) (ppp_pcb *pcb);
    /* Lower layer has come up */
    void (*lowerup) (ppp_pcb *pcb);
    /* Lower layer has gone down */
    void (*lowerdown) (ppp_pcb *pcb);
    /* Open the protocol */
    void (*open) (ppp_pcb *pcb);
    /* Close the protocol */
    void (*close) (ppp_pcb *pcb, const char *reason);

    /* Pra: int packet in readable form */
    int  (*printpkt) (const u_char *pkt, len: int,
			  void (*printer) (void *, const char *, ...),
			  arg: &mut Vec<u8>);


    /* Process a received data packet */
    void (*datainput) (ppp_pcb *pcb, u_char *pkt, len: int);


    name: String;		/* Text name of protocol */
    data_name: String;	/* Text name of corresponding data protocol */


    option_t *options;		/* List of command-line options */
    /* Check requested options, assign defaults */
    void (*check_options) ();


    /* Configure interface for demand-dial */
    int  (*demand_conf) (unit: int);
    /* Say whether to bring up link for this pkt */
    int  (*active_pkt) (u_char *pkt, len: int);

};

/* Table of pointers to supported protocols */
extern const struct protent* const protocols[];


/* Values for auth_pending, auth_done */

pub const PAP_WITHPEER: u32 = 0x1;pub const PAP_WITHPEER: u32 = 0x1;
#define PAP_PEER	0x2


pub const CHAP_WITHPEER: u32 = 0x4;pub const CHAP_WITHPEER: u32 = 0x4;
#define CHAP_PEER	0x8


pub const EAP_WITHPEER: u32 = 0x10;pub const EAP_WITHPEER: u32 = 0x10;
#define EAP_PEER	0x20


/* Values for auth_done only */

pub const CHAP_MD5_WITHPEER: u32 = 0x40;pub const CHAP_MD5_WITHPEER: u32 = 0x40;
#define CHAP_MD5_PEER		0x80

#define CHAP_MS_SHIFT		8	/* LSB position for MS auths */
pub const CHAP_MS_WITHPEER: u32 = 0x100;pub const CHAP_MS_WITHPEER: u32 = 0x100;pub const CHAP_MS_WITHPEER: u32 = 0x100;pub const CHAP_MS_WITHPEER: u32 = 0x100;
#define CHAP_MS_PEER		0x200
#define CHAP_MS2_WITHPEER	0x400
#define CHAP_MS2_PEER		0x800



/* Supported CHAP protocols */



#define CHAP_MDTYPE_SUPPORTED (MDTYPE_MICROSOFT_V2 | MDTYPE_MICROSOFT | MDTYPE_MD5)
#else /* MSCHAP_SUPPORT */
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_MD5)


#else /* CHAP_SUPPORT */
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_NONE)



/*
 * PPP statistics structure
 */
struct pppd_stats {
    unsigned int	bytes_in;
    unsigned int	bytes_out;
    unsigned int	pkts_in;
    unsigned int	pkts_out;
};



/*
 * PPP private functions
 */

 
/*
 * Functions called from lwIP core.
 */

/* initialize the PPP subsystem */
ppp_init: int();

/*
 * Functions called from PPP link protocols.
 */

/* Create a new PPP control block */
ppp_pcb *ppp_new(pppif: &mut netif, const callbacks: &mut link_callbacks, void *link_ctx_cb,
                 ppp_link_status_cb_fn link_status_cb, void *ctx_cb);

/* Initiate LCP open request */
pub fn  ppp_start(ppp_pcb *pcb);

/* Called when link failed to setup */
pub fn  ppp_link_failed(ppp_pcb *pcb);

/* Called when link is normally down (i.e. it was asked to end) */
pub fn  ppp_link_end(ppp_pcb *pcb);

/* function called to process input packet */
pub fn  ppp_input(ppp_pcb *pcb, pb: &mut pbuf);


/*
 * Functions called by PPP protocols.
 */

/* function called by all PPP subsystems to send packets */
pub fn  ppp_write(ppp_pcb *pcb, p: &mut pbuf);

/* functions called by auth.c link_terminated() */
pub fn  ppp_link_terminated(ppp_pcb *pcb);

pub fn  new_phase(ppp_pcb *pcb, p: int);

ppp_send_config: int(ppp_pcb *pcb, mtu: int, accm: u32, pcomp: int, accomp: int);
ppp_recv_config: int(ppp_pcb *pcb, mru: int, accm: u32, pcomp: int, accomp: int);


sifaddr: int(ppp_pcb *pcb, our_adr: u32, his_adr: u32, netmask: u32);
cifaddr: int(ppp_pcb *pcb, our_adr: u32, his_adr: u32);

sifproxyarp: int(ppp_pcb *pcb, his_adr: u32);
cifproxyarp: int(ppp_pcb *pcb, his_adr: u32);


sdns: int(ppp_pcb *pcb, ns1: u32, ns2: u32);
cdns: int(ppp_pcb *pcb, ns1: u32, ns2: u32);


sifvjcomp: int(ppp_pcb *pcb, vjcomp: int, cidcomp: int, maxcid: int);

sifup: int(ppp_pcb *pcb);
sifdown: int (ppp_pcb *pcb);
get_mask: u32(addr: u32);



sif6addr: int(ppp_pcb *pcb, eui64_t our_eui64, eui64_t his_eui64);
cif6addr: int(ppp_pcb *pcb, eui64_t our_eui64, eui64_t his_eui64);
sif6up: int(ppp_pcb *pcb);
sif6down: int (ppp_pcb *pcb);



sifnpmode: int(ppp_pcb *pcb, proto: int, enum NPmode mode);


pub fn  netif_set_mtu(ppp_pcb *pcb, mtu: int);
netif_get_mtu: int(ppp_pcb *pcb);



ccp_test: int(ppp_pcb *pcb, u_char *opt_ptr, opt_len: int, for_transmit: int);

pub fn  ccp_set(ppp_pcb *pcb, isopen: u8, isup: u8, receive_method: u8, transmit_method: u8);
pub fn  ccp_reset_comp(ppp_pcb *pcb);
pub fn  ccp_reset_decomp(ppp_pcb *pcb);

ccp_fatal_error: int(ppp_pcb *pcb);




get_idle_time: int(ppp_pcb *pcb, ip: &mut ppp_idle);



get_loop_output: int();


/* Optional protocol names list, to make our messages a little more informative. */

const char * protocol_name(proto: int);


/* Optional stats support, to get some statistics on the PPP interface */

pub fn  print_link_stats(); /* Prstats: int, if available */
pub fn  reset_link_stats(u: int); /* Reset (init) stats when link goes up */
pub fn  update_link_stats(u: int); /* Get stats at link termination */




/*
 * Inline versions of get/put char/short/long.
 * Pointer is advanced; we assume that both arguments
 * are lvalues and will already be in registers.
 * cp MUST be u_char *.
 */
#define GETCHAR(c, cp) { \
	(c) = *(cp)++; \
}
#define PUTCHAR(c, cp) { \
	*(cp)++ = (u_char) (c); \
}
#define GETSHORT(s, cp) { \
	(s) = *(cp)++ << 8; \
	(s) |= *(cp)++; \
}
#define PUTSHORT(s, cp) { \
	*(cp)++ = (u_char) ((s) >> 8); \
	*(cp)++ = (u_char) (s); \
}
#define GETLONG(l, cp) { \
	(l) = *(cp)++ << 8; \
	(l) |= *(cp)++; (l) <<= 8; \
	(l) |= *(cp)++; (l) <<= 8; \
	(l) |= *(cp)++; \
}
#define PUTLONG(l, cp) { \
	*(cp)++ = (u_char) ((l) >> 24); \
	*(cp)++ = (u_char) ((l) >> 16); \
	*(cp)++ = (u_char) ((l) >> 8); \
	*(cp)++ = (u_char) (l); \
}

#define INCPTR(n, cp)	((cp) += (n))
#define DECPTR(n, cp)	((cp) -= (n))

/*
 * System dependent definitions for user-level 4.3BSD UNIX implementation.
 */
#define TIMEOUT(f, a, t)        do { sys_untimeout((f), (a)); sys_timeout((t)*1000, (f), (a)); } while(0)
#define TIMEOUTMS(f, a, t)      do { sys_untimeout((f), (a)); sys_timeout((t), (f), (a)); } while(0)
#define UNTIMEOUT(f, a)         sys_untimeout((f), (a))

#define BZERO(s, n)		memset(s, 0, n)
#define	BCMP(s1, s2, l)		memcmp(s1, s2, l)

#define PRINTMSG(m, l)		{ ppp_info("Remote message: %0.*v", l, m); }

/*
 * MAKEHEADER - Add Header fields to a packet.
 */
#define MAKEHEADER(p, t) { \
    PUTCHAR(PPP_ALLSTATIONS, p); \
    PUTCHAR(PPP_UI, p); \
    PUTSHORT(t, p); }

/* Procedures exported from auth.c */
pub fn  link_required(ppp_pcb *pcb);     /* we are starting to use the link */
pub fn  link_terminated(ppp_pcb *pcb);   /* we are finished with the link */
pub fn  link_down(ppp_pcb *pcb);	      /* the LCP layer has left the Opened state */
pub fn  upper_layers_down(ppp_pcb *pcb); /* take all NCPs down */
pub fn  link_established(ppp_pcb *pcb);  /* the link is up; authenticate now */
pub fn  start_networks(ppp_pcb *pcb);    /* start all the network control protos */
pub fn  continue_networks(ppp_pcb *pcb); /* start network [ip, etc] control protos */


auth_check_passwd: int(ppp_pcb *pcb, char *auser, userlen: int, char *apasswd, passwdlen: int, const char **msg, int *msglen);
                                /* check the user name and passwd against configuration */
pub fn  auth_peer_fail(ppp_pcb *pcb, protocol: int);
				/* peer failed to authenticate itself */
pub fn  auth_peer_success(ppp_pcb *pcb, protocol: int, prot_flavor: int, const char *name, namelen: int);
				/* peer successfully authenticated itself */

pub fn  auth_withpeer_fail(ppp_pcb *pcb, protocol: int);
				/* we failed to authenticate ourselves */
pub fn  auth_withpeer_success(ppp_pcb *pcb, protocol: int, prot_flavor: int);
				/* we successfully authenticated ourselves */

pub fn  np_up(ppp_pcb *pcb, proto: int);    /* a network protocol has come up */
pub fn  np_down(ppp_pcb *pcb, proto: int);  /* a network protocol has gone down */
pub fn  np_finished(ppp_pcb *pcb, proto: int); /* a network protocol no longer needs link */

get_secret: int(ppp_pcb *pcb, const char *client, const char *server, char *secret, int *secret_len, am_server: int);
				/* get "secret" for chap */


/* Procedures exported from ipcp.c */
/* parse_dotted_ip: int (char *, u32 *); */

/* Procedures exported from demand.c */

pub fn  demand_conf ();	/* config interface(s) for demand-dial */
pub fn  demand_block ();	/* set all NPs to queue up packets */
pub fn  demand_unblock (); /* set all NPs to pass packets */
pub fn  demand_discard (); /* set all NPs to discard packets */
pub fn  demand_rexmit (int, u32); /* retransmit saved frames for an NP*/
int  loop_chars (unsigned char *, int); /* process chars from loopback */
int  loop_frame (unsigned char *, int); /* should we bring link up? */


/* Procedures exported from multilink.c */

pub fn  mp_check_options (); /* Check multilink-related options */
int  mp_join_bundle ();  /* join our link to an appropriate bundle */
pub fn  mp_exit_bundle ();  /* have disconnected our link from bundle */
pub fn  mp_bundle_terminated ();
char *epdisc_to_str (struct epdisc *); /* string from endpodiscrim: int. */
int  str_to_epdisc (struct epdisc *, char *); /* endpt disc. from str */
#else
#define mp_bundle_terminated()	/* nothing */
#define mp_exit_bundle()	/* nothing */
pub const doing_multilink: u32 = 0;pub const doing_multilink: u32 = 0;
#define multilink_master	0


/* Procedures exported from utils.c. */
pub fn  ppp_print_string(const u_char *p, len: int, void (*printer) (void *, const char *, ...), arg: &mut Vec<u8>);   /* Format a string for output */
ppp_slprintf: int(char *buf, buflen: int, const char *fmt, ...);            /* sprintf++ */
ppp_vslprintf: int(char *buf, buflen: int, const char *fmt, va_list args);  /* vsprintf++ */
usize ppp_strlcpy(char *dest, const char *src, usize len);        /* safe strcpy */
usize ppp_strlcat(char *dest, const char *src, usize len);        /* safe strncpy */
pub fn  ppp_dbglog(const char *fmt, ...);    /* log a debug message */
pub fn  ppp_info(const char *fmt, ...);      /* log an informational message */
pub fn  ppp_notice(const char *fmt, ...);    /* log a notice-level message */
pub fn  ppp_warn(const char *fmt, ...);      /* log a warning message */
pub fn  ppp_error(const char *fmt, ...);     /* log an error message */
pub fn  ppp_fatal(const char *fmt, ...);     /* log an error message and die(1) */

pub fn  ppp_dump_packet(ppp_pcb *pcb, const char *tag, unsigned char *p, len: int);
                                /* dump packet to debug log if interesting */


/*
 * Number of necessary timers analysis.
 *
 * PPP use at least one timer per each of its protocol, but not all protocols are
 * active at the same time, thus the number of necessary timeouts is actually
 * lower than enabled protocols. Here is the actual necessary timeouts based
 * on code analysis.
 *
 * Note that many features analysed here are not working at all and are only
 * there for a comprehensive analysis of necessary timers in order to prevent
 * having to redo that each time we add a feature.
 *
 * Timer list
 *
 * | holdoff timeout
 *  | low level protocol timeout (PPPoE or PPPoL2P)
 *   | LCP delayed UP
 *    | LCP retransmit (FSM)
 *     | LCP Echo timer
 *     .| PAP or CHAP or EAP authentication
 *     . | ECP retransmit (FSM)
 *     .  | CCP retransmit (FSM) when MPPE is enabled
 *     .   | CCP retransmit (FSM) when MPPE is NOT enabled
 *     .    | IPCP retransmit (FSM)
 *     .    .| IP6CP retransmit (FSM)
 *     .    . | Idle time limit
 *     .    .  | Max connect time
 *     .    .   | Max octets
 *     .    .    | CCP RACK timeout
 *     .    .    .
 * PPP_PHASE_DEAD
 * PPP_PHASE_HOLDOFF
 * |   .    .    .
 * PPP_PHASE_INITIALIZE
 *  |  .    .    .
 * PPP_PHASE_ESTABLISH
 *   | .    .    .
 *    |.    .    .
 *     |    .    .
 * PPP_PHASE_AUTHENTICATE
 *     |    .    .
 *     ||   .    .
 * PPP_PHASE_NETWORK
 *     | || .    .
 *     |   |||   .
 * PPP_PHASE_RUNNING
 *     |    .|||||
 *     |    . ||||
 * PPP_PHASE_TERMINATE
 *     |    . ||||
 * PPP_PHASE_NETWORK
 *    |.         .
 * PPP_PHASE_ESTABLISH
 * PPP_PHASE_DISCONNECT
 * PPP_PHASE_DEAD
 *
 * Alright, PPP basic retransmission and LCP Echo consume one timer.
 *  1
 *
 * If authentication is enabled one timer is necessary during authentication.
 *  1 + PPP_AUTH_SUPPORT
 *
 * If ECP is enabled one timer is necessary before IPCP and/or IP6CP, one more
 * is necessary if CCP is enabled (only with MPPE support but we don't care much
 * up to this detail level).
 *  1 + ECP_SUPPORT + CCP_SUPPORT
 *
 * If CCP is enabled it might consume a timer during IPCP or IP6CP, thus
 * we might use IPCP, IP6CP and CCP timers simultaneously.
 *  1 + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT
 *
 * When entering running phase, IPCP or IP6CP is still running. If idle time limit
 * is enabled one more timer is necessary. Same for max connect time and max
 * octets features. Furthermore CCP RACK might be used past this point.
 *  1 + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT -1 + PPP_IDLETIMELIMIT + PPP_MAXCONNECT + MAXOCTETS + CCP_SUPPORT
 *
 * IPv4 or IPv6 must be enabled, therefore we don't need to take care the authentication
 * and the CCP + ECP case, thus reducing overall complexity.
 * 1 + LWIP_MAX(PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT, PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT -1 + PPP_IDLETIMELIMIT + PPP_MAXCONNECT + MAXOCTETS + CCP_SUPPORT)
 *
 * We don't support PPP_IDLETIMELIMIT + PPP_MAXCONNECT + MAXOCTETS features
 * and adding those defines to ppp_opts.h just for having the value always
 * defined to 0 isn't worth it.
 * 1 + LWIP_MAX(PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT, PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT -1 + CCP_SUPPORT)
 *
 * Thus, the following is enough for now.
 * 1 + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT
 */


}




