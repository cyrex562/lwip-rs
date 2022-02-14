/**
 * @file
 * HTTP server options list
 */

/*
 * Copyright (c) 2001-2003 Swedish Institute of Computer Science.
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
 * This version of the file has been modified by Texas Instruments to offer
 * simple server-side-include (SSI) and Common Gateway Interface (CGI)
 * capability.
 */







/**
 * @defgroup httpd_opts Options
 * @ingroup httpd
 * @{
 */

/** Set this to 1 to support CGI (old style).
 *
 * This old style CGI support works by registering an array of URLs and
 * associated CGI handler functions (@ref http_set_cgi_handlers).
 * This list is scanned just before fs_open is called from request handling.
 * The handler can return a new URL that is used internally by the httpd to
 * load the returned page (passed to fs_open).
 *
 * Use this CGI type e.g. to execute specific actions and return a page that
 * does not depend on the CGI parameters.
 */

pub const LWIP_HTTPD_CGI: u32 = 0;


/** Set this to 1 to support CGI (new style).
 *
 * This new style CGI support works by calling a global function
 * (@ref tCGIHandler) for all URLs that are found. fs_open is called first
 * and the URL can not be written by the CGI handler. Instead, this handler gets
 * passed the http file state, an object where it can store information derived
 * from the CGI URL or parameters. This file state is later passed to SSI, so
 * the SSI code can return data depending on CGI input.
 *
 * Use this CGI handler if you want CGI information passed on to SSI.
 */

pub const LWIP_HTTPD_CGI_SSI: u32 = 0;


/** Set this to 1 to support SSI (Server-Side-Includes)
 *
 * In contrast to other http servers, this only calls a preregistered callback
 * function (@see http_set_ssi_handler) for each tag (in the format of
 * <!--#tag-->) encountered in SSI-enabled pages.
 * SSI-enabled pages must have one of the predefined SSI-enabled file extensions.
 * All files with one of these extensions are parsed when sent.
 *
 * A downside of the current SSI implementation is that persistent connections
 * don't work, as the file length is not known in advance (and httpd currently
 * relies on the Content-Length header for persistent connections).
 *
 * To save memory, the maximum tag length is limited (@see LWIP_HTTPD_MAX_TAG_NAME_LEN).
 * To save memory, the maximum insertion string length is limited (@see
 * LWIP_HTTPD_MAX_TAG_INSERT_LEN). If this is not enough, @ref LWIP_HTTPD_SSI_MULTIPART
 * can be used.
 */

pub const LWIP_HTTPD_SSI: u32 = 0;


/** Set this to 1 to implement an SSI tag handler callback that gets a const char*
 * to the tag (instead of an index into a pre-registered array of known tags)
 * If this is 0, the SSI handler callback function is only called pre-registered tags.
 */

pub const LWIP_HTTPD_SSI_RAW: u32 = 0;


/** Set this to 0 to prevent parsing the file extension at runtime to decide
 * if a file should be scanned for SSI tags or not.
 * Default is 1 (file extensions are checked using the g_pcSSIExtensions array)
 * Set to 2 to override this runtime test function. In this case, you have to
 * provide an external function that does the check:
 *   u8_t http_uri_is_ssi(struct fs_file *file, const char *uri)
 *
 * This is enabled by default, but if you only use a newer version of makefsdata
 * supporting the "-ssi" option, this info is already present in
 */

pub const LWIP_HTTPD_SSI_BY_FILE_EXTENSION: u32 = 1;


/** This is a list of file extensions handled as SSI files. This define
 * is used to initialize a 'const char *const[]'. It is only used if
 * LWIP_HTTPD_SSI_BY_FILE_EXTENSION != 0.
 */

#define LWIP_HTTPD_SSI_EXTENSIONS ".shtml", ".shtm", ".ssi", ".xml", ".json"


/** Set this to 1 to support HTTP POST */

pub const LWIP_HTTPD_SUPPORT_POST: u32 = 0;


/* The maximum number of parameters that the CGI handler can be sent. */

pub const LWIP_HTTPD_MAX_CGI_PARAMETERS: u32 = 16;


/** LWIP_HTTPD_SSI_MULTIPART==1: SSI handler function is called with 2 more
 * arguments indicating a counter for insert string that are too long to be
 * inserted at once: the SSI handler function must then set 'next_tag_part'
 * which will be passed back to it in the next call. */

pub const LWIP_HTTPD_SSI_MULTIPART: u32 = 0;


/* The maximum length of the string comprising the SSI tag name
 * ATTENTION: tags longer than this are ignored, not truncated!
 */

pub const LWIP_HTTPD_MAX_TAG_NAME_LEN: u32 = 8;


/* The maximum length of string that can be returned to replace any given tag
 * If this buffer is not long enough, use LWIP_HTTPD_SSI_MULTIPART.
 */

pub const LWIP_HTTPD_MAX_TAG_INSERT_LEN: u32 = 192;



pub const LWIP_HTTPD_POST_MANUAL_WND: u32 = 0;


/** This string is passed in the HTTP header as "Server: " */

#define HTTPD_SERVER_AGENT "lwIP/" LWIP_VERSION_STRING " (http://savannah.nongnu.org/projects/lwip)"


/** Set this to 1 if you want to include code that creates HTTP headers
 * at runtime. Default is off: HTTP headers are then created statically
 * by the makefsdata tool. Static headers mean smaller code size, but
 * the (readonly) fsdata will grow a bit as every file includes the HTTP
 * header. */

pub const LWIP_HTTPD_DYNAMIC_HEADERS: u32 = 0;



pub const HTTPD_DEBUG: u32 = LWIP_DBG_OFF;


/** Set this to 1 to use a memp pool for allocating
 * struct http_state instead of the heap.
 * If enabled, you'll need to define MEMP_NUM_PARALLEL_HTTPD_CONNS
 * (and MEMP_NUM_PARALLEL_HTTPD_SSI_CONNS for SSI) to set the size of
 * the pool(s).
 */

pub const HTTPD_USE_MEM_POOL: u32 = 0;


/** The server port for HTTPD to use */

pub const HTTPD_SERVER_PORT: u32 = LWIP_IANA_PORT_HTTP;


/** The https server port for HTTPD to use */

pub const HTTPD_SERVER_PORT_HTTPS: u32 = LWIP_IANA_PORT_HTTPS;


/** Enable https support? */

pub const HTTPD_ENABLE_HTTPS: u32 = 0;


/** Maximum retries before the connection is aborted/closed.
 * - number of times pcb->poll is called -> default is 4*500ms = 2s;
 * - reset when pcb->sent is called
 */

pub const HTTPD_MAX_RETRIES: u32 = 4;


/** The poll delay is X*500ms */

pub const HTTPD_POLL_INTERVAL: u32 = 4;


/** Priority for tcp pcbs created by HTTPD (very low by default).
 *  Lower priorities get killed first when running out of memory.
 */

pub const HTTPD_TCP_PRIO: u32 = TCP_PRIO_MIN;


/** Set this to 1 to enable timing each file sent */

pub const LWIP_HTTPD_TIMING: u32 = 0;

/** Set this to 1 to enable timing each file sent */

pub const HTTPD_DEBUG_TIMING: u32 = LWIP_DBG_OFF;


/** Set this to one to show error pages when parsing a request fails instead
    of simply closing the connection. */

pub const LWIP_HTTPD_SUPPORT_EXTSTATUS: u32 = 0;


/** Set this to 0 to drop support for HTTP/0.9 clients (to save some bytes) */

pub const LWIP_HTTPD_SUPPORT_V09: u32 = 1;


/** Set this to 1 to enable HTTP/1.1 persistent connections.
 * ATTENTION: If the generated file system includes HTTP headers, these must
 * include the "Connection: keep-alive" header (pass argument "-11" to makefsdata).
 */

pub const LWIP_HTTPD_SUPPORT_11_KEEPALIVE: u32 = 0;


/** Set this to 1 to support HTTP request coming in in multiple packets/pbufs */

pub const LWIP_HTTPD_SUPPORT_REQUESTLIST: u32 = 1;



/** Number of rx pbufs to enqueue to parse an incoming request (up to the first
    newline) */

pub const LWIP_HTTPD_REQ_QUEUELEN: u32 = 5;


/** Number of (TCP payload-) bytes (in pbufs) to enqueue to parse and incoming
    request (up to the first double-newline) */

pub const LWIP_HTTPD_REQ_BUFSIZE: u32 = LWIP_HTTPD_MAX_REQ_LENGTH;


/** Defines the maximum length of a HTTP request line (up to the first CRLF,
    copied from pbuf into this a global buffer when pbuf- or packet-queues
    are received - otherwise the input pbuf is used directly) */

pub const LWIP_HTTPD_MAX_REQ_LENGTH: u32 = LWIP_MIN;(1023, (LWIP_HTTPD_REQ_QUEUELEN * PBUF_POOL_BUFSIZE))

 /* LWIP_HTTPD_SUPPORT_REQUESTLIST */

/** This is the size of a static buffer used when URIs end with '/'.
 * In this buffer, the directory requested is concatenated with all the
 * configured default file names.
 * Set to 0 to disable checking default filenames on non-root directories.
 */

pub const LWIP_HTTPD_MAX_REQUEST_URI_LEN: u32 = 63;


/** Maximum length of the filename to send as response to a POST request,
 * filled in by the application when a POST is finished.
 */

pub const LWIP_HTTPD_POST_MAX_RESPONSE_URI_LEN: u32 = 63;


/** Set this to 0 to not send the SSI tag (default is on, so the tag will
 * be sent in the HTML page */

pub const LWIP_HTTPD_SSI_INCLUDE_TAG: u32 = 1;


/** Set this to 1 to call tcp_abort when tcp_close fails with memory error.
 * This can be used to prevent consuming all memory in situations where the
 * HTTP server has low priority compared to other communication. */

pub const LWIP_HTTPD_ABORT_ON_CLOSE_MEM_ERROR: u32 = 0;


/** Set this to 1 to kill the oldest connection when running out of
 * memory for 'struct http_state' or 'struct http_ssi_state'.
 * ATTENTION: This puts all connections on a linked list, so may be kind of slow.
 */

pub const LWIP_HTTPD_KILL_OLD_ON_CONNECTIONS_EXCEEDED: u32 = 0;


/** Set this to 1 to send URIs without extension without headers
 * (who uses this at all??) */

pub const LWIP_HTTPD_OMIT_HEADER_FOR_EXTENSIONLESS_URI: u32 = 0;


/** Default: Tags are sent from struct http_state and are therefore volatile */

#define HTTP_IS_TAG_VOLATILE(ptr) TCP_WRITE_FLAG_COPY


/* By default, the httpd is limited to send 2*pcb->mss to keep resource usage low
   when http is not an important protocol in the device. */

pub const HTTPD_LIMIT_SENDING_TO_2MSS: u32 = 1;


/* Define this to a function that returns the maximum amount of data to enqueue.
   The function have this signature: u16_t fn(struct altcp_pcb* pcb);
   The best place to define this is the hooks file (@see LWIP_HOOK_FILENAME) */


#define HTTPD_MAX_WRITE_LEN(pcb)    ((u16_t)(2 * altcp_mss(pcb)))



/*------------------- FS OPTIONS -------------------*/

/** Set this to 1 and provide the functions:
 * - "int fs_open_custom(struct fs_file *file, const char *name)"
 *    Called first for every opened file to allow opening files
 *    that are not included in fsdata(_custom).c
 * - "void fs_close_custom(struct fs_file *file)"
 *    Called to free resources allocated by fs_open_custom().
 */

pub const LWIP_HTTPD_CUSTOM_FILES: u32 = 0;


/** Set this to 1 to support fs_read() to dynamically read file data.
 * Without this (default=off), only one-block files are supported,
 * and the contents must be ready after fs_open().
 */

pub const LWIP_HTTPD_DYNAMIC_FILE_READ: u32 = 0;


/** Set this to 1 to include an application state argument per file
 * that is opened. This allows to keep a state per connection/file.
 */

pub const LWIP_HTTPD_FILE_STATE: u32 = 0;


/** Set this to 1 to add the pextension field to the fs_file structure.
 * This is included here to retain compatibility with legacy code that
 * relies on the presence of the pextension field.
 * New code should use LWIP_HTTPD_FILE_STATE instead.
 * This option may be removed in a future version of lwip.
 */

pub const LWIP_HTTPD_FILE_EXTENSION: u32 = 0;


/** HTTPD_PRECALCULATED_CHECKSUM==1: include precompiled checksums for
 * predefined (MSS-sized) chunks of the files to prevent having to calculate
 * the checksums at runtime. */

pub const HTTPD_PRECALCULATED_CHECKSUM: u32 = 0;


/** LWIP_HTTPD_FS_ASYNC_READ==1: support asynchronous read operations
 * (fs_read_async returns FS_READ_DELAYED and calls a callback when finished).
 */

pub const LWIP_HTTPD_FS_ASYNC_READ: u32 = 0;


/** Filename (including path) to use as FS data file */

/* HTTPD_USE_CUSTOM_FSDATA: Compatibility with deprecated lwIP option */

#define HTTPD_FSDATA_FILE "fsdata_custom.c"

#define HTTPD_FSDATA_FILE "fsdata.c"



/**
 * @}
 */

 /* LWIP_HDR_APPS_HTTPD_OPTS_H */
