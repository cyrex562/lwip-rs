use crate::core::error::LwipError;
use crate::core::options::LWIP_TCP_PCB_NUM_EXT_ARGS;
use crate::ip::ip2_h::IpContext;
use crate::packetbuffer::pbuf_h::PacketBuffer;
use crate::tcp::tcp_priv_h::tcp_seg;
use crate::tcp::tcpbase_h::TcpState;

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

//

// struct tcp_pcb;
// struct tcp_pcb_listen;

/* Function prototype for tcp accept callback functions. Called when a new
 * connection can be accepted on a listening pcb.
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param newpcb The new connection pcb
 * @param err An error code if there has been an error accepting.
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 */
// typedef err_t (*tcp_accept_fn)(arg: &mut Vec<u8>, newpcb: &mut TcpContext, err: err_t);
pub type tcp_accept_fn = fn(arg: &mut Vec<u8>, newpcb: &mut TcpContext, err: err_t) -> err_t;

/// Function prototype for tcp receive callback functions. Called when data has  been received.
/// @param arg Additional argument to pass to the callback function (@see tcp_arg())
/// @param tpcb The connection pcb which received data
/// @param p The received data (or NULL when the connection has been closed!)
/// @param err An error code if there has been an error receiving
/// Only return ERR_ABRT if you have called tcp_abort from within the callback function!
pub type TcpDataReceivedFunc =
    fn(arg: &mut Vec<u8>, tpcb: &mut TcpContext, p: &mut PacketBuffer) -> Result<(), LwipError>;

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
pub type tcp_sent_fn = fn(arg: &mut Vec<u8>, tpcb: &mut TcpContext, len: usize) -> err_t;

/* Function prototype for tcp poll callback functions. Called periodically as
 * specified by @see tcp_poll.
 *
 * @param arg Additional argument to pass to the callback function (@see tcp_arg())
 * @param tpcb tcp pcb
 * @return ERR_OK: try to send some data by calling tcp_output
 *            Only return ERR_ABRT if you have called tcp_abort from within the
 *            callback function!
 */
pub type tcp_poll_fn = fn(arg: &mut Vec<u8>, tpcb: &mut TcpContext) -> err_t;

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
pub type tcp_err_fn = fn(arg: &mut Vec<u8>, err: err_t);

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
pub type tcp_connected_fn = fn(arg: &mut Vec<u8>, tpcb: &mut TcpContext, err: err_t);

pub fn RCV_WND_SCALE(pcb: &TcpContext, wnd: u16) -> u16 {
    ((wnd) >> (pcb).rcv_scale)
}
pub fn SND_WND_SCALE(pcb: &TcpContext, wnd: u16) -> u16 {
    ((wnd) << (pcb).snd_scale)
}
pub fn TCPWND16(x: u16) -> u16 {
    (LWIP_MIN((x), 0xFFFF))
}
// pub fn TCP_WND_MAX(pcb: &tcp_pcb) -> u16{        ((((pcb).flags & TF_WND_SCALE) ? TCP_WND : TCPWND16(TCP_WND)))}

pub fn RCV_WND_SCALE(pcb: &TcpContext, wnd: u16) -> u16 {
    (wnd)
}
pub fn SND_WND_SCALE(pcb: &TcpContext, wnd: u16) -> u16 {
    (wnd)
}
pub fn TCPWND16(x: u16) -> u16 {
    (x)
}
pub fn TCP_WND_MAX(pcb: &mut TcpContext) -> u16 {
    TCP_WND
}

//  Increments a tcpwnd_and: usize holds at max value rather than rollover
pub fn TCP_WND_INC(wnd: u16, inc: u16) -> u16 {
    if ((wnd + inc) >= wnd) {
        wnd = (wnd + inc);
    } else {
        wnd = -1;
    }
    wnd
}

/* SACK ranges to include in ACK packets.
 * SACK entry is invalid if left==right. */
pub struct tcp_sack_range {
    //  Left edge of the SACK: the first acknowledged sequence number.
    pub left: u32,
    //  Right edge of the SACK: the last acknowledged sequence number +1 (so first NOT acknowledged).
    pub right: u32,
}

/* Function prototype for deallocation of arguments. Called *just before* the
 * pcb is freed, so don't expect to be able to do anything with this pcb!
 *
 * @param id ext arg id (allocated via @ref tcp_ext_arg_alloc_id)
 * @param data pointer to the data (set via @ref tcp_ext_arg_set before)
 */
pub type tcp_extarg_callback_pcb_destroyed_fn = fn(id: u8, data: &mut Vec<u8>);

/* Function prototype to transition arguments from a listening pcb to an accepted pcb
 *
 * @param id ext arg id (allocated via @ref tcp_ext_arg_alloc_id)
 * @param lpcb the listening pcb accepting a connection
 * @param cpcb the newly allocated connection pcb
 * @return ERR_OK if OK, any error if connection should be dropped
 */
pub type tcp_extarg_callback_passive_open_fn =
    fn(id: u8, lpcb: &mut TcpListenContext, cpcb: &mut TcpContext) -> err_t;

//  A table of callback functions that is invoked for ext arguments
pub struct tcp_ext_arg_callbacks {
    //  @ref tcp_extarg_callback_pcb_destroyed_fn
    pub destroy: tcp_extarg_callback_pcb_destroyed_fn,
    //  @ref tcp_extarg_callback_passive_open_fn
    pub passive_open: tcp_extarg_callback_passive_open_fn,
}

pub const LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID: u32 = 0xFF;

//  This is the structure for ext args in tcp pcbs (used as array)
pub struct tcp_pcb_ext_args {
    pub callbacks: &mut tcp_ext_arg_callbacks,
    pub data: Vec<u8>,
}
//  This is a helper define to prevent zero size arrays if disabled
// pub const TCP_PCB_EXTARGS: u32 = struct; tcp_pcb_ext_args ext_args[LWIP_TCP_PCB_NUM_EXT_ARGS];

// #define TCP_PCB_EXTARGS

// typedef tcpflags_t: u16;
pub const TCP_ALLFLAGS: u32 = 0xffff;

/*
 * members common to struct tcp_pcb and struct tcp_listen_pcb
 */
// #define TCP_PCB_COMMON(type) \
//   type *next; //  for the linked list  \
//   callback_arg: &mut Vec<u8>; \
//   TCP_PCB_EXTARGS \
//   let state: tcp_state; //  TCP state  \
//   let prio: u8; \
//   //  ports are in host byte order  \
//   local_port: u16
pub struct TCP_PCB_COMMON {
    pub callback_arg: Vec<u8>,
    pub ext_args: [tcp_pcb_ext_args; LWIP_TCP_PCB_NUM_EXT_ARGS],
    pub state: tcp_state,
    pub prio: u8,
    pub local_port: u16,
}

//  the TCP protocol control block for listening pcbs
#[derive(Clone, Default, Debug)]
pub struct TcpListenContext {
    //  Common members of all PCB types
    pub ip_ctx: IpContext,
    //  Protocol specific PCB members
    pub callback_arg: Vec<u8>,
    pub ext_args: [tcp_pcb_ext_args; LWIP_TCP_PCB_NUM_EXT_ARGS],
    pub state: tcp_state,
    pub prio: u8,
    pub local_port: u16,
    pub backlog: u8,
    pub accepts_pending: u8,
}

pub const TF_ACK_DELAY: u32 = 0x01;
pub const TF_ACK_NOW: u32 = 0x02; //  Immediate ACK.
pub const TF_INFR: u32 = 0x04; //  In fast recovery.
pub const TF_CLOSEPEND: u32 = 0x08; //  If this is set, tcp_close failed to enqueue the FIN (retried in tcp_tmr)
pub const TF_RXCLOSED: u32 = 0x10; //  rx closed by tcp_shutdown
pub const TF_FIN: u32 = 0x20; //  Connection was closed locally (FIN segment enqueued).
pub const TF_NODELAY: u32 = 0x40; //  Disable Nagle algorithm
pub const TF_NAGLEMEMERR: u32 = 0x80; //  nagle enabled, memerr, try to output to prevent delayed ACK to happen
pub const TF_WND_SCALE: u32 = 0x100; //  Window Scale option enabled
pub const TF_BACKLOGPEND: u32 = 0x0200; //  If this is set, a connection pcb has increased the backlog on its listener
pub const TF_TIMESTAMP: u32 = 0x400; //  Timestamp option enabled
pub const TF_SACK: u32 = 0x1000; //  Selective ACKs enabled

pub const TF_RTO: u32 = 0x0800; //  RTO timer has fired, in-flight data moved to unsent and being retransmitted

pub const TCP_SNDQUEUELEN_OVERFLOW: u16 = (0xffff - 3);

// enum {
//     TCP_FLAG_CWR = __constant_cpu_to_be32(0x00800000),
//     TCP_FLAG_ECE = __constant_cpu_to_be32(0x00400000),
//     TCP_FLAG_URG = __constant_cpu_to_be32(0x00200000),
//     TCP_FLAG_ACK = __constant_cpu_to_be32(0x00100000),
//     TCP_FLAG_PSH = __constant_cpu_to_be32(0x00080000),
//     TCP_FLAG_RST = __constant_cpu_to_be32(0x00040000),
//     TCP_FLAG_SYN = __constant_cpu_to_be32(0x00020000),
//     TCP_FLAG_FIN = __constant_cpu_to_be32(0x00010000),
//     TCP_RESERVED_BITS = __constant_cpu_to_be32(0x0F000000),
//     TCP_DATA_OFFSET = __constant_cpu_to_be32(0xF0000000)
// };
pub enum TcpFlag {
    CWR = 0x00800000,
    ECE = 0x00400000,
    URG = 0x00200000,
    ACK = 0x00100000,
    PSH = 0x00080000,
    RST = 0x00040000,
    SYN = 0x00020000,
    FIN = 0x00010000,
    Reserved = 0x0F000000,
    DataOffset = 0xF0000000,
}

//  the TCP protocol control block
#[derive(Clone, Default, Debug)]
pub struct TcpContext {
    pub ip_ctx: IpContext,
    //  common PCB members
    //  protocol specific PCB members
    // pub tcp_pcb_common: TCP_PCB_COMMON,
    pub callback_arg: Vec<u8>,
    pub ext_args: [tcp_pcb_ext_args; LWIP_TCP_PCB_NUM_EXT_ARGS],
    pub state: TcpState,
    pub prio: u8,
    pub local_port: u16,
    //  ports are in host byte order
    pub remote_port: u16,
    // pub flags: [TcpFlag;10],
    pub flags: u32,
    //  Timers
    pub polltmr: u64,
    pub pollinterval: u64,
    pub last_timer: u64,
    pub tmr: u64,

    //  receiver variables
    pub rcv_nxt: u32,              //  next seqno expected
    pub tcpwnd_rcv_wnd: usize,     //  receiver window available
    pub tcpwnd_rcv_ann_wnd: usize, //  receiver window to announce
    pub rcv_ann_right_edge: u32,   //  announced right edge of window
    //  SACK ranges to include in ACK packets (entry is invalid if left==right)
    pub rcv_sacks: [tcp_sack_range; LWIP_TCP_MAX_SACK_NUM],
    // #define LWIP_TCP_SACK_VALID(pcb, idx) ((pcb).rcv_sacks[idx].left != (pcb).rcv_sacks[idx].right)

    //  Retransmission timer.
    pub rtime: i64,

    pub mss: u16, //  maximum segment size

    //  RTT (round trip time) estimation variables
    pub rttest: u32, //  RTT estimate in 500ms ticks
    pub rtseq: u32,  //  sequence number being timed
    pub sa: i16,
    pub sv: i16,  //  @see "Congestion Avoidance and Control" by Van Jacobson and Karels
    pub rto: i16, //  retransmission time-out (in ticks of TCP_SLOW_INTERVAL)
    pub nrtx: u8, //  number of retransmissions

    //  fast retransmit/recovery
    pub dupacks: u8,
    pub lastack: u32, //  Highest acknowledged seqno.

    //  congestion avoidance/control variables
    pub tcpwnd_cwnd: usize,
    pub tcpwnd_ssthresh: usize,

    //  first byte following last rto byte
    pub rto_end: u32,
    //  sender variables
    pub snd_nxt: u32, //  next new seqno to be sent
    pub snd_wl1: u32,
    pub snd_wl2: u32,          /* Sequence and acknowledgement numbers of last
                               window update. */
    pub snd_lbb: u32,          //  Sequence number of next byte to be buffered.
    pub tcpwnd_snd_wnd: usize, //  sender window

    //  the maximum sender window announced by the remote host
    pub tcpwnd_snd_buf: usize, //  Available buffer space for sending (in bytes).

    pub snd_queuelen: u16, //  Number of pbufs currently in the send buffer.

    //  Extra bytes available at the end of the last pbuf in unsent.
    pub unsent_oversize: u16,

    pub tcpwnd_bytes_acked: usize,

    //  These are ordered by sequence number:
    pub unsent: Vec<tcp_seg>, //  Unsent (queued) segments.
    //  Sent but unacknowledged segments.
    pub ooseq: Vec<tcp_seg>,        //  Received out of sequence segments.
    pub refused_data: PacketBuffer, //  Data previously received but not yet taken by upper layer
    // pub listener: tcp_pcb_listen,
    // //  Function to be called when more send buffer space is available.
    // pub sent: tcp_sent_fn,
    // //  Function to be called when (in-sequence) data has arrived.
    // pub recv: tcp_recv_fn,
    // //  Function to be called when a connection has been set up.
    // pub connected: tcp_connected_fn,
    // //  Function which is called periodically.
    // pub poll: tcp_poll_fn,
    // //  Function to be called whenever a fatal error occurs.
    // pub errf: tcp_err_fn,
    pub accept: tcp_accept_fn,
    pub ts_lastacksent: u32,
    pub ts_recent: u32,
    //  idle time before KEEPALIVE is sent
    pub keep_idle: u64,
    pub keep_intvl: u64,
    pub keep_cnt: u32,
    //  Persist timer counter
    pub persist_cnt: u8,
    //  Persist timer back-off
    pub persist_backoff: u8,
    //  Number of persist probes
    pub persist_probe: u8,

    //  KEEPALIVE counter
    pub keep_cnt_sent: u8,
    pub snd_scale: u8,
    pub rcv_scale: u8,
}

impl TcpContext {
    pub fn new() -> TcpContext {
        TcpContext {
            ..Default::default()
        }
    }
}

pub enum LwipEvent {
    LWIP_EVENT_ACCEPT,
    LWIP_EVENT_SENT,
    LWIP_EVENT_RECV,
    LWIP_EVENT_CONNECTED,
    LWIP_EVENT_POLL,
    LWIP_EVENT_ERR,
}

// pub fn  lwip_tcp_event(arg: &mut Vec<u8>, pcb: &mut TcpContext,
//          enum LwipEvent,
//          p: &mut PacketBuffer,
//          size: u16,
//          err: err_t);

//  Application program's interface:
// struct tcp_pcb * tcp_new     ();
// struct tcp_pcb * tcp_new_ip_type (type: u8);

// pub fn              tcp_arg     (pcb: &mut TcpContext, arg: &mut Vec<u8>);

// pub fn              tcp_recv    (pcb: &mut TcpContext, tcp_recv_fn recv);
// pub fn              tcp_sent    (pcb: &mut TcpContext, tcp_sent_fn sent);
// pub fn              tcp_err     (pcb: &mut TcpContext, tcp_err_fn err);
// pub fn              tcp_accept  (pcb: &mut TcpContext, tcp_accept_fn accept);

// pub fn              tcp_poll    (pcb: &mut TcpContext, tcp_poll_fn poll, interval: u8);

// #define          tcp_set_flags(pcb, set_flags)     loop { (pcb).flags = (tcpflags_t)((pcb).flags |  (set_flags)); } while(0)
pub fn tcp_set_flags(tcp_ctx: &mut TcpContext, set_flags: u32) -> Result<(), LwipError> {
    tcp_ctx.flags = &tcp_ctx.flags | set_flags;
    Ok(())
}
// #define          tcp_clear_flags(pcb, clr_flags)   loop { (pcb).flags = (tcpflags_t)((pcb).flags & (tcpflags_t)(!(clr_flags) & TCP_ALLFLAGS)); } while(0)
pub fn tcp_clear_flags(tcp_ctx: &mut TcpContext, flags_to_clear: u32) -> Result<(), LwipError> {
    tcp_ctx.flags = &tcp_ctx.flags & (!flags_to_clear & TCP_ALLFLAGS);
    Ok(())
}
// #define          tcp_is_flag_set(pcb, flag)        (((pcb).flags & (flag)) != 0)
pub fn tcp_is_flag_set(ctx: &mut TcpContext, flag: u32) -> Result<bool, LwipError> {
    Ok(ctx.flags & flag != 0)
}

// #define          tcp_mss(pcb)             (((pcb).flags & TF_TIMESTAMP) ? (pcb.mss - 12)  : pcb.mss)
//  LWIP_TCP_TIMESTAMPS
//  @ingroup tcp_raw

//  @ingroup tcp_raw
// #define          tcp_sndbuf(pcb)          (TCPWND16((pcb).snd_buf))
//  @ingroup tcp_raw
// #define          tcp_sndqueuelen(pcb)     ((pcb).snd_queuelen)
//  @ingroup tcp_raw
// #define          tcp_nagle_disable(pcb)   tcp_set_flags(pcb, TF_NODELAY)
//  @ingroup tcp_raw
// #define          tcp_nagle_enable(pcb)    tcp_clear_flags(pcb, TF_NODELAY)
//  @ingroup tcp_raw
// #define          tcp_nagle_disabled(pcb)  tcp_is_flag_set(pcb, TF_NODELAY)
pub fn tcp_nagle_disabled(ctx: &mut TcpContext) -> Result<bool, LwipError> {
    tcp_is_flag_set(ctx, TF_NODELAY)
}

// #define          tcp_backlog_set(pcb, new_backlog) loop { \
//   LWIP_ASSERT("pcb.state == LISTEN (called for wrong pcb?)", pcb.state == LISTEN); \
//   ((pcb)).backlog = ((new_backlog) ? (new_backlog) : 1); } while(0)
// pub fn              tcp_backlog_delayed(struct tcp_pcb* pcb);
// pub fn              tcp_backlog_accepted(struct tcp_pcb* pcb);
//  TCP_LISTEN_BACKLOG
// #define          tcp_backlog_set(pcb, new_backlog)
// #define          tcp_backlog_delayed(pcb)
// #define          tcp_backlog_accepted(pcb)

// #define          tcp_accepted(pcb) loop {  } while(0) //  compatibility define, not needed any more
// pub fn              tcp_recved  (pcb: &mut TcpContext, len: usize);
// pub fn             tcp_bind    (pcb: &mut TcpContext,  ipaddr: &mut LwipAddr,                               port: u16);
// pub fn              tcp_bind_netif(pcb: &mut TcpContext,  netif: &mut NetIfc);
// pub fn             tcp_connect (pcb: &mut TcpContext,  ipaddr: &mut LwipAddr,                               port: u16, tcp_connected_fn connected);

// struct tcp_pcb * tcp_listen_with_backlog_and_err(pcb: &mut TcpContext, backlog: u8, err: &mut err_t);
// struct tcp_pcb * tcp_listen_with_backlog(pcb: &mut TcpContext, backlog: u8);
//  @ingroup tcp_raw
// #define          tcp_listen(pcb) tcp_listen_with_backlog(pcb, TCP_DEFAULT_LISTEN_BACKLOG)

// pub fn              tcp_abort (pcb: &mut TcpContext);
// pub fn             tcp_close   (pcb: &mut TcpContext);
// pub fn             tcp_shutdown(pcb: &mut TcpContext, shut_rx: i32, shut_tx: i32);

// pub fn             tcp_write   (pcb: &mut TcpContext, dataptr: &Vec<u8>, len: usize,apiflags: u8);

// pub fn              tcp_setprio (pcb: &mut TcpContext, prio: u8);

// pub fn             tcp_output  (pcb: &mut TcpContext);

// pub fn             tcp_tcp_get_tcp_addrinfo(pcb: &mut TcpContext, local: i32, addr: &mut LwipAddr, port: &mut u16);

// #define tcp_dbg_get_tcp_state(pcb) (pcb.state)

//  for compatibility with older implementation
// #define tcp_new_ip6() tcp_new_ip_type(IPADDR_TYPE_V6)

// tcp_ext_arg_alloc_id: u8();
// pub fn  tcp_ext_arg_set_callbacks(pcb: &mut TcpContext, id: u8,  struct tcp_ext_arg_callbacks * const callbacks);
// pub fn  tcp_ext_arg_set(pcb: &mut TcpContext, id: u8, arg: &mut Vec<u8>);
// pub fn  *tcp_ext_arg_get( pcb: &mut TcpContext, id: u8);
