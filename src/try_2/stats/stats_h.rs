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

//

pub const LWIP_STATS_LARGE: u32 = 0;

// pub const STAT_COUNTER: u32 = u32;
// pub const STAT_COUNTER_F: u32 = U32_F;

// pub const STAT_COUNTER: u32 = u16;
// pub const STAT_COUNTER_F: u32 = U16_F;

//  Protocol related stats
pub struct stats_proto {
    pub xmit: u32,    //  Transmitted packets.
    pub recv: u32,    //  Received packets.
    pub fw: u32,      //  Forwarded packets.
    pub drop: u32,    //  Dropped packets.
    pub chkerr: u32,  //  Checksum error.
    pub lenerr: u32,  //  Invalid length error.
    pub memerr: u32,  //  Out of memory error.
    pub rterr: u32,   //  Routing error.
    pub proterr: u32, //  Protocol error.
    pub opterr: u32,  //  Error in options.
    pub err: u32,     //  Misc error.
    pub cachehit: u32,
}

//  IGMP stats
pub struct stats_igmp {
    pub xmit: u32,       //  Transmitted packets.
    pub recv: u32,       //  Received packets.
    pub drop: u32,       //  Dropped packets.
    pub chkerr: u32,     //  Checksum error.
    pub lenerr: u32,     //  Invalid length error.
    pub memerr: u32,     //  Out of memory error.
    pub proterr: u32,    //  Protocol error.
    pub rx_v1: u32,      //  Received v1 frames.
    pub rx_group: u32,   //  Received group-specific queries.
    pub rx_general: u32, //  Received general queries.
    pub rx_report: u32,  //  Received reports.
    pub tx_join: u32,    //  Sent joins.
    pub tx_leave: u32,   //  Sent leaves.
    pub tx_report: u32,  //  Sent reports.
}

//  Memory stats
pub struct stats_mem {
    pub name: String,

    pub err: u32,
    pub mem_avail: usize,
    pub mem_used: usize,
    pub mem_max: usize,
    pub illegal: u32,
}

//  System element stats
pub struct stats_syselem {
    pub used: u32,
    pub max: u32,
    pub err: u32,
}

//  System stats
pub struct stats_sys {
    pub sem: stats_syselem,
    pub mutex: stats_syselem,
    pub mbox: stats_syselem,
}

//  SNMP MIB2 stats
struct stats_mib2 {
    //  IP
    pub ipinhdrerrors: u32,
    pub ipinaddrerrors: u32,
    pub ipinunknownprotos: u32,
    pub ipindiscards: u32,
    pub ipindelivers: u32,
    pub ipoutrequests: u32,
    pub ipoutdiscards: u32,
    pub ipoutnoroutes: u32,
    pub ipreasmoks: u32,
    pub ipreasmfails: u32,
    pub ipfragoks: u32,
    pub ipfragfails: u32,
    pub ipfragcreates: u32,
    pub ipreasmreqds: u32,
    pub ipforwdatagrams: u32,
    pub ipinreceives: u32,

    //  TCP
    pub tcpactiveopens: u32,
    pub tcppassiveopens: u32,
    pub tcpattemptfails: u32,
    pub tcpestabresets: u32,
    pub tcpoutsegs: u32,
    pub tcpretranssegs: u32,
    pub tcpinsegs: u32,
    pub tcpinerrs: u32,
    pub tcpoutrsts: u32,

    //  UDP
    pub udpindatagrams: u32,
    pub udpnoports: u32,
    pub udpinerrors: u32,
    pub udpoutdatagrams: u32,

    //  ICMP
    pub icmpinmsgs: u32,
    pub icmpinerrors: u32,
    pub icmpindestunreachs: u32,
    pub icmpintimeexcds: u32,
    pub icmpinparmprobs: u32,
    pub icmpinsrcquenchs: u32,
    pub icmpinredirects: u32,
    pub icmpinechos: u32,
    pub icmpinechoreps: u32,
    pub icmpintimestamps: u32,
    pub icmpintimestampreps: u32,
    pub icmpinaddrmasks: u32,
    pub icmpinaddrmaskreps: u32,
    pub icmpoutmsgs: u32,
    pub icmpouterrors: u32,
    pub icmpoutdestunreachs: u32,
    pub icmpouttimeexcds: u32,
    pub icmpoutechos: u32,
    //  can be incremented by user application ('ping')  pub icmpinmsgs: u32,
    pub icmpoutechoreps: u32,
}

/*
 * @ingroup netif_mib2
 * SNMP MIB2 interface stats
 */
struct stats_mib2_netif_ctrs {
    //  The total number of octets received on the interface, including framing characters
    pub ifinoctets: u32,
    /* The number of packets, delivered by this sub-layer to a higher (sub-)layer, which were
     * not addressed to a multicast or broadcast address at this sub-layer */
    pub ifinucastpkts: u32,
    /* The number of packets, delivered by this sub-layer to a higher (sub-)layer, which were
     * addressed to a multicast or broadcast address at this sub-layer */
    pub ifinnucastpkts: u32,
    /* The number of inbound packets which were chosen to be discarded even though no errors had
     * been detected to prevent their being deliverable to a higher-layer protocol. One possible
     * reason for discarding such a packet could be to free up buffer space */
    pub ifindiscards: u32,
    /* For packet-oriented interfaces, the number of inbound packets that contained errors
     * preventing them from being deliverable to a higher-layer protocol.  For character-
     * oriented or fixed-length interfaces, the number of inbound transmission units that
     * contained errors preventing them from being deliverable to a higher-layer protocol. */
    pub ifinerrors: u32,
    /* For packet-oriented interfaces, the number of packets received via the interface which
     * were discarded because of an unknown or unsupported protocol.  For character-oriented
     * or fixed-length interfaces that support protocol multiplexing the number of transmission
     * units received via the interface which were discarded because of an unknown or unsupported
     * protocol. For any interface that does not support protocol multiplexing, this counter will
     * always be 0 */
    pub ifinunknownprotos: u32,
    //  The total number of octets transmitted out of the interface, including framing characters.
    pub ifoutoctets: u32,
    /* The total number of packets that higher-level protocols requested be transmitted, and
     * which were not addressed to a multicast or broadcast address at this sub-layer, including
     * those that were discarded or not sent. */
    pub ifoutucastpkts: u32,
    /* The total number of packets that higher-level protocols requested be transmitted, and which
     * were addressed to a multicast or broadcast address at this sub-layer, including
     * those that were discarded or not sent. */
    pub ifoutnucastpkts: u32,
    /* The number of outbound packets which were chosen to be discarded even though no errors had
     * been detected to prevent their being transmitted.  One possible reason for discarding
     * such a packet could be to free up buffer space. */
    pub ifoutdiscards: u32,
    /* For packet-oriented interfaces, the number of outbound packets that could not be transmitted
     * because of errors. For character-oriented or fixed-length interfaces, the number of outbound
     * transmission units that could not be transmitted because of errors. */
    pub ifouterrors: u32,
}

//  lwIP stats container
struct stats_ {
    //  Link level
    pub link: stats_proto,

    //  ARP
    pub etharp: stats_proto,

    //  Fragmentation
    pub ip_frag: stats_proto,

    //  IP
    pub ip: stats_proto,

    //  ICMP
    pub icmp: stats_proto,

    //  IGMP
    pub igmp: stats_igmp,

    //  UDP
    pub udp: stats_proto,

    //  TCP
    pub tcp: stats_proto,

    //  Heap
    pub mem: stats_mem,

    //  Internal memory pools
    pub memp: [stats_mem; MEMP_MAX],

    //  System
    pub sys: stats_sys,

    //  IPv6
    pub ip6: stats_proto,

    //  ICMP6
    pub icmp6: stats_proto,

    //  IPv6 fragmentation
    pub ip6_frag: stats_proto,

    //  Multicast listener discovery
    pub mld6: stats_igmp,

    //  Neighbor discovery
    pub nd6: stats_proto,

    //  SNMP MIB2
    pub mib2: stats_mib2,
}

//  Global variable containing lwIP internal statistics. Add this to your debugger's watchlist.
// extern struct stats_ lwip_stats;

//  Init statistics
// pub fn  stats_init();

// #define STATS_INC(x) += 1lwip_stats.x
// #define STATS_DEC(x) --lwip_stats.x
// #define STATS_INC_USED(x, y, type) loop { lwip_stats.x.used = (type)(lwip_stats.x.used + y); \
//                                 if (lwip_stats.x.max < lwip_stats.x.used) { \
//                                     lwip_stats.x.max = lwip_stats.x.used; \
//                                 } \
//                              } while(0)
// #define STATS_GET(x) lwip_stats.x
//  LWIP_STATS
// #define stats_init()
// #define STATS_INC(x)
// #define STATS_DEC(x)
// #define STATS_INC_USED(x, y, type)

// #define TCP_STATS_INC(x) STATS_INC(x)
// #define TCP_STATS_DISPLAY() stats_display_proto(&lwip_stats.tcp, "TCP")

// #define TCP_STATS_INC(x)
// #define TCP_STATS_DISPLAY()

// #define UDP_STATS_INC(x) STATS_INC(x)
// #define UDP_STATS_DISPLAY() stats_display_proto(&lwip_stats.udp, "UDP")

// #define UDP_STATS_INC(x)
// #define UDP_STATS_DISPLAY()

// #define ICMP_STATS_INC(x) STATS_INC(x)
// #define ICMP_STATS_DISPLAY() stats_display_proto(&lwip_stats.icmp, "ICMP")

// #define ICMP_STATS_INC(x)
// #define ICMP_STATS_DISPLAY()

// #define IGMP_STATS_INC(x) STATS_INC(x)
// #define IGMP_STATS_DISPLAY() stats_display_igmp(&lwip_stats.igmp, "IGMP")

// #define IGMP_STATS_INC(x)
// #define IGMP_STATS_DISPLAY()

// #define IP_STATS_INC(x) STATS_INC(x)
// #define IP_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip, "IP")

// #define IP_STATS_INC(x)
// #define IP_STATS_DISPLAY()

// #define IPFRAG_STATS_INC(x) STATS_INC(x)
// #define IPFRAG_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip_frag, "ip_frag")

// #define IPFRAG_STATS_INC(x)
// #define IPFRAG_STATS_DISPLAY()

// #define ETHARP_STATS_INC(x) STATS_INC(x)
// #define ETHARP_STATS_DISPLAY() stats_display_proto(&lwip_stats.etharp, "ETHARP")

// #define ETHARP_STATS_INC(x)
// #define ETHARP_STATS_DISPLAY()

// #define LINK_STATS_INC(x) STATS_INC(x)
// #define LINK_STATS_DISPLAY() stats_display_proto(&lwip_stats.link, "LINK")

// #define LINK_STATS_INC(x)
// #define LINK_STATS_DISPLAY()

// #define MEM_STATS_AVAIL(x, y) lwip_stats.mem.x = y
// #define MEM_STATS_INC(x) STATS_INC(mem.x)
// #define MEM_STATS_INC_USED(x, y) STATS_INC_USED(mem, y, mem_usize)
// #define MEM_STATS_DEC_USED(x, y) lwip_stats.mem.x = ((lwip_stats.mem.x) - (y))
// #define MEM_STATS_DISPLAY() stats_display_mem(&lwip_stats.mem, "HEAP")

// #define MEM_STATS_AVAIL(x, y)
// #define MEM_STATS_INC(x)
// #define MEM_STATS_INC_USED(x, y)
// #define MEM_STATS_DEC_USED(x, y)
// #define MEM_STATS_DISPLAY()

//
// #define MEMP_STATS_DEC(x, i) STATS_DEC(memp[i].x)
// #define MEMP_STATS_DISPLAY(i) stats_display_memp(lwip_stats.memp[i], i)
// #define MEMP_STATS_GET(x, i) STATS_GET(memp[i].x)
//
// #define MEMP_STATS_DEC(x, i)
// #define MEMP_STATS_DISPLAY(i)
// #define MEMP_STATS_GET(x, i) 0

// #define SYS_STATS_INC(x) STATS_INC(sys.x)
// #define SYS_STATS_DEC(x) STATS_DEC(sys.x)
// #define SYS_STATS_INC_USED(x) STATS_INC_USED(sys.x, 1, STAT_COUNTER)
// #define SYS_STATS_DISPLAY() stats_display_sys(&lwip_stats.sys)

// #define SYS_STATS_INC(x)
// #define SYS_STATS_DEC(x)
// #define SYS_STATS_INC_USED(x)
// #define SYS_STATS_DISPLAY()

// #define IP6_STATS_INC(x) STATS_INC(x)
// #define IP6_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip6, "IPv6")

// #define IP6_STATS_INC(x)
// #define IP6_STATS_DISPLAY()

// #define ICMP6_STATS_INC(x) STATS_INC(x)
// #define ICMP6_STATS_DISPLAY() stats_display_proto(&lwip_stats.icmp6, "ICMPv6")

// #define ICMP6_STATS_INC(x)
// #define ICMP6_STATS_DISPLAY()

// #define IP6_FRAG_STATS_INC(x) STATS_INC(x)
// #define IP6_FRAG_STATS_DISPLAY() stats_display_proto(&lwip_stats.ip6_frag, "IPv6 FRAG")

// #define IP6_FRAG_STATS_INC(x)
// #define IP6_FRAG_STATS_DISPLAY()

// #define MLD6_STATS_INC(x) STATS_INC(x)
// #define MLD6_STATS_DISPLAY() stats_display_igmp(&lwip_stats.mld6, "MLDv1")

// #define MLD6_STATS_INC(x)
// #define MLD6_STATS_DISPLAY()

// #define ND6_STATS_INC(x) STATS_INC(x)
// #define ND6_STATS_DISPLAY() stats_display_proto(&lwip_stats.nd6, "ND")

// #define ND6_STATS_INC(x)
// #define ND6_STATS_DISPLAY()

// #define MIB2_STATS_INC(x) STATS_INC(x)

// #define MIB2_STATS_INC(x)

//  Display of statistics

// pub fn  stats_display();
// pub fn  stats_display_proto(proto: &mut stats_proto, name: &String);
// pub fn  stats_display_igmp(igmp: &mut stats_igmp, name: &String);
// pub fn  stats_display_mem(mem: &mut stats_mem, name: &String);
// pub fn  stats_display_memp(mem: &mut stats_mem, index: i32);
// pub fn  stats_display_sys(sys: &mut stats_sys);
//  //  LWIP_STATS_DISPLAY
// #define stats_display()
// #define stats_display_proto(proto, name)
// #define stats_display_igmp(igmp, name)
// #define stats_display_mem(mem, name)
// #define stats_display_memp(mem, index)
// #define stats_display_sys(sys)
