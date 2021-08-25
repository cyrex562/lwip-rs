/*
 * @file
 * TCP API (to be used from TCPIP thread)\n
 * See also @ref tcp_raw
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

// #define LWIP_HDR_TCP_H


















struct tcp_pcb;
struct tcp_pcb_listen;

/* Function prototype for tcp accept callback functions. Called when a new
 * connection can be accepted on a listening pcb.
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param newpcb The new connection pcb
 * @param err An error code if there has been an error accepting.
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 */
typedef err_t (*tcp_accept_fn)(arg: &mut Vec<u8>, newpcb: &mut tcp_pcb, err: err_t);

/* Function prototype for tcp receive callback functions. Called when data has
 * been received.
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param tpcb The connection pcb which received data
 * @param p The received data (or NULL when the connection has been closed!)
 * @param err An error code if there has been an error receiving
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 */
typedef err_t (*tcp_recv_fn)(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb,
                             p: &mut pbuf, err: err_t);

/* Function prototype for tcp sent callback functions. Called when sent data has
 * been acknowledged by the remote side. Use it to free corresponding resources.
 * This also means that the pcb has now space available to send new data.
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param tpcb The connection pcb for which data has been acknowledged
 * @param len The amount of bytes acknowledged
 * @return ERR_OK: try to send some data by calling tcp_output
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 */
typedef err_t (*tcp_sent_fn)(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb,
                              len: u16);

/* Function prototype for tcp poll callback functions. Called periodically as
 * specified by @see tcp_poll.
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param tpcb tcp pcb
 * @return ERR_OK: try to send some data by calling tcp_output
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 */
typedef err_t (*tcp_poll_fn)(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb);

/* Function prototype for tcp error callback functions. Called when the pcb
 * receives a RST or is unexpectedly closed for any other reason.
 *
 * @note The corresponding pcb is already freed when this callback is called!
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param err Error code to indicate why the pcb has been closed
 *            ERR_ABRT: aborted through tcp_abort or by a TCP timer
 *            ERR_RST: the connection was reset by the remote host
 */
typedef void  (*tcp_err_fn)(arg: &mut Vec<u8>, err: err_t);

/* Function prototype for tcp connected callback functions. Called when a pcb
 * is connected to the remote side after initiating a connection attempt by
 * calling tcp_connect().
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param tpcb The connection pcb which is connected
 * @param err An unused error code, always ERR_OK currently ;-) @todo!
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 *
 * @note When a connection attempt fails, the error callback is currently called!
 */
typedef err_t (*tcp_connected_fn)(arg: &mut Vec<u8>, tpcb: &mut tcp_pcb, err: err_t);


#define RCV_WND_SCALE(pcb, wnd) (((wnd) >> (pcb).rcv_scale))
#define SND_WND_SCALE(pcb, wnd) (((wnd) << (pcb).snd_scale))
#define TCPWND16(x)             (LWIP_MIN((x), 0xFFFF))
#define TCP_WND_MAX(pcb)        ((tcpwnd_usize)(((pcb).flags & TF_WND_SCALE) ? TCP_WND : TCPWND16(TCP_WND)))

#define RCV_WND_SCALE(pcb, wnd) (wnd)
#define SND_WND_SCALE(pcb, wnd) (wnd)
#define TCPWND16(x)             (x)
#define TCP_WND_MAX(pcb)        TCP_WND

/* Increments a tcpwnd_and: usize holds at max value rather than rollover */
#define TCP_WND_INC(wnd, inc)   loop { \
                                  if ((tcpwnd_usize)(wnd + inc) >= wnd) { \
                                    wnd = (tcpwnd_usize)(wnd + inc); \
                                  } else { \
                                    wnd = (tcpwnd_usize)-1; \
                                  } \
                                } while(0)


/* SACK ranges to include in ACK packets.
 * SACK entry is invalid if left==right. */
struct tcp_sack_range {
  /* Left edge of the SACK: the first acknowledged sequence number. */
  let left: u32;
  /* Right edge of the SACK: the last acknowledged sequence number +1 (so first NOT acknowledged). */
  let right: u32;
};


/* Function prototype for deallocation of arguments. Called *just before* the
 * pcb is freed, so don't expect to be able to do anything with this pcb!
 *
 * @param id ext arg id (allocated via @ref tcp_ext_arg_alloc_id)
 * @param data pointer to the data (set via @ref tcp_ext_arg_set before)
 */
typedef void (*tcp_extarg_callback_pcb_destroyed_fn)(id: u8, data: &mut ());

/* Function prototype to transition arguments from a listening pcb to an accepted pcb
 *
 * @param id ext arg id (allocated via @ref tcp_ext_arg_alloc_id)
 * @param lpcb the listening pcb accepting a connection
 * @param cpcb the newly allocated connection pcb
 * @return ERR_OK if OK, any error if connection should be dropped
 */
typedef err_t (*tcp_extarg_callback_passive_open_fn)(id: u8, lpcb: &mut tcp_pcb_listen, cpcb: &mut tcp_pcb);

/* A table of callback functions that is invoked for ext arguments */
struct tcp_ext_arg_callbacks {
  /* @ref tcp_extarg_callback_pcb_destroyed_fn */
  tcp_extarg_callback_pcb_destroyed_fn destroy;
  /* @ref tcp_extarg_callback_passive_open_fn */
  tcp_extarg_callback_passive_open_fn passive_open;
};

pub const LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID: u32 = 0xFF;


/* This is the structure for ext args in tcp pcbs (used as array) */
struct tcp_pcb_ext_args {
  const callbacks: &mut tcp_ext_arg_callbacks;
  data: &mut ();
};
/* This is a helper define to prevent zero size arrays if disabled */
#define TCP_PCB_EXTARGS struct tcp_pcb_ext_args ext_args[LWIP_TCP_PCB_NUM_EXT_ARGS];

#define TCP_PCB_EXTARGS


typedef tcpflags_t: u16;
pub const TCP_ALLFLAGS: u32 = 0xffff;U

/*
 * members common to struct tcp_pcb and struct tcp_listen_pcb
 */
#define TCP_PCB_COMMON(type) \
  type *next; /* for the linked list */ \
  callback_arg: &mut (); \
  TCP_PCB_EXTARGS \
  state: tcp_state; /* TCP state */ \
  let prio: u8; \
  /* ports are in host byte order */ \
  local_port: u16


/* the TCP protocol control block for listening pcbs */
struct tcp_pcb_listen {
/* Common members of all PCB types */
  IP_PCB;
/* Protocol specific PCB members */
  TCP_PCB_COMMON(struct tcp_pcb_listen);


  /* Function to call when a listener has been connected. */
  tcp_accept_fn accept;



  let backlog: u8;
  let accepts_pending: u8;

};


/* the TCP protocol control block */
struct tcp_pcb {
/* common PCB members */
  IP_PCB;
/* protocol specific PCB members */
  TCP_PCB_COMMON(struct tcp_pcb);

  /* ports are in host byte order */
  let remote_port: u16;

  tcpflags_t flags;
pub const TF_ACK_DELAY: u32 = 0x01;U   /* Delayed ACK. */pub const TF_ACK_DELAY: u32 = 0x01;pub const TF_ACK_DELAY: u32 = 0x01;pub const TF_ACK_DELAY: u32 = 0x01;pub const TF_ACK_DELAY: u32 = 0x01;pub const TF_ACK_DELAY: u32 = 0x01;pub const TF_ACK_DELAY: u32 = 0x01;pub const TF_ACK_DELAY: u32 = 0x01;
pub const TF_ACK_NOW: u32 = 0; x02   /* Immediate ACK. */pub const TF_ACK_NOW: u32 = 0; pub const TF_ACK_NOW: u32 = 0; pub const TF_ACK_NOW: u32 = 0; pub const TF_ACK_NOW: u32 = 0; pub const TF_ACK_NOW: u32 = 0; pub const TF_ACK_NOW: u32 = 0; 
#define TF_INFR        0x04   /* In fast recovery. */
#define TF_CLOSEPEND   0x08   /* If this is set, tcp_close failed to enqueue the FIN (retried in tcp_tmr) */
#define TF_RXCLOSED    0x10   /* rx closed by tcp_shutdown */
#define TF_FIN         0x20   /* Connection was closed locally (FIN segment enqueued). */
#define TF_NODELAY     0x40   /* Disable Nagle algorithm */
#define TF_NAGLEMEMERR 0x80   /* nagle enabled, memerr, try to output to prevent delayed ACK to happen */

pub const TF_WND_SCALE: u32 = 0x0100;U /* Window Scale option enabled */


pub const TF_BACKLOGPEND: u32 = 0x0200;U /* If this is set, a connection pcb has increased the backlog on its listener */


pub const TF_TIMESTAMP: u32 = 0x0400;U   /* Timestamp option enabled */

pub const TF_RTO: u32 = 0x0800;U /* RTO timer has fired, in-flight data moved to unsent and being retransmitted */

pub const TF_SACK: u32 = 0x1000;U /* Selective ACKs enabled */


  /* the rest of the fields are in host byte order
     as we have to do some math with them */

  /* Timers */
  polltmr: u8, pollinterval;
  let last_timer: u8;
  let tmr: u32;

  /* receiver variables */
  let rcv_nxt: u32;   /* next seqno expected */
  let tcpwnd_rcv_wnd: usize;   /* receiver window available */  let tcpwnd_rcv_wnd: usize;
  tcpwnd_rcv_ann_wnd: usize; /* receiver window to announce */
  let rcv_ann_right_edge: u32; /* announced right edge of window */


  /* SACK ranges to include in ACK packets (entry is invalid if left==right) */
  struct tcp_sack_range rcv_sacks[LWIP_TCP_MAX_SACK_NUM];
// #define LWIP_TCP_SACK_VALID(pcb, idx) ((pcb).rcv_sacks[idx].left != (pcb).rcv_sacks[idx].right)


  /* Retransmission timer. */
  rtime: i16;

  let mss: u16;   /* maximum segment size */

  /* RTT (round trip time) estimation variables */
  let rttest: u32; /* RTT estimate in 500ms ticks */  let rttest: u32;
  rtseq: u32;  /* sequence number being timed */
  i16 sa, sv; /* @see "Congestion Avoidance and Control" by Van Jacobson and Karels */

  rto: i16;    /* retransmission time-out (in ticks of TCP_SLOW_INTERVAL) */
  let nrtx: u8;    /* number of retransmissions */

  /* fast retransmit/recovery */
  let dupacks: u8;
  let lastack: u32; /* Highest acknowledged seqno. */

  /* congestion avoidance/control variables */
  let tcpwnd_cwnd: usize;
  let tcpwnd_ssthresh: usize;

  /* first byte following last rto byte */
  let rto_end: u32;

  /* sender variables */
  let snd_nxt: u32;   /* next new seqno to be sent */
  snd_wl1: u32, snd_wl2; /* Sequence and acknowledgement numbers of last
                             window update. */
  let snd_lbb: u32;       /* Sequence number of next byte to be buffered. */
  let tcpwnd_snd_wnd: usize;   /* sender window */  let tcpwnd_snd_wnd: usize;  let tcpwnd_snd_wnd: usize;ize; /* the maximum sender window announced by the remote host */

  tcpwnd_snd_buf: usize;   /* Available buffer space for sending (in bytes). */
#define TCP_SNDQUEUELEN_OVERFLOW (0xffffU-3)
  let snd_queuelen: u16; /* Number of pbufs currently in the send buffer. */


  /* Extra bytes available at the end of the last pbuf in unsent. */
  let unsent_oversize: u16;


  let tcpwnd_bytes_acked: usize;

  /* These are ordered by sequence number: */
  unsent: &mut tcp_seg;   /* Unsent (queued) segments. */
  unacked: &mut tcp_seg;  /* Sent but unacknowledged segments. */

  ooseq: &mut tcp_seg;    /* Received out of sequence segments. */


  let refused_data: &mut pbuf; /* Data previously received but not yet taken by upper layer */


  struct tcp_pcb_listen* listener;



  /* Function to be called when more send buffer space is available. */
  tcp_sent_fn sent;
  /* Function to be called when (in-sequence) data has arrived. */
  tcp_recv_fn recv;
  /* Function to be called when a connection has been set up. */
  tcp_connected_fn connected;
  /* Function which is called periodically. */
  tcp_poll_fn poll;
  /* Function to be called whenever a fatal error occurs. */
  tcp_err_fn errf;



  let ts_lastacksent: u32;
  let ts_recent: u32;


  /* idle time before KEEPALIVE is sent */
  let keep_idle: u32;

  let keep_intvl: u32;
  let keep_cnt: u32;


  /* Persist timer counter */
  let persist_cnt: u8;
  /* Persist timer back-off */
  let persist_backoff: u8;
  /* Number of persist probes */
  let persist_probe: u8;

  /* KEEPALIVE counter */
  let keep_cnt_sent: u8;


  let snd_scale: u8;
  let rcv_scale: u8;

};



enum lwip_event {
  LWIP_EVENT_ACCEPT,
  LWIP_EVENT_SENT,
  LWIP_EVENT_RECV,
  LWIP_EVENT_CONNECTED,
  LWIP_EVENT_POLL,
  LWIP_EVENT_ERR
};

pub fn  lwip_tcp_event(arg: &mut Vec<u8>, pcb: &mut tcp_pcb,
         enum lwip_event,
         p: &mut pbuf,
         size: u16,
         err: err_t);



/* Application program's interface: */
struct tcp_pcb * tcp_new     ();
struct tcp_pcb * tcp_new_ip_type (type: u8);

pub fn              tcp_arg     (pcb: &mut tcp_pcb, arg: &mut Vec<u8>);

pub fn              tcp_recv    (pcb: &mut tcp_pcb, tcp_recv_fn recv);
pub fn              tcp_sent    (pcb: &mut tcp_pcb, tcp_sent_fn sent);
pub fn              tcp_err     (pcb: &mut tcp_pcb, tcp_err_fn err);
pub fn              tcp_accept  (pcb: &mut tcp_pcb, tcp_accept_fn accept);

pub fn              tcp_poll    (pcb: &mut tcp_pcb, tcp_poll_fn poll, interval: u8);

#define          tcp_set_flags(pcb, set_flags)     loop { (pcb).flags = (tcpflags_t)((pcb).flags |  (set_flags)); } while(0)
#define          tcp_clear_flags(pcb, clr_flags)   loop { (pcb).flags = (tcpflags_t)((pcb).flags & (tcpflags_t)(!(clr_flags) & TCP_ALLFLAGS)); } while(0)
#define          tcp_is_flag_set(pcb, flag)        (((pcb).flags & (flag)) != 0)


#define          tcp_mss(pcb)             (((pcb).flags & TF_TIMESTAMP) ? (pcb.mss - 12)  : pcb.mss)
 /* LWIP_TCP_TIMESTAMPS */
/* @ingroup tcp_raw */
#define          tcp_mss(pcb)             (pcb.mss)

/* @ingroup tcp_raw */
#define          tcp_sndbuf(pcb)          (TCPWND16((pcb).snd_buf))
/* @ingroup tcp_raw */
#define          tcp_sndqueuelen(pcb)     ((pcb).snd_queuelen)
/* @ingroup tcp_raw */
#define          tcp_nagle_disable(pcb)   tcp_set_flags(pcb, TF_NODELAY)
/* @ingroup tcp_raw */
#define          tcp_nagle_enable(pcb)    tcp_clear_flags(pcb, TF_NODELAY)
/* @ingroup tcp_raw */
#define          tcp_nagle_disabled(pcb)  tcp_is_flag_set(pcb, TF_NODELAY)


#define          tcp_backlog_set(pcb, new_backlog) loop { \
  LWIP_ASSERT("pcb.state == LISTEN (called for wrong pcb?)", pcb.state == LISTEN); \
  ((struct tcp_pcb_listen *)(pcb)).backlog = ((new_backlog) ? (new_backlog) : 1); } while(0)
pub fn              tcp_backlog_delayed(struct tcp_pcb* pcb);
pub fn              tcp_backlog_accepted(struct tcp_pcb* pcb);
  /* TCP_LISTEN_BACKLOG */
#define          tcp_backlog_set(pcb, new_backlog)
#define          tcp_backlog_delayed(pcb)
#define          tcp_backlog_accepted(pcb)

#define          tcp_accepted(pcb) loop {  } while(0) /* compatibility define, not needed any more */

pub fn              tcp_recved  (pcb: &mut tcp_pcb, len: u16);
pub fn             tcp_bind    (pcb: &mut tcp_pcb,  ipaddr: &mut ip_addr_t,
                              port: u16);
pub fn              tcp_bind_netif(pcb: &mut tcp_pcb,  netif: &mut NetIfc);
pub fn             tcp_connect (pcb: &mut tcp_pcb,  ipaddr: &mut ip_addr_t,
                              port: u16, tcp_connected_fn connected);

struct tcp_pcb * tcp_listen_with_backlog_and_err(pcb: &mut tcp_pcb, backlog: u8, err: &mut err_t);
struct tcp_pcb * tcp_listen_with_backlog(pcb: &mut tcp_pcb, backlog: u8);
/* @ingroup tcp_raw */
#define          tcp_listen(pcb) tcp_listen_with_backlog(pcb, TCP_DEFAULT_LISTEN_BACKLOG)

pub fn              tcp_abort (pcb: &mut tcp_pcb);
pub fn             tcp_close   (pcb: &mut tcp_pcb);
pub fn             tcp_shutdown(pcb: &mut tcp_pcb, shut_rx: i32, shut_tx: i32);

pub fn             tcp_write   (pcb: &mut tcp_pcb, dataptr: &Vec<u8>, len: u16,
                              apiflags: u8);

pub fn              tcp_setprio (pcb: &mut tcp_pcb, prio: u8);

pub fn             tcp_output  (pcb: &mut tcp_pcb);

pub fn             tcp_tcp_get_tcp_addrinfo(pcb: &mut tcp_pcb, local: i32, addr: &mut ip_addr_t, port: &mut u16);

#define tcp_dbg_get_tcp_state(pcb) (pcb.state)

/* for compatibility with older implementation */
#define tcp_new_ip6() tcp_new_ip_type(IPADDR_TYPE_V6)


tcp_ext_arg_alloc_id: u8();
pub fn  tcp_ext_arg_set_callbacks(pcb: &mut tcp_pcb, uint8_t id,  struct tcp_ext_arg_callbacks * const callbacks);
pub fn  tcp_ext_arg_set(pcb: &mut tcp_pcb, uint8_t id, arg: &mut Vec<u8>);
pub fn  *tcp_ext_arg_get(const pcb: &mut tcp_pcb, uint8_t id);



}





