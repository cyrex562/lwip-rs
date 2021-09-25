use super::arch_h::LWIP_MEM_ALIGN_SIZE;

/*
 * @file
 *
 * lwIP Options Configuration
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

/*
 * NOTE: || defined __DOXYGEN__ is a workaround for doxygen bug -
 * without this, doxygen does not see the actual #define
 */

// #define LWIP_HDR_OPT_H

/*
 * Include user defined options first. Anything not defined in these files
 * will be set to standard values. Override anything you don't like!
 */

/*
 * @defgroup lwip_opts Options (lwipopts.h)
 * @ingroup lwip
 *
 * @defgroup lwip_opts_debug Debugging
 * @ingroup lwip_opts
 *
 * @defgroup lwip_opts_infrastructure Infrastructure
 * @ingroup lwip_opts
 *
 * @defgroup lwip_opts_callback Callback-style APIs
 * @ingroup lwip_opts
 *
 * @defgroup lwip_opts_threadsafe_apis Thread-safe APIs
 * @ingroup lwip_opts
 */

/*
   ------------------------------------
   -------------- NO SYS --------------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_nosys NO_SYS
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * NO_SYS==1: Use lwIP without OS-awareness (no thread, semaphores, mutexes or
 * mboxes). This means threaded APIs cannot be used (socket, netconn,
 * i.e. everything in the 'api' folder), only the callback-style raw API is
 * available (and you have to watch out for yourself that you don't access
 * lwIP functions/structures from more than one context at a time!)
 */

pub const NO_SYS: u32 = 0;

/*
 * @}
 */

/*
 * @defgroup lwip_opts_timers Timers
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * LWIP_TIMERS==0: Drop support for sys_timeout and lwip-internal cyclic timers.
 * (the array of lwip-internal cyclic timers is still provided)
 * (check NO_SYS_NO_TIMERS for compatibility to old versions)
 */

// #define LWIP_TIMERS                     (!NO_SYS || (NO_SYS && !NO_SYS_NO_TIMERS))

// #define LWIP_TIMERS                     1

/*
 * LWIP_TIMERS_CUSTOM==1: Provide your own timer implementation.
 * Function prototypes in timeouts.h and the array of lwip-internal cyclic timers
 * are still included, but the implementation is not. The following functions
 * will be required: sys_timeouts_init(), sys_timeout(), sys_untimeout(),
 *                   sys_timeouts_mbox_fetch()
 */

pub const LWIP_TIMERS_CUSTOM: u32 = 0;

/*
 * @}
 */

/*
 * @defgroup lwip_opts_memcpy memcpy
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * MEMCPY: override this if you have a faster implementation at hand than the
 * one included in your C library
 */

// #define MEMCPY(dst,src,len)             memcpy(dst,src,len)

/*
 * SMEMCPY: override this with care! Some compilers (e.g. gcc) can inline a
 * call to memcpy() if the length is known at compile time and is small.
 */

// #define SMEMCPY(dst,src,len)            memcpy(dst,src,len)

/*
 * MEMMOVE: override this if you have a faster implementation at hand than the
 * one included in your C library.  lwIP currently uses MEMMOVE only when IPv6
 * fragmentation support is enabled.
 */

// #define MEMMOVE(dst,src,len)            memmove(dst,src,len)

/*
 * @}
 */

/*
   ------------------------------------
   ----------- Core locking -----------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_lock Core locking and MPU
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * LWIP_MPU_COMPATIBLE: enables special memory management mechanism
 * which makes lwip able to work on MPU (Memory Protection Unit) system
 * by not passing stack-pointers to other threads
 * (this decreases performance as memory is allocated from pools instead
 * of keeping it on the stack)
 */

pub const LWIP_MPU_COMPATIBLE: u32 = 0;

/*
 * LWIP_TCPIP_CORE_LOCKING
 * Creates a global mutex that is held during TCPIP thread operations.
 * Can be locked by client code to perform lwIP operations without changing
 * into TCPIP thread using callbacks. See LOCK_TCPIP_CORE() and
 * UNLOCK_TCPIP_CORE().
 * Your system should provide mutexes supporting priority inversion to use this.
 */

// #define LWIP_TCPIP_CORE_LOCKING         1

/*
 * LWIP_TCPIP_CORE_LOCKING_INPUT: when LWIP_TCPIP_CORE_LOCKING is enabled,
 * this lets tcpip_input() grab the mutex for input packets as well,
 * instead of allocating a message and passing it to tcpip_thread.
 *
 * ATTENTION: this does not work when tcpip_input() is called from
 * interrupt context!
 */

pub const LWIP_TCPIP_CORE_LOCKING_INPUT: u32 = 0;

/*
 * SYS_LIGHTWEIGHT_PROT==1: enable inter-task protection (and task-vs-interrupt
 * protection) for certain critical regions during buffer allocation, deallocation
 * and memory allocation and deallocation.
 * ATTENTION: This is required when using lwIP from more than one context! If
 * you disable this, you must be sure what you are doing!
 */

pub const SYS_LIGHTWEIGHT_PROT: u32 = 1;

/*
 * Macro/function to check whether lwIP's threading/locking
 * requirements are satisfied during current function call.
 * This macro usually calls a function that is implemented in the OS-dependent
 * sys layer and performs the following checks:
 * - Not in ISR (this should be checked for NO_SYS==1, too!)
 * - If @ref LWIP_TCPIP_CORE_LOCKING = 1: TCPIP core lock is held
 * - If @ref LWIP_TCPIP_CORE_LOCKING = 0: function is called from TCPIP thread
 * @see @ref multithreading
 */

// #define LWIP_ASSERT_CORE_LOCKED()

/*
 * Called as first thing in the lwIP TCPIP thread. Can be used in conjunction
 * with @ref LWIP_ASSERT_CORE_LOCKED to check core locking.
 * @see @ref multithreading
 */

// #define LWIP_MARK_TCPIP_THREAD()

/*
 * @}
 */

/*
   ------------------------------------
   ---------- Memory options ----------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_mem Heap and memory pools
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * MEM_LIBC_MALLOC==1: Use malloc/free/realloc provided by your C-library
 * instead of the lwip internal allocator. Can save code size if you
 * already use it.
 */

pub const MEM_LIBC_MALLOC: u32 = 0;

/*
 * MEMP_MEM_MALLOC==1: Use mem_malloc/mem_free instead of the lwip pool allocator.
 * Especially useful with MEM_LIBC_MALLOC but handle with care regarding execution
 * speed (heap alloc can be much slower than pool alloc) and usage from interrupts
 * (especially if your netif driver allocates PBUF_POOL pbufs for received frames
 * from interrupt)!
 * ATTENTION: Currently, this uses the heap for ALL pools (also for private pools,
 * not only for internal pools defined in memp_std.h)!
 */

pub const MEMP_MEM_MALLOC: u32 = 0;

/*
 * MEMP_MEM_INIT==1: Force use of memset to initialize pool memory.
 * Useful if pool are moved in uninitialized section of memory. This will ensure
 * default values in pcbs struct are well initialized in all conditions.
 */

pub const MEMP_MEM_INIT: u32 = 0;

/*
 * MEM_ALIGNMENT: should be set to the alignment of the CPU
 *    4 byte alignment -> \#define MEM_ALIGNMENT 4
 *    2 byte alignment -> \#define MEM_ALIGNMENT 2
 */

pub const MEM_ALIGNMENT: u32 = 8;

/*
 * MEM_SIZE: the size of the heap memory. If the application will send
 * a lot of data that needs to be copied, this should be set high.
 */

pub const MEM_SIZE: u32 = 1600;

/*
 * MEMP_OVERFLOW_CHECK: memp overflow protection reserves a configurable
 * amount of bytes before and after each memp element in every pool and fills
 * it with a prominent default value.
 *    MEMP_OVERFLOW_CHECK == 0 no checking
 *    MEMP_OVERFLOW_CHECK == 1 checks each element when it is freed
 *    MEMP_OVERFLOW_CHECK >= 2 checks each element in every pool every time
 *      memp_malloc() or memp_free() is called (useful but slow!)
 */

pub const MEMP_OVERFLOW_CHECK: u32 = 0;

/*
 * MEMP_SANITY_CHECK==1: run a sanity check after each memp_free() to make
 * sure that there are no cycles in the linked lists.
 */

pub const MEMP_SANITY_CHECK: u32 = 0;

/*
 * MEM_OVERFLOW_CHECK: mem overflow protection reserves a configurable
 * amount of bytes before and after each heap allocation chunk and fills
 * it with a prominent default value.
 *    MEM_OVERFLOW_CHECK == 0 no checking
 *    MEM_OVERFLOW_CHECK == 1 checks each element when it is freed
 *    MEM_OVERFLOW_CHECK >= 2 checks all heap elements every time
 *      mem_malloc() or mem_free() is called (useful but slow!)
 */

pub const MEM_OVERFLOW_CHECK: u32 = 0;

/*
 * MEM_SANITY_CHECK==1: run a sanity check after each mem_free() to make
 * sure that the linked list of heap elements is not corrupted.
 */

pub const MEM_SANITY_CHECK: u32 = 0;

/*
 * MEM_USE_POOLS==1: Use an alternative to malloc() by allocating from a set
 * of memory pools of various sizes. When mem_malloc is called, an element of
 * the smallest pool that can provide the length needed is returned.
 * To use this, MEMP_USE_CUSTOM_POOLS also has to be enabled.
 */

pub const MEM_USE_POOLS: u32 = 0;

/*
 * MEM_USE_POOLS_TRY_BIGGER_POOL==1: if one malloc-pool is empty, try the next
 * bigger pool - WARNING: THIS MIGHT WASTE MEMORY but it can make a system more
 * reliable. */

pub const MEM_USE_POOLS_TRY_BIGGER_POOL: u32 = 0;

/*
 * MEMP_USE_CUSTOM_POOLS==1: whether to include a user file lwippools.h
 * that defines additional pools beyond the "standard" ones required
 * by lwIP. If you set this to 1, you must have lwippools.h in your
 * include path somewhere.
 */

pub const MEMP_USE_CUSTOM_POOLS: u32 = 0;

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

pub const LWIP_ALLOW_MEM_FREE_FROM_OTHER_CONTEXT: u32 = 0;

/*
 * @}
 */

/*
   ------------------------------------------------
   ---------- Internal Memory Pool Sizes ----------
   ------------------------------------------------
*/
/*
 * @defgroup lwip_opts_memp Internal memory pools
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * MEMP_NUM_PBUF: the number of memp struct pbufs (used for PBUF_ROM and PBUF_REF).
 * If the application sends a lot of data out of ROM (or other static memory),
 * this should be set high.
 */

pub const MEMP_NUM_PBUF: u32 = 16;

/*
 * MEMP_NUM_RAW_PCB: Number of raw connection PCBs
 * (requires the LWIP_RAW option)
 */

pub const MEMP_NUM_RAW_PCB: u32 = 4;

/*
 * MEMP_NUM_UDP_PCB: the number of UDP protocol control blocks. One
 * per active UDP "connection".
 * (requires the LWIP_UDP option)
 */

pub const MEMP_NUM_UDP_PCB: u32 = 4;

/*
 * MEMP_NUM_TCP_PCB: the number of simultaneously active TCP connections.
 * (requires the LWIP_TCP option)
 */

pub const MEMP_NUM_TCP_PCB: u32 = 5;

/*
 * MEMP_NUM_TCP_PCB_LISTEN: the number of listening TCP connections.
 * (requires the LWIP_TCP option)
 */

pub const MEMP_NUM_TCP_PCB_LISTEN: u32 = 8;

/*
 * MEMP_NUM_TCP_SEG: the number of simultaneously queued TCP segments.
 * (requires the LWIP_TCP option)
 */

pub const MEMP_NUM_TCP_SEG: u32 = 16;

/*
 * MEMP_NUM_ALTCP_PCB: the number of simultaneously active altcp layer pcbs.
 * (requires the LWIP_ALTCP option)
 * Connections with multiple layers require more than one AltcpPcb (e.g. TLS
 * over TCP requires 2 altcp_pcbs, one for TLS and one for TCP).
 */

pub const MEMP_NUM_ALTCP_PCB: u32 = MEMP_NUM_TCP_PCB;

/*
 * MEMP_NUM_REASSDATA: the number of IP packets simultaneously queued for
 * reassembly (whole packets, not fragments!)
 */

pub const MEMP_NUM_REASSDATA: u32 = 5;

/*
 * MEMP_NUM_FRAG_PBUF: the number of IP fragments simultaneously sent
 * (fragments, not whole packets!).
 * This is only used with LWIP_NETIF_TX_SINGLE_PBUF==0 and only has to be > 1
 * with DMA-enabled MACs where the packet is not yet sent when netif.output
 * returns.
 */

pub const MEMP_NUM_FRAG_PBUF: u32 = 15;

/*
 * MEMP_NUM_ARP_QUEUE: the number of simultaneously queued outgoing
 * packets (pbufs) that are waiting for an ARP request (to resolve
 * their destination address) to finish.
 * (requires the ARP_QUEUEING option)
 */

pub const MEMP_NUM_ARP_QUEUE: u32 = 30;

/*
 * MEMP_NUM_IGMP_GROUP: The number of multicast groups whose network interfaces
 * can be members at the same time (one per netif - allsystems group -, plus one
 * per netif membership).
 * (requires the LWIP_IGMP option)
 */

pub const MEMP_NUM_IGMP_GROUP: u32 = 8;

/*
 * The number of sys timeouts used by the core stack (not apps)
 * The default number of timeouts is calculated here for all enabled modules.
 */
// #define LWIP_NUM_SYS_TIMEOUT_INTERNAL   (LWIP_TCP + IP_REASSEMBLY + LWIP_ARP + (2*LWIP_DHCP) + LWIP_AUTOIP + LWIP_IGMP + LWIP_DNS + PPP_NUM_TIMEOUTS + (LWIP_IPV6 * (1 + LWIP_IPV6_REASS + LWIP_IPV6_MLD)))

/*
 * MEMP_NUM_SYS_TIMEOUT: the number of simultaneously active timeouts.
 * The default number of timeouts is calculated here for all enabled modules.
 * The formula expects settings to be either '0' or '1'.
 */

pub const MEMP_NUM_SYS_TIMEOUT: u32 = LWIP_NUM_SYS_TIMEOUT_INTERNAL;

/*
 * MEMP_NUM_NETBUF: the number of struct netbufs.
 * (only needed if you use the sequential API, like api_lib.c)
 */

pub const MEMP_NUM_NETBUF: u32 = 2;

/*
 * MEMP_NUM_NETCONN: the number of struct netconns.
 * (only needed if you use the sequential API, like api_lib.c)
 */

pub const MEMP_NUM_NETCONN: u32 = 4;

/*
 * MEMP_NUM_SELECT_CB: the number of LwipSelectCallback.
 * (Only needed if you have LWIP_MPU_COMPATIBLE==1 and use the socket API.
 * In that case, you need one per thread calling lwip_select.)
 */

pub const MEMP_NUM_SELECT_CB: u32 = 4;

/*
 * MEMP_NUM_TCPIP_MSG_API: the number of struct tcpip_msg, which are used
 * for callback/timeout API communication.
 * (only needed if you use tcpip.c)
 */

pub const MEMP_NUM_TCPIP_MSG_API: u32 = 8;

/*
 * MEMP_NUM_TCPIP_MSG_INPKT: the number of struct tcpip_msg, which are used
 * for incoming packets.
 * (only needed if you use tcpip.c)
 */

pub const MEMP_NUM_TCPIP_MSG_INPKT: u32 = 8;

/*
 * MEMP_NUM_NETDB: the number of concurrently running lwip_addrinfo() calls
 * (before freeing the corresponding memory using lwip_freeaddrinfo()).
 */

pub const MEMP_NUM_NETDB: u32 = 1;

/*
 * MEMP_NUM_LOCALHOSTLIST: the number of host entries in the local host list
 * if DNS_LOCAL_HOSTLIST_IS_DYNAMIC==1.
 */

pub const MEMP_NUM_LOCALHOSTLIST: u32 = 1;

/*
 * PBUF_POOL_SIZE: the number of buffers in the pbuf pool.
 */

pub const PBUF_POOL_SIZE: u32 = 16;

/* MEMP_NUM_API_MSG: the number of concurrently active calls to various
 * socket, netconn, and tcpip functions
 */

pub const MEMP_NUM_API_MSG: u32 = MEMP_NUM_TCPIP_MSG_API;

/* MEMP_NUM_DNS_API_MSG: the number of concurrently active calls to netconn_gethostbyname
 */

pub const MEMP_NUM_DNS_API_MSG: u32 = MEMP_NUM_TCPIP_MSG_API;

/* MEMP_NUM_SOCKET_SETGETSOCKOPT_DATA: the number of concurrently active calls
 * to getsockopt/setsockopt
 */

pub const MEMP_NUM_SOCKET_SETGETSOCKOPT_DATA: u32 = MEMP_NUM_TCPIP_MSG_API;

/* MEMP_NUM_NETIFAPI_MSG: the number of concurrently active calls to the
 * netifapi functions
 */

pub const MEMP_NUM_NETIFAPI_MSG: u32 = MEMP_NUM_TCPIP_MSG_API;

/*
 * @}
 */

/*
   ---------------------------------
   ---------- ARP options ----------
   ---------------------------------
*/
/*
 * @defgroup lwip_opts_arp ARP
 * @ingroup lwip_opts_ipv4
 * @{
 */
/*
 * LWIP_ARP==1: Enable ARP functionality.
 */

// #define LWIP_ARP                        1

/*
 * ARP_TABLE_SIZE: Number of active MAC-IP address pairs cached.
 */

pub const ARP_TABLE_SIZE: u32 = 10;

/* the time an ARP entry stays valid after its last update,
 *  for ARP_TMR_INTERVAL = 1000, this is
 *  (60 * 5) seconds = 5 minutes.
 */

pub const ARP_MAXAGE: u32 = 300;

/*
 * ARP_QUEUEING==1: Multiple outgoing packets are queued during hardware address
 * resolution. By default, only the most recent packet is queued per IP address.
 * This is sufficient for most protocols and mainly reduces TCP connection
 * startup time. Set this to 1 if you know your application sends more than one
 * packet in a row to an IP address that is not in the ARP cache.
 */

pub const ARP_QUEUEING: u32 = 0;

/* The maximum number of packets which may be queued for each
 *  unresolved address by other network layers. Defaults to 3, 0 means disabled.
 *  Old packets are dropped, new packets are queued.
 */

pub const ARP_QUEUE_LEN: u32 = 3;

/*
 * ETHARP_SUPPORT_VLAN==1: support receiving and sending ethernet packets with
 * VLAN header. See the description of LWIP_HOOK_VLAN_CHECK and
 * LWIP_HOOK_VLAN_SET hooks to check/set VLAN headers.
 * Additionally, you can define ETHARP_VLAN_CHECK to an VLAN: u16 ID to check.
 * If ETHARP_VLAN_CHECK is defined, only VLAN-traffic for this VLAN is accepted.
 * If ETHARP_VLAN_CHECK is not defined, all traffic is accepted.
 * Alternatively, define a function/define ETHARP_VLAN_CHECK_FN(eth_hdr, vlan)
 * that returns 1 to accept a packet or 0 to drop a packet.
 */

pub const ETHARP_SUPPORT_VLAN: u32 = 0;

/* LWIP_ETHERNET==1: enable ethernet support even though ARP might be disabled
 */

// #define LWIP_ETHERNET                   LWIP_ARP

/* ETH_PAD_SIZE: number of bytes added before the ethernet header to ensure
 * alignment of payload after that header. Since the header is 14 bytes long,
 * without this padding e.g. addresses in the IP header will not be aligned
 * on a 32-bit boundary, so setting this to 2 can speed up 32-bit-platforms.
 */

pub const ETH_PAD_SIZE: u32 = false;

/* ETHARP_SUPPORT_STATIC_ENTRIES==1: enable code to support static ARP table
 * entries (using etharp_add_static_entry/etharp_remove_static_entry).
 */

pub const ETHARP_SUPPORT_STATIC_ENTRIES: bool = false;

/* ETHARP_TABLE_MATCH_NETIF==1: Match netif for ARP table entries.
 * If disabled, duplicate IP address on multiple netifs are not supported
 * (but this should only occur for AutoIP).
 */

pub const ETHARP_TABLE_MATCH_NETIF: bool = !LWIP_SINGLE_NETIF;

/*
 * @}
 */

/*
   --------------------------------
   ---------- IP options ----------
   --------------------------------
*/
/*
 * @defgroup lwip_opts_ipv4 IPv4
 * @ingroup lwip_opts
 * @{
 */
/*
 * LWIP_IPV4==1: Enable IPv4
 */

// #define LWIP_IPV4                       1

/*
 * IP_FORWARD==1: Enables the ability to forward IP packets across network
 * interfaces. If you are going to run lwIP on a device with only one network
 * interface, define this to 0.
 */

pub const IP_FORWARD: u32 = 0;

/*
 * IP_REASSEMBLY==1: Reassemble incoming fragmented IP packets. Note that
 * this option does not affect outgoing packet sizes, which can be controlled
 * via IP_FRAG.
 */

pub const IP_REASSEMBLY: u32 = 1;

/*
 * IP_FRAG==1: Fragment outgoing IP packets if their size exceeds MTU. Note
 * that this option does not affect incoming packet sizes, which can be
 * controlled via IP_REASSEMBLY.
 */

pub const IP_FRAG: u32 = 1;

/* disable IPv4 extensions when IPv4 is disabled */
// #undef IP_FORWARD
pub const IP_FORWARD: u32 = 0;
// #undef IP_REASSEMBLY
pub const IP_REASSEMBLY: u32 = 0;
// #undef IP_FRAG
pub const IP_FRAG: u32 = 0;

/*
 * IP_OPTIONS_ALLOWED: Defines the behavior for IP options.
 *      IP_OPTIONS_ALLOWED==0: All packets with IP options are dropped.
 *      IP_OPTIONS_ALLOWED==1: IP options are allowed (but not parsed).
 */

pub const IP_OPTIONS_ALLOWED: u32 = 1;

/*
 * IP_REASS_MAXAGE: Maximum time (in multiples of IP_TMR_INTERVAL - so seconds, normally)
 * a fragmented IP packet waits for all fragments to arrive. If not all fragments arrived
 * in this time, the whole packet is discarded.
 */

pub const IP_REASS_MAXAGE: u32 = 15;

/*
 * IP_REASS_MAX_PBUFS: Total maximum amount of pbufs waiting to be reassembled.
 * Since the received pbufs are enqueued, be sure to configure
 * PBUF_POOL_SIZE > IP_REASS_MAX_PBUFS so that the stack is still able to receive
 * packets even if the maximum amount of fragments is enqueued for reassembly!
 * When IPv4 *and* IPv6 are enabled, this even changes to
 * (PBUF_POOL_SIZE > 2 * IP_REASS_MAX_PBUFS)!
 */

pub const IP_REASS_MAX_PBUFS: u32 = 10;

/*
 * IP_DEFAULT_TTL: Default value for Time-To-Live used by transport layers.
 */

pub const IP_DEFAULT_TTL: u32 = 255;

/*
 * IP_SOF_BROADCAST=1: Use the SOF_BROADCAST field to enable broadcast
 * filter per pcb on udp and raw send operations. To enable broadcast filter
 * on recv operations, you also have to set IP_SOF_BROADCAST_RECV=1.
 */

pub const IP_SOF_BROADCAST: u32 = 0;

/*
 * IP_SOF_BROADCAST_RECV (requires IP_SOF_BROADCAST=1) enable the broadcast
 * filter on recv operations.
 */

pub const IP_SOF_BROADCAST_RECV: u32 = 0;

/*
 * IP_FORWARD_ALLOW_TX_ON_RX_NETIF==1: allow ip_forward() to send packets back
 * out on the netif where it was received. This should only be used for
 * wireless networks.
 * ATTENTION: When this is 1, make sure your netif driver correctly marks incoming
 * link-layer-broadcast/multicast packets as such using the corresponding pbuf flags!
 */

pub const IP_FORWARD_ALLOW_TX_ON_RX_NETIF: u32 = 0;

/*
 * @}
 */

/*
   ----------------------------------
   ---------- ICMP options ----------
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_icmp ICMP
 * @ingroup lwip_opts_ipv4
 * @{
 */
/*
 * LWIP_ICMP==1: Enable ICMP module inside the IP stack.
 * Be careful, disable that make your product non-compliant to RFC1122
 */

// #define LWIP_ICMP                       1

/*
 * ICMP_TTL: Default value for Time-To-Live used by ICMP packets.
 */

pub const ICMP_TTL: u32 = IP_DEFAULT_TTL;

/*
 * LWIP_BROADCAST_PING==1: respond to broadcast pings (default is unicast only)
 */

pub const LWIP_BROADCAST_PING: u32 = 0;

/*
 * LWIP_MULTICAST_PING==1: respond to multicast pings (default is unicast only)
 */

pub const LWIP_MULTICAST_PING: u32 = 0;

/*
 * @}
 */

/*
   ---------------------------------
   ---------- RAW options ----------
   ---------------------------------
*/
/*
 * @defgroup lwip_opts_raw RAW
 * @ingroup lwip_opts_callback
 * @{
 */
/*
 * LWIP_RAW==1: Enable application layer to hook into the IP layer itself.
 */

pub const LWIP_RAW: u32 = 0;

/*
 * LWIP_RAW==1: Enable application layer to hook into the IP layer itself.
 */

pub const RAW_TTL: u32 = IP_DEFAULT_TTL;

/*
 * @}
 */

/*
   ----------------------------------
   ---------- DHCP options ----------
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_dhcp DHCP
 * @ingroup lwip_opts_ipv4
 * @{
 */
/*
 * LWIP_DHCP==1: Enable DHCP module.
 */

pub const LWIP_DHCP: u32 = 0;

/* disable DHCP when IPv4 is disabled */
// #undef LWIP_DHCP
pub const LWIP_DHCP: u32 = 0;

/*
 * DHCP_DOES_ARP_CHECK==1: Do an ARP check on the offered address.
 */

pub const DHCP_DOES_ARP_CHECK: u32 = (LWIP_DHCP && LWIP_ARP);

/*
 * LWIP_DHCP_BOOTP_FILE==1: Store offered_si_addr and boot_file_name.
 */

pub const LWIP_DHCP_BOOTP_FILE: u32 = 0;

/*
 * LWIP_DHCP_GETS_NTP==1: Request NTP servers with discover/select. For each
 * response packet, an callback is called, which has to be provided by the port:
 * void dhcp_set_ntp_servers(num_ntp_servers: u8, ntp_server_addrs: &mut LwipAddr);
*/

pub const LWIP_DHCP_GET_NTP_SRV: u32 = 0;

/*
 * The maximum of NTP servers requested
 */

// #define LWIP_DHCP_MAX_NTP_SERVERS       1

/*
 * LWIP_DHCP_MAX_DNS_SERVERS > 0: Request DNS servers with discover/select.
 * DNS servers received in the response are passed to DNS via @ref dns_setserver()
 * (up to the maximum limit defined here).
 */

// #define LWIP_DHCP_MAX_DNS_SERVERS       DNS_MAX_SERVERS

/*
 * @}
 */

/*
   ------------------------------------
   ---------- AUTOIP options ----------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_autoip AUTOIP
 * @ingroup lwip_opts_ipv4
 * @{
 */
/*
 * LWIP_AUTOIP==1: Enable AUTOIP module.
 */

pub const LWIP_AUTOIP: u32 = 0;

/* disable AUTOIP when IPv4 is disabled */
//#undef LWIP_AUTOIP
pub const LWIP_AUTOIP: u32 = 0;

/*
 * LWIP_DHCP_AUTOIP_COOP==1: Allow DHCP and AUTOIP to be both enabled on
 * the same interface at the same time.
 */

pub const LWIP_DHCP_AUTOIP_COOP: u32 = 0;

/*
 * LWIP_DHCP_AUTOIP_COOP_TRIES: Set to the number of DHCP DISCOVER probes
 * that should be sent before falling back on AUTOIP (the DHCP client keeps
 * running in this case). This can be set as low as 1 to get an AutoIP address
 * very  quickly, but you should be prepared to handle a changing IP address
 * when DHCP overrides AutoIP.
 */

// #define LWIP_DHCP_AUTOIP_COOP_TRIES     9

/*
 * @}
 */

/*
   ----------------------------------
   ----- SNMP MIB2 support      -----
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_mib2 SNMP MIB2 callbacks
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * LWIP_MIB2_CALLBACKS==1: Turn on SNMP MIB2 callbacks.
 * Turn this on to get callbacks needed to implement MIB2.
 * Usually MIB2_STATS should be enabled, too.
 */

pub const LWIP_MIB2_CALLBACKS: u32 = 0;

/*
 * @}
 */

/*
   ----------------------------------
   -------- Multicast options -------
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_multicast Multicast
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * LWIP_MULTICAST_TX_OPTIONS==1: Enable multicast TX support like the socket options
 * IP_MULTICAST_TTL/IP_MULTICAST_IF/IP_MULTICAST_LOOP, as well as (currently only)
 * core support for the corresponding IPv6 options.
 */

// #define LWIP_MULTICAST_TX_OPTIONS       ((LWIP_IGMP || LWIP_IPV6_MLD) && (LWIP_UDP || LWIP_RAW))

/*
 * @}
 */

/*
   ----------------------------------
   ---------- IGMP options ----------
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_igmp IGMP
 * @ingroup lwip_opts_ipv4
 * @{
 */
/*
 * LWIP_IGMP==1: Turn on IGMP module.
 */

pub const LWIP_IGMP: u32 = 0;

//#undef LWIP_IGMP
pub const LWIP_IGMP: u32 = 0;

/*
 * @}
 */

/*
   ----------------------------------
   ---------- DNS options -----------
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_dns DNS
 * @ingroup lwip_opts_callback
 * @{
 */
/*
 * LWIP_DNS==1: Turn on DNS module. UDP must be available for DNS
 * transport.
 */

pub const LWIP_DNS: u32 = 0;

/* DNS maximum number of entries to maintain locally. */

pub const DNS_TABLE_SIZE: u32 = 4;

/* DNS maximum host name length supported in the name table. */

pub const DNS_MAX_NAME_LENGTH: u32 = 256;



/* DNS maximum number of retries when asking for a name, before "timeout". */

pub const DNS_MAX_RETRIES: u32 = 4;

/* DNS do a name checking between the query and the response. */

pub const DNS_DOES_NAME_CHECK: u32 = 1;

/* LWIP_DNS_SECURE: controls the security level of the DNS implementation
 * Use all DNS security features by default.
 * This is overridable but should only be needed by very small targets
 * or when using against non standard DNS servers. */

// #define LWIP_DNS_SECURE (LWIP_DNS_SECURE_RAND_XID | LWIP_DNS_SECURE_NO_MULTIPLE_OUTSTANDING | LWIP_DNS_SECURE_RAND_SRC_PORT)

/* A list of DNS security features follows */
// #define LWIP_DNS_SECURE_RAND_XID                1
// #define LWIP_DNS_SECURE_NO_MULTIPLE_OUTSTANDING 2
// #define LWIP_DNS_SECURE_RAND_SRC_PORT           4

/* DNS_LOCAL_HOSTLIST: Implements a local host-to-address list. If enabled, you have to define an initializer:
 *  \#define DNS_LOCAL_HOSTLIST_INIT {DNS_LOCAL_HOSTLIST_ELEM("host_ip4", IPADDR4_INIT_BYTES(1,2,3,4)), \
 *                                    DNS_LOCAL_HOSTLIST_ELEM("host_ip6", IPADDR6_INIT_HOST(123, 234, 345, 456)}
 *
 *  Instead, you can also use an external function:
 *  \#define DNS_LOOKUP_LOCAL_EXTERN(x) extern my_lookup_function: err_t(name: &String, addr: &mut LwipAddr, dns_addrtype: u8)
 *  that looks up the IP address and returns ERR_OK if found (LWIP_DNS_ADDRTYPE_xxx is passed in dns_addrtype).
 */

pub const DNS_LOCAL_HOSTLIST: u32 = 0;

/* If this is turned on, the local host-list can be dynamically changed
 *  at runtime. */

pub const DNS_LOCAL_HOSTLIST_IS_DYNAMIC: u32 = 0;

/* Set this to 1 to enable querying ".local" names via mDNS
 *  using a One-Shot Multicast DNS Query */

pub const LWIP_DNS_SUPPORT_MDNS_QUERIES: u32 = 0;

/*
 * @}
 */

/*
   ---------------------------------
   ---------- UDP options ----------
   ---------------------------------
*/
/*
 * @defgroup lwip_opts_udp UDP
 * @ingroup lwip_opts_callback
 * @{
 */
/*
 * LWIP_UDP==1: Turn on UDP.
 */

// #define LWIP_UDP                        1

/*
 * LWIP_UDPLITE==1: Turn on UDP-Lite. (Requires LWIP_UDP)
 */

pub const LWIP_UDPLITE: u32 = 0;

/*
 * UDP_TTL: Default Time-To-Live value.
 */

pub const UDP_TTL: u32 = IP_DEFAULT_TTL;

/*
 * LWIP_NETBUF_RECVINFO==1: append destination addr and port to every netbuf.
 */

pub const LWIP_NETBUF_RECVINFO: u32 = 0;

/*
 * @}
 */

/*
   ---------------------------------
   ---------- TCP options ----------
   ---------------------------------
*/
/*
 * @defgroup lwip_opts_tcp TCP
 * @ingroup lwip_opts_callback
 * @{
 */
/*
 * LWIP_TCP==1: Turn on TCP.
 */

// #define LWIP_TCP                        1

/*
 * TCP_TTL: Default Time-To-Live value.
 */

pub const TCP_TTL: u32 = IP_DEFAULT_TTL;

/*
 * TCP_WND: The size of a TCP window.  This must be at least
 * (2 * TCP_MSS) for things to work well.
 * ATTENTION: when using TCP_RCV_SCALE, TCP_WND is the total size
 * with scaling applied. Maximum window value in the TCP header
 * will be TCP_WND >> TCP_RCV_SCALE
 */

pub const TCP_WND: u32 = (4 * TCP_MSS);

/*
 * TCP_MAXRTX: Maximum number of retransmissions of data segments.
 */

pub const TCP_MAXRTX: u32 = 12;

/*
 * TCP_SYNMAXRTX: Maximum number of retransmissions of SYN segments.
 */

pub const TCP_SYNMAXRTX: u32 = 6;

/*
 * TCP_QUEUE_OOSEQ==1: TCP will queue segments that arrive out of order.
 * Define to 0 if your device is low on memory.
 */

pub const TCP_QUEUE_OOSEQ: u32 = LWIP_TCP;

/*
 * LWIP_TCP_SACK_OUT==1: TCP will support sending selective acknowledgements (SACKs).
 */

pub const LWIP_TCP_SACK_OUT: u32 = 0;

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

// #define LWIP_TCP_MAX_SACK_NUM           4

/*
 * TCP_MSS: TCP Maximum segment size. (default is 536, a conservative default,
 * you might want to increase this.)
 * For the receive side, this MSS is advertised to the remote side
 * when opening a connection. For the transmit size, this MSS sets
 * an upper limit on the MSS advertised by the remote host.
 */

pub const TCP_MSS: u32 = 536;

/*
 * TCP_CALCULATE_EFF_SEND_MSS: "The maximum size of a segment that TCP really
 * sends, the 'effective send MSS,' MUST be the smaller of the send MSS (which
 * reflects the available reassembly buffer size at the remote host) and the
 * largest size permitted by the IP layer" (RFC 1122)
 * Setting this to 1 enables code that checks TCP_MSS against the MTU of the
 * netif used for a connection and limits the MSS if it would be too big otherwise.
 */

pub const TCP_CALCULATE_EFF_SEND_MSS: u32 = 1;

/*
 * TCP_SND_BUF: TCP sender buffer space (bytes).
 * To achieve good performance, this should be at least 2 * TCP_MSS.
 */

pub const TCP_SND_BUF: u32 = (2 * TCP_MSS);

/*
 * TCP_SND_QUEUELEN: TCP sender buffer space (pbufs). This must be at least
 * as much as (2 * TCP_SND_BUF/TCP_MSS) for things to work.
 */

pub const TCP_SND_QUEUELEN: u32 = ((4 * (TCP_SND_BUF) + (TCP_MSS - 1)) / (TCP_MSS));

/*
 * TCP_SNDLOWAT: TCP writable space (bytes). This must be less than
 * TCP_SND_BUF. It is the amount of space which must be available in the
 * TCP snd_buf for select to return writable (combined with TCP_SNDQUEUELOWAT).
 */

pub const TCP_SNDLOWAT: u32 = LWIP_MIN(
    LWIP_MAX(((TCP_SND_BUF) / 2), (2 * TCP_MSS) + 1),
    (TCP_SND_BUF) - 1,
);

/*
 * TCP_SNDQUEUELOWAT: TCP writable bufs (pbuf count). This must be less
 * than TCP_SND_QUEUELEN. If the number of pbufs queued on a pcb drops below
 * this number, select returns writable (combined with TCP_SNDLOWAT).
 */

pub const TCP_SNDQUEUELOWAT: u32 = LWIP_MAX(((TCP_SND_QUEUELEN) / 2), 5);

/*
 * TCP_OOSEQ_MAX_BYTES: The default maximum number of bytes queued on ooseq per
 * pcb if TCP_OOSEQ_BYTES_LIMIT is not defined. Default is 0 (no limit).
 * Only valid for TCP_QUEUE_OOSEQ==1.
 */

pub const TCP_OOSEQ_MAX_BYTES: u32 = 0;

/*
 * TCP_OOSEQ_BYTES_LIMIT(pcb): Return the maximum number of bytes to be queued
 * on ooseq per pcb, given the pcb. Only valid for TCP_QUEUE_OOSEQ==1 &&
 * TCP_OOSEQ_MAX_BYTES==1.
 * Use this to override TCP_OOSEQ_MAX_BYTES to a dynamic value per pcb.
 */

// #define TCP_OOSEQ_BYTES_LIMIT(pcb)      TCP_OOSEQ_MAX_BYTES
// #elif defined __DOXYGEN__
// #define TCP_OOSEQ_BYTES_LIMIT(pcb)

/*
 * TCP_OOSEQ_MAX_PBUFS: The default maximum number of pbufs queued on ooseq per
 * pcb if TCP_OOSEQ_BYTES_LIMIT is not defined. Default is 0 (no limit).
 * Only valid for TCP_QUEUE_OOSEQ==1.
 */

pub const TCP_OOSEQ_MAX_PBUFS: u32 = 0;

/*
 * TCP_OOSEQ_PBUFS_LIMIT(pcb): Return the maximum number of pbufs to be queued
 * on ooseq per pcb, given the pcb.  Only valid for TCP_QUEUE_OOSEQ==1 &&
 * TCP_OOSEQ_MAX_PBUFS==1.
 * Use this to override TCP_OOSEQ_MAX_PBUFS to a dynamic value per pcb.
 */

// #define TCP_OOSEQ_PBUFS_LIMIT(pcb)      TCP_OOSEQ_MAX_PBUFS
// #elif defined __DOXYGEN__
// #define TCP_OOSEQ_PBUFS_LIMIT(pcb)

/*
 * TCP_LISTEN_BACKLOG: Enable the backlog option for tcp listen pcb.
 */

pub const TCP_LISTEN_BACKLOG: u32 = 0;

/*
 * The maximum allowed backlog for TCP listen netconns.
 * This backlog is used unless another is explicitly specified.
 * 0xff is the maximum .
 */

pub const TCP_DEFAULT_LISTEN_BACKLOG: u32 = 0xff;

/*
* TCP_OVERSIZE: The maximum number of bytes that tcp_write may
* allocate ahead of time in an attempt to create shorter pbuf chains
* for transmission. The meaningful range is 0 to TCP_MSS. Some
* suggested values are:
*
* 0:         Disable oversized allocation. Each tcp_write() allocates a new
             pbuf (old behaviour).
* 1:         Allocate size-aligned pbufs with minimal excess. Use this if your
*            scatter-gather DMA requires aligned fragments.
* 128:       Limit the pbuf/memory overhead to 20%.
* TCP_MSS:   Try to create unfragmented TCP packets.
* TCP_MSS/4: Try to create 4 fragments or less per TCP packet.
*/

pub const TCP_OVERSIZE: u32 = TCP_MSS;

/*
 * LWIP_TCP_TIMESTAMPS==1: support the TCP timestamp option.
 * The timestamp option is currently only used to help remote hosts, it is not
 * really used locally. Therefore, it is only enabled when a TS option is
 * received in the initial SYN packet from a remote host.
 */

pub const LWIP_TCP_TIMESTAMPS: u32 = 0;

/*
 * TCP_WND_UPDATE_THRESHOLD: difference in window to trigger an
 * explicit window update
 */

pub const TCP_WND_UPDATE_THRESHOLD: u32 = LWIP_MIN((TCP_WND / 4), (TCP_MSS * 4));

/*
 * LWIP_EVENT_API and LWIP_CALLBACK_API: Only one of these should be set to 1.
 *     LWIP_EVENT_API==1: The user defines lwip_tcp_event() to receive all
 *         events (accept, sent, etc) that happen in the system.
 *     LWIP_CALLBACK_API==1: The PCB callback function is called directly
 *         for the event. This is the default.
 */

pub const LWIP_EVENT_API: u32 = 0;
// #define LWIP_CALLBACK_API               1

pub const LWIP_EVENT_API: u32 = 0;

pub const LWIP_CALLBACK_API: u32 = 0;

/*
 * LWIP_WND_SCALE and TCP_RCV_SCALE:
 * Set LWIP_WND_SCALE to 1 to enable window scaling.
 * Set TCP_RCV_SCALE to the desired scaling factor (shift count in the
 * range of [0..14]).
 * When LWIP_WND_SCALE is enabled but TCP_RCV_SCALE is 0, we can use a large
 * send window while having a small receive window only.
 */

pub const LWIP_WND_SCALE: u32 = 0;
pub const LWIP_WND_SCALE: u32 = 0;
pub const TCP_RCV_SCALE: u32 = 0;

/*
 * LWIP_TCP_PCB_NUM_EXT_ARGS:
 * When this is > 0, every tcp pcb (including listen pcb) includes a number of
 * additional argument entries in an array (see tcp_ext_arg_alloc_id)
 */

pub const LWIP_TCP_PCB_NUM_EXT_ARGS: usize = 0;

/* LWIP_ALTCP==1: enable the altcp API.
 * altcp is an abstraction layer that prevents applications linking against the
 * tcp.h functions but provides the same functionality. It is used to e.g. add
 * SSL/TLS or proxy-connect support to an application written for the tcp callback
 * API without that application knowing the protocol details.
 *
 * With LWIP_ALTCP==0, applications written against the altcp API can still be
 * compiled but are directly linked against the tcp.h callback API and then
 * cannot use layered protocols.
 *
 * See @ref altcp_api
 */

pub const LWIP_ALTCP: u32 = 0;

/* LWIP_ALTCP_TLS==1: enable TLS support for altcp API.
 * This needs a port of the functions in altcp_tls.h to a TLS library.
 * A port to ARM mbedtls is provided with lwIP, see apps/altcp_tls/ directory
 * and LWIP_ALTCP_TLS_MBEDTLS option.
 */

pub const LWIP_ALTCP_TLS: u32 = 0;

/*
 * @}
 */

/*
   ----------------------------------
   ---------- Pbuf options ----------
   ----------------------------------
*/
/*
 * @defgroup lwip_opts_pbuf PBUF
 * @ingroup lwip_opts
 * @{
 */
/*
 * PBUF_LINK_HLEN: the number of bytes that should be allocated for a
 * link level header. The default is 14, the standard value for
 * Ethernet.
 */

pub const PBUF_LINK_HLEN: usize = (18 + ETH_PAD_SIZE);
/* LWIP_HOOK_VLAN_SET */
pub const PBUF_LINK_HLEN: usize = (14 + ETH_PAD_SIZE);

/*
 * PBUF_LINK_ENCAPSULATION_HLEN: the number of bytes that should be allocated
 * for an additional encapsulation header before ethernet headers (e.g. 802.11)
 */

pub const PBUF_LINK_ENCAPSULATION_HLEN: usize = 0;

/*
 * PBUF_POOL_BUFSIZE: the size of each pbuf in the pbuf pool. The default is
 * designed to accommodate single full size TCP frame in one pbuf, including
 * TCP_MSS, IP header, and link header.
 */

pub const PBUF_POOL_BUFSIZE: usize =
    LWIP_MEM_ALIGN_SIZE(TCP_MSS + 40 + PBUF_LINK_ENCAPSULATION_HLEN + PBUF_LINK_HLEN);

/*
 * LWIP_PBUF_REF_T: Refcount type in pbuf.
 * Default width of can: u8 be increased if 255 refs are not enough for you.
 */

// #define LWIP_PBUF_REF_T                 u8

/*
 * @}
 */

/*
   ------------------------------------------------
   ---------- Network Interfaces options ----------
   ------------------------------------------------
*/
/*
 * @defgroup lwip_opts_netif NETIF
 * @ingroup lwip_opts
 * @{
 */
/*
 * LWIP_SINGLE_NETIF==1: use a single netif only. This is the common case for
 * small real-life targets. Some code like routing etc. can be left out.
 */

pub const LWIP_SINGLE_NETIF: bool = false;

/*
 * LWIP_NETIF_HOSTNAME==1: use DHCP_OPTION_HOSTNAME with netif's hostname
 * field.
 */

pub const LWIP_NETIF_HOSTNAME: u32 = 0;

/*
 * LWIP_NETIF_API==1: Support netif api (in netifapi.c)
 */

pub const LWIP_NETIF_API: u32 = 0;

/*
 * LWIP_NETIF_STATUS_CALLBACK==1: Support a callback function whenever an interface
 * changes its up/down status (i.e., due to DHCP IP acquisition)
 */

pub const LWIP_NETIF_STATUS_CALLBACK: u32 = 0;

/*
 * LWIP_NETIF_EXT_STATUS_CALLBACK==1: Support an extended callback function
 * for several netif related event that supports multiple subscribers.
 * @see netif_ext_status_callback
 */

pub const LWIP_NETIF_EXT_STATUS_CALLBACK: u32 = 0;

/*
 * LWIP_NETIF_LINK_CALLBACK==1: Support a callback function from an interface
 * whenever the link changes (i.e., link down)
 */

pub const LWIP_NETIF_LINK_CALLBACK: u32 = 0;

/*
 * LWIP_NETIF_REMOVE_CALLBACK==1: Support a callback function that is called
 * when a netif has been removed
 */

pub const LWIP_NETIF_REMOVE_CALLBACK: u32 = 0;

/*
 * LWIP_NETIF_HWADDRHINT==1: Cache link-layer-address hints (e.g. table
 * indices) in NetIfc. TCP and UDP can make use of this to prevent
 * scanning the ARP table for every sent packet. While this is faster for big
 * ARP tables or many concurrent connections, it might be counterproductive
 * if you have a tiny ARP table or if there never are concurrent connections.
 */

pub const LWIP_NETIF_HWADDRHINT: u32 = 0;

/*
 * LWIP_NETIF_TX_SINGLE_PBUF: if this is set to 1, lwIP *tries* to put all data
 * to be sent into one single pbuf. This is for compatibility with DMA-enabled
 * MACs that do not support scatter-gather.
 * Beware that this might involve CPU-memcpy before transmitting that would not
 * be needed without this flag! Use this only if you need to!
 *
 * ATTENTION: a driver should *NOT* rely on getting single pbufs but check TX
 * pbufs for being in one piece. If not, @ref pbuf_clone can be used to get
 * a single pbuf:
 *   if (p.next != NULL) {
 *     q: &mut PacketBuffer = pbuf_clone(PBUF_RAW, PBUF_RAM, p);
 *     if (q == NULL) {
 *       return ERR_MEM;
 *     }
 *     p = q; ATTENTION: do NOT free the old 'p' as the ref belongs to the caller!
 *   }
 */

pub const LWIP_NETIF_TX_SINGLE_PBUF: u32 = 0;

/*
 * LWIP_NUM_NETIF_CLIENT_DATA: Number of clients that may store
 * data in client_data member array of NetIfc (max. 256).
 */

pub const LWIP_NUM_NETIF_CLIENT_DATA: u32 = 0;

/*
 * @}
 */

/*
   ------------------------------------
   ---------- LOOPIF options ----------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_loop Loopback interface
 * @ingroup lwip_opts_netif
 * @{
 */
/*
 * LWIP_HAVE_LOOPIF==1: Support loop interface (127.0.0.1).
 * This is only needed when no real netifs are available. If at least one other
 * netif is available, loopback traffic uses this netif.
 */

// #define LWIP_HAVE_LOOPIF                (LWIP_NETIF_LOOPBACK && !LWIP_SINGLE_NETIF)
pub const LWIP_HAVE_LOOPIF: bool = LWIP_NETIF_LOOPBACK && LWIP_SINGLE_NETIF;

/*
 * LWIP_LOOPIF_MULTICAST==1: Support multicast/IGMP on loop interface (127.0.0.1).
 */

pub const LWIP_LOOPIF_MULTICAST: u32 = 0;

/*
 * LWIP_NETIF_LOOPBACK==1: Support sending packets with a destination IP
 * address equal to the netif IP address, looping them back up the stack.
 */

pub const LWIP_NETIF_LOOPBACK: bool = false;

/*
 * LWIP_LOOPBACK_MAX_PBUFS: Maximum number of pbufs on queue for loopback
 * sending for each netif (0 = disabled)
 */

pub const LWIP_LOOPBACK_MAX_PBUFS: u32 = 0;

/*
 * LWIP_NETIF_LOOPBACK_MULTITHREADING: Indicates whether threading is enabled in
 * the system, as netifs must change how they behave depending on this setting
 * for the LWIP_NETIF_LOOPBACK option to work.
 * Setting this is needed to avoid reentering non-reentrant functions like
 * tcp_input().
 *    LWIP_NETIF_LOOPBACK_MULTITHREADING==1: Indicates that the user is using a
 *       multithreaded environment like tcpip.c. In this case, netif.input()
 *       is called directly.
 *    LWIP_NETIF_LOOPBACK_MULTITHREADING==0: Indicates a polling (or NO_SYS) setup.
 *       The packets are put on a list and netif_poll() must be called in
 *       the main application loop.
 */

// #define LWIP_NETIF_LOOPBACK_MULTITHREADING    (!NO_SYS)

/*
 * @}
 */

/*
   ------------------------------------
   ---------- Thread options ----------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_thread Threading
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * TCPIP_THREAD_NAME: The name assigned to the main tcpip thread.
 */

pub const TCPIP_THREAD_NAME: String = "tcpip_thread".to_string();

/*
 * TCPIP_THREAD_STACKSIZE: The stack size used by the main tcpip thread.
 * The stack size value itself is platform-dependent, but is passed to
 * sys_thread_new() when the thread is created.
 */

pub const TCPIP_THREAD_STACKSIZE: u32 = 0;

/*
 * TCPIP_THREAD_PRIO: The priority assigned to the main tcpip thread.
 * The priority value itself is platform-dependent, but is passed to
 * sys_thread_new() when the thread is created.
 */

pub const TCPIP_THREAD_PRIO: u32 = 1;

/*
 * TCPIP_MBOX_SIZE: The mailbox size for the tcpip thread messages
 * The queue size value itself is platform-dependent, but is passed to
 * sys_mbox_new() when tcpip_init is called.
 */

pub const TCPIP_MBOX_SIZE: u32 = 0;

/*
 * Define this to something that triggers a watchdog. This is called from
 * tcpip_thread after processing a message.
 */

// #define LWIP_TCPIP_THREAD_ALIVE()

/*
 * SLIPIF_THREAD_NAME: The name assigned to the slipif_loop thread.
 */

pub const SLIPIF_THREAD_NAME: String = "slipif_loop";

/*
 * SLIP_THREAD_STACKSIZE: The stack size used by the slipif_loop thread.
 * The stack size value itself is platform-dependent, but is passed to
 * sys_thread_new() when the thread is created.
 */

pub const SLIPIF_THREAD_STACKSIZE: u32 = 0;

/*
 * SLIPIF_THREAD_PRIO: The priority assigned to the slipif_loop thread.
 * The priority value itself is platform-dependent, but is passed to
 * sys_thread_new() when the thread is created.
 */

pub const SLIPIF_THREAD_PRIO: u32 = 1;

/*
 * DEFAULT_THREAD_NAME: The name assigned to any other lwIP thread.
 */

pub const DEFAULT_THREAD_NAME: String = "lwIP";

/*
 * DEFAULT_THREAD_STACKSIZE: The stack size used by any other lwIP thread.
 * The stack size value itself is platform-dependent, but is passed to
 * sys_thread_new() when the thread is created.
 */

pub const DEFAULT_THREAD_STACKSIZE: u32 = 0;

/*
 * DEFAULT_THREAD_PRIO: The priority assigned to any other lwIP thread.
 * The priority value itself is platform-dependent, but is passed to
 * sys_thread_new() when the thread is created.
 */

pub const DEFAULT_THREAD_PRIO: u32 = 1;

/*
 * DEFAULT_RAW_RECVMBOX_SIZE: The mailbox size for the incoming packets on a
 * NETCONN_RAW. The queue size value itself is platform-dependent, but is passed
 * to sys_mbox_new() when the recvmbox is created.
 */

pub const DEFAULT_RAW_RECVMBOX_SIZE: u32 = 0;

/*
 * DEFAULT_UDP_RECVMBOX_SIZE: The mailbox size for the incoming packets on a
 * NETCONN_UDP. The queue size value itself is platform-dependent, but is passed
 * to sys_mbox_new() when the recvmbox is created.
 */

pub const DEFAULT_UDP_RECVMBOX_SIZE: u32 = 0;

/*
 * DEFAULT_TCP_RECVMBOX_SIZE: The mailbox size for the incoming packets on a
 * NETCONN_TCP. The queue size value itself is platform-dependent, but is passed
 * to sys_mbox_new() when the recvmbox is created.
 */

pub const DEFAULT_TCP_RECVMBOX_SIZE: u32 = 0;

/*
 * DEFAULT_ACCEPTMBOX_SIZE: The mailbox size for the incoming connections.
 * The queue size value itself is platform-dependent, but is passed to
 * sys_mbox_new() when the acceptmbox is created.
 */

pub const DEFAULT_ACCEPTMBOX_SIZE: u32 = 0;

/*
 * @}
 */

/*
   ----------------------------------------------
   ---------- Sequential layer options ----------
   ----------------------------------------------
*/
/*
 * @defgroup lwip_opts_netconn Netconn
 * @ingroup lwip_opts_threadsafe_apis
 * @{
 */
/*
 * LWIP_NETCONN==1: Enable Netconn API (require to use api_lib.c)
 */

// #define LWIP_NETCONN                    1

/* LWIP_TCPIP_TIMEOUT==1: Enable tcpip_timeout/tcpip_untimeout to create
 * timers running in tcpip_thread from another thread.
 */

pub const LWIP_TCPIP_TIMEOUT: u32 = 0;

/* LWIP_NETCONN_SEM_PER_THREAD==1: Use one (thread-local) semaphore per
 * thread calling socket/netconn functions instead of allocating one
 * semaphore per netconn (and per select etc.)
 * ATTENTION: a thread-local semaphore for API calls is needed:
 * - LWIP_NETCONN_THREAD_SEM_GET() returning a sys_sem_t*
 * - LWIP_NETCONN_THREAD_SEM_ALLOC() creating the semaphore
 * - LWIP_NETCONN_THREAD_SEM_FREE() freeing the semaphore
 * The latter 2 can be invoked up by calling netconn_thread_init()/netconn_thread_cleanup().
 * Ports may call these for threads created with sys_thread_new().
 */

pub const LWIP_NETCONN_SEM_PER_THREAD: u32 = 0;

/* LWIP_NETCONN_FULLDUPLEX==1: Enable code that allows reading from one thread,
 * writing from a 2nd thread and closing from a 3rd thread at the same time.
 * ATTENTION: This is currently really alpha! Some requirements:
 * - LWIP_NETCONN_SEM_PER_THREAD==1 is required to use one socket/netconn from
 *   multiple threads at once
 * - sys_mbox_free() has to unblock receive tasks waiting on recvmbox/acceptmbox
 *   and prevent a task pending on this during/after deletion
 */

pub const LWIP_NETCONN_FULLDUPLEX: u32 = 0;

/*
 * @}
 */

/*
   ------------------------------------
   ---------- Socket options ----------
   ------------------------------------
*/
/*
 * @defgroup lwip_opts_socket Sockets
 * @ingroup lwip_opts_threadsafe_apis
 * @{
 */
/*
 * LWIP_SOCKET==1: Enable Socket API (require to use sockets.c)
 */

// #define LWIP_SOCKET                     1

/*
 * LWIP_COMPAT_SOCKETS==1: Enable BSD-style sockets functions names through defines.
 * LWIP_COMPAT_SOCKETS==2: Same as ==1 but correctly named functions are created.
 * While this helps code completion, it might conflict with existing libraries.
 * (only used if you use sockets.c)
 */

// #define LWIP_COMPAT_SOCKETS             1

/*
 * LWIP_POSIX_SOCKETS_IO_NAMES==1: Enable POSIX-style sockets functions names.
 * Disable this option if you use a POSIX operating system that uses the same
 * names (read, write & close). (only used if you use sockets.c)
 */

// #define LWIP_POSIX_SOCKETS_IO_NAMES     1

/*
 * LWIP_SOCKET_OFFSET==n: Increases the file descriptor number created by LwIP with n.
 * This can be useful when there are multiple APIs which create file descriptors.
 * When they all start with a different offset and you won't make them overlap you can
 * re implement read/write/close/ioctl/fnctl to send the requested action to the right
 * library (sharing select will need more work though).
 */

pub const LWIP_SOCKET_OFFSET: u32 = 0;

/*
 * LWIP_TCP_KEEPALIVE==1: Enable TCP_KEEPIDLE, TCP_KEEPINTVL and TCP_KEEPCNT
 * options processing. Note that TCP_KEEPIDLE and TCP_KEEPINTVL have to be set
 * in seconds. (does not require sockets.c, and will affect tcp.c)
 */

pub const LWIP_TCP_KEEPALIVE: u32 = 0;

/*
 * LWIP_SO_SNDTIMEO==1: Enable send timeout for sockets/netconns and
 * SO_SNDTIMEO processing.
 */

pub const LWIP_SO_SNDTIMEO: u32 = 0;

/*
 * LWIP_SO_RCVTIMEO==1: Enable receive timeout for sockets/netconns and
 * SO_RCVTIMEO processing.
 */

pub const LWIP_SO_RCVTIMEO: u32 = 0;

/*
 * LWIP_SO_SNDRCVTIMEO_NONSTANDARD==1: SO_RCVTIMEO/SO_SNDTIMEO take an int
 * (milliseconds, much like winsock does) instead of a struct timeval (default).
 */

pub const LWIP_SO_SNDRCVTIMEO_NONSTANDARD: u32 = 0;

/*
 * LWIP_SO_RCVBUF==1: Enable SO_RCVBUF processing.
 */

pub const LWIP_SO_RCVBUF: u32 = 0;

/*
 * LWIP_SO_LINGER==1: Enable SO_LINGER processing.
 */

pub const LWIP_SO_LINGER: u32 = 0;

/*
 * If LWIP_SO_RCVBUF is used, this is the default value for recv_bufsize.
 */

pub const RECV_BUFSIZE_DEFAULT: u32 = INT_MAX;

/*
 * By default, TCP socket/netconn close waits 20 seconds max to send the FIN
 */

// #define LWIP_TCP_CLOSE_TIMEOUT_MS_DEFAULT 20000

/*
 * SO_REUSE==1: Enable SO_REUSEADDR option.
 */

pub const SO_REUSE: u32 = 0;

/*
 * SO_REUSE_RXTOALL==1: Pass a copy of incoming broadcast/multicast packets
 * to all local matches if SO_REUSEADDR is turned on.
 * WARNING: Adds a memcpy for every packet if passing to more than one pcb!
 */

pub const SO_REUSE_RXTOALL: u32 = 0;

/*
 * LWIP_FIONREAD_LINUXMODE==0 (default): ioctl/FIONREAD returns the amount of
 * pending data in the network buffer. This is the way windows does it. It's
 * the default for lwIP since it is smaller.
 * LWIP_FIONREAD_LINUXMODE==1: ioctl/FIONREAD returns the size of the next
 * pending datagram in bytes. This is the way linux does it. This code is only
 * here for compatibility.
 */

pub const LWIP_FIONREAD_LINUXMODE: u32 = 0;

/*
 * LWIP_SOCKET_SELECT==1 (default): enable select() for sockets (uses a netconn
 * callback to keep track of events).
 * This saves RAM (counters per socket) and code (netconn event callback), which
 * should improve performance a bit).
 */

// #define LWIP_SOCKET_SELECT              1

/*
 * LWIP_SOCKET_POLL==1 (default): enable poll() for sockets (including
 * struct pollfd, nfds_t, and constants)
 */

// #define LWIP_SOCKET_POLL                1

/*
 * @}
 */

/*
   ----------------------------------------
   ---------- Statistics options ----------
   ----------------------------------------
*/
/*
 * @defgroup lwip_opts_stats Statistics
 * @ingroup lwip_opts_debug
 * @{
 */
/*
 * LWIP_STATS==1: Enable statistics collection in lwip_stats.
 */

// #define LWIP_STATS                      1

/*
 * LWIP_STATS_DISPLAY==1: Compile in the statistics output functions.
 */

pub const LWIP_STATS_DISPLAY: u32 = 0;

/*
 * LINK_STATS==1: Enable link stats.
 */

pub const LINK_STATS: u32 = 1;

/*
 * ETHARP_STATS==1: Enable etharp stats.
 */

pub const ETHARP_STATS: bool = (LWIP_ARP);

/*
 * IP_STATS==1: Enable IP stats.
 */

pub const IP_STATS: u32 = 1;

/*
 * IPFRAG_STATS==1: Enable IP fragmentation stats. Default is
 * on if using either frag or reass.
 */

pub const IPFRAG_STATS: bool = (IP_REASSEMBLY || IP_FRAG);

/*
 * ICMP_STATS==1: Enable ICMP stats.
 */

pub const ICMP_STATS: u32 = 1;

/*
 * IGMP_STATS==1: Enable IGMP stats.
 */

pub const IGMP_STATS: bool = (LWIP_IGMP);

/*
 * UDP_STATS==1: Enable UDP stats. Default is on if
 * UDP enabled, otherwise off.
 */

pub const UDP_STATS: bool = (LWIP_UDP);

/*
 * TCP_STATS==1: Enable TCP stats. Default is on if TCP
 * enabled, otherwise off.
 */

pub const TCP_STATS: bool = (LWIP_TCP);

/*
 * MEM_STATS==1: Enable mem.c stats.
 */

pub const MEM_STATS: bool = ((MEM_LIBC_MALLOC == 0) && (MEM_USE_POOLS == 0));

/*
 * MEMP_STATS==1: Enable memp.c pool stats.
 */

pub const MEMP_STATS: bool = (MEMP_MEM_MALLOC == 0);

/*
 * SYS_STATS==1: Enable system stats (sem and mbox counts, etc).
 */

pub const SYS_STATS: bool = (NO_SYS == 0);

/*
 * IP6_STATS==1: Enable IPv6 stats.
 */

pub const IP6_STATS: bool = (LWIP_IPV6);

/*
 * ICMP6_STATS==1: Enable ICMP for IPv6 stats.
 */

pub const ICMP6_STATS: bool = (LWIP_IPV6 && LWIP_ICMP6);

/*
 * IP6_FRAG_STATS==1: Enable IPv6 fragmentation stats.
 */

pub const IP6_FRAG_STATS: bool = (LWIP_IPV6 && (LWIP_IPV6_FRAG || LWIP_IPV6_REASS));

/*
 * MLD6_STATS==1: Enable MLD for IPv6 stats.
 */

pub const MLD6_STATS: bool = (LWIP_IPV6 && LWIP_IPV6_MLD);

/*
 * ND6_STATS==1: Enable Neighbor discovery for IPv6 stats.
 */

pub const ND6_STATS: bool = (LWIP_IPV6);

/*
 * MIB2_STATS==1: Stats for SNMP MIB2.
 */

pub const MIB2_STATS: u32 = 0;

pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const LINK_STATS: u32 = 0;
pub const ETHARP_STATS: u32 = 0;
pub const IP_STATS: u32 = 0;
pub const IPFRAG_STATS: u32 = 0;
pub const ICMP_STATS: u32 = 0;
pub const IGMP_STATS: u32 = 0;
pub const UDP_STATS: u32 = 0;
pub const TCP_STATS: u32 = 0;
pub const MEM_STATS: u32 = 0;
pub const MEMP_STATS: u32 = 0;
pub const SYS_STATS: u32 = 0;
// #define LWIP_STATS_DISPLAY              0
pub const IP6_STATS: u32 = 0;
pub const ICMP6_STATS: u32 = 0;
pub const IP6_FRAG_STATS: u32 = 0;
pub const MLD6_STATS: u32 = 0;
pub const ND6_STATS: u32 = 0;
pub const MIB2_STATS: u32 = 0;

/*
 * @}
 */

/*
   --------------------------------------
   ---------- Checksum options ----------
   --------------------------------------
*/
/*
 * @defgroup lwip_opts_checksum Checksum
 * @ingroup lwip_opts_infrastructure
 * @{
 */
/*
 * LWIP_CHECKSUM_CTRL_PER_NETIF==1: Checksum generation/check can be enabled/disabled
 * per netif.
 * ATTENTION: if enabled, the CHECKSUM_GEN_* and CHECKSUM_CHECK_* defines must be enabled!
 */

pub const LWIP_CHECKSUM_CTRL_PER_NETIF: u32 = 0;

/*
 * CHECKSUM_GEN_IP==1: Generate checksums in software for outgoing IP packets.
 */

pub const CHECKSUM_GEN_IP: u32 = 1;

/*
 * CHECKSUM_GEN_UDP==1: Generate checksums in software for outgoing UDP packets.
 */

pub const CHECKSUM_GEN_UDP: u32 = 1;

/*
 * CHECKSUM_GEN_TCP==1: Generate checksums in software for outgoing TCP packets.
 */

pub const CHECKSUM_GEN_TCP: u32 = 1;

/*
 * CHECKSUM_GEN_ICMP==1: Generate checksums in software for outgoing ICMP packets.
 */

pub const CHECKSUM_GEN_ICMP: u32 = 1;

/*
 * CHECKSUM_GEN_ICMP6==1: Generate checksums in software for outgoing ICMP6 packets.
 */

pub const CHECKSUM_GEN_ICMP6: u32 = 1;

/*
 * CHECKSUM_CHECK_IP==1: Check checksums in software for incoming IP packets.
 */

pub const CHECKSUM_CHECK_IP: u32 = 1;

/*
 * CHECKSUM_CHECK_UDP==1: Check checksums in software for incoming UDP packets.
 */

pub const CHECKSUM_CHECK_UDP: u32 = 1;

/*
 * CHECKSUM_CHECK_TCP==1: Check checksums in software for incoming TCP packets.
 */

pub const CHECKSUM_CHECK_TCP: u32 = 1;

/*
 * CHECKSUM_CHECK_ICMP==1: Check checksums in software for incoming ICMP packets.
 */

pub const CHECKSUM_CHECK_ICMP: u32 = 1;

/*
 * CHECKSUM_CHECK_ICMP6==1: Check checksums in software for incoming ICMPv6 packets
 */

pub const CHECKSUM_CHECK_ICMP6: u32 = 1;

/*
 * LWIP_CHECKSUM_ON_COPY==1: Calculate checksum when copying data from
 * application buffers to pbufs.
 */

pub const LWIP_CHECKSUM_ON_COPY: u32 = 0;

/*
 * @}
 */

/*
   ---------------------------------------
   ---------- IPv6 options ---------------
   ---------------------------------------
*/
/*
 * @defgroup lwip_opts_ipv6 IPv6
 * @ingroup lwip_opts
 * @{
 */
/*
 * LWIP_IPV6==1: Enable IPv6
 */

pub const LWIP_IPV6: u32 = 0;

/*
 * IPV6_REASS_MAXAGE: Maximum time (in multiples of IP6_REASS_TMR_INTERVAL - so seconds, normally)
 * a fragmented IP packet waits for all fragments to arrive. If not all fragments arrived
 * in this time, the whole packet is discarded.
 */

pub const IPV6_REASS_MAXAGE: u32 = 60;

/*
 * LWIP_IPV6_SCOPES==1: Enable support for IPv6 address scopes, ensuring that
 * e.g. link-local addresses are really treated as link-local. Disable this
 * setting only for single-interface configurations.
 * All addresses that have a scope according to the default policy (link-local
 * unicast addresses, interface-local and link-local multicast addresses) should
 * now have a zone set on them before being passed to the core API, although
 * lwIP will currently attempt to select a zone on the caller's behalf when
 * necessary. Applications that directly assign IPv6 addresses to interfaces
 * (which is NOT recommended) must now ensure that link-local addresses carry
 * the netif's zone. See the new ip6_zone.h header file for more information and
 * relevant macros. For now it is still possible to turn off scopes support
 * through the new LWIP_IPV6_SCOPES option. When upgrading an implementation that
 * uses the core API directly, it is highly recommended to enable
 * LWIP_IPV6_SCOPES_DEBUG at least for a while, to ensure e.g. proper address
 * initialization.
 */

// #define LWIP_IPV6_SCOPES                (LWIP_IPV6 && !LWIP_SINGLE_NETIF)

/*
 * LWIP_IPV6_SCOPES_DEBUG==1: Perform run-time checks to verify that addresses
 * are properly zoned (see ip6_zone.h on what that means) where it matters.
 * Enabling this setting is highly recommended when upgrading from an existing
 * installation that is not yet scope-aware; otherwise it may be too expensive.
 */

pub const LWIP_IPV6_SCOPES_DEBUG: u32 = 0;

/*
 * LWIP_IPV6_NUM_ADDRESSES: Number of IPv6 addresses per netif.
 */

// #define LWIP_IPV6_NUM_ADDRESSES         3

/*
 * LWIP_IPV6_FORWARD==1: Forward IPv6 packets across netifs
 */

pub const LWIP_IPV6_FORWARD: u32 = 0;

/*
 * LWIP_IPV6_FRAG==1: Fragment outgoing IPv6 packets that are too big.
 */

// #define LWIP_IPV6_FRAG                  1

/*
 * LWIP_IPV6_REASS==1: reassemble incoming IPv6 packets that fragmented
 */

// #define LWIP_IPV6_REASS                 LWIP_IPV6

/*
 * LWIP_IPV6_SEND_ROUTER_SOLICIT==1: Send router solicitation messages during
 * network startup.
 */

// #define LWIP_IPV6_SEND_ROUTER_SOLICIT   1

/*
 * LWIP_IPV6_AUTOCONFIG==1: Enable stateless address autoconfiguration as per RFC 4862.
 */

// #define LWIP_IPV6_AUTOCONFIG            LWIP_IPV6

/*
 * LWIP_IPV6_ADDRESS_LIFETIMES==1: Keep valid and preferred lifetimes for each
 * IPv6 address. Required for LWIP_IPV6_AUTOCONFIG. May still be enabled
 * otherwise, in which case the application may assign address lifetimes with
 * the appropriate macros. Addresses with no lifetime are assumed to be static.
 * If this option is disabled, all addresses are assumed to be static.
 */

// #define LWIP_IPV6_ADDRESS_LIFETIMES     LWIP_IPV6_AUTOCONFIG

/*
 * LWIP_IPV6_DUP_DETECT_ATTEMPTS=[0..7]: Number of duplicate address detection attempts.
 */

// #define LWIP_IPV6_DUP_DETECT_ATTEMPTS   1

pub struct LwipOptions {
    // Enable ICMPv6 (mandatory per RFC)
    pub LWIP_ICMP6: bool,
    //  bytes from original packet to send back in ICMPv6 error messages.
    pub LWIP_ICMP6_DATASIZE: usize,
    // default hop limit for ICMPv6 messages
    pub LWIP_ICMP6_HL: usize,
    // Enable multicast listener discovery protocol. If LWIP_IPV6 is enabled but this setting is disabled, the MAC layer must indiscriminately pass all inbound IPv6 multicast traffic to lwIP.
    pub LWIP_IPV6_MLD: bool,
    // Max number of IPv6 multicast groups that can be joined. There must be enough groups so that each netif can join the solicited-node multicast group for each of its local addresses, plus one for MDNS if applicable, plus any number of groups to be joined on UDP sockets.
    pub MEMP_NUM_MLD6_GROUP: usize,
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
    pub LWIP_ND6_MAX_NEIGHBOR_ADVERTISEMENT: usize
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
    // enable DHCPv6 stateful/stateless address autoconfiguration
    pub LWIP_IPV6_DHCP: bool,
    // enable DHCPv6 stateful address autoconfiguration
    pub LWIP_IPV6_DHCP6_STATEFUL: bool,
    //
    pub LWIP_IPV6_DHCP6_STATELESS: bool,
    // if true then request DHCP6 NTP servers
    pub LWIP_DHCP6_GET_NTP_SRV: bool,
    // max DHCPv4 DNS servers requested
    pub DNS_MAX_SERVERS: usize,
    // max DHCPv6 DNS servers requested
    pub LWIP_DHCP6_MAX_DNS_SERVERS: usize,
    // max DHCPv6 NTP servers requested
    pub LWIP_DHCP6_MAX_NTP_SERVERS: usize,
    // enable debugging
    pub LWIP_DEBUG: bool,
    // enable performance testing
    pub LWIP_PERF: bool,
    // changes to make unit testing possible
    pub LWIP_TESTMODE: bool,
    // enable DHCPv6 debugging
}

impl LwipOptions {
    pub fn new() -> LwipOptions {
        LwipOptions {
            LWIP_ICMP6: false,
            LWIP_ICMP6_DATASIZE: 8,
            LWIP_ICMP6_HL: 255,
            LWIP_IPV6_MLD: false,
            MEMP_NUM_MLD6_GROUP: 0,
            LWIP_ND6_QUEUEING: false,
            MEMP_NUM_ND6_QUEUE: 20,
            LWIP_ND6_NUM_NEIGHBORS: 10,
            LWIP_ND6_NUM_DESTINATIONS: 10,
            LWIP_ND6_NUM_PREFIXES: 5,
            LWIP_ND6_MAX_MULTICAST_SOLICIT: 3,
            LWIP_ND6_MAX_UNICAST_SOLICIT: 1,
            LWIP_ND6_MAX_ANYCAST_DELAY_TIME: 1000,
            LWIP_ND6_MAX_NEIGHBOR_ADVERTISEMENT: 1,
            LWIP_ND6_REACHABLE_TIME: 30000,
            LWIP_ND6_RETRANS_TIMER: 1000,
            LWIP_ND6_DELAY_FIRST_PROBE_TIME: 5000,
            LWIP_ND6_ALLOW_RA_UPDATES: false,
            LWIP_ND6_TCP_REACHABILITY_HINTS: true,
            LWIP_ND6_RDNSS_MAX_DNS_SERVERS: 0,
            LWIP_IPV6_DHCP: false,
            LWIP_IPV6_DHCP6_STATEFUL: true,
            LWIP_IPV6_DHCP6_STATELESS: false,
            LWIP_DHCP6_GET_NTP_SRV: false,
            DNS_MAX_SERVERS: 2,
            LWIP_DHCP6_MAX_DNS_SERVERS: 2,
            LWIP_DHCP6_MAX_NTP_SERVERS: 1,
            LWIP_DEBUG: false,
            LWIP_PERF: false,
            LWIP_TESTMODE: false,
        }
    }
}
