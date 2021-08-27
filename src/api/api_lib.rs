use crate::core::{
    api_h::NetConnDesc,
    api_msg_h::{api_msg, NETCONN_SHUT_RD},
    pbuf_h::PacketBuffer,
};

/*
 * @file
 * Sequential API External module
 *
 * @defgroup netconn Netconn API
 * @ingroup sequential_api
 * Thread-safe, to be called from non-TCPIP threads only.
 * TX/RX handling based on @ref netbuf (containing @ref pbuf)
 * to avoid copying data around.
 *
 * @defgroup netconn_common Common functions
 * @ingroup netconn
 * For use with TCP and UDP
 *
 * @defgroup netconn_tcp TCP only
 * @ingroup netconn
 * TCP only functions
 *
 * @defgroup netconn_udp UDP only
 * @ingroup netconn
 * UDP only functions
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
 */

/* This is the part of the API that is linked with
the application */

// #define (name)               (name)
// #define API_MSG_VAR_DECLARE(name)           API_VAR_DECLARE(struct api_msg, name)
// #define API_MSG_VAR_ALLOC(name)             API_VAR_ALLOC(struct api_msg, MEMP_API_MSG, name, ERR_MEM)
// #define API_MSG_VAR_ALLOC_RETURN_NULL(name) API_VAR_ALLOC(struct api_msg, MEMP_API_MSG, name, NULL)
// #define API_MSG_VAR_FREE(name)              API_VAR_FREE(MEMP_API_MSG, name)

/* need to allocate API message for accept so empty message pool does not result in event loss
 * see bug #47512: MPU_COMPATIBLE may fail on empty pool */
// #define API_MSG_VAR_ALLOC_ACCEPT(msg) API_MSG_VAR_ALLOC(msg)
// #define API_MSG_VAR_FREE_ACCEPT(msg) API_MSG_VAR_FREE(msg)
// #else /* TCP_LISTEN_BACKLOG */
// #define API_MSG_VAR_ALLOC_ACCEPT(msg)
// #define API_MSG_VAR_FREE_ACCEPT(msg)

pub fn NETCONN_RECVMBOX_WAITABLE(conn: &mut NetConnDesc) -> bool {
    (sys_mbox_valid(&conn.recvmbox) && (((conn).flags & NETCONN_FLAG_MBOXINVALID) == 0))
}

pub fn NETCONN_ACCEPTMBOX_WAITABLE(conn: &mut NetConnDesc) -> bool {
    (sys_mbox_valid(&conn.acceptmbox)
        && (((conn).flags & (NETCONN_FLAG_MBOXCLOSED | NETCONN_FLAG_MBOXINVALID)) == 0))
}

pub fn NETCONN_MBOX_WAITING_INC(conn: &mut NetConnDesc) {
    SYS_ARCH_INC(conn.mbox_threads_waiting, 1)
}

pub fn NETCONN_MBOX_WAITING_DEC(conn: &mut NetConnDesc) {
    SYS_ARCH_DEC(conn.mbox_threads_waiting, 1)
}

// #else /* LWIP_NETCONN_FULLDUPLEX */
// #define NETCONN_RECVMBOX_WAITABLE(conn)   sys_mbox_valid(&conn.recvmbox)
// #define NETCONN_ACCEPTMBOX_WAITABLE(conn) (sys_mbox_valid(&conn.acceptmbox) && (((conn).flags & NETCONN_FLAG_MBOXCLOSED) == 0))
// #define NETCONN_MBOX_WAITING_INC(conn)
// #define NETCONN_MBOX_WAITING_DEC(conn)

// static netconn_close_shutdown: err_t(conn: &mut netconn, how: u8);

/*
 * Call the lower part of a netconn_* function
 * This function is then running in the thread context
 * of tcpip_thread and has exclusive access to lwIP core code.
 *
 * @param fn function to call
 * @param apimsg a struct containing the function to call and its parameters
 * @return ERR_OK if the function was called, another if: err_t not
 */
pub fn netconn_apimsg(func: tcpip_callback_fn, apimsg: &mut api_msg) -> Result<(), LwipError> {
    let err: err_t;

    /* catch functions that don't set err */
    apimsg.err = ERR_VAL;

    apimsg.op_completed_sem = LWIP_NETCONN_THREAD_SEM_GET();

    err = tcpip_send_msg_wait_sem(func, apimsg, LWIP_API_MSG_SEM(apimsg));
    if (err == ERR_OK) {
        return apimsg.err;
    }
    return err;
}

/*
 * Create a new netconn (of a specific type) that has a callback function.
 * The corresponding pcb is also created.
 *
 * @param t the type of 'connection' to create (@see enum netconn_type)
 * @param proto the IP protocol for RAW IP pcbs
 * @param callback a function to call on status changes (RX available, TX'ed)
 * @return a newly allocated struct netconn or
 *         NULL on memory error
 */
pub fn netconn_new_with_proto_and_callback(
    t: netconn_type,
    proto: u8,
    callback: netconn_callback,
) -> NetConnDesc {
    let conn: &mut NetConnDesc;
    API_MSG_VAR_DECLARE(msg);
    API_MSG_VAR_ALLOC_RETURN_NULL(msg);

    conn = netconn_alloc(t, callback);
    if (conn != NULL) {
        let err: err_t;

        msg.msg.n.proto = proto;
        msg.conn = conn;
        err = netconn_apimsg(lwip_netconn_do_newconn, &(msg));
        if (err != ERR_OK) {
            LWIP_ASSERT("freeing conn without freeing pcb", conn.pcb.tcp == NULL);
            LWIP_ASSERT("conn has no recvmbox", sys_mbox_valid(&conn.recvmbox));

            LWIP_ASSERT(
                "conn.acceptmbox shouldn't exist",
                !sys_mbox_valid(&conn.acceptmbox),
            );

            LWIP_ASSERT(
                "conn has no op_completed",
                sys_sem_valid(&conn.op_completed),
            );
            sys_sem_free(&conn.op_completed);

            sys_mbox_free(&conn.recvmbox);
            memp_free(MEMP_NETCONN, conn);
            API_MSG_VAR_FREE(msg);
            return NULL;
        }
    }
    API_MSG_VAR_FREE(msg);
    return conn;
}

/*
 * @ingroup netconn_common
 * Close a netconn 'connection' and free all its resources but not the netconn itself.
 * UDP and RAW connection are completely closed, TCP pcbs might still be in a waitstate
 * after this returns.
 *
 * @param conn the netconn to delete
 * @return ERR_OK if the connection was deleted
 */
pub fn netconn_prepare_delete(conn: &mut NetConnDesc) {
    let err: err_t;
    API_MSG_VAR_DECLARE(msg);

    /* No ASSERT here because possible to get a (conn == NULL) if we got an accept error */
    if (conn == NULL) {
       return Ok(());
    }

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;

    /* get the time we started, which is later compared to
    sys_now() + conn.send_timeout */
    msg.msg.sd.time_started = sys_now();
    //#else /* LWIP_SO_SNDTIMEO || LWIP_SO_LINGER */
    msg.msg.sd.polls_left =
        ((LWIP_TCP_CLOSE_TIMEOUT_MS_DEFAULT + TCP_SLOW_INTERVAL - 1) / TCP_SLOW_INTERVAL) + 1;

    err = netconn_apimsg(lwip_netconn_do_delconn, &(msg));
    API_MSG_VAR_FREE(msg);

    if (err != ERR_OK) {
        return err;
    }
   return Ok(());
}

/*
 * @ingroup netconn_common
 * Close a netconn 'connection' and free its resources.
 * UDP and RAW connection are completely closed, TCP pcbs might still be in a waitstate
 * after this returns.
 *
 * @param conn the netconn to delete
 * @return ERR_OK if the connection was deleted
 */
pub fn netconn_delete(conn: &mut NetConnDesc) {
    let err: err_t;

    /* No ASSERT here because possible to get a (conn == NULL) if we got an accept error */
    if (conn == NULL) {
       return Ok(());
    }

    if (conn.flags & NETCONN_FLAG_MBOXINVALID) {
        /* Already called netconn_prepare_delete() before */
        err = ERR_OK;
    } else {
        err = netconn_prepare_delete(conn);
    }
    if (err == ERR_OK) {
        netconn_free(conn);
    }
    return err;
}

/*
 * Get the local or remote IP address and port of a netconn.
 * For RAW netconns, this returns the protocol instead of a port!
 *
 * @param conn the netconn to query
 * @param addr a pointer to which to save the IP address
 * @param port a pointer to which to save the port (or protocol for RAW)
 * @param local 1 to get the local IP address, 0 to get the remote one
 * @return ERR_CONN for invalid connections
 *         ERR_OK if the information was retrieved
 */
pub fn netconn_getaddr(
    conn: &mut NetConnDesc,
    addr: &mut LwipAddr,
    port: &mut u16,
    local: u8,
) -> Result<(), LwipError> {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // LWIP_ERROR("netconn_getaddr: invalid conn", (conn != NULL), return ERR_ARG;);
    // if conn == None {
    //   return Err(LwipError::new(ERR_ARG, "netconn_getaddr: invalid conn"));
    // }

    // LWIP_ERROR("netconn_getaddr: invalid addr", (addr != NULL), return ERR_ARG;);

    // LWIP_ERROR("netconn_getaddr: invalid port", (port != NULL), return ERR_ARG;);

    // API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;
    msg.msg.ad.local = local;

    err = netconn_apimsg(lwip_netconn_do_getaddr, &(msg));
    *addr = msg.msg.ad.ipaddr;
    *port = msg.msg.ad.port;
    // #else /* LWIP_MPU_COMPATIBLE */
    msg.msg.ad.ipaddr = addr;
    msg.msg.ad.port = port;
    err = netconn_apimsg(lwip_netconn_do_getaddr, &msg);

    // API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_common
 * Bind a netconn to a specific local IP address and port.
 * Binding one netconn twice might not always be checked correctly!
 *
 * @param conn the netconn to bind
 * @param addr the local IP address to bind the netconn to
 *             (use IP4_ADDR_ANY/IP6_ADDR_ANY to bind to all addresses)
 * @param port the local port to bind the netconn to (not used for RAW)
 * @return ERR_OK if bound, any other on: err_t failure
 */
pub fn netconn_bind(conn: &mut NetConnDesc, addr: &mut LwipAddr, port: u16) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // LWIP_ERROR("netconn_bind: invalid conn", (conn != NULL), return ERR_ARG;);

    /* Don't propagate NULL pointer (IP_ADDR_ANY alias) to subsequent functions */
    if (addr == NULL) {
        addr = IP4_ADDR_ANY;
    }

    /* "Socket API like" dual-stack support: If IP to bind to is IP6_ADDR_ANY,
     * and NETCONN_FLAG_IPV6_V6ONLY is 0, use IP_ANY_TYPE to bind
     */
    if ((netconn_get_ipv6only(conn) == 0) && ip_addr_cmp(addr, IP6_ADDR_ANY)) {
        addr = IP_ANY_TYPE;
    }

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;
    msg.msg.bc.ipaddr = (addr);
    msg.msg.bc.port = port;
    err = netconn_apimsg(lwip_netconn_do_bind, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_common
 * Bind a netconn to a specific interface and port.
 * Binding one netconn twice might not always be checked correctly!
 *
 * @param conn the netconn to bind
 * @param if_idx the local interface index to bind the netconn to
 * @return ERR_OK if bound, any other on: err_t failure
 */
pub fn netconn_bind_if(conn: &mut NetConnDesc, if_idx: u8) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // LWIP_ERROR("netconn_bind_if: invalid conn", (conn != NULL), return ERR_ARG;);

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;
    msg.msg.bc.if_idx = if_idx;
    err = netconn_apimsg(lwip_netconn_do_bind_if, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_common
 * Connect a netconn to a specific remote IP address and port.
 *
 * @param conn the netconn to connect
 * @param addr the remote IP address to connect to
 * @param port the remote port to connect to (no used for RAW)
 * @return ERR_OK if connected, return value of tcp_/udp_/raw_connect otherwise
 */
pub fn netconn_connect(conn: &mut NetConnDesc, addr: &mut LwipAddr, port: u16) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // LWIP_ERROR("netconn_connect: invalid conn", (conn != NULL), return ERR_ARG;);

    /* Don't propagate NULL pointer (IP_ADDR_ANY alias) to subsequent functions */
    if (addr == NULL) {
        addr = IP4_ADDR_ANY;
    }

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;
    msg.msg.bc.ipaddr = (addr);
    msg.msg.bc.port = port;
    err = netconn_apimsg(lwip_netconn_do_connect, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_udp
 * Disconnect a netconn from its current peer (only valid for UDP netconns).
 *
 * @param conn the netconn to disconnect
 * @return See @ref err_t
 */
pub fn netconn_disconnect(conn: &mut NetConnDesc) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // LWIP_ERROR("netconn_disconnect: invalid conn", (conn != NULL), return ERR_ARG;);

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;
    err = netconn_apimsg(lwip_netconn_do_disconnect, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_tcp
 * Set a TCP netconn into listen mode
 *
 * @param conn the tcp netconn to set to listen mode
 * @param backlog the listen backlog, only used if TCP_LISTEN_BACKLOG==1
 * @return ERR_OK if the netconn was set to listen (UDP and RAW netconns
 *         don't return any error (yet?))
 */
pub fn netconn_listen_with_backlog(conn: &mut NetConnDesc, backlog: u8) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    /* This does no harm. If TCP_LISTEN_BACKLOG is off, backlog is unused. */

    // LWIP_ERROR("netconn_listen: invalid conn", (conn != NULL), return ERR_ARG;);

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;

    msg.msg.lb.backlog = backlog;

    err = netconn_apimsg(lwip_netconn_do_listen, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
    // #else /* LWIP_TCP */
    return ERR_ARG;
}

/*
 * @ingroup netconn_tcp
 * Accept a new connection on a TCP listening netconn.
 *
 * @param conn the TCP listen netconn
 * @param new_conn pointer where the new connection is stored
 * @return ERR_OK if a new connection has been received or an error
 *                code otherwise
 */
pub fn netconn_accept(conn: &mut NetConnDesc, new_conn: &mut NetConnDesc) {
    let err: err_t;
    void * accept_ptr;
    let newconn: &mut NetConnDesc;

    API_MSG_VAR_DECLARE(msg);

    // LWIP_ERROR("netconn_accept: invalid pointer",    (new_conn != NULL),                  return ERR_ARG;);
    *new_conn = NULL;
    // LWIP_ERROR("netconn_accept: invalid conn",       (conn != NULL),                      return ERR_ARG;);

    /* NOTE: Although the opengroup spec says a pending error shall be returned to
    send/recv/getsockopt(SO_ERROR) only, we return it for listening
    connections also, to handle embedded-system errors */
    err = netconn_err(conn);
    if (err != ERR_OK) {
        /* return pending error */
        return err;
    }
    if (!NETCONN_ACCEPTMBOX_WAITABLE(conn)) {
        /* don't accept if closed: this might block the application task
        waiting on acceptmbox forever! */
        return ERR_CLSD;
    }

    API_MSG_VAR_ALLOC_ACCEPT(msg);

    NETCONN_MBOX_WAITING_INC(conn);
    if (netconn_is_nonblocking(conn)) {
        if (sys_arch_mbox_tryfetch(&conn.acceptmbox, &accept_ptr) == SYS_ARCH_TIMEOUT) {
            API_MSG_VAR_FREE_ACCEPT(msg);
            NETCONN_MBOX_WAITING_DEC(conn);
            return ERR_WOULDBLOCK;
        }
    } else {
        if (sys_arch_mbox_fetch(&conn.acceptmbox, &accept_ptr, conn.recv_timeout)
            == SYS_ARCH_TIMEOUT)
        {
            API_MSG_VAR_FREE_ACCEPT(msg);
            NETCONN_MBOX_WAITING_DEC(conn);
            return ERR_TIMEOUT;
        }
        // #else
        sys_arch_mbox_fetch(&conn.acceptmbox, &accept_ptr, 0);
    }
    NETCONN_MBOX_WAITING_DEC(conn);

    if (conn.flags & NETCONN_FLAG_MBOXINVALID) {
        if (lwip_netconn_is_deallocated_msg(accept_ptr)) {
            /* the netconn has been closed from another thread */
            API_MSG_VAR_FREE_ACCEPT(msg);
            return ERR_CONN;
        }
    }

    /* Register event with callback */
    API_EVENT(conn, NETCONN_EVT_RCVMINUS, 0);

    if (lwip_netconn_is_err_msg(accept_ptr, &err)) {
        /* a connection has been aborted: e.g. out of pcbs or out of netconns during accept */
        API_MSG_VAR_FREE_ACCEPT(msg);
        return err;
    }
    if (accept_ptr == NULL) {
        /* connection has been aborted */
        API_MSG_VAR_FREE_ACCEPT(msg);
        return ERR_CLSD;
    }
    newconn = accept_ptr;

    /* Let the stack know that we have accepted the connection. */
    msg.conn = newconn;
    /* don't care for the return value of lwip_netconn_do_recv */
    netconn_apimsg(lwip_netconn_do_accepted, &(msg));
    API_MSG_VAR_FREE(msg);

    *new_conn = newconn;
    /* don't set conn.last_err: it's only ERR_OK, anyway */
   return Ok(());
    // #else /* LWIP_TCP */
    return ERR_ARG;
}

/*
 * @ingroup netconn_common
 * Receive data: actual implementation that doesn't care whether pbuf or netbuf
 * is received (this is internal, it's just here for describing common errors)
 *
 * @param conn the netconn from which to receive data
 * @param new_buf pointer where a new pbuf/netbuf is stored when received data
 * @param apiflags flags that control function behaviour. For now only:
 * - NETCONN_DONTBLOCK: only read data that is available now, don't wait for more data
 * @return ERR_OK if data has been received, an error code otherwise (timeout,
 *                memory error or another error)
 *         ERR_CONN if not connected
 *         ERR_CLSD if TCP connection has been closed
 *         ERR_WOULDBLOCK if the netconn is nonblocking but would block to wait for data
 *         ERR_TIMEOUT if the netconn has a receive timeout and no data was received
 */
pub fn netconn_recv_data(
    conn: &mut NetConnDesc,
    new_buf: &mut Vec<u8>,
    apiflags: u8,
) -> Result<(), LwipError> {
    void * buf = NULL;
    let len: usize;

    // LWIP_ERROR("netconn_recv: invalid pointer", (new_buf != NULL), return ERR_ARG;);
    *new_buf = NULL;
    // LWIP_ERROR("netconn_recv: invalid conn",    (conn != NULL),    return ERR_ARG;);

    if (!NETCONN_RECVMBOX_WAITABLE(conn)) {
        let err = netconn_err(conn);
        if (err != ERR_OK) {
            /* return pending error */
            return err;
        }
        return ERR_CONN;
    }

    NETCONN_MBOX_WAITING_INC(conn);
    if (netconn_is_nonblocking(conn)
        || (apiflags & NETCONN_DONTBLOCK)
        || (conn.flags & NETCONN_FLAG_MBOXCLOSED)
        || (conn.pending_err != ERR_OK))
    {
        if (sys_arch_mbox_tryfetch(&conn.recvmbox, &buf) == SYS_ARCH_TIMEOUT) {
            let err: err_t;
            NETCONN_MBOX_WAITING_DEC(conn);
            err = netconn_err(conn);
            if (err != ERR_OK) {
                /* return pending error */
                return err;
            }
            if (conn.flags & NETCONN_FLAG_MBOXCLOSED) {
                return ERR_CONN;
            }
            return ERR_WOULDBLOCK;
        }
    } else {
        if (sys_arch_mbox_fetch(&conn.recvmbox, &buf, conn.recv_timeout) == SYS_ARCH_TIMEOUT) {
            NETCONN_MBOX_WAITING_DEC(conn);
            return ERR_TIMEOUT;
        }
        // #else
        sys_arch_mbox_fetch(&conn.recvmbox, &buf, 0);
    }
    NETCONN_MBOX_WAITING_DEC(conn);

    if (conn.flags & NETCONN_FLAG_MBOXINVALID) {
        if (lwip_netconn_is_deallocated_msg(buf)) {
            /* the netconn has been closed from another thread */
            API_MSG_VAR_FREE_ACCEPT(msg);
            return ERR_CONN;
        }
    }

    if (NETCONNTYPE_GROUP(conn.netconn_type) == NETCONN_TCP) {
        let err: err_t;
        /* Check if this is an error message or a pbuf */
        if (lwip_netconn_is_err_msg(buf, &err)) {
            /* new_buf has been zeroed above already */
            if (err == ERR_CLSD) {
                /* connection closed translates to ERR_OK with *new_buf == NULL */
               return Ok(());
            }
            return err;
        }
        len = (buf).tot_len;
    } else {
        LWIP_ASSERT("buf != NULL", buf != NULL);
        len = netbuf_len(buf);
    }

    SYS_ARCH_DEC(conn.recv_avail, len);

    /* Register event with callback */
    API_EVENT(conn, NETCONN_EVT_RCVMINUS, len);

    // LWIP_DEBUGF(API_LIB_DEBUG, ("netconn_recv_data: received %p, len=%"U16_F"\n", buf, len));

    *new_buf = buf;
    /* don't set conn.last_err: it's only ERR_OK, anyway */
   return Ok(());
}

pub fn netconn_tcp_recvd_msg(
    conn: &mut NetConnDesc,
    len: usize,
    msg: &mut api_msg,
) -> Result<(), LwipError> {
    // LWIP_ERROR("netconn_recv_tcp_pbuf: invalid conn", (conn != NULL) &&
    //            NETCONNTYPE_GROUP(netconn_type(conn)) == NETCONN_TCP, return ERR_ARG;);

    msg.conn = conn;
    msg.msg.r.len = len;

    return netconn_apimsg(lwip_netconn_do_recv, msg);
}

pub fn netconn_tcp_recvd(conn: &mut NetConnDesc, len: usize) {
    let err: err_t;
    API_MSG_VAR_DECLARE(msg);
    // LWIP_ERROR("netconn_recv_tcp_pbuf: invalid conn", (conn != NULL) &&
    //            NETCONNTYPE_GROUP(netconn_type(conn)) == NETCONN_TCP, return ERR_ARG;);

    API_MSG_VAR_ALLOC(msg);
    err = netconn_tcp_recvd_msg(conn, len, &(msg));
    API_MSG_VAR_FREE(msg);
    return err;
}

pub fn netconn_recv_data_tcp(
    conn: &mut NetConnDesc,
    new_buf: &mut PacketBuffer,
    apiflags: u8,
) -> Result<(), LwipError> {
    let err: err_t;
    let buf: &mut pbuf;
    API_MSG_VAR_DECLARE(msg);

    msg = NULL;

    if (!NETCONN_RECVMBOX_WAITABLE(conn)) {
        /* This only happens when calling this function more than once *after* receiving FIN */
        return ERR_CONN;
    }
    if (netconn_is_flag_set(conn, NETCONN_FIN_RX_PENDING)) {
        netconn_clear_flags(conn, NETCONN_FIN_RX_PENDING);
        // // goto handle_fin;
    }

    if (!(apiflags & NETCONN_NOAUTORCVD)) {
        /* need to allocate API message here so empty message pool does not result in event loss
         * see bug #47512: MPU_COMPATIBLE may fail on empty pool */
        API_MSG_VAR_ALLOC(msg);
    }

    err = netconn_recv_data(conn, new_buf, apiflags);
    if (err != ERR_OK) {
        if (!(apiflags & NETCONN_NOAUTORCVD)) {
            API_MSG_VAR_FREE(msg);
        }
        return err;
    }
    buf = *new_buf;
    if (!(apiflags & NETCONN_NOAUTORCVD)) {
        /* Let the stack know that we have taken the data. */
        // len: u16 = buf ? buf.tot_len : 1;
        /* don't care for the return value of lwip_netconn_do_recv */
        /* @todo: this should really be fixed, e.g. by retrying in poll on error */
        netconn_tcp_recvd_msg(conn, len, &(msg));
        API_MSG_VAR_FREE(msg);
    }

    /* If we are closed, we indicate that we no longer wish to use the socket */
    if (buf == NULL) {
        if (apiflags & NETCONN_NOFIN) {
            /* received a FIN but the caller cannot handle it right now:
            re-enqueue it and return "no data" */
            netconn_set_flags(conn, NETCONN_FIN_RX_PENDING);
            return ERR_WOULDBLOCK;
        } else {
            // handle_fin:
            API_EVENT(conn, NETCONN_EVT_RCVMINUS, 0);
            if (conn.pcb.ip == NULL) {
                /* race condition: RST during recv */
                err = netconn_err(conn);
                if (err != ERR_OK) {
                    return err;
                }
                return ERR_RST;
            }
            /* RX side is closed, so deallocate the recvmbox */
            netconn_close_shutdown(conn, NETCONN_SHUT_RD);
            /* Don' store ERR_CLSD as conn.err since we are only half-closed */
            return ERR_CLSD;
        }
    }
    return err;
}

/*
 * @ingroup netconn_tcp
 * Receive data (in form of a pbuf) from a TCP netconn
 *
 * @param conn the netconn from which to receive data
 * @param new_buf pointer where a new pbuf is stored when received data
 * @return ERR_OK if data has been received, an error code otherwise (timeout,
 *                memory error or another error, @see netconn_recv_data)
 *         ERR_ARG if conn is not a TCP netconn
 */
pub fn netconn_recv_tcp_pbuf(conn: &mut NetConnDesc, new_buf: &mut PacketBuffer) {
    // LWIP_ERROR("netconn_recv_tcp_pbuf: invalid conn", (conn != NULL) &&
    //            NETCONNTYPE_GROUP(netconn_type(conn)) == NETCONN_TCP, return ERR_ARG;);

    return netconn_recv_data_tcp(conn, new_buf, 0);
}

/*
 * @ingroup netconn_tcp
 * Receive data (in form of a pbuf) from a TCP netconn
 *
 * @param conn the netconn from which to receive data
 * @param new_buf pointer where a new pbuf is stored when received data
 * @param apiflags flags that control function behaviour. For now only:
 * - NETCONN_DONTBLOCK: only read data that is available now, don't wait for more data
 * @return ERR_OK if data has been received, an error code otherwise (timeout,
 *                memory error or another error, @see netconn_recv_data)
 *         ERR_ARG if conn is not a TCP netconn
 */
pub fn netconn_recv_tcp_pbuf_flags(conn: &mut NetConnDesc, new_buf: &mut PacketBuffer, apiflags: u8) {
    // LWIP_ERROR("netconn_recv_tcp_pbuf: invalid conn", (conn != NULL) &&
    //            NETCONNTYPE_GROUP(netconn_type(conn)) == NETCONN_TCP, return ERR_ARG;);

    return netconn_recv_data_tcp(conn, new_buf, apiflags);
}

/*
 * Receive data (in form of a netbuf) from a UDP or RAW netconn
 *
 * @param conn the netconn from which to receive data
 * @param new_buf pointer where a new netbuf is stored when received data
 * @return ERR_OK if data has been received, an error code otherwise (timeout,
 *                memory error or another error)
 *         ERR_ARG if conn is not a UDP/RAW netconn
 */
pub fn netconn_recv_udp_raw_netbuf(conn: &mut NetConnDesc, new_buf: &mut netbuf) {
    // LWIP_ERROR("netconn_recv_udp_raw_netbuf: invalid conn", (conn != NULL) &&
    //            NETCONNTYPE_GROUP(netconn_type(conn)) != NETCONN_TCP, return ERR_ARG;);

    return netconn_recv_data(conn, new_buf, 0);
}

/*
 * Receive data (in form of a netbuf) from a UDP or RAW netconn
 *
 * @param conn the netconn from which to receive data
 * @param new_buf pointer where a new netbuf is stored when received data
 * @param apiflags flags that control function behaviour. For now only:
 * - NETCONN_DONTBLOCK: only read data that is available now, don't wait for more data
 * @return ERR_OK if data has been received, an error code otherwise (timeout,
 *                memory error or another error)
 *         ERR_ARG if conn is not a UDP/RAW netconn
 */
pub fn netconn_recv_udp_raw_netbuf_flags(conn: &mut NetConnDesc, new_buf: &mut netbuf, apiflags: u8) {
    // LWIP_ERROR("netconn_recv_udp_raw_netbuf: invalid conn", (conn != NULL) &&
    //            NETCONNTYPE_GROUP(netconn_type(conn)) != NETCONN_TCP, return ERR_ARG;);

    return netconn_recv_data(conn, new_buf, apiflags);
}

/*
 * @ingroup netconn_common
 * Receive data (in form of a netbuf containing a packet buffer) from a netconn
 *
 * @param conn the netconn from which to receive data
 * @param new_buf pointer where a new netbuf is stored when received data
 * @return ERR_OK if data has been received, an error code otherwise (timeout,
 *                memory error or another error)
 */
pub fn netconn_recv(conn: &mut NetConnDesc, new_buf: &mut netbuf) {
    let buf: &mut netbuf = NULL;
    let err: err_t;

    // LWIP_ERROR("netconn_recv: invalid pointer", (new_buf != NULL), return ERR_ARG;);
    // *new_buf = NULL;
    // LWIP_ERROR("netconn_recv: invalid conn",    (conn != NULL),    return ERR_ARG;);

    if (NETCONNTYPE_GROUP(conn.netconn_type) == NETCONN_TCP) {
        let p: &mut pbuf = NULL;
        /* This is not a listening netconn, since recvmbox is set */

        buf = memp_malloc(MEMP_NETBUF);
        if (buf == NULL) {
            return ERR_MEM;
        }

        err = netconn_recv_data_tcp(conn, &p, 0);
        if (err != ERR_OK) {
            memp_free(MEMP_NETBUF, buf);
            return err;
        }
        LWIP_ASSERT("p != NULL", p != NULL);

        buf.p = p;
        buf.ptr = p;
        buf.port = 0;
        ip_addr_set_zero(&buf.addr);
        *new_buf = buf;
        /* don't set conn.last_err: it's only ERR_OK, anyway */
       return Ok(());
    } else {
        return netconn_recv_data(conn, new_buf, 0);
    }
}

/*
 * @ingroup netconn_udp
 * Send data (in form of a netbuf) to a specific remote IP address and port.
 * Only to be used for UDP and RAW netconns (not TCP).
 *
 * @param conn the netconn over which to send data
 * @param buf a netbuf containing the data to send
 * @param addr the remote IP address to which to send the data
 * @param port the remote port to which to send the data
 * @return ERR_OK if data was sent, any other on: err_t error
 */
pub fn netconn_sendto(conn: &mut NetConnDesc, buf: &mut netbuf, addr: &mut LwipAddr, port: u16) {
    if (buf != NULL) {
        ip_addr_set(&buf.addr, addr);
        buf.port = port;
        return netconn_send(conn, buf);
    }
    return ERR_VAL;
}

/*
 * @ingroup netconn_udp
 * Send data over a UDP or RAW netconn (that is already connected).
 *
 * @param conn the UDP or RAW netconn over which to send data
 * @param buf a netbuf containing the data to send
 * @return ERR_OK if data was sent, any other on: err_t error
 */
pub fn netconn_send(conn: &mut NetConnDesc, buf: &mut netbuf) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // LWIP_ERROR("netconn_send: invalid conn",  (conn != NULL), return ERR_ARG;);

    // LWIP_DEBUGF(API_LIB_DEBUG, ("netconn_send: sending %"U16_F" bytes\n", buf.p.tot_len));

    API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;
    msg.msg.b = buf;
    err = netconn_apimsg(lwip_netconn_do_send, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_tcp
 * Send data over a TCP netconn.
 *
 * @param conn the TCP netconn over which to send data
 * @param dataptr pointer to the application buffer that contains the data to send
 * @param size size of the application data to send
 * @param apiflags combination of following flags :
 * - NETCONN_COPY: data will be copied into memory belonging to the stack
 * - NETCONN_MORE: for TCP connection, PSH flag will be set on last segment sent
 * - NETCONN_DONTBLOCK: only write the data if all data can be written at once
 * @param bytes_written pointer to a location that receives the number of written bytes
 * @return ERR_OK if data was sent, any other on: err_t error
 */
pub fn netconn_write_partly(
    conn: &mut NetConnDesc,
    dataptr: &Vec<u8>,
    size: usize,
    apiflags: u8,
    bytes_written: usize,
) {
    let vector: netvector;
    vector.ptr = dataptr;
    vector.len = size;
    return netconn_write_vectors_partly(conn, &vector, 1, apiflags, bytes_written);
}

/*
 * Send vectorized data atomically over a TCP netconn.
 *
 * @param conn the TCP netconn over which to send data
 * @param vectors array of vectors containing data to send
 * @param vectorcnt number of vectors in the array
 * @param apiflags combination of following flags :
 * - NETCONN_COPY: data will be copied into memory belonging to the stack
 * - NETCONN_MORE: for TCP connection, PSH flag will be set on last segment sent
 * - NETCONN_DONTBLOCK: only write the data if all data can be written at once
 * @param bytes_written pointer to a location that receives the number of written bytes
 * @return ERR_OK if data was sent, any other on: err_t error
 */
pub fn netconn_write_vectors_partly(
    conn: &mut NetConnDesc,
    vectors: &mut netvector,
    vectorcnt: u16,
    apiflags: u8,
    bytes_written: usize,
) {
    // API_MSG_VAR_DECLARE(msg);
    let msg: api_msg;
    let err: err_t;
    let dontblock: u8;
    let size: usize;
    let i: i32;

    // LWIP_ERROR("netconn_write: invalid conn",  (conn != NULL), return ERR_ARG;);
    // LWIP_ERROR("netconn_write: invalid conn.netconntype",  (NETCONNTYPE_GROUP(conn.netconntype) == NETCONN_TCP), return ERR_VAL;);
    dontblock = netconn_is_nonblocking(conn) || (apiflags & NETCONN_DONTBLOCK);

    if (conn.send_timeout != 0) {
        dontblock = 1;
    }

    if (dontblock && !bytes_written) {
        /* This implies netconn_write() cannot be used for non-blocking send, since
        it has no way to return the number of bytes written. */
        return ERR_VAL;
    }

    /* sum up the total size */
    size = 0;
    // TODO:
    // for (i = 0; i < vectorcnt; i+= 1) {
    //   size += vectors[i].len;
    //   if (size < vectors[i].len) {
    //     /* overflow */
    //     return ERR_VAL;
    //   }
    // }
    if (size == 0) {
       return Ok(());
    } else if (size > SSIZE_MAX) {
        let slimited: usize;
        /* this is required by the socket layer (cannot send full range: usize) */
        if (!bytes_written) {
            return ERR_VAL;
        }
        /* limit the amount of data to send */
        limited = SSIZE_MAX;
        size = limited;
    }

    // API_MSG_VAR_ALLOC(msg);

    /* non-blocking write sends as much  */
    msg.conn = conn;
    msg.msg.w.vector = vectors;
    msg.msg.w.vector_cnt = vectorcnt;
    msg.msg.w.vector_off = 0;
    msg.msg.w.apiflags = apiflags;
    msg.msg.w.len = size;
    msg.msg.w.offset = 0;

    if (conn.send_timeout != 0) {
        /* get the time we started, which is later compared to
        sys_now() + conn.send_timeout */
        msg.msg.w.time_started = sys_now();
    } else {
        msg.msg.w.time_started = 0;
    }

    /* For locking the core: this _can_ be delayed on low memory/low send buffer,
    but if it is, this is done inside api_msg.c:do_write(), so we can use the
    non-blocking version here. */
    err = netconn_apimsg(lwip_netconn_do_write, &(msg));
    if (err == ERR_OK) {
        if (bytes_written != NULL) {
            *bytes_written = msg.msg.w.offset;
        }
        /* for blocking, check all requested bytes were written, NOTE: send_timeout is
        treated as dontblock (see dontblock assignment above) */
        if (!dontblock) {
            LWIP_ASSERT(
                "do_write failed to write all bytes",
                msg.msg.w.offset == size,
            );
        }
    }
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_tcp
 * Close or shutdown a TCP netconn (doesn't delete it).
 *
 * @param conn the TCP netconn to close or shutdown
 * @param how fully close or only shutdown one side?
 * @return ERR_OK if the netconn was closed, any other on: err_t error
 */
pub fn netconn_close_shutdown(conn: &mut NetConnDesc, how: u8) -> Result<(), LwipError> {
    // API_MSG_VAR_DECLARE(msg);
    let msg: api_msg;
    let err: err_t;
    //

    // LWIP_ERROR("netconn_close: invalid conn",  (conn != NULL), return ERR_ARG;);

    // API_MSG_VAR_ALLOC(msg);
    msg.conn = conn;

    /* shutting down both ends is the same as closing */
    msg.msg.sd.shut = how;

    /* get the time we started, which is later compared to
    sys_now() + conn.send_timeout */
    msg.msg.sd.time_started = sys_now();
    // #else /* LWIP_SO_SNDTIMEO || LWIP_SO_LINGER */
    msg.msg.sd.polls_left =
        ((LWIP_TCP_CLOSE_TIMEOUT_MS_DEFAULT + TCP_SLOW_INTERVAL - 1) / TCP_SLOW_INTERVAL) + 1;

    err = netconn_apimsg(lwip_netconn_do_close, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_tcp
 * Close a TCP netconn (doesn't delete it).
 *
 * @param conn the TCP netconn to close
 * @return ERR_OK if the netconn was closed, any other on: err_t error
 */
pub fn netconn_close(conn: &mut NetConnDesc) {
    /* shutting down both ends is the same as closing */
    return netconn_close_shutdown(conn, NETCONN_SHUT_RDWR);
}

/*
 * @ingroup netconn_common
 * Get and reset pending error on a netconn
 *
 * @param conn the netconn to get the error from
 * @return and pending error or ERR_OK if no error was pending
 */
pub fn netconn_err(conn: &mut NetConnDesc) {
    let err: err_t;
    SYS_ARCH_DECL_PROTECT(lev);
    if (conn == NULL) {
       return Ok(());
    }
    SYS_ARCH_PROTECT(lev);
    err = conn.pending_err;
    conn.pending_err = ERR_OK;
    SYS_ARCH_UNPROTECT(lev);
    return err;
}

/*
 * @ingroup netconn_tcp
 * Shut down one or both sides of a TCP netconn (doesn't delete it).
 *
 * @param conn the TCP netconn to shut down
 * @param shut_rx shut down the RX side (no more read possible after this)
 * @param shut_tx shut down the TX side (no more write possible after this)
 * @return ERR_OK if the netconn was closed, any other on: err_t error
 */
pub fn netconn_shutdown(conn: &mut NetConnDesc, shut_rx: u8, shut_tx: u8) {
    let mut read = 0u32;
    if shut_rx != 0 {
        read = NETCONN_SHUT_RD;
    }
    let mut write = 0u32;
    if shut_tx != 0 {
        write = NETCONN_SHUT_WR;
    }
    return netconn_close_shutdown(conn, read | write);
}

/*
 * @ingroup netconn_udp
 * Join multicast groups for UDP netconns.
 *
 * @param conn the UDP netconn for which to change multicast addresses
 * @param multiaddr IP address of the multicast group to join or leave
 * @param netif_addr the IP address of the network interface on which to send
 *                  the igmp message
 * @param join_or_leave flag whether to send a join- or leave-message
 * @return ERR_OK if the action was taken, any on: err_t error
 */
pub fn netconn_join_leave_group(
    conn: &mut NetConnDesc,
    multiaddr: &mut LwipAddr,
    netif_addr: &mut LwipAddr,
    join_or_leave: netconn_igmp,
) {
    let msg = api_msg;
    // API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    //  TODO:
    // LWIP_ERROR("netconn_join_leave_group: invalid conn",  (conn != NULL), return ERR_ARG;);

    API_MSG_VAR_ALLOC(msg);

    /* Don't propagate NULL pointer (IP_ADDR_ANY alias) to subsequent functions */
    if (multiaddr == NULL) {
        multiaddr = IP4_ADDR_ANY;
    }
    if (netif_addr == NULL) {
        netif_addr = IP4_ADDR_ANY;
    }

    msg.conn = conn;
    msg.msg.jl.multiaddr = (multiaddr);
    msg.msg.jl.netif_addr = (netif_addr);
    msg.msg.jl.join_or_leave = join_or_leave;
    err = netconn_apimsg(lwip_netconn_do_join_leave_group, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}
/*
 * @ingroup netconn_udp
 * Join multicast groups for UDP netconns.
 *
 * @param conn the UDP netconn for which to change multicast addresses
 * @param multiaddr IP address of the multicast group to join or leave
 * @param if_idx the index of the netif
 * @param join_or_leave flag whether to send a join- or leave-message
 * @return ERR_OK if the action was taken, any on: err_t error
 */
pub fn netconn_join_leave_group_netif(
    conn: &mut NetConnDesc,
    multiaddr: &mut LwipAddr,
    if_idx: u8,
    join_or_leave: netconn_igmp,
) {
    API_MSG_VAR_DECLARE(msg);
    let err: err_t;

    // TODO:
    // LWIP_ERROR("netconn_join_leave_group: invalid conn",  (conn != NULL), return ERR_ARG;);

    API_MSG_VAR_ALLOC(msg);

    /* Don't propagate NULL pointer (IP_ADDR_ANY alias) to subsequent functions */
    if (multiaddr == NULL) {
        multiaddr = IP4_ADDR_ANY;
    }
    if (if_idx == NETIF_NO_INDEX) {
        return ERR_IF;
    }

    msg.conn = conn;
    msg.msg.jl.multiaddr = (multiaddr);
    msg.msg.jl.if_idx = if_idx;
    msg.msg.jl.join_or_leave = join_or_leave;
    err = netconn_apimsg(lwip_netconn_do_join_leave_group_netif, &(msg));
    API_MSG_VAR_FREE(msg);

    return err;
}

/*
 * @ingroup netconn_common
 * Execute a DNS query, only one IP address is returned
 *
 * @param name a string representation of the DNS host name to query
 * @param addr a preallocated LwipAddr where to store the resolved IP address
 * @param dns_addrtype IP address type (IPv4 / IPv6)
 * @return ERR_OK: resolving succeeded
 *         ERR_MEM: memory error, try again later
 *         ERR_ARG: dns client not initialized or invalid hostname
 *         ERR_VAL: dns server response was invalid
 */

pub fn netconn_gethostbyname_addrtype(name: &String, addr: &mut LwipAddr, dns_addrtype: u8) {
    // API_VAR_DECLARE(struct dns_api_msg, msg);
    let msg: dns_api_msg;

    let sem: sys_sem_t;

    let err: err_t;
    let cberr: err_t;

    //  TODO:
    // LWIP_ERROR("netconn_gethostbyname: invalid name", (name != NULL), return ERR_ARG;);
    //  TODO
    // LWIP_ERROR("netconn_gethostbyname: invalid addr", (addr != NULL), return ERR_ARG;);

    if (strlen(name) >= DNS_MAX_NAME_LENGTH) {
        return ERR_ARG;
    }

    if (LWIP_HOOK_NETCONN_EXTERNAL_RESOLVE(name, addr, dns_addrtype, &err)) {
        // if (LWIP_HOOK_NETCONN_EXTERNAL_RESOLVE(name, addr, NETCONN_DNS_DEFAULT, &err)) {

        return err;
    }

    // API_VAR_ALLOC(struct dns_api_msg, MEMP_DNS_API_MSG, msg, ERR_MEM);

    strncpy(msg.name, name, DNS_MAX_NAME_LENGTH - 1);
    msg.name[DNS_MAX_NAME_LENGTH - 1] = 0;
    // #else /* LWIP_MPU_COMPATIBLE */
    msg.err = &err;
    msg.sem = &sem;
    msg.addr = (addr);
    msg.name = name;

    msg.dns_addrtype = dns_addrtype;

    msg.sem = LWIP_NETCONN_THREAD_SEM_GET();
    // #else /* LWIP_NETCONN_SEM_PER_THREAD*/
    err = sys_sem_new(API_EXPR_REF(msg.sem), 0);
    if (err != ERR_OK) {
        API_VAR_FREE(MEMP_DNS_API_MSG, msg);
        return err;
    }

    cberr = tcpip_send_msg_wait_sem(lwip_netconn_do_gethostbyname, &(msg), API_EXPR_REF(msg.sem));

    sys_sem_free(API_EXPR_REF(msg.sem));

    if (cberr != ERR_OK) {
        API_VAR_FREE(MEMP_DNS_API_MSG, msg);
        return cberr;
    }

    *addr = msg.addr;
    err = msg.err;

    API_VAR_FREE(MEMP_DNS_API_MSG, msg);
    return err;
}

pub fn netconn_thread_init() {
    sys_sem_t * sem = LWIP_NETCONN_THREAD_SEM_GET();
    if ((sem == NULL) || !sys_sem_valid(sem)) {
        /* call alloc only once */
        LWIP_NETCONN_THREAD_SEM_ALLOC();
        LWIP_ASSERT(
            "LWIP_NETCONN_THREAD_SEM_ALLOC() failed",
            sys_sem_valid(LWIP_NETCONN_THREAD_SEM_GET()),
        );
    }
}

pub fn netconn_thread_cleanup() {
    sys_sem_t * sem = LWIP_NETCONN_THREAD_SEM_GET();
    if ((sem != NULL) && sys_sem_valid(sem)) {
        /* call free only once */
        LWIP_NETCONN_THREAD_SEM_FREE();
    }
}
