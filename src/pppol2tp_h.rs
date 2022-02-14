/**
 * @file
 * Network Point to Point Protocol over Layer 2 Tunneling Protocol header file.
 *
 */

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












/* Timeout */
#define PPPOL2TP_CONTROL_TIMEOUT         (5*1000)  /* base for quick timeout calculation */
#define PPPOL2TP_SLOW_RETRY              (60*1000) /* persistent retry interval */

pub const PPPOL2TP_MAXSCCRQ: u32 = 4;         /* retry SCCRQ four times (quickly) */
pub const PPPOL2TP_MAXICRQ: u32 = 4;         /* retry IRCQ four times */
pub const PPPOL2TP_MAXICCN: u32 = 4;         /* retry ICCN four times */

/* L2TP header flags */
pub const PPPOL2TP_HEADERFLAG_CONTROL: u32 = 0x8000;
pub const PPPOL2TP_HEADERFLAG_LENGTH: u32 = 0x4000;
pub const PPPOL2TP_HEADERFLAG_SEQUENCE: u32 = 0x0800;
pub const PPPOL2TP_HEADERFLAG_OFFSET: u32 = 0x0200;
pub const PPPOL2TP_HEADERFLAG_PRIORITY: u32 = 0x0100;
pub const PPPOL2TP_HEADERFLAG_VERSION: u32 = 0x0002;

/* Mandatory bits for control: Control, Length, Sequence, Version 2 */
#define PPPOL2TP_HEADERFLAG_CONTROL_MANDATORY     (PPPOL2TP_HEADERFLAG_CONTROL|PPPOL2TP_HEADERFLAG_LENGTH|PPPOL2TP_HEADERFLAG_SEQUENCE|PPPOL2TP_HEADERFLAG_VERSION)
/* Forbidden bits for control: Offset, Priority */
#define PPPOL2TP_HEADERFLAG_CONTROL_FORBIDDEN     (PPPOL2TP_HEADERFLAG_OFFSET|PPPOL2TP_HEADERFLAG_PRIORITY)

/* Mandatory bits for data: Version 2 */
#define PPPOL2TP_HEADERFLAG_DATA_MANDATORY        (PPPOL2TP_HEADERFLAG_VERSION)

/* AVP (Attribute Value Pair) header */
pub const PPPOL2TP_AVPHEADERFLAG_MANDATORY: u32 = 0x8000;
pub const PPPOL2TP_AVPHEADERFLAG_HIDDEN: u32 = 0x4000;
pub const PPPOL2TP_AVPHEADERFLAG_LENGTHMASK: u32 = 0x03ff;

/* -- AVP - Message type */
pub const PPPOL2TP_AVPTYPE_MESSAGE: u32 = 0; /* Message type */

/* Control Connection Management */
pub const PPPOL2TP_MESSAGETYPE_SCCRQ: u32 = 1; /* Start Control Connection Request */
pub const PPPOL2TP_MESSAGETYPE_SCCRP: u32 = 2; /* Start Control Connection Reply */
pub const PPPOL2TP_MESSAGETYPE_SCCCN: u32 = 3; /* Start Control Connection Connected */
pub const PPPOL2TP_MESSAGETYPE_STOPCCN: u32 = 4; /* Stop Control Connection Notification */
pub const PPPOL2TP_MESSAGETYPE_HELLO: u32 = 6; /* Hello */
/* Call Management */
pub const PPPOL2TP_MESSAGETYPE_OCRQ: u32 = 7; /* Outgoing Call Request */
pub const PPPOL2TP_MESSAGETYPE_OCRP: u32 = 8; /* Outgoing Call Reply */
pub const PPPOL2TP_MESSAGETYPE_OCCN: u32 = 9; /* Outgoing Call Connected */
pub const PPPOL2TP_MESSAGETYPE_ICRQ: u32 = 10; /* Incoming Call Request */
pub const PPPOL2TP_MESSAGETYPE_ICRP: u32 = 11; /* Incoming Call Reply */
pub const PPPOL2TP_MESSAGETYPE_ICCN: u32 = 12; /* Incoming Call Connected */
pub const PPPOL2TP_MESSAGETYPE_CDN: u32 = 14; /* Call Disconnect Notify */
/* Error reporting */
pub const PPPOL2TP_MESSAGETYPE_WEN: u32 = 15; /* WAN Error Notify */
/* PPP Session Control */
pub const PPPOL2TP_MESSAGETYPE_SLI: u32 = 16; /* Set Link Info */

/* -- AVP - Result code */
pub const PPPOL2TP_AVPTYPE_RESULTCODE: u32 = 1; /* Result code */
pub const PPPOL2TP_RESULTCODE: u32 = 1; /* General request to clear control connection */

/* -- AVP - Protocol version (!= L2TP Header version) */
pub const PPPOL2TP_AVPTYPE_VERSION: u32 = 2;
pub const PPPOL2TP_VERSION: u32 = 0x0100; /* L2TP Protocol version 1, revision 0 */

/* -- AVP - Framing capabilities */
pub const PPPOL2TP_AVPTYPE_FRAMINGCAPABILITIES: u32 = 3; /* Bearer capabilities */
pub const PPPOL2TP_FRAMINGCAPABILITIES: u32 = 0x00000003; /* Async + Sync framing */

/* -- AVP - Bearer capabilities */
pub const PPPOL2TP_AVPTYPE_BEARERCAPABILITIES: u32 = 4; /* Bearer capabilities */
pub const PPPOL2TP_BEARERCAPABILITIES: u32 = 0x00000003; /* Analog + Digital Access */

/* -- AVP - Tie breaker */
pub const PPPOL2TP_AVPTYPE_TIEBREAKER: u32 = 5;

/* -- AVP - Host name */
pub const PPPOL2TP_AVPTYPE_HOSTNAME: u32 = 7; /* Host name */
#define PPPOL2TP_HOSTNAME        "lwIP" /* FIXME: make it configurable */

/* -- AVP - Vendor name */
pub const PPPOL2TP_AVPTYPE_VENDORNAME: u32 = 8; /* Vendor name */
#define PPPOL2TP_VENDORNAME      "lwIP" /* FIXME: make it configurable */

/* -- AVP - Assign tunnel ID */
pub const PPPOL2TP_AVPTYPE_TUNNELID: u32 = 9; /* Assign Tunnel ID */

/* -- AVP - Receive window size */
pub const PPPOL2TP_AVPTYPE_RECEIVEWINDOWSIZE: u32 = 10; /* Receive window size */
pub const PPPOL2TP_RECEIVEWINDOWSIZE: u32 = 8; /* FIXME: make it configurable */

/* -- AVP - Challenge */
pub const PPPOL2TP_AVPTYPE_CHALLENGE: u32 = 11; /* Challenge */

/* -- AVP - Cause code */
pub const PPPOL2TP_AVPTYPE_CAUSECODE: u32 = 12; /* Cause code*/

/* -- AVP - Challenge response */
pub const PPPOL2TP_AVPTYPE_CHALLENGERESPONSE: u32 = 13; /* Challenge response */
pub const PPPOL2TP_AVPTYPE_CHALLENGERESPONSE_SIZE: u32 = 16;

/* -- AVP - Assign session ID */
pub const PPPOL2TP_AVPTYPE_SESSIONID: u32 = 14; /* Assign Session ID */

/* -- AVP - Call serial number */
pub const PPPOL2TP_AVPTYPE_CALLSERIALNUMBER: u32 = 15; /* Call Serial Number */

/* -- AVP - Framing type */
pub const PPPOL2TP_AVPTYPE_FRAMINGTYPE: u32 = 19; /* Framing Type */
pub const PPPOL2TP_FRAMINGTYPE: u32 = 0x00000001; /* Sync framing */

/* -- AVP - TX Connect Speed */
pub const PPPOL2TP_AVPTYPE_TXCONNECTSPEED: u32 = 24; /* TX Connect Speed */
pub const PPPOL2TP_TXCONNECTSPEED: u32 = 100000000; /* Connect speed: 100 Mbits/s */

/* L2TP Session state */
pub const PPPOL2TP_STATE_INITIAL: u32 = 0;
pub const PPPOL2TP_STATE_SCCRQ_SENT: u32 = 1;
pub const PPPOL2TP_STATE_ICRQ_SENT: u32 = 2;
pub const PPPOL2TP_STATE_ICCN_SENT: u32 = 3;
pub const PPPOL2TP_STATE_DATA: u32 = 4;

pub const PPPOL2TP_OUTPUT_DATA_HEADER_LEN: u32 = 6; /* Our data header len */

/*
 * PPPoL2TP interface control block.
 */
typedef struct pppol2tp_pcb_s pppol2tp_pcb;
struct pppol2tp_pcb_s {
  ppp_pcb *ppp;                /* PPP PCB */
  u8_t phase;                  /* L2TP phase */
  struct udp_pcb *udp;         /* UDP L2TP Socket */
  struct netif *netif;         /* Output interface, used as a default route */
  ip_addr_t remote_ip;         /* LNS IP Address */
  u16_t remote_port;           /* LNS port */

  const u8_t *secret;          /* Secret string */
  u8_t secret_len;             /* Secret string length */
  u8_t secret_rv[16];          /* Random vector */
  u8_t challenge_hash[16];     /* Challenge response */
  u8_t send_challenge;         /* Boolean whether the next sent packet should contains a challenge response */
 /* PPPOL2TP_AUTH_SUPPORT */

  u16_t tunnel_port;           /* Tunnel port */
  u16_t our_ns;                /* NS to peer */
  u16_t peer_nr;               /* NR from peer */
  u16_t peer_ns;               /* Expected NS from peer */
  u16_t source_tunnel_id;      /* Tunnel ID assigned by peer */
  u16_t remote_tunnel_id;      /* Tunnel ID assigned to peer */
  u16_t source_session_id;     /* Session ID assigned by peer */
  u16_t remote_session_id;     /* Session ID assigned to peer */

  u8_t sccrq_retried;          /* number of SCCRQ retries already done */
  u8_t icrq_retried;           /* number of ICRQ retries already done */
  u8_t iccn_retried;           /* number of ICCN retries already done */
};


/* Create a new L2TP session. */
ppp_pcb *pppol2tp_create(struct netif *pppif,
       struct netif *netif, const ip_addr_t *ipaddr, u16_t port,
       const u8_t *secret, u8_t secret_len,
       ppp_link_status_cb_fn link_status_cb, void *ctx_cb);


}


 /* PPPOL2TP_H */
 /* PPP_SUPPORT && PPPOL2TP_SUPPORT */
