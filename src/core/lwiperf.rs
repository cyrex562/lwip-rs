/*
 * @file
 * lwIP iPerf server implementation
 */

/*
 * @defgroup iperf Iperf server
 * @ingroup apps
 *
 * This is a simple performance measuring client/server to check your bandwith using
 * iPerf2 on a PC as server/client.
 * It is currently a minimal implementation providing a TCP client/server only.
 *
 * @todo:
 * - implement UDP mode
 * - protect combined sessions handling (via 'related_master_state') against reallocation
 *   (this is a pointer address, currently, so if the same memory is allocated again,
 *    session pairs (tx/rx) can be confused on reallocation)
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
 */

/* Currently, only TCP is implemented */

/* Specify the idle timeout (in seconds) after that the test fails */

pub const LWIPERF_TCP_MAX_IDLE_SEC: u64 = 10;

// #error LWIPERF_TCP_MAX_IDLE_SEC must fit into an u8

/* Change this if you don't want to lwiperf to listen to any IP version */

pub const LWIPERF_SERVER_IP_TYPE: u32 = IPADDR_TYPE_ANY;

/* File internal memory allocation (struct lwiperf_*): this defaults to
the heap */

// #define LWIPERF_ALLOC(client_type)         mem_malloc(sizeof(client_type))
// #define LWIPERF_FREE(client_type, item)    mem_free(item)

/* If this is 1, check that received data has the correct format */

pub const LWIPERF_CHECK_RX_DATA: u32 = 0;

pub const LWIPERF_FLAGS_ANSWER_TEST: u32 = 0x80000000;
pub const LWIPERF_FLAGS_ANSWER_NOW: u32 = 0x00000001;
/* This is the Iperf settings struct sent from the client */
pub struct lwiperf_settings_t {
    pub flags: u32,
    pub num_threads: u32, /* unused for now */
    pub flags: u32,
    pub buffer_len: u32, /* unused for now */
    pub win_band: u32,   /* TCP window / UDP rate: unused for now */
    pub amount: u32,     /* pos. value: bytes?; neg. values: time (unit is 10ms: 1/100 second) */
}

/* Basic connection handle */
// struct _lwiperf_state_base;
// typedef struct _lwiperf_state_base lwiperf_state_base_t;
pub struct _lwiperf_state_base {
    /* linked list */
    // lwiperf_state_base_t *next;
    /* 1=tcp, 0=udp */
    pub tcp: u8,
    /* 1=server, 0=client */
    pub server: u8,
    /* master state used to abort sessions (e.g. listener, main client) */
    pub related_master_state: lwiperf_state_base_t,
}

/* Connection handle for a TCP iperf session */
pub struct lwiperf_state_tcp_t {
    pub base: lwiperf_state_base_t,
    pub server_pcb: &mut TcpContext,
    pub conn_pcb: &mut TcpContext,
    pub time_started: u32,
    pub report_fn: lwiperf_report_fn,
    pub report_arg: &mut Vec<u8>,
    pub poll_count: u8,
    pub next_num: u8,
    /* 1=start server when client is closed */
    pub client_tradeoff_mode: u8,
    pub bytes_transferred: u32,
    pub settings: lwiperf_settings_t,
    pub have_settings_buf: u8,
    pub specific_remote: u8,
    pub remote_addr: LwipAddr,
}

/* List of active iperf sessions */
// static lwiperf_state_base_t *lwiperf_all_connections;
/* A const buffer to send from: we want to measure sending, not copying! */
// static const lwiperf_txbuf_const: [u8;1600] = {
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//   '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
// };

// static lwiperf_tcp_poll: err_t(arg: &mut Vec<u8>, tpcb: &mut TcpContext);
// pub fn lwiperf_tcp_err(arg: &mut Vec<u8>, err: err_t);
// static lwiperf_start_tcp_server_impl: err_t( local_addr: &mut LwipAddr, local_port: u16,
//                                            lwiperf_report_fn report_fn, report_arg: &mut Vec<u8>,
//                                            lwiperf_state_base_t *related_master_state, lwiperf_state_tcp_t **state);

/* Add an iperf session to the 'active' list */
pub fn lwiperf_list_add(item: lwiperf_state_base_t) {
    item.next = lwiperf_all_connections;
    lwiperf_all_connections = item;
}

/* Remove an iperf session from the 'active' list */
pub fn lwiperf_list_remove(item: lwiperf_state_base_t) {
    lwiperf_state_base_t * prev = None;
    lwiperf_state_base_t * iter;
    // for (iter = lwiperf_all_connections; iter != NULL; prev = iter, iter = iter.next) {
    //   if (iter == item) {
    //     if (prev == NULL) {
    //       lwiperf_all_connections = iter.next;
    //     } else {
    //       prev.next = iter.next;
    //     }
    //     /* @debug: ensure this item is listed only once */
    //     for (iter = iter.next; iter != NULL; iter = iter.next) {
    //       LWIP_ASSERT("duplicate entry", iter != item);
    //     }
    //     break;
    //   }
    // }
}

pub fn lwiperf_list_find(item: lwiperf_state_base_t) -> lwiperf_state_base_t {
    lwiperf_state_base_t * iter;
    // for (iter = lwiperf_all_connections; iter != NULL; iter = iter.next) {
    //   if (iter == item) {
    //     return item;
    //   }
    // }
    return None;
}

/* Call the report function of an iperf tcp session */
pub fn lwip_tcp_conn_report(conn: lwiperf_state_tcp_t, report_type: lwiperf_report_type) {
    if ((conn != None) && (conn.report_fn != None)) {
        let now: u32;
        let duration_ms;
        let bandwidth_kbitpsec;
        now = sys_now();
        duration_ms = now - conn.time_started;
        if (duration_ms == 0) {
            bandwidth_kbitpsec = 0;
        } else {
            bandwidth_kbitpsec = (conn.bytes_transferred / duration_ms) * 8;
        }
        conn.report_fn(
            conn.report_arg,
            report_type,
            &conn.conn_pcb.local_ip,
            conn.conn_pcb.local_port,
            &conn.conn_pcb.remote_ip,
            conn.conn_pcb.remote_port,
            conn.bytes_transferred,
            duration_ms,
            bandwidth_kbitpsec,
        );
    }
}

/* Close an iperf tcp session */
pub fn lwiperf_tcp_close(conn: lwiperf_state_tcp_t, report_type: lwiperf_report_type) {
    let err: err_t;

    lwiperf_list_remove(&conn.base);
    lwip_tcp_conn_report(conn, report_type);
    if (conn.conn_pcb != None) {
        tcp_arg(conn.conn_pcb, None);
        tcp_poll(conn.conn_pcb, None, 0);
        tcp_sent(conn.conn_pcb, None);
        tcp_recv(conn.conn_pcb, None);
        tcp_err(conn.conn_pcb, None);
        err = tcp_close(conn.conn_pcb);
        if (err != ERR_OK) {
            /* don't want to wait for free memory here... */
            tcp_abort(conn.conn_pcb);
        }
    } else {
        /* no conn pcb, this is the listener pcb */
        err = tcp_close(conn.server_pcb);
        LWIP_ASSERT("error", err == ERR_OK);
    }
    LWIPERF_FREE(lwiperf_state_tcp_t, conn);
}

/* Try to send more data on an iperf tcp session */
pub fn lwiperf_tcp_client_send_more(conn: lwiperf_state_tcp_t) -> Result<(), LwipError> {
    let letsend_more: i32;
    let err: err_t;
    let txlen: u16;
    let txlen_max: u16;
    let txptr: &mut Vec<u8>;
    let apiflags: u8;

    LWIP_ASSERT(
        "conn invalid",
        (conn != None) && conn.base.tcp && (conn.base.server == 0),
    );

    loop {
        send_more = 0;
        if (conn.settings.amount & PP_HTONL(0x80000000)) {
            /* this session is time-limited */
            let now: u32 = sys_now();
            let diff_ms: u32 = now - conn.time_started;
            let time: u32 = -lwip_htonl(conn.settings.amount);
            let time_ms: u32 = time * 10;
            if (diff_ms >= time_ms) {
                /* time specified by the client is over -> close the connection */
                lwiperf_tcp_close(conn, LWIPERF_TCP_DONE_CLIENT);
                return Ok(());
            }
        } else {
            /* this session is byte-limited */
            let amount_bytes: u32 = lwip_htonl(conn.settings.amount);
            /* @todo: this can send up to 1*MSS more than requested... */
            if (amount_bytes >= conn.bytes_transferred) {
                /* all requested bytes transferred -> close the connection */
                lwiperf_tcp_close(conn, LWIPERF_TCP_DONE_CLIENT);
                return Ok(());
            }
        }

        if (conn.bytes_transferred < 24) {
            /* transmit the settings a first time */
            txptr = &(&conn.settings)[conn.bytes_transferred];
            txlen_max = (24 - conn.bytes_transferred);
            apiflags = TCP_WRITE_FLAG_COPY;
        } else if (conn.bytes_transferred < 48) {
            /* transmit the settings a second time */
            txptr = &(&conn.settings)[conn.bytes_transferred - 24];
            txlen_max = (48 - conn.bytes_transferred);
            apiflags = TCP_WRITE_FLAG_COPY | TCP_WRITE_FLAG_MORE;
            send_more = 1;
        } else {
            /* transmit data */
            /* @todo: every x bytes, transmit the settings again */
            txptr = &lwiperf_txbuf_const[conn.bytes_transferred % 10];
            txlen_max = TCP_MSS;
            if (conn.bytes_transferred == 48) {
                /* @todo: fix this for intermediate settings, too */
                txlen_max = TCP_MSS - 24;
            }
            apiflags = 0; /* no copying needed */
            send_more = 1;
        }
        txlen = txlen_max;
        loop {
            err = tcp_write(conn.conn_pcb, txptr, txlen, apiflags);
            if (err == ERR_MEM) {
                txlen /= 2;
            }
            if !((err == ERR_MEM) && (txlen >= (TCP_MSS / 2))) {
                break;
            }
        }

        if (err == ERR_OK) {
            conn.bytes_transferred += txlen;
        } else {
            send_more = 0;
        }
        if !send_more {
            break;
        }
    }

    tcp_output(conn.conn_pcb);
    return Ok(());
}

/* TCP sent callback, try to send more data */
pub fn lwiperf_tcp_client_sent(
    arg: &mut Vec<u8>,
    tpcb: &mut TcpContext,
    len: usize,
) -> Result<(), LwipError> {
    lwiperf_state_tcp_t * conn = arg;
    /* @todo: check 'len' (e.g. to time ACK of all data)? for now, we just send more... */
    LWIP_ASSERT("invalid conn", conn.conn_pcb == tpcb);

    conn.poll_count = 0;

    return lwiperf_tcp_client_send_more(conn);
}

/* TCP connected callback (active connection), send data now */
pub fn lwiperf_tcp_client_connected(
    arg: &mut Vec<u8>,
    tpcb: &mut TcpContext,
    err: err_t,
) -> Result<(), LwipError> {
    lwiperf_state_tcp_t * conn = arg;
    LWIP_ASSERT("invalid conn", conn.conn_pcb == tpcb);

    if (err != ERR_OK) {
        lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_REMOTE);
        return Ok(());
    }
    conn.poll_count = 0;
    conn.time_started = sys_now();
    return lwiperf_tcp_client_send_more(conn);
}

/* Start TCP connection back to the client (either parallel or after the
 * receive test has finished.
 */
pub fn lwiperf_tx_start_impl(
    remote_ip: &mut LwipAddr,
    remote_port: u16,
    settings: &lwiperf_settings_t,
    report_fn: lwiperf_report_fn,
    report_arg: &mut Vec<u8>,
    related_master_state: lwiperf_state_base_t,
    new_conn: lwiperf_state_tcp_t,
) {
    let err: err_t;
    let client_conn: lwiperf_state_tcp_t;
    let newpcb: &mut TcpContext;
    let remote_addr: LwipAddr;

    LWIP_ASSERT("remote_ip != NULL", remote_ip != None);
    LWIP_ASSERT("remote_ip != NULL", settings != None);
    LWIP_ASSERT("new_conn != NULL", new_conn != None);
    *new_conn = None;

    client_conn = LWIPERF_ALLOC(lwiperf_state_tcp_t);
    if (client_conn == None) {
        return ERR_MEM;
    }
    newpcb = tcp_new_ip_type(IP_GET_TYPE(remote_ip));
    if (newpcb == None) {
        LWIPERF_FREE(lwiperf_state_tcp_t, client_conn);
        return ERR_MEM;
    }
    //memset(client_conn, 0, sizeof(lwiperf_state_tcp_t));
    client_conn.base.tcp = 1;
    client_conn.base.related_master_state = related_master_state;
    client_conn.conn_pcb = newpcb;
    client_conn.time_started = sys_now(); /* @todo: set this again on 'connected' */
    client_conn.report_fn = report_fn;
    client_conn.report_arg = report_arg;
    client_conn.next_num = 4; /* initial nr is '4' since the header has 24 byte */
    client_conn.bytes_transferred = 0;
    memcpy(&client_conn.settings, settings, sizeof(*settings));
    client_conn.have_settings_buf = 1;

    tcp_arg(newpcb, client_conn);
    tcp_sent(newpcb, lwiperf_tcp_client_sent);
    tcp_poll(newpcb, lwiperf_tcp_poll, 2);
    tcp_err(newpcb, lwiperf_tcp_err);

    ip_addr_copy(remote_addr, *remote_ip);

    err = tcp_connect(
        newpcb,
        &remote_addr,
        remote_port,
        lwiperf_tcp_client_connected,
    );
    if (err != ERR_OK) {
        lwiperf_tcp_close(client_conn, LWIPERF_TCP_ABORTED_LOCAL);
        return err;
    }
    lwiperf_list_add(&client_conn.base);
    *new_conn = client_conn;
    return Ok(());
}

pub fn lwiperf_tx_start_passive(conn: lwiperf_state_tcp_t) -> Result<(), LwipError> {
    let ret: err_t;
    lwiperf_state_tcp_t * new_conn = None;
    let remote_port: u16 = lwip_htonl(conn.settings.remote_port);

    ret = lwiperf_tx_start_impl(
        &conn.conn_pcb.remote_ip,
        remote_port,
        &conn.settings,
        conn.report_fn,
        conn.report_arg,
        conn.base.related_master_state,
        &new_conn,
    );
    if (ret == ERR_OK) {
        LWIP_ASSERT("new_conn != NULL", new_conn != None);
        new_conn.settings.flags = 0; /* prevent the remote side starting back as client again */
    }
    return ret;
}

/* Receive data on an iperf tcp session */
pub fn lwiperf_tcp_recv(
    arg: &mut Vec<u8>,
    tpcb: &mut TcpContext,
    p: &mut PacketBuffer,
    err: err_t,
) -> Result<(), LwipError> {
    let tmp: u8;
    let tot_len: u16;
    let packet_idx: u32;
    let q: &mut PacketBuffer;
    lwiperf_state_tcp_t * conn = arg;

    LWIP_ASSERT("pcb mismatch", conn.conn_pcb == tpcb);

    if (err != ERR_OK) {
        lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_REMOTE);
        return Ok(());
    }
    if (p == None) {
        /* connection closed -> test done */
        if (conn.settings.flags & PP_HTONL(LWIPERF_FLAGS_ANSWER_TEST)) {
            if ((conn.settings.flags & PP_HTONL(LWIPERF_FLAGS_ANSWER_NOW)) == 0) {
                /* client requested transmission after end of test */
                lwiperf_tx_start_passive(conn);
            }
        }
        lwiperf_tcp_close(conn, LWIPERF_TCP_DONE_SERVER);
        return Ok(());
    }
    tot_len = p.tot_len;

    conn.poll_count = 0;

    if ((!conn.have_settings_buf) || ((conn.bytes_transferred - 24) % (1024 * 128) == 0)) {
        /* wait for 24-byte header */
        if (p.tot_len < sizeof(lwiperf_settings_t)) {
            lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_LOCAL_DATAERROR);
            pbuf_free(p);
            return Ok(());
        }
        if (!conn.have_settings_buf) {
            if (pbuf_copy_partial(p, &conn.settings, sizeof(lwiperf_settings_t), 0)
                != sizeof(lwiperf_settings_t))
            {
                lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_LOCAL);
                pbuf_free(p);
                return Ok(());
            }
            conn.have_settings_buf = 1;
            if (conn.settings.flags & PP_HTONL(LWIPERF_FLAGS_ANSWER_TEST)) {
                if (conn.settings.flags & PP_HTONL(LWIPERF_FLAGS_ANSWER_NOW)) {
                    /* client requested parallel transmission test */
                    let err2: err_t = lwiperf_tx_start_passive(conn);
                    if (err2 != ERR_OK) {
                        lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_LOCAL_TXERROR);
                        pbuf_free(p);
                        return Ok(());
                    }
                }
            }
        } else {
            if (conn.settings.flags & PP_HTONL(LWIPERF_FLAGS_ANSWER_TEST)) {
                if (pbuf_memcmp(p, 0, &conn.settings, sizeof(lwiperf_settings_t)) != 0) {
                    lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_LOCAL_DATAERROR);
                    pbuf_free(p);
                    return Ok(());
                }
            }
        }
        conn.bytes_transferred += sizeof(lwiperf_settings_t);
        if (conn.bytes_transferred <= 24) {
            conn.time_started = sys_now();
            tcp_recved(tpcb, p.tot_len);
            pbuf_free(p);
            return Ok(());
        }
        conn.next_num = 4; /* 24 bytes received... */
        tmp = pbuf_remove_header(p, 24);
        LWIP_ASSERT("pbuf_remove_header failed", tmp == 0);
        /* for LWIP_NOASSERT */
    }

    packet_idx = 0;
    // for (q = p; q != NULL; q = q.next) {

    //   const payload: &mut Vec<u8>= q.payload;
    //   let i: u16;
    //   for (i = 0; i < q.len; i+= 1) {
    //     val: u8 = payload[i];
    //     num: u8 = val - '0';
    //     if (num == conn.next_num) {
    //       conn.next_num+= 1;
    //       if (conn.next_num == 10) {
    //         conn.next_num = 0;
    //       }
    //     } else {
    //       lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_LOCAL_DATAERROR);
    //       pbuf_free(p);
    //      return Ok(());
    //     }
    //   }

    //   packet_idx += q.len;
    // }
    LWIP_ASSERT("count mismatch", packet_idx == p.tot_len);
    conn.bytes_transferred += packet_idx;
    tcp_recved(tpcb, tot_len);
    pbuf_free(p);
    return Ok(());
}

/* Error callback, iperf tcp session aborted */
pub fn lwiperf_tcp_err(arg: &mut Vec<u8>, err: err_t) {
    lwiperf_state_tcp_t * conn = arg;

    lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_REMOTE);
}

/* TCP poll callback, try to send more data */
pub fn lwiperf_tcp_poll(arg: &mut Vec<u8>, tpcb: &mut TcpContext) -> Result<(), LwipError> {
    lwiperf_state_tcp_t * conn = arg;
    LWIP_ASSERT("pcb mismatch", conn.conn_pcb == tpcb);

    if (conn.poll_count += 1 >= LWIPERF_TCP_MAX_IDLE_SEC) {
        lwiperf_tcp_close(conn, LWIPERF_TCP_ABORTED_LOCAL);
        return Ok(()); /* lwiperf_tcp_close frees conn */
    }

    if (!conn.base.server) {
        lwiperf_tcp_client_send_more(conn);
    }

    return Ok(());
}

/* This is called when a new client connects for an iperf tcp session */
pub fn lwiperf_tcp_accept(
    arg: &mut Vec<u8>,
    newpcb: &mut TcpContext,
    err: err_t,
) -> Result<(), LwipError> {
    let s: lwiperf_state_tcp_t;
    let conn: lwiperf_state_tcp_t;
    if ((err != ERR_OK) || (newpcb == None) || (arg == None)) {
        return ERR_VAL;
    }

    s = arg;
    LWIP_ASSERT("invalid session", s.base.server);
    LWIP_ASSERT("invalid listen pcb", s.server_pcb != None);
    LWIP_ASSERT("invalid conn pcb", s.conn_pcb == None);
    if (s.specific_remote) {
        LWIP_ASSERT(
            "s.base.related_master_state != NULL",
            s.base.related_master_state != None,
        );
        if (!ip_addr_cmp(&newpcb.remote_ip, &s.remote_addr)) {
            /* this listener belongs to a client session, and this is not the correct remote */
            return ERR_VAL;
        }
    } else {
        LWIP_ASSERT(
            "s.base.related_master_state == NULL",
            s.base.related_master_state == None,
        );
    }

    conn = LWIPERF_ALLOC(lwiperf_state_tcp_t);
    if (conn == None) {
        return ERR_MEM;
    }
    //memset(conn, 0, sizeof(lwiperf_state_tcp_t));
    conn.base.tcp = 1;
    conn.base.server = 1;
    conn.base.related_master_state = &s.base;
    conn.conn_pcb = newpcb;
    conn.time_started = sys_now();
    conn.report_fn = s.report_fn;
    conn.report_arg = s.report_arg;

    /* setup the tcp rx connection */
    tcp_arg(newpcb, conn);
    tcp_recv(newpcb, lwiperf_tcp_recv);
    tcp_poll(newpcb, lwiperf_tcp_poll, 2);
    tcp_err(conn.conn_pcb, lwiperf_tcp_err);

    if (s.specific_remote) {
        /* this listener belongs to a client, so make the client the master of the newly created connection */
        conn.base.related_master_state = s.base.related_master_state;
        /* if dual mode or (tradeoff mode AND client is done): close the listener */
        if (!s.client_tradeoff_mode || !lwiperf_list_find(s.base.related_master_state)) {
            /* prevent report when closing: this is expected */
            s.report_fn = None;
            lwiperf_tcp_close(s, LWIPERF_TCP_ABORTED_LOCAL);
        }
    }
    lwiperf_list_add(&conn.base);
    return Ok(());
}

/*
 * @ingroup iperf
 * Start a TCP iperf server on the default TCP port (5001) and listen for
 * incoming connections from iperf clients.
 *
 * @returns a connection handle that can be used to abort the server
 *          by calling @ref lwiperf_abort()
 */
pub fn lwiperf_start_tcp_server_default(report_fn: lwiperf_report_fn, report_arg: &mut Vec<u8>) {
    return lwiperf_start_tcp_server(IP_ADDR_ANY, LWIPERF_TCP_PORT_DEFAULT, report_fn, report_arg);
}

/*
 * @ingroup iperf
 * Start a TCP iperf server on a specific IP address and port and listen for
 * incoming connections from iperf clients.
 *
 * @returns a connection handle that can be used to abort the server
 *          by calling @ref lwiperf_abort()
 */
pub fn lwiperf_start_tcp_server(
    local_addr: &mut LwipAddr,
    local_port: u16,
    report_fn: lwiperf_report_fn,
    report_arg: &mut Vec<u8>,
) {
    let err: err_t;
    lwiperf_state_tcp_t * state = None;

    err =
        lwiperf_start_tcp_server_impl(local_addr, local_port, report_fn, report_arg, None, &state);
    if (err == ERR_OK) {
        return state;
    }
    return None;
}

pub fn lwiperf_start_tcp_server_impl(
    local_addr: &mut LwipAddr,
    local_port: u16,
    report_fn: lwiperf_report_fn,
    report_arg: &mut Vec<u8>,
    related_master_state: lwiperf_state_base_t,
    state: lwiperf_state_tcp_t,
) -> Result<(), LwipError> {
    let err: err_t;
    let pcb: &mut TcpContext;
    let s: lwiperf_state_tcp_t;

    LWIP_ASSERT_CORE_LOCKED();

    LWIP_ASSERT("state != NULL", state != None);

    if (local_addr == None) {
        return ERR_ARG;
    }

    s = LWIPERF_ALLOC(lwiperf_state_tcp_t);
    if (s == None) {
        return ERR_MEM;
    }
    //memset(s, 0, sizeof(lwiperf_state_tcp_t));
    s.base.tcp = 1;
    s.base.server = 1;
    s.base.related_master_state = related_master_state;
    s.report_fn = report_fn;
    s.report_arg = report_arg;

    pcb = tcp_new_ip_type(LWIPERF_SERVER_IP_TYPE);
    if (pcb == None) {
        return ERR_MEM;
    }
    err = tcp_bind(pcb, local_addr, local_port);
    if (err != ERR_OK) {
        return err;
    }
    s.server_pcb = tcp_listen_with_backlog(pcb, 1);
    if (s.server_pcb == None) {
        if (pcb != None) {
            tcp_close(pcb);
        }
        LWIPERF_FREE(lwiperf_state_tcp_t, s);
        return ERR_MEM;
    }
    pcb = None;

    tcp_arg(s.server_pcb, s);
    tcp_accept(s.server_pcb, lwiperf_tcp_accept);

    lwiperf_list_add(&s.base);
    *state = s;
    return Ok(());
}

/*
 * @ingroup iperf
 * Start a TCP iperf client to the default TCP port (5001).
 *
 * @returns a connection handle that can be used to abort the client
 *          by calling @ref lwiperf_abort()
 */
pub fn lwiperf_start_tcp_client_default(
    remote_addr: &mut LwipAddr,
    report_fn: lwiperf_report_fn,
    report_arg: &mut Vec<u8>,
) {
    return lwiperf_start_tcp_client(
        remote_addr,
        LWIPERF_TCP_PORT_DEFAULT,
        LWIPERF_CLIENT,
        report_fn,
        report_arg,
    );
}

/*
 * @ingroup iperf
 * Start a TCP iperf client to a specific IP address and port.
 *
 * @returns a connection handle that can be used to abort the client
 *          by calling @ref lwiperf_abort()
 */
pub fn lwiperf_start_tcp_client(
    remote_addr: &mut LwipAddr,
    remote_port: u16,
    client_type: lwiperf_client_type,
    report_fn: lwiperf_report_fn,
    report_arg: &mut Vec<u8>,
) {
    let ret: err_t;
    let settings: lwiperf_settings_t;
    let state: lwiperf_state_tcp_t = None;

    //memset(&settings, 0, sizeof(settings));
    match (client_type) {
        LWIPERF_CLIENT =>
        /* Unidirectional tx only test */
        {
            settings.flags = 0;
        }

        LWIPERF_DUAL => {
            /* Do a bidirectional test simultaneously */
            settings.flags = htonl(LWIPERF_FLAGS_ANSWER_TEST | LWIPERF_FLAGS_ANSWER_NOW);
        }

        LWIPERF_TRADEOFF => {
            /* Do a bidirectional test individually */
            settings.flags = htonl(LWIPERF_FLAGS_ANSWER_TEST);
        }

        _ => {
            /* invalid argument */
            return None;
        }
    }
    settings.num_threads = htonl(1);
    settings.remote_port = htonl(LWIPERF_TCP_PORT_DEFAULT);
    /* TODO: implement passing duration/amount of bytes to transfer */
    settings.amount = htonl(-1000);

    ret = lwiperf_tx_start_impl(
        remote_addr,
        remote_port,
        &settings,
        report_fn,
        report_arg,
        None,
        &state,
    );
    if (ret == ERR_OK) {
        LWIP_ASSERT("state != NULL", state != None);
        if (client_type != LWIPERF_CLIENT) {
            /* start corresponding server now */
            lwiperf_state_tcp_t * server = None;
            ret = lwiperf_start_tcp_server_impl(
                &state.conn_pcb.local_ip,
                LWIPERF_TCP_PORT_DEFAULT,
                report_fn,
                report_arg,
                state,
                &server,
            );
            if (ret != ERR_OK) {
                /* starting server failed, abort client */
                lwiperf_abort(state);
                return None;
            }
            /* make this server accept one connection only */
            server.specific_remote = 1;
            server.remote_addr = state.conn_pcb.remote_ip;
            if (client_type == LWIPERF_TRADEOFF) {
                /* tradeoff means that the remote host connects only after the client is done,
                so keep the listen pcb open until the client is done */
                server.client_tradeoff_mode = 1;
            }
        }
        return state;
    }
    return None;
}

/*
 * @ingroup iperf
 * Abort an iperf session (handle returned by lwiperf_start_tcp_server*())
 */
pub fn lwiperf_abort(lwiperf_session: &mut Vec<u8>) {
    let i: lwiperf_state_base_t;
    let dealloc: lwiperf_state_base_t;
    let last: lwiperf_state_base_t;

    LWIP_ASSERT_CORE_LOCKED();

    // for (i = lwiperf_all_connections; i != NULL; ) {
    //   if ((i == lwiperf_session) || (i.related_master_state == lwiperf_session)) {
    //     dealloc = i;
    //     i = i.next;
    //     if (last != NULL) {
    //       last.next = i;
    //     }
    //     LWIPERF_FREE(lwiperf_state_tcp_t, dealloc); /* @todo: client_type? */
    //   } else {
    //     last = i;
    //     i = i.next;
    //   }
    // }
}
