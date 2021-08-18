/*
 * @file
 * Sockets API internal implementations (do not use in application code)
 */

/*
 * Copyright (c) 2017 Joel Cunningham, Garmin International, Inc. <joel.cunningham@garmin.com>
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
 * Author: Joel Cunningham <joel.cunningham@me.com>
 *
 */

// #define LWIP_HDR_SOCKETS_PRIV_H

pub const NUM_SOCKETS: usize = MEMP_NUM_NETCONN;

/* This is overridable for the rare case where more than 255 threads
 * select on the same socket...
 */

// #define SELWAIT_T u8

// union lwip_sock_lastdata {
//   netbuf: &mut netbuf;
//   pbuf: &mut pbuf;
// };

/* Contains all internal pointers and states used for a socket */
// struct lwip_sock {
//   /* sockets currently are built on netconns, each socket has one netconn */
//    let conn: &mut netconn;
//   /* data that was left from the previous read */
//   union lwip_sock_lastdata lastdata;

//   /* number of times data was received, set by event_callback(),
//       tested by the receive and select functions */
//   rcvevent: i16;
//   /* number of times data was ACKed (free send buffer), set by event_callback(),
//       tested by select */
//   sendevent: u16;
//   /* error happened for this socket, set by event_callback(), tested by select */
//   errevent: u16;
//   /* counter of how many threads are waiting for this socket using select */
//   SELWAIT_T select_waiting;

//   /* counter of how many threads are using a struct lwip_sock (not the 'int') */
//   fd_used: u8;
//   /* status of pending close/delete actions */
//   fd_free_pending: u8;
// // #define LWIP_SOCK_FD_FREE_TCP  1
// // #define LWIP_SOCK_FD_FREE_FREE 2

// };

pub fn set_errno(err: i32) {
    if (err) {
        errno = (err);
    }
}

/* Maximum optlen used by setsockopt/getsockopt */
// #define LWIP_SETGETSOCKOPT_MAXOPTLEN LWIP_MAX(16, sizeof(struct ifreq))

/* This struct is used to pass data to the set/getsockopt_internal
 * functions running in tcpip_thread context (only a void* is allowed) */
pub struct lwip_setgetsockopt_data {
    /* socket index for which to change options */
    pub s: i32,
    /* level of the option to process */
    pub level: i32,
    /* name of the option to process */
    pub optname: i32,
    /* set: value to set the option to
     * get: value of the option is stored here */
    pub optval: [u8; LWIP_SETGETSOCKOPT_MAXOPTLEN],
    p: (),
    pc: Vec<u8>,
    /* size of *optval */
    pub optlen: usize,
    /* if an error occurs, it is temporarily stored here */
    pub err: i32,
    /* semaphore to wake up the calling task */
    pub completed_sem: (),
}

// struct lwip_sock* lwip_socket_dbg_get_socket(fd: i32);

// #define SELECT_SEM_T        sys_sem_t*
// #define SELECT_SEM_PTR(sem) (sem)
/* LWIP_NETCONN_SEM_PER_THREAD */
// #define SELECT_SEM_T        sys_sem_t
// #define SELECT_SEM_PTR(sem) (&(sem))

/* Description for a task waiting in select */
pub struct LwipSelectCallback {
    /* Pointer to the next waiting task */
    pub next: &mut lwip_select_cb,
    /* Pointer to the previous waiting task */
    pub prev: &mut lwip_select_cb,
    /* readset passed to select */
    pub readset: fdset,
    /* writeset passed to select */
    pub writeset: fdset,
    /* unimplemented: exceptset passed to select */
    pub exceptset: fd_set,
    /* fds passed to poll; NULL if select */
    pub poll_fds: Vec<pollfd>,
    /* nfds passed to poll; 0 if select */
    pub poll_nfds: nfds_t,
    /* don't signal the same semaphore twice: set to 1 when signalled */
    pub sem_signalled: i32,
    /* semaphore to wake up a task waiting for select */
    sem: u32,
}
