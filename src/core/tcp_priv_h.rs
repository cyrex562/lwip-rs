/*
 * @file
 * TCP internal implementations (do not use in application code)
 */

/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
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
 * Author: Adam Dunkels <adam@sics.se>
 *
 */

// #define LWIP_HDR_TCP_PRIV_H



















/* Functions for interfacing with TCP: */

/* Lower layer interface to TCP: */
pub fn              tcp_init    ();  /* Initialize this module. */
pub fn              tcp_tmr     ();  /* Must be called every
                                         TCP_TMR_INTERVAL
                                         ms. (Typically 250 ms). */
/* It is also possible to call these two functions at the right
   intervals (instead of calling tcp_tmr()). */
pub fn              tcp_slowtmr ();
pub fn              tcp_fasttmr ();

/* Call this from a netif driver (watch out for threading issues!) that has
   returned a memory error on transmit and now has free buffers to send more.
   This iterates all active pcbs that had an error and tries to call
   tcp_output, so use this with care as it might slow down the system. */
pub fn              tcp_txnow   ();

/* Only used by IP to pass a TCP segment to TCP: */
pub fn              tcp_input   (p: &mut pbuf, inp: &mut NetIfc);
/* Used within the TCP code only: */
struct tcp_pcb * tcp_alloc   (prio: u8);
pub fn              tcp_free    (pcb: &mut tcp_pcb);
pub fn              tcp_abandon (pcb: &mut tcp_pcb, reset: i32);
pub fn             tcp_send_empty_ack(pcb: &mut tcp_pcb);
pub fn             tcp_rexmit  (pcb: &mut tcp_pcb);
pub fn             tcp_rexmit_rto_prepare(pcb: &mut tcp_pcb);
pub fn              tcp_rexmit_rto_commit(pcb: &mut tcp_pcb);
pub fn              tcp_rexmit_rto  (pcb: &mut tcp_pcb);
pub fn              tcp_rexmit_fast (pcb: &mut tcp_pcb);
u32            tcp_update_rcv_ann_wnd(pcb: &mut tcp_pcb);
pub fn             tcp_process_refused_data(pcb: &mut tcp_pcb);

/*
 * This is the Nagle algorithm: try to combine user data to send as few TCP
 * segments as possible. Only send if
 * - no previously transmitted data on the connection remains unacknowledged or
 * - the TF_NODELAY flag is set (nagle algorithm turned off for this pcb) or
 * - the only unsent segment is at least pcb.mss bytes long (or there is more
 *   than one unsent segment - with lwIP, this can happen although unsent.len < mss)
 * - or if we are in fast-retransmit (TF_INFR)
 */
#define tcp_do_output_nagle(tpcb) ((((tpcb).unacked == NULL) || \
                            ((tpcb).flags & (TF_NODELAY | TF_INFR)) || \
                            (((tpcb).unsent != NULL) && (((tpcb).unsent.next != NULL) || \
                              ((tpcb).unsent.len >= tpcb.mss))) || \
                            ((tcp_sndbuf(tpcb) == 0) || (tcp_sndqueuelen(tpcb) >= TCP_SND_QUEUELEN)) \
                            ))
#define tcp_output_nagle(tpcb) (tcp_do_output_nagle(tpcb) ? tcp_output(tpcb) : ERR_OK)


#define TCP_SEQ_LT(a,b)     (((a) - (b)) < 0)
#define TCP_SEQ_LEQ(a,b)    (((a) - (b)) <= 0)
#define TCP_SEQ_GT(a,b)     (((a) - (b)) > 0)
#define TCP_SEQ_GEQ(a,b)    (((a) - (b)) >= 0)
/* is b<=a<=c? */

#define TCP_SEQ_BETWEEN(a,b,c) ((c)-(b) >= (a)-(b))

#define TCP_SEQ_BETWEEN(a,b,c) (TCP_SEQ_GEQ(a,b) && TCP_SEQ_LEQ(a,c))


pub const TCP_TMR_INTERVAL: u32 = 250;   /* The TCP timer interval in milliseconds. */



#define TCP_FAST_INTERVAL      TCP_TMR_INTERVAL /* the fine grained timeout in milliseconds */



#define TCP_SLOW_INTERVAL      (2*TCP_TMR_INTERVAL)  /* the coarse grained timeout in milliseconds */


pub const TCP_FIN_WAIT_TIMEOUT: u32 = 20000;  /* milliseconds */
pub const TCP_SYN_RCVD_TIMEOUT: u32 = 20000;  /* milliseconds */pub const TCP_SYN_RCVD_TIMEOUT: u32 = 20000; 
#define TCP_OOSEQ_TIMEOUT        6 /* x RTO */


pub const TCP_MSL: u32 = 60000;  /* The maximum segment lifetime in milliseconds */


/* Keepalive values, compliant with RFC 1122. Don't change this unless you know what you're doing */

#define  TCP_KEEPIDLE_DEFAULT     7200000 /* Default KEEPALIVE timer in milliseconds */



#define  TCP_KEEPINTVL_DEFAULT    75000   /* Default Time between KEEPALIVE probes in milliseconds */



#define  TCP_KEEPCNT_DEFAULT      9        /* Default Counter for KEEPALIVE probes */


#define  TCP_MAXIDLE              TCP_KEEPCNT_DEFAULT * TCP_KEEPINTVL_DEFAULT  /* Maximum KEEPALIVE probe time */

#define TCP_TCPLEN(seg) ((seg).len + (((TCPH_FLAGS((seg).tcphdr) & (TCP_FIN | TCP_SYN)) != 0)))

/* Flags used on input processing, not on pcb.flags
*/
pub const TF_RESET: u32 = 0; x08   /* Connection was reset. */pub const TF_RESET: u32 = 0; pub const TF_RESET: u32 = 0; 
#define TF_CLOSED    0x10   /* Connection was successfully closed. */
#define TF_GOT_FIN   0x20   /* Connection was closed by the remote end. */




#define TCP_EVENT_ACCEPT(lpcb,pcb,arg,err,ret) ret = lwip_tcp_event(arg, (pcb),\
                LWIP_EVENT_ACCEPT, NULL, 0, err)
#define TCP_EVENT_SENT(pcb,space,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
                   LWIP_EVENT_SENT, NULL, space, ERR_OK)
#define TCP_EVENT_RECV(pcb,p,err,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
                LWIP_EVENT_RECV, (p), 0, (err))
#define TCP_EVENT_CLOSED(pcb,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
                LWIP_EVENT_RECV, NULL, 0, ERR_OK)
#define TCP_EVENT_CONNECTED(pcb,err,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
                LWIP_EVENT_CONNECTED, NULL, 0, (err))
#define TCP_EVENT_POLL(pcb,ret)       loop { if (pcb.state != SYN_RCVD) {                          \
                ret = lwip_tcp_event((pcb).callback_arg, (pcb), LWIP_EVENT_POLL, NULL, 0, ERR_OK); \
                } else {                                                                            \
                ret = ERR_ARG; } } while(0)
/* For event API, last state SYN_RCVD must be excluded here: the application
   has not seen this pcb, yet! */
#define TCP_EVENT_ERR(last_state,errf,arg,err)  loop { if (last_state != SYN_RCVD) {                \
                lwip_tcp_event((arg), NULL, LWIP_EVENT_ERR, NULL, 0, (err)); } } while(0)

 /* LWIP_EVENT_API */

#define TCP_EVENT_ACCEPT(lpcb,pcb,arg,err,ret)                 \
  loop {                                                         \
    if(lpcb.accept != NULL)                                 \
      (ret) = lpcb.accept((arg),(pcb),(err));               \
    else (ret) = ERR_ARG;                                      \
  } while (0)

#define TCP_EVENT_SENT(pcb,space,ret)                          \
  loop {                                                         \
    if((pcb).sent != NULL)                                    \
      (ret) = (pcb).sent((pcb).callback_arg,(pcb),(space));  \
    else (ret) = ERR_OK;                                       \
  } while (0)

#define TCP_EVENT_RECV(pcb,p,err,ret)                          \
  loop {                                                         \
    if(pcb.recv != NULL) {                                  \
      (ret) = pcb.recv((pcb).callback_arg,(pcb),(p),(err));\
    } else {                                                   \
      (ret) = tcp_recv_null(NULL, (pcb), (p), (err));          \
    }                                                          \
  } while (0)

#define TCP_EVENT_CLOSED(pcb,ret)                                \
  loop {                                                           \
    if((pcb.recv != NULL)) {                                  \
      (ret) = pcb.recv((pcb).callback_arg,(pcb),NULL,ERR_OK);\
    } else {                                                     \
      (ret) = ERR_OK;                                            \
    }                                                            \
  } while (0)

#define TCP_EVENT_CONNECTED(pcb,err,ret)                         \
  loop {                                                           \
    if(pcb.connected != NULL)                                 \
      (ret) = pcb.connected((pcb).callback_arg,(pcb),(err)); \
    else (ret) = ERR_OK;                                         \
  } while (0)

#define TCP_EVENT_POLL(pcb,ret)                                \
  loop {                                                         \
    if(pcb.poll != NULL)                                    \
      (ret) = pcb.poll((pcb).callback_arg,(pcb));          \
    else (ret) = ERR_OK;                                       \
  } while (0)

#define TCP_EVENT_ERR(last_state,errf,arg,err)                 \
  loop {                                                         \
                                   \
    if((errf) != NULL)                                         \
      (errf)((arg),(err));                                     \
  } while (0)



/* Enabled extra-check for TCP_OVERSIZE if LWIP_DEBUG is enabled */

pub const TCP_OVERSIZE_DBGCHECK: u32 = 1; 

pub const TCP_OVERSIZE_DBGCHECK: u32 = 0;


/* Don't generate checksum on copy if CHECKSUM_GEN_TCP is disabled */
#define TCP_CHECKSUM_ON_COPY  (LWIP_CHECKSUM_ON_COPY && CHECKSUM_GEN_TCP)

/* This structure represents a TCP segment on the unsent, unacked and ooseq queues */
struct tcp_seg {
  next: &mut tcp_seg;    /* used when putting segments on a queue */
  let p: &mut pbuf;          /* buffer containing data + TCP header */
  let len: u16;               /* the TCP length of this segment */  let len: u16;
  let oversize_left: u16;     /* Extra bytes available at the end of the last
                              pbuf in unsent (used for asserting vs.
                              tcp_pcb.unsent_oversize only) */


  let chksum: u16;
  u8  chksum_swapped;

  u8  flags;
pub const TF_SEG_OPTS_MSS: u32 = 0; x01 /* Include MSS option (only used in SYN segments) */pub const TF_SEG_OPTS_MSS: u32 = 0; pub const TF_SEG_OPTS_MSS: u32 = 0; 
#define TF_SEG_OPTS_TS          0x02 /* Include timestamp option. */
#define TF_SEG_DATA_CHECKSUMMED 0x04 /* ALL data (not the header) is
                                               checksummed into 'chksum' */
pub const TF_SEG_OPTS_WND_SCALE: u32 = 0; x08 /* Include WND SCALE option (only used in SYN segments) */pub const TF_SEG_OPTS_WND_SCALE: u32 = 0; 
#define TF_SEG_OPTS_SACK_PERM   0x10 /* Include SACK Permitted option (only used in SYN segments) */
  tcphdr: &mut tcp_hdr;  /* the TCP header */
};

pub const LWIP_TCP_OPT_EOL: u32 = 0;
// #define LWIP_TCP_OPT_NOP        1
// #define LWIP_TCP_OPT_MSS        2
// #define LWIP_TCP_OPT_WS         3
// #define LWIP_TCP_OPT_SACK_PERM  4
// #define LWIP_TCP_OPT_TS         8

// #define LWIP_TCP_OPT_LEN_MSS    4

// #define LWIP_TCP_OPT_LEN_TS     10
// #define LWIP_TCP_OPT_LEN_TS_OUT 12 /* aligned for output (includes NOP padding) */

pub const LWIP_TCP_OPT_LEN_TS_OUT: u32 = 0;


// #define LWIP_TCP_OPT_LEN_WS     3
// #define LWIP_TCP_OPT_LEN_WS_OUT 4 /* aligned for output (includes NOP padding) */

pub const LWIP_TCP_OPT_LEN_WS_OUT: u32 = 0;



// #define LWIP_TCP_OPT_LEN_SACK_PERM     2
// #define LWIP_TCP_OPT_LEN_SACK_PERM_OUT 4 /* aligned for output (includes NOP padding) */

pub const LWIP_TCP_OPT_LEN_SACK_PERM_OUT: u32 = 0;


// #define LWIP_TCP_OPT_LENGTH(flags) \
  ((flags) & TF_SEG_OPTS_MSS       ? LWIP_TCP_OPT_LEN_MSS           : 0) + \
  ((flags) & TF_SEG_OPTS_TS        ? LWIP_TCP_OPT_LEN_TS_OUT        : 0) + \
  ((flags) & TF_SEG_OPTS_WND_SCALE ? LWIP_TCP_OPT_LEN_WS_OUT        : 0) + \
  ((flags) & TF_SEG_OPTS_SACK_PERM ? LWIP_TCP_OPT_LEN_SACK_PERM_OUT : 0)

/* This returns a TCP header option for MSS in an u32 */
#define TCP_BUILD_MSS_OPTION(mss) lwip_htonl(0x02040000 | ((mss) & 0xFFFF))


#define TCPWNDSIZE_F       U32_F
pub const TCPWND_MAX: u32 = 0xFFFFFFFF;U
#define TCPWND_CHECK16(x)  LWIP_ASSERT("window size > 0xFFFF", (x) <= 0xFFFF)
#define TCPWND_MIN16(x)    (LWIP_MIN((x), 0xFFFF))
 /* LWIP_WND_SCALE */
#define TCPWNDSIZE_F       U16_F
pub const TCPWND_MAX: u32 = 0xFFFF;U
#define TCPWND_CHECK16(x)
#define TCPWND_MIN16(x)    x


/* Global variables: */
extern tcp_input_pcb: &mut tcp_pcb;
extern tcp_ticks: u32;
extern tcp_active_pcbs_changed: u8;

/* The TCP PCB lists. */
union tcp_listen_pcbs_t { /* List of all TCP PCBs in LISTEN state. */
  listen_pcbs: &mut tcp_pcb_listen;
  pcbs: &mut tcp_pcb;
};
extern tcp_bound_pcbs: &mut tcp_pcb;
extern union tcp_listen_pcbs_t tcp_listen_pcbs;
extern tcp_active_pcbs: &mut tcp_pcb;  /* List of all TCP PCBs that are in a
              state in which they accept or send
              data. */
extern tcp_tw_pcbs: &mut tcp_pcb;      /* List of all TCP PCBs in TIME-WAIT. */

pub const NUM_TCP_PCB_LISTS_NO_TIME_WAIT: u32 = 3; 
pub const NUM_TCP_PCB_LISTS: u32 = 4; 
extern struct tcp_pcb ** const tcp_pcb_lists[NUM_TCP_PCB_LISTS];

/* Axioms about the above lists:
   1) Every TCP PCB that is not CLOSED is in one of the lists.
   2) A PCB is only in one of the lists.
   3) All PCBs in the tcp_listen_pcbs list is in LISTEN state.
   4) All PCBs in the tcp_tw_pcbs list is in TIME-WAIT state.
*/
/* Define two macros, TCP_REG and TCP_RMV that registers a TCP PCB
   with a PCB list or removes a PCB from a list, respectively. */

pub const TCP_DEBUG_PCB_LISTS: u32 = 0;


#define TCP_REG(pcbs, npcb) loop {\
                            tcp_tmp_pcb: &mut tcp_pcb; \
//                            LWIP_DEBUGF(TCP_DEBUG, ("TCP_REG %p local port %"U16_F"\n", (npcb), npcb.local_port)); \
                            for (tcp_tmp_pcb = *(pcbs); \
          tcp_tmp_pcb != NULL; \
        tcp_tmp_pcb = tcp_tmp_pcb.next) { \
                                LWIP_ASSERT("TCP_REG: already registered\n", tcp_tmp_pcb != (npcb)); \
                            } \
                            LWIP_ASSERT("TCP_REG: pcb.state != CLOSED", ((pcbs) == &tcp_bound_pcbs) || (npcb.state != CLOSED)); \
                            (npcb).next = *(pcbs); \
                            LWIP_ASSERT("TCP_REG: npcb.next != npcb", (npcb).next != (npcb)); \
                            *(pcbs) = (npcb); \
                            LWIP_ASSERT("TCP_REG: tcp_pcbs sane", tcp_pcbs_sane()); \
              tcp_timer_needed(); \
                            } while(0)
#define TCP_RMV(pcbs, npcb) loop { \
                            tcp_tmp_pcb: &mut tcp_pcb; \
                            LWIP_ASSERT("TCP_RMV: pcbs != NULL", *(pcbs) != NULL); \
//                            LWIP_DEBUGF(TCP_DEBUG, ("TCP_RMV: removing %p from %p\n", (npcb), (*(pcbs)))); \
                            if(*(pcbs) == (npcb)) { \
                               *(pcbs) = (*pcbs).next; \
                            } else for (tcp_tmp_pcb = *(pcbs); tcp_tmp_pcb != NULL; tcp_tmp_pcb = tcp_tmp_pcb.next) { \
                               if(tcp_tmp_pcb.next == (npcb)) { \
                                  tcp_tmp_pcb.next = (npcb).next; \
                                  break; \
                               } \
                            } \
                            (npcb).next = NULL; \
                            LWIP_ASSERT("TCP_RMV: tcp_pcbs sane", tcp_pcbs_sane()); \
//                            LWIP_DEBUGF(TCP_DEBUG, ("TCP_RMV: removed %p from %p\n", (npcb), (*(pcbs)))); \
                            } while(0)

 /* LWIP_DEBUG */

#define TCP_REG(pcbs, npcb)                        \
  loop {                                             \
    (npcb).next = *pcbs;                          \
    *(pcbs) = (npcb);                              \
    tcp_timer_needed();                            \
  } while (0)

#define TCP_RMV(pcbs, npcb)                        \
  loop {                                             \
    if(*(pcbs) == (npcb)) {                        \
      (*(pcbs)) = (*pcbs).next;                   \
    }                                              \
    else {                                         \
      tcp_tmp_pcb: &mut tcp_pcb;                 \
      for (tcp_tmp_pcb = *pcbs;                    \
          tcp_tmp_pcb != NULL;                     \
          tcp_tmp_pcb = tcp_tmp_pcb.next) {       \
        if(tcp_tmp_pcb.next == (npcb)) {          \
          tcp_tmp_pcb.next = (npcb).next;        \
          break;                                   \
        }                                          \
      }                                            \
    }                                              \
    (npcb).next = NULL;                           \
  } while(0)



#define TCP_REG_ACTIVE(npcb)                       \
  loop {                                             \
    TCP_REG(&tcp_active_pcbs, npcb);               \
    tcp_active_pcbs_changed = 1;                   \
  } while (0)

#define TCP_RMV_ACTIVE(npcb)                       \
  loop {                                             \
    TCP_RMV(&tcp_active_pcbs, npcb);               \
    tcp_active_pcbs_changed = 1;                   \
  } while (0)

#define TCP_PCB_REMOVE_ACTIVE(pcb)                 \
  loop {                                             \
    tcp_pcb_remove(&tcp_active_pcbs, pcb);         \
    tcp_active_pcbs_changed = 1;                   \
  } while (0)


/* Internal functions: */
tcp_pcb_copy: &mut tcp_pcb(pcb: &mut tcp_pcb);
pub fn  tcp_pcb_purge(pcb: &mut tcp_pcb);
pub fn  tcp_pcb_remove(struct tcp_pcb **pcblist, pcb: &mut tcp_pcb);

pub fn  tcp_segs_free(seg: &mut tcp_seg);
pub fn  tcp_seg_free(seg: &mut tcp_seg);
tcp_seg_copy: &mut tcp_seg(seg: &mut tcp_seg);

#define tcp_ack(pcb)                               \
  loop {                                             \
    if((pcb).flags & TF_ACK_DELAY) {              \
      tcp_clear_flags(pcb, TF_ACK_DELAY);          \
      tcp_ack_now(pcb);                            \
    }                                              \
    else {                                         \
      tcp_set_flags(pcb, TF_ACK_DELAY);            \
    }                                              \
  } while (0)

#define tcp_ack_now(pcb)                           \
  tcp_set_flags(pcb, TF_ACK_NOW)

pub fn  tcp_send_fin(pcb: &mut tcp_pcb);
pub fn  tcp_enqueue_flags(pcb: &mut tcp_pcb, flags: u8);

pub fn  tcp_rexmit_seg(pcb: &mut tcp_pcb, seg: &mut tcp_seg);

pub fn  tcp_rst(const struct tcp_pcb* pcb, seqno: u32, ackno: u32,
       const local_ip: &mut LwipAddr,  remote_ip: &mut LwipAddr,
       local_port: u16, remote_port: u16);

tcp_next_iss: u32(pcb: &mut tcp_pcb);

pub fn  tcp_keepalive(pcb: &mut tcp_pcb);
pub fn  tcp_split_unsent_seg(pcb: &mut tcp_pcb, split: u16);
pub fn  tcp_zero_window_probe(pcb: &mut tcp_pcb);
pub fn   tcp_trigger_input_pcb_close();


tcp_eff_send_mss_netif: u16(sendmss: u16, outif: &mut NetIfc,
                             const dest: &mut LwipAddr);
#define tcp_eff_send_mss(sendmss, src, dest) \
    tcp_eff_send_mss_netif(sendmss, ip_route(src, dest), dest)



pub fn  tcp_recv_null(arg: &mut Vec<u8>, pcb: &mut tcp_pcb, p: &mut pbuf, err: err_t);



pub fn  tcp_debug_print(tcphdr: &mut tcp_hdr);
pub fn  tcp_debug_print_flags(flags: u8);
pub fn  tcp_debug_print_state(s: tcp_state);
pub fn  tcp_debug_print_pcbs();
i16 tcp_pcbs_sane();

#  define tcp_debug_print(tcphdr)
#  define tcp_debug_print_flags(flags)
#  define tcp_debug_print_state(s)
#  define tcp_debug_print_pcbs()
#  define tcp_pcbs_sane() 1


/* External function (implemented in timers.c), called when TCP detects
 * that a timer is needed (i.e. active- or time-wait-pcb found). */
pub fn  tcp_timer_needed();

pub fn  tcp_netif_ip_addr_changed(const old_addr: &mut LwipAddr,  new_addr: &mut LwipAddr);


pub fn  tcp_free_ooseq(pcb: &mut tcp_pcb);



pub fn  tcp_ext_arg_invoke_callbacks_passive_open(lpcb: &mut tcp_pcb_listen, cpcb: &mut tcp_pcb);



}





