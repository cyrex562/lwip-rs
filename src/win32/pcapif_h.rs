
// #define LWIP_PCAPIF_H







/* Set to 1 to let rx use an own thread (only for NO_SYS==0).
 * If set to 0, ethernetif_poll is used to poll for packets.
 */

#define PCAPIF_RX_USE_THREAD !NO_SYS


#error "Can't create a dedicated RX thread with NO_SYS==1"


struct netif;

pub fn  pcapif_init    (netif: &mut netif);
pub fn   pcapif_shutdown(netif: &mut netif);

pub fn   pcapif_poll    (netif: &mut netif);



}



