


// #include "lwip/opt.h"




/**
 * @defgroup smtp_opts Options
 * @ingroup smtp
 *
 * @{
 */

/** Set this to 1 to enable data handler callback on BODY */

pub const SMTP_BODYDH: u32 = 0; /** SMTP_DEBUG: Enable debugging for SNTP. */

#define SMTP_DEBUG              LWIP_DBG_OFF


/** Maximum length reserved for server name including terminating 0 byte */

pub const SMTP_MAX_SERVERNAME_LEN: u32 = 256; /** Maximum length reserved for username */

pub const SMTP_MAX_USERNAME_LEN: u32 = 32; /** Maximum length reserved for password */

pub const SMTP_MAX_PASS_LEN: u32 = 32; /** Set this to 0 if you know the authentication data will not change
 * during the smtp session, which saves some heap space. */

pub const SMTP_COPY_AUTHDATA: u32 = 1; /** Set this to 0 to save some code space if you know for sure that all data
 * passed to this module conforms to the requirements in the SMTP RFC.
 * WARNING: use this with care!
 */

pub const SMTP_CHECK_DATA: u32 = 1; /** Set this to 1 to enable AUTH PLAIN support */

pub const SMTP_SUPPORT_AUTH_PLAIN: u32 = 1; /** Set this to 1 to enable AUTH LOGIN support */

pub const SMTP_SUPPORT_AUTH_LOGIN: u32 = 1; /* Memory allocation/deallocation can be overridden... */

#define SMTP_STATE_MALLOC(size)       mem_malloc(size)
#define SMTP_STATE_FREE(ptr)          mem_free(ptr)


/**
 * @}
 */




 /* SMTP_OPTS_H */
