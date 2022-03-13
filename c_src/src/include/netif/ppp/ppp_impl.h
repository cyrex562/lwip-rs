/*****************************************************************************
* ppp.h - Network Point to Point Protocol header file.
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



// #include "netif/ppp/ppp_opts.h"

// #if PPP_SUPPORT /* don't build if not configured for use in lwipopts.h */

#ifdef PPP_INCLUDE_SETTINGS_HEADER
// #include "ppp_settings.h"







// #include "lwip/netif.h"
// #include "lwip/def.h"
// #include "lwip/timeouts.h"

// #include "ppp.h"
// #include "pppdebug.h"




/*
 * Memory used for control packets.
 *
 * PPP_CTRL_PBUF_UNKNOWN_SIZE is the amount of memory we allocate when we
 * cannot figure out how much we are going to use before filling the buffer.
 */
pub const PPP_CTRL_PBUF_UNKNOWN_SIZE: u32 = 512; /*
 * The basic PPP frame.
 */
#define PPP_ADDRESS(p)	(((u_char *)(p))[0])
#define PPP_CONTROL(p)	(((u_char *)(p))[1])
#define PPP_PROTOCOL(p)	((((u_char *)(p))[2] << 8) + ((u_char *)(p))[3])

/*
 * Significant octet values.
 */
pub const PPP_ALLSTATIONS: u32 = 0xff; /* All-Stations broadcast address */
pub const PPP_UI: u32 = 0x03; /* Unnumbered Information */
pub const PPP_FLAG: u32 = 0x7e; /* Flag Sequence */
pub const PPP_ESCAPE: u32 = 0x7d; /* Asynchronous Control Escape */
pub const PPP_TRANS: u32 = 0x20; /* Asynchronous transparency modifier */

/*
 * PPP_DEFMRU: MRU value used prior negotiation and unless negotiated later.
 * Must be 1500.
 */
pub const PPP_DEFMRU: u32 = 1500; /*
 * Protocol field values.
 */
// #if PPP_IPV4_SUPPORT
pub const PPP_IP: u32 = 0x21; /* Internet Protocol */
 /* PPP_IPV4_SUPPORT */
/* UNUSED */
pub const PPP_AT: u32 = 0x29; /* AppleTalk Protocol */
pub const PPP_IPX: u32 = 0x2b; /* IPX protocol */
 /* UNUSED */
_SUPPORT
pub const PPP_VJC_COMP: u32 = 0x2d; /* VJ compressed TCP */
pub const PPP_VJC_UNCOMP: u32 = 0x2f; /* VJ uncompressed TCP */
 /* VJ_SUPPORT */
P_IPV6_SUPPORT
pub const PPP_IPV6: u32 = 0x57; /* Internet Protocol Version 6 */
 /* PPP_IPV6_SUPPORT */
P_SUPPORT
pub const PPP_COMP: u32 = 0xfd; /* compressed packet */
 /* CCP_SUPPORT */
e PPP_IPCP	0x8021	/* IP Control Protocol */
// #if 0 /* UNUSED */
pub const PPP_ATCP: u32 = 0x8029; /* AppleTalk Control Protocol */
pub const PPP_IPXCP: u32 = 0x802b; /* IPX Control Protocol */
 /* UNUSED */
P_IPV6_SUPPORT
pub const PPP_IPV6CP: u32 = 0x8057; /* IPv6 Control Protocol */
 /* PPP_IPV6_SUPPORT */
P_SUPPORT
pub const PPP_CCP: u32 = 0x80fd; /* Compression Control Protocol */
 /* CCP_SUPPORT */
P_SUPPORT
pub const PPP_ECP: u32 = 0x8053; /* Encryption Control Protocol */
 /* ECP_SUPPORT */
e PPP_LCP		0xc021	/* Link Control Protocol */
// #if PAP_SUPPORT
pub const PPP_PAP: u32 = 0xc023; /* Password Authentication Protocol */
 /* PAP_SUPPORT */
R_SUPPORT
pub const PPP_LQR: u32 = 0xc025; /* Link Quality Report protocol */
 /* LQR_SUPPORT */
AP_SUPPORT
pub const PPP_CHAP: u32 = 0xc223; /* Cryptographic Handshake Auth. Protocol */
 /* CHAP_SUPPORT */
CP_SUPPORT
pub const PPP_CBCP: u32 = 0xc029; /* Callback Control Protocol */
 /* CBCP_SUPPORT */
P_SUPPORT
pub const PPP_EAP: u32 = 0xc227; /* Extensible Authentication Protocol */
 /* EAP_SUPPORT */

/*
 * The following struct gives the addresses of procedures to call
 * for a particular lower link level protocol.
 */
struct link_callbacks {
  /* Start a connection (e.g. Initiate discovery phase) */
  void (*connect) (ppp_pcb *pcb, void *ctx);
// #if PPP_SERVER
  /* Listen for an incoming connection (Passive mode) */
  void (*listen) (ppp_pcb *pcb, void *ctx);
 /* PPP_SERVER */
nd a connection (i.e. initiate disconnect phase) */
  void (*disconnect) (ppp_pcb *pcb, void *ctx);
  /* Free lower protocol control block */
  err_t (*free) (ppp_pcb *pcb, void *ctx);
  /* Write a pbuf to a ppp link, only used from PPP functions to send PPP packets. */
  err_t (*write)(ppp_pcb *pcb, void *ctx, struct pbuf *p);
  /* Send a packet from lwIP core (IPv4 or IPv6) */
  err_t (*netif_output)(ppp_pcb *pcb, void *ctx, struct pbuf *p, u_short protocol);
  /* configure the transmit-side characteristics of the PPP interface */
  void (*send_config)(ppp_pcb *pcb, void *ctx, u32_t accm, int pcomp, int accomp);
  /* confire the receive-side characteristics of the PPP interface */
  void (*recv_config)(ppp_pcb *pcb, void *ctx, u32_t accm, int pcomp, int accomp);
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
// #if PPP_STATS_SUPPORT
struct pppstat	{
    unsigned int ppp_ibytes;	/* bytes received */
    unsigned int ppp_ipackets;	/* packets received */
    unsigned int ppp_ierrors;	/* receive errors */
    unsigned int ppp_obytes;	/* bytes sent */
    unsigned int ppp_opackets;	/* packets sent */
    unsigned int ppp_oerrors;	/* transmit errors */
};

// #if VJ_SUPPORT
struct vjstat {
    unsigned int vjs_packets;	/* outbound packets */
    unsigned int vjs_compressed; /* outbound compressed packets */
    unsigned int vjs_searches;	/* searches for connection state */
    unsigned int vjs_misses;	/* times couldn't find conn. state */
    unsigned int vjs_uncompressedin; /* inbound uncompressed packets */
    unsigned int vjs_compressedin; /* inbound compressed packets */
    unsigned int vjs_errorin;	/* inbound unknown type packets */
    unsigned int vjs_tossed;	/* inbound packets tossed because of error */
};
 /* VJ_SUPPORT */

struct ppp_stats {
    struct pppstat p;		/* basic PPP statistics */
// #if VJ_SUPPORT
    struct vjstat vj;		/* VJ header compression statistics */
 /* VJ_SUPPORT */


// #if CCP_SUPPORT
struct compstat {
    unsigned int unc_bytes;	/* total uncompressed bytes */
    unsigned int unc_packets;	/* total uncompressed packets */
    unsigned int comp_bytes;	/* compressed bytes */
    unsigned int comp_packets;	/* compressed packets */
    unsigned int inc_bytes;	/* incompressible bytes */
    unsigned int inc_packets;	/* incompressible packets */
    unsigned int ratio;		/* recent compression ratio << 8 */
};

struct ppp_comp_stats {
    struct compstat c;		/* packet compression statistics */
    struct compstat d;		/* packet decompression statistics */
};
 /* CCP_SUPPORT */

 /* PPP_STATS_SUPPORT */

// #if PPP_IDLETIMELIMIT
/*
 * The following structure records the time in seconds since
 * the last NP packet was sent or received.
 */
struct ppp_idle {
    time_t xmit_idle;		/* time since last NP packet sent */
    time_t recv_idle;		/* time since last NP packet received */
};
 /* PPP_IDLETIMELIMIT */

/* values for epdisc.class */
pub const EPD_NULL: u32 = 0; /* null discriminator, no data */
pub const EPD_LOCAL: u32 = 1; #define EPD_IP		2
pub const EPD_MAC: u32 = 3; #define EPD_MAGIC	4
pub const EPD_PHONENUM: u32 = 5; /*
 * Global variables.
 */
#ifdef HAVE_MULTILINK
extern multilink: u8;	/* enable multilink operation */
extern doing_multilink: u8;
extern multilink_master: u8;
extern bundle_eof: u8;
extern bundle_terminating: u8;


#ifdef MAXOCTETS
extern unsigned int maxoctets;	     /* Maximum octetes per session (in bytes) */
extern int       maxoctets_dir;      /* Direction :
				      0 - in+out (default)
				      1 - in
				      2 - out
				      3 - max(in,out) */
extern int       maxoctets_timeout;  /* Timeout for check of octets limit */
pub const PPP_OCTETS_DIRECTION_SUM: u32 = 0; #define PPP_OCTETS_DIRECTION_IN         1
pub const PPP_OCTETS_DIRECTION_OUT: u32 = 2; #define PPP_OCTETS_DIRECTION_MAXOVERAL  3
/* same as previous, but little different on RADIUS side */
pub const PPP_OCTETS_DIRECTION_MAXSESSION: u32 = 4; /* Data input may be used by CCP and ECP, remove this entry
 * from struct protent to save some flash
 */
pub const PPP_DATAINPUT: u32 = 0; /*
 * The following struct gives the addresses of procedures to call
 * for a particular protocol.
 */
struct protent {
    u_short protocol;		/* PPP protocol number */
    /* Initialization procedure */
    void (*init) (ppp_pcb *pcb);
    /* Process a received packet */
    void (*input) (ppp_pcb *pcb, u_char *pkt, int len);
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
// #if PRINTPKT_SUPPORT
    /* Print a packet in readable form */
    int  (*printpkt) (const u_char *pkt, int len,
			  void (*printer) (void *, const char *, ...),
			  void *arg);
 /* PRINTPKT_SUPPORT */
P_DATAINPUT
    /* Process a received data packet */
    void (*datainput) (ppp_pcb *pcb, u_char *pkt, int len);
 /* PPP_DATAINPUT */
INTPKT_SUPPORT
    const char *name;		/* Text name of protocol */
    const char *data_name;	/* Text name of corresponding data protocol */
 /* PRINTPKT_SUPPORT */
P_OPTIONS
    option_t *options;		/* List of command-line options */
    /* Check requested options, assign defaults */
    void (*check_options) ();
 /* PPP_OPTIONS */
MAND_SUPPORT
    /* Configure interface for demand-dial */
    int  (*demand_conf) (int unit);
    /* Say whether to bring up link for this pkt */
    int  (*active_pkt) (u_char *pkt, int len);
 /* DEMAND_SUPPORT */


/* Table of pointers to supported protocols */
extern const struct protent* const protocols[];


/* Values for auth_pending, auth_done */
// #if PAP_SUPPORT
pub const PAP_WITHPEER: u32 = 0x1; #define PAP_PEER	0x2
 /* PAP_SUPPORT */
AP_SUPPORT
pub const CHAP_WITHPEER: u32 = 0x4; #define CHAP_PEER	0x8
 /* CHAP_SUPPORT */
P_SUPPORT
pub const EAP_WITHPEER: u32 = 0x10; #define EAP_PEER	0x20
 /* EAP_SUPPORT */

/* Values for auth_done only */
// #if CHAP_SUPPORT
pub const CHAP_MD5_WITHPEER: u32 = 0x40; #define CHAP_MD5_PEER		0x80
// #if MSCHAP_SUPPORT
pub const CHAP_MS_SHIFT: u32 = 8; /* LSB position for MS auths */
pub const CHAP_MS_WITHPEER: u32 = 0x100; #define CHAP_MS_PEER		0x200
pub const CHAP_MS2_WITHPEER: u32 = 0x400; #define CHAP_MS2_PEER		0x800
 /* MSCHAP_SUPPORT */
 /* CHAP_SUPPORT */

/* Supported CHAP protocols */
// #if CHAP_SUPPORT

// #if MSCHAP_SUPPORT
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_MICROSOFT_V2 | MDTYPE_MICROSOFT | MDTYPE_MD5)
#else /* MSCHAP_SUPPORT */
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_MD5)
 /* MSCHAP_SUPPORT */

#else /* CHAP_SUPPORT */
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_NONE)
 /* CHAP_SUPPORT */

// #if PPP_STATS_SUPPORT
/*
 * PPP statistics structure
 */
struct pppd_stats {
    unsigned int	bytes_in;
    unsigned int	bytes_out;
    unsigned int	pkts_in;
    unsigned int	pkts_out;
};
 /* PPP_STATS_SUPPORT */


/*
 * PPP private functions
 */

 
/*
 * Functions called from lwIP core.
 */

/* initialize the PPP subsystem */
int ppp_init();

/*
 * Functions called from PPP link protocols.
 */

/* Create a new PPP control block */
ppp_pcb *ppp_new(struct netif *pppif, const struct link_callbacks *callbacks, void *link_ctx_cb,
                 ppp_link_status_cb_fn link_status_cb, void *ctx_cb);

/* Initiate LCP open request */
void ppp_start(ppp_pcb *pcb);

/* Called when link failed to setup */
void ppp_link_failed(ppp_pcb *pcb);

/* Called when link is normally down (i.e. it was asked to end) */
void ppp_link_end(ppp_pcb *pcb);

/* function called to process input packet */
void ppp_input(ppp_pcb *pcb, struct pbuf *pb);


/*
 * Functions called by PPP protocols.
 */

/* function called by all PPP subsystems to send packets */
err_t ppp_write(ppp_pcb *pcb, struct pbuf *p);

/* functions called by auth.c link_terminated() */
void ppp_link_terminated(ppp_pcb *pcb);

void new_phase(ppp_pcb *pcb, int p);

int ppp_send_config(ppp_pcb *pcb, int mtu, u32_t accm, int pcomp, int accomp);
int ppp_recv_config(ppp_pcb *pcb, int mru, u32_t accm, int pcomp, int accomp);

// #if PPP_IPV4_SUPPORT
int sifaddr(ppp_pcb *pcb, u32_t our_adr, u32_t his_adr, u32_t netmask);
int cifaddr(ppp_pcb *pcb, u32_t our_adr, u32_t his_adr);
// #if 0 /* UNUSED - PROXY ARP */
int sifproxyarp(ppp_pcb *pcb, u32_t his_adr);
int cifproxyarp(ppp_pcb *pcb, u32_t his_adr);
 /* UNUSED - PROXY ARP */
IP_DNS
int sdns(ppp_pcb *pcb, u32_t ns1, u32_t ns2);
int cdns(ppp_pcb *pcb, u32_t ns1, u32_t ns2);
 /* LWIP_DNS */
_SUPPORT
int sifvjcomp(ppp_pcb *pcb, int vjcomp, int cidcomp, int maxcid);
 /* VJ_SUPPORT */
fup(ppp_pcb *pcb);
int sifdown (ppp_pcb *pcb);
u32_t get_mask(u32_t addr);
 /* PPP_IPV4_SUPPORT */

// #if PPP_IPV6_SUPPORT
int sif6addr(ppp_pcb *pcb, eui64_t our_eui64, eui64_t his_eui64);
int cif6addr(ppp_pcb *pcb, eui64_t our_eui64, eui64_t his_eui64);
int sif6up(ppp_pcb *pcb);
int sif6down (ppp_pcb *pcb);
 /* PPP_IPV6_SUPPORT */

// #if DEMAND_SUPPORT
int sifnpmode(ppp_pcb *pcb, int proto, enum NPmode mode);
 /* DEMAND_SUPPORt */

void ppp_netif_set_mtu(ppp_pcb *pcb, int mtu);
int ppp_netif_get_mtu(ppp_pcb *pcb);

// #if CCP_SUPPORT
// #if 0 /* unused */
int ccp_test(ppp_pcb *pcb, u_char *opt_ptr, int opt_len, int for_transmit);
 /* unused */
cp_set(ppp_pcb *pcb, u8_t isopen, u8_t isup, u8_t receive_method, u8_t transmit_method);
void ccp_reset_comp(ppp_pcb *pcb);
void ccp_reset_decomp(ppp_pcb *pcb);
// #if 0 /* unused */
int ccp_fatal_error(ppp_pcb *pcb);
 /* unused */
 /* CCP_SUPPORT */

// #if PPP_IDLETIMELIMIT
int get_idle_time(ppp_pcb *pcb, struct ppp_idle *ip);
 /* PPP_IDLETIMELIMIT */

// #if DEMAND_SUPPORT
int get_loop_output();
 /* DEMAND_SUPPORT */

/* Optional protocol names list, to make our messages a little more informative. */
// #if PPP_PROTOCOLNAME
const char * protocol_name(int proto);
 /* PPP_PROTOCOLNAME  */

/* Optional stats support, to get some statistics on the PPP interface */
// #if PPP_STATS_SUPPORT
void print_link_stats(); /* Print stats, if available */
void reset_link_stats(int u); /* Reset (init) stats when link goes up */
void update_link_stats(int u); /* Get stats at link termination */
 /* PPP_STATS_SUPPORT */



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

#define PRINTMSG(m, l)		{ ppp_info(("Remote message: %0.*v", l, m)); }

/*
 * MAKEHEADER - Add Header fields to a packet.
 */
#define MAKEHEADER(p, t) { \
    PUTCHAR(PPP_ALLSTATIONS, p); \
    PUTCHAR(PPP_UI, p); \
    PUTSHORT(t, p); }

/* Procedures exported from auth.c */
void link_required(ppp_pcb *pcb);     /* we are starting to use the link */
void link_terminated(ppp_pcb *pcb);   /* we are finished with the link */
void link_down(ppp_pcb *pcb);	      /* the LCP layer has left the Opened state */
void upper_layers_down(ppp_pcb *pcb); /* take all NCPs down */
void link_established(ppp_pcb *pcb);  /* the link is up; authenticate now */
void start_networks(ppp_pcb *pcb);    /* start all the network control protos */
void continue_networks(ppp_pcb *pcb); /* start network [ip, etc] control protos */
// #if PPP_AUTH_SUPPORT
// #if PPP_SERVER
int auth_check_passwd(ppp_pcb *pcb, char *auser, unsigned int userlen, char *apasswd, unsigned int passwdlen, const char **msg, int *msglen);
                                /* check the user name and passwd against configuration */
void auth_peer_fail(ppp_pcb *pcb, int protocol);
				/* peer failed to authenticate itself */
void auth_peer_success(ppp_pcb *pcb, int protocol, int prot_flavor, const char *name, int namelen);
				/* peer successfully authenticated itself */
 /* PPP_SERVER */
uth_withpeer_fail(ppp_pcb *pcb, int protocol);
				/* we failed to authenticate ourselves */
void auth_withpeer_success(ppp_pcb *pcb, int protocol, int prot_flavor);
				/* we successfully authenticated ourselves */
 /* PPP_AUTH_SUPPORT */
p_up(ppp_pcb *pcb, int proto);    /* a network protocol has come up */
void np_down(ppp_pcb *pcb, int proto);  /* a network protocol has gone down */
void np_finished(ppp_pcb *pcb, int proto); /* a network protocol no longer needs link */
// #if PPP_AUTH_SUPPORT
int get_secret(ppp_pcb *pcb, const char *client, const char *server, char *secret, int *secret_len, int am_server);
				/* get "secret" for chap */
 /* PPP_AUTH_SUPPORT */

/* Procedures exported from ipcp.c */
/* int parse_dotted_ip (char *, u32_t *); */

/* Procedures exported from demand.c */
// #if DEMAND_SUPPORT
void demand_conf ();	/* config interface(s) for demand-dial */
void demand_block ();	/* set all NPs to queue up packets */
void demand_unblock (); /* set all NPs to pass packets */
void demand_discard (); /* set all NPs to discard packets */
void demand_rexmit (int, u32_t); /* retransmit saved frames for an NP*/
int  loop_chars (unsigned char *, int); /* process chars from loopback */
int  loop_frame (unsigned char *, int); /* should we bring link up? */
 /* DEMAND_SUPPORT */

/* Procedures exported from multilink.c */
#ifdef HAVE_MULTILINK
void mp_check_options (); /* Check multilink-related options */
int  mp_join_bundle ();  /* join our link to an appropriate bundle */
void mp_exit_bundle ();  /* have disconnected our link from bundle */
void mp_bundle_terminated ();
char *epdisc_to_str (struct epdisc *); /* string from endpoint discrim. */
int  str_to_epdisc (struct epdisc *, char *); /* endpt disc. from str */
#else
#define mp_bundle_terminated()	/* nothing */
#define mp_exit_bundle()	/* nothing */
pub const doing_multilink: u32 = 0; #define multilink_master	0


/* Procedures exported from utils.c. */
void ppp_print_string(const u_char *p, int len, void (*printer) (void *, const char *, ...), void *arg);   /* Format a string for output */
int ppp_slprintf(char *buf, int buflen, const char *fmt, ...);            /* sprintf++ */
int ppp_vslprintf(char *buf, int buflen, const char *fmt, va_list args);  /* vsprintf++ */
size_t ppp_strlcpy(char *dest, const char *src, size_t len);        /* safe strcpy */
size_t ppp_strlcat(char *dest, const char *src, size_t len);        /* safe strncpy */
void ppp_dbglog_impl(const char *fmt, ...);    /* log a debug message */
void ppp_info_impl(const char *fmt, ...);      /* log an informational message */
void ppp_notice_impl(const char *fmt, ...);    /* log a notice-level message */
void ppp_warn_impl(const char *fmt, ...);      /* log a warning message */
void ppp_error_impl(const char *fmt, ...);     /* log an error message */
void ppp_fatal_impl(const char *fmt, ...);     /* log an error message and die(1) */
/* wrap all the above functions so they will only be linked when enabled */
#define ppp_dbglog(x) do { if (LWIP_DEBUG_ENABLED(LOG_DEBUG)) { ppp_dbglog_impl x; }} while(0)
#define ppp_info(x)   do { if (LWIP_DEBUG_ENABLED(LOG_INFO)) { ppp_info_impl x; }} while(0)
#define ppp_notice(x) do { if (LWIP_DEBUG_ENABLED(LOG_NOTICE)) { ppp_notice_impl x; }} while(0)
#define ppp_warn(x)   do { if (LWIP_DEBUG_ENABLED(LOG_WARNING)) { ppp_warn_impl x; }} while(0)
#define ppp_error(x)  do { if (LWIP_DEBUG_ENABLED(LOG_ERR)) { ppp_error_impl x; }} while(0)
#define ppp_fatal(x)  do { if (LWIP_DEBUG_ENABLED(LOG_CRITICAL)) { ppp_fatal_impl x; }} while(0)
// #if PRINTPKT_SUPPORT
void ppp_dump_packet(ppp_pcb *pcb, const char *tag, unsigned char *p, int len);
                                /* dump packet to debug log if interesting */
 /* PRINTPKT_SUPPORT */

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
 *     ||   .    .
 * PPP_PHASE_NETWORK
 *     |||| .    .
 *     ||  |||   .
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
 * This timer might still be running up to network phase for any necessary
 * rechallenge, mostly for PPP server support.
 *  1 + PPP_AUTH_SUPPORT
 *
 * If ECP is enabled one timer is necessary before IPCP and/or IP6CP, one more
 * is necessary if CCP is enabled (only with MPPE support but we don't care much
 * up to this detail level).
 *  1 + PPP_AUTH_SUPPORT + ECP_SUPPORT + CCP_SUPPORT
 *
 * If CCP is enabled it might consume a timer during IPCP or IP6CP, thus
 * we might use AUTH, IPCP, IP6CP and CCP timers simultaneously.
 *  1 + PPP_AUTH_SUPPORT + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT
 *
 * When entering running phase, IPCP or IP6CP is still running. If idle time limit
 * is enabled one more timer is necessary. Same for max connect time and max
 * octets features. Furthermore CCP RACK might be used past this point.
 *  1 + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT -1 + PPP_IDLETIMELIMIT + PPP_MAXCONNECT + MAXOCTETS + CCP_SUPPORT
 *
 * Then the maximum number of simultaneously running timers is given by:
 *  1 + MAX(PPP_AUTH_SUPPORT + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT,
 *          PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT -1 + PPP_IDLETIMELIMIT + PPP_MAXCONNECT + MAXOCTETS + CCP_SUPPORT)
 *
 * We don't support ECP_SUPPORT + PPP_IDLETIMELIMIT + PPP_MAXCONNECT + MAXOCTETS features
 * and adding those defines to ppp_opts.h just for having the value always defined to 0
 * is not worth it, thus reducing the overall complexity.
 *  1 + MAX(PPP_AUTH_SUPPORT + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT,
 *          PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT -1 + CCP_SUPPORT)
 *
 * PPP_AUTH_SUPPORT is not available in ppp_opts.h because it is defined later in ppp.h,
 * but we do not need to be that picky about the real number of simultaneously running
 * timers so we just set the base number of timeouts to 2, thus the following is enough
 * for now.
 *  2 + PPP_IPV4_SUPPORT + PPP_IPV6_SUPPORT + CCP_SUPPORT
 */




 /* PPP_SUPPORT */
 /* LWIP_HDR_PPP_IMPL_H */
