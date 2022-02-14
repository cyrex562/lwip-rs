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

//

//  Functions for interfacing with TCP:

//  Lower layer interface to TCP:
// pub fn              tcp_init    ();  //  Initialize this module.
// pub fn              tcp_tmr     ();
/* Must be called every
TCP_TMR_INTERVAL
ms. (Typically 250 ms). */
/* It is also possible to call these two functions at the right
intervals (instead of calling tcp_tmr()). */
// pub fn              tcp_slowtmr ();
// pub fn              tcp_fasttmr ();

/* Call this from a netif driver (watch out for threading issues!) that has
returned a memory error on transmit and now has free buffers to send more.
This iterates all active pcbs that had an error and tries to call
tcp_output, so use this with care as it might slow down the system. */
// pub fn              tcp_txnow   ();

//  Only used by IP to pass a TCP segment to TCP:
// pub fn              tcp_input   (p: &mut PacketBuffer, inp: &mut NetIfc);
//  Used within the TCP code only:
// struct tcp_pcb * tcp_alloc   (prio: u8);
// pub fn              tcp_free    (pcb: &mut TcpContext);
// pub fn              tcp_abandon (pcb: &mut TcpContext, reset: i32);
// pub fn             tcp_send_empty_ack(pcb: &mut TcpContext);
// pub fn             tcp_rexmit  (pcb: &mut TcpContext);
// pub fn             tcp_rexmit_rto_prepare(pcb: &mut TcpContext);
// pub fn              tcp_rexmit_rto_commit(pcb: &mut TcpContext);
// pub fn              tcp_rexmit_rto  (pcb: &mut TcpContext);
// pub fn              tcp_rexmit_fast (pcb: &mut TcpContext);
// u32            tcp_update_rcv_ann_wnd(pcb: &mut TcpContext);
// pub fn             tcp_process_refused_data(pcb: &mut TcpContext);

/*
 * This is the Nagle algorithm: try to combine user data to send as few TCP
 * segments as possible. Only send if
 * - no previously transmitted data on the connection remains unacknowledged or
 * - the TF_NODELAY flag is set (nagle algorithm turned off for this pcb) or
 * - the only unsent segment is at least pcb.mss bytes long (or there is more
 *   than one unsent segment - with lwIP, this can happen although unsent.len < mss)
 * - or if we are in fast-retransmit (TF_INFR)
 */
use crate::core::common::lwip_htonl;
use crate::core::error::LwipError;
use crate::core::options::{CHECKSUM_GEN_TCP, LWIP_CHECKSUM_ON_COPY, TCP_SND_QUEUELEN};
use crate::tcp::tcp2::{tcp_pcb_remove, tcp_pcbs_sane};
use crate::tcp::tcp2_h::{
    tcp_set_flags, TcpContext, TF_ACK_DELAY, TF_ACK_NOW, TF_INFR, TF_NODELAY,
};
use crate::tcp::tcp_h::{TCPH_FLAGS, TCP_FIN, TCP_SYN};
use crate::tcp::tcpbase_h::TcpState::CLOSED;

pub fn tcp_do_output_nagle(tpcb: &tcp_pcb) {
    (((tpcb).unacked == None)
        || ((tpcb).flags & (TF_NODELAY | TF_INFR))
        || (((tpcb).unsent != None)
            && (((tpcb).unsent.next != None) || ((tpcb).unsent.len >= tpcb.mss)))
        || ((tcp_sndbuf(tpcb) == 0) || (tcp_sndqueuelen(tpcb) >= TCP_SND_QUEUELEN)))
}
// pub fn tcp_output_nagle(tpcb: &tcp_pcb){ (tcp_do_output_nagle(tpcb) ? tcp_output(tpcb) : ERR_OK)}

// #define TCP_SEQ_LT(a,b)     (((a) - (b)) < 0)
// #define TCP_SEQ_LEQ(a,b)    (((a) - (b)) <= 0)
// #define TCP_SEQ_GT(a,b)     (((a) - (b)) > 0)
// #define TCP_SEQ_GEQ(a,b)    (((a) - (b)) >= 0)
//  is b<=a<=c?

// #define TCP_SEQ_BETWEEN(a,b,c) ((c)-(b) >= (a)-(b))

// #define TCP_SEQ_BETWEEN(a,b,c) (TCP_SEQ_GEQ(a,b) && TCP_SEQ_LEQ(a,c))

pub const TCP_TMR_INTERVAL: u32 = 250; //  The TCP timer interval in milliseconds.

pub const TCP_FAST_INTERVAL: u32 = TCP_TMR_INTERVAL; //  the fine grained timeout in milliseconds

pub const TCP_SLOW_INTERVAL: u64 = (2 * TCP_TMR_INTERVAL); //  the coarse grained timeout in milliseconds

pub const TCP_FIN_WAIT_TIMEOUT: u32 = 20000; //  milliseconds
pub const TCP_SYN_RCVD_TIMEOUT: u32 = 20000; //  milliseconds
pub const TCP_SYN_RCVD_TIMEOUT: u32 = 20000;
pub const TCP_OOSEQ_TIMEOUT: u32 = 6; //  x RTO

pub const TCP_MSL: u32 = 60000; //  The maximum segment lifetime in milliseconds

//  Keepalive values, compliant with RFC 1122. Don't change this unless you know what you're doing

pub const TCP_KEEPIDLE_DEFAULT: u32 = 7200000; //  Default KEEPALIVE timer in milliseconds

pub const TCP_KEEPINTVL_DEFAULT: u32 = 75000; //  Default Time between KEEPALIVE probes in milliseconds

pub const TCP_KEEPCNT_DEFAULT: u32 = 9; //  Default Counter for KEEPALIVE probes

pub const TCP_MAXIDLE: u32 = TCP_KEEPCNT_DEFAULT * TCP_KEEPINTVL_DEFAULT; //  Maximum KEEPALIVE probe time

pub fn TCP_TCPLEN(seg: tcp_seg) {
    ((seg).len + ((TCPH_FLAGS((seg).tcphdr) & (TCP_FIN | TCP_SYN)) != 0))
}

/* Flags used on input processing, not on pcb.flags
*/
pub const TF_RESET: u32 = 0x08; //  Connection was reset.
pub const TF_CLOSED: u32 = 0x10; //  Connection was successfully closed.
pub const TF_GOT_FIN: u32 = 0x20; //  Connection was closed by the remote end.

// #define TCP_EVENT_ACCEPT(lpcb,pcb,arg,err,ret) ret = lwip_tcp_event(arg, (pcb),\
//                 LWIP_EVENT_ACCEPT, None, 0, err)
// #define TCP_EVENT_SENT(pcb,space,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
//                    LWIP_EVENT_SENT, None, space, ERR_OK)
// #define TCP_EVENT_RECV(pcb,p,err,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
//                 LWIP_EVENT_RECV, (p), 0, (err))
// #define TCP_EVENT_CLOSED(pcb,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
//                 LWIP_EVENT_RECV, None, 0, ERR_OK)
// #define TCP_EVENT_CONNECTED(pcb,err,ret) ret = lwip_tcp_event((pcb).callback_arg, (pcb),\
//                 LWIP_EVENT_CONNECTED, None, 0, (err))
// #define TCP_EVENT_POLL(pcb,ret)       loop { if (pcb.state != SYN_RCVD) {                          \
//                 ret = lwip_tcp_event((pcb).callback_arg, (pcb), LWIP_EVENT_POLL, None, 0, ERR_OK); \
//                 } else {                                                                            \
//                 ret = ERR_ARG; } } while(0)
/* For event API, last state SYN_RCVD must be excluded here: the application
has not seen this pcb, yet! */
// #define TCP_EVENT_ERR(last_state,errf,arg,err)  loop { if (last_state != SYN_RCVD) {                \
//                 lwip_tcp_event((arg), None, LWIP_EVENT_ERR, None, 0, (err)); } } while(0)

//  LWIP_EVENT_API

// #define TCP_EVENT_ACCEPT(lpcb,pcb,arg,err,ret)                 \
//   loop {                                                         \
//     if(lpcb.accept != None)                                 \
//       (ret) = lpcb.accept((arg),(pcb),(err));               \
//     else (ret) = ERR_ARG;                                      \
//   } while (0)

// #define TCP_EVENT_SENT(pcb,space,ret)                          \
//   loop {                                                         \
//     if((pcb).sent != None)                                    \
//       (ret) = (pcb).sent((pcb).callback_arg,(pcb),(space));  \
//     else (ret) = ERR_OK;                                       \
//   } while (0)

// #define TCP_EVENT_RECV(pcb,p,err,ret)                          \
//   loop {                                                         \
//     if(pcb.recv != None) {                                  \
//       (ret) = pcb.recv((pcb).callback_arg,(pcb),(p),(err));\
//     } else {                                                   \
//       (ret) = tcp_recv_None(None, (pcb), (p), (err));          \
//     }                                                          \
//   } while (0)

// #define TCP_EVENT_CLOSED(pcb,ret)                                \
//   loop {                                                           \
//     if((pcb.recv != None)) {                                  \
//       (ret) = pcb.recv((pcb).callback_arg,(pcb),None,ERR_OK);\
//     } else {                                                     \
//       (ret) = ERR_OK;                                            \
//     }                                                            \
//   } while (0)

// #define TCP_EVENT_CONNECTED(pcb,err,ret)                         \
//   loop {                                                           \
//     if(pcb.connected != None)                                 \
//       (ret) = pcb.connected((pcb).callback_arg,(pcb),(err)); \
//     else (ret) = ERR_OK;                                         \
//   } while (0)

// #define TCP_EVENT_POLL(pcb,ret)                                \
//   loop {                                                         \
//     if(pcb.poll != None)                                    \
//       (ret) = pcb.poll((pcb).callback_arg,(pcb));          \
//     else (ret) = ERR_OK;                                       \
//   } while (0)

// #define TCP_EVENT_ERR(last_state,errf,arg,err)                 \
//   loop {                                                         \
//                                    \
//     if((errf) != None)                                         \
//       (errf)((arg),(err));                                     \
//   } while (0)

//  Enabled extra-check for TCP_OVERSIZE if LWIP_DEBUG is enabled

pub const TCP_OVERSIZE_DBGCHECK: u32 = 1;

pub const TCP_OVERSIZE_DBGCHECK: u32 = 0;

//  Don't generate checksum on copy if CHECKSUM_GEN_TCP is disabled
pub const TCP_CHECKSUM_ON_COPY: bool = (LWIP_CHECKSUM_ON_COPY && CHECKSUM_GEN_TCP);

pub const TF_SEG_OPTS_MSS: u32 = 0x01; //  Include MSS option (only used in SYN segments)
pub const TF_SEG_OPTS_MSS: u32 = 0;
pub const TF_SEG_OPTS_TS: u32 = 0x02; //  Include timestamp option.
pub const TF_SEG_DATA_CHECKSUMMED: u32 = 0x04; /* ALL data (not the header) is
                                               checksummed into 'chksum' */
pub const TF_SEG_OPTS_WND_SCALE: u32 = 0x08; //  Include WND SCALE option (only used in SYN segments)
pub const TF_SEG_OPTS_WND_SCALE: u32 = 0;
pub const TF_SEG_OPTS_SACK_PERM: u32 = 0x10; //  Include SACK Permitted option (only used in SYN segments)

//  This structure represents a TCP segment on the unsent, unacked and ooseq queues
pub struct tcp_seg {
    // pub next: &mut tcp_seg,   //  used when putting segments on a queue
    pub p: &mut PacketBuffer, //  buffer containing data + TCP header
    pub len: usize,           //  the TCP length of this segment
    pub oversize_left: u16,   /* Extra bytes available at the end of the last
                              pbuf in unsent (used for asserting vs.
                              tcp_pcb.unsent_oversize only) */

    pub chksum: u16,
    pub chksum_swapped: u8,

    pub flags: u8,

    pub tcphdr: tcp_hdr, //  the TCP header
}

pub const LWIP_TCP_OPT_EOL: u32 = 0;
// pub const LWIP_TCP_OPT_NOP: u32 = 1;
// pub const LWIP_TCP_OPT_MSS: u32 = 2;
// pub const LWIP_TCP_OPT_WS: u32 = 3;
// pub const LWIP_TCP_OPT_SACK_PERM: u32 = 4;
// pub const LWIP_TCP_OPT_TS: u32 = 8;

// pub const LWIP_TCP_OPT_LEN_MSS: u32 = 4;

// pub const LWIP_TCP_OPT_LEN_TS: u32 = 10;
// pub const LWIP_TCP_OPT_LEN_TS_OUT: u32 = 12; //  aligned for output (includes NOP padding)
pub const LWIP_TCP_OPT_LEN_TS_OUT: u32 = 0;

// pub const LWIP_TCP_OPT_LEN_WS: u32 = 3;
// pub const LWIP_TCP_OPT_LEN_WS_OUT: u32 = 4; //  aligned for output (includes NOP padding)
pub const LWIP_TCP_OPT_LEN_WS_OUT: u32 = 0;

// pub const LWIP_TCP_OPT_LEN_SACK_PERM: u32 = 2;
// pub const LWIP_TCP_OPT_LEN_SACK_PERM_OUT: u32 = 4; //  aligned for output (includes NOP padding)
pub const LWIP_TCP_OPT_LEN_SACK_PERM_OUT: u32 = 0;

//  #define LWIP_TCP_OPT_LENGTH(flags) \
//   ((flags) & TF_SEG_OPTS_MSS       ? LWIP_TCP_OPT_LEN_MSS           : 0) + \
//   ((flags) & TF_SEG_OPTS_TS        ? LWIP_TCP_OPT_LEN_TS_OUT        : 0) + \
//   ((flags) & TF_SEG_OPTS_WND_SCALE ? LWIP_TCP_OPT_LEN_WS_OUT        : 0) + \
//   ((flags) & TF_SEG_OPTS_SACK_PERM ? LWIP_TCP_OPT_LEN_SACK_PERM_OUT : 0)

//  This returns a TCP header option for MSS in an u32
pub fn TCP_BUILD_MSS_OPTION(mss: u16) -> u16 {
    lwip_htonl(0x02040000 | ((mss) & 0xFFFF))
}

// pub const TCPWNDSIZE_F: u32 = U32_F;
pub const TCPWND_MAX: u32 = 0xFFFFFFFF;
// #define TCPWND_CHECK16(x)  LWIP_ASSERT("window size > 0xFFFF", (x) <= 0xFFFF)
pub fn TCPWND_MIN16(x: u16) {
    (LWIP_MIN((x), 0xFFFF))
}
//  LWIP_WND_SCALE
// pub const TCPWNDSIZE_F: u32 = U16_F;
pub const TCPWND_MAX: u32 = 0xFFFF;
// #define TCPWND_CHECK16(x)
// #define TCPWND_MIN16(x)    x

//  Global variables:
// extern tcp_input_pcb: &mut TcpContext;
// extern tcp_ticks: u32;
// extern tcp_active_pcbs_changed: u8;

//  The TCP PCB lists.
// union tcp_listen_pcbs_t { //  List of all TCP PCBs in LISTEN state.
//   let mut listen_pcbs: &mut TcpContext_listen;
//   let mut pcbs: &mut TcpContext;
// };

pub struct tcp_listen_pcbs_t {
    pub listen_pcbs: Vec<tcp_pcb_listen>,
    pub pcbs: Vec<tcp_pcb>,
}

// extern tcp_bound_pcbs: &mut TcpContext;
// extern union tcp_listen_pcbs_t tcp_listen_pcbs;
// extern tcp_active_pcbs: &mut TcpContext;
/* List of all TCP PCBs that are in a
state in which they accept or send
data. */
// extern tcp_tw_pcbs: &mut TcpContext;      //  List of all TCP PCBs in TIME-WAIT.
pub const NUM_TCP_PCB_LISTS_NO_TIME_WAIT: usize = 3;
pub const NUM_TCP_PCB_LISTS: usize = 4;
// extern struct tcp_pcb ** const tcp_pcb_lists[NUM_TCP_PCB_LISTS];

/* Axioms about the above lists:
   1) Every TCP PCB that is not CLOSED is in one of the lists.
   2) A PCB is only in one of the lists.
   3) All PCBs in the tcp_listen_pcbs list is in LISTEN state.
   4) All PCBs in the tcp_tw_pcbs list is in TIME-WAIT state.
*/
/* Define two macros, TCP_REG and TCP_RMV that registers a TCP PCB
with a PCB list or removes a PCB from a list, respectively. */

pub const TCP_DEBUG_PCB_LISTS: u32 = 0;

pub fn TCP_REG(pcbs: &mut Vec<tcp_pcb>, npcb: &mut TcpContext) {
    let mut tcp_tmp_pcb: &mut TcpContext;
    //                            LWIP_DEBUGF(TCP_DEBUG, ("TCP_REG %p local port %"U16_F"\n", (npcb), npcb.local_port)); \
    //                     for (tcp_tmp_pcb = *(pcbs);
    //   tcp_tmp_pcb != None;
    // tcp_tmp_pcb = tcp_tmp_pcb.next) {
    //    LWIP_ASSERT("TCP_REG: already registered\n", tcp_tmp_pcb != (npcb));
    //                     }
    LWIP_ASSERT(
        "TCP_REG: pcb.state != CLOSED",
        ((pcbs) == &tcp_bound_pcbs) || (npcb.state != CLOSED),
    );
    (npcb).next = *(pcbs);
    LWIP_ASSERT("TCP_REG: npcb.next != npcb", (npcb).next != (npcb));
    *(pcbs) = (npcb);
    LWIP_ASSERT("TCP_REG: tcp_pcbs sane", tcp_pcbs_sane());
    tcp_timer_needed();
}
pub fn TCP_RMV(pcbs: &mut Vec<tcp_pcb>, npcb: &mut TcpContext) {
    let mut tcp_tmp_pcb: &mut TcpContext;
    LWIP_ASSERT("TCP_RMV: pcbs != NULL", *(pcbs) != None);
    //                            LWIP_DEBUGF(TCP_DEBUG, ("TCP_RMV: removing %p from %p\n", (npcb), (*(pcbs)))); \
    if (*(pcbs) == (npcb)) {
        *(pcbs) = (*pcbs).next;
    } else {
        //   for (tcp_tmp_pcb = *(pcbs); tcp_tmp_pcb != None; tcp_tmp_pcb = tcp_tmp_pcb.next) {
        //    if(tcp_tmp_pcb.next == (npcb)) {
        //       tcp_tmp_pcb.next = (npcb).next;
        //       break;
        //    }
        // }
    }
    (npcb).next = None;
    LWIP_ASSERT("TCP_RMV: tcp_pcbs sane", tcp_pcbs_sane());
    //                            LWIP_DEBUGF(TCP_DEBUG, ("TCP_RMV: removed %p from %p\n", (npcb), (*(pcbs)))); \
}

//  LWIP_DEBUG

pub fn TCP_REG(pcbs: &mut Vec<tcp_pcb>, npcb: &mut TcpContext) {
    (npcb).next = *pcbs;
    *(pcbs) = (npcb);
    tcp_timer_needed();
}

pub fn TCP_RMV(pcbs: &mut Vec<tcp_pcb>, npcb: &mut TcpContext) {
    if (*(pcbs) == (npcb)) {
        (*(pcbs)) = (*pcbs).next;
    } else {
        let mut tcp_tmp_pcb: &mut TcpContext;
        // for (tcp_tmp_pcb = *pcbs;
        //     tcp_tmp_pcb != None;
        //     tcp_tmp_pcb = tcp_tmp_pcb.next) {
        //   if(tcp_tmp_pcb.next == (npcb)) {
        //     tcp_tmp_pcb.next = (npcb).next;
        //     break;
        //   }
        // }
    }
    (npcb).next = None;
}

pub fn TCP_REG_ACTIVE(npcb: &mut TcpContext) {
    TCP_REG(&tcp_active_pcbs, npcb);
    tcp_active_pcbs_changed = 1;
}

pub fn TCP_RMV_ACTIVE(npcb: &mut TcpContext) {
    TCP_RMV(&tcp_active_pcbs, npcb);
    tcp_active_pcbs_changed = 1;
}

pub fn TCP_PCB_REMOVE_ACTIVE(pcb: &mut TcpContext) {
    tcp_pcb_remove(&tcp_active_pcbs, pcb);
    tcp_active_pcbs_changed = 1;
}

//  Internal functions:
// tcp_pcb_copy: &mut TcpContext(pcb: &mut TcpContext);
// pub fn  tcp_pcb_purge(pcb: &mut TcpContext);
// pub fn  tcp_pcb_remove(struct tcp_pcb **pcblist, pcb: &mut TcpContext);

// pub fn  tcp_segs_free(seg: &mut tcp_seg);
// pub fn  tcp_seg_free(seg: &mut tcp_seg);
// tcp_seg_copy: &mut tcp_seg(seg: &mut tcp_seg);

pub fn tcp_ack(pcb: &mut TcpContext) {
    if ((pcb).flags & TF_ACK_DELAY) {
        tcp_clear_flags(pcb, TF_ACK_DELAY);
        tcp_ack_now(pcb);
    } else {
        tcp_set_flags(pcb, TF_ACK_DELAY);
    }
}

pub fn tcp_ack_now(tcp_ctx: &mut TcpContext) -> Result<(), LwipError> {
    tcp_set_flags(tcp_ctx, TF_ACK_NOW)
}
//   tcp_set_flags(pcb, TF_ACK_NOW)

// pub fn  tcp_send_fin(pcb: &mut TcpContext);
// pub fn  tcp_enqueue_flags(pcb: &mut TcpContext, flags: u8);

// pub fn  tcp_rexmit_seg(pcb: &mut TcpContext, seg: &mut tcp_seg);

// pub fn  tcp_rst( struct tcp_pcb* pcb, seqno: u32, ackno: u32,
//  local_ip: &mut LwipAddr,  remote_ip: &mut LwipAddr,
//        local_port: u16, remote_port: u16);

// tcp_next_iss: u32(pcb: &mut TcpContext);

// pub fn  tcp_keepalive(pcb: &mut TcpContext);
// pub fn  tcp_split_unsent_seg(pcb: &mut TcpContext, split: u16);
// pub fn  tcp_zero_window_probe(pcb: &mut TcpContext);
// pub fn   tcp_trigger_input_pcb_close();

// tcp_eff_send_mss_netif: u16(sendmss: u16, outif: &mut NetIfc,
//  dest: &mut LwipAddr);
// #define tcp_eff_send_mss(sendmss, src, dest) \
//     tcp_eff_send_mss_netif(sendmss, ip_route(src, dest), dest)

// pub fn  tcp_recv_None(arg: &mut Vec<u8>, pcb: &mut TcpContext, p: &mut PacketBuffer, err: err_t);

// pub fn  tcp_debug_print(tcphdr: &mut tcp_hdr);
// pub fn  tcp_debug_print_flags(flags: u8);
// pub fn  tcp_debug_print_state(s: tcp_state);
// pub fn  tcp_debug_print_pcbs();
// tcp_pcbs_sane: i16();

// #  define tcp_debug_print(tcphdr)
// #  define tcp_debug_print_flags(flags)
// #  define tcp_debug_print_state(s)
// #  define tcp_debug_print_pcbs()
// #  define tcp_pcbs_sane() 1

/* External function (implemented in timers.c), called when TCP detects
 * that a timer is needed (i.e. active- or time-wait-pcb found). */
// pub fn  tcp_timer_needed();

// pub fn  tcp_netif_ip_addr_changed( old_addr: &mut LwipAddr,  new_addr: &mut LwipAddr);

// pub fn  tcp_free_ooseq(pcb: &mut TcpContext);

// pub fn  tcp_ext_arg_invoke_callbacks_passive_open(lpcb: &mut TcpContext_listen, cpcb: &mut TcpContext);
