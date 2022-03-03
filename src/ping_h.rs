


// #include "lwip/ip_addr.h"

/**
 * PING_USE_SOCKETS: Set to 1 to use sockets, otherwise the raw api is used
 */

pub const PING_USE_SOCKETS: u32 = LWIP_SOCKET;


void ping_init(const ip_addr_t* ping_addr);

#if !PING_USE_SOCKETS
void ping_send_now(void);
 /* !PING_USE_SOCKETS */

 /* LWIP_PING_H */
