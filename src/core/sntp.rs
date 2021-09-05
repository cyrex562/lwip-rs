/*
 * Copyright (c) 2007-2009 Frédéric Bernon, Simon Goldschmidt
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
 * Author: Frédéric Bernon, Simon Goldschmidt
 */

/*
 * @defgroup sntp SNTP
 * @ingroup apps
 *
 * This is simple "SNTP" client for the lwIP raw API.
 * It is a minimal implementation of SNTPv4 as specified in RFC 4330.
 *
 * For a list of some public NTP servers, see this link:
 * http://support.ntp.org/bin/view/Servers/NTPPoolServers
 *
 * @todo:
 * - complete SNTP_CHECK_RESPONSE checks 3 and 4
 */

/* Handle support for more than one server via SNTP_MAX_SERVERS */

pub const SNTP_SUPPORT_MULTIPLE_SERVERS: u32 = 1;
/* NTP_MAX_SERVERS > 1 */
pub const SNTP_SUPPORT_MULTIPLE_SERVERS: u32 = 0;

// #error "SNTPv4 RFC 4330 enforces a minimum update time of 15 seconds (define SNTP_SUPPRESS_DELAY_CHECK to disable this error)!"

/* the various debug levels for this file */
// pub const SNTP_DEBUG_TRACE: bool        (SNTP_DEBUG | LWIP_DBG_TRACE)
// #define SNTP_DEBUG_STATE        (SNTP_DEBUG | LWIP_DBG_STATE)
// #define SNTP_DEBUG_WARN         (SNTP_DEBUG | LWIP_DBG_LEVEL_WARNING)
// #define SNTP_DEBUG_WARN_STATE   (SNTP_DEBUG | LWIP_DBG_LEVEL_WARNING | LWIP_DBG_STATE)
// #define SNTP_DEBUG_SERIOUS      (SNTP_DEBUG | LWIP_DBG_LEVEL_SERIOUS)

pub const SNTP_ERR_KOD: u32 = 1;

/* SNTP protocol defines */
pub const SNTP_MSG_LEN: u32 = 48;

pub const SNTP_OFFSET_LI_VN_MODE: u32 = 0;
pub const SNTP_LI_MASK: u32 = 0xC0;
pub const SNTP_LI_NO_WARNING: u32 = (0x00 << 6);
pub const SNTP_LI_LAST_MINUTE_61_SEC: u32 = (0x01 << 6);
pub const SNTP_LI_LAST_MINUTE_59_SEC: u32 = (0x02 << 6);
pub const SNTP_LI_ALARM_CONDITION: u32 = (0x03 << 6); /* (clock not synchronized) */

pub const SNTP_VERSION_MASK: u32 = 0x38;
pub const SNTP_VERSION: u32 = (4/* NTP Version 4*/<<3);

pub const SNTP_MODE_MASK: u32 = 0x07;
pub const SNTP_MODE_MASK: u32 = 0x07;
pub const SNTP_MODE_MASK: u32 = 0x07;
pub const SNTP_MODE_MASK: u32 = 0x07;
pub const SNTP_MODE_CLIENT: u32 = 0x03;
pub const SNTP_MODE_SERVER: u32 = 0x04;
pub const SNTP_MODE_BROADCAST: u32 = 0x05;

pub const SNTP_OFFSET_STRATUM: u32 = 1;
pub const SNTP_STRATUM_KOD: u32 = 0x00;

pub const SNTP_OFFSET_ORIGINATE_TIME: u32 = 24;
pub const SNTP_OFFSET_RECEIVE_TIME: u32 = 32;
pub const SNTP_OFFSET_TRANSMIT_TIME: u32 = 40;

/* Number of seconds between 1970 and Feb 7, 2036 06:28:16 UTC (epoch 1) */
pub const DIFF_SEC_1970_2036: u64 = (2085978496);

/* Convert NTP timestamp fraction to microseconds.
 */

// # if LWIP_HAVE_INT64
// #  define SNTP_FRAC_TO_US(f)        ((((f) * 1000000) >> 32))
// // # else
// #  define SNTP_FRAC_TO_US(f)        ((f) / 4295)
// # endif

/* Configure behaviour depending on native, microsecond or second precision.
 * Treat NTP timestamps as signed two's-complement integers. This way,
 * timestamps that have the MSB set simply become negative offsets from
 * the epoch (Feb 7, 2036 06:28:16 UTC). Representable dates range from
 * 1968 to 2104.
 */

// # ifdef SNTP_SET_SYSTEM_TIME_US
// #  define SNTP_SET_SYSTEM_TIME_NTP(s, f) \
//     SNTP_SET_SYSTEM_TIME_US(((s) + DIFF_SEC_1970_2036), SNTP_FRAC_TO_US(f))
// // # else
// #  define SNTP_SET_SYSTEM_TIME_NTP(s, f) \
//     SNTP_SET_SYSTEM_TIME(((s) + DIFF_SEC_1970_2036))
// # endif

/* Get the system time either natively as NTP timestamp or convert from
 * Unix time in seconds and microseconds. Take care to avoid overflow if the
 * microsecond value is at the maximum of 999999. Also add 0.5 us fudge to
 * avoid special values like 0, and to mask round-off errors that would
 * otherwise break round-trip conversion identity.
 */

// # define SNTP_GET_SYSTEM_TIME_NTP(s, f) loop { \
//     sec_: u32, usec_; \
//     SNTP_GET_SYSTEM_TIME(sec_, usec_); \
//     (s) = (sec_ - DIFF_SEC_1970_2036); \
//     (f) = usec_ * 4295 - ((usec_ * 2143) >> 16) + 2147; \
//   } while (0)

/* Start offset of the timestamps to extract from the SNTP packet */
pub const SNTP_OFFSET_TIMESTAMPS: usize = (SNTP_OFFSET_TRANSMIT_TIME + 8 - sizeof(sntp_timestamps));

/* Round-trip delay arithmetic helpers */

// # if !LWIP_HAVE_INT64
// #  error "SNTP round-trip delay compensation requires 64-bit arithmetic"
// # endif
// # define SNTP_SEC_FRAC_TO_S64(s, f) \
//     ((s64_t)(((s) << 32) | (f)))
// # define SNTP_TIMESTAMP_TO_S64(t) \
//     SNTP_SEC_FRAC_TO_S64(lwip_ntohl(t.sec), lwip_ntohl(t.frac))

/*
 * 64-bit NTP timestamp, in network byte order.
 */
pub struct sntp_time {
    pub sec: u32,
    pub frac: u32,
}

/*
 * Timestamps to be extracted from the NTP header.
 */
pub struct sntp_timestamps {
    pub orig: sntp_time,
    pub recv: sntp_time,

    pub xmit: sntp_time,
}

/*
 * SNTP packet format (without optional fields)
 * Timestamps are coded as 64 bits:
 * - signed 32 bits seconds since Feb 07, 2036, 06:28:16 UTC (epoch 1)
 * -  32 bits seconds fraction (2^32 = 1 second)
 */

pub struct sntp_msg {
    pub li_vn_mode: u8,
    pub stratum: u8,
    pub poll: u8,
    pub precision: u8,
    pub root_delay: u32,
    pub root_dispersion: u32,
    pub reference_identifier: u32,
    pub reference_timestamp: [u32; 2],
    pub originate_timestamp: [u32; 2],
    pub receive_timestamp: [u32; 2],
    pub transmit_timestamp: [u32; 2],
}

/* function prototypes */
fn sntp_request(arg: &mut Vec<u8>);

/* The operating mode */
static sntp_opmode: u8;

/* The UDP pcb used by the SNTP client */
static sntp_pcb: &mut udp_pcb;
/* Names/Addresses of servers */
pub struct sntp_server {
    pub name: String,

    pub addr: LwipAddr,

    /* Reachability shift register as described in RFC 5905 */
    pub reachability: u8,
}
// static struct sntp_server sntp_servers[SNTP_MAX_SERVERS];

// static sntp_set_servers_from_dhcp: u8;

/* The currently used server (initialized to 0) */
// static sntp_current_server: u8;
/* SNTP_SUPPORT_MULTIPLE_SERVERS */
// pub const sntp_current_server: u32 = 0;

// #define SNTP_RESET_RETRY_TIMEOUT() sntp_retry_timeout = SNTP_RETRY_TIMEOUT
/* Retry time, initialized with SNTP_RETRY_TIMEOUT and doubled with each retry. */
// static sntp_retry_timeout: u32;
/* SNTP_RETRY_TIMEOUT_EXP */
// #define SNTP_RESET_RETRY_TIMEOUT()
// #define sntp_retry_timeout SNTP_RETRY_TIMEOUT

/* Saves the last server address to compare with response */
// static LwipAddr sntp_last_server_address;

/* Saves the last timestamp sent (which is sent back by the server)
 * to compare against in response. Stored in network byte order. */
// static struct sntp_time sntp_last_timestamp_sent;

/* Debug prhelper: i32. */
// static const char *
// sntp_format_time(i32 sec)
// {
//   let ut: time_t;
//   ut = (sec + DIFF_SEC_1970_2036);
//   return ctime(&ut);
// }

/*
 * SNTP processing of received timestamp
 */
pub fn sntp_process(timestamps: &mut sntp_timestamps) {
    let letsec: i32;
    let frac: u32;

    sec = lwip_ntohl(timestamps.xmit.sec);
    frac = lwip_ntohl(timestamps.xmit.frac);

    if (timestamps.recv.sec != 0 || timestamps.recv.frac != 0) {
        let letdest_sec: i32;
        let dest_frac: u32;
        let step_sec: u32;

        /* Get the destination time stamp, i.e. the current system time */
        SNTP_GET_SYSTEM_TIME_NTP(dest_sec, dest_frac);

        // TODO:
        // step_sec = (dest_sec < sec) ? (sec - dest_sec)
        //            : (dest_sec - sec);
        /* In order to avoid overflows, skip the compensation if the clock step
         * is larger than about 34 years. */
        if ((step_sec >> 30) == 0) {
            // s64_t t1, t2, t3, t4;
            let t1: s64;
            let t2: s64;
            let t3: s64;
            let t4: s64;

            t4 = SNTP_SEC_FRAC_TO_S64(dest_sec, dest_frac);
            t3 = SNTP_SEC_FRAC_TO_S64(sec, frac);
            t1 = SNTP_TIMESTAMP_TO_S64(timestamps.orig);
            t2 = SNTP_TIMESTAMP_TO_S64(timestamps.recv);
            /* Clock offset calculation according to RFC 4330 */
            t4 += ((t2 - t1) + (t3 - t4)) / 2;

            sec = (t4 >> 32);
            frac = (t4);
        }
    }

    SNTP_SET_SYSTEM_TIME_NTP(sec, frac);
    /* might be unused if only seconds are set */
    /*LWIP_DEBUGF(SNTP_DEBUG_TRACE, ("sntp_process: %s, %" U32_F " us\n",
    sntp_format_time(sec), SNTP_FRAC_TO_US(frac)));*/
}

/*
 * Initialize request struct to be sent to server.
 */
pub fn sntp_initialize_request(req: &mut sntp_msg) {
    //memset(req, 0, SNTP_MSG_LEN);
    req.li_vn_mode = SNTP_LI_NO_WARNING | SNTP_VERSION | SNTP_MODE_CLIENT;

    {
        let letsecs: i32;
        let sec: u32;
        let frac: u32;
        /* Get the transmit timestamp */
        SNTP_GET_SYSTEM_TIME_NTP(secs, frac);
        sec = lwip_htonl(secs);
        frac = lwip_htonl(frac);

        sntp_last_timestamp_sent.sec = sec;
        sntp_last_timestamp_sent.frac = frac;
        req.transmit_timestamp[0] = sec;
        req.transmit_timestamp[1] = frac;
    }
}

/*
 * Retry: send a new request (and increase retry timeout).
 *
 * @param arg is unused (only necessary to conform to sys_timeout)
 */
pub fn sntp_retry(arg: &mut Vec<u8>) {
    /*LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_retry: Next request will be sent in %"U32_F" ms\n",
    sntp_retry_timeout));*/

    /* set up a timer to send a retry and increase the retry delay */
    sys_timeout(sntp_retry_timeout, sntp_request, None);

    {
        let new_retry_timeout: u32;
        /* increase the timeout for next retry */
        new_retry_timeout = sntp_retry_timeout << 1;
        /* limit to maximum timeout and prevent overflow */
        if ((new_retry_timeout <= SNTP_RETRY_TIMEOUT_MAX)
            && (new_retry_timeout > sntp_retry_timeout))
        {
            sntp_retry_timeout = new_retry_timeout;
        }
    }
}

/*
 * If Kiss-of-Death is received (or another packet parsing error),
 * try the next server or retry the current server and increase the retry
 * timeout if only one server is available.
 * (implicitly, SNTP_MAX_SERVERS > 1)
 *
 * @param arg is unused (only necessary to conform to sys_timeout)
 */
pub fn sntp_try_next_server(arg: &mut Vec<u8>) {
    let old_server: u8;
    let i;

    old_server = sntp_current_server;
    //   for (i = 0; i < SNTP_MAX_SERVERS - 1; i+= 1) {
    //     sntp_current_server+= 1;
    //     if (sntp_current_server >= SNTP_MAX_SERVERS) {
    //       sntp_current_server = 0;
    //     }
    //     if (!ip_addr_isany(&sntp_servers[sntp_current_server].addr)

    //         || (sntp_servers[sntp_current_server].name != None)

    //        ) {
    // /*LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_try_next_server: Sending request to server %"U16_F"\n",
    //                                      sntp_current_server));*/
    //       /* new server: reset retry timeout */
    //       SNTP_RESET_RETRY_TIMEOUT();
    //       /* instantly send a request to the next server */
    //       sntp_request(None);
    //       return;
    //     }
    //   }
    /* no other valid server found */
    sntp_current_server = old_server;
    sntp_retry(None);
}
/* SNTP_SUPPORT_MULTIPLE_SERVERS */
/* Always retry on error if only one server is supported */
// #define sntp_try_next_server    sntp_retry

/* UDP recv callback for the sntp pcb */
pub fn sntp_recv(
    arg: &mut Vec<u8>,
    pcb: &mut udp_pcb,
    p: &mut pbuf,
    addr: &mut LwipAddr,
    port: u16,
) {
    let timestamps: sntp_timestamps;
    let mode: u8;
    let stratum: u8;
    let err: err_t;

    err = ERR_ARG;

    /* check server address and port */
    if (((sntp_opmode != SNTP_OPMODE_POLL) || ip_addr_cmp(addr, &sntp_last_server_address))
        && (port == SNTP_PORT))
    /* SNTP_CHECK_RESPONSE >= 1 */
    {
        /* process the response */
        if (p.tot_len == SNTP_MSG_LEN) {
            mode = pbuf_get_at(p, SNTP_OFFSET_LI_VN_MODE) & SNTP_MODE_MASK;
            /* if this is a SNTP response... */
            if (((sntp_opmode == SNTP_OPMODE_POLL) && (mode == SNTP_MODE_SERVER))
                || ((sntp_opmode == SNTP_OPMODE_LISTENONLY) && (mode == SNTP_MODE_BROADCAST)))
            {
                stratum = pbuf_get_at(p, SNTP_OFFSET_STRATUM);

                if (stratum == SNTP_STRATUM_KOD) {
                    /* Kiss-of-death packet. Use another server or increase UPDATE_DELAY. */
                    err = SNTP_ERR_KOD;
                //          LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_recv: Received Kiss-of-Death\n"));
                } else {
                    pbuf_copy_partial(p, &timestamps, sizeof(timestamps), SNTP_OFFSET_TIMESTAMPS);

                    /* check originate_timetamp against sntp_last_timestamp_sent */
                    if (timestamps.orig.sec != sntp_last_timestamp_sent.sec
                        || timestamps.orig.frac != sntp_last_timestamp_sent.frac)
                    {
                        /*LWIP_DEBUGF(SNTP_DEBUG_WARN,
                        ("sntp_recv: Invalid originate timestamp in response\n"));*/
                    } else
                    /* @todo: add code for SNTP_CHECK_RESPONSE >= 3 and >= 4 here */
                    {
                        /* correct answer */
                        err = ERR_OK;
                    }
                }
            } else {
                //        LWIP_DEBUGF(SNTP_DEBUG_WARN, ("sntp_recv: Invalid mode in response: %"U16_F"\n", mode));
                /* wait for correct response */
                err = ERR_TIMEOUT;
            }
        } else {
            //      LWIP_DEBUGF(SNTP_DEBUG_WARN, ("sntp_recv: Invalid packet length: %"U16_F"\n", p.tot_len));
        }
    } else {
        /* packet from wrong remote address or port, wait for correct response */
        err = ERR_TIMEOUT;
    }

    pbuf_free(p);

    if (err == ERR_OK) {
        /* correct packet received: process it it */
        sntp_process(&timestamps);

        /* indicate that server responded */
        sntp_servers[sntp_current_server].reachability |= 1;

        /* Set up timeout for next request (only if poll response was received)*/
        if (sntp_opmode == SNTP_OPMODE_POLL) {
            let sntp_update_delay: u32;
            sys_untimeout(sntp_try_next_server, None);
            sys_untimeout(sntp_request, None);

            /* Correct response, reset retry timeout */
            SNTP_RESET_RETRY_TIMEOUT();

            sntp_update_delay = SNTP_UPDATE_DELAY;
            sys_timeout(sntp_update_delay, sntp_request, None);
            /*LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_recv: Scheduled next time request: %"U32_F" ms\n",
            sntp_update_delay));*/
        }
    } else if (err == SNTP_ERR_KOD) {
        /* KOD errors are only processed in case of an explicit poll response */
        if (sntp_opmode == SNTP_OPMODE_POLL) {
            /* Kiss-of-death packet. Use another server or increase UPDATE_DELAY. */
            sntp_try_next_server(None);
        }
    } else {
        /* ignore any broken packet, poll mode: retry after timeout to avoid flooding */
    }
}

/* Actually send an sntp request to a server.
 *
 * @param server_addr resolved IP address of the SNTP server
 */
pub fn sntp_send_request(server_addr: &mut LwipAddr) {
    let p: &mut pbuf;

    LWIP_ASSERT("server_addr != NULL", server_addr != None);

    p = pbuf_alloc(PBUF_TRANSPORT, SNTP_MSG_LEN, PBUF_RAM);
    if (p != None) {
        let sntpmsg: &mut sntp_msg = p.payload;
        //    LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_send_request: Sending request to server\n"));
        /* initialize request message */
        sntp_initialize_request(sntpmsg);
        /* send request */
        udp_sendto(sntp_pcb, p, server_addr, SNTP_PORT);
        /* free the pbuf after sending it */
        pbuf_free(p);

        /* indicate new packet has been sent */
        sntp_servers[sntp_current_server].reachability <<= 1;

        /* set up receive timeout: try next server or retry on timeout */
        sys_timeout(SNTP_RECV_TIMEOUT, sntp_try_next_server, None);

        /* save server address to verify it in sntp_recv */
        ip_addr_copy(sntp_last_server_address, *server_addr);
    } else {
        /*LWIP_DEBUGF(SNTP_DEBUG_SERIOUS, ("sntp_send_request: Out of memory, trying again in %"U32_F" ms\n",
        SNTP_RETRY_TIMEOUT));*/
        /* out of memory: set up a timer to send a retry */
        sys_timeout(SNTP_RETRY_TIMEOUT, sntp_request, None);
    }
}

/*
 * DNS found callback when using DNS names as server address.
 */
pub fn sntp_dns_found(hostname: &String, ipaddr: &mut LwipAddr, arg: &mut Vec<u8>) {
    if (ipaddr != None) {
        /* Address resolved, send request */
        //    LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_dns_found: Server address resolved, sending request\n"));
        sntp_servers[sntp_current_server].addr = *ipaddr;
        sntp_send_request(ipaddr);
    } else {
        /* DNS resolving failed -> try another server */
        //    LWIP_DEBUGF(SNTP_DEBUG_WARN_STATE, ("sntp_dns_found: Failed to resolve server address resolved, trying next server\n"));
        sntp_try_next_server(None);
    }
}

/*
 * Send out an sntp request.
 *
 * @param arg is unused (only necessary to conform to sys_timeout)
 */
pub fn sntp_request(arg: &mut Vec<u8>) {
    let sntp_server_address: LwipAddr;
    let err: err_t;

    /* initialize SNTP server address */

    if (sntp_servers[sntp_current_server].name) {
        /* always resolve the name and rely on dns-internal caching & timeout */
        ip_addr_set_zero(&sntp_servers[sntp_current_server].addr);
        err = dns_gethostbyname(
            sntp_servers[sntp_current_server].name,
            &sntp_server_address,
            sntp_dns_found,
            None,
        );
        if (err == ERR_INPROGRESS) {
            /* DNS request sent, wait for sntp_dns_found being called */
            //      LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_request: Waiting for server address to be resolved.\n"));
            return;
        } else if (err == ERR_OK) {
            sntp_servers[sntp_current_server].addr = sntp_server_address;
        }
    } else {
        sntp_server_address = sntp_servers[sntp_current_server].addr;
        // err = (ip_addr_isany_val(sntp_server_address)) ? ERR_ARG : ERR_OK;
    }

    if (err == ERR_OK) {
        /*LWIP_DEBUGF(SNTP_DEBUG_TRACE, ("sntp_request: current server address is %s\n",
        ipaddr_ntoa(&sntp_server_address)));*/
        sntp_send_request(&sntp_server_address);
    } else {
        /* address conversion failed, try another server */
        //    LWIP_DEBUGF(SNTP_DEBUG_WARN_STATE, ("sntp_request: Invalid server address, trying next server.\n"));
        sys_timeout(SNTP_RETRY_TIMEOUT, sntp_try_next_server, None);
    }
}

/*
 * @ingroup sntp
 * Initialize this module.
 * Send out request instantly or after SNTP_STARTUP_DELAY(_FUNC).
 */
pub fn sntp_init() {
    /* LWIP_ASSERT_CORE_LOCKED(); is checked by udp_new() */

    sntp_setservername(0, SNTP_SERVER_ADDRESS);

    // #error SNTP_SERVER_ADDRESS string not supported SNTP_SERVER_DNS==0

    if (sntp_pcb == None) {
        sntp_pcb = udp_new_ip_type(IPADDR_TYPE_ANY);
        LWIP_ASSERT(
            "Failed to allocate udp pcb for sntp client",
            sntp_pcb != None,
        );
        if (sntp_pcb != None) {
            udp_recv(sntp_pcb, sntp_recv, None);

            if (sntp_opmode == SNTP_OPMODE_POLL) {
                SNTP_RESET_RETRY_TIMEOUT();

                sys_timeout(SNTP_STARTUP_DELAY_FUNC, sntp_request, None);

                sntp_request(None);
            } else if (sntp_opmode == SNTP_OPMODE_LISTENONLY) {
                ip_set_option(sntp_pcb, SOF_BROADCAST);
                udp_bind(sntp_pcb, IP_ANY_TYPE, SNTP_PORT);
            }
        }
    }
}

/*
 * @ingroup sntp
 * Stop this module.
 */
pub fn sntp_stop() {
    LWIP_ASSERT_CORE_LOCKED();
    if (sntp_pcb != None) {
        let i: u8;
        // TODO:
        // for (i = 0; i < SNTP_MAX_SERVERS; i+= 1) {
        //   sntp_servers[i].reachability = 0;
        // }

        sys_untimeout(sntp_request, None);
        sys_untimeout(sntp_try_next_server, None);
        udp_remove(sntp_pcb);
        sntp_pcb = None;
    }
}

/*
 * @ingroup sntp
 * Get enabled state.
 */
pub fn sntp_enabled() -> u8 {
    return (sntp_pcb != None);
}

/*
 * @ingroup sntp
 * Sets the operating mode.
 * @param operating_mode one of the available operating modes
 */
pub fn sntp_setoperatingmode(operating_mode: u8) {
    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT(
        "Invalid operating mode",
        operating_mode <= SNTP_OPMODE_LISTENONLY,
    );
    LWIP_ASSERT(
        "Operating mode must not be set while SNTP client is running",
        sntp_pcb == None,
    );
    sntp_opmode = operating_mode;
}

/*
 * @ingroup sntp
 * Gets the operating mode.
 */
pub fn sntp_getoperatingmode() -> u8 {
    return sntp_opmode;
}

/*
 * @ingroup sntp
 * Gets the server reachability shift register as described in RFC 5905.
 *
 * @param idx the index of the NTP server
 */
pub fn sntp_getreachability(idx: u8) -> u8 {
    if (idx < SNTP_MAX_SERVERS) {
        return sntp_servers[idx].reachability;
    }
    return 0;
}

/*
 * Config SNTP server handling by IP address, name, or DHCP; clear table
 * @param set_servers_from_dhcp enable or disable getting server addresses from dhcp
 */
pub fn sntp_servermode_dhcp(set_servers_from_dhcp: i32) {
    let new_mode: u8 = set_servers_from_dhcp;
    LWIP_ASSERT_CORE_LOCKED();
    if (sntp_set_servers_from_dhcp != new_mode) {
        sntp_set_servers_from_dhcp = new_mode;
    }
}

/*
 * @ingroup sntp
 * Initialize one of the NTP servers by IP address
 *
 * @param idx the index of the NTP server to set must be < SNTP_MAX_SERVERS
 * @param server IP address of the NTP server to set
 */
pub fn sntp_setserver(idx: u8, server: &mut LwipAddr) {
    LWIP_ASSERT_CORE_LOCKED();
    if (idx < SNTP_MAX_SERVERS) {
        if (server != None) {
            sntp_servers[idx].addr = (*server);
        } else {
            ip_addr_set_zero(&sntp_servers[idx].addr);
        }

        sntp_servers[idx].name = None;
    }
}

/*
 * Initialize one of the NTP servers by IP address, required by DHCP
 *
 * @param num the index of the NTP server to set must be < SNTP_MAX_SERVERS
 * @param server IP address of the NTP server to set
 */
pub fn dhcp_set_ntp_servers(num: u8, server: &mut ip4_addr) {
    /*LWIP_DEBUGF(SNTP_DEBUG_TRACE, ("sntp: %s %u.%u.%u.%u as NTP server #%u via DHCP\n",
    (sntp_set_servers_from_dhcp ? "Got" : "Rejected"),
    ip4_addr1(server), ip4_addr2(server), ip4_addr3(server), ip4_addr4(server), num));*/
    if (sntp_set_servers_from_dhcp && num) {
        let i: u8;
        // TODO
        // for (i = 0; (i < num) && (i < SNTP_MAX_SERVERS); i+= 1) {
        //   let addr: LwipAddr;
        //   ip_addr_copy_from_ip4(addr, server[i]);
        //   sntp_setserver(i, &addr);
        // }
        // for (i = num; i < SNTP_MAX_SERVERS; i+= 1) {
        //   sntp_setserver(i, None);
        // }
    }
}

/*
 * @ingroup sntp
 * Obtain one of the currently configured by IP address (or DHCP) NTP servers
 *
 * @param idx the index of the NTP server
 * @return IP address of the indexed NTP server or "ip_addr_any" if the NTP
 *         server has not been configured by address (or at all).
 */
pub fn sntp_getserver(idx: u8) -> LwipAddr {
    if (idx < SNTP_MAX_SERVERS) {
        return &sntp_servers[idx].addr;
    }
    return IP_ADDR_ANY;
}

/*
 * Initialize one of the NTP servers by name
 *
 * @param idx the index of the NTP server to set must be < SNTP_MAX_SERVERS
 * @param server DNS name of the NTP server to set, to be resolved at contact time
 */
pub fn sntp_setservername(idx: u8, server: &String) {
    LWIP_ASSERT_CORE_LOCKED();
    if (idx < SNTP_MAX_SERVERS) {
        sntp_servers[idx].name = server;
    }
}

/*
 * Obtain one of the currently configured by name NTP servers.
 *
 * @param idx the index of the NTP server
 * @return IP address of the indexed NTP server or NULL if the NTP
 *         server has not been configured by name (or at all)
 */
pub fn sntp_getservername(idx: u8) -> String {
    if (idx < SNTP_MAX_SERVERS) {
        return sntp_servers[idx].name;
    }
    return None;
}
