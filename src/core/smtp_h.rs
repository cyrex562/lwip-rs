
// #define LWIP_HDR_APPS_SMTP_H









/* The default TCP port used for SMTP */
pub const SMTP_DEFAULT_PORT: u32 = LWIP_IANA_PORT_SMTP;
/* The default TCP port used for SMTPS */
pub const SMTPS_DEFAULT_PORT: u32 = LWIP_IANA_PORT_SMTPS;

/* Email successfully sent */
pub const SMTP_RESULT_OK: u32 = 0;
/* Unknown error */
pub const SMTP_RESULT_ERR_UNKNOWN: u32 = 1; 
/* Connection to server failed */
pub const SMTP_RESULT_ERR_CONNECT: u32 = 2; 
/* Failed to resolve server hostname */
pub const SMTP_RESULT_ERR_HOSTNAME: u32 = 3; 
/* Connection unexpectedly closed by remote server */
pub const SMTP_RESULT_ERR_CLOSED: u32 = 4; 
/* Connection timed out (server didn't respond in time) */
pub const SMTP_RESULT_ERR_TIMEOUT: u32 = 5; 
/* Server responded with an unknown response code */
pub const SMTP_RESULT_ERR_SVR_RESP: u32 = 6; 
/* Out of resources locally */
pub const SMTP_RESULT_ERR_MEM: u32 = 7; 

/* Prototype of an smtp callback function
 *
 * @param arg argument specified when initiating the email
 * @param smtp_result result of the mail transfer (see defines SMTP_RESULT_*)
 * @param srv_err if aborted by the server, this contains the error code received
 * @param err an error returned by internal lwip functions, can help to specify
 *            the source of the error but must not necessarily be != ERR_OK
 */
type smtp_result_fn =fn(arg: &mut Vec<u8>, smtp_result: u8, srv_err: u16, err: err_t);

/* This structure is used as argument for smtp_send_mail_int(),
 * which in turn can be used with tcpip_callback() to send mail
 * from interrupt context, e.g. like this:
 *    req: &mut smtp_send_request; (to be filled)
 *    tcpip_try_callback(smtp_send_mail_int, req);
 *
 * For member description, see parameter description of smtp_send_mail().
 * When using with tcpip_callback, this structure has to stay allocated
 * (e.g. using mem_malloc/mem_free) until its 'callback_fn' is called.
 */
pub struct smtp_send_request {
  pub from: String,
 pub to: String,
 pub subject: String,
 pub body: String,
  pub callback_fn: smtp_result_fn,
  pub callback_arg: Vec<u8>,
  /* If this is != 0, data is *not* copied into an extra buffer
   * but used from the pointers supplied in this struct.
   * This means less memory usage, but data must stay untouched until
   * the callback function is called. */
  pub static_data: u8,
}

pub const SMTP_BODYDH_BUFFER_SIZE: u32 = 256; 

pub struct smtp_bodydh {
  pub state: u16,
  pub length: u16, /* Length of content in buffer */
  pub buffer: String, /* buffer for generated content */
  pub user: Vec<u8>,
}

pub enum bdh_retvals_e {
  BDH_DONE = 0,
  BDH_WORKING
}

/* Prototype of an smtp body callback function
 * It receives a struct smtp_bodydh, and a buffer to write data,
 * must return BDH_WORKING to be called again and BDH_DONE when
 * it has finished processing. This one tries to fill one TCP buffer with
 * data, your function will be repeatedly called until that happens; so if you
 * know you'll be taking too long to serve your request, pause once in a while
 * by writing length=0 to avoid hogging system resources
 *
 * @param arg argument specified when initiating the email
 * @param smtp_bodydh state handling + buffer structure
 */
type smtp_bodycback_fn = fn(arg: &mut Vec<u8>, bodydh: &mut smtp_bodydh) -> i32;

// pub fn  smtp_send_mail_bodycback(from: &String,  char* to,  char* subject,
//                      smtp_bodycback_fn bodycback_fn, smtp_result_fn callback_fn, void* callback_arg);




// pub fn  smtp_set_server_addr( char* server);
// pub fn  smtp_set_server_port(port: u16);

// struct altcp_tls_config;
// pub fn  smtp_set_tls_config(tls_config: &mut altcp_tls_config);

// pub fn  smtp_set_auth( char* username,  char* pass);
// pub fn  smtp_send_mail(from: &String,  char* to,  char* subject,  char* body,
                    //  smtp_result_fn callback_fn, void* callback_arg);
// pub fn  smtp_send_mail_static(from: &String,  char* to,  char* subject,  char* body,
//                      smtp_result_fn callback_fn, void* callback_arg);
// pub fn  smtp_send_mail_int(arg: &mut Vec<u8>);

// const char* smtp_result_str(smtp_result: u8);







