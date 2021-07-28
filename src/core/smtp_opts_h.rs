
#define LWIP_HDR_APPS_SMTP_OPTS_H




extern "C" {

    
/*
 * @defgroup smtp_opts Options
 * @ingroup smtp
 * 
 * @{
 */
    
/* Set this to 1 to enable data handler callback on BODY */

pub const SMTP_BODYDH: u32 = 0;


/* SMTP_DEBUG: Enable debugging for SNTP. */

#define SMTP_DEBUG              LWIP_DBG_OFF


/* Maximum length reserved for server name including terminating 0 byte */

#define SMTP_MAX_SERVERNAME_LEN 256


/* Maximum length reserved for username */

#define SMTP_MAX_USERNAME_LEN   32


/* Maximum length reserved for password */

#define SMTP_MAX_PASS_LEN       32


/* Set this to 0 if you know the authentication data will not change
 * during the smtp session, which saves some heap space. */

#define SMTP_COPY_AUTHDATA      1


/* Set this to 0 to save some code space if you know for sure that all data
 * passed to this module conforms to the requirements in the SMTP RFC.
 * WARNING: use this with care!
 */

#define SMTP_CHECK_DATA         1


/* Set this to 1 to enable AUTH PLAIN support */

#define SMTP_SUPPORT_AUTH_PLAIN 1


/* Set this to 1 to enable AUTH LOGIN support */

#define SMTP_SUPPORT_AUTH_LOGIN 1


/* Memory allocation/deallocation can be overridden... */

#define SMTP_STATE_MALLOC(size)       mem_malloc(size)
#define SMTP_STATE_FREE(ptr)          mem_free(ptr)


/*
 * @}
 */


}




