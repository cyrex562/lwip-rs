/**
 * @file
 * Exports Private lwIP MIB 
 */


#define LWIP_HDR_PRIVATE_MIB_H






extern "C" {


/* export MIB */
extern const struct snmp_mib mib_private;

pub fn  lwip_privmib_init(void);


}



