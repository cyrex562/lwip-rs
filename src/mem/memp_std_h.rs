/*
 * @file
 * lwIP internal memory pools (do not use in application code)
 * This file is deliberately included multiple times: once with empty
 * definition of LWIP_MEMPOOL() to handle all includes and multiple times
 * to build up various lists of mem pools.
 */

/*
 * SETUP: Make sure we define everything we will need.
 *
 * We have create three types of pools:
 *   1) MEMPOOL - standard pools
 *   2) MALLOC_MEMPOOL - to be used by mem_malloc in mem.c
 *   3) PBUF_MEMPOOL - a mempool of pbuf's, so include space for the pbuf struct
 *
 * If the include'r doesn't require any special treatment of each of the types
 * above, then will declare #2 & #3 to be just standard mempools.
 */

/* This treats "malloc pools" just like any other pool.
The pools are a little bigger to provide 'size' as the amount of user data. */
// #define LWIP_MALLOC_MEMPOOL(num, size) /* LWIP_MEMPOOL(POOL_##size, num, (size + LWIP_MEM_ALIGN_SIZE(sizeof(memp_malloc_helper))), "MALLOC_"#size) */
// #define LWIP_MALLOC_MEMPOOL_START
// #define LWIP_MALLOC_MEMPOOL_END

/* This treats "pbuf pools" just like any other pool.
 * Allocates buffers for a pbuf struct AND a payload size */
// #define LWIP_PBUF_MEMPOOL(name, num, payload, desc) /* LWIP_MEMPOOL(name, num, (LWIP_MEM_ALIGN_SIZE(sizeof(pbuf)) + LWIP_MEM_ALIGN_SIZE(payload)), desc) */
/*
 * A list of internal pools used by LWIP.
 *
 * /* LWIP_MEMPOOL(pool_name, number_elements, element_size, pool_description) */
 *     creates a pool name MEMP_pool_name. description is used in stats.c
 */

/* LWIP_MEMPOOL(RAW_PCB,        MEMP_NUM_RAW_PCB,         sizeof(raw_pcb),        "RAW_PCB") */

/* LWIP_MEMPOOL(UDP_PCB,        MEMP_NUM_UDP_PCB,         sizeof(udp_pcb),        "UDP_PCB") */

/* LWIP_MEMPOOL(TCP_PCB,        MEMP_NUM_TCP_PCB,         sizeof(tcp_pcb),        "TCP_PCB") */
/* LWIP_MEMPOOL(TCP_PCB_LISTEN, MEMP_NUM_TCP_PCB_LISTEN,  sizeof(tcp_pcb_listen), "TCP_PCB_LISTEN") */
/* LWIP_MEMPOOL(TCP_SEG,        MEMP_NUM_TCP_SEG,         sizeof(tcp_seg),        "TCP_SEG") */

/* LWIP_MEMPOOL(ALTCP_PCB,      MEMP_NUM_ALTCP_PCB,       sizeof(AltcpPcb),      "ALTCP_PCB") */

/* LWIP_MEMPOOL(REASSDATA,      MEMP_NUM_REASSDATA,       sizeof(ip_reassdata),   "REASSDATA") */

/* LWIP_MEMPOOL(FRAG_PBUF,      MEMP_NUM_FRAG_PBUF,       sizeof(pbuf_custom_ref),"FRAG_PBUF") */

/* LWIP_MEMPOOL(NETBUF,         MEMP_NUM_NETBUF,          sizeof(netbuf),         "NETBUF") */
/* LWIP_MEMPOOL(NETCONN,        MEMP_NUM_NETCONN,         sizeof(netconn),        "NETCONN") */

/* LWIP_MEMPOOL(TCPIP_MSG_API,  MEMP_NUM_TCPIP_MSG_API,   sizeof(tcpip_msg),      "TCPIP_MSG_API") */

/* LWIP_MEMPOOL(API_MSG,        MEMP_NUM_API_MSG,         sizeof(api_msg),        "API_MSG") */

/* LWIP_MEMPOOL(DNS_API_MSG,    MEMP_NUM_DNS_API_MSG,     sizeof(dns_api_msg),    "DNS_API_MSG") */

/* LWIP_MEMPOOL(SOCKET_SETGETSOCKOPT_DATA, MEMP_NUM_SOCKET_SETGETSOCKOPT_DATA, sizeof(lwip_setgetsockopt_data), "SOCKET_SETGETSOCKOPT_DATA") */

/* LWIP_MEMPOOL(SELECT_CB,      MEMP_NUM_SELECT_CB,       sizeof(LwipSelectCallback), "SELECT_CB") */

/* LWIP_MEMPOOL(NETIFAPI_MSG,   MEMP_NUM_NETIFAPI_MSG,    sizeof(NetIfcapi_msg),   "NETIFAPI_MSG") */

/* LWIP_MEMPOOL(TCPIP_MSG_INPKT,MEMP_NUM_TCPIP_MSG_INPKT, sizeof(tcpip_msg),      "TCPIP_MSG_INPKT") */

/* LWIP_MEMPOOL(ARP_QUEUE,      MEMP_NUM_ARP_QUEUE,       sizeof(etharp_q_entry), "ARP_QUEUE") */

/* LWIP_MEMPOOL(IGMP_GROUP,     MEMP_NUM_IGMP_GROUP,      sizeof(igmp_group),     "IGMP_GROUP") */

/* LWIP_MEMPOOL(SYS_TIMEOUT,    MEMP_NUM_SYS_TIMEOUT,     sizeof(sys_timeo),      "SYS_TIMEOUT") */

/* LWIP_MEMPOOL(NETDB,          MEMP_NUM_NETDB,           NETDB_ELEM_SIZE,               "NETDB") */

/* LWIP_MEMPOOL(LOCALHOSTLIST,  MEMP_NUM_LOCALHOSTLIST,   LOCALHOSTLIST_ELEM_SIZE,       "LOCALHOSTLIST") */

/* LWIP_MEMPOOL(ND6_QUEUE,      MEMP_NUM_ND6_QUEUE,       sizeof(nd6_q_entry),    "ND6_QUEUE") */

/* LWIP_MEMPOOL(IP6_REASSDATA,  MEMP_NUM_REASSDATA,       sizeof(ip6_reassdata),  "IP6_REASSDATA") */

/* LWIP_MEMPOOL(MLD6_GROUP,     MEMP_NUM_MLD6_GROUP,      sizeof(mld_group),      "MLD6_GROUP") */

/*
 * A list of pools of pbuf's used by LWIP.
 *
 * LWIP_PBUF_MEMPOOL(pool_name, number_elements, pbuf_payload_size, pool_description)
 *     creates a pool name MEMP_pool_name. description is used in stats.c
 *     This allocates enough space for the pbuf struct and a payload.
 *     (Example: PacketBuffer_payload_size=0 allocates only size for the struct)
 */
/* LWIP_MEMPOOL(PBUF,           MEMP_NUM_PBUF,            sizeof(pbuf),           "PBUF_REF/ROM") */
// LWIP_PBUF_MEMPOOL(PBUF_POOL, PBUF_POOL_SIZE,           PBUF_POOL_BUFSIZE,             "PBUF_POOL")

/*
 * Allow for user-defined pools; this must be explicitly set in lwipopts.h
 * since the default is to NOT look for lwippools.h
 */

/*
 * REQUIRED CLEANUP: Clear up so we don't get "multiply defined" error later
 * (#undef is ignored for something that is not defined)
 */
// #undef LWIP_MEMPOOL
// #undef LWIP_MALLOC_MEMPOOL
// #undef LWIP_MALLOC_MEMPOOL_START
// #undef LWIP_MALLOC_MEMPOOL_END
// #undef LWIP_PBUF_MEMPOOL
