/*
 * @file
 * lwIP iPerf server implementation
 */

/*
 * Copyright (c) 2014 Simon Goldschmidt
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
 * Author: Simon Goldschmidt
 *
 */

// #define LWIP_HDR_APPS_LWIPERF_H

pub const LWIPERF_TCP_PORT_DEFAULT: u32 = 5001;

/* lwIPerf test results */
pub enum lwiperf_report_type {
    /* The server side test is done */
    LWIPERF_TCP_DONE_SERVER,
    /* The client side test is done */
    LWIPERF_TCP_DONE_CLIENT,
    /* Local error lead to test abort */
    LWIPERF_TCP_ABORTED_LOCAL,
    /* Data check error lead to test abort */
    LWIPERF_TCP_ABORTED_LOCAL_DATAERROR,
    /* Transmit error lead to test abort */
    LWIPERF_TCP_ABORTED_LOCAL_TXERROR,
    /* Remote side aborted the test */
    LWIPERF_TCP_ABORTED_REMOTE,
}

/* Control */
pub enum lwiperf_client_type {
    /* Unidirectional tx only test */
    LWIPERF_CLIENT,
    /* Do a bidirectional test simultaneously */
    LWIPERF_DUAL,
    /* Do a bidirectional test individually */
    LWIPERF_TRADEOFF,
}

/* Prototype of a report function that is called when a session is finished.
This report function can show the test results.
@param report_type contains the test result */
// typedef void (*lwiperf_report_fn)(arg: &mut Vec<u8>, report_type: lwiperf_report_type,
//   const local_addr: &mut LwipAddr, local_port: u16,  remote_addr: &mut LwipAddr, remote_port: u16,
//   bytes_transferred: u32, ms_duration: u32, bandwidth_kbitpsec: u32);

// pub fn * lwiperf_start_tcp_server( local_addr: &mut LwipAddr, local_port: u16,
//                                lwiperf_report_fn report_fn, report_arg: &mut Vec<u8>);
// pub fn * lwiperf_start_tcp_server_default(lwiperf_report_fn report_fn, report_arg: &mut Vec<u8>);
// pub fn * lwiperf_start_tcp_client( remote_addr: &mut LwipAddr, remote_port: u16,
//                                type: lwiperf_client_type,
//                                lwiperf_report_fn report_fn, report_arg: &mut Vec<u8>);
// pub fn * lwiperf_start_tcp_client_default( remote_addr: &mut LwipAddr,
//                                lwiperf_report_fn report_fn, report_arg: &mut Vec<u8>);

// pub fn   lwiperf_abort(lwiperf_session: &mut Vec<u8>);
