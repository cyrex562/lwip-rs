/*
 * @file
 * Statistics API (to be used from TCPIP thread)
 */

/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Adam Dunkels <adam@sics.se>
 *
 */

// #define LWIP_HDR_STATS_H













pub const LWIP_STATS_LARGE: u32 = 0;



#define STAT_COUNTER     u32
#define STAT_COUNTER_F   U32_F

#define STAT_COUNTER     u16
#define STAT_COUNTER_F   U16_F


/* Protocol related stats */
struct stats_proto {
  STAT_COUNTER xmit;             /* Transmitted packets. */
  STAT_COUNTER recv;             /* Received packets. */
  STAT_COUNTER fw;               /* Forwarded packets. */
  STAT_COUNTER drop;             /* Dropped packets. */
  STAT_COUNTER chkerr;           /* Checksum error. */
  STAT_COUNTER lenerr;           /* Invalid length error. */
  STAT_COUNTER memerr;           /* Out of memory error. */
  STAT_COUNTER rterr;            /* Routing error. */
  STAT_COUNTER proterr;          /* Protocol error. */
  STAT_COUNTER opterr;           /* Error in options. */
  STAT_COUNTER err;              /* Misc error. */
  STAT_COUNTER cachehit;
};

/* IGMP stats */
struct stats_igmp {
  STAT_COUNTER xmit;             /* Transmitted packets. */
  STAT_COUNTER recv;             /* Received packets. */
  STAT_COUNTER drop;             /* Dropped packets. */
  STAT_COUNTER chkerr;           /* Checksum error. */
  STAT_COUNTER lenerr;           /* Invalid length error. */
  STAT_COUNTER memerr;           /* Out of memory error. */
  STAT_COUNTER proterr;          /* Protocol error. */
  STAT_COUNTER rx_v1;            /* Received v1 frames. */
  STAT_COUNTER rx_group;         /* Received group-specific queries. */
  STAT_COUNTER rx_general;       /* Received general queries. */
  STAT_COUNTER rx_report;        /* Received reports. */
  STAT_COUNTER tx_join;          /* Sent joins. */
  STAT_COUNTER tx_leave;         /* Sent leaves. */
  STAT_COUNTER tx_report;        /* Sent reports. */
};

/* Memory stats */
struct stats_mem {

  let name: String;

  STAT_COUNTER err;
  let mem_avail: usize;
  let mem_used: usize;
  let mem_max: usize;
  STAT_COUNTER illegal;
};

/* System element stats */
struct stats_syselem {
  STAT_COUNTER used;
  STAT_COUNTER max;
  STAT_COUNTER err;
};

/* System stats */
struct stats_sys {
  let sem: stats_syselem;
  let mutex: stats_syselem;
  let mbox: stats_syselem;
};

/* SNMP MIB2 stats */
struct stats_mib2 {
  /* IP */
  let ipinhdrerrors: u32;
  let ipinaddrerrors: u32;
  let ipinunknownprotos: u32;
  let ipindiscards: u32;
  let ipindelivers: u32;
  let ipoutrequests: u32;
  let ipoutdiscards: u32;
  let ipoutnoroutes: u32;
  let ipreasmoks: u32;
  let ipreasmfails: u32;
  let ipfragoks: u32;
  let ipfragfails: u32;
  let ipfragcreates: u32;
  let ipreasmreqds: u32;
  let ipforwdatagrams: u32;
  let ipinreceives: u32;

  /* TCP */
  let tcpactiveopens: u32;
  let tcppassiveopens: u32;
  let tcpattemptfails: u32;
  let tcpestabresets: u32;
  let tcpoutsegs: u32;
  let tcpretranssegs: u32;
  let tcpinsegs: u32;
  let tcpinerrs: u32;
  let tcpoutrsts: u32;

  /* UDP */
  let udpindatagrams: u32;
  let udpnoports: u32;
  let udpinerrors: u32;
  let udpoutdatagrams: u32;

  /* ICMP */
  let icmpinmsgs: u32;
  let icmpinerrors: u32;
  let icmpindestunreachs: u32;
  let icmpintimeexcds: u32;
  let icmpinparmprobs: u32;
  let icmpinsrcquenchs: u32;
  let icmpinredirects: u32;
  let icmpinechos: u32;
  let icmpinechoreps: u32;
  let icmpintimestamps: u32;
  let icmpintimestampreps: u32;
  let icmpinaddrmasks: u32;
  let icmpinaddrmaskreps: u32;
  let icmpoutmsgs: u32;
  let icmpouterrors: u32;
  let icmpoutdestunreachs: u32;
  let icmpouttimeexcds: u32;
  let icmpoutechos: u32; /* can be incremented by user application ('ping') */  let icmpinmsgs: u32;
  let icmpoutechoreps: u32;
};

/*
 * @ingroup netif_mib2
 * SNMP MIB2 interface stats
 */
struct stats_mib2_netif_ctrs {
  /* The total number of octets received on the interface, including framing characters */
  let ifinoctets: u32;
  /* The number of packets, delivered by this sub-layer to a higher (sub-)layer, which were
   * not addressed to a multicast or broadcast address at this sub-layer */
  let ifinucastpkts: u32;
  /* The number of packets, delivered by this sub-layer to a higher (sub-)layer, which were
   * addressed to a multicast or broadcast address at this sub-layer */
  let ifinnucastpkts: u32;
  /* The number of inbound packets which were chosen to be discarded even though no errors had
   * been detected to prevent their being deliverable to a higher-layer protocol. One possible
   * reason for discarding such a packet could be to free up buffer space */
  let ifindiscards: u32;
  /* For packet-oriented interfaces, the number of inbound packets that contained errors
   * preventing them from being deliverable to a higher-layer protocol.  For character-
   * oriented or fixed-length interfaces, the number of inbound transmission units that
   * contained errors preventing them from being deliverable to a higher-layer protocol. */
  let ifinerrors: u32;
  /* For packet-oriented interfaces, the number of packets received via the interface which
   * were discarded because of an unknown or unsupported protocol.  For character-oriented
   * or fixed-length interfaces that support protocol multiplexing the number of transmission
   * units received via the interface which were discarded because of an unknown or unsupported
   * protocol. For any interface that does not support protocol multiplexing, this counter will
   * always be 0 */
  let ifinunknownprotos: u32;
  /* The total number of octets transmitted out of the interface, including framing characters. */
  let ifoutoctets: u32;
  /* The total number of packets that higher-level protocols requested be transmitted, and
   * which were not addressed to a multicast or broadcast address at this sub-layer, including
   * those that were discarded or not sent. */
  let ifoutucastpkts: u32;
  /* The total number of packets that higher-level protocols requested be transmitted, and which
   * were addressed to a multicast or broadcast address at this sub-layer, including
   * those that were discarded or not sent. */
  let ifoutnucastpkts: u32;
  /* The number of outbound packets which were chosen to be discarded even though no errors had
   * been detected to prevent their being transmitted.  One possible reason for discarding
   * such a packet could be to free up buffer space. */
  let ifoutdiscards: u32;
  /* For packet-oriented interfaces, the number of outbound packets that could not be transmitted
   * because of errors. For character-oriented or fixed-length interfaces, the number of outbound
   * transmission units that could not be transmitted because of errors. */
  let ifouterrors: u32;
};

/* lwIP stats container */
struct stats_ {

  /* Link level */
  let link: stats_proto;


  /* ARP */
  let etharp: stats_proto;


  /* Fragmentation */
  let ip_frag: stats_proto;


  /* IP */
  let ip: stats_proto;


  /* ICMP */
  let icmp: stats_proto;


  /* IGMP */
  let igmp: stats_igmp;


  /* UDP */
  let udp: stats_proto;


  /* TCP */
  let tcp: stats_proto;


  /* Heap */
  let mem: stats_mem;


  /* Internal memory pools */
  memp: &mut stats_mem[MEMP_MAX];


  /* System */
  let sys: stats_sys;


  /* IPv6 */
  let ip6: stats_proto;


  /* ICMP6 */
  let icmp6: stats_proto;


  /* IPv6 fragmentation */
  let ip6_frag: stats_proto;


  /* Multicast listener discovery */
  let mld6: stats_igmp;


  /* Neighbor discovery */
  let nd6: stats_proto;


  /* SNMP MIB2 */
  let mib2: stats_mib2;

};

/* Global variable containing lwIP internal statistics. Add this to your debugger's watchlist. */
extern struct stats_ lwip_stats;

/* Init statistics */
pub fn  stats_init();

#define STATS_INC(x) += 1lwip_stats.x
#define STATS_DEC(x) --lwip_stats.x
#define STATS_INC_USED(x, y, type) loop { lwip_stats.x.used = (type)(lwip_stats.x.used + y); \
                                if (lwip_stats.x.max < lwip_stats.x.used) { \
                                    lwip_stats.x.max = lwip_stats.x.used; \
                                } \
                             } while(0)
#define STATS_GET(x) lwip_stats.x
 /* LWIP_STATS */
#define stats_init()
#define STATS_INC(x)
#define STATS_DEC(x)
#define STATS_INC_USED(x, y, type)



#define TCP_STATS_INC(x) STATS_INC(x)
#define TCP_STATS_DISPLAY() stats_display_proto(&lwip_stats.tcp, "TCP")

#define TCP_STATS_INC(x)
#define TCP_STATS_DISPLAY()



#define UDP_STATS_INC(x) STATS_INC(x)
#define UDP_STATS_DISPLAY() stats_display_proto(&lwip_stats.udp, "UDP")

#define UDP_STATS_INC(x)
#define UDP_STATS_DISPLAY()



#define ICMP_STATS_INC(x) STATS_INC(x)
#define ICMP_STATS_DISPLAY() stats_display_proto(&lwip_stats.icmp, "ICMP")

#define ICMP_STATS_INC(x)
#define ICMP_STATS_DISPLAY()



#define IGMP_STATS_INC(x) STATS_INC(x)
#define IGMP_STATS_DISPLAY() stats_display_igmp(&lwip_stats.igmp, "IGMP")

#define IGMP_STATS_INC(x)
#define IGMP_STATS_DISPLAY()



#define IP_STATS_INC(x) STATS_INC(x)
#define IP_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip, "IP")

#define IP_STATS_INC(x)
#define IP_STATS_DISPLAY()



#define IPFRAG_STATS_INC(x) STATS_INC(x)
#define IPFRAG_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip_frag, "IP_FRAG")

#define IPFRAG_STATS_INC(x)
#define IPFRAG_STATS_DISPLAY()



#define ETHARP_STATS_INC(x) STATS_INC(x)
#define ETHARP_STATS_DISPLAY() stats_display_proto(&lwip_stats.etharp, "ETHARP")

#define ETHARP_STATS_INC(x)
#define ETHARP_STATS_DISPLAY()



#define LINK_STATS_INC(x) STATS_INC(x)
#define LINK_STATS_DISPLAY() stats_display_proto(&lwip_stats.link, "LINK")

#define LINK_STATS_INC(x)
#define LINK_STATS_DISPLAY()



#define MEM_STATS_AVAIL(x, y) lwip_stats.mem.x = y
#define MEM_STATS_INC(x) STATS_INC(mem.x)
#define MEM_STATS_INC_USED(x, y) STATS_INC_USED(mem, y, mem_usize)
#define MEM_STATS_DEC_USED(x, y) lwip_stats.mem.x = (mem_usize)((lwip_stats.mem.x) - (y))
#define MEM_STATS_DISPLAY() stats_display_mem(&lwip_stats.mem, "HEAP")

#define MEM_STATS_AVAIL(x, y)
#define MEM_STATS_INC(x)
#define MEM_STATS_INC_USED(x, y)
#define MEM_STATS_DEC_USED(x, y)
#define MEM_STATS_DISPLAY()


 #if MEMP_STATS
#define MEMP_STATS_DEC(x, i) STATS_DEC(memp[i].x)
#define MEMP_STATS_DISPLAY(i) stats_display_memp(lwip_stats.memp[i], i)
#define MEMP_STATS_GET(x, i) STATS_GET(memp[i].x)
 #else
#define MEMP_STATS_DEC(x, i)
#define MEMP_STATS_DISPLAY(i)
#define MEMP_STATS_GET(x, i) 0



#define SYS_STATS_INC(x) STATS_INC(sys.x)
#define SYS_STATS_DEC(x) STATS_DEC(sys.x)
#define SYS_STATS_INC_USED(x) STATS_INC_USED(sys.x, 1, STAT_COUNTER)
#define SYS_STATS_DISPLAY() stats_display_sys(&lwip_stats.sys)

#define SYS_STATS_INC(x)
#define SYS_STATS_DEC(x)
#define SYS_STATS_INC_USED(x)
#define SYS_STATS_DISPLAY()



#define IP6_STATS_INC(x) STATS_INC(x)
#define IP6_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip6, "IPv6")

#define IP6_STATS_INC(x)
#define IP6_STATS_DISPLAY()



#define ICMP6_STATS_INC(x) STATS_INC(x)
#define ICMP6_STATS_DISPLAY() stats_display_proto(&lwip_stats.icmp6, "ICMPv6")

#define ICMP6_STATS_INC(x)
#define ICMP6_STATS_DISPLAY()



#define IP6_FRAG_STATS_INC(x) STATS_INC(x)
#define IP6_FRAG_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip6_frag, "IPv6 FRAG")

#define IP6_FRAG_STATS_INC(x)
#define IP6_FRAG_STATS_DISPLAY()



#define MLD6_STATS_INC(x) STATS_INC(x)
#define MLD6_STATS_DISPLAY() stats_display_igmp(&lwip_stats.mld6, "MLDv1")

#define MLD6_STATS_INC(x)
#define MLD6_STATS_DISPLAY()



#define ND6_STATS_INC(x) STATS_INC(x)
#define ND6_STATS_DISPLAY() stats_display_proto(&lwip_stats.nd6, "ND")

#define ND6_STATS_INC(x)
#define ND6_STATS_DISPLAY()



#define MIB2_STATS_INC(x) STATS_INC(x)

#define MIB2_STATS_INC(x)


/* Display of statistics */

pub fn  stats_display();
pub fn  stats_display_proto(proto: &mut stats_proto, name: &String);
pub fn  stats_display_igmp(igmp: &mut stats_igmp, name: &String);
pub fn  stats_display_mem(mem: &mut stats_mem, name: &String);
pub fn  stats_display_memp(mem: &mut stats_mem, index: i32);
pub fn  stats_display_sys(sys: &mut stats_sys);
 /* LWIP_STATS_DISPLAY */
#define stats_display()
#define stats_display_proto(proto, name)
#define stats_display_igmp(igmp, name)
#define stats_display_mem(mem, name)
#define stats_display_memp(mem, index)
#define stats_display_sys(sys)



}



