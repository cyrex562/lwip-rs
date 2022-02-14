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
 *
 */

/*
 * @ingroup httpc
 * HTTPC_HAVE_FILE_IO: define this to 1 to have functions dowloading directly
 * to disk via fopen/fwrite.
 * These functions are example implementations of the interface only.
 */

pub const LWIP_HTTPC_HAVE_FILE_IO: u32 = 0;

/*
 * @ingroup httpc
 * The default TCP port used for HTTP
 */
pub const HTTP_DEFAULT_PORT: u16 = LWIP_IANA_PORT_HTTP;

/*
 * @ingroup httpc
 * HTTP client result codes
 */
pub enum httpc_result_t {
    //  File successfully received 
    HTTPC_RESULT_OK = 0,
    //  Unknown error 
    HTTPC_RESULT_ERR_UNKNOWN = 1,
    //  Connection to server failed 
    HTTPC_RESULT_ERR_CONNECT = 2,
    //  Failed to resolve server hostname 
    HTTPC_RESULT_ERR_HOSTNAME = 3,
    //  Connection unexpectedly closed by remote server 
    HTTPC_RESULT_ERR_CLOSED = 4,
    //  Connection timed out (server didn't respond in time) 
    HTTPC_RESULT_ERR_TIMEOUT = 5,
    //  Server responded with an error code 
    HTTPC_RESULT_ERR_SVR_RESP = 6,
    //  Local memory error 
    HTTPC_RESULT_ERR_MEM = 7,
    //  Local abort 
    HTTPC_RESULT_LOCAL_ABORT = 8,
    //  Content length mismatch 
    HTTPC_RESULT_ERR_CONTENT_LEN = 9,
}

// typedef struct _httpc_state httpc_state_t;

/*
 * @ingroup httpc
 * Prototype of a http client callback function
 *
 * @param arg argument specified when initiating the request
 * @param httpc_result result of the http transfer (see enum httpc_result_t)
 * @param rx_content_len number of bytes received (without headers)
 * @param srv_res this contains the http status code received (if any)
 * @param err an error returned by internal lwip functions, can help to specify
 *            the source of the error but must not necessarily be != ERR_OK
 */
type httpc_result_fn = fn(
    arg: &mut Vec<u8>,
    httpc_result: httpc_result_t,
    rx_content_len: u32,
    srv_res: u32,
    err: err_t,
);

/*
 * @ingroup httpc
 * Prototype of http client callback: called when the headers are received
 *
 * @param connection http client connection
 * @param arg argument specified when initiating the request
 * @param hdr header pbuf(s) (may contain data also)
 * @param hdr_len length of the heders in 'hdr'
 * @param content_len content length as received in the headers (-1 if not received)
 * @return if != ERR_OK is returned, the connection is aborted
 */
type httpc_headers_done_fn = fn(
    connection: httpc_state_t,
    arg: &mut Vec<u8>,
    hdr: &mut PacketBuffer,
    hdr_len: u16,
    content_len: u32,
) -> Result<(), LwipError>;

pub struct httpc_connection_t {
    pub proxy_addr: LwipAddr,
    pub proxy_port: u16,
    pub use_proxy: u8,
    //  @todo: add username:pass? 
    pub altcp_allocator: altcp_allocator_t,
    //  this callback is called when the transfer is finished (or aborted) 
    pub result_fn: httpc_result_fn,
    /* this callback is called after receiving the http headers
    It can abort the connection by returning != ERR_OK */
    pub headers_done_fn: httpc_headers_done_fn,
}

// pub fn  httpc_get_file( server_addr: &mut LwipAddr, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
//                      recv_fn: altcp_recv_fn callback_arg: &mut Vec<u8>, connection: &mut httpc_state_t);
// pub fn  httpc_get_file_dns( server_name: &mut String, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
//                      recv_fn: altcp_recv_fn callback_arg: &mut Vec<u8>, connection: &mut httpc_state_t);

// pub fn  httpc_get_file_to_disk( server_addr: &mut LwipAddr, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
//                      callback_arg: &mut Vec<u8>,  local_file_name: &mut String, connection: &mut httpc_state_t);
// pub fn  httpc_get_file_dns_to_disk( server_name: &mut String, port: u16,  uri: &mut String,  settings: &mut httpc_connection_t,
//                      callback_arg: &mut Vec<u8>,  local_file_name: &mut String, connection: &mut httpc_state_t);
