/*
 * @file
 * LWIP HTTP server implementation
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
 *         Simon Goldschmidt
 *
 */

/*
 * @defgroup httpd HTTP server
 * @ingroup apps
 *
 * This httpd supports for a
 * rudimentary server-side-include facility which will replace tags of the form
 * <!--#tag--> in any file whose extension is .shtml, .shtm or .ssi with
 * strings provided by an include handler whose pointer is provided to the
 * module via function http_set_ssi_handler().
 * Additionally, a simple common
 * gateway interface (CGI) handling mechanism has been added to allow clients
 * to hook functions to particular request URIs.
 *
 * To enable SSI support, define label LWIP_HTTPD_SSI in lwipopts.h.
 * To enable CGI support, define label LWIP_HTTPD_CGI in lwipopts.h.
 *
 * By default, the server assumes that HTTP headers are already present in
 * each file stored in the file system.  By defining LWIP_HTTPD_DYNAMIC_HEADERS in
 * lwipopts.h, this behavior can be changed such that the server inserts the
 * headers automatically based on the extension of the file being served.  If
 * this mode is used, be careful to ensure that the file system image used
 * does not already contain the header information.
 *
 * File system images without headers can be created using the makefsfile
 * tool with the -h command line option.
 *
 *
 * Notes about valid SSI tags
 * --------------------------
 *
 * The following assumptions are made about tags used in SSI markers:
 *
 * 1. No tag may contain '-' or whitespace characters within the tag name.
 * 2. Whitespace is allowed between the tag leadin "<!--#" and the start of
 *    the tag name and between the tag name and the leadout string "-->".
 * 3. The maximum tag name length is LWIP_HTTPD_MAX_TAG_NAME_LEN, currently 8 characters.
 *
 * Notes on CGI usage
 * ------------------
 *
 * The simple CGI support offered here works with GET method requests only
 * and can handle up to 16 parameters encoded into the URI. The handler
 * function may not write directly to the HTTP output but must return a
 * filename that the HTTP server will send to the browser as a response to
 * the incoming CGI request.
 *
 *
 *
 * The list of supported file types is quite short, so if makefsdata complains
 * about an unknown extension, make sure to add it (and its doctype) to
 * the 'g_psHTTPHeaders' list.
 */


























/* Minimum length for a valid HTTP/0.9 request: "GET /\r\n" -> 7 bytes */
pub const MIN_REQ_LEN: u32 = 7; 

#define CRLF "\r\n"

#define HTTP11_CONNECTIONKEEPALIVE  "Connection: keep-alive"
#define HTTP11_CONNECTIONKEEPALIVE2 "Connection: Keep-Alive"



#define HTTP_IS_DYNAMIC_FILE(hs) ((hs).buf != NULL)

#define HTTP_IS_DYNAMIC_FILE(hs) 0


/* This defines checks whether tcp_write has to copy data or not */


/* tcp_write does not have to copy data when sent from rom-file-system directly */
#define HTTP_IS_DATA_VOLATILE(hs)       (HTTP_IS_DYNAMIC_FILE(hs) ? TCP_WRITE_FLAG_COPY : 0)

/* _ => dynamic headers are sent from ROM (non-dynamic headers are handled like file data) */

#define HTTP_IS_HDR_VOLATILE(hs, ptr)   0


/* Return values for http_send_*() */
pub const HTTP_DATA_TO_SEND_FREED: u32 = 3; 
pub const HTTP_DATA_TO_SEND_BREAK: u32 = 2; 
pub const HTTP_DATA_TO_SEND_CONTINUE: u32 = 1; 
pub const HTTP_NO_DATA_TO_SEND: u32 = 0;

typedef struct {
  name: String;
  let shtml: u8;
} default_filename;

static const default_filename httpd_default_filenames[] = {
  {"/index.shtml", 1 },
  {"/index.ssi",   1 },
  {"/index.shtm",  1 },
  {"/index.html",  0 },
  {"/index.htm",   0 }
};

#define NUM_DEFAULT_FILENAMES LWIP_ARRAYSIZE(httpd_default_filenames)


/* HTTP request is copied here from pbufs for simple parsing */
static char httpd_req_buf[LWIP_HTTPD_MAX_REQ_LENGTH + 1];




// #define LWIP_HTTPD_URI_BUF_LEN LWIP_HTTPD_POST_MAX_RESPONSE_URI_LEN



// #define LWIP_HTTPD_URI_BUF_LEN LWIP_HTTPD_MAX_REQUEST_URI_LEN


/* Filename for response file to send when POST is finished or
 * search for default files when a directory is requested. */
static char http_uri_buf[LWIP_HTTPD_URI_BUF_LEN + 1];



/* The number of individual strings that comprise the headers sent before each
 * requested file.
 */
pub const NUM_FILE_HDR_STRINGS: u32 = 5; 
pub const HDR_STRINGS_IDX_HTTP_STATUS: u32 = 0;  /* e.g. "HTTP/1.0 200 OK\r\n" */pub const NUM_FILE_HDR_STRINGS: u32 = 5; pub const NUM_FILE_HDR_STRINGS: u32 = 5; pub const NUM_FILE_HDR_STRINGS: u32 = 5; pub const NUM_FILE_HDR_STRINGS: u32 = 5; 
#define HDR_STRINGS_IDX_SERVER_NAME           1 /* e.g. "Server: "HTTPD_SERVER_AGENT"\r\n" */
#define HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE 2 /* e.g. "Content-Length: xy\r\n" and/or "Connection: keep-alive\r\n" */
#define HDR_STRINGS_IDX_CONTENT_LEN_NR        3 /* the byte count, when content-length is used */
#define HDR_STRINGS_IDX_CONTENT_TYPE          4 /* the content type (or default answer content type including default document) */

/* The dynamically generated Content-Length buffer needs space for CRLF + NULL */
// #define LWIP_HTTPD_MAX_CONTENT_LEN_OFFSET 3

/* The dynamically generated Content-Length buffer shall be able to work with
   !953 MB (9 digits) */
// #define LWIP_HTTPD_MAX_CONTENT_LEN_SIZE   (9 + LWIP_HTTPD_MAX_CONTENT_LEN_OFFSET)





pub const HTTPD_LAST_TAG_PART: u32 = 0xFFFF;

enum tag_check_state {
  TAG_NONE,       /* Not processing an SSI tag */
  TAG_LEADIN,     /* Tag lead in "<!--#" being processed */
  TAG_FOUND,      /* Tag name being read, looking for lead-out start */
  TAG_LEADOUT,    /* Tag lead out "-->" being processed */
  TAG_SENDING     /* Sending tag replacement string */
};

struct http_ssi_state {
  parsed: String;     /* Pointer to the first unparsed byte in buf. */

  tag_started: String;/* Pointer to the first opening '<' of the tag. */

  tag_end: String;    /* Pointer to char after the closing '>' of the tag. */
  let parse_left: u32; /* Number of unparsed bytes in buf. */
  let tag_index: u16;   /* Counter used by tag parsing state machine */  let tag_index: u16;  let tag_index: u16;: u16; /* Length of insert in string tag_insert */

  tag_part: u16; /* Counter passed to and changed by tag insertion function to insert multiple times */

  let tag_type: u8; /* index into http_ssi_tag_desc array */
  let tag_type: u8;
  tag_name_len: u8; /* Length of the tag name in string tag_name */
  char tag_name[LWIP_HTTPD_MAX_TAG_NAME_LEN + 1]; /* Last tag name extracted */
  char tag_insert[LWIP_HTTPD_MAX_TAG_INSERT_LEN + 1]; /* Insert string for tag_name */
  tag_state: tag_check_state; /* State of the tag processor */
};

struct http_ssi_tag_description {
  lead_in: String;
  lead_out: String; 
};



struct http_state {

  next: &mut http_state;

  struct fs_file file_handle;
  handle: &mut fs_file;
  file: String;       /* Pointer to first unsent byte in buf. */

  pcb: &mut altcp_pcb;

  let req: &mut pbuf;



  buf: &mut String;        /* File read buffer. */
  let letbuf_len: i32;      /* Size of file read buffer, buf. */

  let left: u32;       /* Number of unsent bytes in buf. */
  let retries: u8;

  let keepalive: u8;


  ssi: &mut http_ssi_state;


  params: &mut String[LWIP_HTTPD_MAX_CGI_PARAMETERS]; /* Params extracted from the request URI */
  param_vals: &mut String[LWIP_HTTPD_MAX_CGI_PARAMETERS]; /* Values for each extracted param */


  hdrs: &String[NUM_FILE_HDR_STRINGS]; /* HTTP headers to be sent. */
  let hdr_content_len: String;
  let hdr_pos: u16;     /* The position of the first unsent header byte in the
                        current string */
  let hdr_index: u16;   /* The index of the hdr string currently being sent. */


  let time_started: u32;


  let post_content_len_left: u32;

  let unrecved_bytes: u32;
  let no_auto_wnd: u8;
  let post_finished: u8;


};


LWIP_MEMPOOL_DECLARE(HTTPD_STATE,     MEMP_NUM_PARALLEL_HTTPD_CONNS,     sizeof(struct http_state),     "HTTPD_STATE")

LWIP_MEMPOOL_DECLARE(HTTPD_SSI_STATE, MEMP_NUM_PARALLEL_HTTPD_SSI_CONNS, sizeof(struct http_ssi_state), "HTTPD_SSI_STATE")
#define HTTP_FREE_SSI_STATE(x)  LWIP_MEMPOOL_FREE(HTTPD_SSI_STATE, (x))
#define HTTP_ALLOC_SSI_STATE()  (struct http_ssi_state *)LWIP_MEMPOOL_ALLOC(HTTPD_SSI_STATE)

#define HTTP_ALLOC_HTTP_STATE() (struct http_state *)LWIP_MEMPOOL_ALLOC(HTTPD_STATE)
#define HTTP_FREE_HTTP_STATE(x) LWIP_MEMPOOL_FREE(HTTPD_STATE, (x))
 /* HTTPD_USE_MEM_POOL */
#define HTTP_ALLOC_HTTP_STATE() (struct http_state *)mem_malloc(sizeof(struct http_state))
#define HTTP_FREE_HTTP_STATE(x) mem_free(x)

#define HTTP_ALLOC_SSI_STATE()  (struct http_ssi_state *)mem_malloc(sizeof(struct http_ssi_state))
#define HTTP_FREE_SSI_STATE(x)  mem_free(x)



static http_close_conn: err_t(pcb: &mut altcp_pcb, hs: &mut http_state);
static http_close_or_abort_conn: err_t(pcb: &mut altcp_pcb, hs: &mut http_state, abort_conn: u8);
static http_find_file: err_t(hs: &mut http_state, uri: &String, is_09: i32);
static http_init_file: err_t(hs: &mut http_state, file: &mut fs_file, is_09: i32, uri: &String, tag_check: u8, params: &mut String);
static http_poll: err_t(arg: &mut Vec<u8>, pcb: &mut altcp_pcb);
static http_check_eof: u8(pcb: &mut altcp_pcb, hs: &mut http_state);

pub fn http_continue(connection: &mut ());



/* SSI insert handler function pointer. */
static tSSIHandler httpd_ssi_handler;

static httpd_num_tags: i32;
static const char **httpd_tags;


/* Define the available tag lead-ins and corresponding lead-outs.
 * ATTENTION: for the algorithm below using this array, it is essential
 * that the lead in differs in the first character! */
const struct http_ssi_tag_description http_ssi_tag_desc[] = {
  {"<!--#", "-->"},
  {"/*#", "*/"}
};




/* CGI handler information */
static const tCGI *httpd_cgis;
static httpd_num_cgis: i32;
static http_cgi_paramcount: i32;
#define http_cgi_params     hs.params
#define http_cgi_param_vals hs.param_vals
#elif LWIP_HTTPD_CGI_SSI
static http_cgi_params: &mut String[LWIP_HTTPD_MAX_CGI_PARAMETERS]; /* Params extracted from the request URI */
static http_cgi_param_vals: &mut String[LWIP_HTTPD_MAX_CGI_PARAMETERS]; /* Values for each extracted param */



/* global list of active HTTP connections, use to kill the oldest when
    running out of memory */
static http_connections: &mut http_state;

pub fn
http_add_connection(hs: &mut http_state)
{
  /* add the connection to the list */
  hs.next = http_connections;
  http_connections = hs;
}

pub fn
http_remove_connection(hs: &mut http_state)
{
  /* take the connection off the list */
  if (http_connections) {
    if (http_connections == hs) {
      http_connections = hs.next;
    } else {
      last: &mut http_state;
      for (last = http_connections; last.next != NULL; last = last.next) {
        if (last.next == hs) {
          last.next = hs.next;
          break;
        }
      }
    }
  }
}

pub fn
http_kill_oldest_connection(ssi_required: u8)
{
  hs: &mut http_state = http_connections;
  hs_free_next: &mut http_state = NULL;
  while (hs && hs.next) {

    if (ssi_required) {
      if (hs.next.ssi != NULL) {
        hs_free_next = hs;
      }
    } else
 /* LWIP_HTTPD_SSI */
    

    {
      hs_free_next = hs;
    }
    LWIP_ASSERT("broken list", hs != hs.next);
    hs = hs.next;
  }
  if (hs_free_next != NULL) {
    LWIP_ASSERT("hs_free_next.next != NULL", hs_free_next.next != NULL);
    LWIP_ASSERT("hs_free_next.next.pcb != NULL", hs_free_next.next.pcb != NULL);
    /* send RST when killing a connection because of memory shortage */
    http_close_or_abort_conn(hs_free_next.next.pcb, hs_free_next.next, 1); /* this also unlinks the http_state from the list */
  }
}
 /* LWIP_HTTPD_KILL_OLD_ON_CONNECTIONS_EXCEEDED */

#define http_add_connection(hs)
#define http_remove_connection(hs)




/* Allocate as struct http_ssi_state. */
static struct http_ssi_state *
http_ssi_state_alloc()
{
  ret: &mut http_ssi_state = HTTP_ALLOC_SSI_STATE();

  if (ret == NULL) {
    http_kill_oldest_connection(1);
    ret = HTTP_ALLOC_SSI_STATE();
  }

  if (ret != NULL) {
    memset(ret, 0, sizeof(struct http_ssi_state));
  }
  return ret;
}

/* Free a struct http_ssi_state. */
pub fn
http_ssi_state_free(ssi: &mut http_ssi_state)
{
  if (ssi != NULL) {
    HTTP_FREE_SSI_STATE(ssi);
  }
}


/* Initialize a struct http_state.
 */
pub fn
http_state_init(hs: &mut http_state)
{
  /* Initialize the structure. */
  memset(hs, 0, sizeof(struct http_state));

  /* Indicate that the headers are not yet valid */
  hs.hdr_index = NUM_FILE_HDR_STRINGS;

}

/* Allocate a struct http_state. */
static struct http_state *
http_state_alloc()
{
  ret: &mut http_state = HTTP_ALLOC_HTTP_STATE();

  if (ret == NULL) {
    http_kill_oldest_connection(0);
    ret = HTTP_ALLOC_HTTP_STATE();
  }

  if (ret != NULL) {
    http_state_init(ret);
    http_add_connection(ret);
  }
  return ret;
}

/* Free a struct http_state.
 * Also frees the file data if dynamic.
 */
pub fn
http_state_eof(hs: &mut http_state)
{
  if (hs.handle) {

    ms_needed: u32 = sys_now() - hs.time_started;
    needed: u32 = LWIP_MAX(1, (ms_needed / 100));
/*LWIP_DEBUGF(HTTPD_DEBUG_TIMING, ("httpd: needed %"U32_F" ms to send file of %d bytes -> %"U32_F" bytes/sec\n",
                                     ms_needed, hs.handle.len, (((hs.handle.len) * 10) / needed)));*/

    fs_close(hs.handle);
    hs.handle = NULL;
  }

  if (hs.buf != NULL) {
    mem_free(hs.buf);
    hs.buf = NULL;
  }


  if (hs.ssi) {
    http_ssi_state_free(hs.ssi);
    hs.ssi = NULL;
  }


  if (hs.req) {
    pbuf_free(hs.req);
    hs.req = NULL;
  }

}

/* Free a struct http_state.
 * Also frees the file data if dynamic.
 */
pub fn
http_state_free(hs: &mut http_state)
{
  if (hs != NULL) {
    http_state_eof(hs);
    http_remove_connection(hs);
    HTTP_FREE_HTTP_STATE(hs);
  }
}

/* Call tcp_write() in a loop trying smaller and smaller length
 *
 * @param pcb altcp_pcb to send
 * @param ptr Data to send
 * @param length Length of data to send (in/out: on return, contains the
 *        amount of data sent)
 * @param apiflags directly passed to tcp_write
 * @return the return value of tcp_write
 */
pub fn http_write(pcb: &mut altcp_pcb, ptr: &Vec<u8>, length: &mut u16, apiflags: u8) -> Result<(), LwipError>
{
  len: u16, max_len;
  let err: err_t;
  LWIP_ASSERT("length != NULL", length != NULL);
  len = *length;
  if (len == 0) {
    return ERR_OK;
  }
  /* We cannot send more data than space available in the send buffer. */
  max_len = altcp_sndbuf(pcb);
  if (max_len < len) {
    len = max_len;
  }

  /* Additional limitation: e.g. don't enqueue more than 2*mss at once */
  max_len = HTTPD_MAX_WRITE_LEN(pcb);
  if (len > max_len) {
    len = max_len;
  }

  loop {
//    LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Trying to send %d bytes\n", len));
    err = altcp_write(pcb, ptr, len, apiflags);
    if (err == ERR_MEM) {
      if ((altcp_sndbuf(pcb) == 0) ||
          (altcp_sndqueuelen(pcb) >= TCP_SND_QUEUELEN)) {
        /* no need to try smaller sizes */
        len = 1;
      } else {
        len /= 2;
      }
/*LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE,
                  ("Send failed, trying less (%d bytes)\n", len));*/
    }
  } while ((err == ERR_MEM) && (len > 1));

  if (err == ERR_OK) {
//    LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Sent %d bytes\n", len));
    *length = len;
  } else {
//    LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Send failed with err %d (\"%s\")\n", err, lwip_strerr(err)));
    *length = 0;
  }


  /* ensure nagle is normally enabled (only disabled for persistent connections
     when all data has been enqueued but the connection stays open for the next
     request */
  altcp_nagle_enable(pcb);


  return err;
}

/*
 * The connection shall be actively closed (using RST to close from fault states).
 * Reset the sent- and recv-callbacks.
 *
 * @param pcb the tcp pcb to reset callbacks
 * @param hs connection state to free
 */
pub fn http_close_or_abort_conn(pcb: &mut altcp_pcb, hs: &mut http_state, abort_conn: u8) -> Result<(), LwipError>
{
  let err: err_t;
//  LWIP_DEBUGF(HTTPD_DEBUG, ("Closing connection %p\n", pcb));


  if (hs != NULL) {
    if ((hs.post_content_len_left != 0)

        || ((hs.no_auto_wnd != 0) && (hs.unrecved_bytes != 0))

       ) {
      /* make sure the post code knows that the connection is closed */
      http_uri_buf[0] = 0;
      httpd_post_finished(hs, http_uri_buf, LWIP_HTTPD_URI_BUF_LEN);
    }
  }



  altcp_arg(pcb, NULL);
  altcp_recv(pcb, NULL);
  altcp_err(pcb, NULL);
  altcp_poll(pcb, NULL, 0);
  altcp_sent(pcb, NULL);
  if (hs != NULL) {
    http_state_free(hs);
  }

  if (abort_conn) {
    altcp_abort(pcb);
    return ERR_OK;
  }
  err = altcp_close(pcb);
  if (err != ERR_OK) {
//    LWIP_DEBUGF(HTTPD_DEBUG, ("Error %d closing %p\n", err, pcb));
    /* error closing, try again later in poll */
    altcp_poll(pcb, http_poll, HTTPD_POLL_INTERVAL);
  }
  return err;
}

/*
 * The connection shall be actively closed.
 * Reset the sent- and recv-callbacks.
 *
 * @param pcb the tcp pcb to reset callbacks
 * @param hs connection state to free
 */
pub fn http_close_conn(pcb: &mut altcp_pcb, hs: &mut http_state) -> Result<(), LwipError>
{
  return http_close_or_abort_conn(pcb, hs, 0);
}

/* End of file: either close the connection (Connection: close) or
 * close the file (Connection: keep-alive)
 */
pub fn
http_eof(pcb: &mut altcp_pcb, hs: &mut http_state)
{
  /* HTTP/1.1 persistent connection? (Not supported for SSI) */

  if (hs.keepalive) {
    http_remove_connection(hs);

    http_state_eof(hs);
    http_state_init(hs);
    /* restore state: */
    hs.pcb = pcb;
    hs.keepalive = 1;
    http_add_connection(hs);
    /* ensure nagle doesn't interfere with sending all data as fast as possible: */
    altcp_nagle_disable(pcb);
  } else

  {
    http_close_conn(pcb, hs);
  }
}


/*
 * Extract URI parameters from the parameter-part of an URI in the form
 * "test.cgi?x=y" @todo: better explanation!
 * Pointers to the parameters are stored in hs.param_vals.
 *
 * @param hs http connection state
 * @param params pointer to the NULL-terminated parameter string from the URI
 * @return number of parameters extracted
 */
pub fn extract_uri_parameters(hs: &mut http_state, params: &mut String)
{
  pair: &mut String;
  equals: &mut String;
  let letloop: i32;

  

  /* If we have no parameters at all, return immediately. */
  if (!params || (params[0] == '\0')) {
    return (0);
  }

  /* Get a pointer to our first parameter */
  pair = params;

  /* Parse up to LWIP_HTTPD_MAX_CGI_PARAMETERS from the passed string and ignore the
   * remainder (if any) */
  for (loop = 0; (loop < LWIP_HTTPD_MAX_CGI_PARAMETERS) && pair; loop+= 1) {

    /* Save the name of the parameter */
    http_cgi_params[loop] = pair;

    /* Remember the start of this name=value pair */
    equals = pair;

    /* Find the start of the next name=value pair and replace the delimiter
     * with a 0 to terminate the previous pair string. */
    pair = strchr(pair, '&');
    if (pair) {
      *pair = '\0';
      pair+= 1;
    } else {
      /* We didn't find a new parameter so find the end of the URI and
       * replace the space with a '\0' */
      pair = strchr(equals, ' ');
      if (pair) {
        *pair = '\0';
      }

      /* Revert to NULL so that we exit the loop as expected. */
      pair = NULL;
    }

    /* Now find the '=' in the previous pair, replace it with '\0' and save
     * the parameter value string. */
    equals = strchr(equals, '=');
    if (equals) {
      *equals = '\0';
      http_cgi_param_vals[loop] = equals + 1;
    } else {
      http_cgi_param_vals[loop] = NULL;
    }
  }

  return loop;
}



/*
 * Insert a tag (found in an shtml in the form of "<!--#tagname-->" into the file.
 * The tag's name is stored in ssi.tag_name (NULL-terminated), the replacement
 * should be written to hs.tag_insert (up to a length of LWIP_HTTPD_MAX_TAG_INSERT_LEN).
 * The amount of data written is stored to ssi.tag_insert_len.
 *
 * @todo: return tag_insert_len - maybe it can be removed from struct http_state?
 *
 * @param hs http connection state
 */
pub fn
get_tag_insert(hs: &mut http_state)
{

  tag: String;
 /* LWIP_HTTPD_SSI_RAW */
  let lettag: i32;

  let len: usize;
  ssi: &mut http_ssi_state;

  let current_tag_part: u16;


  LWIP_ASSERT("hs != NULL", hs != NULL);
  ssi = hs.ssi;
  LWIP_ASSERT("ssi != NULL", ssi != NULL);

  current_tag_part = ssi.tag_part;
  ssi.tag_part = HTTPD_LAST_TAG_PART;


  tag = ssi.tag_name;


  if (httpd_ssi_handler

      && httpd_tags && httpd_num_tags

     ) {

    /* Find this tag in the list we have been provided. */

    {
 /* LWIP_HTTPD_SSI_RAW */
    for (tag = 0; tag < httpd_num_tags; tag+= 1) {
      if (strcmp(ssi.tag_name, httpd_tags[tag]) == 0)

      {
        ssi.tag_insert_len = httpd_ssi_handler(tag, ssi.tag_insert,
                                              LWIP_HTTPD_MAX_TAG_INSERT_LEN

                                              , current_tag_part, &ssi.tag_part


                                              , (hs.handle ? hs.handle.state : NULL)

                                             );

        if (ssi.tag_insert_len != HTTPD_SSI_TAG_UNKNOWN)

        {
          return;
        }
      }
    }
  }

  /* If we drop out, we were asked to serve a page which contains tags that
   * we don't have a handler for. Merely echo back the tags with an error
   * marker. */
#define UNKNOWN_TAG1_TEXT "<b>***UNKNOWN TAG "
pub const UNKNOWN_TAG1_LEN: u32 = 18; 
#define UNKNOWN_TAG2_TEXT "***</b>"
pub const UNKNOWN_TAG2_LEN: u32 = 7; 
  len = LWIP_MIN(sizeof(ssi.tag_name), LWIP_MIN(strlen(ssi.tag_name),
                 LWIP_HTTPD_MAX_TAG_INSERT_LEN - (UNKNOWN_TAG1_LEN + UNKNOWN_TAG2_LEN)));
  MEMCPY(ssi.tag_insert, UNKNOWN_TAG1_TEXT, UNKNOWN_TAG1_LEN);
  MEMCPY(&ssi.tag_insert[UNKNOWN_TAG1_LEN], ssi.tag_name, len);
  MEMCPY(&ssi.tag_insert[UNKNOWN_TAG1_LEN + len], UNKNOWN_TAG2_TEXT, UNKNOWN_TAG2_LEN);
  ssi.tag_insert[UNKNOWN_TAG1_LEN + len + UNKNOWN_TAG2_LEN] = 0;

  len = strlen(ssi.tag_insert);
  LWIP_ASSERT("len <= 0xffff", len <= 0xffff);
  ssi.tag_insert_len = len;
}



/*
 * Generate the relevant HTTP headers for the given filename and write
 * them into the supplied buffer.
 */
pub fn
get_http_headers(hs: &mut http_state, uri: &String)
{
  let content_type: usize;
  tmp: &mut String;
  ext: &mut String;
  vars: &mut String;

  /* In all cases, the second header we send is the server identification
     so set it here. */
  hs.hdrs[HDR_STRINGS_IDX_SERVER_NAME] = g_psHTTPHeaderStrings[HTTP_HDR_SERVER];
  hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE] = NULL;
  hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_NR] = NULL;

  /* Is this a normal file or the special case we use to send back the
     default "404: Page not found" response? */
  if (uri == NULL) {
    hs.hdrs[HDR_STRINGS_IDX_HTTP_STATUS] = g_psHTTPHeaderStrings[HTTP_HDR_NOT_FOUND];

    if (hs.keepalive) {
      hs.hdrs[HDR_STRINGS_IDX_CONTENT_TYPE] = g_psHTTPHeaderStrings[DEFAULT_404_HTML_PERSISTENT];
    } else

    {
      hs.hdrs[HDR_STRINGS_IDX_CONTENT_TYPE] = g_psHTTPHeaderStrings[DEFAULT_404_HTML];
    }

    /* Set up to send the first header string. */
    hs.hdr_index = 0;
    hs.hdr_pos = 0;
    return;
  }
  /* We are dealing with a particular filename. Look for one other
      special case.  We assume that any filename with "404" in it must be
      indicative of a 404 server error whereas all other files require
      the 200 OK header. */
  if (strstr(uri, "404")) {
    hs.hdrs[HDR_STRINGS_IDX_HTTP_STATUS] = g_psHTTPHeaderStrings[HTTP_HDR_NOT_FOUND];
  } else if (strstr(uri, "400")) {
    hs.hdrs[HDR_STRINGS_IDX_HTTP_STATUS] = g_psHTTPHeaderStrings[HTTP_HDR_BAD_REQUEST];
  } else if (strstr(uri, "501")) {
    hs.hdrs[HDR_STRINGS_IDX_HTTP_STATUS] = g_psHTTPHeaderStrings[HTTP_HDR_NOT_IMPL];
  } else {
    hs.hdrs[HDR_STRINGS_IDX_HTTP_STATUS] = g_psHTTPHeaderStrings[HTTP_HDR_OK];
  }

  /* Determine if the URI has any variables and, if so, temporarily remove
      them. */
  vars = strchr(uri, '?');
  if (vars) {
    *vars = '\0';
  }

  /* Get a pointer to the file extension.  We find this by looking for the
      last occurrence of "." in the filename passed. */
  ext = NULL;
  tmp = strchr(uri, '.');
  while (tmp) {
    ext = tmp + 1;
    tmp = strchr(ext, '.');
  }
  if (ext != NULL) {
    /* Now determine the content type and add the relevant header for that. */
    for (content_type = 0; content_type < NUM_HTTP_HEADERS; content_type+= 1) {
      /* Have we found a matching extension? */
      if (!lwip_stricmp(g_psHTTPHeaders[content_type].extension, ext)) {
        break;
      }
    }
  } else {
    content_type = NUM_HTTP_HEADERS;
  }

  /* Reinstate the parameter marker if there was one in the original URI. */
  if (vars) {
    *vars = '?';
  }


  /* Does the URL passed have any file extension?  If not, we assume it
     is a special-case URL used for control state notification and we do
     not send any HTTP headers with the response. */
  if (!ext) {
    /* Force the header index to a value indicating that all headers
       have already been sent. */
    hs.hdr_index = NUM_FILE_HDR_STRINGS;
    return;
  }

  /* Did we find a matching extension? */
  if (content_type < NUM_HTTP_HEADERS) {
    /* yes, store it */
    hs.hdrs[HDR_STRINGS_IDX_CONTENT_TYPE] = g_psHTTPHeaders[content_type].content_type;
  } else if (!ext) {
    /* no, no extension found -> use binary transfer to prevent the browser adding '.txt' on save */
    hs.hdrs[HDR_STRINGS_IDX_CONTENT_TYPE] = HTTP_HDR_APP;
  } else {
    /* No - use the default, plain text file type. */
    hs.hdrs[HDR_STRINGS_IDX_CONTENT_TYPE] = HTTP_HDR_DEFAULT_TYPE;
  }
  /* Set up to send the first header string. */
  hs.hdr_index = 0;
  hs.hdr_pos = 0;
}

/* Add content-length header? */
pub fn
get_http_content_length(hs: &mut http_state)
{
  add_content_len: u8 = 0;

  LWIP_ASSERT("already been here?", hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE] == NULL);

  add_content_len = 0;

  if (hs.ssi == NULL) /* @todo: get maximum file length from SSI */

  {
    if ((hs.handle != NULL) && (hs.handle.flags & FS_FILE_FLAGS_HEADER_PERSISTENT)) {
      add_content_len = 1;
    }
  }
  if (add_content_len) {
    let len: usize;
    lwip_itoa(hs.hdr_content_len, LWIP_HTTPD_MAX_CONTENT_LEN_SIZE,
              hs.handle.len);
    len = strlen(hs.hdr_content_len);
    if (len <= LWIP_HTTPD_MAX_CONTENT_LEN_SIZE - LWIP_HTTPD_MAX_CONTENT_LEN_OFFSET) {
      SMEMCPY(&hs.hdr_content_len[len], CRLF, 3);
      hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_NR] = hs.hdr_content_len;
    } else {
      add_content_len = 0;
    }
  }

  if (add_content_len) {
    hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE] = g_psHTTPHeaderStrings[HTTP_HDR_KEEPALIVE_LEN];
  } else {
    hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE] = g_psHTTPHeaderStrings[HTTP_HDR_CONN_CLOSE];
    hs.keepalive = 0;
  }
 /* LWIP_HTTPD_SUPPORT_11_KEEPALIVE */
  if (add_content_len) {
    hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE] = g_psHTTPHeaderStrings[HTTP_HDR_CONTENT_LENGTH];
  }

}

/* Sub-function of http_send(): send dynamic headers
 *
 * @returns: - HTTP_NO_DATA_TO_SEND: no new data has been enqueued
 *           - HTTP_DATA_TO_SEND_CONTINUE: continue with sending HTTP body
 *           - HTTP_DATA_TO_SEND_BREAK: data has been enqueued, headers pending,
 *                                      so don't send HTTP body yet
 *           - HTTP_DATA_TO_SEND_FREED: http_state and pcb are already freed
 */
pub fn http_send_headers(pcb: &mut altcp_pcb, hs: &mut http_state)
{
  let err: err_t;
  let len: u16;
  data_to_send: u8 = HTTP_NO_DATA_TO_SEND;
  hdrlen: u16, sendlen;

  if (hs.hdrs[HDR_STRINGS_IDX_CONTENT_LEN_KEEPALIVE] == NULL) {
    /* set up "content-length" and "connection:" headers */
    get_http_content_length(hs);
  }

  /* How much data can we send? */
  len = altcp_sndbuf(pcb);
  sendlen = len;

  while (len && (hs.hdr_index < NUM_FILE_HDR_STRINGS) && sendlen) {
    ptr: &Vec<u8>;
    let old_sendlen: u16;
    let apiflags: u8;
    /* How much do we have to send from the current header? */
    hdrlen = strlen(hs.hdrs[hs.hdr_index]);

    /* How much of this can we send? */
    sendlen = (len < (hdrlen - hs.hdr_pos)) ? len : (hdrlen - hs.hdr_pos);

    /* Send this amount of data or as much as we can given memory
     * constraints. */
    ptr = (hs.hdrs[hs.hdr_index] + hs.hdr_pos);
    old_sendlen = sendlen;
    apiflags = HTTP_IS_HDR_VOLATILE(hs, ptr);
    if (hs.hdr_index == HDR_STRINGS_IDX_CONTENT_LEN_NR) {
      /* content-length is always volatile */
      apiflags |= TCP_WRITE_FLAG_COPY;
    }
    if (hs.hdr_index < NUM_FILE_HDR_STRINGS - 1) {
      apiflags |= TCP_WRITE_FLAG_MORE;
    }
    err = http_write(pcb, ptr, &sendlen, apiflags);
    if ((err == ERR_OK) && (old_sendlen != sendlen)) {
      /* Remember that we added some more data to be transmitted. */
      data_to_send = HTTP_DATA_TO_SEND_CONTINUE;
    } else if (err != ERR_OK) {
      /* special case: http_write does not try to send 1 byte */
      sendlen = 0;
    }

    /* Fix up the header position for the next time round. */
    hs.hdr_pos += sendlen;
    len -= sendlen;

    /* Have we finished sending this string? */
    if (hs.hdr_pos == hdrlen) {
      /* Yes - move on to the next one */
      hs.hdr_index+= 1;
      /* skip headers that are NULL (not all headers are required) */
      while ((hs.hdr_index < NUM_FILE_HDR_STRINGS) &&
             (hs.hdrs[hs.hdr_index] == NULL)) {
        hs.hdr_index+= 1;
      }
      hs.hdr_pos = 0;
    }
  }

  if ((hs.hdr_index >= NUM_FILE_HDR_STRINGS) && (hs.file == NULL)) {
    /* When we are at the end of the headers, check for data to send
     * instead of waiting for ACK from remote side to continue
     * (which would happen when sending files from async read). */
    if (http_check_eof(pcb, hs)) {
      data_to_send = HTTP_DATA_TO_SEND_BREAK;
    } else {
      /* At this point, for non-keepalive connections, hs is deallocated an
         pcb is closed. */
      return HTTP_DATA_TO_SEND_FREED;
    }
  }
  /* If we get here and there are still header bytes to send, we send
   * the header information we just wrote immediately. If there are no
   * more headers to send, but we do have file data to send, drop through
   * to try to send some file data too. */
  if ((hs.hdr_index < NUM_FILE_HDR_STRINGS) || !hs.file) {
//    LWIP_DEBUGF(HTTPD_DEBUG, ("tcp_output\n"));
    return HTTP_DATA_TO_SEND_BREAK;
  }
  return data_to_send;
}


/* Sub-function of http_send(): end-of-file (or block) is reached,
 * either close the file or read the next block (if supported).
 *
 * @returns: 0 if the file is finished or no data has been read
 *           1 if the file is not finished and data has been read
 */
pub fn http_check_eof(pcb: &mut altcp_pcb, hs: &mut http_state)
{
  let letbytes_left: i32;

  let letcount: i32;

  let letmax_write_len: i32;



  /* Do we have a valid file handle? */
  if (hs.handle == NULL) {
    /* No - close the connection. */
    http_eof(pcb, hs);
    return 0;
  }
  bytes_left = fs_bytes_left(hs.handle);
  if (bytes_left <= 0) {
    /* We reached the end of the file so this request is done. */
//    LWIP_DEBUGF(HTTPD_DEBUG, ("End of file.\n"));
    http_eof(pcb, hs);
    return 0;
  }

  /* Do we already have a send buffer allocated? */
  if (hs.buf) {
    /* Yes - get the length of the buffer */
    count = LWIP_MIN(hs.buf_len, bytes_left);
  } else {
    /* We don't have a send buffer so allocate one now */
    count = altcp_sndbuf(pcb);
    if (bytes_left < count) {
      count = bytes_left;
    }

    /* Additional limitation: e.g. don't enqueue more than 2*mss at once */
    max_write_len = HTTPD_MAX_WRITE_LEN(pcb);
    if (count > max_write_len) {
      count = max_write_len;
    }

    loop {
      hs.buf = mem_malloc((mem_usize)count);
      if (hs.buf != NULL) {
        hs.buf_len = count;
        break;
      }
      count = count / 2;
    } while (count > 100);

    /* Did we get a send buffer? If not, return immediately. */
    if (hs.buf == NULL) {
//      LWIP_DEBUGF(HTTPD_DEBUG, ("No buff\n"));
      return 0;
    }
  }

  /* Read a block of data from the file. */
//  LWIP_DEBUGF(HTTPD_DEBUG, ("Trying to read %d bytes.\n", count));


  count = fs_read_async(hs.handle, hs.buf, count, http_continue, hs);
 /* LWIP_HTTPD_FS_ASYNC_READ */
  count = fs_read(hs.handle, hs.buf, count);

  if (count < 0) {
    if (count == FS_READ_DELAYED) {
      /* Delayed read, wait for FS to unblock us */
      return 0;
    }
    /* We reached the end of the file so this request is done.
     * @todo: close here for HTTP/1.1 when reading file fails */
//    LWIP_DEBUGF(HTTPD_DEBUG, ("End of file.\n"));
    http_eof(pcb, hs);
    return 0;
  }

  /* Set up to send the block of data we just read */
//  LWIP_DEBUGF(HTTPD_DEBUG, ("Read %d bytes.\n", count));
  hs.left = count;
  hs.file = hs.buf;

  if (hs.ssi) {
    hs.ssi.parse_left = count;
    hs.ssi.parsed = hs.buf;
  }

 /* LWIP_HTTPD_DYNAMIC_FILE_READ */
  LWIP_ASSERT("SSI and DYNAMIC_HEADERS turned off but eof not reached", 0);

  return 1;
}

/* Sub-function of http_send(): This is the normal send-routine for non-ssi files
 *
 * @returns: - 1: data has been written (so call tcp_ouput)
 *           - 0: no data has been written (no need to call tcp_output)
 */
pub fn http_send_data_nonssi(pcb: &mut altcp_pcb, hs: &mut http_state)
{
  let err: err_t;
  let len: u16;
  data_to_send: u8 = 0;

  /* We are not processing an SHTML file so no tag checking is necessary.
   * Just send the data as we received it from the file. */
  len = LWIP_MIN(hs.left, 0xffff);

  err = http_write(pcb, hs.file, &len, HTTP_IS_DATA_VOLATILE(hs));
  if (err == ERR_OK) {
    data_to_send = 1;
    hs.file += len;
    hs.left -= len;
  }

  return data_to_send;
}


/* Sub-function of http_send(): This is the send-routine for ssi files
 *
 * @returns: - 1: data has been written (so call tcp_ouput)
 *           - 0: no data has been written (no need to call tcp_output)
 */
pub fn http_send_data_ssi(pcb: &mut altcp_pcb, hs: &mut http_state)
{
  err: err_t = ERR_OK;
  let len: u16;
  data_to_send: u8 = 0;
  let tag_type: u8;

  ssi: &mut http_ssi_state = hs.ssi;
  LWIP_ASSERT("ssi != NULL", ssi != NULL);
  /* We are processing an SHTML file so need to scan for tags and replace
   * them with insert strings. We need to be careful here since a tag may
   * straddle the boundary of two blocks read from the file and we may also
   * have to split the insert string between two tcp_write operations. */

  /* How much data could we send? */
  len = altcp_sndbuf(pcb);

  /* Do we have remaining data to send before parsing more? */
  if (ssi.parsed > hs.file) {
    len = LWIP_MIN(ssi.parsed - hs.file, 0xffff);

    err = http_write(pcb, hs.file, &len, HTTP_IS_DATA_VOLATILE(hs));
    if (err == ERR_OK) {
      data_to_send = 1;
      hs.file += len;
      hs.left -= len;
    }

    /* If the send buffer is full, return now. */
    if (altcp_sndbuf(pcb) == 0) {
      return data_to_send;
    }
  }

//  LWIP_DEBUGF(HTTPD_DEBUG, ("State %d, %d left\n", ssi.tag_state, ssi.parse_left));

  /* We have sent all the data that was already parsed so continue parsing
   * the buffer contents looking for SSI tags. */
  while (((ssi.tag_state == TAG_SENDING) || ssi.parse_left) && (err == ERR_OK)) {
    if (len == 0) {
      return data_to_send;
    }
    match (ssi.tag_state) {
      TAG_NONE =>
        /* We are not currently processing an SSI tag so scan for the
         * start of the lead-in marker. */
        for (tag_type = 0; tag_type < LWIP_ARRAYSIZE(http_ssi_tag_desc); tag_type+= 1) {
          if (*ssi.parsed == http_ssi_tag_desc[tag_type].lead_in[0]) {
            /* We found what could be the lead-in for a new tag so change
             * state appropriately. */
            ssi.tag_type = tag_type;
            ssi.tag_state = TAG_LEADIN;
            ssi.tag_index = 1;
  #if !LWIP_HTTPD_SSI_INCLUDE_TAG
            ssi.tag_started = ssi.parsed;
  #endif /* !LWIP_HTTPD_SSI_INCLUDE_TAG */
            break;
          }
        }

        /* Move on to the next character in the buffer */
        ssi.parse_left -= 1;
        ssi.parsed+= 1;
        break;

      TAG_LEADIN =>
        /* We are processing the lead-in marker, looking for the start of
         * the tag name. */

        /* Have we reached the end of the leadin? */
        if (http_ssi_tag_desc[ssi.tag_type].lead_in[ssi.tag_index] == 0) {
          ssi.tag_index = 0;
          ssi.tag_state = TAG_FOUND;
        } else {
          /* Have we found the next character we expect for the tag leadin? */
          if (*ssi.parsed == http_ssi_tag_desc[ssi.tag_type].lead_in[ssi.tag_index]) {
            /* Yes - move to the next one unless we have found the complete
             * leadin, in which case we start looking for the tag itself */
            ssi.tag_index+= 1;
          } else {
            /* We found an unexpected character so this is not a tag. Move
             * back to idle state. */
            ssi.tag_state = TAG_NONE;
          }

          /* Move on to the next character in the buffer */
          ssi.parse_left -= 1;
          ssi.parsed+= 1;
        }
        break;

      TAG_FOUND =>
        /* We are reading the tag name, looking for the start of the
         * lead-out marker and removing any whitespace found. */

        /* Remove leading whitespace between the tag leading and the first
         * tag name character. */
        if ((ssi.tag_index == 0) && ((*ssi.parsed == ' ') ||
                                      (*ssi.parsed == '\t') || (*ssi.parsed == '\n') ||
                                      (*ssi.parsed == '\r'))) {
          /* Move on to the next character in the buffer */
          ssi.parse_left -= 1;
          ssi.parsed+= 1;
          break;
        }

        /* Have we found the end of the tag name? This is signalled by
         * us finding the first leadout character or whitespace */
        if ((*ssi.parsed == http_ssi_tag_desc[ssi.tag_type].lead_out[0]) ||
            (*ssi.parsed == ' ')  || (*ssi.parsed == '\t') ||
            (*ssi.parsed == '\n') || (*ssi.parsed == '\r')) {

          if (ssi.tag_index == 0) {
            /* We read a zero length tag so ignore it. */
            ssi.tag_state = TAG_NONE;
          } else {
            /* We read a non-empty tag so go ahead and look for the
             * leadout string. */
            ssi.tag_state = TAG_LEADOUT;
            LWIP_ASSERT("ssi.tag_index <= 0xff", ssi.tag_index <= 0xff);
            ssi.tag_name_len = ssi.tag_index;
            ssi.tag_name[ssi.tag_index] = '\0';
            if (*ssi.parsed == http_ssi_tag_desc[ssi.tag_type].lead_out[0]) {
              ssi.tag_index = 1;
            } else {
              ssi.tag_index = 0;
            }
          }
        } else {
          /* This character is part of the tag name so save it */
          if (ssi.tag_index < LWIP_HTTPD_MAX_TAG_NAME_LEN) {
            ssi.tag_name[ssi.tag_index+= 1] = *ssi.parsed;
          } else {
            /* The tag was too long so ignore it. */
            ssi.tag_state = TAG_NONE;
          }
        }

        /* Move on to the next character in the buffer */
        ssi.parse_left -= 1;
        ssi.parsed+= 1;

        break;

      /* We are looking for the end of the lead-out marker. */
      TAG_LEADOUT =>
        /* Remove leading whitespace between the tag leading and the first
         * tag leadout character. */
        if ((ssi.tag_index == 0) && ((*ssi.parsed == ' ') ||
                                      (*ssi.parsed == '\t') || (*ssi.parsed == '\n') ||
                                      (*ssi.parsed == '\r'))) {
          /* Move on to the next character in the buffer */
          ssi.parse_left -= 1;
          ssi.parsed+= 1;
          break;
        }

        /* Have we found the next character we expect for the tag leadout? */
        if (*ssi.parsed == http_ssi_tag_desc[ssi.tag_type].lead_out[ssi.tag_index]) {
          /* Yes - move to the next one unless we have found the complete
           * leadout, in which case we need to call the client to process
           * the tag. */

          /* Move on to the next character in the buffer */
          ssi.parse_left -= 1;
          ssi.parsed+= 1;
          ssi.tag_index+= 1;

          if (http_ssi_tag_desc[ssi.tag_type].lead_out[ssi.tag_index] == 0) {
            /* Call the client to ask for the insert string for the
             * tag we just found. */

            ssi.tag_part = 0; /* start with tag part 0 */

            get_tag_insert(hs);

            /* Next time through, we are going to be sending data
             * immediately, either the end of the block we start
             * sending here or the insert string. */
            ssi.tag_index = 0;
            ssi.tag_state = TAG_SENDING;
            ssi.tag_end = ssi.parsed;

            ssi.parsed = ssi.tag_started;


            /* If there is any unsent data in the buffer prior to the
             * tag, we need to send it now. */
            if (ssi.tag_end > hs.file) {
              /* How much of the data can we send? */

              len = LWIP_MIN(ssi.tag_end - hs.file, 0xffff);
 /* LWIP_HTTPD_SSI_INCLUDE_TAG*/
              /* we would include the tag in sending */
              len = LWIP_MIN(ssi.tag_started - hs.file, 0xffff);


              err = http_write(pcb, hs.file, &len, HTTP_IS_DATA_VOLATILE(hs));
              if (err == ERR_OK) {
                data_to_send = 1;

                if (ssi.tag_started <= hs.file) {
                  /* pretend to have sent the tag, too */
                  len += (ssi.tag_end - ssi.tag_started);
                }

                hs.file += len;
                hs.left -= len;
              }
            }
          }
        } else {
          /* We found an unexpected character so this is not a tag. Move
           * back to idle state. */
          ssi.parse_left -= 1;
          ssi.parsed+= 1;
          ssi.tag_state = TAG_NONE;
        }
        break;

      /*
       * We have found a valid tag and are in the process of sending
       * data as a result of that discovery. We send either remaining data
       * from the file prior to the insert poor: i32 the insert string itself.
       */
      TAG_SENDING =>
        /* Do we have any remaining file data to send from the buffer prior
         * to the tag? */
        if (ssi.tag_end > hs.file) {
          /* How much of the data can we send? */

          len = LWIP_MIN(ssi.tag_end - hs.file, 0xffff);
 /* LWIP_HTTPD_SSI_INCLUDE_TAG*/
          LWIP_ASSERT("hs.started >= hs.file", ssi.tag_started >= hs.file);
          /* we would include the tag in sending */
          len = LWIP_MIN(ssi.tag_started - hs.file, 0xffff);

          if (len != 0) {
            err = http_write(pcb, hs.file, &len, HTTP_IS_DATA_VOLATILE(hs));
          } else {
            err = ERR_OK;
          }
          if (err == ERR_OK) {
            data_to_send = 1;

            if (ssi.tag_started <= hs.file) {
              /* pretend to have sent the tag, too */
              len += (ssi.tag_end - ssi.tag_started);
            }

            hs.file += len;
            hs.left -= len;
          }
        } else {

          if (ssi.tag_index >= ssi.tag_insert_len) {
            /* Did the last SSIHandler have more to send? */
            if (ssi.tag_part != HTTPD_LAST_TAG_PART) {
              /* If so, call it again */
              ssi.tag_index = 0;
              get_tag_insert(hs);
            }
          }


          /* Do we still have insert data left to send? */
          if (ssi.tag_index < ssi.tag_insert_len) {
            /* We are sending the insert string itself. How much of the
             * insert can we send? */
            len = (ssi.tag_insert_len - ssi.tag_index);

            /* Note that we set the copy flag here since we only have a
             * single tag insert buffer per connection. If we don't do
             * this, insert corruption can occur if more than one insert
             * is processed before we call tcp_output. */
            err = http_write(pcb, &(ssi.tag_insert[ssi.tag_index]), &len,
                             HTTP_IS_TAG_VOLATILE(hs));
            if (err == ERR_OK) {
              data_to_send = 1;
              ssi.tag_index += len;
              /* Don't return here: keep on sending data */
            }
          } else {

            if (ssi.tag_part == HTTPD_LAST_TAG_PART)

            {
              /* We have sent all the insert data so go back to looking for
               * a new tag. */
//              LWIP_DEBUGF(HTTPD_DEBUG, ("Everything sent.\n"));
              ssi.tag_index = 0;
              ssi.tag_state = TAG_NONE;

              ssi.parsed = ssi.tag_end;

            }
          }
          break;
        _ =>
          break;
        }
    }
  }

  /* If we drop out of the end of the for loop, this implies we must have
   * file data to send so send it now. In TAG_SENDING state, we've already
   * handled this so skip the send if that's the case. */
  if ((ssi.tag_state != TAG_SENDING) && (ssi.parsed > hs.file)) {

    if ((ssi.tag_state != TAG_NONE) && (ssi.tag_started > ssi.tag_end)) {
      /* If we found tag on the edge of the read buffer: just throw away the first part
         (we have copied/saved everything required for parsing on later). */
      len = (ssi.tag_started - hs.file);
      hs.left -= (ssi.parsed - ssi.tag_started);
      ssi.parsed = ssi.tag_started;
      ssi.tag_started = hs.buf;
    } else

    {
      len = LWIP_MIN(ssi.parsed - hs.file, 0xffff);
    }

    err = http_write(pcb, hs.file, &len, HTTP_IS_DATA_VOLATILE(hs));
    if (err == ERR_OK) {
      data_to_send = 1;
      hs.file += len;
      hs.left -= len;
    }
  }
  return data_to_send;
}


/*
 * Try to send more data on this pcb.
 *
 * @param pcb the pcb to send data
 * @param hs connection state
 */
pub fn http_send(pcb: &mut altcp_pcb, hs: &mut http_state)
{
  data_to_send: u8 = HTTP_NO_DATA_TO_SEND;
/*LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("http_send: pcb=%p hs=%p left=%d\n", pcb,
              hs, hs != NULL ? hs.left : 0));*/


  if (hs.unrecved_bytes != 0) {
    return 0;
  }


  /* If we were passed a NULL state structure pointer, ignore the call. */
  if (hs == NULL) {
    return 0;
  }


  /* Check if we are allowed to read from this file.
     (e.g. SSI might want to delay sending until data is available) */
  if (!fs_is_file_ready(hs.handle, http_continue, hs)) {
    return 0;
  }



  /* Do we have any more header data to send for this file? */
  if (hs.hdr_index < NUM_FILE_HDR_STRINGS) {
    data_to_send = http_send_headers(pcb, hs);
    if ((data_to_send == HTTP_DATA_TO_SEND_FREED) ||
        ((data_to_send != HTTP_DATA_TO_SEND_CONTINUE) &&
         (hs.hdr_index < NUM_FILE_HDR_STRINGS))) {
      return data_to_send;
    }
  }


  /* Have we run out of file data to send? If so, we need to read the next
   * block from the file. */
  if (hs.left == 0) {
    if (!http_check_eof(pcb, hs)) {
      return 0;
    }
  }


  if (hs.ssi) {
    data_to_send = http_send_data_ssi(pcb, hs);
  } else

  {
    data_to_send = http_send_data_nonssi(pcb, hs);
  }

  if ((hs.left == 0) && (fs_bytes_left(hs.handle) <= 0)) {
    /* We reached the end of the file so this request is done.
     * This adds the FIN flag right into the last data segment. */
//    LWIP_DEBUGF(HTTPD_DEBUG, ("End of file.\n"));
    http_eof(pcb, hs);
    return 0;
  }
//  LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("send_data end.\n"));
  return data_to_send;
}


/* Initialize a http connection with a file to send for an error message
 *
 * @param hs http connection state
 * @param error_nr HTTP error number
 * @return ERR_OK if file was found and hs has been initialized correctly
 *         another otherwise: err_t
 */
pub fn http_find_error_file(hs: &mut http_state, error_nr: u16) -> Result<(), LwipError>
{
  uri: &String, *uri1, *uri2, *uri3;

  if (error_nr == 501) {
    uri1 = "/501.html";
    uri2 = "/501.htm";
    uri3 = "/501.shtml";
  } else {
    /* 400 (bad request is the default) */
    uri1 = "/400.html";
    uri2 = "/400.htm";
    uri3 = "/400.shtml";
  }
  if (fs_open(&hs.file_handle, uri1) == ERR_OK) {
    uri = uri1;
  } else if (fs_open(&hs.file_handle, uri2) == ERR_OK) {
    uri = uri2;
  } else if (fs_open(&hs.file_handle, uri3) == ERR_OK) {
    uri = uri3;
  } else {
/*LWIP_DEBUGF(HTTPD_DEBUG, ("Error page for error %"U16_F" not found\n",
                              error_nr));*/
    return ERR_ARG;
  }
  return http_init_file(hs, &hs.file_handle, 0, uri, 0, NULL);
}
 /* LWIP_HTTPD_SUPPORT_EXTSTATUS */
#define http_find_error_file(hs, error_nr) ERR_ARG


/*
 * Get the file struct for a 404 error page.
 * Tries some file names and returns NULL if none found.
 *
 * @param uri pointer that receives the actual file name URI
 * @return file struct for the error page or NULL no matching file was found
 */
static struct fs_file *
http_get_404_file(hs: &mut http_state,  char **uri)
{
  let err: err_t;

  *uri = "/404.html";
  err = fs_open(&hs.file_handle, *uri);
  if (err != ERR_OK) {
    /* 404.html doesn't exist. Try 404.htm instead. */
    *uri = "/404.htm";
    err = fs_open(&hs.file_handle, *uri);
    if (err != ERR_OK) {
      /* 404.htm doesn't exist either. Try 404.shtml instead. */
      *uri = "/404.shtml";
      err = fs_open(&hs.file_handle, *uri);
      if (err != ERR_OK) {
        /* 404.htm doesn't exist either. Indicate to the caller that it should
         * send back a default 404 page.
         */
        *uri = NULL;
        return NULL;
      }
    }
  }

  return &hs.file_handle;
}


pub fn http_handle_post_finished(hs: &mut http_state) -> Result<(), LwipError>
{

  /* Prevent multiple calls to httpd_post_finished, since it might have already
     been called before from httpd_post_data_recved(). */
  if (hs.post_finished) {
    return ERR_OK;
  }
  hs.post_finished = 1;

  /* application error or POST finished */
  /* NULL-terminate the buffer */
  http_uri_buf[0] = 0;
  httpd_post_finished(hs, http_uri_buf, LWIP_HTTPD_URI_BUF_LEN);
  return http_find_file(hs, http_uri_buf, 0);
}

/* Pass received POST body data to the application and correctly handle
 * returning a response document or closing the connection.
 * ATTENTION: The application is responsible for the pbuf now, so don't free it!
 *
 * @param hs http connection state
 * @param p pbuf to pass to the application
 * @return ERR_OK if passed successfully, another if: err_t the response file
 *         hasn't been found (after POST finished)
 */
pub fn http_post_rxpbuf(hs: &mut http_state, p: &mut pbuf) -> Result<(), LwipError>
{
  let err: err_t;

  if (p != NULL) {
    /* adjust remaining Content-Length */
    if (hs.post_content_len_left < p.tot_len) {
      hs.post_content_len_left = 0;
    } else {
      hs.post_content_len_left -= p.tot_len;
    }
  }

  /* prevent connection being closed if httpd_post_data_recved() is called nested */
  hs.unrecved_bytes+= 1;

  if (p != NULL) {
    err = httpd_post_receive_data(hs, p);
  } else {
    err = ERR_OK;
  }

  hs.unrecved_bytes -= 1;

  if (err != ERR_OK) {
    /* Ignore remaining content in case of application error */
    hs.post_content_len_left = 0;
  }
  if (hs.post_content_len_left == 0) {

    if (hs.unrecved_bytes != 0) {
      return ERR_OK;
    }

    /* application error or POST finished */
    return http_handle_post_finished(hs);
  }

  return ERR_OK;
}

/* Handle a post request. Called from http_parse_request when method 'POST'
 * is found.
 *
 * @param p The input pbuf (containing the POST header and body).
 * @param hs The http connection state.
 * @param data HTTP request (header and part of body) from input pbuf(s).
 * @param data_len Size of 'data'.
 * @param uri The HTTP URI parsed from input pbuf(s).
 * @param uri_end Pointer to the end of 'uri' (here, the rest of the HTTP
 *                header starts).
 * @return ERR_OK: POST correctly parsed and accepted by the application.
 *         ERR_INPROGRESS: POST not completely parsed (no error yet)
 *         another err_t: Error parsing POST or denied by the application
 */
pub fn http_post_request(inp: &mut pbuf, hs: &mut http_state,
                  data: &mut String, data_len: u16, uri: &mut String, uri_end: &mut String)
{
  let err: err_t;
  /* search for end-of-header (first double-CRLF) */
  crlfcrlf: &mut String = lwip_strnstr(uri_end + 1, CRLF CRLF, data_len - (uri_end + 1 - data));

  if (crlfcrlf != NULL) {
    /* search for "Content-Length: " */
#define HTTP_HDR_CONTENT_LEN                "Content-Length: "
pub const HTTP_HDR_CONTENT_LEN_LEN: u32 = 16; 
pub const HTTP_HDR_CONTENT_LEN_DIGIT_MAX_LEN: u32 = 10; 
    scontent_len: &mut String = lwip_strnstr(uri_end + 1, HTTP_HDR_CONTENT_LEN, crlfcrlf - (uri_end + 1));
    if (scontent_len != NULL) {
      scontent_len_end: &mut String = lwip_strnstr(scontent_len + HTTP_HDR_CONTENT_LEN_LEN, CRLF, HTTP_HDR_CONTENT_LEN_DIGIT_MAX_LEN);
      if (scontent_len_end != NULL) {
        let letcontent_len: i32;
        content_len_num: &mut String = scontent_len + HTTP_HDR_CONTENT_LEN_LEN;
        content_len = atoi(content_len_num);
        if (content_len == 0) {
          /* if atoi returns 0 on error, fix this */
          if ((content_len_num[0] != '0') || (content_len_num[1] != '\r')) {
            content_len = -1;
          }
        }
        if (content_len >= 0) {
          /* adjust length of HTTP header passed to application */
          hdr_start_after_uri: &String = uri_end + 1;
          hdr_len: u16 = LWIP_MIN(data_len, crlfcrlf + 4 - data);
          hdr_data_len: u16 = LWIP_MIN(data_len, crlfcrlf + 4 - hdr_start_after_uri);
          post_auto_wnd: u8 = 1;
          http_uri_buf[0] = 0;
          /* trim http header */
          *crlfcrlf = 0;
          err = httpd_post_begin(hs, uri, hdr_start_after_uri, hdr_data_len, content_len,
                                 http_uri_buf, LWIP_HTTPD_URI_BUF_LEN, &post_auto_wnd);
          if (err == ERR_OK) {
            /* try to pass in data of the first pbuf(s) */
            q: &mut pbuf = inp;
            start_offset: u16 = hdr_len;

            hs.no_auto_wnd = !post_auto_wnd;

            /* set the Content-Length to be received for this POST */
            hs.post_content_len_left = content_len;

            /* get to the pbuf where the body starts */
            while ((q != NULL) && (q.len <= start_offset)) {
              start_offset -= q.len;
              q = q.next;
            }
            if (q != NULL) {
              /* hide the remaining HTTP header */
              pbuf_remove_header(q, start_offset);

              if (!post_auto_wnd) {
                /* already tcp_recved() this data... */
                hs.unrecved_bytes = q.tot_len;
              }

              pbuf_ref(q);
              return http_post_rxpbuf(hs, q);
            } else if (hs.post_content_len_left == 0) {
              q = pbuf_alloc(PBUF_RAW, 0, PBUF_REF);
              return http_post_rxpbuf(hs, q);
            } else {
              return ERR_OK;
            }
          } else {
            /* return file passed from application */
            return http_find_file(hs, http_uri_buf, 0);
          }
        } else {
/*LWIP_DEBUGF(HTTPD_DEBUG, ("POST received invalid Content-Length: %s\n",
                                    content_len_num));*/
          return ERR_ARG;
        }
      }
    }
    /* If we come here, headers are fully received (double-crlf), but Content-Length
       was not included. Since this is currently the only supported method, we have
       to fail in this case! */
//    LWIP_DEBUGF(HTTPD_DEBUG, ("Error when parsing Content-Length\n"));
    return ERR_ARG;
  }
  /* if we come here, the POST is incomplete */

  return ERR_INPROGRESS;
 /* LWIP_HTTPD_SUPPORT_REQUESTLIST */
  return ERR_ARG;

}


/*
 * @ingroup httpd
 * A POST implementation can call this function to update the TCP window.
 * This can be used to throttle data reception (e.g. when received data is
 * programmed to flash and data is received faster than programmed).
 *
 * @param connection A connection handle passed to httpd_post_begin for which
 *        httpd_post_finished has *NOT* been called yet!
 * @param recved_len Length of data received (for window update)
 */
pub fn  httpd_post_data_recved(connection: &mut (), recved_len: u16)
{
  hs: &mut http_state = (struct http_state *)connection;
  if (hs != NULL) {
    if (hs.no_auto_wnd) {
      len: u16 = recved_len;
      if (hs.unrecved_bytes >= recved_len) {
        hs.unrecved_bytes -= recved_len;
      } else {
//        LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_LEVEL_WARNING, ("httpd_post_data_recved: recved_len too big\n"));
        len = hs.unrecved_bytes;
        hs.unrecved_bytes = 0;
      }
      if (hs.pcb != NULL) {
        if (len != 0) {
          altcp_recved(hs.pcb, len);
        }
        if ((hs.post_content_len_left == 0) && (hs.unrecved_bytes == 0)) {
          /* finished handling POST */
          http_handle_post_finished(hs);
          http_send(hs.pcb, hs);
        }
      }
    }
  }
}





/* Try to send more data if file has been blocked before
 * This is a callback function passed to fs_read_async().
 */
pub fn
http_continue(connection: &mut ())
{
  hs: &mut http_state = (struct http_state *)connection;
  LWIP_ASSERT_CORE_LOCKED();
  if (hs && (hs.pcb) && (hs.handle)) {
    LWIP_ASSERT("hs.pcb != NULL", hs.pcb != NULL);
//    LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("httpd_continue: try to send more data\n"));
    if (http_send(hs.pcb, hs)) {
      /* If we wrote anything to be sent, go ahead and send it now. */
//      LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("tcp_output\n"));
      altcp_output(hs.pcb);
    }
  }
}


/*
 * When data has been received in the correct state, try to parse it
 * as a HTTP request.
 *
 * @param inp the received pbuf
 * @param hs the connection state
 * @param pcb the altcp_pcb which received this packet
 * @return ERR_OK if request was OK and hs has been initialized correctly
 *         ERR_INPROGRESS if request was OK so far but not fully received
 *         another otherwise: err_t
 */
pub fn http_parse_request(inp: &mut pbuf, hs: &mut http_state, pcb: &mut altcp_pcb) -> Result<(), LwipError>
{
  data: &mut String;
  crlf: &mut String;
  let data_len: u16;
  p: &mut pbuf = inp;

  let clen: u16;


  let err: err_t;


   /* only used for post */
  LWIP_ASSERT("p != NULL", p != NULL);
  LWIP_ASSERT("hs != NULL", hs != NULL);

  if ((hs.handle != NULL) || (hs.file != NULL)) {
//    LWIP_DEBUGF(HTTPD_DEBUG, ("Received data while sending a file\n"));
    /* already sending a file */
    /* @todo: abort? */
    return ERR_USE;
  }



//  LWIP_DEBUGF(HTTPD_DEBUG, ("Received %"U16_F" bytes\n", p.tot_len));

  /* first check allowed characters in this pbuf? */

  /* enqueue the pbuf */
  if (hs.req == NULL) {
//    LWIP_DEBUGF(HTTPD_DEBUG, ("First pbuf\n"));
    hs.req = p;
  } else {
//    LWIP_DEBUGF(HTTPD_DEBUG, ("pbuf enqueued\n"));
    pbuf_cat(hs.req, p);
  }
  /* increase pbuf ref counter as it is freed when we return but we want to
     keep it on the req list */
  pbuf_ref(p);

  if (hs.req.next != NULL) {
    data_len = LWIP_MIN(hs.req.tot_len, LWIP_HTTPD_MAX_REQ_LENGTH);
    pbuf_copy_partial(hs.req, httpd_req_buf, data_len, 0);
    data = httpd_req_buf;
  } else

  {
    data = p.payload;
    data_len = p.len;
    if (p.len != p.tot_len) {
//      LWIP_DEBUGF(HTTPD_DEBUG, ("Warning: incomplete header due to chained pbufs\n"));
    }
  }

  /* received enough data for minimal request? */
  if (data_len >= MIN_REQ_LEN) {
    /* wait for CRLF before parsing anything */
    crlf = lwip_strnstr(data, CRLF, data_len);
    if (crlf != NULL) {

      is_post: i32 = 0;

      is_09: i32 = 0;
      sp1: &mut String, *sp2;
      left_len: u16, uri_len;
//      LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("CRLF received, parsing request\n"));
      /* parse method */
      if (!strncmp(data, "GET ", 4)) {
        sp1 = data + 3;
        /* received GET request */
//        LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Received GET request\"\n"));

      } else if (!strncmp(data, "POST ", 5)) {
        /* store request type */
        is_post = 1;
        sp1 = data + 4;
        /* received GET request */
//        LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Received POST request\n"));

      } else {
        /* null-terminate the METHOD (pbuf is freed anyway wen returning) */
        data[4] = 0;
        /* unsupported method! */
/*LWIP_DEBUGF(HTTPD_DEBUG, ("Unsupported request method (not implemented): \"%s\"\n",
                                  data));*/
        return http_find_error_file(hs, 501);
      }
      /* if we come here, method is OK, parse URI */
      left_len = (data_len - ((sp1 + 1) - data));
      sp2 = lwip_strnstr(sp1 + 1, " ", left_len);

      if (sp2 == NULL) {
        /* HTTP 0.9: respond with correct protocol version */
        sp2 = lwip_strnstr(sp1 + 1, CRLF, left_len);
        is_09 = 1;

        if (is_post) {
          /* HTTP/0.9 does not support POST */
          // goto badrequest;
        }

      }

      uri_len = (sp2 - (sp1 + 1));
      if ((sp2 != 0) && (sp2 > sp1)) {
        /* wait for CRLFCRLF (indicating end of HTTP headers) before parsing anything */
        if (lwip_strnstr(data, CRLF CRLF, data_len) != NULL) {
          uri: &mut String = sp1 + 1;

          /* This is HTTP/1.0 compatible: for strict 1.1, a connection
             would always be persistent unless "close" was specified. */
          if (!is_09 && (lwip_strnstr(data, HTTP11_CONNECTIONKEEPALIVE, data_len) ||
                         lwip_strnstr(data, HTTP11_CONNECTIONKEEPALIVE2, data_len))) {
            hs.keepalive = 1;
          } else {
            hs.keepalive = 0;
          }

          /* null-terminate the METHOD (pbuf is freed anyway wen returning) */
          *sp1 = 0;
          uri[uri_len] = 0;
/*LWIP_DEBUGF(HTTPD_DEBUG, ("Received \"%s\" request for URI: \"%s\"\n",
                                    data, uri));*/

          if (is_post) {

            q: &mut pbuf = hs.req;
 /* LWIP_HTTPD_SUPPORT_REQUESTLIST */
            q: &mut pbuf = inp;

            err = http_post_request(q, hs, data, data_len, uri, sp2);
            if (err != ERR_OK) {
              /* restore header for next try */
              *sp1 = ' ';
              *sp2 = ' ';
              uri[uri_len] = ' ';
            }
            if (err == ERR_ARG) {
              // goto badrequest;
            }
            return err;
          } else

          {
            return http_find_file(hs, uri, is_09);
          }
        }
      } else {
//        LWIP_DEBUGF(HTTPD_DEBUG, ("invalid URI\n"));
      }
    }
  }


  clen = pbuf_clen(hs.req);
  if ((hs.req.tot_len <= LWIP_HTTPD_REQ_BUFSIZE) &&
      (clen <= LWIP_HTTPD_REQ_QUEUELEN)) {
    /* request not fully received (too short or CRLF is missing) */
    return ERR_INPROGRESS;
  } else

  {

badrequest:

//    LWIP_DEBUGF(HTTPD_DEBUG, ("bad request\n"));
    /* could not parse request */
    return http_find_error_file(hs, 400);
  }
}


/* Check if SSI should be parsed for this file/URL
 * (With LWIP_HTTPD_SSI_BY_FILE_EXTENSION == 2, this function can be
 * overridden by an external implementation.)
 *
 * @return 1 for SSI, 0 for standard files
 */
pub fn http_uri_is_ssi(file: &mut fs_file, uri: &String)
{
  let loop: usize;
  tag_check: u8 = 0;
  if (file != NULL) {
    /* See if we have been asked for an shtml file and, if so,
        enable tag checking. */
    ext: &String = NULL, *sub;
    param: &mut String = strstr(uri, "?");
    if (param != NULL) {
      /* separate uri from parameters for now, set back later */
      *param = 0;
    }
    sub = uri;
    ext = uri;
    for (sub = strstr(sub, "."); sub != NULL; sub = strstr(sub, ".")) {
      ext = sub;
      sub+= 1;
    }
    for (loop = 0; loop < NUM_SHTML_EXTENSIONS; loop+= 1) {
      if (!lwip_stricmp(ext, g_pcSSIExtensions[loop])) {
        tag_check = 1;
        break;
      }
    }
    if (param != NULL) {
      *param = '?';
    }
  }
  return tag_check;
}


/* Try to find the file specified by uri and, if found, initialize hs
 * accordingly.
 *
 * @param hs the connection state
 * @param uri the HTTP header URI
 * @param is_09 1 if the request is HTTP/0.9 (no HTTP headers in response)
 * @return ERR_OK if file was found and hs has been initialized correctly
 *         another otherwise: err_t
 */
pub fn http_find_file(hs: &mut http_state, uri: &String, is_09: i32) -> Result<(), LwipError>
{
  let loop: usize;
  file: &mut fs_file = NULL;
  params: &mut String = NULL;
  let err: err_t;

  let leti: i32;


  const

  /* By default, assume we will not be processing server-side-includes tags */
  tag_check: u8 = 0;

  /* Have we been asked for the default file (in root or a directory) ? */

  uri_len: usize = strlen(uri);
  if ((uri_len > 0) && (uri[uri_len - 1] == '/') &&
      ((uri != http_uri_buf) || (uri_len == 1))) {
    copy_len: usize = LWIP_MIN(sizeof(http_uri_buf) - 1, uri_len - 1);
    if (copy_len > 0) {
      MEMCPY(http_uri_buf, uri, copy_len);
      http_uri_buf[copy_len] = 0;
    }
 /* LWIP_HTTPD_MAX_REQUEST_URI_LEN */
  if ((uri[0] == '/') &&  (uri[1] == 0)) {

    /* Try each of the configured default filenames until we find one
       that exists. */
    for (loop = 0; loop < NUM_DEFAULT_FILENAMES; loop+= 1) {
      file_name: String;

      if (copy_len > 0) {
        len_left: usize = sizeof(http_uri_buf) - copy_len - 1;
        if (len_left > 0) {
          name_len: usize = strlen(httpd_default_filenames[loop].name);
          name_copy_len: usize = LWIP_MIN(len_left, name_len);
          MEMCPY(&http_uri_buf[copy_len], httpd_default_filenames[loop].name, name_copy_len);
          http_uri_buf[copy_len + name_copy_len] = 0;
        }
        file_name = http_uri_buf;
      } else

      {
        file_name = httpd_default_filenames[loop].name;
      }
//      LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Looking for %s...\n", file_name));
      err = fs_open(&hs.file_handle, file_name);
      if (err == ERR_OK) {
        uri = file_name;
        file = &hs.file_handle;
//        LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Opened.\n"));

        tag_check = httpd_default_filenames[loop].shtml;

        break;
      }
    }
  }
  if (file == NULL) {
    /* No - we've been asked for a specific file. */
    /* First, isolate the base URI (without any parameters) */
    params = strchr(uri, '?');
    if (params != NULL) {
      /* URI contains parameters. NULL-terminate the base URI */
      *params = '\0';
      params+= 1;
    }


    http_cgi_paramcount = -1;
    /* Does the base URI we have isolated correspond to a CGI handler? */
    if (httpd_num_cgis && httpd_cgis) {
      for (i = 0; i < httpd_num_cgis; i+= 1) {
        if (strcmp(uri, httpd_cgis[i].pcCGIName) == 0) {
          /*
           * We found a CGI that handles this URI so extract the
           * parameters and call the handler.
           */
          http_cgi_paramcount = extract_uri_parameters(hs, params);
          uri = httpd_cgis[i].pfnCGIHandler(i, http_cgi_paramcount, hs.params,
                                         hs.param_vals);
          break;
        }
      }
    }


//    LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("Opening %s\n", uri));

    err = fs_open(&hs.file_handle, uri);
    if (err == ERR_OK) {
      file = &hs.file_handle;
    } else {
      file = http_get_404_file(hs, &uri);
    }

    if (file != NULL) {
      if (file.flags & FS_FILE_FLAGS_SSI) {
        tag_check = 1;
      } else {

        tag_check = http_uri_is_ssi(file, uri);

      }
    }

  }
  if (file == NULL) {
    /* None of the default filenames exist so send back a 404 page */
    file = http_get_404_file(hs, &uri);
  }
  return http_init_file(hs, file, is_09, uri, tag_check, params);
}

/* Initialize a http connection with a file to send (if found).
 * Called by http_find_file and http_find_error_file.
 *
 * @param hs http connection state
 * @param file file structure to send (or NULL if not found)
 * @param is_09 1 if the request is HTTP/0.9 (no HTTP headers in response)
 * @param uri the HTTP header URI
 * @param tag_check enable SSI tag checking
 * @param params != NULL if URI has parameters (separated by '?')
 * @return ERR_OK if file was found and hs has been initialized correctly
 *         another otherwise: err_t
 */
pub fn http_init_file(hs: &mut http_state, file: &mut fs_file, is_09: i32, uri: &String,
               tag_check: u8, params: &mut String)
{

  

  if (file != NULL) {
    /* file opened, initialise struct http_state */

    /* If dynamic read is disabled, file data must be in one piece and available now */
    LWIP_ASSERT("file.data != NULL", file.data != NULL);



    if (tag_check) {
      ssi: &mut http_ssi_state = http_ssi_state_alloc();
      if (ssi != NULL) {
        ssi.tag_index = 0;
        ssi.tag_state = TAG_NONE;
        ssi.parsed = file.data;
        ssi.parse_left = file.len;
        ssi.tag_end = file.data;
        hs.ssi = ssi;
      }
    }
 /* LWIP_HTTPD_SSI */
    

    hs.handle = file;

    if (params != NULL) {
      /* URI contains parameters, call generic CGI handler */
      let letcount: i32;

      if (http_cgi_paramcount >= 0) {
        count = http_cgi_paramcount;
      } else

      {
        count = extract_uri_parameters(hs, params);
      }
      httpd_cgi_handler(file, uri, count, http_cgi_params, http_cgi_param_vals

                        , file.state

                       );
    }
 /* LWIP_HTTPD_CGI_SSI */
    

    hs.file = file.data;
    LWIP_ASSERT("File length must be positive!", (file.len >= 0));

    if (file.is_custom_file && (file.data == NULL)) {
      /* custom file, need to read data first (via fs_read_custom) */
      hs.left = 0;
    } else

    {
      hs.left = file.len;
    }
    hs.retries = 0;

    hs.time_started = sys_now();


    LWIP_ASSERT("HTTP headers not included in file system",
                (hs.handle.flags & FS_FILE_FLAGS_HEADER_INCLUDED) != 0);


    if (is_09 && ((hs.handle.flags & FS_FILE_FLAGS_HEADER_INCLUDED) != 0)) {
      /* HTTP/0.9 responses are sent without HTTP header,
         search for the end of the header. */
      file_start: &mut String = lwip_strnstr(hs.file, CRLF CRLF, hs.left);
      if (file_start != NULL) {
        diff: i32 = file_start + 4 - hs.file;
        hs.file += diff;
        hs.left -= diff;
      }
    }

  } else {
    hs.handle = NULL;
    hs.file = NULL;
    hs.left = 0;
    hs.retries = 0;
  }

  /* Determine the HTTP headers to send based on the file extension of
   * the requested URI. */
  if ((hs.handle == NULL) || ((hs.handle.flags & FS_FILE_FLAGS_HEADER_INCLUDED) == 0)) {
    get_http_headers(hs, uri);
  }
 /* LWIP_HTTPD_DYNAMIC_HEADERS */
  


  if (hs.keepalive) {

    if (hs.ssi != NULL) {
      hs.keepalive = 0;
    } else

    {
      if ((hs.handle != NULL) &&
          ((hs.handle.flags & (FS_FILE_FLAGS_HEADER_INCLUDED | FS_FILE_FLAGS_HEADER_PERSISTENT)) == FS_FILE_FLAGS_HEADER_INCLUDED)) {
        hs.keepalive = 0;
      }
    }
  }

  return ERR_OK;
}

/*
 * The pcb had an error and is already deallocated.
 * The argument might still be valid (if != NULL).
 */
pub fn
http_err(arg: &mut Vec<u8>, err: err_t)
{
  hs: &mut http_state = (struct http_state *)arg;
  

//  LWIP_DEBUGF(HTTPD_DEBUG, ("http_err: %s", lwip_strerr(err)));

  if (hs != NULL) {
    http_state_free(hs);
  }
}

/*
 * Data has been sent and acknowledged by the remote host.
 * This means that more data can be sent.
 */
pub fn http_sent(arg: &mut Vec<u8>, pcb: &mut altcp_pcb, len: u16) -> Result<(), LwipError>
{
  hs: &mut http_state = (struct http_state *)arg;

//  LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("http_sent %p\n", pcb));

  

  if (hs == NULL) {
    return ERR_OK;
  }

  hs.retries = 0;

  http_send(pcb, hs);

  return ERR_OK;
}

/*
 * The poll function is called every 2nd second.
 * If there has been no data sent (which resets the retries) in 8 seconds, close.
 * If the last portion of a file has not been sent in 2 seconds, close.
 *
 * This could be increased, but we don't want to waste resources for bad connections.
 */
pub fn http_poll(arg: &mut Vec<u8>, pcb: &mut altcp_pcb) -> Result<(), LwipError>
{
  hs: &mut http_state = (struct http_state *)arg;
/*LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("http_poll: pcb=%p hs=%p pcb_state=%s\n",
              pcb, hs, tcp_debug_state_str(altcp_dbg_get_tcp_state(pcb))));*/

  if (hs == NULL) {
    closed: err_t;
    /* arg is null, close. */
//    LWIP_DEBUGF(HTTPD_DEBUG, ("http_poll: arg is NULL, close\n"));
    closed = http_close_conn(pcb, NULL);
    

    if (closed == ERR_MEM) {
      altcp_abort(pcb);
      return ERR_ABRT;
    }

    return ERR_OK;
  } else {
    hs.retries+= 1;
    if (hs.retries == HTTPD_MAX_RETRIES) {
//      LWIP_DEBUGF(HTTPD_DEBUG, ("http_poll: too many retries, close\n"));
      http_close_conn(pcb, hs);
      return ERR_OK;
    }

    /* If this connection has a file open, try to send some more data. If
     * it has not yet received a GET request, don't do this since it will
     * cause the connection to close immediately. */
    if (hs.handle) {
//      LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("http_poll: try to send more data\n"));
      if (http_send(pcb, hs)) {
        /* If we wrote anything to be sent, go ahead and send it now. */
//        LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("tcp_output\n"));
        altcp_output(pcb);
      }
    }
  }

  return ERR_OK;
}

/*
 * Data has been received on this pcb.
 * For HTTP 1.0, this should normally only happen once (if the request fits in one packet).
 */
pub fn http_recv(arg: &mut Vec<u8>, pcb: &mut altcp_pcb, p: &mut pbuf, err: err_t) -> Result<(), LwipError>
{
  hs: &mut http_state = (struct http_state *)arg;
/*LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("http_recv: pcb=%p pbuf=%p err=%s\n", pcb,
              p, lwip_strerr(err)));*/

  if ((err != ERR_OK) || (p == NULL) || (hs == NULL)) {
    /* error or closed by other side? */
    if (p != NULL) {
      /* Inform TCP that we have taken the data. */
      altcp_recved(pcb, p.tot_len);
      pbuf_free(p);
    }
    if (hs == NULL) {
      /* this should not happen, only to be robust */
//      LWIP_DEBUGF(HTTPD_DEBUG, ("Error, http_recv: hs is NULL, close\n"));
    }
    http_close_conn(pcb, hs);
    return ERR_OK;
  }


  if (hs.no_auto_wnd) {
    hs.unrecved_bytes += p.tot_len;
  } else

  {
    /* Inform TCP that we have taken the data. */
    altcp_recved(pcb, p.tot_len);
  }


  if (hs.post_content_len_left > 0) {
    /* reset idle counter when POST data is received */
    hs.retries = 0;
    /* this is data for a POST, pass the complete pbuf to the application */
    http_post_rxpbuf(hs, p);
    /* pbuf is passed to the application, don't free it! */
    if (hs.post_content_len_left == 0) {
      /* all data received, send response or close connection */
      http_send(pcb, hs);
    }
    return ERR_OK;
  } else

  {
    if (hs.handle == NULL) {
      parsed: err_t = http_parse_request(p, hs, pcb);
      LWIP_ASSERT("http_parse_request: unexpected return value", parsed == ERR_OK
                  || parsed == ERR_INPROGRESS || parsed == ERR_ARG || parsed == ERR_USE);

      if (parsed != ERR_INPROGRESS) {
        /* request fully parsed or error */
        if (hs.req != NULL) {
          pbuf_free(hs.req);
          hs.req = NULL;
        }
      }

      pbuf_free(p);
      if (parsed == ERR_OK) {

        if (hs.post_content_len_left == 0)

        {
//          LWIP_DEBUGF(HTTPD_DEBUG | LWIP_DBG_TRACE, ("http_recv: data %p len %"S32_F"\n", hs.file, hs.left));
          http_send(pcb, hs);
        }
      } else if (parsed == ERR_ARG) {
        /* @todo: close on ERR_USE? */
        http_close_conn(pcb, hs);
      }
    } else {
//      LWIP_DEBUGF(HTTPD_DEBUG, ("http_recv: already sending data\n"));
      /* already sending but still receiving data, we might want to RST here? */
      pbuf_free(p);
    }
  }
  return ERR_OK;
}

/*
 * A new incoming connection has been accepted.
 */
pub fn http_accept(arg: &mut Vec<u8>, pcb: &mut altcp_pcb, err: err_t) -> Result<(), LwipError>
{
  hs: &mut http_state;
  
  
//  LWIP_DEBUGF(HTTPD_DEBUG, ("http_accept %p / %p\n", pcb, arg));

  if ((err != ERR_OK) || (pcb == NULL)) {
    return ERR_VAL;
  }

  /* Set priority */
  altcp_setprio(pcb, HTTPD_TCP_PRIO);

  /* Allocate memory for the structure that holds the state of the
     connection - initialized by that function. */
  hs = http_state_alloc();
  if (hs == NULL) {
//    LWIP_DEBUGF(HTTPD_DEBUG, ("http_accept: Out of memory, RST\n"));
    return ERR_MEM;
  }
  hs.pcb = pcb;

  /* Tell TCP that this is the structure we wish to be passed for our
     callbacks. */
  altcp_arg(pcb, hs);

  /* Set up the various callback functions */
  altcp_recv(pcb, http_recv);
  altcp_err(pcb, http_err);
  altcp_poll(pcb, http_poll, HTTPD_POLL_INTERVAL);
  altcp_sent(pcb, http_sent);

  return ERR_OK;
}

pub fn
httpd_init_pcb(pcb: &mut altcp_pcb, port: u16)
{
  let err: err_t;

  if (pcb) {
    altcp_setprio(pcb, HTTPD_TCP_PRIO);
    /* set SOF_REUSEADDR here to explicitly bind httpd to multiple interfaces */
    err = altcp_bind(pcb, IP_ANY_TYPE, port);
     /* in case of LWIP_NOASSERT */
    LWIP_ASSERT("httpd_init: tcp_bind failed", err == ERR_OK);
    pcb = altcp_listen(pcb);
    LWIP_ASSERT("httpd_init: tcp_listen failed", pcb != NULL);
    altcp_accept(pcb, http_accept);
  }
}

/*
 * @ingroup httpd
 * Initialize the httpd: set up a listening PCB and bind it to the defined port
 */
pub fn 
httpd_init()
{
  pcb: &mut altcp_pcb;


  LWIP_MEMPOOL_INIT(HTTPD_STATE);

  LWIP_MEMPOOL_INIT(HTTPD_SSI_STATE);


//  LWIP_DEBUGF(HTTPD_DEBUG, ("httpd_init\n"));

  /* LWIP_ASSERT_CORE_LOCKED(); is checked by tcp_new() */

  pcb = altcp_tcp_new_ip_type(IPADDR_TYPE_ANY);
  LWIP_ASSERT("httpd_init: tcp_new failed", pcb != NULL);
  httpd_init_pcb(pcb, HTTPD_SERVER_PORT);
}


/*
 * @ingroup httpd
 * Initialize the httpd: set up a listening PCB and bind it to the defined port.
 * Also set up TLS connection handling (HTTPS).
 */
pub fn 
httpd_inits(conf: &mut altcp_tls_config)
{

  pcb_tls: &mut altcp_pcb = altcp_tls_new(conf, IPADDR_TYPE_ANY);
  LWIP_ASSERT("httpd_init: altcp_tls_new failed", pcb_tls != NULL);
  httpd_init_pcb(pcb_tls, HTTPD_SERVER_PORT_HTTPS);
 /* LWIP_ALTCP_TLS */
  

}



/*
 * @ingroup httpd
 * Set the SSI handler function.
 *
 * @param ssi_handler the SSI handler function
 * @param tags an array of SSI tag strings to search for in SSI-enabled files
 * @param num_tags number of tags in the 'tags' array
 */
pub fn 
http_set_ssi_handler(tSSIHandler ssi_handler,  char **tags, num_tags: i32)
{
//  LWIP_DEBUGF(HTTPD_DEBUG, ("http_set_ssi_handler\n"));

  LWIP_ASSERT("no ssi_handler given", ssi_handler != NULL);
  httpd_ssi_handler = ssi_handler;


  
  
 /* LWIP_HTTPD_SSI_RAW */
  LWIP_ASSERT("no tags given", tags != NULL);
  LWIP_ASSERT("invalid number of tags", num_tags > 0);

  httpd_tags = tags;
  httpd_num_tags = num_tags;

}



/*
 * @ingroup httpd
 * Set an array of CGI filenames/handler functions
 *
 * @param cgis an array of CGI filenames/handler functions
 * @param num_handlers number of elements in the 'cgis' array
 */
pub fn 
http_set_cgi_handlers(const tCGI *cgis, num_handlers: i32)
{
  LWIP_ASSERT("no cgis given", cgis != NULL);
  LWIP_ASSERT("invalid number of handlers", num_handlers > 0);

  httpd_cgis = cgis;
  httpd_num_cgis = num_handlers;
}



