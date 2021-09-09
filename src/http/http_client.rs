/*
 * @file
 * HTTP client
 */

/*
 * Copyright (c) 2018 Simon Goldschmidt <goldsimon@gmx.de>
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
 */

/*
 * @defgroup httpc HTTP client
 * @ingroup apps
 * @todo:
 * - persistent connections
 * - select outgoing http version
 * - optionally follow redirect
 * - check request uri for invalid characters? (e.g. encode spaces)
 * - IPv6 support
 */
















/*
 * HTTPC_DEBUG: Enable debugging for HTTP client.
 */

pub const HTTPC_DEBUG: u32 = LWIP_DBG_OFF;


/* Set this to 1 to keep server name and uri in request state */

pub const HTTPC_DEBUG_REQUEST: u32 = 0;


/* This string is passed in the HTTP header as "User-Agent: " */

// pub const HTTPC_CLIENT_AGENT "lwIP/" LWIP_VERSION_STRING " (http://savannah.nongnu.org/projects/lwip)"


/* the various debug levels for this file */
// #define HTTPC_DEBUG_TRACE        (HTTPC_DEBUG | LWIP_DBG_TRACE)
// #define HTTPC_DEBUG_STATE        (HTTPC_DEBUG | LWIP_DBG_STATE)
// #define HTTPC_DEBUG_WARN         (HTTPC_DEBUG | LWIP_DBG_LEVEL_WARNING)
// #define HTTPC_DEBUG_WARN_STATE   (HTTPC_DEBUG | LWIP_DBG_LEVEL_WARNING | LWIP_DBG_STATE)
// #define HTTPC_DEBUG_SERIOUS      (HTTPC_DEBUG | LWIP_DBG_LEVEL_SERIOUS)

pub const HTTPC_POLL_INTERVAL: u32 = 1; 
pub const HTTPC_POLL_TIMEOUT: u32 = 30;  /* 15 seconds */
pub const HTTPC_CONTENT_LEN_INVALID: u32 = 0xFFFFFFFF;

/* GET request basic */
// TODO: compose using format strings
/*
  GET %s HTTP/1.1
  User-Agent: %s
  Accept: &#42/&#42
  Host: %s
  Connection: Close
*/


// pub const HTTPC_REQ_11: String = r#"GET %s HTTP/1.1" /* URI */
//     "User-Agent: %s" /* User-Agent */ 
//     "Accept: */*
//     "Connection: Close\r\n" /* we don't support persistent connections, yet */ 
//     "\r\n"#.to_string();
// // #define HTTPC_REQ_11_FORMAT(uri) HTTPC_REQ_11, uri, HTTPC_CLIENT_AGENT

// /* GET request with host */
// pub const HTTPC_REQ_11_HOST: String = r#"GET %s HTTP/1.1
//     "User-Agent: %s\
//     "Accept: */*
//     "Host: %s
//     "Connection: Close
//     "\r\n"#.to_string();
// // #define HTTPC_REQ_11_HOST_FORMAT(uri, srv_name) HTTPC_REQ_11_HOST, uri, HTTPC_CLIENT_AGENT, srv_name

// /* GET request with proxy */
// pub const HTTPC_REQ_11_PROXY: String "GET http://%s%s HTTP/1.1\r\n" /* HOST, URI */\
//     "User-Agent: %s\r\n" /* User-Agent */ \
//     "Accept: */*\r\n" \
//     "Host: %s\r\n" /* server name */ \
//     "Connection: Close\r\n" /* we don't support persistent connections, yet */ \
//     "\r\n"
// // #define HTTPC_REQ_11_PROXY_FORMAT(host, uri, srv_name) HTTPC_REQ_11_PROXY, host, uri, HTTPC_CLIENT_AGENT, srv_name

// /* GET request with proxy (non-default server port) */
// #define HTTPC_REQ_11_PROXY_PORT "GET http://%s:%d%s HTTP/1.1\r\n" /* HOST, host-port, URI */\
//     "User-Agent: %s\r\n" /* User-Agent */ \
//     "Accept: */*\r\n" \
//     "Host: %s\r\n" /* server name */ \
//     "Connection: Close\r\n" /* we don't support persistent connections, yet */ \
//     "\r\n"
// #define HTTPC_REQ_11_PROXY_PORT_FORMAT(host, host_port, uri, srv_name) HTTPC_REQ_11_PROXY_PORT, host, host_port, uri, HTTPC_CLIENT_AGENT, srv_name

pub enum httpc_parse_state_t {
  HTTPC_PARSE_WAIT_FIRST_LINE = 0,
  HTTPC_PARSE_WAIT_HEADERS,
  HTTPC_PARSE_RX_DATA
} 

pub struct httpc_state_t
{
  pub pc: AlTcpPcb,
  pub remote_addr: LwipAddr,
  pub remote_port: u16,
  pub lettimeout_ticks: u64,
  pub request: PacketBuffer,
  pub rx_hdrs: PacketBuffer,
  pub rx_http_version: u32,
  pub rx_status: u32,
  pub recv_fn: altcp_recv_fn,
  pub conn_settings: httpc_connection_t,
  pub callback_arg: Vec<u8>,
  pub rx_content_len: usize,
  pub hdr_content_len: usize,
  pub parse_state: httpc_parse_state_t,
  pub server_name: String,
  pub uri: String,

} 

/* Free http client state and deallocate all resources within */
pub fn httpc_free_state(req: &mut httpc_state_t) -> Result<(), LwipError>
{
  let tpcb: &mut AlTcpPcb;

  if (req.request != None) {
    pbuf_free(req.request);
    req.request = None;
  }
  if (req.rx_hdrs != None) {
    pbuf_free(req.rx_hdrs);
    req.rx_hdrs = None;
  }

  tpcb = req.pcb;
  mem_free(req);
  req = None;

  if (tpcb != None) {
    let r: err_t;
    altcp_arg(tpcb, None);
    altcp_recv(tpcb, None);
    altcp_err(tpcb, None);
    altcp_poll(tpcb, None, 0);
    altcp_sent(tpcb, None);
    r = altcp_close(tpcb);
    if (r != ERR_OK) {
      altcp_abort(tpcb);
      return ERR_ABRT;
    }
  }
 return Ok(());
}

/* Close the connection: call finished callback and free the state */
pub fn httpc_close(req: &mut httpc_state_t, result: httpc_result_t, server_response: u32, err: err_t) -> Result<(), LwipError>
{
  if (req != None) {
    if (req.conn_settings != None) {
      if (req.conn_settings.result_fn != None) {
        req.conn_settings.result_fn(req.callback_arg, result, req.rx_content_len, server_response, err);
      }
    }
    return httpc_free_state(req);
  }
 return Ok(());
}

/* Parse http header response line 1 */
pub fn http_parse_response_status(p: &mut pbuf, http_version: &mut u16, http_status: &mut u16, http_status_str_offset: &mut u16) -> Result<(), LwipError>
{
  let end1: u16 = pbuf_memfind(p, "\r\n", 2, 0);
  if (end1 != 0xFFFF) {
    /* get parts of first line */
    let space1: u16;
    let space2: u16;
    space1 = pbuf_memfind(p, " ", 1, 0);
    if (space1 != 0xFFFF) {
      if ((pbuf_memcmp(p, 0, "HTTP/", 5) == 0)  && (pbuf_get_at(p, 6) == '.')) {
        let status_num: String;
        let status_num_len: usize;
        /* parse http version */
        let version: u16 = pbuf_get_at(p, 5) - '0';
        version <<= 8;
        version |= pbuf_get_at(p, 7) - '0';
        *http_version = version;

        /* parse http status number */
        space2 = pbuf_memfind(p, " ", 1, space1 + 1);
        if (space2 != 0xFFFF) {
          *http_status_str_offset = space2 + 1;
          status_num_len = space2 - space1 - 1;
        } else {
          status_num_len = end1 - space1 - 1;
        }
        //memset(status_num, 0, sizeof(status_num));
        if (pbuf_copy_partial(p, status_num, status_num_len, space1 + 1) == status_num_len) {
          let status: i32 = atoi(status_num);
          if ((status > 0) && (status <= 0xFFFF)) {
            *http_status = status;
           return Ok(());
          }
        }
      }
    }
  }
  return ERR_VAL;
}

/* Wait for all headers to be received, return its length and content-length (if available) */
pub fn http_wait_headers(p: &mut pbuf, content_length: &mut u32, total_header_len: &mut u16) -> Result<(), LwipError>
{
  let end1: u16 = pbuf_memfind(p, "\r\n\r\n", 4, 0);
  if (end1 < (0xFFFF - 2)) {
    /* all headers received */
    /* check if we have a content length (@todo: case insensitive?) */
    let content_len_hdr: u16;
    *content_length = HTTPC_CONTENT_LEN_INVALID;
    *total_header_len = end1 + 4;

    content_len_hdr = pbuf_memfind(p, "Content-Length: ", 16, 0);
    if (content_len_hdr != 0xFFFF) {
      let content_len_line_end: u16 = pbuf_memfind(p, "\r\n", 2, content_len_hdr);
      if (content_len_line_end != 0xFFFF) {
        let content_len_num: String;
        let content_len_num_len: u16 = (content_len_line_end - content_len_hdr - 16);
        //memset(content_len_num, 0, sizeof(content_len_num));
        if (pbuf_copy_partial(p, content_len_num, content_len_num_len, content_len_hdr + 16) == content_len_num_len) {
          let len: i32 = atoi(content_len_num);
          if ((len >= 0) && (len < HTTPC_CONTENT_LEN_INVALID)) {
            *content_length = len;
          }
        }
      }
    }
   return Ok(());
  }
  return ERR_VAL;
}

/* http client tcp recv callback */
pub fn httpc_tcp_recv(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, p: &mut pbuf, r: err_t) -> Result<(), LwipError>
{
  let req: &mut httpc_state_t = arg;
  

  if (p == None) {
    let result: httpc_result_t;
    if (req.parse_state != HTTPC_PARSE_RX_DATA) {
      /* did not get RX data yet */
      result = HTTPC_RESULT_ERR_CLOSED;
    } else if ((req.hdr_content_len != HTTPC_CONTENT_LEN_INVALID) &&
      (req.hdr_content_len != req.rx_content_len)) {
      /* header has been received with content length but not all data received */
      result = HTTPC_RESULT_ERR_CONTENT_LEN;
    } else {
      /* receiving data and either all data received or no content length header */
      result = HTTPC_RESULT_OK;
    }
    return httpc_close(req, result, req.rx_status, ERR_OK);
  }
  if (req.parse_state != HTTPC_PARSE_RX_DATA) {
    if (req.rx_hdrs == None) {
      req.rx_hdrs = p;
    } else {
      pbuf_cat(req.rx_hdrs, p);
    }
    if (req.parse_state == HTTPC_PARSE_WAIT_FIRST_LINE) {
      let status_str_off: u16;
      let err: err_t = http_parse_response_status(req.rx_hdrs, &req.rx_http_version, &req.rx_status, &status_str_off);
      if (err == ERR_OK) {
        /* don't care status string */
        req.parse_state = HTTPC_PARSE_WAIT_HEADERS;
      }
    }
    if (req.parse_state == HTTPC_PARSE_WAIT_HEADERS) {
      let total_header_len: u16;
      let err: err_t = http_wait_headers(req.rx_hdrs, &req.hdr_content_len, &total_header_len);
      if (err == ERR_OK) {
        let q: &mut pbuf;
        /* full header received, send window update for header bytes and call into client callback */
        altcp_recved(pcb, total_header_len);
        if (req.conn_settings) {
          if (req.conn_settings.headers_done_fn) {
            err = req.conn_settings.headers_done_fn(req, req.callback_arg, req.rx_hdrs, total_header_len, req.hdr_content_len);
            if (err != ERR_OK) {
              return httpc_close(req, HTTPC_RESULT_LOCAL_ABORT, req.rx_status, err);
            }
          }
        }
        /* hide header bytes in pbuf */
        q = pbuf_free_header(req.rx_hdrs, total_header_len);
        p = q;
        req.rx_hdrs = None;
        /* go on with data */
        req.parse_state = HTTPC_PARSE_RX_DATA;
      }
    }
  }
  if ((p != None) && (req.parse_state == HTTPC_PARSE_RX_DATA)) {
    req.rx_content_len += p.tot_len;
    if (req.recv_fn != None) {
      /* directly return here: the connection migth already be aborted from the callback! */
      return req.recv_fn(req.callback_arg, pcb, p, r);
    } else {
      altcp_recved(pcb, p.tot_len);
      pbuf_free(p);
    }
  }
 return Ok(());
}

/* http client tcp err callback */
pub fn
httpc_tcp_err(arg: &mut Vec<u8>, err: err_t)
{
  let req: &mut httpc_state_t = arg;
  if (req != None) {
    /* pcb has already been deallocated */
    req.pcb = None;
    httpc_close(req, HTTPC_RESULT_ERR_CLOSED, 0, err);
  }
}

/* http client tcp poll callback */
pub fn httpc_tcp_poll(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb) -> Result<(), LwipError>
{
  /* implement timeout */
  let req: &mut httpc_state_t = arg;
  
  if (req != None) {
    if (req.timeout_ticks) {
      req.timeout_ticks -= 1;
    }
    if (!req.timeout_ticks) {
      return httpc_close(req, HTTPC_RESULT_ERR_TIMEOUT, 0, ERR_OK);
    }
  }
 return Ok(());
}

/* http client tcp sent callback */
pub fn httpc_tcp_sent(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, len: usize) -> Result<(), LwipError>
{
  /* nothing to do here for now */
  
  
  
 return Ok(());
}

/* http client tcp connected callback */
pub fn httpc_tcp_connected(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, err: err_t) -> Result<(), LwipError>
{
  let r: err_t;
  let req: &mut httpc_state_t = arg;
  
  

  /* send request; last char is zero termination */
  r = altcp_write(req.pcb, req.request.payload, req.request.len - 1, TCP_WRITE_FLAG_COPY);
  if (r != ERR_OK) {
     /* could not write the single small request -> fail, don't retry */
     return httpc_close(req, HTTPC_RESULT_ERR_MEM, 0, r);
  }
  /* everything written, we can free the request */
  pbuf_free(req.request);
  req.request = None;

  altcp_output(req.pcb);
 return Ok(());
}

/* Start the http request when the server IP addr is known */
pub fn httpc_get_internal_addr(req: &mut httpc_state_t,  ipaddr: &mut LwipAddr) -> Result<(), LwipError>
{
  let err: err_t;
  LWIP_ASSERT("req != NULL", req != None);

  if (&req.remote_addr != ipaddr) {
    /* fill in remote addr if called externally */
    req.remote_addr = *ipaddr;
  }

  err = altcp_connect(req.pcb, &req.remote_addr, req.remote_port, httpc_tcp_connected);
  if (err == ERR_OK) {
   return Ok(());
  }
//  LWIP_DEBUGF(HTTPC_DEBUG_WARN_STATE, ("tcp_connect failed: %d\n", err));
  return err;
}


/* DNS callback
 * If ipaddr is non-NULL, resolving succeeded and the request can be sent, otherwise it failed.
 */
pub fn
httpc_dns_found( hostname: &mut String,  ipaddr: &mut LwipAddr, arg: &mut Vec<u8>)
{
  let req: &mut httpc_state_t = arg;
  let err: err_t;
  let result:  httpc_result_t;

  

  if (ipaddr != None) {
    err = httpc_get_internal_addr(req, ipaddr);
    if (err == ERR_OK) {
      return;
    }
    result = HTTPC_RESULT_ERR_CONNECT;
  } else {
/*LWIP_DEBUGF(HTTPC_DEBUG_WARN_STATE, ("httpc_dns_found: failed to resolve hostname: %s\n",
      hostname));*/
    result = HTTPC_RESULT_ERR_HOSTNAME;
    err = ERR_ARG;
  }
  httpc_close(req, result, 0, err);
}


/* Start the http request after converting 'server_name' to ip address (DNS or address string) */
pub fn httpc_get_internal_dns(req: &mut httpc_state_t,  server_name: &mut String) -> Result<(), LwipError>
{
  let err: err_t;
  LWIP_ASSERT("req != NULL", req != None);


  err = dns_gethostbyname(server_name, &req.remote_addr, httpc_dns_found, req);

  // err = ipaddr_aton(server_name, &req.remote_addr) ? ERR_OK : ERR_ARG;


  if (err == ERR_OK) {
    /* cached or IP-string */
    err = httpc_get_internal_addr(req, &req.remote_addr);
  } else if (err == ERR_INPROGRESS) {
   return Ok(());
  }
  return err;
}

pub fn httpc_create_request_string( settings: &mut httpc_connection_t,  server_name: &mut String, server_port: i32,  uri: &mut String,
                            use_host: i32, buffer: &mut String, buffer_size: usize)
{
  if (settings.use_proxy) {
    LWIP_ASSERT("server_name != NULL", server_name != None);
    if (server_port != HTTP_DEFAULT_PORT) {
      return snprintf(buffer, buffer_size, HTTPC_REQ_11_PROXY_PORT_FORMAT(server_name, server_port, uri, server_name));
    } else {
      return snprintf(buffer, buffer_size, HTTPC_REQ_11_PROXY_FORMAT(server_name, uri, server_name));
    }
  } else if (use_host) {
    LWIP_ASSERT("server_name != NULL", server_name != None);
    return snprintf(buffer, buffer_size, HTTPC_REQ_11_HOST_FORMAT(uri, server_name));
  } else {
    return snprintf(buffer, buffer_size, HTTPC_REQ_11_FORMAT(uri));
  }
}

/* Initialize the connection struct */
pub fn httpc_init_connection_common(connection: &mut httpc_state_t,  settings: &mut httpc_connection_t,  server_name: &mut String,
                      server_port: u16,  uri: &mut String, recv_fn: altcp_recv_fn, callback_arg: &mut Vec<u8>, use_host: i32)
{
  let alloc_len: usize;
  let mem_mem_alloc_len: usize;
  // req_len: i32, req_len2;
  let req_len: usize;
  let req_len2: usize;
  httpc_state_t *req;

  let server_name_len: usize;
  let uri_len: usize;


  LWIP_ASSERT("uri != NULL", uri != None);

  /* get request len */
  req_len = httpc_create_request_string(settings, server_name, server_port, uri, use_host, None, 0);
  if ((req_len < 0) || (req_len > 0xFFFF)) {
    return ERR_VAL;
  }
  /* alloc state and request in one block */
  alloc_len = sizeof(httpc_state_t);

  // server_name_len = server_name ? strlen(server_name) : 0;
  uri_len = strlen(uri);
  alloc_len += server_name_len + 1 + uri_len + 1;

  mem_alloc_len = alloc_len;
  if ((mem_alloc_len < alloc_len) || (req_len + 1 > 0xFFFF)) {
    return ERR_VAL;
  }

  req = mem_malloc(alloc_len);
  if(req == None) {
    return ERR_MEM;
  }
  //memset(req, 0, sizeof(httpc_state_t));
  req.timeout_ticks = HTTPC_POLL_TIMEOUT;
  req.request = pbuf_alloc(PBUF_RAW, (req_len + 1), PBUF_RAM);
  if (req.request == None) {
    httpc_free_state(req);
    return ERR_MEM;
  }
  if (req.request.next != None) {
    /* need a pbuf in one piece */
    httpc_free_state(req);
    return ERR_MEM;
  }
  req.hdr_content_len = HTTPC_CONTENT_LEN_INVALID;

  req.server_name = (req + 1);
  if (server_name) {
    memcpy(req.server_name, server_name, server_name_len + 1);
  }
  req.uri = req.server_name + server_name_len + 1;
  memcpy(req.uri, uri, uri_len + 1);

  req.pcb = altcp_new(settings.altcp_allocator);
  if(req.pcb == None) {
    httpc_free_state(req);
    return ERR_MEM;
  }
  // req.remote_port = settings.use_proxy ? settings.proxy_port : server_port;
  altcp_arg(req.pcb, req);
  altcp_recv(req.pcb, httpc_tcp_recv);
  altcp_err(req.pcb, httpc_tcp_err);
  altcp_poll(req.pcb, httpc_tcp_poll, HTTPC_POLL_INTERVAL);
  altcp_sent(req.pcb, httpc_tcp_sent);

  /* set up request buffer */
  req_len2 = httpc_create_request_string(settings, server_name, server_port, uri, use_host,
    req.request.payload, req_len + 1);
  if (req_len2 != req_len) {
    httpc_free_state(req);
    return ERR_VAL;
  }

  req.recv_fn = recv_fn;
  req.conn_settings = settings;
  req.callback_arg = callback_arg;

  *connection = req;
 return Ok(());
}

/*
 * Initialize the connection struct
 */
pub fn httpc_init_connection(connection: &mut httpc_state_t,  settings: &mut httpc_connection_t,  server_name: &mut String,
                      server_port: u16,  uri: &mut String, recv_fn: altcp_recv_fn, callback_arg: &mut Vec<u8>)
{
  return httpc_init_connection_common(connection, settings, server_name, server_port, uri, recv_fn, callback_arg, 1);
}


/*
 * Initialize the connection struct (from IP address)
 */
pub fn httpc_init_connection_addr(connection: &mut httpc_state_t,  settings: &mut httpc_connection_t,
 server_addr: &mut LwipAddr, server_port: u16,  uri: &mut String,
                           recv_fn: altcp_recv_fn, callback_arg: &mut Vec<u8>)
{
  let server_addr_str:  &mut String = ipaddr_ntoa(server_addr);
  if (server_addr_str == None) {
    return ERR_VAL;
  }
  return httpc_init_connection_common(connection, settings, server_addr_str, server_port, uri,
    recv_fn, callback_arg, 1);
}

/*
 * @ingroup httpc 
 * HTTP client API: get a file by passing server IP address
 *
 * @param server_addr IP address of the server to connect
 * @param port tcp port of the server
 * @param uri uri to get from the server, remember leading "/"!
 * @param settings connection settings (callbacks, proxy, etc.)
 * @param recv_fn the http body (not the headers) are passed to this callback
 * @param callback_arg argument passed to all the callbacks
 * @param connection retreives the connection handle (to match in callbacks)
 * @return ERR_OK if starting the request succeeds (callback_fn will be called later)
 *         or an error code
 */
pub fn 
httpc_get_file( server_addr: &mut LwipAddr, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
               recv_fn: altcp_recv_fn, callback_arg: &mut Vec<u8>, connection: &mut httpc_state_t)
{
  let err: err_t;
  let req:  &mut httpc_state_t;

  // LWIP_ERROR("invalid parameters", (server_addr != None) && (uri != None) && (recv_fn != None), return ERR_ARG;);

  err = httpc_init_connection_addr(&req, settings, server_addr, port,
    uri, recv_fn, callback_arg);
  if (err != ERR_OK) {
    return err;
  }

  if (settings.use_proxy) {
    err = httpc_get_internal_addr(req, &settings.proxy_addr);
  } else {
    err = httpc_get_internal_addr(req, server_addr);
  }
  if(err != ERR_OK) {
    httpc_free_state(req);
    return err;
  }

  if (connection != None) {
    *connection = req;
  }
 return Ok(());
}

/*
 * @ingroup httpc 
 * HTTP client API: get a file by passing server name as string (DNS name or IP address string)
 *
 * @param server_name server name as string (DNS name or IP address string)
 * @param port tcp port of the server
 * @param uri uri to get from the server, remember leading "/"!
 * @param settings connection settings (callbacks, proxy, etc.)
 * @param recv_fn the http body (not the headers) are passed to this callback
 * @param callback_arg argument passed to all the callbacks
 * @param connection retreives the connection handle (to match in callbacks)
 * @return ERR_OK if starting the request succeeds (callback_fn will be called later)
 *         or an error code
 */
pub fn 
httpc_get_file_dns( server_name: &mut String, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
                   recv_fn: altcp_recv_fn, callback_arg: &mut Vec<u8>, connection: &mut httpc_state_t)
{
  let err: err_t;
  let req:  &mut httpc_state_t;

  // LWIP_ERROR("invalid parameters", (server_name != None) && (uri != None) && (recv_fn != None), return ERR_ARG;);

  err = httpc_init_connection(&req, settings, server_name, port, uri, recv_fn, callback_arg);
  if (err != ERR_OK) {
    return err;
  }

  if (settings.use_proxy) {
    err = httpc_get_internal_addr(req, &settings.proxy_addr);
  } else {
    err = httpc_get_internal_dns(req, server_name);
  }
  if(err != ERR_OK) {
    httpc_free_state(req);
    return err;
  }

  if (connection != None) {
    *connection = req;
  }
 return Ok(());
}


/* Implementation to disk via fopen/fwrite/fclose follows */

pub struct httpc_filestate_t
{
 pub local_file_name: String,
  pub file: FILE,
  pub settings: httpc_connection_t,
 pub client_settings: httpc_connection_t,
  pub callback_arg: Vec<u8>,
} 

pub fn httpc_fs_result(arg: &mut Vec<u8>, httpc_result: httpc_result_t, rx_content_len: u32,
  srv_res: u32, err: err_t);

/* Initalize http client state for download to file system */
pub fn httpc_fs_init(filestate_out: &mut httpc_filestate_t,  local_file_name: &mut String,
 settings: &mut httpc_connection_t, callback_arg: &mut Vec<u8>)
{
  httpc_filestate_t *filestate;
  let file_len: usize;
  let alloc_len: usize;
  let f:  &mut FILE;

  file_len = strlen(local_file_name);
  alloc_len = sizeof(httpc_filestate_t) + file_len + 1;

  filestate = mem_malloc(alloc_len);
  if (filestate == None) {
    return ERR_MEM;
  }
  //memset(filestate, 0, sizeof(httpc_filestate_t));
  filestate.local_file_name = (filestate + 1);
  memcpy((filestate + 1), local_file_name, file_len + 1);
  filestate.file = None;
  filestate.client_settings = settings;
  filestate.callback_arg = callback_arg;
  /* copy client settings but override result callback */
  memcpy(&filestate.settings, settings, sizeof(httpc_connection_t));
  filestate.settings.result_fn = httpc_fs_result;

  f = fopen(local_file_name, "wb");
  if(f == None) {
    /* could not open file */
    mem_free(filestate);
    return ERR_VAL;
  }
  filestate.file = f;
  *filestate_out = filestate;
 return Ok(());
}

/* Free http client state for download to file system */
pub fn
httpc_fs_free(httpc_filestate_t *filestate)
{
  if (filestate != None) {
    if (filestate.file != None) {
      fclose(filestate.file);
      filestate.file = None;
    }
    mem_free(filestate);
  }
}

/* Connection closed (success or error) */
pub fn
httpc_fs_result(arg: &mut Vec<u8>, httpc_result: httpc_result_t, rx_content_len: u32,
                srv_res: u32, err: err_t)
{
  httpc_filestate_t *filestate = arg;
  if (filestate != None) {
    if (filestate.client_settings.result_fn != None) {
      filestate.client_settings.result_fn(filestate.callback_arg, httpc_result, rx_content_len,
        srv_res, err);
    }
    httpc_fs_free(filestate);
  }
}

/* tcp recv callback */
pub fn httpc_fs_tcp_recv(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, p: &mut pbuf, err: err_t) -> Result<(), LwipError>
{
  httpc_filestate_t *filestate = arg;
  let q: &mut pbuf;
  

  LWIP_ASSERT("p != NULL", p != None);

  for (q = p; q != None; q = q.next) {
    fwrite(q.payload, 1, q.len, filestate.file);
  }
  altcp_recved(pcb, p.tot_len);
  pbuf_free(p);
 return Ok(());
}

/*
 * @ingroup httpc 
 * HTTP client API: get a file to disk by passing server IP address
 *
 * @param server_addr IP address of the server to connect
 * @param port tcp port of the server
 * @param uri uri to get from the server, remember leading "/"!
 * @param settings connection settings (callbacks, proxy, etc.)
 * @param callback_arg argument passed to all the callbacks
 * @param connection retreives the connection handle (to match in callbacks)
 * @return ERR_OK if starting the request succeeds (callback_fn will be called later)
 *         or an error code
 */
pub fn 
httpc_get_file_to_disk( server_addr: &mut LwipAddr, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
                       callback_arg: &mut Vec<u8>,  local_file_name: &mut String, connection: &mut httpc_state_t)
{
  let err: err_t;
  let req:  &mut httpc_state_t;
  httpc_filestate_t *filestate;

  LWIP_ERROR("invalid parameters", (server_addr != None) && (uri != None) && (local_file_name != None), return ERR_ARG;);

  err = httpc_fs_init(&filestate, local_file_name, settings, callback_arg);
  if (err != ERR_OK) {
    return err;
  }

  err = httpc_init_connection_addr(&req, &filestate.settings, server_addr, port,
    uri, httpc_fs_tcp_recv, filestate);
  if (err != ERR_OK) {
    httpc_fs_free(filestate);
    return err;
  }

  if (settings.use_proxy) {
    err = httpc_get_internal_addr(req, &settings.proxy_addr);
  } else {
    err = httpc_get_internal_addr(req, server_addr);
  }
  if(err != ERR_OK) {
    httpc_fs_free(filestate);
    httpc_free_state(req);
    return err;
  }

  if (connection != None) {
    *connection = req;
  }
 return Ok(());
}

/*
 * @ingroup httpc 
 * HTTP client API: get a file to disk by passing server name as string (DNS name or IP address string)
 *
 * @param server_name server name as string (DNS name or IP address string)
 * @param port tcp port of the server
 * @param uri uri to get from the server, remember leading "/"!
 * @param settings connection settings (callbacks, proxy, etc.)
 * @param callback_arg argument passed to all the callbacks
 * @param connection retreives the connection handle (to match in callbacks)
 * @return ERR_OK if starting the request succeeds (callback_fn will be called later)
 *         or an error code
 */
pub fn 
httpc_get_file_dns_to_disk( server_name: &mut String, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
                           callback_arg: &mut Vec<u8>,  local_file_name: &mut String, connection: &mut httpc_state_t)
{
  let err: err_t;
  let req:  &mut httpc_state_t;
  httpc_filestate_t *filestate;

  LWIP_ERROR("invalid parameters", (server_name != None) && (uri != None) && (local_file_name != None), return ERR_ARG;);

  err = httpc_fs_init(&filestate, local_file_name, settings, callback_arg);
  if (err != ERR_OK) {
    return err;
  }

  err = httpc_init_connection(&req, &filestate.settings, server_name, port,
    uri, httpc_fs_tcp_recv, filestate);
  if (err != ERR_OK) {
    httpc_fs_free(filestate);
    return err;
  }

  if (settings.use_proxy) {
    err = httpc_get_internal_addr(req, &settings.proxy_addr);
  } else {
    err = httpc_get_internal_dns(req, server_name);
  }
  if(err != ERR_OK) {
    httpc_fs_free(filestate);
    httpc_free_state(req);
    return err;
  }

  if (connection != None) {
    *connection = req;
  }
 return Ok(());
}



