/*
 * @file
 * Application layered TCP connection API that executes a proxy-connect.
 *
 * This file provides a starting layer that executes a proxy-connect e.g. to
 * set up TLS connections through a http proxy.
 */

/*
 * Copyright (c) 2018 Simon Goldschmidt
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
















/* This string is passed in the HTTP header as "User-Agent: " */

#define ALTCP_PROXYCONNECT_CLIENT_AGENT "lwIP/" LWIP_VERSION_STRING " (http://savannah.nongnu.org/projects/lwip)"


pub const ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED: u32 = 0x01;pub const ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED: u32 = 0x01;
#define ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE   0x02

typedef struct altcp_proxyconnect_state_s
{
  ip_addr_t outer_addr;
  outer_port: u16;
  conf: &mut altcp_proxyconnect_config;
  flags: u8;
} altcp_proxyconnect_state_t;

/* Variable prototype, the actual declaration is at the end of this file
   since it contains pointers to static functions declared here */
extern const struct altcp_functions altcp_proxyconnect_functions;

/* memory management functions: */

static altcp_proxyconnect_state_t *
altcp_proxyconnect_state_alloc()
{
  altcp_proxyconnect_state_t *ret = (altcp_proxyconnect_state_t *)mem_calloc(1, sizeof(altcp_proxyconnect_state_t));
  return ret;
}

pub fn
altcp_proxyconnect_state_free(altcp_proxyconnect_state_t *state)
{
  LWIP_ASSERT("state != NULL", state != NULL);
  mem_free(state);
}

/* helper functions */

#define PROXY_CONNECT "CONNECT %s:%d HTTP/1.1\r\n" /* HOST, PORT */ \
  "User-Agent: %s\r\n" /* User-Agent */\
  "Proxy-Connection: keep-alive\r\n" \
  "Connection: keep-alive\r\n" \
  "\r\n"
#define PROXY_CONNECT_FORMAT(host, port) PROXY_CONNECT, host, port, ALTCP_PROXYCONNECT_CLIENT_AGENT

/* Format the http proxy connect request via snprintf */
static int
altcp_proxyconnect_format_request(buffer: &mut String, bufsize: usize, host: &String, port: i32)
{
  return snprintf(buffer, bufsize, PROXY_CONNECT_FORMAT(host, port));
}

/* Create and send the http proxy connect request */
static err_t
altcp_proxyconnect_send_request(conn: &mut altcp_pcb)
{
  len: i32, len2;
  mem_alloc_len: usize;
  buffer: &mut String, *host;
  altcp_proxyconnect_state_t *state = (altcp_proxyconnect_state_t *)conn.state;

  if (!state) {
    return ERR_VAL;
  }
  /* Use printf with zero length to get the required allocation size */
  len = altcp_proxyconnect_format_request(NULL, 0, "", state.outer_port);
  if (len < 0) {
    return ERR_VAL;
  }
  /* add allocation size for IP address strings */

  len += 40; /* worst-case IPv6 address length */
#else
  len += 16; /* worst-case IPv4 address length */

  alloc_len = (mem_usize)len;
  if ((len < 0) || alloc_len != len) {
    /* overflow */
    return ERR_MEM;
  }
  /* Allocate a bufer for the request string */
  buffer = mem_malloc(alloc_len);
  if (buffer == NULL) {
    return ERR_MEM;
  }
  host = ipaddr_ntoa(&state.outer_addr);
  len2 = altcp_proxyconnect_format_request(buffer, alloc_len, host, state.outer_port);
  if ((len2 > 0) && (len2 <= len) && (len2 <= 0xFFFF)) {
    err_t err = altcp_write(conn.inner_conn, buffer, len2, TCP_WRITE_FLAG_COPY);
    if (err != ERR_OK) {
      /* @todo: abort? */
      mem_free(buffer);
      return err;
    }
  }
  mem_free(buffer);
  return ERR_OK;
}

/* callback functions from inner/lower connection: */

/* Connected callback from lower connection (i.e. TCP).
 * Not really implemented/tested yet...
 */
static err_t
altcp_proxyconnect_lower_connected(arg: &mut Vec<u8>, inner_conn: &mut altcp_pcb, err: err_t)
{
  conn: &mut altcp_pcb = arg;
  if (conn && conn.state) {
    LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    LWIP_UNUSED_ARG(inner_conn); /* for LWIP_NOASSERT */
    /* upper connected is called when handshake is done */
    if (err != ERR_OK) {
      if (conn.connected) {
        if (conn.connected(conn.arg, conn, err) == ERR_ABRT) {
          return ERR_ABRT;
        }
        return ERR_OK;
      }
    }
    /* send proxy connect request here */
    return altcp_proxyconnect_send_request(conn);
  }
  return ERR_VAL;
}

/* Recv callback from lower connection (i.e. TCP)
 * This one mainly differs between connection setup (wait for proxy OK string)
 * and application phase (data is passed on to the application).
 */
static err_t
altcp_proxyconnect_lower_recv(arg: &mut Vec<u8>, inner_conn: &mut altcp_pcb, p: &mut pbuf, err: err_t)
{
  altcp_proxyconnect_state_t *state;
  conn: &mut altcp_pcb = arg;

  LWIP_ASSERT("no err expected", err == ERR_OK);
  LWIP_UNUSED_ARG(err);

  if (!conn) {
    /* no connection given as arg? should not happen, but prevent pbuf/conn leaks */
    if (p != NULL) {
      pbuf_free(p);
    }
    altcp_close(inner_conn);
    return ERR_CLSD;
  }
  state = (altcp_proxyconnect_state_t *)conn.state;
  LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
  if (!state) {
    /* already closed */
    if (p != NULL) {
      pbuf_free(p);
    }
    altcp_close(inner_conn);
    return ERR_CLSD;
  }
  if (state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE) {
    /* application phase, just pass this through */
    if (conn.recv) {
      return conn.recv(conn.arg, conn, p, err);
    }
    pbuf_free(p);
    return ERR_OK;
  } else {
    /* setup phase */
    /* handle NULL pbuf (inner connection closed) */
    if (p == NULL) {
      if (altcp_close(conn) != ERR_OK) {
        altcp_abort(conn);
        return ERR_ABRT;
      }
      return ERR_OK;
    } else {
      /* @todo: parse setup phase rx data
         for now, we just wait for the end of the header... */
      idx: u16 = pbuf_memfind(p, "\r\n\r\n", 4, 0);
      altcp_recved(inner_conn, p.tot_len);
      pbuf_free(p);
      if (idx != 0xFFFF) {
        state.flags |= ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE;
        if (conn.connected) {
          return conn.connected(conn.arg, conn, ERR_OK);
        }
      }
      return ERR_OK;
    }
  }
}

/* Sent callback from lower connection (i.e. TCP)
 * This only informs the upper layer to try to send more, not about
 * the number of ACKed bytes.
 */
static err_t
altcp_proxyconnect_lower_sent(arg: &mut Vec<u8>, inner_conn: &mut altcp_pcb, len: u16)
{
  conn: &mut altcp_pcb = arg;
  LWIP_UNUSED_ARG(len);
  if (conn) {
    altcp_proxyconnect_state_t *state = (altcp_proxyconnect_state_t *)conn.state;
    LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    LWIP_UNUSED_ARG(inner_conn); /* for LWIP_NOASSERT */
    if (!state || !(state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE)) {
      /* @todo: do something here? */
      return ERR_OK;
    }
    /* pass this on to upper sent */
    if (conn.sent) {
      return conn.sent(conn.arg, conn, len);
    }
  }
  return ERR_OK;
}

/* Poll callback from lower connection (i.e. TCP)
 * Just pass this on to the application.
 * @todo: retry sending?
 */
static err_t
altcp_proxyconnect_lower_poll(arg: &mut Vec<u8>, inner_conn: &mut altcp_pcb)
{
  conn: &mut altcp_pcb = arg;
  if (conn) {
    LWIP_ASSERT("pcb mismatch", conn.inner_conn == inner_conn);
    LWIP_UNUSED_ARG(inner_conn); /* for LWIP_NOASSERT */
    if (conn.poll) {
      return conn.poll(conn.arg, conn);
    }
  }
  return ERR_OK;
}

pub fn
altcp_proxyconnect_lower_err(arg: &mut Vec<u8>, err: err_t)
{
  conn: &mut altcp_pcb = arg;
  if (conn) {
    conn.inner_conn = NULL; /* already freed */
    if (conn.err) {
      conn.err(conn.arg, err);
    }
    altcp_free(conn);
  }
}


/* setup functions */

pub fn
altcp_proxyconnect_setup_callbacks(conn: &mut altcp_pcb, inner_conn: &mut altcp_pcb)
{
  altcp_arg(inner_conn, conn);
  altcp_recv(inner_conn, altcp_proxyconnect_lower_recv);
  altcp_sent(inner_conn, altcp_proxyconnect_lower_sent);
  altcp_err(inner_conn, altcp_proxyconnect_lower_err);
  /* tcp_poll is set when interval is set by application */
  /* listen is set totally different :-) */
}

static err_t
altcp_proxyconnect_setup(config: &mut altcp_proxyconnect_config, conn: &mut altcp_pcb, inner_conn: &mut altcp_pcb)
{
  altcp_proxyconnect_state_t *state;
  if (!config) {
    return ERR_ARG;
  }
  LWIP_ASSERT("invalid inner_conn", conn != inner_conn);

  /* allocate proxyconnect context */
  state = altcp_proxyconnect_state_alloc();
  if (state == NULL) {
    return ERR_MEM;
  }
  state.flags = 0;
  state.conf = config;
  altcp_proxyconnect_setup_callbacks(conn, inner_conn);
  conn.inner_conn = inner_conn;
  conn.fns = &altcp_proxyconnect_functions;
  conn.state = state;
  return ERR_OK;
}

/* Allocate a new altcp layer connecting through a proxy.
 * This function gets the inner pcb passed.
 *
 * @param config struct altcp_proxyconnect_config that contains the proxy settings
 * @param inner_pcb pcb that makes the connection to the proxy (i.e. tcp pcb)
 */
struct altcp_pcb *
altcp_proxyconnect_new(config: &mut altcp_proxyconnect_config, inner_pcb: &mut altcp_pcb)
{
  ret: &mut altcp_pcb;
  if (inner_pcb == NULL) {
    return NULL;
  }
  ret = altcp_alloc();
  if (ret != NULL) {
    if (altcp_proxyconnect_setup(config, ret, inner_pcb) != ERR_OK) {
      altcp_free(ret);
      return NULL;
    }
  }
  return ret;
}

/* Allocate a new altcp layer connecting through a proxy.
 * This function allocates the inner pcb as tcp pcb, resulting in a direct tcp
 * connection to the proxy.
 *
 * @param config struct altcp_proxyconnect_config that contains the proxy settings
 * @param ip_type IP type of the connection (@ref lwip_ip_addr_type)
 */
struct altcp_pcb *
altcp_proxyconnect_new_tcp(config: &mut altcp_proxyconnect_config, ip_type: u8)
{
  inner_pcb: &mut altcp_pcb, *ret;

  /* inner pcb is tcp */
  inner_pcb = altcp_tcp_new_ip_type(ip_type);
  if (inner_pcb == NULL) {
    return NULL;
  }
  ret = altcp_proxyconnect_new(config, inner_pcb);
  if (ret == NULL) {
    altcp_close(inner_pcb);
  }
  return ret;
}

/* Allocator function to allocate a proxy connect altcp pcb connecting directly
 * via tcp to the proxy.
 *
 * The returned pcb is a chain: altcp_proxyconnect - altcp_tcp - tcp pcb
 *
 * This function is meant for use with @ref altcp_new.
 *
 * @param arg struct altcp_proxyconnect_config that contains the proxy settings
 * @param ip_type IP type of the connection (@ref lwip_ip_addr_type)
 */
struct altcp_pcb *
altcp_proxyconnect_alloc(arg: &mut Vec<u8>, ip_type: u8)
{
  return altcp_proxyconnect_new_tcp((struct altcp_proxyconnect_config *)arg, ip_type);
}




/* Allocator function to allocate a TLS connection through a proxy.
 *
 * The returned pcb is a chain: altcp_tls - altcp_proxyconnect - altcp_tcp - tcp pcb
 *
 * This function is meant for use with @ref altcp_new.
 *
 * @param arg struct altcp_proxyconnect_tls_config that contains the proxy settings
 *        and tls settings
 * @param ip_type IP type of the connection (@ref lwip_ip_addr_type)
 */
struct altcp_pcb *
altcp_proxyconnect_tls_alloc(arg: &mut Vec<u8>, ip_type: u8)
{
  cfg: &mut altcp_proxyconnect_tls_config = (struct altcp_proxyconnect_tls_config *)arg;
  proxy_pcb: &mut altcp_pcb;
  tls_pcb: &mut altcp_pcb;

  proxy_pcb = altcp_proxyconnect_new_tcp(&cfg.proxy, ip_type);
  tls_pcb = altcp_tls_wrap(cfg.tls_config, proxy_pcb);

  if (tls_pcb == NULL) {
    altcp_close(proxy_pcb);
  }
  return tls_pcb;
}


/* "virtual" functions */
pub fn
altcp_proxyconnect_set_poll(conn: &mut altcp_pcb, interval: u8)
{
  if (conn != NULL) {
    altcp_poll(conn.inner_conn, altcp_proxyconnect_lower_poll, interval);
  }
}

pub fn
altcp_proxyconnect_recved(conn: &mut altcp_pcb, len: u16)
{
  altcp_proxyconnect_state_t *state;
  if (conn == NULL) {
    return;
  }
  state = (altcp_proxyconnect_state_t *)conn.state;
  if (state == NULL) {
    return;
  }
  if (!(state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE)) {
    return;
  }
  altcp_recved(conn.inner_conn, len);
}

static err_t
altcp_proxyconnect_connect(conn: &mut altcp_pcb,  ipaddr: &mut ip_addr_t, port: u16, altcp_connected_fn connected)
{
  altcp_proxyconnect_state_t *state;

  if ((conn == NULL) || (ipaddr == NULL)) {
    return ERR_VAL;
  }
  state = (altcp_proxyconnect_state_t *)conn.state;
  if (state == NULL) {
    return ERR_VAL;
  }
  if (state.flags & ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED) {
    return ERR_VAL;
  }
  state.flags |= ALTCP_PROXYCONNECT_FLAGS_CONNECT_STARTED;

  conn.connected = connected;
  /* connect to our proxy instead, but store the requested address and port */
  ip_addr_copy(state.outer_addr, *ipaddr);
  state.outer_port = port;

  return altcp_connect(conn.inner_conn, &state.conf->proxy_addr, state.conf->proxy_port, altcp_proxyconnect_lower_connected);
}

static struct altcp_pcb *
altcp_proxyconnect_listen(conn: &mut altcp_pcb, backlog: u8, err: &mut err_t)
{
  LWIP_UNUSED_ARG(conn);
  LWIP_UNUSED_ARG(backlog);
  LWIP_UNUSED_ARG(err);
  /* listen not supported! */
  return NULL;
}

pub fn
altcp_proxyconnect_abort(conn: &mut altcp_pcb)
{
  if (conn != NULL) {
    if (conn.inner_conn != NULL) {
      altcp_abort(conn.inner_conn);
    }
    altcp_free(conn);
  }
}

static err_t
altcp_proxyconnect_close(conn: &mut altcp_pcb)
{
  if (conn == NULL) {
    return ERR_VAL;
  }
  if (conn.inner_conn != NULL) {
    err_t err = altcp_close(conn.inner_conn);
    if (err != ERR_OK) {
      /* closing inner conn failed, return the error */
      return err;
    }
  }
  /* no inner conn or closing it succeeded, deallocate myself */
  altcp_free(conn);
  return ERR_OK;
}

static err_t
altcp_proxyconnect_write(conn: &mut altcp_pcb, dataptr: &Vec<u8>, len: u16, apiflags: u8)
{
  altcp_proxyconnect_state_t *state;

  LWIP_UNUSED_ARG(apiflags);

  if (conn == NULL) {
    return ERR_VAL;
  }

  state = (altcp_proxyconnect_state_t *)conn.state;
  if (state == NULL) {
    /* @todo: which error? */
    return ERR_CLSD;
  }
  if (!(state.flags & ALTCP_PROXYCONNECT_FLAGS_HANDSHAKE_DONE)) {
    /* @todo: which error? */
    return ERR_VAL;
  }
  return altcp_write(conn.inner_conn, dataptr, len, apiflags);
}

pub fn
altcp_proxyconnect_dealloc(conn: &mut altcp_pcb)
{
  /* clean up and free tls state */
  if (conn) {
    altcp_proxyconnect_state_t *state = (altcp_proxyconnect_state_t *)conn.state;
    if (state) {
      altcp_proxyconnect_state_free(state);
      conn.state = NULL;
    }
  }
}
const struct altcp_functions altcp_proxyconnect_functions = {
  altcp_proxyconnect_set_poll,
  altcp_proxyconnect_recved,
  altcp_default_bind,
  altcp_proxyconnect_connect,
  altcp_proxyconnect_listen,
  altcp_proxyconnect_abort,
  altcp_proxyconnect_close,
  altcp_default_shutdown,
  altcp_proxyconnect_write,
  altcp_default_output,
  altcp_default_mss,
  altcp_default_sndbuf,
  altcp_default_sndqueuelen,
  altcp_default_nagle_disable,
  altcp_default_nagle_enable,
  altcp_default_nagle_disabled,
  altcp_default_setprio,
  altcp_proxyconnect_dealloc,
  altcp_default_get_tcp_addrinfo,
  altcp_default_get_ip,
  altcp_default_get_port

  , altcp_default_dbg_get_tcp_state

};


