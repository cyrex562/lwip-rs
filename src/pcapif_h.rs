





// #include "lwip/err.h"

/** Set to 1 to let rx use an own thread (only for NO_SYS==0).
 * If set to 0, ethernetif_poll is used to poll for packets.
 */

#define PCAPIF_RX_USE_THREAD !NO_SYS

APIF_RX_USE_THREAD && NO_SYS
#error "Can't create a dedicated RX thread with NO_SYS==1"


struct netif;

err_t pcapif_init    (struct netif *netif);
void  pcapif_shutdown(struct netif *netif);
#if !PCAPIF_RX_USE_THREAD
void  pcapif_poll    (struct netif *netif);
 /* !PCAPIF_RX_USE_THREAD */




 /* LWIP_PCAPIF_H */
