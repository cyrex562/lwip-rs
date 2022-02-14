use crate::ip::ip4_addr::{IPADDR_ANY, IPADDR_BROADCAST, IPADDR_LOOPBACK, IPADDR_NONE};

//  255.255.255.255
// pub const INADDR_NONE: u32 = IPADDR_NONE;
//  127.0.0.1
// pub const INADDR_LOOPBACK: u32 = IPADDR_LOOPBACK;
//  0.0.0.0
// pub const INADDR_ANY: u32 = IPADDR_ANY;
//  255.255.255.255
// pub const INADDR_BROADCAST: u32 = IPADDR_BROADCAST;

/* This macro can be used to initialize a variable of type struct in6_addr
to the IPv6 wildcard address. */
// pub const IN6ADDR_ANY_INIT {{{0,0,0,0}}}
/* This macro can be used to initialize a variable of type struct in6_addr
to the IPv6 loopback address. */
// #define IN6ADDR_LOOPBACK_INIT {{{0,0,0,PP_HTONL(1)}}}
//  This variable is initialized by the system to contain the wildcard IPv6 address.
// extern const struct in6_addr in6addr_any;

/* Definitions of the bits in an (IPv4) Internet address integer.

On subnets, host and network parts are found according to
the subnet mask, not these masks.  */
// #define IN_CLASSA(a)        IP_CLASSA(a)
// pub const IN_CLASSA_NET: u32 = IP_CLASSA_NET;
// pub const IN_CLASSA_NSHIFT: u32 = IP_CLASSA_NSHIFT;
// pub const IN_CLASSA_HOST: u32 = IP_CLASSA_HOST;
// pub const IN_CLASSA_MAX: u32 = IP_CLASSA_MAX;

// #define IN_CLASSB(b)        IP_CLASSB(b)
// pub const IN_CLASSB_NET: u32 = IP_CLASSB_NET;
// pub const IN_CLASSB_NSHIFT: u32 = IP_CLASSB_NSHIFT;
// pub const IN_CLASSB_HOST: u32 = IP_CLASSB_HOST;
// pub const IN_CLASSB_MAX: u32 = IP_CLASSB_MAX;

// #define IN_CLASSC(c)        IP_CLASSC(c)
// pub const IN_CLASSC_NET: u32 = IP_CLASSC_NET;
// pub const IN_CLASSC_NSHIFT: u32 = IP_CLASSC_NSHIFT;
// pub const IN_CLASSC_HOST: u32 = IP_CLASSC_HOST;
// pub const IN_CLASSC_MAX: u32 = IP_CLASSC_MAX;

// #define IN_CLASSD(d)        IP_CLASSD(d)
// pub const IN_CLASSD_NET: u32 = IP_CLASSD_NET;     //  These ones aren't really
// pub const IN_CLASSD_NSHIFT: u32 = IP_CLASSD_NSHIFT;  //    net and host fields, but
// pub const IN_CLASSD_HOST: u32 = IP_CLASSD_HOST;    //    routing needn't know.
// pub const IN_CLASSD_MAX: u32 = IP_CLASSD_MAX;

// #define IN_MULTICAST(a)     IP_MULTICAST(a)

// #define IN_EXPERIMENTAL(a)  IP_EXPERIMENTAL(a)
// #define IN_BADCLASS(a)      IP_BADCLASS(a)

// pub const IN_LOOPBACKNET: u32 = IP_LOOPBACKNET;

// pub const INET_ADDRSTRLEN: u32 = IP4ADDR_STRLEN_MAX;

// pub const INET6_ADDRSTRLEN: u32 = IP6ADDR_STRLEN_MAX;

// #define inet_addr_from_ip4addr(target_inaddr, source_ipaddr) ((target_inaddr).s_addr = ip4_addr_get_u32(source_ipaddr))

// #define inet_addr_to_ip4addr(target_ipaddr, source_inaddr)   (ip4_addr_set_u32(target_ipaddr, (source_inaddr).s_addr))

//  directly map this to the lwip internal functions
// #define inet_addr(cp)                   ipaddr_addr(cp)
// #define inet_aton(cp, addr)             ip4addr_aton(cp, addr)
// #define inet_ntoa(addr)                 ip4addr_ntoa(( ip4_addr*)&(addr))
// #define inet_ntoa_r(addr, buf, buflen)  ip4addr_ntoa_r(( ip4_addr*)&(addr), buf, buflen)

// #define inet6_addr_from_ip6addr(target_in6addr, source_ip6addr) {(target_in6addr).un.u32_addr[0] = (source_ip6addr).addr[0]; \
//                                                                  (target_in6addr).un.u32_addr[1] = (source_ip6addr).addr[1]; \
//                                                                  (target_in6addr).un.u32_addr[2] = (source_ip6addr).addr[2]; \
//                                                                  (target_in6addr).un.u32_addr[3] = (source_ip6addr).addr[3];}
// #define inet6_addr_to_ip6addr(target_ip6addr, source_in6addr)   {(target_ip6addr).addr[0] = (source_in6addr).un.u32_addr[0]; \
//                                                                  (target_ip6addr).addr[1] = (source_in6addr).un.u32_addr[1]; \
//                                                                  (target_ip6addr).addr[2] = (source_in6addr).un.u32_addr[2]; \
//                                                                  (target_ip6addr).addr[3] = (source_in6addr).un.u32_addr[3]; \
//                                                                  ip6_addr_clear_zone(target_ip6addr);}

// //  directly map this to the lwip internal functions
// #define inet6_aton(cp, addr)            ip6addr_aton(cp, addr)
// #define inet6_ntoa(addr)                ip6addr_ntoa(( ip6_addr_t*)&(addr))
// #define inet6_ntoa_r(addr, buf, buflen) ip6addr_ntoa_r(( ip6_addr_t*)&(addr), buf, buflen)
