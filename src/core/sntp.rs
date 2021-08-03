 * @file
 * SNTP client module
 */

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

#define SNTP_SUPPORT_MULTIPLE_SERVERS 1
#else /* NTP_MAX_SERVERS > 1 */
pub const SNTP_SUPPORT_MULTIPLE_SERVERS: u32 = 0;




#error "SNTPv4 RFC 4330 enforces a minimum update time of 15 seconds (define SNTP_SUPPRESS_DELAY_CHECK to disable this error)!"



/* the various debug levels for this file */
#define SNTP_DEBUG_TRACE        (SNTP_DEBUG | LWIP_DBG_TRACE)
#define SNTP_DEBUG_STATE        (SNTP_DEBUG | LWIP_DBG_STATE)
#define SNTP_DEBUG_WARN         (SNTP_DEBUG | LWIP_DBG_LEVEL_WARNING)
#define SNTP_DEBUG_WARN_STATE   (SNTP_DEBUG | LWIP_DBG_LEVEL_WARNING | LWIP_DBG_STATE)
#define SNTP_DEBUG_SERIOUS      (SNTP_DEBUG | LWIP_DBG_LEVEL_SERIOUS)

#define SNTP_ERR_KOD                1

/* SNTP protocol defines */
#define SNTP_MSG_LEN                48

pub const SNTP_OFFSET_LI_VN_MODE: u32 = 0;
pub const SNTP_LI_MASK: u32 = 0xC0;
#define SNTP_LI_NO_WARNING          (0x00 << 6)
#define SNTP_LI_LAST_MINUTE_61_SEC  (0x01 << 6)
#define SNTP_LI_LAST_MINUTE_59_SEC  (0x02 << 6)
#define SNTP_LI_ALARM_CONDITION     (0x03 << 6) /* (clock not synchronized) */

pub const SNTP_VERSION_MASK: u32 = 0x38;
#define SNTP_VERSION                (4/* NTP Version 4*/<<3)

pub const SNTP_MODE_MASK: u32 = 0x07;pub const SNTP_MODE_MASK: u32 = 0x07;pub const SNTP_MODE_MASK: u32 = 0x07;pub const SNTP_MODE_MASK: u32 = 0x07;
#define SNTP_MODE_CLIENT            0x03
#define SNTP_MODE_SERVER            0x04
#define SNTP_MODE_BROADCAST         0x05

#define SNTP_OFFSET_STRATUM         1
pub const SNTP_STRATUM_KOD: u32 = 0x00;

#define SNTP_OFFSET_ORIGINATE_TIME  24
#define SNTP_OFFSET_RECEIVE_TIME    32
#define SNTP_OFFSET_TRANSMIT_TIME   40

/* Number of seconds between 1970 and Feb 7, 2036 06:28:16 UTC (epoch 1) */
#define DIFF_SEC_1970_2036          ((u32)2085978496L)

/* Convert NTP timestamp fraction to microseconds.
 */

# if LWIP_HAVE_INT64
#  define SNTP_FRAC_TO_US(f)        ((u32)(((u64_t)(f) * 1000000) >> 32))
# else
#  define SNTP_FRAC_TO_US(f)        ((u32)(f) / 4295)
# endif


/* Configure behaviour depending on native, microsecond or second precision.
 * Treat NTP timestamps as signed two's-complement integers. This way,
 * timestamps that have the MSB set simply become negative offsets from
 * the epoch (Feb 7, 2036 06:28:16 UTC). Representable dates range from
 * 1968 to 2104.
 */

# ifdef SNTP_SET_SYSTEM_TIME_US
#  define SNTP_SET_SYSTEM_TIME_NTP(s, f) \
    SNTP_SET_SYSTEM_TIME_US((u32)((s) + DIFF_SEC_1970_2036), SNTP_FRAC_TO_US(f))
# else
#  define SNTP_SET_SYSTEM_TIME_NTP(s, f) \
    SNTP_SET_SYSTEM_TIME((u32)((s) + DIFF_SEC_1970_2036))
# endif


/* Get the system time either natively as NTP timestamp or convert from
 * Unix time in seconds and microseconds. Take care to avoid overflow if the
 * microsecond value is at the maximum of 999999. Also add 0.5 us fudge to
 * avoid special values like 0, and to mask round-off errors that would
 * otherwise break round-trip conversion identity.
 */

# define SNTP_GET_SYSTEM_TIME_NTP(s, f) do { \
    sec_: u32, usec_; \
    SNTP_GET_SYSTEM_TIME(sec_, usec_); \
    (s) = (i32)(sec_ - DIFF_SEC_1970_2036); \
    (f) = usec_ * 4295 - ((usec_ * 2143) >> 16) + 2147; \
  } while (0)


/* Start offset of the timestamps to extract from the SNTP packet */
#define SNTP_OFFSET_TIMESTAMPS \
    (SNTP_OFFSET_TRANSMIT_TIME + 8 - sizeof(struct sntp_timestamps))

/* Round-trip delay arithmetic helpers */

# if !LWIP_HAVE_INT64
#  error "SNTP round-trip delay compensation requires 64-bit arithmetic"
# endif
# define SNTP_SEC_FRAC_TO_S64(s, f) \
    ((s64_t)(((u64_t)(s) << 32) | (u32)(f)))
# define SNTP_TIMESTAMP_TO_S64(t) \
    SNTP_SEC_FRAC_TO_S64(lwip_ntohl(t.sec), lwip_ntohl(t.frac))


/*
 * 64-bit NTP timestamp, in network byte order.
 */
struct sntp_time {
  sec: u32;
  frac: u32;
};

/*
 * Timestamps to be extracted from the NTP header.
 */
struct sntp_timestamps {

  struct sntp_time orig;
  struct sntp_time recv;

  struct sntp_time xmit;
};

/*
 * SNTP packet format (without optional fields)
 * Timestamps are coded as 64 bits:
 * - signed 32 bits seconds since Feb 07, 2036, 06:28:16 UTC (epoch 1)
 * - unsigned 32 bits seconds fraction (2^32 = 1 second)
 */

#  include "arch/bpstruct.h"


struct sntp_msg {
  (u8  li_vn_mode);
  (u8  stratum);
  (u8  poll);
  (u8  precision);
  (root_delay: u32);
  (root_dispersion: u32);
  (reference_identifier: u32);
  (reference_timestamp: u32[2]);
  (originate_timestamp: u32[2]);
  (receive_timestamp: u32[2]);
  (transmit_timestamp: u32[2]);
} ;


#  include "arch/epstruct.h"


/* function prototypes */
pub fn sntp_request(arg: &mut Vec<u8>);

/* The operating mode */
static sntp_opmode: u8;

/* The UDP pcb used by the SNTP client */
static sntp_pcb: &mut udp_pcb;
/* Names/Addresses of servers */
struct sntp_server {

  name: String;

  ip_addr_t addr;

  /* Reachability shift register as described in RFC 5905 */
  reachability: u8;

};
static struct sntp_server sntp_servers[SNTP_MAX_SERVERS];


static sntp_set_servers_from_dhcp: u8;


/* The currently used server (initialized to 0) */
static sntp_current_server: u8;
#else /* SNTP_SUPPORT_MULTIPLE_SERVERS */
pub const sntp_current_server: u32 = 0;



#define SNTP_RESET_RETRY_TIMEOUT() sntp_retry_timeout = SNTP_RETRY_TIMEOUT
/* Retry time, initialized with SNTP_RETRY_TIMEOUT and doubled with each retry. */
static sntp_retry_timeout: u32;
#else /* SNTP_RETRY_TIMEOUT_EXP */
#define SNTP_RESET_RETRY_TIMEOUT()
#define sntp_retry_timeout SNTP_RETRY_TIMEOUT



/* Saves the last server address to compare with response */
static ip_addr_t sntp_last_server_address;



/* Saves the last timestamp sent (which is sent back by the server)
 * to compare against in response. Stored in network byte order. */
static struct sntp_time sntp_last_timestamp_sent;



/* Debug prhelper: int. */
static const char *
sntp_format_time(i32 sec)
{
  time_t ut;
  ut = (u32)((u32)sec + DIFF_SEC_1970_2036);
  return ctime(&ut);
}


/*
 * SNTP processing of received timestamp
 */
pub fn
sntp_process(const timestamps: &mut sntp_timestamps)
{
  sec: i32;
  frac: u32;

  sec  = (i32)lwip_ntohl(timestamps.xmit.sec);
  frac = lwip_ntohl(timestamps.xmit.frac);


# if SNTP_CHECK_RESPONSE >= 2
  if (timestamps.recv.sec != 0 || timestamps.recv.frac != 0)
# endif
  {
    dest_sec: i32;
    dest_frac: u32;
    step_sec: u32;

    /* Get the destination time stamp, i.e. the current system time */
    SNTP_GET_SYSTEM_TIME_NTP(dest_sec, dest_frac);

    step_sec = (dest_sec < sec) ? ((u32)sec - (u32)dest_sec)
               : ((u32)dest_sec - (u32)sec);
    /* In order to avoid overflows, skip the compensation if the clock step
     * is larger than about 34 years. */
    if ((step_sec >> 30) == 0) {
      s64_t t1, t2, t3, t4;

      t4 = SNTP_SEC_FRAC_TO_S64(dest_sec, dest_frac);
      t3 = SNTP_SEC_FRAC_TO_S64(sec, frac);
      t1 = SNTP_TIMESTAMP_TO_S64(timestamps.orig);
      t2 = SNTP_TIMESTAMP_TO_S64(timestamps.recv);
      /* Clock offset calculation according to RFC 4330 */
      t4 += ((t2 - t1) + (t3 - t4)) / 2;

      sec  = (i32)((u64_t)t4 >> 32);
      frac = (u32)((u64_t)t4);
    }
  }


  SNTP_SET_SYSTEM_TIME_NTP(sec, frac);
  LWIP_UNUSED_ARG(frac); /* might be unused if only seconds are set */
  LWIP_DEBUGF(SNTP_DEBUG_TRACE, ("sntp_process: %s, %" U32_F " us\n",
                                 sntp_format_time(sec), SNTP_FRAC_TO_US(frac)));
}

/*
 * Initialize request struct to be sent to server.
 */
pub fn
sntp_initialize_request(req: &mut sntp_msg)
{
  memset(req, 0, SNTP_MSG_LEN);
  req.li_vn_mode = SNTP_LI_NO_WARNING | SNTP_VERSION | SNTP_MODE_CLIENT;


  {
    secs: i32;
    sec: u32, frac;
    /* Get the transmit timestamp */
    SNTP_GET_SYSTEM_TIME_NTP(secs, frac);
    sec  = lwip_htonl((u32)secs);
    frac = lwip_htonl(frac);

# if SNTP_CHECK_RESPONSE >= 2
    sntp_last_timestamp_sent.sec  = sec;
    sntp_last_timestamp_sent.frac = frac;
# endif
    req.transmit_timestamp[0] = sec;
    req.transmit_timestamp[1] = frac;
  }

}

/*
 * Retry: send a new request (and increase retry timeout).
 *
 * @param arg is unused (only necessary to conform to sys_timeout)
 */
pub fn
sntp_retry(arg: &mut Vec<u8>)
{
  LWIP_UNUSED_ARG(arg);

  LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_retry: Next request will be sent in %"U32_F" ms\n",
                                 sntp_retry_timeout));

  /* set up a timer to send a retry and increase the retry delay */
  sys_timeout(sntp_retry_timeout, sntp_request, NULL);


  {
    new_retry_timeout: u32;
    /* increase the timeout for next retry */
    new_retry_timeout = sntp_retry_timeout << 1;
    /* limit to maximum timeout and prevent overflow */
    if ((new_retry_timeout <= SNTP_RETRY_TIMEOUT_MAX) &&
        (new_retry_timeout > sntp_retry_timeout)) {
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
pub fn
sntp_try_next_server(arg: &mut Vec<u8>)
{
  old_server: u8, i;
  LWIP_UNUSED_ARG(arg);

  old_server = sntp_current_server;
  for (i = 0; i < SNTP_MAX_SERVERS - 1; i++) {
    sntp_current_server++;
    if (sntp_current_server >= SNTP_MAX_SERVERS) {
      sntp_current_server = 0;
    }
    if (!ip_addr_isany(&sntp_servers[sntp_current_server].addr)

        || (sntp_servers[sntp_current_server].name != NULL)

       ) {
      LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_try_next_server: Sending request to server %"U16_F"\n",
                                     sntp_current_server));
      /* new server: reset retry timeout */
      SNTP_RESET_RETRY_TIMEOUT();
      /* instantly send a request to the next server */
      sntp_request(NULL);
      return;
    }
  }
  /* no other valid server found */
  sntp_current_server = old_server;
  sntp_retry(NULL);
}
#else /* SNTP_SUPPORT_MULTIPLE_SERVERS */
/* Always retry on error if only one server is supported */
#define sntp_try_next_server    sntp_retry


/* UDP recv callback for the sntp pcb */
pub fn
sntp_recv(arg: &mut Vec<u8>, pcb: &mut udp_pcb, p: &mut pbuf, const addr: &mut ip_addr_t, port: u16)
{
  struct sntp_timestamps timestamps;
  mode: u8;
  stratum: u8;
  let err: err_t;

  LWIP_UNUSED_ARG(arg);
  LWIP_UNUSED_ARG(pcb);

  err = ERR_ARG;

  /* check server address and port */
  if (((sntp_opmode != SNTP_OPMODE_POLL) || ip_addr_cmp(addr, &sntp_last_server_address)) &&
      (port == SNTP_PORT))
#else /* SNTP_CHECK_RESPONSE >= 1 */
  LWIP_UNUSED_ARG(addr);
  LWIP_UNUSED_ARG(port);

  {
    /* process the response */
    if (p.tot_len == SNTP_MSG_LEN) {
      mode = pbuf_get_at(p, SNTP_OFFSET_LI_VN_MODE) & SNTP_MODE_MASK;
      /* if this is a SNTP response... */
      if (((sntp_opmode == SNTP_OPMODE_POLL)       && (mode == SNTP_MODE_SERVER)) ||
          ((sntp_opmode == SNTP_OPMODE_LISTENONLY) && (mode == SNTP_MODE_BROADCAST))) {
        stratum = pbuf_get_at(p, SNTP_OFFSET_STRATUM);

        if (stratum == SNTP_STRATUM_KOD) {
          /* Kiss-of-death packet. Use another server or increase UPDATE_DELAY. */
          err = SNTP_ERR_KOD;
          LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_recv: Received Kiss-of-Death\n"));
        } else {
          pbuf_copy_partial(p, &timestamps, sizeof(timestamps), SNTP_OFFSET_TIMESTAMPS);

          /* check originate_timetamp against sntp_last_timestamp_sent */
          if (timestamps.orig.sec != sntp_last_timestamp_sent.sec ||
              timestamps.orig.frac != sntp_last_timestamp_sent.frac) {
            LWIP_DEBUGF(SNTP_DEBUG_WARN,
                        ("sntp_recv: Invalid originate timestamp in response\n"));
          } else

            /* @todo: add code for SNTP_CHECK_RESPONSE >= 3 and >= 4 here */
          {
            /* correct answer */
            err = ERR_OK;
          }
        }
      } else {
        LWIP_DEBUGF(SNTP_DEBUG_WARN, ("sntp_recv: Invalid mode in response: %"U16_F"\n", mode));
        /* wait for correct response */
        err = ERR_TIMEOUT;
      }
    } else {
      LWIP_DEBUGF(SNTP_DEBUG_WARN, ("sntp_recv: Invalid packet length: %"U16_F"\n", p.tot_len));
    }
  }

  else {
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
      sntp_update_delay: u32;
      sys_untimeout(sntp_try_next_server, NULL);
      sys_untimeout(sntp_request, NULL);

      /* Correct response, reset retry timeout */
      SNTP_RESET_RETRY_TIMEOUT();

      sntp_update_delay = (u32)SNTP_UPDATE_DELAY;
      sys_timeout(sntp_update_delay, sntp_request, NULL);
      LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_recv: Scheduled next time request: %"U32_F" ms\n",
                                     sntp_update_delay));
    }
  } else if (err == SNTP_ERR_KOD) {
    /* KOD errors are only processed in case of an explicit poll response */
    if (sntp_opmode == SNTP_OPMODE_POLL) {
      /* Kiss-of-death packet. Use another server or increase UPDATE_DELAY. */
      sntp_try_next_server(NULL);
    }
  } else {
    /* ignore any broken packet, poll mode: retry after timeout to avoid flooding */
  }
}

/* Actually send an sntp request to a server.
 *
 * @param server_addr resolved IP address of the SNTP server
 */
pub fn
sntp_send_request(const server_addr: &mut ip_addr_t)
{
  p: &mut pbuf;

  LWIP_ASSERT("server_addr != NULL", server_addr != NULL);

  p = pbuf_alloc(PBUF_TRANSPORT, SNTP_MSG_LEN, PBUF_RAM);
  if (p != NULL) {
    sntpmsg: &mut sntp_msg = (struct sntp_msg *)p.payload;
    LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_send_request: Sending request to server\n"));
    /* initialize request message */
    sntp_initialize_request(sntpmsg);
    /* send request */
    udp_sendto(sntp_pcb, p, server_addr, SNTP_PORT);
    /* free the pbuf after sending it */
    pbuf_free(p);

    /* indicate new packet has been sent */
    sntp_servers[sntp_current_server].reachability <<= 1;

    /* set up receive timeout: try next server or retry on timeout */
    sys_timeout((u32)SNTP_RECV_TIMEOUT, sntp_try_next_server, NULL);

    /* save server address to verify it in sntp_recv */
    ip_addr_copy(sntp_last_server_address, *server_addr);

  } else {
    LWIP_DEBUGF(SNTP_DEBUG_SERIOUS, ("sntp_send_request: Out of memory, trying again in %"U32_F" ms\n",
                                     (u32)SNTP_RETRY_TIMEOUT));
    /* out of memory: set up a timer to send a retry */
    sys_timeout((u32)SNTP_RETRY_TIMEOUT, sntp_request, NULL);
  }
}


/*
 * DNS found callback when using DNS names as server address.
 */
pub fn
sntp_dns_found(const char *hostname, const ipaddr: &mut ip_addr_t, arg: &mut Vec<u8>)
{
  LWIP_UNUSED_ARG(hostname);
  LWIP_UNUSED_ARG(arg);

  if (ipaddr != NULL) {
    /* Address resolved, send request */
    LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_dns_found: Server address resolved, sending request\n"));
    sntp_servers[sntp_current_server].addr = *ipaddr;
    sntp_send_request(ipaddr);
  } else {
    /* DNS resolving failed -> try another server */
    LWIP_DEBUGF(SNTP_DEBUG_WARN_STATE, ("sntp_dns_found: Failed to resolve server address resolved, trying next server\n"));
    sntp_try_next_server(NULL);
  }
}


/*
 * Send out an sntp request.
 *
 * @param arg is unused (only necessary to conform to sys_timeout)
 */
pub fn
sntp_request(arg: &mut Vec<u8>)
{
  ip_addr_t sntp_server_address;
  let err: err_t;

  LWIP_UNUSED_ARG(arg);

  /* initialize SNTP server address */

  if (sntp_servers[sntp_current_server].name) {
    /* always resolve the name and rely on dns-internal caching & timeout */
    ip_addr_set_zero(&sntp_servers[sntp_current_server].addr);
    err = dns_gethostbyname(sntp_servers[sntp_current_server].name, &sntp_server_address,
                            sntp_dns_found, NULL);
    if (err == ERR_INPROGRESS) {
      /* DNS request sent, wait for sntp_dns_found being called */
      LWIP_DEBUGF(SNTP_DEBUG_STATE, ("sntp_request: Waiting for server address to be resolved.\n"));
      return;
    } else if (err == ERR_OK) {
      sntp_servers[sntp_current_server].addr = sntp_server_address;
    }
  } else

  {
    sntp_server_address = sntp_servers[sntp_current_server].addr;
    err = (ip_addr_isany_val(sntp_server_address)) ? ERR_ARG : ERR_OK;
  }

  if (err == ERR_OK) {
    LWIP_DEBUGF(SNTP_DEBUG_TRACE, ("sntp_request: current server address is %s\n",
                                   ipaddr_ntoa(&sntp_server_address)));
    sntp_send_request(&sntp_server_address);
  } else {
    /* address conversion failed, try another server */
    LWIP_DEBUGF(SNTP_DEBUG_WARN_STATE, ("sntp_request: Invalid server address, trying next server.\n"));
    sys_timeout((u32)SNTP_RETRY_TIMEOUT, sntp_try_next_server, NULL);
  }
}

/*
 * @ingroup sntp
 * Initialize this module.
 * Send out request instantly or after SNTP_STARTUP_DELAY(_FUNC).
 */
pub fn 
sntp_init()
{
  /* LWIP_ASSERT_CORE_LOCKED(); is checked by udp_new() */



  sntp_setservername(0, SNTP_SERVER_ADDRESS);
#else
#error SNTP_SERVER_ADDRESS string not supported SNTP_SERVER_DNS==0



  if (sntp_pcb == NULL) {
    sntp_pcb = udp_new_ip_type(IPADDR_TYPE_ANY);
    LWIP_ASSERT("Failed to allocate udp pcb for sntp client", sntp_pcb != NULL);
    if (sntp_pcb != NULL) {
      udp_recv(sntp_pcb, sntp_recv, NULL);

      if (sntp_opmode == SNTP_OPMODE_POLL) {
        SNTP_RESET_RETRY_TIMEOUT();

        sys_timeout((u32)SNTP_STARTUP_DELAY_FUNC, sntp_request, NULL);
#else
        sntp_request(NULL);

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
pub fn 
sntp_stop()
{
  LWIP_ASSERT_CORE_LOCKED();
  if (sntp_pcb != NULL) {

    i: u8;
    for (i = 0; i < SNTP_MAX_SERVERS; i++) {
      sntp_servers[i].reachability = 0;
    }

    sys_untimeout(sntp_request, NULL);
    sys_untimeout(sntp_try_next_server, NULL);
    udp_remove(sntp_pcb);
    sntp_pcb = NULL;
  }
}

/*
 * @ingroup sntp
 * Get enabled state.
 */
sntp_enabled: u8()
{
  return (sntp_pcb != NULL) ? 1 : 0;
}

/*
 * @ingroup sntp
 * Sets the operating mode.
 * @param operating_mode one of the available operating modes
 */
pub fn 
sntp_setoperatingmode(operating_mode: u8)
{
  LWIP_ASSERT_CORE_LOCKED();
  LWIP_ASSERT("Invalid operating mode", operating_mode <= SNTP_OPMODE_LISTENONLY);
  LWIP_ASSERT("Operating mode must not be set while SNTP client is running", sntp_pcb == NULL);
  sntp_opmode = operating_mode;
}

/*
 * @ingroup sntp
 * Gets the operating mode.
 */
u8
sntp_getoperatingmode()
{
  return sntp_opmode;
}


/*
 * @ingroup sntp
 * Gets the server reachability shift register as described in RFC 5905.
 *
 * @param idx the index of the NTP server
 */
u8
sntp_getreachability(idx: u8)
{
  if (idx < SNTP_MAX_SERVERS) {
    return sntp_servers[idx].reachability;
  }
  return 0;
}



/*
 * Config SNTP server handling by IP address, name, or DHCP; clear table
 * @param set_servers_from_dhcp enable or disable getting server addresses from dhcp
 */
pub fn 
sntp_servermode_dhcp(set_servers_from_dhcp: int)
{
  new_mode: u8 = set_servers_from_dhcp ? 1 : 0;
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
pub fn 
sntp_setserver(idx: u8, const server: &mut ip_addr_t)
{
  LWIP_ASSERT_CORE_LOCKED();
  if (idx < SNTP_MAX_SERVERS) {
    if (server != NULL) {
      sntp_servers[idx].addr = (*server);
    } else {
      ip_addr_set_zero(&sntp_servers[idx].addr);
    }

    sntp_servers[idx].name = NULL;

  }
}


/*
 * Initialize one of the NTP servers by IP address, required by DHCP
 *
 * @param num the index of the NTP server to set must be < SNTP_MAX_SERVERS
 * @param server IP address of the NTP server to set
 */
pub fn 
dhcp_set_ntp_servers(num: u8, const server: &mut ip4_addr)
{
  LWIP_DEBUGF(SNTP_DEBUG_TRACE, ("sntp: %s %u.%u.%u.%u as NTP server #%u via DHCP\n",
                                 (sntp_set_servers_from_dhcp ? "Got" : "Rejected"),
                                 ip4_addr1(server), ip4_addr2(server), ip4_addr3(server), ip4_addr4(server), num));
  if (sntp_set_servers_from_dhcp && num) {
    i: u8;
    for (i = 0; (i < num) && (i < SNTP_MAX_SERVERS); i++) {
      ip_addr_t addr;
      ip_addr_copy_from_ip4(addr, server[i]);
      sntp_setserver(i, &addr);
    }
    for (i = num; i < SNTP_MAX_SERVERS; i++) {
      sntp_setserver(i, NULL);
    }
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
const ip_addr_t *
sntp_getserver(idx: u8)
{
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
pub fn 
sntp_setservername(idx: u8, const char *server)
{
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
const char *
sntp_getservername(idx: u8)
{
  if (idx < SNTP_MAX_SERVERS) {
    return sntp_servers[idx].name;
  }
  return NULL;
}



