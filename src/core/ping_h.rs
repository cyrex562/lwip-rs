
#define LWIP_PING_H



/**
 * PING_USE_SOCKETS: Set to 1 to use sockets, otherwise the raw api is used
 */

#define PING_USE_SOCKETS    LWIP_SOCKET


pub fn  ping_init(const ip_addr_t* ping_addr);


pub fn  ping_send_now(void);



