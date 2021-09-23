/*
 * @file
 * Application layered TCP connection API (to be used from TCPIP thread)\n
 *
 * This file contains the generic API.
 * For more details see @ref altcp_api.
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 */

use crate::altcp_tls::altcp_tls_mbedtls_structs::AlTcpMbedTlsState;
use crate::altcp_tls::altcp_tls_mbedtls::AlTcpTlsConfig;
use crate::defines::LwipAddr;
use crate::core::tcpbase_h::TcpState;
use crate::net_ops::NetOperations;
use crate::core::err_h::LwipError;
use crate::core::tcp2_h::TcpContext;

// // typedef err_t (*AltcpAcceptFn)(arg: &mut Vec<u8>, new_conn: &mut AltcpPcb, err: err_t);
// type AltcpAcceptFn = fn(arg: &mut AlTcpPcb, new_conn: &mut AlTcpPc) -> Result<(), LwipError>;
// // typedef err_t (*AltcpConnectedFn)(arg: &mut Vec<u8>, conn: &mut AltcpPcb, err: err_t);
type AltcpConnectedFunc = fn(arg: &mut AlTcpContext, conn: &mut AlTcpContext) -> Result<(), LwipError>;
// // typedef err_t (*AltcpRecvFn)(arg: &mut Vec<u8>, conn: &mut AltcpPcb, p: &mut pbuf, err: err_t);
// type AltcpRecvFn = fn(arg: &mut Vec<u8>, conn: &mut AlTcpPcb, p: &mut pbuf, err: err_t) -> err_t;
// // typedef err_t (*AltcpSentFn)(arg: &mut Vec<u8>, conn: &mut AltcpPcb, len: u16);
// type AltcpSentFn = fn(arg: &mut Vec<u8>, conn: &mut AlTcpPcb, len: usize) -> err_t;
// // typedef err_t (*AltcpPollFn)(arg: &mut Vec<u8>, conn: &mut AltcpPcb);
type AlTcpPollFn = fn(arg: &mut Vec<u8>, conn: &mut AlTcpPcb) -> err_t;
// // typedef void  (*AltcpErrFn)(arg: &mut Vec<u8>, err: err_t);
// type AltcpErrFn = fn(arg: &mut AlTcpPcb, err: err_t);
// // typedef struct AltcpPcb* (*AltcpNewFn)(arg: &mut Vec<u8>, ip_type: u8);
// type AltcpNewFn = fn(arg: &mut AlTcpPcb, ip_type: u8) -> &mut AlTcpPcb;
// // typedef void (*AltcpSetPollFn)(conn: &mut AltcpPcb, interval: u8);
// type AltcpSetPollFn = fn(conn: &mut AlTcpPcb, u8: i32erval);
// // typedef void (*AltcpRecvedFn)(conn: &mut AltcpPcb, len: u16);
// type AltcpRecvedFn = fn(conn: &mut AlTcpPcb, len: usize);
// // typedef err_t (*AltcpBindFn)(conn: &mut AltcpPcb,  ipaddr: &mut LwipAddr, port: u16);
// type AltcpBindFn = fn(conn: &mut AlTcpPcb, ip_addr: &LwipAddr, port: u16) -> err_t;
// // typedef err_t (*AltcpConnectFn)(conn: &mut AltcpPcb,  ipaddr: &mut LwipAddr, port: u16, AltcpConnectedFn connected);
// type AltcpConnectFn =
//     fn(conn: &mut AlTcpPcb, ipaddr: &LwipAddr, port: u16, connected: AltcpConnectedFn) -> err_t;
// // typedef struct AltcpPcb *(*AltcpListenFn)(conn: &mut AltcpPcb, backlog: u8, err: &mut err_t);
// type AltcpListenFn = fn(conn: &mut AlTcpPcb, backlog: u8, err: &mut err_t) -> &mut AlTcpPcb;
// // typedef void  (*AltcpAbortFn)(conn: &mut AltcpPcb);
// type AltcpAbortFn = fn(conn: &mut AlTcpPcb);
// // typedef err_t (*AltcpCloseFn)(conn: &mut AltcpPcb);
// type AltcpCloseFn = fn(conn: &mut AlTcpPcb) -> err_t;
// // typedef err_t (*AltcpShutdownFn)(conn: &mut AltcpPcb, shut_rx: i32, shut_tx: i32);
// type AltcpShutdownFn = fn(conn: &mut AlTcpPcb, shut_rx: i32, shut_tx: i32) -> err_t;
// // typedef err_t (*AltcpWriteFn)(conn: &mut AltcpPcb, dataptr: &Vec<u8>, len: u16, apiflags: u8);
// type AltcpWriteFn =
//     fn(conn: &mut AlTcpPcb, dataptr: &[u8], len: usize, apiflags: u8) -> err_t;
// // typedef err_t (*AltcpOutputFn)(conn: &mut AltcpPcb);
// type AltcpOutputFn = fn(conn: &mut AlTcpPcb) -> err_t;
// // typedef u16 (*AltcpMssFn)(conn: &mut AltcpPcb);
// type AltcpMssFn = fn(conn: &mut AlTcpPcb) -> u16;
// // typedef u16 (*AltcpSndbufFn)(conn: &mut AltcpPcb);
// type AltcpSndbufFn = fn(conn: &mut AlTcpPcb) -> u16;
// // typedef u16 (*AltcpSndqueuelenFn)(conn: &mut AltcpPcb);
// type AltcpSndqueuelenFn = fn(conn: &mut AlTcpPcb) -> u16;
// // typedef void  (*AltcpNagleDisableFn)(conn: &mut AltcpPcb);
// type AltcpNagleDisableFn = fn(conn: &mut AlTcpPcb);
// // typedef void  (*AltcpNagleEnableFn)(conn: &mut AltcpPcb);
// type AltcpNagleEnableFn = fn(conn: &mut AlTcpPcb);
// // typedef int   (*AltcpNagleDisabledFn)(conn: &mut AltcpPcb);
// type AltcpNagleDisabledFn = fn(conn: &mut AlTcpPcb) -> i32;
// // typedef void  (*AltcpSetprioFn)(conn: &mut AltcpPcb, prio: u8);
// type AltcpSetprioFn = fn(conn: &mut AlTcpPcb, prio: u8);
// // typedef void  (*AltcpDeallocFn)(conn: &mut AltcpPcb);
// type AltcpDeallocFn = fn(conn: &mut AlTcpPcb);
// // typedef err_t (*AltcpGetTcpAddrinfoFn)(conn: &mut AltcpPcb, local: i32, addr: &mut LwipAddr, port: &mut u16);
// type AltcpGetTcpAddrinfoFn =
//     fn(conn: &mut AlTcpPcb, local: i32, addr: &LwipAddr, port: &u16) -> err_t;
// // typedef LwipAddr *(*AltcpGetIpFn)(conn: &mut AltcpPcb, local: i32);
// type AltcpGetIpFn = fn(conn: &mut AlTcpPcb, local: i32) -> LwipAddr;
// // typedef u16 (*AltcpGetPortFn)(conn: &mut AltcpPcb, local: i32);
// type AltcpGetPortFn = fn(conn: &mut AlTcpPcb, local: i32) -> u16;
// // typedef enum tcp_state (*AltcpDbgGetTcpStateFn)(conn: &mut AltcpPcb);
// type AltcpDbgGetTcpStateFn = fn(conn: &mut AlTcpPcb) -> TcpState;

// pub struct AltcpFunctions {
//     pub set_poll: Option<AltcpSetPollFn>,
//     pub recved: Option<AltcpRecvedFn>,
//     pub bind: Option<AltcpBindFn>,
//     pub connect: Option<AltcpConnectFn>,
//     pub listen: Option<AltcpListenFn>,
//     pub abort: Option<AltcpAbortFn>,
//     pub close: Option<AltcpCloseFn>,
//     pub shutdown: Option<AltcpShutdownFn>,
//     pub write: Option<AltcpWriteFn>,
//     pub output: Option<AltcpOutputFn>,
//     pub mss: Option<AltcpMssFn>,
//     pub sndbuf: Option<AltcpSndbufFn>,
//     pub sndqueuelen: Option<AltcpSndqueuelenFn>,
//     pub nagle_disable: Option<AltcpNagleDisableFn>,
//     pub nagle_enable: Option<AltcpNagleEnableFn>,
//     pub nagle_disabled: Option<AltcpNagleDisabledFn>,
//     pub setprio: Option<AltcpSetprioFn>,
//     pub dealloc: Option<AltcpDeallocFn>,
//     pub addrinfo: Option<AltcpGetTcpAddrinfoFn>,
//     pub getip: Option<AltcpGetIpFn>,
//     pub getport: Option<AltcpGetPortFn>,
//     pub dbg_get_tcp_state: Option<AltcpDbgGetTcpStateFn>,
// }
//
// impl AltcpFunctions {
//     pub fn new() -> AltcpFunctions {
//         AltcpFunctions {
//             set_poll: None,
//             recved: None,
//             bind: None,
//             connect: None,
//             listen: None,
//             abort: None,
//             close: None,
//             shutdown: None,
//             write: None,
//             output: None,
//             mss: None,
//             sndbuf: None,
//             sndqueuelen: None,
//             nagle_disable: None,
//             nagle_enable: None,
//             nagle_disabled: None,
//             setprio: None,
//             dealloc: None,
//             addrinfo: None,
//             getip: None,
//             getport: None,
//             dbg_get_tcp_state: None,
//         }
//     }
// }

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct AlTcpContext {
    pub tcp_ctx: TcpContext,
    pub al_tcp_proxy_conn_state: AlTcpProxyConnectState,

    // pub functions: AltcpFunctions,
    pub inner_conn_key : u32,
    // TODO: figure out how to handle self-referencing inner struct
    // arg: &mut Vec<u8>;
    // pub arg: Option<T>,
    // state: &mut Vec<u8>;
    pub state: Option<AlTcpMbedTlsState>,
    /* application callbacks */
    // AltcpAcceptFn     accept;
    // pub accept: Option<AltcpAcceptFn>,
    // AltcpConnectedFn  connected;
    // pub connected: Option<AltcpConnectedFn>,
    // AltcpRecvFn       recv;
    // pub recv: Option<AltcpRecvFn>,
    // AltcpSentFn       sent;
    // pub sent: Option<AltcpSentFn>,
    // AltcpPollFn       poll;
    // pub poll: Option<AltcpPollFn>,
    // AltcpErrFn        err;
    // pub err: Option<AltcpErrFn>,
    // pollinterval: u8;
    pub pollinterval: u64,
}

impl AlTcpContext {
    pub fn new<T>() -> AlTcpContext {
        AlTcpContext {
            inner_conn_key: 0,
            tcp_ctx: TcpContext::new(),
            // functions: NetOperations::new(),
            // arg: None,
            state: some(AlTcpMbedTlsState::new()),
            // accept: None,
            // connected: None,
            // recv: None,
            // sent: None,
            // poll: None,
            // err: None,
            pollinterval: 0,
            al_tcp_proxy_conn_state: ()
        }
    }
}

/* @ingroup altcp */
// typedef struct altcp_allocator_s {
//   /* Allocator function */
//   AltcpNewFn  alloc;
//   /* Argument to allocator function */
//   void         *arg;
// } AltcpAllocatorT;
pub struct AltcpAllocatorT {
    alloc: AltcpNewFn,
    arg: Vec<u8>,
}

pub struct AltcpProxyconnectConfig {
    // LwipAddr proxy_addr;
    pub proxy_addr: LwipAddr,
    // proxy_port: u16;
    pub proxy_port: u16,
}

pub struct AltcpProxyconnectTlsConfig {
    // struct AltcpProxyconnectConfig proxy;
    proxy: AltcpProxyconnectConfig,
    // tls_config: &mut altcp_tls_config;
    tls_config: AlTcpTlsConfig,
}
