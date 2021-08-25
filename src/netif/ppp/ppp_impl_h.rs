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

// #define LWIP_HDR_PPP_IMPL_H

























/*
 * Memory used for control packets.
 *
 * PPP_CTRL_PBUF_MAX_SIZE is the amount of memory we allocate when we
 * cannot figure out how much we are going to use before filling the buffer.
 */

#define PPP_CTRL_PBUF_TYPE       PBUF_RAM
pub const PPP_CTRL_PBUF_MAX_SIZE: u32 = 512; 
 /* PPP_USE_PBUF_RAM */
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
pub const PPP_IPX: u32 = 0; x2b	/* IPX protocol */


pub const PPP_VJC_COMP: u32 = 0x2d;	/* VJ compressed TCP */pub const PPP_VJC_COMP: u32 = 0x2d;
#define	PPP_VJC_UNCOMP	0x2f	/* VJ uncompressed TCP */


pub const PPP_IPV6: u32 = 0x57;	/* Internet Protocol Version 6 */


pub const PPP_COMP: u32 = 0xfd;	/* compressed packet */

pub const PPP_IPCP: u32 = 0x8021;	/* IP Control Protocol */

pub const PPP_ATCP: u32 = 0x8029;	/* AppleTalk Control Protocol */pub const PPP_ATCP: u32 = 0x8029;
pub const PPP_IPXCP: u32 = 0; x802b	/* IPX Control Protocol */


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
  void (*connect) (pcb: &mut ppp_pcb, ctx: &mut ());

  /* Listen for an incoming connection (Passive mode) */
  void (*listen) (pcb: &mut ppp_pcb, ctx: &mut ());

  /* End a connection (i.e. initiate disconnect phase) */
  void (*disconnect) (pcb: &mut ppp_pcb, ctx: &mut ());
  /* Free lower protocol control block */
  err_t (*free) (pcb: &mut ppp_pcb, ctx: &mut ());
  /* Write a pbuf to a ppp link, only used from PPP functions to send PPP packets. */
  err_t (*write)(pcb: &mut ppp_pcb, ctx: &mut (), p: &mut pbuf);
  /* Send a packet from lwIP core (IPv4 or IPv6) */
  err_t (*netif_output)(pcb: &mut ppp_pcb, ctx: &mut (), p: &mut pbuf, u_short protocol);
  /* configure the transmit-side characteristics of the PPP interface */
  void (*send_config)(pcb: &mut ppp_pcb, ctx: &mut (), accm: u32, pcomp: i32, accomp: i32);
  /* confire the receive-side characteristics of the PPP interface */
  void (*recv_config)(pcb: &mut ppp_pcb, ctx: &mut (), accm: u32, pcomp: i32, accomp: i32);
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
     let letppp_ibytes: i32;	/* bytes received */     letppp_ibytes: i32;     letppp_ibytes: i32;     letppp_ibytes: i32;     letppp_ibytes: i32;     letppp_ibytes: i32;     let letppp_ibytes: i32;     let letppp_ibytes: i32;     let letppp_ibytes: i32;     let letppp_ibytes: i32;     let letppp_ibytes: i32;
     ppp_ipackets: i32;	/* packets received */
     ppp_ierrors: i32;	/* receive errors */
     ppp_obytes: i32;	/* bytes sent */
     ppp_opackets: i32;	/* packets sent */
     ppp_oerrors: i32;	/* transmit errors */
};


struct vjstat {
     let letvjs_packets: i32;	/* outbound packets */     letvjs_packets: i32;     letvjs_packets: i32;     letvjs_packets: i32;     letvjs_packets: i32;     letvjs_packets: i32;     letvjs_packets: i32;     letvjs_packets: i32;     let letvjs_packets: i32;     let letvjs_packets: i32;     let letvjs_packets: i32;     let letvjs_packets: i32;     let letvjs_packets: i32;     let letvjs_packets: i32;     let letvjs_packets: i32;
     vjs_compressed: i32; /* outbound compressed packets */
     vjs_searches: i32;	/* searches for connection state */
     vjs_misses: i32;	/* times couldn't find conn. state */
     vjs_uncompressedin: i32; /* inbound uncompressed packets */
     vjs_compressedin: i32; /* inbound compressed packets */
     vjs_errorin: i32;	/* inbound unknown type packets */
     vjs_tossed: i32;	/* inbound packets tossed because of error */
};


struct ppp_stats {
    struct pppstat p;		/* basic PPP statistics */

    struct vjstat vj;		/* VJ header compression statistics */

};


struct compstat {
     let letunc_bytes: i32;	/* total uncompressed bytes */     letunc_bytes: i32;     letunc_bytes: i32;     letunc_bytes: i32;     letunc_bytes: i32;     letunc_bytes: i32;     letunc_bytes: i32;     let letunc_bytes: i32;     let letunc_bytes: i32;     let letunc_bytes: i32;     let letunc_bytes: i32;     let letunc_bytes: i32;     let letunc_bytes: i32;
     unc_packets: i32;	/* total uncompressed packets */
     comp_bytes: i32;	/* compressed bytes */
     comp_packets: i32;	/* compressed packets */
     inc_bytes: i32;	/* incompressible bytes */
     inc_packets: i32;	/* incompressible packets */
     ratio: i32;		/* recent compression ratio << 8 */
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
    xmit_idle: time_t;		/* time since last NP packet sent */
    recv_idle: time_t;		/* time since last NP packet received */
};


/* values for epdisc.class */
pub const EPD_NULL: u32 = 0; 	/* null discriminator, no data */pub const EPD_NULL: u32 = 0; pub const EPD_NULL: u32 = 0; 
pub const EPD_IP: u32 = 2; 
pub const EPD_MAC: u32 = 3; 
pub const EPD_MAGIC: u32 = 4; 
#define EPD_PHONENUM	5

/*
 * Global variables.
 */

extern u8	multilink;	/* enable multilink operation */
extern u8	doing_multilink;
extern u8	multilink_master;
extern u8	bundle_eof;
extern u8	bundle_terminating;



extern  maxoctets: i32;	     /* Maximum octetes per session (in bytes) */
extern int       maxoctets_dir;      /* Direction :
				      0 - in+out (default)
				      1 - in
				      2 - out
				      3 - max(in,out) */
extern int       maxoctets_timeout;  /* Timeout for check of octets limit */
pub const PPP_OCTETS_DIRECTION_SUM: u32 = 0;
pub const PPP_OCTETS_DIRECTION_IN: u32 = 1; 
pub const PPP_OCTETS_DIRECTION_OUT: u32 = 2; 
pub const PPP_OCTETS_DIRECTION_MAXOVERAL: u32 = 3; 
/* same as previos, but little different on RADIUS side */
pub const PPP_OCTETS_DIRECTION_MAXSESSION: u32 = 4; 


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
    void (*init) (pcb: &mut ppp_pcb);
    /* Process a received packet */
    void (*input) (pcb: &mut ppp_pcb, u_pkt: &mut String, len: i32);
    /* Process a received protocol-reject */
    void (*protrej) (pcb: &mut ppp_pcb);
    /* Lower layer has come up */
    void (*lowerup) (pcb: &mut ppp_pcb);
    /* Lower layer has gone down */
    void (*lowerdown) (pcb: &mut ppp_pcb);
    /* Open the protocol */
    void (*open) (pcb: &mut ppp_pcb);
    /* Close the protocol */
    void (*close) (pcb: &mut ppp_pcb, reason: &String);

    /* Pra: i32 packet in readable form */
    int  (*printpkt) (const u_pkt: &mut String, len: i32,
			  void (*printer) (void *,  char *, ...),
			  arg: &mut Vec<u8>);


    /* Process a received data packet */
    void (*datainput) (pcb: &mut ppp_pcb, u_pkt: &mut String, len: i32);


    name: String;		/* Text name of protocol */
    data_name: String;	/* Text name of corresponding data protocol */


    option_t *options;		/* List of command-line options */
    /* Check requested options, assign defaults */
    void (*check_options) ();


    /* Configure interface for demand-dial */
    int  (*demand_conf) (unit: i32);
    /* Say whether to bring up link for this pkt */
    int  (*active_pkt) (u_pkt: &mut String, len: i32);

};

/* Table of pointers to supported protocols */
extern const struct protent* const protocols[];


/* Values for auth_pending, auth_done */

pub const PAP_WITHPEER: u32 = 0x1;pub const PAP_WITHPEER: u32 = 0x1;
pub const PAP_PEER: u32 = 0; x2


pub const CHAP_WITHPEER: u32 = 0x4;pub const CHAP_WITHPEER: u32 = 0x4;
pub const CHAP_PEER: u32 = 0; x8


pub const EAP_WITHPEER: u32 = 0x10;pub const EAP_WITHPEER: u32 = 0x10;
pub const EAP_PEER: u32 = 0; x20


/* Values for auth_done only */

pub const CHAP_MD5_WITHPEER: u32 = 0x40;pub const CHAP_MD5_WITHPEER: u32 = 0x40;
pub const CHAP_MD5_PEER: u32 = 0; x80pub const CHAP_MD5_PEER: u32 = 0; 
#define CHAP_MS_SHIFT		8	/* LSB position for MS auths */
pub const CHAP_MS_WITHPEER: u32 = 0x100;pub const CHAP_MS_WITHPEER: u32 = 0x100;pub const CHAP_MS_WITHPEER: u32 = 0x100;pub const CHAP_MS_WITHPEER: u32 = 0x100;
pub const CHAP_MS_PEER: u32 = 0; x200pub const CHAP_MS_PEER: u32 = 0; pub const CHAP_MS_PEER: u32 = 0; 
#define CHAP_MS2_WITHPEER	0x400
#define CHAP_MS2_PEER		0x800



/* Supported CHAP protocols */



#define CHAP_MDTYPE_SUPPORTED (MDTYPE_MICROSOFT_V2 | MDTYPE_MICROSOFT | MDTYPE_MD5)
 /* MSCHAP_SUPPORT */
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_MD5)


 /* CHAP_SUPPORT */
#define CHAP_MDTYPE_SUPPORTED (MDTYPE_NONE)



/*
 * PPP statistics structure
 */
struct pppd_stats {
     int	bytes_in;
     int	bytes_out;
     int	pkts_in;
     int	pkts_out;
};



/*
 * PPP private functions
 */

 
/*
 * Functions called from lwIP core.
 */

/* initialize the PPP subsystem */
ppp_init: i32();

/*
 * Functions called from PPP link protocols.
 */

/* Create a new PPP control block */
ppp_new: &mut ppp_pcb(pppif: &mut NetIfc,  callbacks: &mut link_callbacks, link_ctx_cb: &mut (),
                 ppp_link_status_cb_fn link_status_cb, ctx_cb: &mut ());

/* Initiate LCP open request */
pub fn  ppp_start(pcb: &mut ppp_pcb);

/* Called when link failed to setup */
pub fn  ppp_link_failed(pcb: &mut ppp_pcb);

/* Called when link is normally down (i.e. it was asked to end) */
pub fn  ppp_link_end(pcb: &mut ppp_pcb);

/* function called to process input packet */
pub fn  ppp_input(pcb: &mut ppp_pcb, pb: &mut pbuf);


/*
 * Functions called by PPP protocols.
 */

/* function called by all PPP subsystems to send packets */
pub fn  ppp_write(pcb: &mut ppp_pcb, p: &mut pbuf);

/* functions called by auth.c link_terminated() */
pub fn  ppp_link_terminated(pcb: &mut ppp_pcb);

pub fn  new_phase(pcb: &mut ppp_pcb, p: i32);

ppp_send_config: i32(pcb: &mut ppp_pcb, mtu: i32, accm: u32, pcomp: i32, accomp: i32);
ppp_recv_config: i32(pcb: &mut ppp_pcb, mru: i32, accm: u32, pcomp: i32, accomp: i32);


sifaddr: i32(pcb: &mut ppp_pcb, our_adr: u32, his_adr: u32, netmask: u32);
cifaddr: i32(pcb: &mut ppp_pcb, our_adr: u32, his_adr: u32);

sifproxyarp: i32(pcb: &mut ppp_pcb, his_adr: u32);
cifproxyarp: i32(pcb: &mut ppp_pcb, his_adr: u32);


sdns: i32(pcb: &mut ppp_pcb, ns1: u32, ns2: u32);
cdns: i32(pcb: &mut ppp_pcb, ns1: u32, ns2: u32);


sifvjcomp: i32(pcb: &mut ppp_pcb, vjcomp: i32, cidcomp: i32, maxcid: i32);

sifup: i32(pcb: &mut ppp_pcb);
sifdown: i32 (pcb: &mut ppp_pcb);
get_mask: u32(addr: u32);



sif6addr: i32(pcb: &mut ppp_pcb, eui64_t our_eui64, eui64_t his_eui64);
cif6addr: i32(pcb: &mut ppp_pcb, eui64_t our_eui64, eui64_t his_eui64);
sif6up: i32(pcb: &mut ppp_pcb);
sif6down: i32 (pcb: &mut ppp_pcb);



sifnpmode: i32(pcb: &mut ppp_pcb, proto: i32, mode: NPmode);


pub fn  netif_set_mtu(pcb: &mut ppp_pcb, mtu: i32);
netif_get_mtu: i32(pcb: &mut ppp_pcb);



ccp_test: i32(pcb: &mut ppp_pcb, u_opt_ptr: &mut String, opt_len: i32, for_transmit: i32);

pub fn  ccp_set(pcb: &mut ppp_pcb, isopen: u8, isup: u8, receive_method: u8, transmit_method: u8);
pub fn  ccp_reset_comp(pcb: &mut ppp_pcb);
pub fn  ccp_reset_decomp(pcb: &mut ppp_pcb);

ccp_fatal_error: i32(pcb: &mut ppp_pcb);




get_idle_time: i32(pcb: &mut ppp_pcb, ip: &mut ppp_idle);



get_loop_output: i32();


/* Optional protocol names list, to make our messages a little more informative. */

const char * protocol_name(proto: i32);


/* Optional stats support, to get some statistics on the PPP interface */

pub fn  print_link_stats(); /* Prstats: i32, if available */
pub fn  reset_link_stats(u: i32); /* Reset (init) stats when link goes up */
pub fn  update_link_stats(u: i32); /* Get stats at link termination */




/*
 * Inline versions of get/put char/short/long.
 * Pointer is advanced; we assume that both arguments
 * are lvalues and will already be in registers.
 * cp MUST be u_char *.
 */
#define GETCHAR(c, cp) { \
	(c) = *(cp)+= 1; \
}
#define PUTCHAR(c, cp) { \
	*(cp)+= 1 = (u_char) (c); \
}
#define GETSHORT(s, cp) { \
	(s) = *(cp)+= 1 << 8; \
	(s) |= *(cp)+= 1; \
}
#define PUTSHORT(s, cp) { \
	*(cp)+= 1 = (u_char) ((s) >> 8); \
	*(cp)+= 1 = (u_char) (s); \
}
#define GETLONG(l, cp) { \
	(l) = *(cp)+= 1 << 8; \
	(l) |= *(cp)+= 1; (l) <<= 8; \
	(l) |= *(cp)+= 1; (l) <<= 8; \
	(l) |= *(cp)+= 1; \
}
#define PUTLONG(l, cp) { \
	*(cp)+= 1 = (u_char) ((l) >> 24); \
	*(cp)+= 1 = (u_char) ((l) >> 16); \
	*(cp)+= 1 = (u_char) ((l) >> 8); \
	*(cp)+= 1 = (u_char) (l); \
}

#define INCPTR(n, cp)	((cp) += (n))
#define DECPTR(n, cp)	((cp) -= (n))

/*
 * System dependent definitions for user-level 4.3BSD UNIX implementation.
 */
#define TIMEOUT(f, a, t)        loop { sys_untimeout((f), (a)); sys_timeout((t)*1000, (f), (a)); } while(0)
#define TIMEOUTMS(f, a, t)      loop { sys_untimeout((f), (a)); sys_timeout((t), (f), (a)); } while(0)
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
pub fn  link_required(pcb: &mut ppp_pcb);     /* we are starting to use the link */
pub fn  link_terminated(pcb: &mut ppp_pcb);   /* we are finished with the link */
pub fn  link_down(pcb: &mut ppp_pcb);	      /* the LCP layer has left the Opened state */
pub fn  upper_layers_down(pcb: &mut ppp_pcb); /* take all NCPs down */
pub fn  link_established(pcb: &mut ppp_pcb);  /* the link is up; authenticate now */
pub fn  start_networks(pcb: &mut ppp_pcb);    /* start all the network control protos */
pub fn  continue_networks(pcb: &mut ppp_pcb); /* start network [ip, etc] control protos */


auth_check_passwd: i32(pcb: &mut ppp_pcb, auser: &mut String, userlen: i32, apasswd: &mut String, passwdlen: i32,  char **msg, int *msglen);
                                /* check the user name and passwd against configuration */
pub fn  auth_peer_fail(pcb: &mut ppp_pcb, protocol: i32);
				/* peer failed to authenticate itself */
pub fn  auth_peer_success(pcb: &mut ppp_pcb, protocol: i32, prot_flavor: i32, name: &String, namelen: i32);
				/* peer successfully authenticated itself */

pub fn  auth_withpeer_fail(pcb: &mut ppp_pcb, protocol: i32);
				/* we failed to authenticate ourselves */
pub fn  auth_withpeer_success(pcb: &mut ppp_pcb, protocol: i32, prot_flavor: i32);
				/* we successfully authenticated ourselves */

pub fn  np_up(pcb: &mut ppp_pcb, proto: i32);    /* a network protocol has come up */
pub fn  np_down(pcb: &mut ppp_pcb, proto: i32);  /* a network protocol has gone down */
pub fn  np_finished(pcb: &mut ppp_pcb, proto: i32); /* a network protocol no longer needs link */

get_secret: i32(pcb: &mut ppp_pcb, client: &String, server: &String, secret: &mut String, int *secret_len, am_server: i32);
				/* get "secret" for chap */


/* Procedures exported from ipcp.c */
/* parse_dotted_ip: i32 (char *, u32 *); */

/* Procedures exported from demand.c */

pub fn  demand_conf ();	/* config interface(s) for demand-dial */
pub fn  demand_block ();	/* set all NPs to queue up packets */
pub fn  demand_unblock (); /* set all NPs to pass packets */
pub fn  demand_discard (); /* set all NPs to discard packets */
pub fn  demand_rexmit (int, u32); /* retransmit saved frames for an NP*/
int  loop_chars ( char *, int); /* process chars from loopback */
int  loop_frame ( char *, int); /* should we bring link up? */


/* Procedures exported from multilink.c */

pub fn  mp_check_options (); /* Check multilink-related options */
int  mp_join_bundle ();  /* join our link to an appropriate bundle */
pub fn  mp_exit_bundle ();  /* have disconnected our link from bundle */
pub fn  mp_bundle_terminated ();
epdisc_to_str: &mut String (struct epdisc *); /* string from endpodiscrim: i32. */
int  str_to_epdisc (struct epdisc *, char *); /* endpt disc. from str */

#define mp_bundle_terminated()	/* nothing */
#define mp_exit_bundle()	/* nothing */
pub const doing_multilink: u32 = 0;pub const doing_multilink: u32 = 0;
pub const multilink_master: u32 = 0; 


/* Procedures exported from utils.c. */
pub fn  ppp_print_string(const u_p: &mut String, len: i32, void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>);   /* Format a string for output */
ppp_slprintf: i32(buf: &mut String, buflen: i32, fmt: &String, ...);            /* sprintf+= 1 */
ppp_vslprintf: i32(buf: &mut String, buflen: i32, fmt: &String, va_list args);  /* vsprintf+= 1 */
ppp_strlcpy: usize(dest: &mut String, src: &String, len: usize);        /* safe strcpy */
ppp_strlcat: usize(dest: &mut String, src: &String, len: usize);        /* safe strncpy */
pub fn  ppp_dbglog(fmt: &String, ...);    /* log a debug message */
pub fn  ppp_info(fmt: &String, ...);      /* log an informational message */
pub fn  ppp_notice(fmt: &String, ...);    /* log a notice-level message */
pub fn  ppp_warn(fmt: &String, ...);      /* log a warning message */
pub fn  ppp_error(fmt: &String, ...);     /* log an error message */
pub fn  ppp_fatal(fmt: &String, ...);     /* log an error message and die(1) */

pub fn  ppp_dump_packet(pcb: &mut ppp_pcb, tag: &String,  p: &mut String, len: i32);
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




