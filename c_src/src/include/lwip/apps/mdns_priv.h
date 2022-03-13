/**
 * @file
 * MDNS responder private definitions
 */

 /*
 * Copyright (c) 2015 Verisure Innovation AB
 * All rights reserved.
 *
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
 * Author: Erik Ekman <erik@kryo.se>
 * Author: Jasper Verschueren <jasper.verschueren@apart-audio.com>
 *
 */



// #include "lwip/apps/mdns.h"
// #include "lwip/apps/mdns_opts.h"
// #include "lwip/pbuf.h"




// #if LWIP_MDNS_RESPONDER

pub const MDNS_READNAME_ERROR: u32 = 0xFFFF; #define NUM_DOMAIN_OFFSETS 10

pub const SRV_PRIORITY: u32 = 0; #define SRV_WEIGHT   0

/* mDNS TTL: (RFC6762 section 10)
 *  - 120 seconds if the hostname appears somewhere in the RR
 *  - 75 minutes if not (4500 seconds)
 *  - 10 seconds if responding to a legacy query
 */
pub const MDNS_TTL_10: u32 = 10; #define MDNS_TTL_120   120
pub const MDNS_TTL_4500: u32 = 4500; /* RFC6762 section 8.1: If fifteen conflicts occur within any ten-second period,
 * then the host MUST wait at least five seconds before each successive
 * additional probe attempt.
 */
pub const MDNS_PROBE_MAX_CONFLICTS_BEFORE_RATE_LIMIT: u32 = 15; #define MDNS_PROBE_MAX_CONFLICTS_TIME_WINDOW        10000
pub const MDNS_PROBE_MAX_CONFLICTS_TIMEOUT: u32 = 5000; #if LWIP_MDNS_SEARCH
/** Description of a search request */
struct mdns_request {
  /** Name of service, like 'myweb' */
  char name[MDNS_LABEL_MAXLEN + 1];
  /** Type of service, like '_http' or '_services._dns-sd' */
  struct mdns_domain service;
  /** Callback function called for each response */
  search_result_fn_t result_fn;
  void *arg;
  /** Protocol, TCP or UDP */
  proto: u16;
  /** Query type (PTR, SRV, ...) */
  qtype: u8;
  /** PTR only request. */
  only_ptr: u16;
};


/** Description of a service */
struct mdns_service {
  /** TXT record to answer with */
  struct mdns_domain txtdata;
  /** Name of service, like 'myweb' */
  char name[MDNS_LABEL_MAXLEN + 1];
  /** Type of service, like '_http' */
  char service[MDNS_LABEL_MAXLEN + 1];
  /** Callback function and userdata
   * to update txtdata buffer */
  service_get_txt_fn_t txt_fn;
  void *txt_userdata;
  /** Protocol, TCP or UDP */
  proto: u16;
  /** Port of the service */
  port: u16;
};

/** mDNS output packet */
struct mdns_outpacket {
  /** Packet data */
  struct pbuf *pbuf;
  /** Current write offset in packet */
  write_offset: u16;
  /** Number of questions written */
  questions: u16;
  /** Number of normal answers written */
  answers: u16;
  /** Number of authoritative answers written */
  authoritative: u16;
  /** Number of additional answers written */
  additional: u16;
  /** Offsets for written domain names in packet.
   *  Used for compression */
  u16_t domain_offsets[NUM_DOMAIN_OFFSETS];
};

/** mDNS output message */
struct mdns_outmsg {
  /** Identifier. Used in legacy queries */
  tx_id: u16;
  /** dns flags */
  flags: u8;
  /** Destination IP/port if sent unicast */
  ip_addr_t dest_addr;
  dest_port: u16;
  /** If all answers in packet should set cache_flush bit */
  cache_flush: u8;
  /** If reply should be sent unicast (as requested) */
  unicast_reply_requested: u8;
  /** If legacy query. (tx_id needed, and write
   *  question again in reply before answer) */
  legacy_query: u8;
  /** If the query is a probe msg we need to respond immediately. Independent of
   *  the QU or QM flag. */
  probe_query_recv: u8;
  /* Question bitmask for host information */
  host_questions: u8;
  /* Questions bitmask per service */
  u8_t serv_questions[MDNS_MAX_SERVICES];
  /* Reply bitmask for host information */
  host_replies: u8;
  /* Bitmask for which reverse IPv6 hosts to answer */
  host_reverse_v6_replies: u8;
  /* Reply bitmask per service */
  u8_t serv_replies[MDNS_MAX_SERVICES];
#ifdef LWIP_MDNS_SEARCH
  /** Search query to send */
  struct mdns_request *query;



/** Delayed msg info */
struct mdns_delayed_msg {
  /** Signals if a multicast msg needs to be send out */
  multicast_msg_waiting: u8;
  /** Multicast timeout for all multicast traffic except probe answers */
  multicast_timeout: u8;
  /** Multicast timeout only for probe answers */
  multicast_probe_timeout: u8;
  /** Output msg used for delayed multicast responses */
  struct mdns_outmsg delayed_msg_multicast;
  /** Prefer multicast over unicast timeout -> 25% of TTL = we take 30s as
      general delay. */
  multicast_timeout_25TTL: u8;
  /** Only send out new unicast message if previous was send */
  unicast_msg_in_use: u8;
  /** Output msg used for delayed unicast responses */
  struct mdns_outmsg delayed_msg_unicast;
};

/* MDNS states */
typedef enum {
  /* MDNS module is off */
  MDNS_STATE_OFF,
  /* Waiting before probing can be started */
  MDNS_STATE_PROBE_WAIT,
  /* Probing the unique records */
  MDNS_STATE_PROBING,
  /* Waiting before announcing the probed unique records */
  MDNS_STATE_ANNOUNCE_WAIT,
  /* Announcing all records */
  MDNS_STATE_ANNOUNCING,
  /* Probing and announcing completed */
  MDNS_STATE_COMPLETE
} mdns_resp_state_enum_t;

/** Description of a host/netif */
struct mdns_host {
  /** Hostname */
  char name[MDNS_LABEL_MAXLEN + 1];
  /** Pointer to services */
  struct mdns_service *services[MDNS_MAX_SERVICES];
  /** Number of probes/announces sent for the current name */
  sent_num: u8;
  /** State of the mdns responder */
  mdns_resp_state_enum_t state;

  /** delayed msg struct for IPv4 */
  struct mdns_delayed_msg ipv4;

IP_IPV6
  /** delayed msg struct for IPv6 */
  struct mdns_delayed_msg ipv6;

Timestamp of probe conflict saved in list */
  u32_t conflict_time[MDNS_PROBE_MAX_CONFLICTS_BEFORE_RATE_LIMIT];
  /** Rate limit flag */
  rate_limit_activated: u8;
  /** List index for timestamps */
  index: u8;
  /** number of conflicts since startup */
  num_conflicts: u8;
};

struct mdns_host* netif_mdns_data(struct netif *netif);
struct udp_pcb* get_mdns_pcb();

 /* LWIP_MDNS_RESPONDER */




 /* LWIP_HDR_MDNS_PRIV_H */
