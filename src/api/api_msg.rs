use crate::core::{
    api_h::{NetConnDesc, NETCONN_FLAG_IN_NONBLOCKING_CONNECT},
    api_msg_h::api_msg,
    err_h::ERR_WOULDBLOCK,
};

/*
 * @file
 * Sequential API Internal module
 *
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

/* netconns are polled once per second (e.g. continue write on memory error) */
pub const NETCONN_TCP_POLL_INTERVAL: u64 = 2;

pub fn SET_NONBLOCKING_CONNECT(conn: NetConnDesc, val: u32) {
    if (val) {
        netconn_set_flags(conn, NETCONN_FLAG_IN_NONBLOCKING_CONNECT);
    } else {
        netconn_clear_flags(conn, NETCONN_FLAG_IN_NONBLOCKING_CONNECT);
    }
}

pub fn IN_NONBLOCKING_CONNECT(conn: NetConnDesc) -> bool {
    netconn_is_flag_set(conn, NETCONN_FLAG_IN_NONBLOCKING_CONNECT)
}

pub fn NETCONN_MBOX_VALID(conn: NetConnDesc, mbox: mbox) -> bool {
    (sys_mbox_valid(mbox) && ((conn.flags & NETCONN_FLAG_MBOXINVALID) == 0))
}

// #define NETCONN_MBOX_VALID(conn, mbox) sys_mbox_valid(mbox)

/* forward declarations */

// pub const WRITE_DELAYED: u8 =1;
// #define WRITE_DELAYED_PARAM   , delayed: u8
/* LWIP_TCPIP_CORE_LOCKING */
// #define WRITE_DELAYED
// #define WRITE_DELAYED_PARAM

// static lwip_netconn_do_writemore: err_t(conn: &mut netconn  WRITE_DELAYED_PARAM);
// static lwip_netconn_do_close_internal: err_t(conn: &mut netconn  WRITE_DELAYED_PARAM);

fn netconn_drain(conn: &mut NetConnDesc);

// #define TCPIP_APIMSG_ACK(m)
/* LWIP_TCPIP_CORE_LOCKING */
pub fn TCPIP_APIMSG_ACK(m: u32) {
    sys_sem_signal(LWIP_API_MSG_SEM(m));
}

pub const netconn_deleted: u8 = 0;

pub fn lwip_netconn_is_deallocated_msg(msg: &mut ()) {
    if (msg == &netconn_deleted) {
        return 1;
    }
    return 0;
}

const netconn_aborted: u8 = 0;
const netconn_reset: u8 = 0;
const netconn_closed: u8 = 0;

/* Translate an error to a unique void* passed via an mbox */
pub fn lwip_netconn_err_to_msg(err: err_t) {
    match (err) {
        ERR_ABRT => return &netconn_aborted,
        ERR_RST => return &netconn_reset,
        ERR_CLSD => return &netconn_closed,
        _ => {
            LWIP_ASSERT("unhandled error", err == ERR_OK);
            return NULL;
        }
    }
}

pub fn lwip_netconn_is_err_msg(msg: &mut (), err: &mut err_t) {
    LWIP_ASSERT("err != NULL", err != NULL);

    if (msg == &netconn_aborted) {
        *err = ERR_ABRT;
        return 1;
    } else if (msg == &netconn_reset) {
        *err = ERR_RST;
        return 1;
    } else if (msg == &netconn_closed) {
        *err = ERR_CLSD;
        return 1;
    }
    return 0;
}

/*
 * Receive callback function for RAW netconns.
 * Doesn't 'eat' the packet, only copies it and sends it to
 * conn.recvmbox
 *
 * @see raw.h (struct raw_pcb.recv) for parameters and return value
 */
pub fn recv_raw(arg: &mut Vec<u8>, pcb: &mut raw_pcb, p: &mut pbuf, addr: &mut LwipAddr) -> u8 {
    let q: &mut pbuf;
    let buf: &mut netbuf;
    let conn: &mut NetConnDesc;
    conn = arg;

    if ((conn != NULL) && NETCONN_MBOX_VALID(conn, &conn.recvmbox)) {
        let recv_avail: i32;
        SYS_ARCH_GET(conn.recv_avail, recv_avail);
        if ((recv_avail + (p.tot_len)) > conn.recv_bufsize) {
            return 0;
        }

        /* copy the whole packet into new pbufs */
        q = pbuf_clone(PBUF_RAW, PBUF_RAM, p);
        if (q != NULL) {
            let len: usize;
            buf = memp_malloc(MEMP_NETBUF);
            if (buf == NULL) {
                pbuf_free(q);
                return 0;
            }

            buf.p = q;
            buf.ptr = q;
            ip_addr_copy(buf.addr, *ip_current_src_addr());
            buf.port = pcb.protocol;

            len = q.tot_len;
            if (sys_mbox_trypost(&conn.recvmbox, buf) != ERR_OK) {
                netbuf_delete(buf);
                return 0;
            } else {
                SYS_ARCH_INC(conn.recv_avail, len);

                /* Register event with callback */
                API_EVENT(conn, NETCONN_EVT_RCVPLUS, len);
            }
        }
    }

    return 0; /* do not eat the packet */
}

/*
 * Receive callback function for UDP netconns.
 * Posts the packet to conn.recvmbox or deletes it on memory error.
 *
 * @see udp.h (struct udp_pcb.recv) for parameters
 */
pub fn recv_udp(
    arg: &mut Vec<u8>,
    pcb: &mut udp_pcb,
    p: &mut pbuf,
    addr: &mut LwipAddr,
    port: u16,
) {
    let buf: &mut netbuf;
    let conn: &mut NetConnDesc;
    let len: usize;
    let recv_avail: i32;

    /* only used for asserts... */
    LWIP_ASSERT("recv_udp must have a pcb argument", pcb != NULL);
    LWIP_ASSERT("recv_udp must have an argument", arg != NULL);
    conn = arg;

    if (conn == NULL) {
        pbuf_free(p);
        return;
    }

    LWIP_ASSERT("recv_udp: recv for wrong pcb!", conn.pcb.udp == pcb);

    SYS_ARCH_GET(conn.recv_avail, recv_avail);
    if (!NETCONN_MBOX_VALID(conn, &conn.recvmbox)
        || ((recv_avail + (p.tot_len)) > conn.recv_bufsize))
    {
        /* LWIP_SO_RCVBUF */
        if (!NETCONN_MBOX_VALID(conn, &conn.recvmbox)) {
            pbuf_free(p);
            return;
        }

        buf = memp_malloc(MEMP_NETBUF);
        if (buf == NULL) {
            pbuf_free(p);
            return;
        }
    } else {
        buf.p = p;
        buf.ptr = p;
        ip_addr_set(&buf.addr, addr);
        buf.port = port;

        if (conn.flags & NETCONN_FLAG_PKTINFO) {
            /* get the UDP header - always in the first pbuf, ensured by udp_input */
            const udphdr: &mut udp_hdr = ip_next_header_ptr();
            buf.flags = NETBUF_FLAG_DESTADDR;
            ip_addr_set(&buf.toaddr, ip_current_dest_addr());
            buf.toport_chksum = udphdr.dest;
        }
    }

    len = p.tot_len;
    if (sys_mbox_trypost(&conn.recvmbox, buf) != ERR_OK) {
        netbuf_delete(buf);
        return;
    } else {
        SYS_ARCH_INC(conn.recv_avail, len);

        /* Register event with callback */
        API_EVENT(conn, NETCONN_EVT_RCVPLUS, len);
    }
}

/*
 * Receive callback function for TCP netconns.
 * Posts the packet to conn.recvmbox, but doesn't delete it on errors.
 *
 * @see tcp.h (struct tcp_pcb.recv) for parameters and return value
 */
pub fn recv_tcp(
    arg: &mut Vec<u8>,
    pcb: &mut tcp_pcb,
    p: &mut pbuf,
    err: err_t,
) -> Result<(), LwipError> {
    let conn: &mut NetConnDesc;
    let len: usize;
    let msg: &mut ();

    LWIP_ASSERT("recv_tcp must have a pcb argument", pcb != NULL);
    LWIP_ASSERT("recv_tcp must have an argument", arg != NULL);
    LWIP_ASSERT("err != ERR_OK unhandled", err == ERR_OK);
    /* for LWIP_NOASSERT */
    conn = arg;

    if (conn == NULL) {
        return ERR_VAL;
    }
    LWIP_ASSERT("recv_tcp: recv for wrong pcb!", conn.pcb.tcp == pcb);

    if (!NETCONN_MBOX_VALID(conn, &conn.recvmbox)) {
        /* recvmbox already deleted */
        if (p != NULL) {
            tcp_recved(pcb, p.tot_len);
            pbuf_free(p);
        }
       return Ok(());
    }
    /* Unlike for UDP or RAW pcbs, don't check for available space
    using recv_avail since that could break the connection
    (data is already ACKed) */

    if (p != NULL) {
        msg = p;
        len = p.tot_len;
    } else {
        msg = &netconn_closed;
        len = 0;
    }

    if (sys_mbox_trypost(&conn.recvmbox, msg) != ERR_OK) {
        /* don't deallocate p: it is presented to us later again from tcp_fasttmr! */
        return ERR_MEM;
    } else {
        SYS_ARCH_INC(conn.recv_avail, len);

        /* Register event with callback */
        API_EVENT(conn, NETCONN_EVT_RCVPLUS, len);
    }

   return Ok(());
}

/*
 * Poll callback function for TCP netconns.
 * Wakes up an application thread that waits for a connection to close
 * or data to be sent. The application thread then takes the
 * appropriate action to go on.
 *
 * Signals the conn.sem.
 * netconn_close waits for conn.sem if closing failed.
 *
 * @see tcp.h (struct tcp_pcb.poll) for parameters and return value
 */
pub fn poll_tcp(arg: &mut Vec<u8>, pcb: &mut tcp_pcb) -> Result<(), LwipError> {
    let conn: &mut NetConnDesc = arg;

    LWIP_ASSERT("conn != NULL", (conn != NULL));

    if (conn.state == NETCONN_WRITE) {
        lwip_netconn_do_writemore(conn, WRITE_DELAYED);
    } else if (conn.state == NETCONN_CLOSE) {
        if (conn.current_msg && conn.current_msg.msg.sd.polls_left) {
            conn.current_msg.msg.sd.polls_left -= 1;
        }

        lwip_netconn_do_close_internal(conn, WRITE_DELAYED);
    }
    /* @todo: implement connect timeout here? */

    /* Did a nonblocking write fail before? Then check available write-space. */
    if (conn.flags & NETCONN_FLAG_CHECK_WRITESPACE) {
        /* If the queued byte- or pbuf-count drops below the configured low-water limit,
        let select mark this pcb as writable again. */
        if ((conn.pcb.tcp != NULL)
            && (tcp_sndbuf(conn.pcb.tcp) > TCP_SNDLOWAT)
            && (tcp_sndqueuelen(conn.pcb.tcp) < TCP_SNDQUEUELOWAT))
        {
            netconn_clear_flags(conn, NETCONN_FLAG_CHECK_WRITESPACE);
            API_EVENT(conn, NETCONN_EVT_SENDPLUS, 0);
        }
    }

   return Ok(());
}

/*
 * Sent callback function for TCP netconns.
 * Signals the conn.sem and calls API_EVENT.
 * netconn_write waits for conn.sem if send buffer is low.
 *
 * @see tcp.h (struct tcp_pcb.sent) for parameters and return value
 */
pub fn sent_tcp(arg: &mut Vec<u8>, pcb: &mut tcp_pcb, len: usize) -> Result<(), LwipError> {
    let conn: &mut NetConnDesc = arg;

    LWIP_ASSERT("conn != NULL", (conn != NULL));

    if (conn) {
        if (conn.state == NETCONN_WRITE) {
            lwip_netconn_do_writemore(conn, WRITE_DELAYED);
        } else if (conn.state == NETCONN_CLOSE) {
            lwip_netconn_do_close_internal(conn, WRITE_DELAYED);
        }

        /* If the queued byte- or pbuf-count drops below the configured low-water limit,
        let select mark this pcb as writable again. */
        if ((conn.pcb.tcp != NULL)
            && (tcp_sndbuf(conn.pcb.tcp) > TCP_SNDLOWAT)
            && (tcp_sndqueuelen(conn.pcb.tcp) < TCP_SNDQUEUELOWAT))
        {
            netconn_clear_flags(conn, NETCONN_FLAG_CHECK_WRITESPACE);
            API_EVENT(conn, NETCONN_EVT_SENDPLUS, len);
        }
    }

   return Ok(());
}

/*
 * Error callback function for TCP netconns.
 * Signals conn.sem, posts to all conn mboxes and calls API_EVENT.
 * The application thread has then to decide what to do.
 *
 * @see tcp.h (struct tcp_pcb.err) for parameters
 */
pub fn cp(arg: &mut Vec<u8>, err: err_t) {
    let conn: &mut NetConnDesc;
    let old_state: netconn_state;
    let mbox_msg: &mut ();
    SYS_ARCH_DECL_PROTECT(lev);

    conn = arg;
    LWIP_ASSERT("conn != NULL", (conn != NULL));

    SYS_ARCH_PROTECT(lev);

    /* when err is called, the pcb is deallocated, so delete the reference */
    conn.pcb.tcp = NULL;
    /* store pending error */
    conn.pending_err = err;
    /* prevent application threads from blocking on 'recvmbox'/'acceptmbox' */
    conn.flags |= NETCONN_FLAG_MBOXCLOSED;

    /* reset conn.state now before waking up other threads */
    old_state = conn.state;
    conn.state = NETCONN_NONE;

    SYS_ARCH_UNPROTECT(lev);

    /* Notify the user layer about a connection error. Used to signal select. */
    API_EVENT(conn, NETCONN_EVT_ERROR, 0);
    /* Try to release selects pending on 'read' or 'write', too.
    They will get an error if they actually try to read or write. */
    API_EVENT(conn, NETCONN_EVT_RCVPLUS, 0);
    API_EVENT(conn, NETCONN_EVT_SENDPLUS, 0);

    mbox_msg = &mut lwip_netconn_err_to_msg(err);
    /* pass error message to recvmbox to wake up pending recv */
    if (NETCONN_MBOX_VALID(conn, &conn.recvmbox)) {
        /* use trypost to prevent deadlock */
        sys_mbox_trypost(&conn.recvmbox, mbox_msg);
    }
    /* pass error message to acceptmbox to wake up pending accept */
    if (NETCONN_MBOX_VALID(conn, &conn.acceptmbox)) {
        /* use trypost to preven deadlock */
        sys_mbox_trypost(&conn.acceptmbox, mbox_msg);
    }

    if ((old_state == NETCONN_WRITE)
        || (old_state == NETCONN_CLOSE)
        || (old_state == NETCONN_CONNECT))
    {
        /* calling lwip_netconn_do_writemore/lwip_netconn_do_close_internal is not necessary
        since the pcb has already been deleted! */
        let was_nonblocking_connect: i32 = IN_NONBLOCKING_CONNECT(conn);
        SET_NONBLOCKING_CONNECT(conn, 0);

        if (!was_nonblocking_connect) {
            sys_sem_t * op_completed_sem;
            /* set error return code */
            LWIP_ASSERT("conn.current_msg != NULL", conn.current_msg != NULL);
            if (old_state == NETCONN_CLOSE) {
                /* let close succeed: the connection is closed after all... */
                conn.current_msg.err = ERR_OK;
            } else {
                /* Write and connect fail */
                conn.current_msg.err = err;
            }
            op_completed_sem = LWIP_API_MSG_SEM(conn.current_msg);
            LWIP_ASSERT("inavlid op_completed_sem", sys_sem_valid(op_completed_sem));
            conn.current_msg = NULL;
            /* wake up the waiting task */
            sys_sem_signal(op_completed_sem);
        } else {
            /* @todo: test what happens for error on nonblocking connect */
        }
    } else {
        LWIP_ASSERT("conn.current_msg == NULL", conn.current_msg == NULL);
    }
}

/*
 * Setup a tcp_pcb with the correct callback function pointers
 * and their arguments.
 *
 * @param conn the TCP netconn to setup
 */
pub fn setup_tcp(conn: &mut NetConnDesc) {
    let pcb: &mut tcp_pcb;

    pcb = conn.pcb.tcp;
    tcp_arg(pcb, conn);
    tcp_recv(pcb, recv_tcp);
    tcp_sent(pcb, sent_tcp);
    tcp_poll(pcb, poll_tcp, NETCONN_TCP_POLL_INTERVAL);
    tcp_err(pcb, err_tcp);
}

/*
 * Accept callback function for TCP netconns.
 * Allocates a new netconn and posts that to conn.acceptmbox.
 *
 * @see tcp.h (struct tcp_pcb_listen.accept) for parameters and return value
 */
pub fn accept_function(
    arg: &mut Vec<u8>,
    newpcb: &mut tcp_pcb,
    err: err_t,
) -> Result<(), LwipError> {
    let newconn: &mut NetConnDesc;
    let conn: &mut NetConnDesc = arg;

    if (conn == NULL) {
        return ERR_VAL;
    }
    if (!NETCONN_MBOX_VALID(conn, &conn.acceptmbox)) {
        /*LWIP_DEBUGF(
            API_MSG_DEBUG,
            ("accept_function: acceptmbox already deleted\n"),
        );*/
        return ERR_VAL;
    }

    if (newpcb == NULL) {
        /* out-of-pcbs during connect: pass on this error to the application */
        if (sys_mbox_trypost(&conn.acceptmbox, lwip_netconn_err_to_msg(ERR_ABRT)) == ERR_OK) {
            /* Register event with callback */
            API_EVENT(conn, NETCONN_EVT_RCVPLUS, 0);
        }
        return ERR_VAL;
    }
    LWIP_ASSERT("expect newpcb == NULL or err == ERR_OK", err == ERR_OK);
    /* for LWIP_NOASSERT */
    /*LWIP_DEBUGF(
        API_MSG_DEBUG,
        (
            "accept_function: newpcb.state: %s\n",
            tcp_debug_state_str(newpcb.state),
        ),
    );*/

    /* We have to set the callback here even though
     * the new socket is unknown. newconn.socket is marked as -1. */
    // TODO:
    // newconn = netconn_alloc(conn.netconntype, conn.callback);
    if (newconn == NULL) {
        /* outof netconns: pass on this error to the application */
        if (sys_mbox_trypost(&conn.acceptmbox, lwip_netconn_err_to_msg(ERR_ABRT)) == ERR_OK) {
            /* Register event with callback */
            API_EVENT(conn, NETCONN_EVT_RCVPLUS, 0);
        }
        return ERR_MEM;
    }
    newconn.pcb.tcp = newpcb;
    setup_tcp(newconn);

    /* handle backlog counter */
    tcp_backlog_delayed(newpcb);

    if (sys_mbox_trypost(&conn.acceptmbox, newconn) != ERR_OK) {
        /* When returning != ERR_OK, the pcb is aborted in tcp_process(),
        so do nothing here! */
        /* remove all references to this netconn from the pcb */
        let pcb: &mut tcp_pcb = newconn.pcb.tcp;
        tcp_arg(pcb, NULL);
        tcp_recv(pcb, NULL);
        tcp_sent(pcb, NULL);
        tcp_poll(pcb, NULL, 0);
        tcp_err(pcb, NULL);
        /* remove reference from to the pcb from this netconn */
        newconn.pcb.tcp = NULL;
        /* no need to drain since we know the recvmbox is empty. */
        sys_mbox_free(&newconn.recvmbox);
        sys_mbox_set_invalid(&newconn.recvmbox);
        netconn_free(newconn);
        return ERR_MEM;
    } else {
        /* Register event with callback */
        API_EVENT(conn, NETCONN_EVT_RCVPLUS, 0);
    }

   return Ok(());
}

/*
 * Create a new pcb of a specific type.
 * Called from lwip_netconn_do_newconn().
 *
 * @param msg the api_msg describing the connection type
 */
pub fn pcb_new(msg: &mut api_msg) {
    let iptype: lwip_LwipAddrype = IPADDR_TYPE_V4;

    LWIP_ASSERT("pcb_new: pcb already allocated", msg.conn.pcb.tcp == NULL);

    /* IPv6: Dual-stack by default, unless netconn_set_ipv6only() is called */
    if (NETCONNTYPE_ISIPV6(netconn_type(msg.conn))) {
        iptype = IPADDR_TYPE_ANY;
    }

    /* Allocate a PCB for this connection */
    match (NETCONNTYPE_GROUP(msg.conn.netconn_type)) {
        NETCONN_RAW => {
            msg.conn.pcb.raw = raw_new_ip_type(iptype, msg.msg.n.proto);
            if (msg.conn.pcb.raw != NULL) {
                /* ICMPv6 packets should always have checksum calculated by the stack as per RFC 3542 chapter 3.1 */
                if (NETCONNTYPE_ISIPV6(msg.conn.netconn_type)
                    && msg.conn.pcb.raw.protocol == IP6_NEXTH_ICMP6)
                {
                    msg.conn.pcb.raw.chksum_reqd = 1;
                    msg.conn.pcb.raw.chksum_offset = 2;
                }

                raw_recv(msg.conn.pcb.raw, recv_raw, msg.conn);
            }
        }

        NETCONN_UDP => {
            msg.conn.pcb.udp = udp_new_ip_type(iptype);
            if (msg.conn.pcb.udp != NULL) {
                if (NETCONNTYPE_ISUDPLITE(msg.conn.netconn_type)) {
                    udp_setflags(msg.conn.pcb.udp, UDP_FLAGS_UDPLITE);
                }

                if (NETCONNTYPE_ISUDPNOCHKSUM(msg.conn.netconn_type)) {
                    udp_setflags(msg.conn.pcb.udp, UDP_FLAGS_NOCHKSUM);
                }
                udp_recv(msg.conn.pcb.udp, recv_udp, msg.conn);
            }
        }

        NETCONN_TCP => {
            msg.conn.pcb.tcp = tcp_new_ip_type(iptype);
            if (msg.conn.pcb.tcp != NULL) {
                setup_tcp(msg.conn);
            }
        }

        _ => {
            /* Unsupported netconn type, e.g. protocol disabled */
            msg.err = ERR_VAL;
            return;
        }
    }
    if (msg.conn.pcb.ip == NULL) {
        msg.err = ERR_MEM;
    }
}

/*
 * Create a new pcb of a specific type inside a netconn.
 * Called from netconn_new_with_proto_and_callback.
 *
 * @param m the api_msg describing the connection type
 */
pub fn lwip_netconn_do_newconn(m: &mut ()) {
    let msg: &mut api_msg = m;

    msg.err = ERR_OK;
    if (msg.conn.pcb.tcp == NULL) {
        pcb_new(msg);
    }
    /* Else? This "new" connection already has a PCB allocated. */
    /* Is this an error condition? Should it be deleted? */
    /* We currently just are happy and return. */

    TCPIP_APIMSG_ACK(msg);
}

/*
 * Create a new netconn (of a specific type) that has a callback function.
 * The corresponding pcb is NOT created!
 *
 * @param t the type of 'connection' to create (@see enum netconn_type)
 * @param callback a function to call on status changes (RX available, TX'ed)
 * @return a newly allocated struct netconn or
 *         NULL on memory error
 */
pub fn netconn_alloc(t: netconn_type, callback: netconn_callback) -> NetConnDesc {
    let conn: &mut NetConnDesc;
    let size: i32;
    let init_flags: u8 = 0;

    conn = memp_malloc(MEMP_NETCONN);
    if (conn == NULL) {
        return NULL;
    }

    conn.pending_err = ERR_OK;
    conn.netconn_callback = t;
    conn.pcb.tcp = NULL;

    /* If all sizes are the same, every compiler should optimize this match to nothing */
    match (NETCONNTYPE_GROUP(t)) {
        NETCONN_RAW => size = DEFAULT_RAW_RECVMBOX_SIZE,
        NETCONN_UDP => {
            size = DEFAULT_UDP_RECVMBOX_SIZE;
            init_flags |= NETCONN_FLAG_PKTINFO;
        }
        NETCONN_TCP => size = DEFAULT_TCP_RECVMBOX_SIZE,

        _ => {
            LWIP_ASSERT("netconn_alloc: undefined netconn_type", 0);
        } // goto free_and_return;
    }

    if (sys_mbox_new(&conn.recvmbox, size) != ERR_OK) {
        // goto free_and_return;
    }

    if (sys_sem_new(&conn.op_completed, 0) != ERR_OK) {
        sys_mbox_free(&conn.recvmbox);
        // goto free_and_return;
    }

    sys_mbox_set_invalid(&conn.acceptmbox);

    conn.state = NETCONN_NONE;
    /* initialize socket to -1 since 0 is a valid socket */
    conn.socket = -1;
    conn.callback = callback;
    conn.current_msg = NULL;
    conn.send_timeout = 0;
    conn.recv_timeout = 0;
    conn.recv_bufsize = RECV_BUFSIZE_DEFAULT;
    conn.recv_avail = 0;
    conn.linger = -1;
    conn.flags = init_flags;
    return conn;
    // free_and_return:
    memp_free(MEMP_NETCONN, conn);
    return NULL;
}

/*
 * Delete a netconn and all its resources.
 * The pcb is NOT freed (since we might not be in the right thread context do this).
 *
 * @param conn the netconn to free
 */
pub fn netconn_free(conn: &mut NetConnDesc) {
    LWIP_ASSERT(
        "PCB must be deallocated outside this function",
        conn.pcb.tcp == NULL,
    );

    /* in fullduplex, netconn is drained here */
    netconn_drain(conn);

    LWIP_ASSERT(
        "recvmbox must be deallocated before calling this function",
        !sys_mbox_valid(&conn.recvmbox),
    );

    LWIP_ASSERT(
        "acceptmbox must be deallocated before calling this function",
        !sys_mbox_valid(&conn.acceptmbox),
    );

    sys_sem_free(&conn.op_completed);
    sys_sem_set_invalid(&conn.op_completed);

    memp_free(MEMP_NETCONN, conn);
}

/*
 * Delete rcvmbox and acceptmbox of a netconn and free the left-over data in
 * these mboxes
 *
 * @param conn the netconn to free
 * @bytes_drained bytes drained from recvmbox
 * @accepts_drained pending connections drained from acceptmbox
 */
pub fn netconn_drain(conn: &mut NetConnDesc) {
    let mem: &mut ();

    /* This runs when mbox and netconn are marked as closed,
    so we don't need to lock against rx packets */

    LWIP_ASSERT(
        "netconn marked closed",
        conn.flags & NETCONN_FLAG_MBOXINVALID,
    );

    /* Delete and drain the recvmbox. */
    if (sys_mbox_valid(&conn.recvmbox)) {
        while (sys_mbox_tryfetch(&conn.recvmbox, &mem) != SYS_MBOX_EMPTY) {
            if (!lwip_netconn_is_deallocated_msg(mem)) {
                if (NETCONNTYPE_GROUP(conn.netconntype) == NETCONN_TCP) {
                    let err: err_t;
                    if (!lwip_netconn_is_err_msg(mem, &err)) {
                        pbuf_free(mem);
                    }
                } else {
                    netbuf_delete(mem);
                }
            }
        }
        sys_mbox_free(&conn.recvmbox);
        sys_mbox_set_invalid(&conn.recvmbox);
    }

    /* Delete and drain the acceptmbox. */

    if (sys_mbox_valid(&conn.acceptmbox)) {
        while (sys_mbox_tryfetch(&conn.acceptmbox, &mem) != SYS_MBOX_EMPTY) {
            if (!lwip_netconn_is_deallocated_msg(mem)) {
                let err: err_t;
                if (!lwip_netconn_is_err_msg(mem, &err)) {
                    let newconn: &mut NetConnDesc = mem;
                    /* Only tcp pcbs have an acceptmbox, so no need to check conn.netconntype */
                    /* pcb might be set to NULL already by err_tcp() */
                    /* drain recvmbox */
                    netconn_drain(newconn);
                    if (newconn.pcb.tcp != NULL) {
                        tcp_abort(newconn.pcb.tcp);
                        newconn.pcb.tcp = NULL;
                    }
                    netconn_free(newconn);
                }
            }
        }
        sys_mbox_free(&conn.acceptmbox);
        sys_mbox_set_invalid(&conn.acceptmbox);
    }
}

pub fn netconn_mark_mbox_invalid(conn: &mut NetConnDesc) {
    let i: i32;
    let num_waiting;
    let msg: &mut () = &netconn_deleted;

    /* Prevent new calls/threads from reading from the mbox */
    conn.flags |= NETCONN_FLAG_MBOXINVALID;

    SYS_ARCH_LOCKED(num_waiting = conn.mbox_threads_waiting);
    // for (i = 0; i < num_waiting; i+= 1) {
    //   if (sys_mbox_valid_val(conn.recvmbox)) {
    //     sys_mbox_trypost(&conn.recvmbox, msg);
    //   } else {
    //     sys_mbox_trypost(&conn.acceptmbox, msg);
    //   }
    // }
}

/*
 * Internal helper function to close a TCP netconn: since this sometimes
 * doesn't work at the first attempt, this function is called from multiple
 * places.
 *
 * @param conn the TCP netconn to close
 */
pub fn lwip_netconn_do_close_internal(conn: &mut NetConnDesc) -> Result<(), LwipError> {
    let err: err_t;
    let shut: u8;
    let shut_rx: u8;
    let shut_tx;
    let shut_close;
    let close_finished: u8 = 0;
    let tpcb: &mut tcp_pcb;

    let linger_wait_required: u8 = 0;

    LWIP_ASSERT("invalid conn", (conn != NULL));
    LWIP_ASSERT(
        "this is for tcp netconns only",
        (NETCONNTYPE_GROUP(conn.netconntype) == NETCONN_TCP),
    );
    LWIP_ASSERT(
        "conn must be in state NETCONN_CLOSE",
        (conn.state == NETCONN_CLOSE),
    );
    LWIP_ASSERT("pcb already closed", (conn.pcb.tcp != NULL));
    LWIP_ASSERT("conn.current_msg != NULL", conn.current_msg != NULL);

    tpcb = conn.pcb.tcp;
    shut = conn.current_msg.msg.sd.shut;
    shut_rx = shut & NETCONN_SHUT_RD;
    shut_tx = shut & NETCONN_SHUT_WR;
    /* shutting down both ends is the same as closing
    (also if RD or WR side was shut down before already) */
    if (shut == NETCONN_SHUT_RDWR) {
        shut_close = 1;
    } else if (shut_rx
        && ((tpcb.state == FIN_WAIT_1) || (tpcb.state == FIN_WAIT_2) || (tpcb.state == CLOSING)))
    {
        shut_close = 1;
    } else if (shut_tx && ((tpcb.flags & TF_RXCLOSED) != 0)) {
        shut_close = 1;
    } else {
        shut_close = 0;
    }

    /* Set back some callback pointers */
    if (shut_close) {
        tcp_arg(tpcb, NULL);
    }
    if (tpcb.state == LISTEN) {
        tcp_accept(tpcb, NULL);
    } else {
        /* some callbacks have to be reset if tcp_close is not successful */
        if (shut_rx) {
            tcp_recv(tpcb, NULL);
            tcp_accept(tpcb, NULL);
        }
        if (shut_tx) {
            tcp_sent(tpcb, NULL);
        }
        if (shut_close) {
            tcp_poll(tpcb, NULL, 0);
            tcp_err(tpcb, NULL);
        }
    }
    /* Try to close the connection */
    if (shut_close) {
        /* check linger possibilites before calling tcp_close */
        err = ERR_OK;
        /* linger enabled/required at all? (i.e. is there untransmitted data left?) */
        if ((conn.linger >= 0) && (conn.pcb.tcp.unsent || conn.pcb.tcp.unacked)) {
            if (conn.linger == 0) {
                /* data left but linger prevents waiting */
                tcp_abort(tpcb);
                tpcb = NULL;
            } else if (conn.linger > 0) {
                /* data left and linger says we should wait */
                if (netconn_is_nonblocking(conn)) {
                    /* data left on a nonblocking netconn -> cannot linger */
                    err = ERR_WOULDBLOCK;
                } else if ((sys_now() - conn.current_msg.msg.sd.time_started)
                    >= (conn.linger * 1000))
                {
                    /* data left but linger timeout has expired (this happens on further
                    calls to this function through poll_tcp */
                    tcp_abort(tpcb);
                    tpcb = NULL;
                } else {
                    /* data left -> need to wait for ACK after successful close */
                    linger_wait_required = 1;
                }
            }
        }
        if ((err == ERR_OK) && (tpcb != NULL)) {
            err = tcp_close(tpcb);
        }
    } else {
        err = tcp_shutdown(tpcb, shut_rx, shut_tx);
    }
    if (err == ERR_OK) {
        close_finished = 1;

        if (linger_wait_required) {
            /* wait for ACK of all unsent/unacked data by just getting called again */
            close_finished = 0;
            err = ERR_INPROGRESS;
        }
    } else {
        if (err == ERR_MEM) {
            /* Closing failed because of memory shortage, try again later. Even for
            nonblocking netconns, we have to wait since no standard socket application
            is prepared for close failing because of resource shortage.
            Check the timeout: this is kind of an lwip addition to the standard sockets:
            we wait for some time when failing to allocate a segment for the FIN */

            let close_timeout: i32 = LWIP_TCP_CLOSE_TIMEOUT_MS_DEFAULT;

            if (conn.send_timeout > 0) {
                close_timeout = conn.send_timeout;
            }

            if (conn.linger >= 0) {
                /* use linger timeout (seconds) */
                close_timeout = conn.linger * 1000;
            }

            if ((sys_now() - conn.current_msg.msg.sd.time_started) >= close_timeout) {
                /* LWIP_SO_SNDTIMEO || LWIP_SO_LINGER */
                if (conn.current_msg.msg.sd.polls_left == 0) {
                    close_finished = 1;
                    if (shut_close) {
                        /* in this case, we want to RST the connection */
                        tcp_abort(tpcb);
                        err = ERR_OK;
                    }
                }
            }
        } else {
            /* Closing failed for a non-memory error: give up */
            close_finished = 1;
        }
    }
    if (close_finished) {
        /* Closing done (succeeded, non-memory error, nonblocking error or timeout) */
        sys_sem_t * op_completed_sem = LWIP_API_MSG_SEM(conn.current_msg);
        conn.current_msg.err = err;
        conn.current_msg = NULL;
        conn.state = NETCONN_NONE;
        if (err == ERR_OK) {
            if (shut_close) {
                /* Set back some callback pointers as conn is going away */
                conn.pcb.tcp = NULL;
                /* Trigger select() in socket layer. Make sure everybody notices activity
                on the connection, error first! */
                API_EVENT(conn, NETCONN_EVT_ERROR, 0);
            }
            if (shut_rx) {
                API_EVENT(conn, NETCONN_EVT_RCVPLUS, 0);
            }
            if (shut_tx) {
                API_EVENT(conn, NETCONN_EVT_SENDPLUS, 0);
            }
        }

        if (delayed) {
            /* wake up the application task */
            sys_sem_signal(op_completed_sem);
        }
       return Ok(());
    }
    if (!close_finished) {
        /* Closing failed and we want to wait: restore some of the callbacks */
        /* Closing of listen pcb will never fail! */
        LWIP_ASSERT("Closing a listen pcb may not fail!", (tpcb.state != LISTEN));
        if (shut_tx) {
            tcp_sent(tpcb, sent_tcp);
        }
        /* when waiting for close, set up poll interval to 500ms */
        tcp_poll(tpcb, poll_tcp, 1);
        tcp_err(tpcb, err_tcp);
        tcp_arg(tpcb, conn);
        /* don't restore recv callback: we don't want to receive any more data */
    }
    /* If closing didn't succeed, we get called again either
    from poll_tcp or from sent_tcp */
    LWIP_ASSERT("err != ERR_OK", err != ERR_OK);
    return err;
}

/*
 * Delete the pcb inside a netconn.
 * Called from netconn_delete.
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_delconn(m: &mut ()) {
    let msg: &mut api_msg = m;

    let state: netconn_state = msg.conn.state;
    LWIP_ASSERT(
        "netconn state error", /* this only happens for TCP netconns */
        (state == NETCONN_NONE) || (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_TCP),
    );

    /* In full duplex mode, blocking write/connect is aborted with ERR_CLSD */
    if (state != NETCONN_NONE) {
        if ((state == NETCONN_WRITE)
            || ((state == NETCONN_CONNECT) && !IN_NONBLOCKING_CONNECT(msg.conn)))
        {
            /* close requested, abort running write/connect */
            sys_sem_t * op_completed_sem;
            LWIP_ASSERT("msg.conn.current_msg != NULL", msg.conn.current_msg != NULL);
            op_completed_sem = LWIP_API_MSG_SEM(msg.conn.current_msg);
            msg.conn.current_msg.err = ERR_CLSD;
            msg.conn.current_msg = NULL;
            msg.conn.state = NETCONN_NONE;
            sys_sem_signal(op_completed_sem);
        }
    }
    /* LWIP_NETCONN_FULLDUPLEX */
    if (((state != NETCONN_NONE) && (state != NETCONN_LISTEN) && (state != NETCONN_CONNECT))
        || ((state == NETCONN_CONNECT) && !IN_NONBLOCKING_CONNECT(msg.conn)))
    {
        /* This means either a blocking write or blocking connect is running
        (nonblocking write returns and sets state to NONE) */
        msg.err = ERR_INPROGRESS;
    } else {
        LWIP_ASSERT(
            "blocking connect in progress",
            (state != NETCONN_CONNECT) || IN_NONBLOCKING_CONNECT(msg.conn),
        );
        msg.err = ERR_OK;

        /* Mark mboxes invalid */
        netconn_mark_mbox_invalid(msg.conn);
        /* LWIP_NETCONN_FULLDUPLEX */
        netconn_drain(msg.conn);

        if (msg.conn.pcb.tcp != NULL) {
            match (NETCONNTYPE_GROUP(msg.conn.netconntype)) {
                NETCONN_RAW => {
                    raw_remove(msg.conn.pcb.raw);
                }

                NETCONN_UDP => {
                    msg.conn.pcb.udp.recv_arg = NULL;
                    udp_remove(msg.conn.pcb.udp);
                }

                NETCONN_TCP => {
                    LWIP_ASSERT("already writing or closing", msg.conn.current_msg == NULL);
                    msg.conn.state = NETCONN_CLOSE;
                    msg.msg.sd.shut = NETCONN_SHUT_RDWR;
                    msg.conn.current_msg = msg;

                    if (lwip_netconn_do_close_internal(msg.conn, 0) != ERR_OK) {
                        LWIP_ASSERT("state!", msg.conn.state == NETCONN_CLOSE);
                        UNLOCK_TCPIP_CORE();
                        sys_arch_sem_wait(LWIP_API_MSG_SEM(msg), 0);
                        LOCK_TCPIP_CORE();
                        LWIP_ASSERT("state!", msg.conn.state == NETCONN_NONE);
                    }
                    /* LWIP_TCPIP_CORE_LOCKING */
                    lwip_netconn_do_close_internal(msg.conn);

                    /* API_EVENT is called inside lwip_netconn_do_close_internal, before releasing
                    the application thread, so we can return at this point! */
                    return;
                }

                _ => {}
            }
            msg.conn.pcb.tcp = NULL;
        }
        /* tcp netconns don't come here! */

        /* @todo: this lets select make the socket readable and writable,
        which is wrong! errfd instead? */
        API_EVENT(msg.conn, NETCONN_EVT_RCVPLUS, 0);
        API_EVENT(msg.conn, NETCONN_EVT_SENDPLUS, 0);
    }
    if (sys_sem_valid(LWIP_API_MSG_SEM(msg))) {
        TCPIP_APIMSG_ACK(msg);
    }
}

/*
 * Bind a pcb contained in a netconn
 * Called from netconn_bind.
 *
 * @param m the api_msg pointing to the connection and containing
 *          the IP address and port to bind to
 */
pub fn lwip_netconn_do_bind(m: &mut ()) {
    let msg: &mut api_msg = m;
    let err: err_t;

    if (msg.conn.pcb.tcp != NULL) {
        match (NETCONNTYPE_GROUP(msg.conn.netconntype)) {
            NETCONN_RAW => {
                err = raw_bind(msg.conn.pcb.raw, API_EXPR_REF(msg.msg.bc.ipaddr));
            }
            NETCONN_UDP => {
                err = udp_bind(
                    msg.conn.pcb.udp,
                    API_EXPR_REF(msg.msg.bc.ipaddr),
                    msg.msg.bc.port,
                );
            }
            NETCONN_TCP => {
                err = tcp_bind(
                    msg.conn.pcb.tcp,
                    API_EXPR_REF(msg.msg.bc.ipaddr),
                    msg.msg.bc.port,
                );
            }
            _ => {
                err = ERR_VAL;
            }
        }
    } else {
        err = ERR_VAL;
    }
    msg.err = err;
    TCPIP_APIMSG_ACK(msg);
}
/*
 * Bind a pcb contained in a netconn to an interface
 * Called from netconn_bind_if.
 *
 * @param m the api_msg pointing to the connection and containing
 *          the IP address and port to bind to
 */
pub fn lwip_netconn_do_bind_if(m: &mut ()) {
    let netif: &mut NetIfc;
    let msg: &mut api_msg = m;
    let err: err_t;

    netif = netif_get_by_index(msg.msg.bc.if_idx);

    if ((netif != NULL) && (msg.conn.pcb.tcp != NULL)) {
        err = ERR_OK;
        match (NETCONNTYPE_GROUP(msg.conn.netconntype)) {
            NETCONN_RAW => raw_bind_netif(msg.conn.pcb.raw, netif),
            NETCONN_UDP => udp_bind_netif(msg.conn.pcb.udp, netif),
            NETCONN_TCP => tcp_bind_netif(msg.conn.pcb.tcp, netif),
            _ => err = ERR_VAL,
        }
    } else {
        err = ERR_VAL;
    }
    msg.err = err;
    TCPIP_APIMSG_ACK(msg);
}

/*
 * TCP callback function if a connection (opened by tcp_connect/lwip_netconn_do_connect) has
 * been established (or reset by the remote host).
 *
 * @see tcp.h (struct tcp_pcb.connected) for parameters and return values
 */
pub fn lwip_netconn_do_connected(
    arg: &mut Vec<u8>,
    pcb: &mut tcp_pcb,
    err: err_t,
) -> Result<(), LwipError> {
    let conn: &mut NetConnDesc;
    let was_blocking: i32;
    sys_sem_t * op_completed_sem = NULL;
    conn = arg;

    if (conn == NULL) {
        return ERR_VAL;
    }

    LWIP_ASSERT(
        "conn.state == NETCONN_CONNECT",
        conn.state == NETCONN_CONNECT,
    );
    LWIP_ASSERT(
        "(conn.current_msg != NULL) || conn.in_non_blocking_connect",
        (conn.current_msg != NULL) || IN_NONBLOCKING_CONNECT(conn),
    );

    if (conn.current_msg != NULL) {
        conn.current_msg.err = err;
        op_completed_sem = LWIP_API_MSG_SEM(conn.current_msg);
    }
    if ((NETCONNTYPE_GROUP(conn.netconntype) == NETCONN_TCP) && (err == ERR_OK)) {
        setup_tcp(conn);
    }
    was_blocking = !IN_NONBLOCKING_CONNECT(conn);
    SET_NONBLOCKING_CONNECT(conn, 0);
    LWIP_ASSERT(
        "blocking connect state error",
        (was_blocking && op_completed_sem != NULL) || (!was_blocking && op_completed_sem == NULL),
    );
    conn.current_msg = NULL;
    conn.state = NETCONN_NONE;
    API_EVENT(conn, NETCONN_EVT_SENDPLUS, 0);

    if (was_blocking) {
        sys_sem_signal(op_completed_sem);
    }
   return Ok(());
}

/*
 * Connect a pcb contained inside a netconn
 * Called from netconn_connect.
 *
 * @param m the api_msg pointing to the connection and containing
 *          the IP address and port to connect to
 */
pub fn lwip_netconn_do_connect(m: &mut ()) {
    let msg: &mut api_msg = m;
    let err: err_t;

    if (msg.conn.pcb.tcp == NULL) {
        /* This may happen when calling netconn_connect() a second time */
        err = ERR_CLSD;
    } else {
        match (NETCONNTYPE_GROUP(msg.conn.netconntype)) {
            NETCONN_RAW => err = raw_connect(msg.conn.pcb.raw, API_EXPR_REF(msg.msg.bc.ipaddr)),
            NETCONN_UDP => {
                err = udp_connect(
                    msg.conn.pcb.udp,
                    API_EXPR_REF(msg.msg.bc.ipaddr),
                    msg.msg.bc.port,
                )
            }
            NETCONN_TCP => {
                /* Prevent connect while doing any other action. */
                if (msg.conn.state == NETCONN_CONNECT) {
                    err = ERR_ALREADY;
                } else if (msg.conn.state != NETCONN_NONE) {
                    err = ERR_ISCONN;
                } else {
                    setup_tcp(msg.conn);
                    err = tcp_connect(
                        msg.conn.pcb.tcp,
                        API_EXPR_REF(msg.msg.bc.ipaddr),
                        msg.msg.bc.port,
                        lwip_netconn_do_connected,
                    );
                    if (err == ERR_OK) {
                        let non_blocking = netconn_is_nonblocking(msg.conn);
                        msg.conn.state = NETCONN_CONNECT;
                        SET_NONBLOCKING_CONNECT(msg.conn, non_blocking);
                        if (non_blocking) {
                            err = ERR_INPROGRESS;
                        } else {
                            msg.conn.current_msg = msg;
                            /* sys_sem_signal() is called from lwip_netconn_do_connected (or err_tcp()),
                            when the connection is established! */

                            LWIP_ASSERT("state!", msg.conn.state == NETCONN_CONNECT);
                            UNLOCK_TCPIP_CORE();
                            sys_arch_sem_wait(LWIP_API_MSG_SEM(msg), 0);
                            LOCK_TCPIP_CORE();
                            LWIP_ASSERT("state!", msg.conn.state != NETCONN_CONNECT);

                            return;
                        }
                    }
                }
            }

            _ => {
                // LWIP_ERROR("Invalid netconn type", 0, loop {
                //   err = ERR_VAL;
                // } while (0));
            }
        }
    }
    msg.err = err;
    /* For all other protocols, netconn_connect() calls netconn_apimsg(),
    so use TCPIP_APIMSG_ACK() here. */
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Disconnect a pcb contained inside a netconn
 * Only used for UDP netconns.
 * Called from netconn_disconnect.
 *
 * @param m the api_msg pointing to the connection to disconnect
 */
pub fn lwip_netconn_do_disconnect(m: &mut ()) {
    let msg: &mut api_msg = m;
    if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_UDP) {
        udp_disconnect(msg.conn.pcb.udp);
        msg.err = ERR_OK;
    } else {
        msg.err = ERR_VAL;
    }
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Set a TCP pcb contained in a netconn into listen mode
 * Called from netconn_listen.
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_listen(m: &mut ()) {
    let msg: &mut api_msg = m;
    let err: err_t;

    if (msg.conn.pcb.tcp != NULL) {
        if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_TCP) {
            if (msg.conn.state == NETCONN_NONE) {
                let lpcb: &mut tcp_pcb;
                if (msg.conn.pcb.tcp.state != CLOSED) {
                    /* connection is not closed, cannot listen */
                    err = ERR_VAL;
                } else {
                    let backlog: u8;

                    backlog = msg.msg.lb.backlog;
                    /* TCP_LISTEN_BACKLOG */
                    backlog = TCP_DEFAULT_LISTEN_BACKLOG;

                    /* "Socket API like" dual-stack support: If IP to listen to is IP6_ADDR_ANY,
                     * and NETCONN_FLAG_IPV6_V6ONLY is NOT set, use IP_ANY_TYPE to listen
                     */
                    if (ip_addr_cmp(&msg.conn.pcb.ip.local_ip, IP6_ADDR_ANY)
                        && (netconn_get_ipv6only(msg.conn) == 0))
                    {
                        /* change PCB type to IpaddrTypeAny */
                        IP_SET_TYPE_VAL(msg.conn.pcb.tcp.local_ip, IPADDR_TYPE_ANY);
                        IP_SET_TYPE_VAL(msg.conn.pcb.tcp.remote_ip, IPADDR_TYPE_ANY);
                    }

                    lpcb = tcp_listen_with_backlog_and_err(msg.conn.pcb.tcp, backlog, &err);

                    if (lpcb == NULL) {
                        /* in this case, the old pcb is still allocated */
                    } else {
                        /* delete the recvmbox and allocate the acceptmbox */
                        if (sys_mbox_valid(&msg.conn.recvmbox)) {
                            /* @todo: should we drain the recvmbox here? */
                            sys_mbox_free(&msg.conn.recvmbox);
                            sys_mbox_set_invalid(&msg.conn.recvmbox);
                        }
                        err = ERR_OK;
                        if (!sys_mbox_valid(&msg.conn.acceptmbox)) {
                            err = sys_mbox_new(&msg.conn.acceptmbox, DEFAULT_ACCEPTMBOX_SIZE);
                        }
                        if (err == ERR_OK) {
                            msg.conn.state = NETCONN_LISTEN;
                            msg.conn.pcb.tcp = lpcb;
                            tcp_arg(msg.conn.pcb.tcp, msg.conn);
                            tcp_accept(msg.conn.pcb.tcp, accept_function);
                        } else {
                            /* since the old pcb is already deallocated, free lpcb now */
                            tcp_close(lpcb);
                            msg.conn.pcb.tcp = NULL;
                        }
                    }
                }
            } else if (msg.conn.state == NETCONN_LISTEN) {
                /* already listening, allow updating of the backlog */
                err = ERR_OK;
                tcp_backlog_set(msg.conn.pcb.tcp, msg.msg.lb.backlog);
            } else {
                err = ERR_CONN;
            }
        } else {
            err = ERR_ARG;
        }
    } else {
        err = ERR_CONN;
    }
    msg.err = err;
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Send some data on a RAW or UDP pcb contained in a netconn
 * Called from netconn_send
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_send(m: &mut ()) {
    let msg: &mut api_msg = m;

    let err: err_t = netconn_err(msg.conn);
    if (err == ERR_OK) {
        if (msg.conn.pcb.tcp != NULL) {
            match (NETCONNTYPE_GROUP(msg.conn.netconntype)) {
                NETCONN_RAW => {
                    if (ip_addr_isany(&msg.msg.b.addr) || IP_IS_ANY_TYPE_VAL(msg.msg.b.addr)) {
                        err = raw_send(msg.conn.pcb.raw, msg.msg.b.p);
                    } else {
                        err = raw_sendto(msg.conn.pcb.raw, msg.msg.b.p, &msg.msg.b.addr);
                    }
                }

                NETCONN_UDP => {
                    if (ip_addr_isany(&msg.msg.b.addr) || IP_IS_ANY_TYPE_VAL(msg.msg.b.addr)) {
                        err = udp_send_chksum(
                            msg.conn.pcb.udp,
                            msg.msg.b.p,
                            msg.msg.b.flags & NETBUF_FLAG_CHKSUM,
                            msg.msg.b.toport_chksum,
                        );
                    } else {
                        err = udp_sendto_chksum(
                            msg.conn.pcb.udp,
                            msg.msg.b.p,
                            &msg.msg.b.addr,
                            msg.msg.b.port,
                            msg.msg.b.flags & NETBUF_FLAG_CHKSUM,
                            msg.msg.b.toport_chksum,
                        );
                    }
                    /* LWIP_CHECKSUM_ON_COPY */
                    if (ip_addr_isany_val(msg.msg.b.addr) || IP_IS_ANY_TYPE_VAL(msg.msg.b.addr)) {
                        err = udp_send(msg.conn.pcb.udp, msg.msg.b.p);
                    } else {
                        err = udp_sendto(
                            msg.conn.pcb.udp,
                            msg.msg.b.p,
                            &msg.msg.b.addr,
                            msg.msg.b.port,
                        );
                    }
                }

                _ => {
                    err = ERR_CONN;
                }
            }
        } else {
            err = ERR_CONN;
        }
    }
    msg.err = err;
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Indicate data has been received from a TCP pcb contained in a netconn
 * Called from netconn_recv
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_recv(m: &mut ()) {
    let msg: &mut api_msg = m;

    msg.err = ERR_OK;
    if (msg.conn.pcb.tcp != NULL) {
        if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_TCP) {
            let remaining: usize = msg.msg.r.len;
            loop {
                if remaining > 0xffff {
                    recved = 0xffff;
                } else {
                    recved = remaining;
                }

                tcp_recved(msg.conn.pcb.tcp, recved);
                remaining -= recved;
                if remaining == 0 {
                    break;
                }
            }
        }
    }
    TCPIP_APIMSG_ACK(msg);
}

/* Indicate that a TCP pcb has been accepted
 * Called from netconn_accept
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_accepted(m: &mut ()) {
    let msg: &mut api_msg = m;

    msg.err = ERR_OK;
    if (msg.conn.pcb.tcp != NULL) {
        if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_TCP) {
            tcp_backlog_accepted(msg.conn.pcb.tcp);
        }
    }
    TCPIP_APIMSG_ACK(msg);
}

/*
 * See if more data needs to be written from a previous call to netconn_write.
 * Called initially from lwip_netconn_do_write. If the first call can't send all data
 * (because of low memory or empty send-buffer), this function is called again
 * from sent_tcp() or poll_tcp() to send more data. If all data is sent, the
 * blocking application thread (waiting in netconn_write) is released.
 *
 * @param conn netconn (that is currently in state NETCONN_WRITE) to process
 * @return ERR_OK
 *         ERR_MEM if LWIP_TCPIP_CORE_LOCKING=1 and sending hasn't yet finished
 */
pub fn lwip_netconn_do_writemore(conn: &mut NetConnDesc) -> Result<(), LwipError> {
    let err: err_t;
    let dataptr: &Vec<u8>;
    let len: usize;
    let available: u16;
    let write_finished: u8 = 0;
    let diff: usize;
    let dontblock: u8;
    let apiflags: u8;
    let write_more: u8;

    LWIP_ASSERT("conn != NULL", conn != NULL);
    LWIP_ASSERT("conn.state == NETCONN_WRITE", (conn.state == NETCONN_WRITE));
    LWIP_ASSERT("conn.current_msg != NULL", conn.current_msg != NULL);
    LWIP_ASSERT("conn.pcb.tcp != NULL", conn.pcb.tcp != NULL);
    LWIP_ASSERT(
        "conn.current_msg.msg.w.offset < conn.current_msg.msg.w.len",
        conn.current_msg.msg.w.offset < conn.current_msg.msg.w.len,
    );
    LWIP_ASSERT(
        "conn.current_msg.msg.w.vector_cnt > 0",
        conn.current_msg.msg.w.vector_cnt > 0,
    );

    apiflags = conn.current_msg.msg.w.apiflags;
    dontblock = netconn_is_nonblocking(conn) || (apiflags & NETCONN_DONTBLOCK);

    if ((conn.send_timeout != 0)
        && ((sys_now() - conn.current_msg.msg.w.time_started) >= conn.send_timeout))
    {
        write_finished = 1;
        if (conn.current_msg.msg.w.offset == 0) {
            /* nothing has been written */
            err = ERR_WOULDBLOCK;
        } else {
            /* partial write */
            err = ERR_OK;
        }
    } else {
        loop {
            dataptr = conn.current_msg.msg.w.vector.ptr + conn.current_msg.msg.w.vector_off;
            diff = conn.current_msg.msg.w.vector.len - conn.current_msg.msg.w.vector_off;
            if (diff > 0xffff) {
                /* max_u16 */
                len = 0xffff;
                apiflags |= TCP_WRITE_FLAG_MORE;
            } else {
                len = diff;
            }
            available = tcp_sndbuf(conn.pcb.tcp);
            if (available < len) {
                /* don't try to write more than sendbuf */
                len = available;
                if (dontblock) {
                    if (!len) {
                        /* set error according to partial write or not */
                        // err = (conn.current_msg.msg.w.offset == 0) ? ERR_WOULDBLOCK : ERR_OK;
                        if conn.current_msg.msg.w.offset == 0 {
                            err = ERR_WOULDBLOCK;
                        } else {
                            err = ERR_OK;
                        }
                        // goto err_mem;
                    }
                } else {
                    apiflags |= TCP_WRITE_FLAG_MORE;
                }
            }
            LWIP_ASSERT(
                "lwip_netconn_do_writemore: invalid length!",
                ((conn.current_msg.msg.w.vector_off + len) <= conn.current_msg.msg.w.vector.len),
            );
            /* we should loop around for more sending in the following cases:
            1) We couldn't finish the current vector because of 16-bit size limitations.
               tcp_write() and tcp_sndbuf() both are limited to 16-bit sizes
            2) We are sending the remainder of the current vector and have more */
            if ((len == 0xffff && diff > 0xffff)
                || (len == diff && conn.current_msg.msg.w.vector_cnt > 1))
            {
                write_more = 1;
                apiflags |= TCP_WRITE_FLAG_MORE;
            } else {
                write_more = 0;
            }
            err = tcp_write(conn.pcb.tcp, dataptr, len, apiflags);
            if (err == ERR_OK) {
                conn.current_msg.msg.w.offset += len;
                conn.current_msg.msg.w.vector_off += len;
                /* check if current vector is finished */
                if (conn.current_msg.msg.w.vector_off == conn.current_msg.msg.w.vector.len) {
                    conn.current_msg.msg.w.vector_cnt -= 1;
                    /* if we have additional vectors, move on to them */
                    if (conn.current_msg.msg.w.vector_cnt > 0) {
                        conn.current_msg.msg.w.vector += 1;
                        conn.current_msg.msg.w.vector_off = 0;
                    }
                }
            }
            if write_more == false || err != ERR_OK {
                break;
            }
        }
        /* if OK or memory error, check available space */
        if ((err == ERR_OK) || (err == ERR_MEM)) {
            // err_mem:
            if (dontblock && (conn.current_msg.msg.w.offset < conn.current_msg.msg.w.len)) {
                /* non-blocking write did not write everything: mark the pcb non-writable
                and let poll_tcp check writable space to mark the pcb writable again */
                API_EVENT(conn, NETCONN_EVT_SENDMINUS, 0);
                conn.flags |= NETCONN_FLAG_CHECK_WRITESPACE;
            } else if ((tcp_sndbuf(conn.pcb.tcp) <= TCP_SNDLOWAT)
                || (tcp_sndqueuelen(conn.pcb.tcp) >= TCP_SNDQUEUELOWAT))
            {
                /* The queued byte- or pbuf-count exceeds the configured low-water limit,
                let select mark this pcb as non-writable. */
                API_EVENT(conn, NETCONN_EVT_SENDMINUS, 0);
            }
        }

        if (err == ERR_OK) {
            let out_err: err_t;
            if ((conn.current_msg.msg.w.offset == conn.current_msg.msg.w.len) || dontblock) {
                /* return sent length (caller reads length from msg.w.offset) */
                write_finished = 1;
            }
            out_err = tcp_output(conn.pcb.tcp);
            if (out_err == ERR_RTE) {
                /* If tcp_output fails because no route is found,
                don't try writing any more but return the error
                to the application thread. */
                err = out_err;
                write_finished = 1;
            }
        } else if (err == ERR_MEM) {
            /* If ERR_MEM, we wait for sent_tcp or poll_tcp to be called.
            For blocking sockets, we do NOT return to the application
            thread, since ERR_MEM is only a temporary error! Non-blocking
            will remain non-writable until sent_tcp/poll_tcp is called */

            /* tcp_write returned ERR_MEM, try tcp_output anyway */
            let out_err: err_t = tcp_output(conn.pcb.tcp);
            if (out_err == ERR_RTE) {
                /* If tcp_output fails because no route is found,
                don't try writing any more but return the error
                to the application thread. */
                err = out_err;
                write_finished = 1;
            } else if (dontblock) {
                /* non-blocking write is done on ERR_MEM, set error according
                to partial write or not */
                // err = (conn.current_msg.msg.w.offset == 0) ? ERR_WOULDBLOCK : ERR_OK;
                if conn.current_msg.msg.w.offset == 0 {}
                write_finished = 1;
            }
        } else {
            /* On errors != ERR_MEM, we don't try writing any more but return
            the error to the application thread. */
            write_finished = 1;
        }
    }
    if (write_finished) {
        /* everything was written: set back connection state
        and back to application task */
        sys_sem_t * op_completed_sem = LWIP_API_MSG_SEM(conn.current_msg);
        conn.current_msg.err = err;
        conn.current_msg = NULL;
        conn.state = NETCONN_NONE;

        if (delayed) {
            sys_sem_signal(op_completed_sem);
        }
    } else {
        return ERR_MEM;
    }

   return Ok(());
}

/*
 * Send some data on a TCP pcb contained in a netconn
 * Called from netconn_write
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_write(m: &mut ()) {
    let msg: &mut api_msg = m;

    let err: err_t = netconn_err(msg.conn);
    if (err == ERR_OK) {
        if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_TCP) {
            if (msg.conn.state != NETCONN_NONE) {
                /* netconn is connecting, closing or in blocking write */
                err = ERR_INPROGRESS;
            } else if (msg.conn.pcb.tcp != NULL) {
                msg.conn.state = NETCONN_WRITE;
                /* set all the variables used by lwip_netconn_do_writemore */
                LWIP_ASSERT("already writing or closing", msg.conn.current_msg == NULL);
                LWIP_ASSERT("msg.msg.w.len != 0", msg.msg.w.len != 0);
                msg.conn.current_msg = msg;

                if (lwip_netconn_do_writemore(msg.conn, 0) != ERR_OK) {
                    LWIP_ASSERT("state!", msg.conn.state == NETCONN_WRITE);
                    UNLOCK_TCPIP_CORE();
                    sys_arch_sem_wait(LWIP_API_MSG_SEM(msg), 0);
                    LOCK_TCPIP_CORE();
                    LWIP_ASSERT("state!", msg.conn.state != NETCONN_WRITE);
                }
                /* LWIP_TCPIP_CORE_LOCKING */
                lwip_netconn_do_writemore(msg.conn);

                /* for both cases: if lwip_netconn_do_writemore was called, don't ACK the APIMSG
                since lwip_netconn_do_writemore ACKs it! */
                return;
            } else {
                err = ERR_CONN;
            }
            /* LWIP_TCP */
            err = ERR_VAL;
        } else {
            err = ERR_VAL;
        }
    }
    msg.err = err;
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Return a connection's local or remote address
 * Called from netconn_getaddr
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_getaddr(m: &mut ()) {
    let msg: &mut api_msg = m;

    if (msg.conn.pcb.ip != NULL) {
        if (msg.msg.ad.local) {
            ip_addr_copy(API_EXPR_DEREF(msg.msg.ad.ipaddr), msg.conn.pcb.ip.local_ip);
        } else {
            ip_addr_copy(API_EXPR_DEREF(msg.msg.ad.ipaddr), msg.conn.pcb.ip.remote_ip);
        }

        msg.err = ERR_OK;
        match (NETCONNTYPE_GROUP(msg.conn.netconntype)) {
            NETCONN_RAW => {
                if (msg.msg.ad.local) {
                    API_EXPR_DEREF(msg.msg.ad.port) = msg.conn.pcb.raw.protocol;
                } else {
                    /* return an error as connecting is only a helper for upper layers */
                    msg.err = ERR_CONN;
                }
            }

            NETCONN_UDP => {
                if (msg.msg.ad.local) {
                    API_EXPR_DEREF(msg.msg.ad.port) = msg.conn.pcb.udp.local_port;
                } else {
                    if ((msg.conn.pcb.udp.flags & UDP_FLAGS_CONNECTED) == 0) {
                        msg.err = ERR_CONN;
                    } else {
                        API_EXPR_DEREF(msg.msg.ad.port) = msg.conn.pcb.udp.remote_port;
                    }
                }
            }

            NETCONN_TCP => {
                if ((msg.msg.ad.local == 0)
                    && ((msg.conn.pcb.tcp.state == CLOSED) || (msg.conn.pcb.tcp.state == LISTEN)))
                {
                    /* pcb is not connected and remote name is requested */
                    msg.err = ERR_CONN;
                } else {
                    // msg.msg.ad.port = (msg.msg.ad.local ? msg.conn.pcb.tcp.local_port : msg.conn.pcb.tcp.remote_port);
                    if msg.msg.ad.local {
                        msg.msg.ad.port = msg.conn.pcb.tcp.local_port;
                    } else {
                        msg.msg.ad.port = msg.conn.pcb.tcp.remote_port;
                    }
                }
            }

            _ => {
                LWIP_ASSERT("invalid netconn_type", 0);
            }
        }
    } else {
        msg.err = ERR_CONN;
    }
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Close or half-shutdown a TCP pcb contained in a netconn
 * Called from netconn_close
 * In contrast to closing sockets, the netconn is not deallocated.
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_close(m: &mut ()) {
    let msg: &mut api_msg = m;
    let state: netconn_state = msg.conn.state;
    /* First check if this is a TCP netconn and if it is in a correct state
    (LISTEN doesn't support half shutdown) */
    if ((msg.conn.pcb.tcp != NULL)
        && (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_TCP)
        && ((msg.msg.sd.shut == NETCONN_SHUT_RDWR) || (state != NETCONN_LISTEN)))
    {
        /* Check if we are in a connected state */
        if (state == NETCONN_CONNECT) {
            /* TCP connect in progress: cannot shutdown */
            msg.err = ERR_CONN;
        } else if (state == NETCONN_WRITE) {
            if (msg.msg.sd.shut & NETCONN_SHUT_WR) {
                /* close requested, abort running write */
                sys_sem_t * write_completed_sem;
                LWIP_ASSERT("msg.conn.current_msg != NULL", msg.conn.current_msg != NULL);
                write_completed_sem = LWIP_API_MSG_SEM(msg.conn.current_msg);
                msg.conn.current_msg.err = ERR_CLSD;
                msg.conn.current_msg = NULL;
                msg.conn.state = NETCONN_NONE;
                state = NETCONN_NONE;
                sys_sem_signal(write_completed_sem);
            } else {
                LWIP_ASSERT(
                    "msg.msg.sd.shut == NETCONN_SHUT_RD",
                    msg.msg.sd.shut == NETCONN_SHUT_RD,
                );
                /* In this case, let the write continue and do not interfere with
                conn.current_msg or conn.state! */
                msg.err = tcp_shutdown(msg.conn.pcb.tcp, 1, 0);
            }
        }
        if (state == NETCONN_NONE) {
            /* LWIP_NETCONN_FULLDUPLEX */
            msg.err = ERR_INPROGRESS;
        } else {
            if (msg.msg.sd.shut & NETCONN_SHUT_RD) {
                /* Mark mboxes invalid */
                netconn_mark_mbox_invalid(msg.conn);
                /* LWIP_NETCONN_FULLDUPLEX */
                netconn_drain(msg.conn);
            }
            LWIP_ASSERT("already writing or closing", msg.conn.current_msg == NULL);
            msg.conn.state = NETCONN_CLOSE;
            msg.conn.current_msg = msg;

            if (lwip_netconn_do_close_internal(msg.conn, 0) != ERR_OK) {
                LWIP_ASSERT("state!", msg.conn.state == NETCONN_CLOSE);
                UNLOCK_TCPIP_CORE();
                sys_arch_sem_wait(LWIP_API_MSG_SEM(msg), 0);
                LOCK_TCPIP_CORE();
                LWIP_ASSERT("state!", msg.conn.state == NETCONN_NONE);
            }
            /* LWIP_TCPIP_CORE_LOCKING */
            lwip_netconn_do_close_internal(msg.conn);

            /* for tcp netconns, lwip_netconn_do_close_internal ACKs the message */
            return;
        }
    } else {
        msg.err = ERR_CONN;
    }
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Join multicast groups for UDP netconns.
 * Called from netconn_join_leave_group
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_join_leave_group(m: &mut ()) {
    let msg: &mut api_msg = m;

    msg.err = ERR_CONN;
    if (msg.conn.pcb.tcp != NULL) {
        if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_UDP) {
            if (NETCONNTYPE_ISIPV6(msg.conn.netconntype)) {
                if (msg.msg.jl.join_or_leave == NETCONN_JOIN) {
                    msg.err = mld6_joingroup(
                        ip_2_ip6(API_EXPR_REF(msg.msg.jl.netif_addr)),
                        ip_2_ip6(API_EXPR_REF(msg.msg.jl.multiaddr)),
                    );
                } else {
                    msg.err = mld6_leavegroup(
                        ip_2_ip6(API_EXPR_REF(msg.msg.jl.netif_addr)),
                        ip_2_ip6(API_EXPR_REF(msg.msg.jl.multiaddr)),
                    );
                }
            } else {
                if (msg.msg.jl.join_or_leave == NETCONN_JOIN) {
                    msg.err = igmp_joingroup(
                        ip_2_ip4(API_EXPR_REF(msg.msg.jl.netif_addr)),
                        ip_2_ip4(API_EXPR_REF(msg.msg.jl.multiaddr)),
                    );
                } else {
                    msg.err = igmp_leavegroup(
                        ip_2_ip4(API_EXPR_REF(msg.msg.jl.netif_addr)),
                        ip_2_ip4(API_EXPR_REF(msg.msg.jl.multiaddr)),
                    );
                }
            }
        } else {
            msg.err = ERR_VAL;
        }
    }
    TCPIP_APIMSG_ACK(msg);
}
/*
 * Join multicast groups for UDP netconns.
 * Called from netconn_join_leave_group_netif
 *
 * @param m the api_msg pointing to the connection
 */
pub fn lwip_netconn_do_join_leave_group_netif(m: &mut ()) {
    let msg: &mut api_msg = m;
    let netif: &mut NetIfc;

    netif = netif_get_by_index(msg.msg.jl.if_idx);
    if (netif == NULL) {
        msg.err = ERR_IF;
        // goto done;
    }

    msg.err = ERR_CONN;
    if (msg.conn.pcb.tcp != NULL) {
        if (NETCONNTYPE_GROUP(msg.conn.netconntype) == NETCONN_UDP) {
            if (NETCONNTYPE_ISIPV6(msg.conn.netconntype)) {
                if (msg.msg.jl.join_or_leave == NETCONN_JOIN) {
                    msg.err =
                        mld6_joingroup_netif(netif, ip_2_ip6(API_EXPR_REF(msg.msg.jl.multiaddr)));
                } else {
                    msg.err =
                        mld6_leavegroup_netif(netif, ip_2_ip6(API_EXPR_REF(msg.msg.jl.multiaddr)));
                }
            } else {
                if (msg.msg.jl.join_or_leave == NETCONN_JOIN) {
                    msg.err =
                        igmp_joingroup_netif(netif, ip_2_ip4(API_EXPR_REF(msg.msg.jl.multiaddr)));
                } else {
                    msg.err =
                        igmp_leavegroup_netif(netif, ip_2_ip4(API_EXPR_REF(msg.msg.jl.multiaddr)));
                }
            }
        } else {
            msg.err = ERR_VAL;
        }
    }

    // done:
    TCPIP_APIMSG_ACK(msg);
}

/*
 * Callback function that is called when DNS name is resolved
 * (or on timeout). A waiting application thread is waked up by
 * signaling the semaphore.
 */
pub fn lwip_netconn_do_dns_found(name: &String, ipaddr: &mut LwipAddr, arg: &mut Vec<u8>) {
    let msg: &mut dns_api_msg = arg;

    /* we trust the internal implementation to be correct :-) */

    if (ipaddr == NULL) {
        /* timeout or memory error */
        API_EXPR_DEREF(msg.err) = ERR_VAL;
    } else {
        /* address was resolved */
        API_EXPR_DEREF(msg.err) = ERR_OK;
        API_EXPR_DEREF(msg.addr) = *ipaddr;
    }
    /* wake up the application task waiting in netconn_gethostbyname */
    sys_sem_signal(API_EXPR_REF_SEM(msg.sem));
}

/*
 * Execute a DNS query
 * Called from netconn_gethostbyname
 *
 * @param arg the dns_api_msg pointing to the query
 */
pub fn lwip_netconn_do_gethostbyname(arg: &mut Vec<u8>) {
    let msg: &mut dns_api_msg = arg;
    let addrtype: u8 = msg.dns_addrtype;

    // LWIP_DNS_ADDRTYPE_DEFAULT;

    msg.err = dns_gethostbyname_addrtype(
        msg.name,
        API_EXPR_REF(msg.addr),
        lwip_netconn_do_dns_found,
        msg,
        addrtype,
    );

    /* For core locking, only block if we need to wait for answer/timeout */
    if (API_EXPR_DEREF(msg.err) == ERR_INPROGRESS) {
        UNLOCK_TCPIP_CORE();
        sys_sem_wait(API_EXPR_REF_SEM(msg.sem));
        LOCK_TCPIP_CORE();
        LWIP_ASSERT(
            "do_gethostbyname still in progress!!",
            API_EXPR_DEREF(msg.err) != ERR_INPROGRESS,
        );
    }
    /* LWIP_TCPIP_CORE_LOCKING */
    if (API_EXPR_DEREF(msg.err) != ERR_INPROGRESS) {
        /* on error or immediate success, wake up the application
         * task waiting in netconn_gethostbyname */
        sys_sem_signal(API_EXPR_REF_SEM(msg.sem));
    }
}
