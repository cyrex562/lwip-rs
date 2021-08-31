/*
 * @file
 * SMTP client module
 * 
 * Author: Simon Goldschmidt
 *
 * @defgroup smtp SMTP client
 * @ingroup apps
 * 
 * This is simple SMTP client for raw API.
 * It is a minimal implementation of SMTP as specified in RFC 5321.
 *
 * Example usage:
@code{.c}
 void my_smtp_result_fn(arg: &mut Vec<u8>, smtp_result: u8, srv_err: u16, err: err_t)
 {
   printf("mail (%p) sent with results: 0x%02x, 0x%04x, 0x%08x\n", arg,
          smtp_result, srv_err, err);
 }
 pub fn my_smtp_test()
 {
   smtp_set_server_addr("mymailserver.org");
   -> set both username and password as NULL if no auth needed
   smtp_set_auth("username", "password");
   smtp_send_mail("sender", "recipient", "subject", "body", my_smtp_result_fn,
                  some_argument);
 }
@endcode

 * When using from any other thread than the tcpip_thread (for NO_SYS==0), use
 * smtp_send_mail_int()!
 * 
 * SMTP_BODYDH usage:
@code{.c}
 my_smtp_bodydh_fn: i32(arg: &mut Vec<u8>, bdh: &mut smtp_bodydh)
 {
    if(bdh.state >= 10) {
       return BDH_DONE;
    }
    sprintf(bdh.buffer,"Line #%2d\r\n",bdh.state);
    bdh.length = strlen(bdh.buffer);
    += 1bdh.state;
    return BDH_WORKING;
 }
 
 smtp_send_mail_bodycback("sender", "recipient", "subject", 
                my_smtp_bodydh_fn, my_smtp_result_fn, some_argument);
@endcode
 * 
 * @todo:
 * - attachments (the main difficulty here is streaming base64-encoding to
 *   prevent having to allocate a buffer for the whole encoded file at once)
 * - test with more mail servers...
 *
 */















/* TCP poll interval. Unit is 0.5 sec. */
pub const SMTP_POLL_INTERVAL: u32 = 4; 
/* TCP poll timeout while sending message body, reset after every
 * successful write. 3 minutes */
#define SMTP_TIMEOUT_DATABLOCK  ( 3 * 60 * SMTP_POLL_INTERVAL / 2)
/* TCP poll timeout while waiting for confirmation after sending the body.
 * 10 minutes */
#define SMTP_TIMEOUT_DATATERM   (10 * 60 * SMTP_POLL_INTERVAL / 2)
/* TCP poll timeout while not sending the body.
 * This is somewhat lower than the RFC states (5 minutes for initial, MAIL
 * and RCPT) but still OK for us here.
 * 2 minutes */
#define SMTP_TIMEOUT            ( 2 * 60 * SMTP_POLL_INTERVAL / 2)

/* the various debug levels for this file */
#define SMTP_DEBUG_TRACE        (SMTP_DEBUG | LWIP_DBG_TRACE)
#define SMTP_DEBUG_STATE        (SMTP_DEBUG | LWIP_DBG_STATE)
#define SMTP_DEBUG_WARN         (SMTP_DEBUG | LWIP_DBG_LEVEL_WARNING)
#define SMTP_DEBUG_WARN_STATE   (SMTP_DEBUG | LWIP_DBG_LEVEL_WARNING | LWIP_DBG_STATE)
#define SMTP_DEBUG_SERIOUS      (SMTP_DEBUG | LWIP_DBG_LEVEL_SERIOUS)


pub const SMTP_RX_BUF_LEN: u32 = 255; 
pub const SMTP_TX_BUF_LEN: u32 = 255; 
#define SMTP_CRLF               "\r\n"
pub const SMTP_CRLF_LEN: u32 = 2; 

#define SMTP_RESP_220           "220"
#define SMTP_RESP_235           "235"
#define SMTP_RESP_250           "250"
#define SMTP_RESP_334           "334"
#define SMTP_RESP_354           "354"
#define SMTP_RESP_LOGIN_UNAME   "VXNlcm5hbWU6"
#define SMTP_RESP_LOGIN_PASS    "UGFzc3dvcmQ6"

#define SMTP_KEYWORD_AUTH_SP    "AUTH "
#define SMTP_KEYWORD_AUTH_EQ    "AUTH="
pub const SMTP_KEYWORD_AUTH_LEN: u32 = 5; 
#define SMTP_AUTH_PARAM_PLAIN   "PLAIN"
#define SMTP_AUTH_PARAM_LOGIN   "LOGIN"

#define SMTP_CMD_EHLO_1           "EHLO ["
pub const SMTP_CMD_EHLO_1_LEN: u32 = 6; 
#define SMTP_CMD_EHLO_2           "]\r\n"
pub const SMTP_CMD_EHLO_2_LEN: u32 = 3; 
#define SMTP_CMD_AUTHPLAIN_1      "AUTH PLAIN "
pub const SMTP_CMD_AUTHPLAIN_1_LEN: u32 = 11; 
#define SMTP_CMD_AUTHPLAIN_2      "\r\n"
pub const SMTP_CMD_AUTHPLAIN_2_LEN: u32 = 2; 
#define SMTP_CMD_AUTHLOGIN        "AUTH LOGIN\r\n"
pub const SMTP_CMD_AUTHLOGIN_LEN: u32 = 12; 
#define SMTP_CMD_MAIL_1           "MAIL FROM: <"
pub const SMTP_CMD_MAIL_1_LEN: u32 = 12; 
#define SMTP_CMD_MAIL_2           ">\r\n"
pub const SMTP_CMD_MAIL_2_LEN: u32 = 3; 
#define SMTP_CMD_RCPT_1           "RCPT TO: <"
pub const SMTP_CMD_RCPT_1_LEN: u32 = 10; 
#define SMTP_CMD_RCPT_2           ">\r\n"
pub const SMTP_CMD_RCPT_2_LEN: u32 = 3; 
#define SMTP_CMD_DATA             "DATA\r\n"
pub const SMTP_CMD_DATA_LEN: u32 = 6; 
#define SMTP_CMD_HEADER_1         "From: <"
pub const SMTP_CMD_HEADER_1_LEN: u32 = 7; 
#define SMTP_CMD_HEADER_2         ">\r\nTo: <"
pub const SMTP_CMD_HEADER_2_LEN: u32 = 8; 
#define SMTP_CMD_HEADER_3         ">\r\nSubject: "
pub const SMTP_CMD_HEADER_3_LEN: u32 = 12; 
#define SMTP_CMD_HEADER_4         "\r\n\r\n"
pub const SMTP_CMD_HEADER_4_LEN: u32 = 4; 
#define SMTP_CMD_BODY_FINISHED    "\r\n.\r\n"
pub const SMTP_CMD_BODY_FINISHED_LEN: u32 = 5; 
#define SMTP_CMD_QUIT             "QUIT\r\n"
pub const SMTP_CMD_QUIT_LEN: u32 = 6; 


#define SMTP_TX_BUF_MAX(len) LWIP_MACRO(if((len) > smtp_tx_buf_len_max) smtp_tx_buf_len_max = (len);)
 /* SMTP_STAT_TX_BUF_MAX */
#define SMTP_TX_BUF_MAX(len)



#define SMTP_USERNAME(session)        (session).username
#define SMTP_PASS(session)            (session).pass
#define SMTP_AUTH_PLAIN_DATA(session) (session).auth_plain
#define SMTP_AUTH_PLAIN_LEN(session)  (session).auth_plain_len
 /* SMTP_COPY_AUTHDATA */
#define SMTP_USERNAME(session)        smtp_username
#define SMTP_PASS(session)            smtp_pass
#define SMTP_AUTH_PLAIN_DATA(session) smtp_auth_plain
#define SMTP_AUTH_PLAIN_LEN(session)  smtp_auth_plain_len




#define SMTP_BODYDH_MALLOC(size)      mem_malloc(size)
#define SMTP_BODYDH_FREE(ptr)         mem_free(ptr)


/* Some internal state return values */
pub const BDHALLDATASENT: u32 = 2; 
pub const BDHSOMEDATASENT: u32 = 1; 

enum bdh_handler_state {
  BDH_SENDING,         /* Serving the user function generating body content */
  BDH_STOP             /* User function stopped, closing */
};


/* State for SMTP client state machine */
enum smtp_session_state {
  SMTP_None,
  SMTP_HELO,
  SMTP_AUTH_PLAIN,
  SMTP_AUTH_LOGIN_UNAME,
  SMTP_AUTH_LOGIN_PASS,
  SMTP_AUTH_LOGIN,
  SMTP_MAIL,
  SMTP_RCPT,
  SMTP_DATA,
  SMTP_BODY,
  SMTP_QUIT,
  SMTP_CLOSED
};


/* State-to-string table for debugging */
static smtp_state_str: &String[] = {
  "SMTP_NULL",
  "SMTP_HELO",
  "SMTP_AUTH_PLAIN",
  "SMTP_AUTH_LOGIN_UNAME",
  "SMTP_AUTH_LOGIN_PASS",
  "SMTP_AUTH_LOGIN",
  "SMTP_MAIL",
  "SMTP_RCPT",
  "SMTP_DATA",
  "SMTP_BODY",
  "SMTP_QUIT",
  "SMTP_CLOSED",
};

static smtp_result_strs: &String[] = {
  "SMTP_RESULT_OK",
  "SMTP_RESULT_ERR_UNKNOWN",
  "SMTP_RESULT_ERR_CONNECT",
  "SMTP_RESULT_ERR_HOSTNAME",
  "SMTP_RESULT_ERR_CLOSED",
  "SMTP_RESULT_ERR_TIMEOUT",
  "SMTP_RESULT_ERR_SVR_RESP",
  "SMTP_RESULT_ERR_MEM"
};



struct smtp_bodydh_state {
  smtp_bodycback_fn callback_fn;  /* The function to call (again) */
  let state: u16;
  let exposed: smtp_bodydh;     /* the user function structure */
};


/* struct keeping the body and state of an smtp session */
struct smtp_session {
  /* keeping the state of the smtp session */
  let state: smtp_session_state;
  /* timeout handling, if this reaches 0, the connection is closed */
  let timer: u16;
  /* helper buffer for transmit, not used for sending body */
  char tx_buf[SMTP_TX_BUF_LEN + 1];
  let p: &mut pbuf;
  /* source email address */
  const char* from;
  /* size of the sourceemail address */
  let from_len: u16;
  /* target email address */
  const char* to;
  /* size of the target email address */
  let to_len: u16;
  /* subject of the email */
  let subject: String;
  /* length of the subject string */
  let subject_len: u16;
  /* this is the body of the mail to be sent */
  const char* body;
  /* this is the length of the body to be sent */
  let body_len: u16;
  /* amount of data from body already sent */
  let body_sent: u16;
  /* callback function to call when closed */
  smtp_result_fn callback_fn;
  /* argument for callback function */
  callback_arg: &mut ();

  /* Username to use for this request */
  username: &mut String;
  /* Password to use for this request */
  pass: &mut String;
  /* Username and password combined as necessary for PLAIN authentication */
  char auth_plain[SMTP_MAX_USERNAME_LEN + SMTP_MAX_PASS_LEN + 3];
  /* Length of smtp_auth_plain string (cannot use strlen since it includes \0) */
  let auth_plain_len: usize;


  bodydh: &mut smtp_bodydh_state;

};

/* IP address or DNS name of the server to use for next SMTP request */
static char smtp_server[SMTP_MAX_SERVERNAME_LEN + 1];
/* TCP port of the server to use for next SMTP request */
static smtp_server_port: u16 = SMTP_DEFAULT_PORT;

/* If this is set, mail is sent using SMTPS */
static smtp_server_tls_config: &mut altcp_tls_config;

/* Username to use for the next SMTP request */
static smtp_username: &mut String;
/* Password to use for the next SMTP request */
static smtp_pass: &mut String;
/* Username and password combined as necessary for PLAIN authentication */
static char smtp_auth_plain[SMTP_MAX_USERNAME_LEN + SMTP_MAX_PASS_LEN + 3];
/* Length of smtp_auth_plain string (cannot use strlen since it includes \0) */
static smtp_auth_plain_len: usize;


static err_t  smtp_verify(data: &String, data_len: usize, linebreaks_allowed: u8);

static err_t  smtp_tcp_recv(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, p: &mut pbuf, err: err_t);
pub fn   smtp_tcp_err(arg: &mut Vec<u8>, err: err_t);
static err_t  smtp_tcp_poll(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb);
static err_t  smtp_tcp_sent(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, len: usize);
static err_t  smtp_tcp_connected(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, err: err_t);

pub fn   smtp_dns_found(const char* hostname,  ipaddr: &mut LwipAddr, arg: &mut Vec<u8>);


static smtp_base64_encode: usize(char* target, target_len: usize,  char* source, source_len: usize);

static enum   smtp_session_state smtp_prepare_mail(s: &mut smtp_session, tx_buf_len: &mut u16);
pub fn   smtp_send_body(s: &mut smtp_session, pcb: &mut AlTcpPcb);
pub fn   smtp_process(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, p: &mut pbuf);

pub fn   smtp_send_body_data_handler(s: &mut smtp_session, pcb: &mut AlTcpPcb);




/* Convert an smtp result to a string */
const char*
smtp_result_str(smtp_result: u8)
{
  if (smtp_result >= LWIP_ARRAYSIZE(smtp_result_strs)) {
    return "UNKNOWN";
  }
  return smtp_result_strs[smtp_result];
}

/* Null-terminates the payload of p for printing out messages.
 * WARNING: use this only if p is not needed any more as the last byte of
 *          payload is deleted!
 */
static const char*
smtp_pbuf_str(struct pbuf* p)
{
  if ((p == None) || (p.len == 0)) {
    return "";
  }
  (p.payload)[p.len] = 0;
  return (const char*)p.payload;
}


/* @ingroup smtp
 * Set IP address or DNS name for next SMTP connection
 *
 * @param server IP address (in ASCII representation) or DNS name of the server
 */
pub fn 
smtp_set_server_addr(const char* server)
{
  len: usize = 0;

  LWIP_ASSERT_CORE_LOCKED();

  if (server != None) {
    /* strlen: returns length WITHOUT terminating 0 byte */
    len = strlen(server);
  }
  if (len > SMTP_MAX_SERVERNAME_LEN) {
    return ERR_MEM;
  }
  if (len != 0) {
    MEMCPY(smtp_server, server, len);
  }
  smtp_server[len] = 0; /* always OK because of smtp_server[SMTP_MAX_SERVERNAME_LEN + 1] */
 return Ok(());
}

/* @ingroup smtp
 * Set TCP port for next SMTP connection
 *
 * @param port TCP port
 */
pub fn 
smtp_set_server_port(port: u16)
{
  LWIP_ASSERT_CORE_LOCKED();
  smtp_server_port = port;
}


/* @ingroup smtp
 * Set TLS configuration for next SMTP connection
 *
 * @param tls_config TLS configuration
 */
pub fn 
smtp_set_tls_config(tls_config: &mut altcp_tls_config)
{
  LWIP_ASSERT_CORE_LOCKED();
  smtp_server_tls_config = tls_config;
}


/* @ingroup smtp
 * Set authentication parameters for next SMTP connection
 *
 * @param username login name as passed to the server
 * @param pass password passed to the server together with username
 */
pub fn 
smtp_set_auth(const char* username,  char* pass)
{
  uname_len: usize = 0;
  pass_len: usize = 0;

  LWIP_ASSERT_CORE_LOCKED();

  //memset(smtp_auth_plain, 0xfa, 64);
  if (username != None) {
    uname_len = strlen(username);
    if (uname_len > SMTP_MAX_USERNAME_LEN) {
/*LWIP_DEBUGF(SMTP_DEBUG_SERIOUS, ("Username is too long, %d instead of %d\n",
        uname_len, SMTP_MAX_USERNAME_LEN));*/
      return ERR_ARG;
    }
  }
  if (pass != None) {

    pass_len = strlen(pass);
    if (pass_len > SMTP_MAX_PASS_LEN) {
/*LWIP_DEBUGF(SMTP_DEBUG_SERIOUS, ("Password is too long, %d instead of %d\n",
        uname_len, SMTP_MAX_USERNAME_LEN));*/
      return ERR_ARG;
    }
 /* SMTP_SUPPORT_AUTH_LOGIN || SMTP_SUPPORT_AUTH_PLAIN */
//    LWIP_DEBUGF(SMTP_DEBUG_WARN, ("Password not supported as no authentication methods are activated\n"));

  }
  *smtp_auth_plain = 0;
  if (username != None) {
    smtp_username = smtp_auth_plain + 1;
    strcpy(smtp_username, username);
  }
  if (pass != None) {
    smtp_pass = smtp_auth_plain + uname_len + 2;
    strcpy(smtp_pass, pass);
  }
  smtp_auth_plain_len = uname_len + pass_len + 2;

 return Ok(());
}


pub fn smtp_free_struct(s: &mut smtp_session)
{
  if (s.bodydh != None) {
    SMTP_BODYDH_FREE(s.bodydh);
  }
  SMTP_STATE_FREE(s);
}
 /* SMTP_BODYDH */
#define smtp_free_struct(x) SMTP_STATE_FREE(x)


static struct AlTcpPcb*
smtp_setup_pcb(s: &mut smtp_session,  remote_ip: &mut LwipAddr)
{
  let pcb: &mut AlTcpPcb;
  


  if (smtp_server_tls_config) {
    pcb = altcp_tls_new(smtp_server_tls_config, IP_GET_TYPE(remote_ip));
  } else

  {
    pcb = altcp_tcp_new_ip_type(IP_GET_TYPE(remote_ip));
  }
  if (pcb != None) {
    altcp_arg(pcb, s);
    altcp_recv(pcb, smtp_tcp_recv);
    altcp_err(pcb, smtp_tcp_err);
    altcp_poll(pcb, smtp_tcp_poll, SMTP_POLL_INTERVAL);
    altcp_sent(pcb, smtp_tcp_sent);
  }
  return pcb;
}

/* The actual mail-sending function, called by smtp_send_mail and
 * smtp_send_mail_static after setting up the struct smtp_session.
 */
pub fn smtp_send_mail_alloced(s: &mut smtp_session) -> Result<(), LwipError>
{
  let err: err_t;
  struct AlTcpPcb* pcb = None;
  let addr: LwipAddr;

  LWIP_ASSERT("no smtp_session supplied", s != None);


  /* check that body conforms to RFC:
   * - convert all single-CR or -LF in body to CRLF
   * - only 7-bit ASCII is allowed
   */
  if (smtp_verify(s.to, s.to_len, 0) != ERR_OK) {
    err = ERR_ARG;
    // goto leave;
  }
  if (smtp_verify(s.from, s.from_len, 0) != ERR_OK) {
    err = ERR_ARG;
    // goto leave;
  }
  if (smtp_verify(s.subject, s.subject_len, 0) != ERR_OK) {
    err = ERR_ARG;
    // goto leave;
  }

  if (s.bodydh == None)

  {
    if (smtp_verify(s.body, s.body_len, 0) != ERR_OK) {
      err = ERR_ARG;
      // goto leave;
    }
  }



  /* copy auth data, ensuring the first byte is always zero */
  MEMCPY(s.auth_plain + 1, smtp_auth_plain + 1, smtp_auth_plain_len - 1);
  s.auth_plain_len = smtp_auth_plain_len;
  /* default username and pass is empty string */
  s.username = s.auth_plain;
  s.pass = s.auth_plain;
  if (smtp_username != None) {
    s.username += smtp_username - smtp_auth_plain;
  }
  if (smtp_pass != None) {
    s.pass += smtp_pass - smtp_auth_plain;
  }


  s.state = SMTP_None;
  s.timer = SMTP_TIMEOUT;


  err = dns_gethostbyname(smtp_server, &addr, smtp_dns_found, s);
 /* LWIP_DNS */
  err = ipaddr_aton(smtp_server, &addr) ? ERR_OK : ERR_ARG;

  if (err == ERR_OK) {
    pcb = smtp_setup_pcb(s, &addr);
    if (pcb == None) {
      err = ERR_MEM;
      // goto leave;
    }
    err = altcp_connect(pcb, &addr, smtp_server_port, smtp_tcp_connected);
    if (err != ERR_OK) {
//      LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("tcp_connect failed: %d\n", err));
      // goto deallocate_and_leave;
    }
  } else if (err != ERR_INPROGRESS) {
//    LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("dns_gethostbyname failed: %d\n", err));
    // goto deallocate_and_leave;
  }
 return Ok(());

// deallocate_and_leave:
  if (pcb != None) {
    altcp_arg(pcb, None);
    altcp_close(pcb);
  }
leave:
  smtp_free_struct(s);
  /* no need to call the callback here since we return != ERR_OK */
  return err;
}

/* @ingroup smtp
 *  Send an email via the currently selected server, username and password.
 *
 * @param from source email address (must be NULL-terminated)
 * @param to target email address (must be NULL-terminated)
 * @param subject email subject (must be NULL-terminated)
 * @param body email body (must be NULL-terminated)
 * @param callback_fn callback function
 * @param callback_arg user argument to callback_fn
 * @returns - ERR_OK if structures were allocated and no error occured starting the connection
 *            (this does not mean the email has been successfully sent!)
 *          - another on: err_t error.
 */
pub fn 
smtp_send_mail(const char* from,  char* to,  char* subject,  char* body,
               smtp_result_fn callback_fn, void* callback_arg)
{
  let s: &mut smtp_session;
  from_len: usize = strlen(from);
  to_len: usize = strlen(to);
  subject_len: usize = strlen(subject);
  body_len: usize = strlen(body);
  mem_len: usize = sizeof(struct smtp_session);
  sfrom: &mut String, *sto, *ssubject, *sbody;

  LWIP_ASSERT_CORE_LOCKED();

  mem_len += from_len + to_len + subject_len + body_len + 4;
  if (mem_len > 0xffff) {
    /* too long! */
    return ERR_MEM;
  }

  /* Allocate memory to keep this email's session state */
  s = (struct smtp_session *)SMTP_STATE_MALLOC((mem_usize)mem_len);
  if (s == None) {
    return ERR_MEM;
  }
  /* initialize the structure */
  //memset(s, 0, mem_len);
  s.from = sfrom = s + sizeof(struct smtp_session);
  s.from_len = from_len;
  s.to = sto = sfrom + from_len + 1;
  s.to_len = to_len;
  s.subject = ssubject = sto + to_len + 1;
  s.subject_len = subject_len;
  s.body = sbody = ssubject + subject_len + 1;
  s.body_len = body_len;
  /* copy source and target email address */
  /* cast to is: usize a hack to cast away constness */
  MEMCPY(sfrom, from, from_len + 1);
  MEMCPY(sto, to, to_len + 1);
  MEMCPY(ssubject, subject, subject_len + 1);
  MEMCPY(sbody, body, body_len + 1);

  s.callback_fn = callback_fn;
  s.callback_arg = callback_arg;

  /* call the actual implementation of this function */
  return smtp_send_mail_alloced(s);
}

/* @ingroup smtp
 * Same as smtp_send_mail, but doesn't copy from, to, subject and body into
 * an internal buffer to save memory.
 * WARNING: the above data must stay untouched until the callback function is
 *          called (unless the function returns != ERR_OK)
 */
pub fn 
smtp_send_mail_static(from: &String,  char* to,  char* subject,
  const char* body, smtp_result_fn callback_fn, void* callback_arg)
{
  let s: &mut smtp_session;
  let len: usize;

  LWIP_ASSERT_CORE_LOCKED();

  s = (struct smtp_session*)SMTP_STATE_MALLOC(sizeof(struct smtp_session));
  if (s == None) {
    return ERR_MEM;
  }
  //memset(s, 0, sizeof(struct smtp_session));
  /* initialize the structure */
  s.from = from;
  len = strlen(from);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.from_len = len;
  s.to = to;
  len = strlen(to);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.to_len = len;
  s.subject = subject;
  len = strlen(subject);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.subject_len = len;
  s.body = body;
  len = strlen(body);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.body_len = len;
  s.callback_fn = callback_fn;
  s.callback_arg = callback_arg;
  /* call the actual implementation of this function */
  return smtp_send_mail_alloced(s);
}


/* @ingroup smtp
 * Same as smtp_send_mail but takes a struct smtp_send_request as single
 * parameter which contains all the other parameters.
 * To be used with tcpip_callback to send mail from interrupt context or from
 * another thread.
 *
 * WARNING: server and authentication must stay untouched until this function has run!
 *
 * Usage example:
 * - allocate a struct smtp_send_request (in a way that is allowed in interrupt context)
 * - fill the members of the struct as if calling smtp_send_mail
 * - specify a callback_function
 * - set callback_arg to the structure itself
 * - call this function
 * - wait for the callback function to be called
 * - in the callback function, deallocate the structure (passed as arg)
 */
pub fn 
smtp_send_mail_int(arg: &mut Vec<u8>)
{
  req: &mut smtp_send_request = (struct smtp_send_request*)arg;
  let err: err_t;

  LWIP_ASSERT_CORE_LOCKED();
  LWIP_ASSERT("smtp_send_mail_int: no argument given", arg != None);

  if (req.static_data) {
    err = smtp_send_mail_static(req.from, req.to, req.subject, req.body,
      req.callback_fn, req.callback_arg);
  } else {
    err = smtp_send_mail(req.from, req.to, req.subject, req.body,
      req.callback_fn, req.callback_arg);
  }
  if ((err != ERR_OK) && (req.callback_fn != None)) {
    req.callback_fn(req.callback_arg, SMTP_RESULT_ERR_UNKNOWN, 0, err);
  }
}


/* Verify that a given string conforms to the SMTP rules
 * (7-bit only, no single CR or LF,
 *  @todo: no line consisting of a single dot only)
 */
pub fn smtp_verify(data: &String, data_len: usize, linebreaks_allowed: u8) -> Result<(), LwipError>
{
  let i: usize;
  last_was_cr: u8 = 0;
  for (i = 0; i < data_len; i+= 1) {
    char current = data[i];
    if ((current & 0x80) != 0) {
//      LWIP_DEBUGF(SMTP_DEBUG_WARN, ("smtp_verify: no 8-bit data supported: %s\n", data));
      return ERR_ARG;
    }
    if (current == '\r') {
      if (!linebreaks_allowed) {
//        LWIP_DEBUGF(SMTP_DEBUG_WARN, ("smtp_verify: found CR where no linebreaks allowed: %s\n", data));
        return ERR_ARG;
      }
      if (last_was_cr) {
//        LWIP_DEBUGF(SMTP_DEBUG_WARN, ("smtp_verify: found double CR: %s\n", data));
        return ERR_ARG;
      }
      last_was_cr = 1;
    } else {
      if (current == '\n') {
        if (!last_was_cr) {
//          LWIP_DEBUGF(SMTP_DEBUG_WARN, ("smtp_verify: found LF without CR before: %s\n", data));
          return ERR_ARG;
        }
      }
      last_was_cr = 0;
    }
  }
 return Ok(());
}


/* Frees the smtp_session and calls the callback function */
pub fn
smtp_free(s: &mut smtp_session, result: u8, srv_err: u16, err: err_t)
{
  smtp_result_fn fn = s.callback_fn;
  arg: &mut Vec<u8> = s.callback_arg;
  if (s.p != None) {
    pbuf_free(s.p);
  }
  smtp_free_struct(s);
  if (fn != None) {
    fn(arg, result, srv_err, err);
  }
}

/* Try to close a pcb and free the arg if successful */
pub fn
smtp_close(s: &mut smtp_session, pcb: &mut AlTcpPcb, result: u8,
           srv_err: u16, err: err_t)
{
  if (pcb != None) {
     altcp_arg(pcb, None);
     if (altcp_close(pcb) == ERR_OK) {
       if (s != None) {
         smtp_free(s, result, srv_err, err);
       }
     } else {
       /* close failed, set back arg */
       altcp_arg(pcb, s);
     }
  } else {
    if (s != None) {
      smtp_free(s, result, srv_err, err);
    }
  }
}

/* Raw API TCP err callback: pcb is already deallocated */
pub fn
smtp_tcp_err(arg: &mut Vec<u8>, err: err_t)
{
  
  if (arg != None) {
//    LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("smtp_tcp_err: connection reset by remote host\n"));
    smtp_free((struct smtp_session*)arg, SMTP_RESULT_ERR_CLOSED, 0, err);
  }
}

/* Raw API TCP poll callback */
pub fn smtp_tcp_poll(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb) -> Result<(), LwipError>
{
  if (arg != None) {
    s: &mut smtp_session = (struct smtp_session*)arg;
    if (s.timer != 0) {
      s.timer -= 1;
    }
  }
  smtp_process(arg, pcb, None);
 return Ok(());
}

/* Raw API TCP sent callback */
pub fn smtp_tcp_sent(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, len: usize) -> Result<(), LwipError>
{
  

  smtp_process(arg, pcb, None);

 return Ok(());
}

/* Raw API TCP recv callback */
pub fn smtp_tcp_recv(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, p: &mut pbuf, err: err_t) -> Result<(), LwipError>
{
  
  if (p != None) {
    altcp_recved(pcb, p.tot_len);
    smtp_process(arg, pcb, p);
  } else {
//    LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("smtp_tcp_recv: connection closed by remote host\n"));
    smtp_close((struct smtp_session*)arg, pcb, SMTP_RESULT_ERR_CLOSED, 0, err);
  }
 return Ok(());
}

pub fn smtp_tcp_connected(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, err: err_t) -> Result<(), LwipError>
{
  

  if (err == ERR_OK) {
//    LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_connected: Waiting for 220\n"));
  } else {
    /* shouldn't happen, but we still check 'err', only to be sure */
//    LWIP_DEBUGF(SMTP_DEBUG_WARN, ("smtp_connected: %d\n", err));
    smtp_close((struct smtp_session*)arg, pcb, SMTP_RESULT_ERR_CONNECT, 0, err);
  }
 return Ok(());
}


/* DNS callback
 * If ipaddr is non-NULL, resolving succeeded, otherwise it failed.
 */
pub fn
smtp_dns_found(const char* hostname,  ipaddr: &mut LwipAddr, arg: &mut Vec<u8>)
{
  s: &mut smtp_session = (struct smtp_session*)arg;
  pcb: &mut AlTcpPcb;
  let err: err_t;
  let result: u8;

  

  if (ipaddr != None) {
    pcb = smtp_setup_pcb(s, ipaddr);
    if (pcb != None) {
//      LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_dns_found: hostname resolved, connecting\n"));
      err = altcp_connect(pcb, ipaddr, smtp_server_port, smtp_tcp_connected);
      if (err == ERR_OK) {
        return;
      }
//      LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("tcp_connect failed: %d\n", err));
      result = SMTP_RESULT_ERR_CONNECT;
    } else {
//      LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_dns_found: failed to allocate tcp pcb\n"));
      result = SMTP_RESULT_ERR_MEM;
      err = ERR_MEM;
    }
  } else {
/*LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("smtp_dns_found: failed to resolve hostname: %s\n",
      hostname));*/
    pcb = None;
    result = SMTP_RESULT_ERR_HOSTNAME;
    err = ERR_ARG;
  }
  smtp_close(s, pcb, result, 0, err);
}




/* Table 6-bit-index-to-ASCII used for base64-encoding */
static const char base64_table[] = {
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
  'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
  'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
  'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
  'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
  '+', '/'
};

/* Base64 encoding */
pub fn smtp_base64_encode(char* target, target_len: usize,  char* source, source_len: usize)
{
  let i: usize;
  let j: i8;
  target_idx: usize = 0;
  longer: usize = (source_len % 3) ? (3 - (source_len % 3)) : 0;
  source_len_b64: usize = source_len + longer;
  len: usize = (((source_len_b64) * 4) / 3);
  x: u8 = 5;
  current: u8 = 0;
  

  LWIP_ASSERT("target_len is too short", target_len >= len);

  for (i = 0; i < source_len_b64; i+= 1) {
    b: u8 = (i < source_len ? source[i] : 0);
    for (j = 7; j >= 0; j--, x--) {
      if ((b & (1 << j)) != 0) {
        current = (current | (1 << x));
      }
      if (x == 0) {
        target[target_idx+= 1] = base64_table[current];
        x = 6;
        current = 0;
      }
    }
  }
  for (i = len - longer; i < len; i+= 1) {
    target[i] = '=';
  }
  return len;
}


/* Parse pbuf to see if it contains the beginning of an answer.
 * If so, it returns the contained response code as number between 1 and 999.
 * If not, zero is returned.
 *
 * @param s smtp session struct
 */
pub fn smtp_is_response(s: &mut smtp_session)
{
  let digits: String;
  let num: i32;

  if (s.p == None) {
    return 0;
  }
  /* copy three digits and convert them to int */
  if (pbuf_copy_partial(s.p, digits, 3, 0) != 3) {
    /* pbuf was too short */
    return 0;
  }
  digits[3] = 0;
  num = strtol(digits, None, 10);
  if ((num <= 0) || (num >= 1000)) {
    /* failed to find response code at start of line */
    return 0;
  }
  return num;
}

/* Parse pbuf to see if it contains a fully received answer.
 * If one is found, ERR_OK is returned.
 * If none is found, ERR_VAL is returned.
 *
 * A fully received answer is a 3-digit number followed by a space,
 * some string and a CRLF as line ending.
 *
 * @param s smtp session struct
 */
pub fn smtp_is_response_finished(s: &mut smtp_session) -> Result<(), LwipError>
{
  let sp: u8;
  let crlf: u16;
  let offset: u16;

  if (s.p == None) {
    return ERR_VAL;
  }
  offset = 0;
again:
  /* We could check the response number here, but we trust the
   * protocol definition which says the client can rely on it being
   * the same on every line. */

  /* find CRLF */
  if (offset > 0xFFFF - 4) {
    /* would overflow */
    return ERR_VAL;
  }
  crlf = pbuf_memfind(s.p, SMTP_CRLF, SMTP_CRLF_LEN, (offset + 4));
  if (crlf == 0xFFFF) {
    /* no CRLF found */
    return ERR_VAL;
  }
  sp = pbuf_get_at(s.p, (offset + 3));
  if (sp == '-') {
    /* no space after response code -> try next line */
    offset = (crlf + 2);
    if (offset < crlf) {
      /* overflow */
      return ERR_VAL;
    }
    // goto again;
  } else if (sp == ' ') {
    /* CRLF found after response code + space -> valid response */
   return Ok(());
  }
  /* sp contains invalid character */
  return ERR_VAL;
}

/* Prepare HELO/EHLO message */
static enum smtp_session_state
smtp_prepare_helo(s: &mut smtp_session, tx_buf_len: &mut u16, pcb: &mut AlTcpPcb)
{
  let ipa_len: usize;
  ipa: &String = ipaddr_ntoa(altcp_get_ip(pcb, 1));
  LWIP_ASSERT("ipaddr_ntoa returned NULL", ipa != None);
  ipa_len = strlen(ipa);
  LWIP_ASSERT("string too long", ipa_len <= (SMTP_TX_BUF_LEN-SMTP_CMD_EHLO_1_LEN-SMTP_CMD_EHLO_2_LEN));

  *tx_buf_len = (SMTP_CMD_EHLO_1_LEN + ipa_len + SMTP_CMD_EHLO_2_LEN);
  LWIP_ASSERT("tx_buf overflow detected", *tx_buf_len <= SMTP_TX_BUF_LEN);

  SMEMCPY(s.tx_buf, SMTP_CMD_EHLO_1, SMTP_CMD_EHLO_1_LEN);
  MEMCPY(&s.tx_buf[SMTP_CMD_EHLO_1_LEN], ipa, ipa_len);
  SMEMCPY(&s.tx_buf[SMTP_CMD_EHLO_1_LEN + ipa_len], SMTP_CMD_EHLO_2, SMTP_CMD_EHLO_2_LEN);
  return SMTP_HELO;
}


/* Parse last server response (in rx_buf) for supported authentication method,
 * create data to send out (to tx_buf), set tx_data_len correctly
 * and return the next state.
 */
static enum smtp_session_state
smtp_prepare_auth_or_mail(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  /* check response for supported authentication method */
  auth: u16 = pbuf_strstr(s.p, SMTP_KEYWORD_AUTH_SP);
  if (auth == 0xFFFF) {
    auth = pbuf_strstr(s.p, SMTP_KEYWORD_AUTH_EQ);
  }
  if (auth != 0xFFFF) {
    crlf: u16 = pbuf_memfind(s.p, SMTP_CRLF, SMTP_CRLF_LEN, auth);
    if ((crlf != 0xFFFF) && (crlf > auth)) {
      /* use tx_buf temporarily */
      copied: u16 = pbuf_copy_partial(s.p, s.tx_buf, (crlf - auth), auth);
      if (copied != 0) {
        sep: &mut String = s.tx_buf + SMTP_KEYWORD_AUTH_LEN;
        s.tx_buf[copied] = 0;

        /* favour PLAIN over LOGIN since it involves less requests */
        if (strstr(sep, SMTP_AUTH_PARAM_PLAIN) != None) {
          let auth_len: usize;
          /* server supports AUTH PLAIN */
          SMEMCPY(s.tx_buf, SMTP_CMD_AUTHPLAIN_1, SMTP_CMD_AUTHPLAIN_1_LEN);

          /* add base64-encoded string "\0username\0password" */
          auth_len = smtp_base64_encode(&s.tx_buf[SMTP_CMD_AUTHPLAIN_1_LEN],
            SMTP_TX_BUF_LEN - SMTP_CMD_AUTHPLAIN_1_LEN, SMTP_AUTH_PLAIN_DATA(s),
            SMTP_AUTH_PLAIN_LEN(s));
          LWIP_ASSERT("string too long", auth_len <= (SMTP_TX_BUF_LEN-SMTP_CMD_AUTHPLAIN_1_LEN-SMTP_CMD_AUTHPLAIN_2_LEN));
          *tx_buf_len = (SMTP_CMD_AUTHPLAIN_1_LEN + SMTP_CMD_AUTHPLAIN_2_LEN + auth_len);
          SMEMCPY(&s.tx_buf[SMTP_CMD_AUTHPLAIN_1_LEN + auth_len], SMTP_CMD_AUTHPLAIN_2,
            SMTP_CMD_AUTHPLAIN_2_LEN);
          return SMTP_AUTH_PLAIN;
        } else

        {

          if (strstr(sep, SMTP_AUTH_PARAM_LOGIN) != None) {
            /* server supports AUTH LOGIN */
            *tx_buf_len = SMTP_CMD_AUTHLOGIN_LEN;
            SMEMCPY(s.tx_buf, SMTP_CMD_AUTHLOGIN, SMTP_CMD_AUTHLOGIN_LEN);
            return SMTP_AUTH_LOGIN_UNAME;
          }

        }
      }
    }
  }
  /* server didnt's send correct keywords for AUTH, try sending directly */
  return smtp_prepare_mail(s, tx_buf_len);
}



/* Send base64-encoded username */
static enum smtp_session_state
smtp_prepare_auth_login_uname(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  base64_len: usize = smtp_base64_encode(s.tx_buf, SMTP_TX_BUF_LEN,
    SMTP_USERNAME(s), strlen(SMTP_USERNAME(s)));
  /* @todo: support base64-encoded longer than 64k */
  LWIP_ASSERT("string too long", base64_len <= 0xffff);
  LWIP_ASSERT("tx_buf overflow detected", base64_len <= SMTP_TX_BUF_LEN - SMTP_CRLF_LEN);
  *tx_buf_len = (base64_len + SMTP_CRLF_LEN);

  SMEMCPY(&s.tx_buf[base64_len], SMTP_CRLF, SMTP_CRLF_LEN);
  s.tx_buf[*tx_buf_len] = 0;
  return SMTP_AUTH_LOGIN_PASS;
}

/* Send base64-encoded password */
static enum smtp_session_state
smtp_prepare_auth_login_pass(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  base64_len: usize = smtp_base64_encode(s.tx_buf, SMTP_TX_BUF_LEN,
    SMTP_PASS(s), strlen(SMTP_PASS(s)));
  /* @todo: support base64-encoded longer than 64k */
  LWIP_ASSERT("string too long", base64_len <= 0xffff);
  LWIP_ASSERT("tx_buf overflow detected", base64_len <= SMTP_TX_BUF_LEN - SMTP_CRLF_LEN);
  *tx_buf_len = (base64_len + SMTP_CRLF_LEN);

  SMEMCPY(&s.tx_buf[base64_len], SMTP_CRLF, SMTP_CRLF_LEN);
  s.tx_buf[*tx_buf_len] = 0;
  return SMTP_AUTH_LOGIN;
}


/* Prepare MAIL message */
static enum smtp_session_state
smtp_prepare_mail(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  target: &mut String = s.tx_buf;
  LWIP_ASSERT("tx_buf overflow detected", s.from_len <= (SMTP_TX_BUF_LEN - SMTP_CMD_MAIL_1_LEN - SMTP_CMD_MAIL_2_LEN));
  *tx_buf_len = (SMTP_CMD_MAIL_1_LEN + SMTP_CMD_MAIL_2_LEN + s.from_len);
  target[*tx_buf_len] = 0;

  SMEMCPY(target, SMTP_CMD_MAIL_1, SMTP_CMD_MAIL_1_LEN);
  target += SMTP_CMD_MAIL_1_LEN;
  MEMCPY(target, s.from, s.from_len);
  target += s.from_len;
  SMEMCPY(target, SMTP_CMD_MAIL_2, SMTP_CMD_MAIL_2_LEN);
  return SMTP_MAIL;
}

/* Prepare RCPT message */
static enum smtp_session_state
smtp_prepare_rcpt(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  target: &mut String = s.tx_buf;
  LWIP_ASSERT("tx_buf overflow detected", s.to_len <= (SMTP_TX_BUF_LEN - SMTP_CMD_RCPT_1_LEN - SMTP_CMD_RCPT_2_LEN));
  *tx_buf_len = (SMTP_CMD_RCPT_1_LEN + SMTP_CMD_RCPT_2_LEN + s.to_len);
  target[*tx_buf_len] = 0;

  SMEMCPY(target, SMTP_CMD_RCPT_1, SMTP_CMD_RCPT_1_LEN);
  target += SMTP_CMD_RCPT_1_LEN;
  MEMCPY(target, s.to, s.to_len);
  target += s.to_len;
  SMEMCPY(target, SMTP_CMD_RCPT_2, SMTP_CMD_RCPT_2_LEN);
  return SMTP_RCPT;
}

/* Prepare header of body */
static enum smtp_session_state
smtp_prepare_header(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  target: &mut String = s.tx_buf;
  len: i32 = SMTP_CMD_HEADER_1_LEN + SMTP_CMD_HEADER_2_LEN +
    SMTP_CMD_HEADER_3_LEN + SMTP_CMD_HEADER_4_LEN + s.from_len + s.to_len +
    s.subject_len;
  LWIP_ASSERT("tx_buf overflow detected", len > 0 && len <= SMTP_TX_BUF_LEN);
  *tx_buf_len = len;
  target[*tx_buf_len] = 0;

  SMEMCPY(target, SMTP_CMD_HEADER_1, SMTP_CMD_HEADER_1_LEN);
  target += SMTP_CMD_HEADER_1_LEN;
  MEMCPY(target, s.from, s.from_len);
  target += s.from_len;
  SMEMCPY(target, SMTP_CMD_HEADER_2, SMTP_CMD_HEADER_2_LEN);
  target += SMTP_CMD_HEADER_2_LEN;
  MEMCPY(target, s.to, s.to_len);
  target += s.to_len;
  SMEMCPY(target, SMTP_CMD_HEADER_3, SMTP_CMD_HEADER_3_LEN);
  target += SMTP_CMD_HEADER_3_LEN;
  MEMCPY(target, s.subject, s.subject_len);
  target += s.subject_len;
  SMEMCPY(target, SMTP_CMD_HEADER_4, SMTP_CMD_HEADER_4_LEN);

  return SMTP_BODY;
}

/* Prepare QUIT message */
static enum smtp_session_state
smtp_prepare_quit(s: &mut smtp_session, tx_buf_len: &mut u16)
{
  *tx_buf_len = SMTP_CMD_QUIT_LEN;
  s.tx_buf[*tx_buf_len] = 0;
  SMEMCPY(s.tx_buf, SMTP_CMD_QUIT, SMTP_CMD_QUIT_LEN);
  LWIP_ASSERT("tx_buf overflow detected", *tx_buf_len <= SMTP_TX_BUF_LEN);
  return SMTP_CLOSED;
}

/* If in state SMTP_BODY, try to send more body data */
pub fn
smtp_send_body(s: &mut smtp_session, pcb: &mut AlTcpPcb)
{
  let err: err_t;

  if (s.state == SMTP_BODY) {

    if (s.bodydh) {
      smtp_send_body_data_handler(s, pcb);
    } else

    {
      send_len: u16 = (s.body_len - s.body_sent);
      if (send_len > 0) {
        snd_buf: u16 = altcp_sndbuf(pcb);
        if (send_len > snd_buf) {
          send_len = snd_buf;
        }
        if (send_len > 0) {
          /* try to send something out */
          err = altcp_write(pcb, &s.body[s.body_sent], send_len, TCP_WRITE_FLAG_COPY);
          if (err == ERR_OK) {
            s.timer = SMTP_TIMEOUT_DATABLOCK;
            s.body_sent = (s.body_sent + send_len);
            if (s.body_sent < s.body_len) {
/*LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_send_body: %d of %d bytes written\n",
                s.body_sent, s.body_len));*/
            }
          }
        }
      }
    }
    if (s.body_sent == s.body_len) {
      /* the whole body has been written, write last line */
/*LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_send_body: body completely written (%d bytes), appending end-of-body\n",
        s.body_len));*/
      err = altcp_write(pcb, SMTP_CMD_BODY_FINISHED, SMTP_CMD_BODY_FINISHED_LEN, 0);
      if (err == ERR_OK) {
        s.timer = SMTP_TIMEOUT_DATATERM;
/*LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_send_body: end-of-body written, changing state to %s\n",
          smtp_state_str[SMTP_QUIT]));*/
        /* last line written, change state, wait for confirmation */
        s.state = SMTP_QUIT;
      }
    }
  }
}

/* State machine-like implementation of an SMTP client.
 */
pub fn
smtp_process(arg: &mut Vec<u8>, pcb: &mut AlTcpPcb, p: &mut pbuf)
{
  struct smtp_session* s = (struct smtp_session*)arg;
let   response_code: u16 = 0;let 
  tx_buf_len: u16 = 0;
  let next_state: smtp_session_state;

  if (arg == None) {
    /* already closed SMTP connection */
    if (p != None) {
/*LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("Received %d bytes after closing: %s\n",
        p.tot_len, smtp_pbuf_str(p)));*/
      pbuf_free(p);
    }
    return;
  }

  next_state = s.state;

  if (p != None) {
    /* received data */
    if (s.p == None) {
      s.p = p;
    } else {
      pbuf_cat(s.p, p);
    }
  } else {
    /* idle timer, close connection if timed out */
    if (s.timer == 0) {
//      LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("smtp_process: connection timed out, closing\n"));
      smtp_close(s, pcb, SMTP_RESULT_ERR_TIMEOUT, 0, ERR_TIMEOUT);
      return;
    }
    if (s.state == SMTP_BODY) {
      smtp_send_body(s, pcb);
      return;
    }
  }
  response_code = smtp_is_response(s);
  if (response_code) {
//    LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_process: received response code: %d\n", response_code));
    if (smtp_is_response_finished(s) != ERR_OK) {
//      LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_process: partly received response code: %d\n", response_code));
      /* wait for next packet to complete the respone */
      return;
    }
  } else {
    if (s.p != None) {
/*LWIP_DEBUGF(SMTP_DEBUG_WARN, ("smtp_process: unknown data received (%s)\n",
        smtp_pbuf_str(s.p)));*/
      pbuf_free(s.p);
      s.p = None;
    }
    return;
  }

  match(s.state)
  {
  case(SMTP_None):
    /* wait for 220 */
    if (response_code == 220) {
      /* then send EHLO */
      next_state = smtp_prepare_helo(s, &tx_buf_len, pcb);
    }
    break;
  case(SMTP_HELO):
    /* wait for 250 */
    if (response_code == 250) {

      /* then send AUTH or MAIL */
      next_state = smtp_prepare_auth_or_mail(s, &tx_buf_len);
    }
    break;
  case(SMTP_AUTH_LOGIN):
  case(SMTP_AUTH_PLAIN):
    /* wait for 235 */
    if (response_code == 235) {

      /* send MAIL */
      next_state = smtp_prepare_mail(s, &tx_buf_len);
    }
    break;

  case(SMTP_AUTH_LOGIN_UNAME):
    /* wait for 334 Username */
    if (response_code == 334) {
      if (pbuf_strstr(s.p, SMTP_RESP_LOGIN_UNAME) != 0xFFFF) {
        /* send username */
        next_state = smtp_prepare_auth_login_uname(s, &tx_buf_len);
      }
    }
    break;
  case(SMTP_AUTH_LOGIN_PASS):
    /* wait for 334 Password */
    if (response_code == 334) {
      if (pbuf_strstr(s.p, SMTP_RESP_LOGIN_PASS) != 0xFFFF) {
        /* send username */
        next_state = smtp_prepare_auth_login_pass(s, &tx_buf_len);
      }
    }
    break;

  case(SMTP_MAIL):
    /* wait for 250 */
    if (response_code == 250) {
      /* send RCPT */
      next_state = smtp_prepare_rcpt(s, &tx_buf_len);
    }
    break;
  case(SMTP_RCPT):
    /* wait for 250 */
    if (response_code == 250) {
      /* send DATA */
      SMEMCPY(s.tx_buf, SMTP_CMD_DATA, SMTP_CMD_DATA_LEN);
      tx_buf_len = SMTP_CMD_DATA_LEN;
      next_state = SMTP_DATA;
    }
    break;
  case(SMTP_DATA):
    /* wait for 354 */
    if (response_code == 354) {
      /* send email header */
      next_state = smtp_prepare_header(s, &tx_buf_len);
    }
    break;
  case(SMTP_BODY):
    /* nothing to be done here, handled somewhere else */
    break;
  case(SMTP_QUIT):
    /* wait for 250 */
    if (response_code == 250) {
      /* send QUIT */
      next_state = smtp_prepare_quit(s, &tx_buf_len);
    }
    break;
  case(SMTP_CLOSED):
    /* nothing to do, wait for connection closed from server */
    return;
  _ =>
/*LWIP_DEBUGF(SMTP_DEBUG_SERIOUS, ("Invalid state: %d/%s\n", s.state,
      smtp_state_str[s.state]));*/
    break;
  }
  if (s.state == next_state) {
/*LWIP_DEBUGF(SMTP_DEBUG_WARN_STATE, ("smtp_process[%s]: unexpected response_code, closing: %d (%s)\n",
      smtp_state_str[s.state], response_code, smtp_pbuf_str(s.p)));*/
    /* close connection */
    smtp_close(s, pcb, SMTP_RESULT_ERR_SVR_RESP, response_code, ERR_OK);
    return;
  }
  if (tx_buf_len > 0) {
    SMTP_TX_BUF_MAX(tx_buf_len);
    if (altcp_write(pcb, s.tx_buf, tx_buf_len, TCP_WRITE_FLAG_COPY) == ERR_OK) {
/*LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_process[%s]: received command %d (%s)\n",
        smtp_state_str[s.state], response_code, smtp_pbuf_str(s.p)));*//*LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_process[%s]: sent %"U16_F" bytes: \"%s\"\n",
        smtp_state_str[s.state], tx_buf_len, s.tx_buf));*/
      s.timer = SMTP_TIMEOUT;
      pbuf_free(s.p);
      s.p = None;
/*LWIP_DEBUGF(SMTP_DEBUG_STATE, ("smtp_process: changing state from %s to %s\n",
        smtp_state_str[s.state], smtp_state_str[next_state]));*/
      s.state = next_state;
      if (next_state == SMTP_BODY) {
        /* try to stream-send body data right now */
        smtp_send_body(s, pcb);
      } else if (next_state == SMTP_CLOSED) {
        /* sent out all data, delete structure */
        altcp_arg(pcb, None);
        smtp_free(s, SMTP_RESULT_OK, 0, ERR_OK);
      }
    }
  }
}


/* Elementary sub-function to send data
 *
 * @returns: BDHALLDATASENT all data has been written
 *           BDHSOMEDATASENT some data has been written
 *           0 no data has been written
 */
pub fn smtp_send_bodyh_data(pcb: &mut AlTcpPcb,  char **from, howmany: &mut u16)
{
  let err: err_t;
  len: usize = *howmany;

  len = LWIP_MIN(len, altcp_sndbuf(pcb));
  err = altcp_write(pcb, *from, len, TCP_WRITE_FLAG_COPY);
  if (err == ERR_OK) {
    *from += len;
    if ((*howmany -= len) > 0) {
      return BDHSOMEDATASENT;
    }
    return BDHALLDATASENT;
  }
  return 0;
}

/* Same as smtp_send_mail_static, but uses a callback function to send body data
 */
pub fn 
smtp_send_mail_bodycback(from: &String,  char* to,  char* subject,
  smtp_bodycback_fn bodycback_fn, smtp_result_fn callback_fn, void* callback_arg)
{
  let s: &mut smtp_session;
  let len: usize;

  LWIP_ASSERT_CORE_LOCKED();

  s = (struct smtp_session*)SMTP_STATE_MALLOC(sizeof(struct smtp_session));
  if (s == None) {
    return ERR_MEM;
  }
  //memset(s, 0, sizeof(struct smtp_session));
  s.bodydh = (struct smtp_bodydh_state*)SMTP_BODYDH_MALLOC(sizeof(struct smtp_bodydh_state));
  if (s.bodydh == None) {
    SMTP_STATE_FREE(s);
    return ERR_MEM;
  }
  //memset(s.bodydh, 0, sizeof(struct smtp_bodydh_state));
  /* initialize the structure */
  s.from = from;
  len = strlen(from);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.from_len = len;
  s.to = to;
  len = strlen(to);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.to_len = len;
  s.subject = subject;
  len = strlen(subject);
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.subject_len = len;
  s.body = None;
  LWIP_ASSERT("string is too long", len <= 0xffff);
  s.callback_fn = callback_fn;
  s.callback_arg = callback_arg;
  s.bodydh.callback_fn = bodycback_fn;
  s.bodydh.state = BDH_SENDING;
  /* call the actual implementation of this function */
  return smtp_send_mail_alloced(s);
}

pub fn
smtp_send_body_data_handler(s: &mut smtp_session, pcb: &mut AlTcpPcb)
{
  bdh: &mut smtp_bodydh_state;
  res: i32 = 0, ret;
  LWIP_ASSERT("s != NULL", s != None);
  bdh = s.bodydh;
  LWIP_ASSERT("bodydh != NULL", bdh != None);

  /* resume any leftovers from prior memory constraints */
  if (s.body_len) {
//    LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_send_body_data_handler: resume\n"));
    if((res = smtp_send_bodyh_data(pcb, (const char **)&s.body, &s.body_len))
        != BDHALLDATASENT) {
      s.body_sent = s.body_len - 1;
      return;
    }
  }
  ret = res;
  /* all data on buffer has been queued, resume execution */
  if (bdh.state == BDH_SENDING) {
//    LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_send_body_data_handler: run\n"));
    loop {
      ret |= res; /* remember if we once queued something to send */
      bdh.exposed.length = 0;
      if (bdh.callback_fn(s.callback_arg, &bdh.exposed) == BDH_DONE) {
        bdh.state = BDH_STOP;
      }
      s.body = bdh.exposed.buffer;
      s.body_len = bdh.exposed.length;
//      LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_send_body_data_handler: trying to send %u bytes\n", s.body_len));
    } while (s.body_len &&
            ((res = smtp_send_bodyh_data(pcb, (const char **)&s.body, &s.body_len)) == BDHALLDATASENT)
            && (bdh.state != BDH_STOP));
  }
  if ((bdh.state != BDH_SENDING) && (ret != BDHSOMEDATASENT)) {
//    LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_send_body_data_handler: stop\n"));
    s.body_sent = s.body_len;
  } else {
//    LWIP_DEBUGF(SMTP_DEBUG_TRACE, ("smtp_send_body_data_handler: pause\n"));
    s.body_sent = s.body_len - 1;
  }
}



