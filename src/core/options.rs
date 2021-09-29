//!
//!  @file
//!
//!  lwIP Options Configuration
//!

//   Copyright (c) 2001-2004 Swedish Institute of Computer Science.
//   All rights reserved.
//  
//   Redistribution and use in source and binary forms, with or without modification,
//   are permitted provided that the following conditions are met:
//  
//   1. Redistributions of source code must retain the above copyright notice,
//      this list of conditions and the following disclaimer.
//   2. Redistributions in binary form must reproduce the above copyright notice,
//      this list of conditions and the following disclaimer in the documentation
//      and/or other materials provided with the distribution.
//   3. The name of the author may not be used to endorse or promote products
//      derived from this software without specific prior written permission.
//  
//   THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
//   WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
//   MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
//   SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
//   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
//   OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
//   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
//   CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
//   IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
//   OF SUCH DAMAGE.
//  
//   This file is part of the lwIP TCP/IP stack.
//  
//   Author: Adam Dunkels <adam@sics.se>

use crate::netif::netif_h::NETIF_NAMESIZE;
use crate::platform_support::arch::LWIP_MEM_ALIGN_SIZE;

#[derive(Clone, Debug, Default)]
pub struct LwipAlTcpTlsOptions {
    /* Set a session timeout in seconds for the basic session cache
     * ATTENTION: Using a session cache can lower security by reusing keys!
     */
    pub altcp_mbedtls_session_cache_timeout_seconds: u32, // =  0;
}

#[derive(Clone, Debug, Default)]
pub struct LwipMemOptions {
    /*
     * MEM_SIZE: the size of the heap memory. If the application will send
     * a lot of data that needs to be copied, this should be set high.
     */ pub MEM_SIZE: u32, // = 1600;

    /*
     * MEMP_OVERFLOW_CHECK: memp overflow protection reserves a configurable
     * amount of bytes before and after each memp element in every pool and fills
     * it with a prominent default value.
     *    MEMP_OVERFLOW_CHECK == 0 no checking
     *    MEMP_OVERFLOW_CHECK == 1 checks each element when it is freed
     *    MEMP_OVERFLOW_CHECK >= 2 checks each element in every pool every time
     *      memp_malloc() or memp_free() is called (useful but slow!)
     */

    pub MEMP_OVERFLOW_CHECK: u32, // = 0;

    /*
     * MEMP_SANITY_CHECK==1: run a sanity check after each memp_free() to make
     * sure that there are no cycles in the linked lists.
     */

    pub MEMP_SANITY_CHECK: u32, // = 0;

    /*
     * MEM_USE_POOLS==1: Use an alternative to malloc() by allocating from a set
     * of memory pools of various sizes. When mem_malloc is called, an element of
     * the smallest pool that can provide the length needed is returned.
     * To use this, MEMP_USE_CUSTOM_POOLS also has to be enabled.
     */

    pub MEM_USE_POOLS: u32, // = 0;

    /*
     * MEM_USE_POOLS_TRY_BIGGER_POOL==1: if one malloc-pool is empty, try the next
     * bigger pool - WARNING: THIS MIGHT WASTE MEMORY but it can make a system more
     * reliable. */

    pub MEM_USE_POOLS_TRY_BIGGER_POOL: u32, // = 0;


    /*
     * Set this to 1 if you want to free PBUF_RAM pbufs (or call mem_free()) from
     * interrupt context (or another context that doesn't allow waiting for a
     * semaphore).
     * If set to 1, mem_malloc will be protected by a semaphore and SYS_ARCH_PROTECT,
     * while mem_free will only use SYS_ARCH_PROTECT. mem_malloc SYS_ARCH_UNPROTECTs
     * with each loop so that mem_free can run.
     *
     * ATTENTION: As you can see from the above description, this leads to dis-/
     * enabling interrupts often, which can be slow! Also, on low memory, mem_malloc
     * can need longer.
     *
     * If you don't want that, at least for NO_SYS=0, you can still use the following
     * functions to enqueue a deallocation call which then runs in the tcpip_thread
     * context:
     * - pbuf_free_callback(p);
     * - mem_free_callback(m);
     */

    pub LWIP_ALLOW_MEM_FREE_FROM_OTHER_CONTEXT: u32, // = 0;

    pub MEMP_NUM_PBUF: u32, // = 16;

    /*
     * MEMP_NUM_RAW_PCB: Number of raw connection PCBs
     * (requires the LWIP_RAW option)
     */

    pub MEMP_NUM_RAW_PCB: u32, // = 4;

    /*
     * MEMP_NUM_UDP_PCB: the number of UDP protocol control blocks. One
     * per active UDP "connection".
     * (requires the LWIP_UDP option)
     */

    pub MEMP_NUM_UDP_PCB: u32, // = 4;

    /*
     * MEMP_NUM_TCP_PCB: the number of simultaneously active TCP connections.
     * (requires the LWIP_TCP option)
     */

    pub MEMP_NUM_TCP_PCB: u32, // = 5;

    /*
     * MEMP_NUM_TCP_PCB_LISTEN: the number of listening TCP connections.
     * (requires the LWIP_TCP option)
     */

    pub MEMP_NUM_TCP_PCB_LISTEN: u32, // = 8;

    /*
     * MEMP_NUM_TCP_SEG: the number of simultaneously queued TCP segments.
     * (requires the LWIP_TCP option)
     */

    pub MEMP_NUM_TCP_SEG: u32, // = 16;

    /*
     * MEMP_NUM_ALTCP_PCB: the number of simultaneously active abstract_tcp layer pcbs.
     * (requires the lwip_altcp option)
     * Connections with multiple layers require more than one AltcpPcb (e.g. TLS
     * over TCP requires 2 altcp_pcbs, one for TLS and one for TCP).
     */

    pub MEMP_NUM_ALTCP_PCB: u32, // = MEMP_NUM_TCP_PCB;

    /*
     * MEMP_NUM_REASSDATA: the number of IP packets simultaneously queued for
     * reassembly (whole packets, not fragments!)
     */

    pub MEMP_NUM_REASSDATA: u32, // = 5;

    /*
     * MEMP_NUM_FRAG_PBUF: the number of IP fragments simultaneously sent
     * (fragments, not whole packets!).
     * This is only used with LWIP_NETIF_TX_SINGLE_PBUF==0 and only has to be > 1
     * with DMA-enabled MACs where the packet is not yet sent when netif.output
     * returns.
     */

    pub MEMP_NUM_FRAG_PBUF: u32, // = 15;

    /*
     * MEMP_NUM_ARP_QUEUE: the number of simultaneously queued outgoing
     * packets (pbufs) that are waiting for an ARP request (to resolve
     * their destination address) to finish.
     * (requires the arp_queueing option)
     */

    pub MEMP_NUM_ARP_QUEUE: u32, // = 30;

    /*
     * MEMP_NUM_IGMP_GROUP: The number of multicast groups whose network interfaces
     * can be members at the same time (one per netif - allsystems group -, plus one
     * per netif membership).
     * (requires the LWIP_IGMP option)
     */

    pub MEMP_NUM_IGMP_GROUP: u32, // = 8;

    /*
     * The number of sys timeouts used by the core stack (not apps)
     * The default number of timeouts is calculated here for all enabled modules.
     */
// #define LWIP_NUM_SYS_TIMEOUT_INTERNAL   (LWIP_TCP + ip_reassembly + LWIP_ARP + (2*LWIP_DHCP) + LWIP_AUTOIP + LWIP_IGMP + LWIP_DNS + PPP_NUM_TIMEOUTS + (LWIP_IPV6 * (1 + LWIP_IPV6_REASS + LWIP_IPV6_MLD)))

    /*
     * MEMP_NUM_SYS_TIMEOUT: the number of simultaneously active timeouts.
     * The default number of timeouts is calculated here for all enabled modules.
     * The formula expects settings to be either '0' or '1'.
     */

    pub MEMP_NUM_SYS_TIMEOUT: u32, // = LWIP_NUM_SYS_TIMEOUT_INTERNAL;

    /*
     * MEMP_NUM_NETBUF: the number of struct netbufs.
     * (only needed if you use the sequential API, like api_lib.c)
     */

    pub MEMP_NUM_NETBUF: u32, // = 2;

    /*
     * MEMP_NUM_NETCONN: the number of struct netconns.
     * (only needed if you use the sequential API, like api_lib.c)
     */

    pub MEMP_NUM_NETCONN: u32, // = 4;

    /*
     * MEMP_NUM_SELECT_CB: the number of LwipSelectCallback.
     * (Only needed if you have LWIP_MPU_COMPATIBLE==1 and use the socket API.
     * In that case, you need one per thread calling lwip_select.)
     */

    pub MEMP_NUM_SELECT_CB: u32, // = 4;

    /*
     * MEMP_NUM_TCPIP_MSG_API: the number of struct tcpip_msg, which are used
     * for callback/timeout API communication.
     * (only needed if you use tcpip.c)
     */

    pub MEMP_NUM_TCPIP_MSG_API: u32, // = 8;

    /*
     * MEMP_NUM_TCPIP_MSG_INPKT: the number of struct tcpip_msg, which are used
     * for incoming packets.
     * (only needed if you use tcpip.c)
     */

    pub MEMP_NUM_TCPIP_MSG_INPKT: u32, // = 8;

    /*
     * MEMP_NUM_NETDB: the number of concurrently running lwip_addrinfo() calls
     * (before freeing the corresponding memory using lwip_freeaddrinfo()).
     */

    pub MEMP_NUM_NETDB: u32, // = 1;

    /*
     * MEMP_NUM_LOCALHOSTLIST: the number of host entries in the local host list
     * if DNS_LOCAL_HOSTLIST_IS_DYNAMIC==1.
     */

    pub MEMP_NUM_LOCALHOSTLIST: u32, // = 1;

    /*
     * PBUF_POOL_SIZE: the number of buffers in the pbuf pool.
     */

    pub PBUF_POOL_SIZE: u32, // = 16;

    /* MEMP_NUM_API_MSG: the number of concurrently active calls to various
     * socket, netconn, and tcpip functions
     */

    pub MEMP_NUM_API_MSG: u32, // = MEMP_NUM_TCPIP_MSG_API;

    /* MEMP_NUM_DNS_API_MSG: the number of concurrently active calls to netconn_gethostbyname
     */

    pub MEMP_NUM_DNS_API_MSG: u32, // = MEMP_NUM_TCPIP_MSG_API;

    /* MEMP_NUM_SOCKET_SETGETSOCKOPT_DATA: the number of concurrently active calls
     * to getsockopt/setsockopt
     */

    pub MEMP_NUM_SOCKET_SETGETSOCKOPT_DATA: u32, // = MEMP_NUM_TCPIP_MSG_API;

    /* MEMP_NUM_NETIFAPI_MSG: the number of concurrently active calls to the
     * netifapi functions
     */

    pub MEMP_NUM_NETIFAPI_MSG: u32, // = MEMP_NUM_TCPIP_MSG_API;
}

#[derive(Debug, Clone, Default)]
pub struct LwipArpOptions {
    /*
     * ARP_TABLE_SIZE: Number of active MAC-IP address pairs cached.
     */

    pub ARP_TABLE_SIZE: usize, // = 10;

    /* the time an ARP entry stays valid after its last update,
     *  for ARP_TMR_INTERVAL = 1000, this is
     *  (60 * 5) seconds = 5 minutes.
     */

    pub arp_maxage: u32, // = 300;

    /*
     * arp_queueing==1: Multiple outgoing packets are queued during hardware address
     * resolution. By default, only the most recent packet is queued per IP address.
     * This is sufficient for most protocols and mainly reduces TCP connection
     * startup time. Set this to 1 if you know your application sends more than one
     * packet in a row to an IP address that is not in the ARP cache.
     */

    pub arp_queueing: u32, // = 0;

    /* The maximum number of packets which may be queued for each
     *  unresolved address by other network layers. Defaults to 3, 0 means disabled.
     *  Old packets are dropped, new packets are queued.
     */

    pub arp_queue_len: u32, // = 3;
}

#[derive(Debug, Clone, Default)]
pub struct LwipEtherOptions {
    /*
     * etharp_support_vlan==1: support receiving and sending ethernet packets with
     * VLAN header. See the description of LWIP_HOOK_VLAN_CHECK and
     * LWIP_HOOK_VLAN_SET hooks to check/set VLAN headers.
     * Additionally, you can define ETHARP_VLAN_CHECK to an VLAN: u16 ID to check.
     * If ETHARP_VLAN_CHECK is defined, only VLAN-traffic for this VLAN is accepted.
     * If ETHARP_VLAN_CHECK is not defined, all traffic is accepted.
     * Alternatively, define a function/define ETHARP_VLAN_CHECK_FN(eth_hdr, vlan)
     * that returns 1 to accept a packet or 0 to drop a packet.
     */


    pub etharp_support_vlan: u32, // = 0;

    /* LWIP_ETHERNET==1: enable ethernet support even though ARP might be disabled
 */

// #define LWIP_ETHERNET                   LWIP_ARP

    /* eth_pad_size: number of bytes added before the ethernet header to ensure
 * alignment of payload after that header. Since the header is 14 bytes long,
 * without this padding e.g. addresses in the IP header will not be aligned
 * on a 32-bit boundary, so setting this to 2 can speed up 32-bit-platforms.
 */
    pub eth_pad_size: u32, // = 2;

    /* etharp_support_static_entries==1: enable code to support static ARP table
 * entries (using etharp_add_static_entry/etharp_remove_static_entry).
 */

    pub etharp_support_static_entries: bool, // = false;

    /* etharp_table_match_netif==1: Match netif for ARP table entries.
 * If disabled, duplicate IP address on multiple netifs are not supported
 * (but this should only occur for AutoIP).
 */

    pub etharp_table_match_netif: bool, // = ! LWIP_SINGLE_NETIF;
}

#[derive(Debug, Clone, Default)]
pub struct LwipIpOptions {
    // ip_forward==1: Enables the ability to forward IP packets across network
    // interfaces. If you are going to run lwIP on a device with only one network
    // interface, define this to 0.
    pub ip_forward: bool,
    // = true;
    // ip_reassembly==1: Reassemble incoming fragmented IP packets. Note that
    // this option does not affect outgoing packet sizes, which can be controlled
    // via ip_frag.
    pub ip_reassembly: u32,
    // = 1;
    // ip_frag==1: Fragment outgoing IP packets if their size exceeds MTU. Note
    // that this option does not affect incoming packet sizes, which can be
    // controlled via ip_reassembly.
    pub ip_frag: bool,
    // = 1;
    // ip_options_allowed: Defines the behavior for IP options.
    //   ip_options_allowed==0: All packets with IP options are dropped.
    //   ip_options_allowed==1: IP options are allowed (but not parsed).
    pub ip_options_allowed: bool,
    // = 1;
    // ip_reass_maxage: Maximum time (in multiples of IP_TMR_INTERVAL - so seconds, normally)
    // a fragmented IP packet waits for all fragments to arrive. If not all fragments arrived
    // in this time, the whole packet is discarded.
    pub ip_reass_maxage: u32,
    // = 15;
    // ip_reass_max_pbufs: Total maximum amount of pbufs waiting to be reassembled.
    // Since the received pbufs are enqueued, be sure to configure
    // PBUF_POOL_SIZE > ip_reass_max_pbufs so that the stack is still able to receive
    // packets even if the maximum amount of fragments is enqueued for reassembly!
    // When IPv4 *and* IPv6 are enabled, this even changes to
    // (PBUF_POOL_SIZE > 2 * ip_reass_max_pbufs)!
    pub ip_reass_max_pbufs: u32,
    // = 10;
    // ip_default_ttl: Default value for Time-To-Live used by transport layers.
    pub ip_default_ttl: u32,
    // = 255;
    // ip_sof_broadcast=1: Use the SOF_BROADCAST field to enable broadcast
    // filter per pcb on udp and raw send operations. To enable broadcast filter
    // on recv operations, you also have to set ip_sof_broadcast_recv=1.
    pub ip_sof_broadcast: u32,
    // = 0;
    // ip_sof_broadcast_recv (requires ip_sof_broadcast=1) enable the broadcast
    // filter on recv operations.
    pub ip_sof_broadcast_recv: u32,
    // = 0,
    // ip_forward_allow_tx_on_rx_netif==1: allow ip_forward() to send packets back
    // out on the netif where it was received. This should only be used for
    // wireless networks.
    // ATTENTION: When this is 1, make sure your netif driver correctly marks incoming
    // link-layer-broadcast/multicast packets as such using the corresponding pbuf flags!
    pub ip_forward_allow_tx_on_rx_netif: u32, // = 0;
}

#[derive(Debug, Clone, Default)]
pub struct LwipIcmpOptions {
    pub icmp_ttl: u32,
    // = ip_default_ttl
    pub lwip_broadcast_ping: bool,
    // = false
    pub lwip_multicast_ping: bool, // = false
}

#[derive(Debug, Clone, Default)]
pub struct LwipRawOptions {
    //
    pub raw_ttl: u32, // = ip_default_ttl
}

#[derive(Debug, Clone, Default)]
pub struct LwipDhcpOptions {
    /*
 * dhcp_does_arp_check==1: Do an ARP check on the offered address.
 */
    pub dhcp_does_arp_check: bool,
    // = (LWIP_DHCP && LWIP_ARP);
    /*
 * lwip_dhcp_bootp_file==1: Store offered_si_addr and boot_file_name.
 */
    pub lwip_dhcp_bootp_file: bool,
    // = false;
    /*
 * LWIP_DHCP_GETS_NTP==1: Request NTP servers with discover/select. For each
 * response packet, an callback is called, which has to be provided by the port:
 * void dhcp_set_ntp_servers(num_ntp_servers: u8, ntp_server_addrs: &mut LwipAddr);
*/
    pub lwip_dhcp_get_ntp_srv: bool,
    // = false
    /*
 * The maximum of NTP servers requested
 */
    pub lwip_dhcp_max_ntp_servers: u32,
    // = 1
    /*
 * LWIP_DHCP_MAX_DNS_SERVERS > 0: Request DNS servers with discover/select.
 * DNS servers received in the response are passed to DNS via @ref dns_setserver()
 * (up to the maximum limit defined here).
 */
    pub lwip_dhcp_max_dns_sesrvers: usize, // = DNS_MAX_SERVERS
}

#[derive(Debug, Clone, Default)]
pub struct LwipDhcpAutoIpOptions {
    /*
 * lwip_dhcp_autoip_coop_tries: Set to the number of DHCP DISCOVER probes
 * that should be sent before falling back on AUTOIP (the DHCP client keeps
 * running in this case). This can be set as low as 1 to get an AutoIP address
 * very  quickly, but you should be prepared to handle a changing IP address
 * when DHCP overrides AutoIP.
 */
    pub lwip_dhcp_autoip_coop_tries: u32, // = 9;
    /*
 * lwip_dhcp_autoip_coop==1: Allow DHCP and AUTOIP to be both enabled on
 * the same interface at the same time.
 */

    pub lwip_dhcp_autoip_coop: bool, // = false;
}

#[derive(Debug, Clone, Default)]
pub struct LwipDnsOptions {
    // DNS maximum number of entries to maintain locally.
    pub dns_table_size: usize,
    // = 4
    //
    pub dns_max_name_length: usize,
    // = 256
    //  DNS maximum number of retries when asking for a name, before "timeout".
    pub dns_max_retries: u32,
    // = 4
    //  DNS do a name checking between the query and the response. 
    pub dns_does_name_check: bool,
    // = true
    /* dns_local_hostlist: Implements a local host-to-address list. If enabled, you have to define an initializer:
 *  \#define DNS_LOCAL_HOSTLIST_INIT {DNS_LOCAL_HOSTLIST_ELEM("host_ip4", IPADDR4_INIT_BYTES(1,2,3,4)), \
 *                                    DNS_LOCAL_HOSTLIST_ELEM("host_ip6", IPADDR6_INIT_HOST(123, 234, 345, 456)}
 *
 *  Instead, you can also use an external function:
 *  \#define DNS_LOOKUP_LOCAL_EXTERN(x) extern my_lookup_function: err_t(name: &String, addr: &mut LwipAddr, dns_addrtype: u8)
 *  that looks up the IP address and returns ERR_OK if found (LWIP_DNS_ADDRTYPE_xxx is passed in dns_addrtype).
 */
    pub dns_local_hostlist: bool,
    // = false
    /* If this is turned on, the local host-list can be dynamically changed
 *  at runtime. */
    pub dns_local_hostlist_is_dyamic: bool,
    // = false
    /* Set this to 1 to enable querying ".local" names via mDNS
 *  using a One-Shot Multicast DNS Query */
    pub lwip_dns_support_mdns_queries: bool, // = false
}

#[derive(Debug, Clone, Default)]
pub struct LwipUdpOptions {
    pub udp_ttl: u32, // = IP_DEFAULT)TTL
}

#[derive(Debug, Clone, Default)]
pub struct LwipAlTcpOptions {
    // enable TLS support for abstract_tcp API.
    //  * This needs a port of the functions in abstract_tcp.h to a TLS library.
    //  * A port to ARM mbedtls is provided with lwIP, see apps/abstract_tcp/ directory
    //  * and LWIP_ALTCP_TLS_MBEDTLS option.
    pub lwip_altcp_tls: bool,
    // lwip_altcp==1: enable the abstract_tcp API.
    //  * abstract_tcp is an abstraction layer that prevents applications linking against the
    //  * tcp.h functions but provides the same functionality. It is used to e.g. add
    //  * SSL/TLS or proxy-connect support to an application written for the tcp callback
    //  * API without that application knowing the protocol details.
    //  *
    //  * With lwip_altcp==0, applications written against the abstract_tcp API can still be
    //  * compiled but are directly linked against the tcp.h callback API and then
    //  * cannot use layered protocols.
    pub lwip_altcp: bool,

}

#[derive(Debug, Clone, Default)]
pub struct LwipPacketBufferOptions {
    // he number of bytes that should be allocated
    //  * for an additional encapsulation header before ethernet headers (e.g. 802.11)
    pub pbuf_link_encapsulation_hlen: usize,
    // the number of bytes that should be allocated for a
    //  * link level header. The default is 14, the standard value for
    //  * Ethernet., 14 + ETH_PAD_SZ
    pub pbuf_link_hlen: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LwipNetifOptions {
    // Number of clients that may store ata in client_data member array of NetIfc (max. 256).
    pub lwip_num_netif_client_data: usize,
    // if this is set to 1, lwIP *tries* to put all data
    //  * to be sent into one single pbuf. This is for compatibility with DMA-enabled
    //  * MACs that do not support scatter-gather.
    //  * Beware that this might involve CPU-memcpy before transmitting that would not
    //  * be needed without this flag! Use this only if you need to!
    //  *
    //  * ATTENTION: a driver should *NOT* rely on getting single pbufs but check TX
    //  * pbufs for being in one piece. If not, @ref pbuf_clone can be used to get
    //  * a single pbuf:
    //  *   if (p.next != NULL) {
    //  *     q: &mut PacketBuffer = pbuf_clone(PBUF_RAW, PBUF_RAM, p);
    //  *     if (q == NULL) {
    //  *       return ERR_MEM;
    //  *     }
    //  *     p = q; ATTENTION: do NOT free the old 'p' as the ref belongs to the caller!
    //  *   }
    pub LWIP_NETIF_TX_SINGLE_PBUF: bool,
    // Cache link-layer-address hints (e.g. table
    //  * indices) in NetIfc. TCP and UDP can make use of this to prevent
    //  * scanning the ARP table for every sent packet. While this is faster for big
    //  * ARP tables or many concurrent connections, it might be counterproductive
    //  * if you have a tiny ARP table or if there never are concurrent connections.
    pub LWIP_NETIF_HWADDRHINT: bool,
    // Support a callback function that is called
    //  * when a netif has been removed
    pub LWIP_NETIF_REMOVE_CALLBACK: bool,
    // Support a callback function from an interface
    //  * whenever the link changes (i.e., link down)
    pub LWIP_NETIF_LINK_CALLBACK: bool,
    // Support an extended callback function
    //  * for several netif related event that supports multiple subscribers.
    //  * @see netif_ext_status_callback
    pub LWIP_NETIF_EXT_STATUS_CALLBACK: bool,
    // Support a callback function whenever an interface
    //  * changes its up/down status (i.e., due to DHCP IP acquisition)
    pub LWIP_NETIF_STATUS_CALLBACK: bool,
    //  Support netif api (in netifapi.c)
    pub LWIP_NETIF_API: bool,
    // DHCP_OPTION_HOSTNAME with netif's hostname
    //  * field.
    pub LWIP_NETIF_HOSTNAME: bool,
    // use a single netif only. This is the common case for
    //  * small real-life targets. Some code like routing etc. can be left out.
    pub LWIP_SINGLE_NETIF: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LwipLoopbackOptions {
    // Indicates whether threading is enabled in
    // the system, as netifs must change how they behave depending on this setting
    // for the LWIP_NETIF_LOOPBACK option to work.
    // Setting this is needed to avoid reentering non-reentrant functions like
    // tcp_input().
    //    LWIP_NETIF_LOOPBACK_MULTITHREADING==1: Indicates that the user is using a
    //       multithreaded environment like tcpip.c. In this case, netif.input()
    //       is called directly.
    //    LWIP_NETIF_LOOPBACK_MULTITHREADING==0: Indicates a polling (or NO_SYS) setup.
    //       The packets are put on a list and netif_poll() must be called in
    //       the main application loop.
    pub LWIP_NETIF_LOOPBACK_MULTITHREADING: bool,
    // Maximum number of pbufs on queue for loopback
    // sending for each netif (0 = disabled)
    pub LWIP_LOOPBACK_MAX_PBUFS: usize,
    // Support sending packets with a destination IP address equal to the netif IP address, looping them back up the stack.
    pub LWIP_NETIF_LOOPBACK: bool,
    //  Support multicast/IGMP on loop interface (127.0.0.1).
    pub LWIP_LOOPIF_MULTICAST: bool,
    //
}

#[derive(Debug, Clone, Default)]
pub struct LwipTcpOptions {
    // TCP_TTL: Default Time-To-Live value.
    pub TCP_TTL: u32,
    // = ip_default_ttl,
    /*
 * TCP_WND: The size of a TCP window.  This must be at least
 * (2 * TCP_MSS) for things to work well.
 * ATTENTION: when using TCP_RCV_SCALE, TCP_WND is the total size
 * with scaling applied. Maximum window value in the TCP header
 * will be TCP_WND >> TCP_RCV_SCALE
 */
    pub TCP_WND: u32,
    // = 4 * TCP_MSS
    // TCP_MAXRTX: Maximum number of retransmissions of data segments.
    pub TCP_MAX_RTX: u32,
    // = 12
    // TCP_SYNMAXRTX: Maximum number of retransmissions of SYN segments.
    pub TCP_SYN_MAX_RTX: u32,
    // = 4
    /*
 * TCP_QUEUE_OOSEQ==1: TCP will queue segments that arrive out of order.
 * Define to 0 if your device is low on memory.
 */
    pub TCP_QUEUE_OOSEQ: bool,
    // LWIP_TCP
    /*
 * LWIP_TCP_SACK_OUT==1: TCP will support sending selective acknowledgements (SACKs).
 */
    pub LWIP_TCP_SACK_OUT: bool,
    // false
    /*
 * LWIP_TCP_MAX_SACK_NUM: The maximum number of SACK values to include in TCP segments.
 * Must be at least 1, but is only used if LWIP_TCP_SACK_OUT is enabled.
 * NOTE: Even though we never send more than 3 or 4 SACK ranges in a single segment
 * (depending on other options), setting this option to values greater than 4 is not pointless.
 * This is basically the max number of SACK ranges we want to keep track of.
 * As new data is delivered, some of the SACK ranges may be removed or merged.
 * In that case some of those older SACK ranges may be used again.
 * The amount of memory used to store SACK ranges is LWIP_TCP_MAX_SACK_NUM * 8 bytes for each TCP PCB.
 */
    pub LWIP_TCP_MAX_SACK_NUM: u32,
    // = 4
    /*
 * TCP_MSS: TCP Maximum segment size. (default is 536, a conservative default,
 * you might want to increase this.)
 * For the receive side, this MSS is advertised to the remote side
 * when opening a connection. For the transmit size, this MSS sets
 * an upper limit on the MSS advertised by the remote host.
 */
    pub TCP_MSS: u32,
    // = 536
    /*
 * TCP_CALCULATE_EFF_SEND_MSS: "The maximum size of a segment that TCP really
 * sends, the 'effective send MSS,' MUST be the smaller of the send MSS (which
 * reflects the available reassembly buffer size at the remote host) and the
 * largest size permitted by the IP layer" (RFC 1122)
 * Setting this to 1 enables code that checks TCP_MSS against the MTU of the
 * netif used for a connection and limits the MSS if it would be too big otherwise.
 */
    pub TCP_CALCULATE_EFF_SEND_MSS: bool,
    // = true
    /*
 * TCP_SND_BUF: TCP sender buffer space (bytes).
 * To achieve good performance, this should be at least 2 * TCP_MSS.
 */
    pub TCP_SND_BUF: u32,
    // = 2 * TCP_MSS
    /*
 * TCP_SND_QUEUELEN: TCP sender buffer space (pbufs). This must be at least
 * as much as (2 * TCP_SND_BUF/TCP_MSS) for things to work.
 */
    pub TCP_SND_QUEUELEN: u32,
    // = ((4 * (TCP_SND_BUF) + (TCP_MSS - 1)) / (TCP_MSS));
    /*
 * TCP_SNDLOWAT: TCP writable space (bytes). This must be less than
 * TCP_SND_BUF. It is the amount of space which must be available in the
 * TCP snd_buf for select to return writable (combined with TCP_SNDQUEUELOWAT).
 */
    pub TCP_SNDLOWAT: u32,
    // LWIP_MIN(
    // LWIP_MAX(((TCP_SND_BUF) / 2), (2 * TCP_MSS) + 1),
    // (TCP_SND_BUF) - 1,
    /*
 * TCP_SNDQUEUELOWAT: TCP writable bufs (pbuf count). This must be less
 * than TCP_SND_QUEUELEN. If the number of pbufs queued on a pcb drops below
 * this number, select returns writable (combined with TCP_SNDLOWAT).
 */
    pub TCP_SNDQUEUELOWAT: u32,
    // = LWIP_MAX(((TCP_SND_QUEUELEN) / 2), 5);
    /*
 * TCP_OOSEQ_MAX_BYTES: The default maximum number of bytes queued on ooseq per
 * pcb if TCP_OOSEQ_BYTES_LIMIT is not defined. Default is 0 (no limit).
 * Only valid for TCP_QUEUE_OOSEQ==1.
 */
    pub TCP_OOSEQ_MAX_BYTES: u32,
    // = 0
    /*
 * TCP_OOSEQ_MAX_PBUFS: The default maximum number of pbufs queued on ooseq per
 * pcb if TCP_OOSEQ_BYTES_LIMIT is not defined. Default is 0 (no limit).
 * Only valid for TCP_QUEUE_OOSEQ==1.
 */
    pub TCP_OOSEQ_MAX_PBUFS: u32,
    // = 0
    // The maximum allowed backlog for TCP listen netconns.
// This backlog is used unless another is explicitly specified.
// 0xff is the maximum .
    pub TCP_DEFAULT_LISTEN_BACKLOG: u32,
    // = 0xff;
    // LWIP_TCP_TIMESTAMPS==1: support the TCP timestamp option.
    // The timestamp option is currently only used to help remote hosts, it is not
    // really used locally. Therefore, it is only enabled when a TS option is
    // received in the initial SYN packet from a remote host. pub const LWIP_TCP_TIMESTAMPS: u32 = 0; // The mailbox size for the tcpip thread messages
    //  The queue size value itself is platform-dependent, but is passed to
    //  sys_mbox_new() when tcpip_init is called.
    pub TCPIP_MBOX_SIZE: usize,
    // The priority assigned to the main tcpip thread.
    //  The priority value itself is platform-dependent, but is passed to
    //  sys_thread_new() when the thread is created.
    pub TCPIP_THREAD_PRIO: u64,
    // he stack size used by the main tcpip thread.
    // The stack size value itself is platform-dependent, but is passed to
    // sys_thread_new() when the thread is created.
    pub TCPIP_THREAD_STACKSIZE: usize,
    // The name assigned to the main tcpip thread.
    pub TCPIP_THREAD_NAME: String,
    // When this is > 0, every tcp pcb (including listen pcb) includes a number of
    //  * additional argument entries in an array (see tcp_ext_arg_alloc_id)
    pub LWIP_TCP_PCB_NUM_EXT_ARGS: usize,
    // LWIP_WND_SCALE and TCP_RCV_SCALE:
// Set LWIP_WND_SCALE to 1 to enable window scaling.
// Set TCP_RCV_SCALE to the desired scaling factor (shift count in the
// range of [0..14]).
// When LWIP_WND_SCALE is enabled but TCP_RCV_SCALE is 0, we can use a large
// send window while having a small receive window only.
    pub LWIP_WND_SCALE: u32,
    pub TCP_RCV_SCALE: u32,
    // LWIP_EVENT_API and LWIP_CALLBACK_API: Only one of these should be set to 1.
//     LWIP_EVENT_API==1: The user defines lwip_tcp_event() to receive all
//         events (accept, sent, etc) that happen in the system.
//     LWIP_CALLBACK_API==1: The PCB callback function is called directly
//         for the event. This is the default.
    pub LWIP_EVENT_API: u32,
    pub LWIP_CALLBACK_API: u32,
    // TCP_OVERSIZE: The maximum number of bytes that tcp_write may
// allocate ahead of time in an attempt to create shorter pbuf chains
// for transmission. The meaningful range is 0 to TCP_MSS. Some
// suggested values are:
//
// 0:         Disable oversized allocation. Each tcp_write() allocates a new
//             pbuf (old behaviour).
// 1:         Allocate size-aligned pbufs with minimal excess. Use this if your
//            scatter-gather DMA requires aligned fragments.
// 128:       Limit the pbuf/memory overhead to 20%.
// TCP_MSS:   Try to create unfragmented TCP packets.
// TCP_MSS/4: Try to create 4 fragments or less per TCP packet.
    pub TCP_OVERSIZE: u32,
    // = TCP_MSS;
    // TCP_WND_UPDATE_THRESHOLD: difference in window to trigger an explicit window update
    pub TCP_WND_UPDATE_THRESHOLD: u32, // = LWIP_MIN((TCP_WND / 4), (TCP_MSS * 4));
}

impl LwipTcpOptions {
    pub fn new() -> LwipTcpOptions {
        LwipTcpOptions {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LwipSlipIfOptions {
    // The priority assigned to the slipif_loop thread. The priority value itself is platform-dependent, but is passed to sys_thread_new() when the thread is created.
    pub SLIPIF_THREAD_PRIO: u64,
    // The stack size used by the slipif_loop thread. The stack size value itself is platform-dependent, but is passed to sys_thread_new() when the thread is created.
    pub SLIPIF_THREAD_STACKSIZE: usize,
    // The name assigned to the slipif_loop thread.
    pub SLIPIF_THREAD_NAME: String,
}

#[derive(Debug, Clone, Default)]
pub struct LwipThreadOptions {
    // The priority assigned to any other lwIP thread. The priority value itself is platform-dependent, but is passed to sys_thread_new() when the thread is created.
    pub DEFAULT_THREAD_PRIO: u64,
    // The stack size used by any other lwIP thread. The stack size value itself is platform-dependent, but is passed to sys_thread_new() when the thread is created.
    pub DEFAULT_THREAD_STACKSIZE: usize,
    // The name assigned to any other lwIP thread.
    pub DEFAULT_THREAD_NAME: String,

}

#[derive(Debug, Clone, Default)]
pub struct LwipMailboxOptions {
    // The mailbox size for the incoming connections.
    // The queue size value itself is platform-dependent, but is passed to
    // sys_mbox_new() when the acceptmbox is created.
    pub DEFAULT_ACCEPTMBOX_SIZE: usize,
    // he mailbox size for the incoming packets on a
    //  * NETCONN_TCP. The queue size value itself is platform-dependent, but is passed
    //  * to sys_mbox_new() when the recvmbox is created.
    pub DEFAULT_TCP_RECVMBOX_SIZE: usize,
    // The mailbox size for the incoming packets on a NETCONN_UDP. The queue size value itself is platform-dependent, but is passed to sys_mbox_new() when the recvmbox is created.
    pub DEFAULT_UDP_RECVMBOX_SIZE: usize,
    // The mailbox size for the incoming packets on a
    //  * NETCONN_RAW. The queue size value itself is platform-dependent, but is passed
    //  * to sys_mbox_new() when the recvmbox is created.
    pub DEFAULT_RAW_RECVMBOX_SIZE: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LwipConnectionOptions {
    // LWIP_NETCONN_FULLDUPLEX==1: Enable code that allows reading from one thread, writing from a 2nd thread and closing from a 3rd thread at the same time. ATTENTION: This is currently really alpha! Some requirements: - LWIP_NETCONN_SEM_PER_THREAD==1 is required to use one socket/netconn from
    //    multiple threads at once
    //  - sys_mbox_free() has to unblock receive tasks waiting on recvmbox/acceptmbox
    //    and prevent a task pending on this during/after deletion
    pub LWIP_NETCONN_FULLDUPLEX: bool,
    // se one (thread-local) semaphore per
    //  thread calling socket/netconn functions instead of allocating one
    //  semaphore per netconn (and per select etc.)
    //  ATTENTION: a thread-local semaphore for API calls is needed:
    //  - LWIP_NETCONN_THREAD_SEM_GET() returning a sys_sem_t*
    //  - LWIP_NETCONN_THREAD_SEM_ALLOC() creating the semaphore
    //  - LWIP_NETCONN_THREAD_SEM_FREE() freeing the semaphore
    //  The latter 2 can be invoked up by calling netconn_thread_init()/netconn_thread_cleanup().
    //  Ports may call these for threads created with sys_thread_new().
    pub LWIP_NETCONN_SEM_PER_THREAD: bool,
    // Enable tcpip_timeout/tcpip_untimeout to create timers running in tcpip_thread from another thread.
    pub LWIP_TCPIP_TIMEOUT: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LwipSocketOptions {
    // enable poll() for sockets (including struct pollfd, nfds_t, and constants)
    pub LWIP_SOCKET_POLL: bool,
    // enable select() for sockets (uses a netconn callback to keep track of events). This saves RAM (counters per socket) and code (netconn event callback), which should improve performance a bit).
    pub LWIP_SOCKET_SELECT: bool,
    // LWIP_FIONREAD_LINUXMODE==0 (default): ioctl/FIONREAD returns the amount of
    // pending data in the network buffer. This is the way windows does it. It's
    // the default for lwIP since it is smaller.
    // LWIP_FIONREAD_LINUXMODE==1: ioctl/FIONREAD returns the size of the next
    // pending datagram in bytes. This is the way linux does it. This code is only
    // here for compatibility.
    pub LWIP_FIONREAD_LINUXMODE: bool,
    // Pass a copy of incoming broadcast/multicast packets to all local matches if SO_REUSEADDR is turned on. WARNING: Adds a memcpy for every packet if passing to more than one pcb!
    pub SO_REUSE_RXTOALL: bool,
    // Enable SO_REUSEADDR option.
    pub SO_REUSE: bool,
    // By default, TCP socket/netconn close waits 20 seconds max to send the FIN
    pub LWIP_TCP_CLOSE_TIMEOUT_MS_DEFAULT: u64,
    //  If LWIP_SO_RCVBUF is used, this is the default value for recv_bufsize. INT_MAX
    pub RECV_BUFSIZE_DEFAULT: usize,
    // Enable SO_LINGER processing.
    pub LWIP_SO_LINGER: bool,
    // Enable SO_RCVBUF processing.
    pub LWIP_SO_RCVBUF: bool,
    // SO_RCVTIMEO/SO_SNDTIMEO take an int (milliseconds, much like winsock does) instead of a struct timeval (default).
    pub LWIP_SO_SNDRCVTIMEO_NONSTANDARD: bool,
    // Enable receive timeout for sockets/netconns and SO_RCVTIMEO processing.
    pub LWIP_SO_RCVTIMEO: bool,
    // Enable send timeout for sockets/netconns and SO_SNDTIMEO processing.
    pub LWIP_SO_SNDTIMEO: bool,
    // Enable TCP_KEEPIDLE, TCP_KEEPINTVL and TCP_KEEPCNT options processing. Note that TCP_KEEPIDLE and TCP_KEEPINTVL have to be set in seconds. (does not require sockets.c, and will affect tcp.c)
    pub LWIP_TCP_KEEPALIVE: bool,
    // Increases the file descriptor number created by LwIP with n. This can be useful when there are multiple APIs which create file descriptors. When they all start with a different offset and you won't make them overlap you can re implement read/write/close/ioctl/fnctl to send the requested action to the right library (sharing select will need more work though).
    pub LWIP_SOCKET_OFFSET: usize,
    // Enable POSIX-style sockets functions names. Disable this option if you use a POSIX operating system that uses the same names (read, write & close). (only used if you use sockets.c)
    pub LWIP_POSIX_SOCKETS_IO_NAMES: bool,
    // LWIP_COMPAT_SOCKETS==1: Enable BSD-style sockets functions names through defines. LWIP_COMPAT_SOCKETS==2: Same as ==1 but correctly named functions are created. While this helps code completion, it might conflict with existing libraries. (only used if you use sockets.c)
    pub LWIP_COMPAT_SOCKETS: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LwipChecksumOptions {
    // alculate checksum when copying data from application buffers to pbufs.
    pub LWIP_CHECKSUM_ON_COPY: bool,
    // Check checksums in software for incoming ICMPv6 packets
    pub CHECKSUM_CHECK_ICMP6: bool,
    // Check checksums in software for incoming ICMP packets
    pub CHECKSUM_CHECK_ICMP: bool,
    // Check checksums in software for incoming TCP packets.
    pub CHECKSUM_CHECK_TCP: bool,
    // Check checksums in software for incoming UDP packets.
    pub CHECKSUM_CHECK_UDP: bool,
    // Check checksums in software for incoming IP packets.
    pub CHECKSUM_CHECK_IP: bool,
    // Generate checksums in software for outgoing ICMP6 packets.
    pub CHECKSUM_GEN_ICMP6: bool,
    // Generate checksums in software for outgoing ICMP packets.
    pub CHECKSUM_GEN_ICMP: bool,
    // Generate checksums in software for outgoing TCP packets
    pub CHECKSUM_GEN_TCP: bool,
    // Generate checksums in software for outgoing UDP packets.
    pub CHECKSUM_GEN_UDP: bool,
    // Generate checksums in software for outgoing IP packets.
    pub CHECKSUM_GEN_IP: bool,
    // Checksum generation/check can be enabled/disabled per netif. ATTENTION: if enabled, the CHECKSUM_GEN_* and CHECKSUM_CHECK_* defines must be enabled!
    pub LWIP_CHECKSUM_CTRL_PER_NETIF: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LwipIpv6Options {
    // LWIP_IPV6
    pub LWIP_IPV6: bool,
    // Maximum time (in multiples of IP6_REASS_TMR_INTERVAL - so seconds, normally)
    // a fragmented IP packet waits for all fragments to arrive. If not all fragments arrived
    // in this time, the whole packet is discarded.
    pub IPV6_REASS_MAXAGE: u32,
    // Enable support for IPv6 address scopes, ensuring that
    //  e.g. link-local addresses are really treated as link-local. Disable this
    //  setting only for single-interface configurations.
    //  All addresses that have a scope according to the default policy (link-local
    //  unicast addresses, interface-local and link-local multicast addresses) should
    //  now have a zone set on them before being passed to the core API, although
    //  lwIP will currently attempt to select a zone on the caller's behalf when
    //  necessary. Applications that directly assign IPv6 addresses to interfaces
    //  (which is NOT recommended) must now ensure that link-local addresses carry
    //  the netif's zone. See the new ip6_zone.h header file for more information and
    //  relevant macros. For now it is still possible to turn off scopes support
    //  through the new LWIP_IPV6_SCOPES option. When upgrading an implementation that
    //  uses the core API directly, it is highly recommended to enable
    //  LWIP_IPV6_SCOPES_DEBUG at least for a while, to ensure e.g. proper address
    //  initialization.
    pub LWIP_IPV6_SCOPES: bool,
    // Number of IPv6 addresses per netif
    pub LWIP_IPV6_NUM_ADDRESSES: usize,
    // Fragment outgoing IPv6 packets that are too big.
    pub LWIP_IPV6_FRAG: bool,
    // Forward IPv6 packets across netifs
    pub LWIP_IPV6_FORWARD: bool,
    // reassemble incoming IPv6 packets that fragmented
    pub LWIP_IPV6_REASS: bool,
    // Send router solicitation messages during network startup.
    pub LWIP_IPV6_SEND_ROUTER_SOLICIT: bool,
    // Enable stateless address autoconfiguration as per RFC 4862.
    pub LWIP_IPV6_AUTOCONFIG: bool,
    // eep valid and preferred lifetimes for each IPv6 address. Required for LWIP_IPV6_AUTOCONFIG. May still be enabled otherwise, in which case the application may assign address lifetimes with the appropriate macros. Addresses with no lifetime are assumed to be static. If this option is disabled, all addresses are assumed to be static.
    pub LWIP_IPV6_ADDRESS_LIFETIMES: bool,
    // umber of duplicate address detection attempts
    pub LWIP_IPV6_DUP_DETECT_ATTEMPTS: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LwipIcmpv6Options {
    // Enable ICMPv6 (mandatory per RFC)
    pub LWIP_ICMP6: bool,
    //  bytes from original packet to send back in ICMPv6 error messages.
    pub LWIP_ICMP6_DATASIZE: usize,
    // default hop limit for ICMPv6 messages
    pub LWIP_ICMP6_HL: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LwipMld6Options {
    // Enable multicast listener discovery protocol. If LWIP_IPV6 is enabled but this setting is disabled, the MAC layer must indiscriminately pass all inbound IPv6 multicast traffic to lwIP.
    pub LWIP_IPV6_MLD: bool,
    // Max number of IPv6 multicast groups that can be joined. There must be enough groups so that each netif can join the solicited-node multicast group for each of its local addresses, plus one for MDNS if applicable, plus any number of groups to be joined on UDP sockets.
    pub MEMP_NUM_MLD6_GROUP: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LwipNd6Options {
    // queue outgoing IPv6 packets while MAC address is being resolved.
    pub LWIP_ND6_QUEUEING: bool,
    // Max number of IPv6 packets to queue during MAC resolution.
    pub MEMP_NUM_ND6_QUEUE: usize,
    // Number of entries in IPv6 neighbor cache,
    pub LWIP_ND6_NUM_NEIGHBORS: usize,
    // number of entries in IPv6 destination cache
    pub LWIP_ND6_NUM_DESTINATIONS: usize,
    // number of entries in IPv6 on-link prefixes cache
    pub LWIP_ND6_NUM_PREFIXES: usize,
    // max number of multicast solicit messages to send (neighbor solicit and router solicit)
    pub LWIP_ND6_MAX_MULTICAST_SOLICIT: usize,
    //  max number of unicast neighbor solicitation messages to send during neighbor reachability detection.
    pub LWIP_ND6_MAX_UNICAST_SOLICIT: usize,
    //
    pub LWIP_ND6_MAX_ANYCAST_DELAY_TIME: u64,
    //
    pub LWIP_ND6_MAX_NEIGHBOR_ADVERTISEMENT: usize,
    // default neighbor reachable time (in milliseconds). May be updated by router advertisement messages.
    pub LWIP_ND6_REACHABLE_TIME: u64,
    // default retransmission timer for solicitation messages
    pub LWIP_ND6_RETRANS_TIMER: u64,
    // Delay before first unicast neighbor solicitation message is sent, during neighbor reachability detection.
    pub LWIP_ND6_DELAY_FIRST_PROBE_TIME: u64,
    // Allow Router Advertisement messages to update Reachable time and retransmission timers, and netif MTU.
    pub LWIP_ND6_ALLOW_RA_UPDATES: bool,
    // Allow TCP to provide Neighbor Discovery with reachability hints for connected destinations. This helps avoid sending unicast neighbor solicitation messages.
    pub LWIP_ND6_TCP_REACHABILITY_HINTS: bool,
    // Use IPv6 Router Advertisement Recursive DNS Server Option (as per RFC 6106) to copy a defined maximum number of DNS servers to the DNS module.
    pub LWIP_ND6_RDNSS_MAX_DNS_SERVERS: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LwipDhcp6Options {
    // enable DHCPv6 stateful/stateless address autoconfiguration
    pub LWIP_IPV6_DHCP: bool,
    // enable DHCPv6 stateful address autoconfiguration
    pub LWIP_IPV6_DHCP6_STATEFUL: bool,
    //
    pub LWIP_IPV6_DHCP6_STATELESS: bool,
    // if true then request DHCP6 NTP servers
    pub LWIP_DHCP6_GET_NTP_SRV: bool,

    // max DHCPv6 DNS servers requested
    pub LWIP_DHCP6_MAX_DNS_SERVERS: usize,
    // max DHCPv6 NTP servers requested
    pub LWIP_DHCP6_MAX_NTP_SERVERS: usize,
    // enable debugging
}

#[derive(Debug, Clone, Default)]
pub struct LwipOptions {
    pub mem_options: LwipMemOptions,
    pub ether_options: LwipEtherOptions,
    pub arp_options: LwipArpOptions,
    pub icmp_options: LwipIcmpOptions,
    pub ip_options: LwipIpOptions,
    pub raw_options: LwipRawOptions,
    pub dhcp_options: LwipDhcpOptions,
    pub dhcp_auto_ip_options: LwipDhcpAutoIpOptions,
    pub dns_options: LwipDnsOptions,
    pub udp_options: LwipUdpOptions,
    pub tcp_options: LwipTcpOptions,
    pub al_tcp_options: LwipAlTcpOptions,
    pub packet_buffer_options: LwipPacketBufferOptions,
    pub loopback_options: LwipLoopbackOptions,
    pub slip_if_options: LwipSlipIfOptions,
    pub thread_options: LwipThreadOptions,
    pub connection_options: LwipConnectionOptions,
    pub socket_options: LwipSocketOptions,
    pub checksum_options: LwipChecksumOptions,
    pub ipv6_options: LwipIpv6Options,
    pub icmpv6_options: LwipIcmpv6Options,
    pub mld6_options: LwipMld6Options,
    pub nd6_options: LwipNd6Options,
    pub mailbox_options: LwipMailboxOptions,
    pub al_tcp_mbedtls_options: LwipAlTcpTlsOptions,
    // collect stats
    pub LWIP_STATS: bool,
    // max DHCPv4 DNS servers requested
    pub DNS_MAX_SERVERS: usize,
    // enable debugging
    pub LWIP_DEBUG: bool,
    // enable performance testing
    pub LWIP_PERF: bool,
    // changes to make unit testing possible
    pub LWIP_TESTMODE: bool,
    // LWIP_NETBUF_RECVINFO==1: append destination addr and port to every netbuf.
    pub LWIP_NETBUF_RECVINFO: bool,
    // = false
    /*
 * LWIP_MULTICAST_TX_OPTIONS==1: Enable multicast TX support like the socket options
 * IP_MULTICAST_TTL/IP_MULTICAST_IF/IP_MULTICAST_LOOP, as well as (currently only)
 * core support for the corresponding IPv6 options.
 */
// #define LWIP_MULTICAST_TX_OPTIONS       ((LWIP_IGMP || LWIP_IPV6_MLD) && (LWIP_UDP || LWIP_RAW))
    pub LWIP_MULTICAST_TX_OPTIONS: bool, // = false;

    /*
     * LWIP_MIB2_CALLBACKS==1: Turn on SNMP MIB2 callbacks.
     * Turn this on to get callbacks needed to implement MIB2.
     * Usually MIB2_STATS should be enabled, too.
     */

    pub LWIP_MIB2_CALLBACKS: bool, // = false;
}

impl LwipOptions {
    pub fn new() -> LwipOptions {
        LwipOptions {

            // LWIP_IPV6: false,
            // IPV6_REASS_MAXAGE: 30,
            // LWIP_IPV6_SCOPES: false,
            // LWIP_IPV6_NUM_ADDRESSES: 3,
            // LWIP_IPV6_FRAG: false,
            // LWIP_IPV6_FORWARD: false,
            // LWIP_IPV6_REASS: false,
            // LWIP_IPV6_SEND_ROUTER_SOLICIT: false,
            // LWIP_IPV6_AUTOCONFIG: false,
            // LWIP_ICMP6: false,
            // LWIP_ICMP6_DATASIZE: 8,
            // LWIP_ICMP6_HL: 255,
            // LWIP_IPV6_MLD: false,
            // MEMP_NUM_MLD6_GROUP: 0,
            // LWIP_ND6_QUEUEING: false,
            // MEMP_NUM_ND6_QUEUE: 20,
            // LWIP_ND6_NUM_NEIGHBORS: 10,
            // LWIP_ND6_NUM_DESTINATIONS: 10,
            // LWIP_ND6_NUM_PREFIXES: 5,
            // LWIP_ND6_MAX_MULTICAST_SOLICIT: 3,
            // LWIP_ND6_MAX_UNICAST_SOLICIT: 1,
            // LWIP_ND6_MAX_ANYCAST_DELAY_TIME: 1000,
            // LWIP_ND6_MAX_NEIGHBOR_ADVERTISEMENT: 1,
            // LWIP_ND6_REACHABLE_TIME: 30000,
            // LWIP_ND6_RETRANS_TIMER: 1000,
            // LWIP_ND6_DELAY_FIRST_PROBE_TIME: 5000,
            // LWIP_ND6_ALLOW_RA_UPDATES: false,
            // LWIP_ND6_TCP_REACHABILITY_HINTS: true,
            // LWIP_ND6_RDNSS_MAX_DNS_SERVERS: 0,
            // LWIP_IPV6_DHCP: false,
            // LWIP_IPV6_DHCP6_STATEFUL: true,
            // LWIP_IPV6_DHCP6_STATELESS: false,
            // LWIP_DHCP6_GET_NTP_SRV: false,
            // DNS_MAX_SERVERS: 2,
            // LWIP_DHCP6_MAX_DNS_SERVERS: 2,
            // LWIP_DHCP6_MAX_NTP_SERVERS: 1,
            // LWIP_DEBUG: false,
            // LWIP_PERF: false,
            // LWIP_TESTMODE: false,
            // LWIP_IPV6_DUP_DETECT_ATTEMPTS: 1,
            // LWIP_IPV6_ADDRESS_LIFETIMES: false,
            mem_options: Default::default(),
            ether_options: Default::default(),
            arp_options: Default::default(),
            icmp_options: Default::default(),
            ip_options: Default::default(),
            raw_options: Default::default(),
            dhcp_options: Default::default(),
            dhcp_auto_ip_options: Default::default(),
            dns_options: Default::default(),
            udp_options: Default::default(),
            tcp_options: Default::default(),
            al_tcp_options: Default::default(),
            packet_buffer_options: Default::default(),
            loopback_options: Default::default(),
            slip_if_options: Default::default(),
            thread_options: Default::default(),
            connection_options: Default::default(),
            socket_options: Default::default(),
            checksum_options: Default::default(),
            ipv6_options: Default::default(),
            icmpv6_options: Default::default(),
            mld6_options: Default::default(),
            nd6_options: Default::default(),
            mailbox_options: Default::default(),
            al_tcp_mbedtls_options: Default::default(),
            LWIP_STATS: false,
            DNS_MAX_SERVERS: 0,
            LWIP_DEBUG: false,
            LWIP_PERF: false,
            LWIP_TESTMODE: false,
            LWIP_NETBUF_RECVINFO: false,
            LWIP_MULTICAST_TX_OPTIONS: false,
            LWIP_MIB2_CALLBACKS: false,
        }
    }
}

pub const IF_NAMESIZE: usize = NETIF_NAMESIZE;
