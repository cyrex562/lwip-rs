/*
 * @file
 * UDP API (to be used from TCPIP thread)\n
 * See also @ref udp_raw
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

// #define LWIP_HDR_UDP_H
















pub const UDP_FLAGS_NOCHKSUM: u32 = 0x01;Upub const UDP_FLAGS_NOCHKSUM: u32 = 0x01;pub const UDP_FLAGS_NOCHKSUM: u32 = 0x01;pub const UDP_FLAGS_NOCHKSUM: u32 = 0x01;
#define UDP_FLAGS_UDPLITE        0x02U
#define UDP_FLAGS_CONNECTED      0x04U
#define UDP_FLAGS_MULTICAST_LOOP 0x08U

struct udp_pcb;

/* Function prototype for udp pcb receive callback functions
 * addr and port are in same byte order as in the pcb
 * The callback is responsible for freeing the pbuf
 * if it's not used any more.
 *
 * ATTENTION: Be aware that 'addr' might pointo: i32 the pbuf 'p' so freeing this pbuf
 *            can make 'addr' invalid, too.
 *
 * @param arg user supplied argument (udp_pcb.recv_arg)
 * @param pcb the udp_pcb which received data
 * @param p the packet buffer that was received
 * @param addr the remote IP address from which the packet was received
 * @param port the remote port from which the packet was received
 */
typedef void (*udp_recv_fn)(arg: &mut Vec<u8>, pcb: &mut udp_pcb, p: &mut pbuf,
    const addr: &mut ip_addr_t, port: u16);

/* the UDP protocol control block */
struct udp_pcb {
/* Common members of all PCB types */
  IP_PCB;

/* Protocol specific PCB members */

  next: &mut udp_pcb;

  flags: u8;
  /* ports are in host byte order */
  local_port: u16, remote_port;



  /* outgoing network interface for multicast packets, by IPv4 address (if not 'any') */
  ip4_addr mcast_ip4;

  /* outgoing network interface for multicast packets, by interface index (if nonzero) */
  mcast_ifindex: u8;
  /* TTL for outgoing multicast packets */
  mcast_ttl: u8;



  /* used for UDP_LITE only */
  chksum_len_rx: u16, chksum_len_tx;


  /* receive callback function */
  udp_recv_fn recv;
  /* user-supplied argument for the recv callback */
  void *recv_arg;
};
/* udp_pcbs export for external reference (e.g. SNMP agent) */
extern udp_pcbs: &mut udp_pcb;

/* The following functions is the application layer interface to the
   UDP code. */
struct udp_pcb * udp_new        ();
struct udp_pcb * udp_new_ip_type(type: u8);
pub fn              udp_remove     (pcb: &mut udp_pcb);
pub fn             udp_bind       (pcb: &mut udp_pcb,  ipaddr: &mut ip_addr_t,
                                 port: u16);
pub fn              udp_bind_netif (pcb: &mut udp_pcb,  struct netif* netif);
pub fn             udp_connect    (pcb: &mut udp_pcb,  ipaddr: &mut ip_addr_t,
                                 port: u16);
pub fn              udp_disconnect (pcb: &mut udp_pcb);
pub fn              udp_recv       (pcb: &mut udp_pcb, udp_recv_fn recv,
                                 void *recv_arg);
pub fn             udp_sendto_if  (pcb: &mut udp_pcb, p: &mut pbuf,
                                 const dst_ip: &mut ip_addr_t, dst_port: u16,
                                 netif: &mut netif);
pub fn             udp_sendto_if_src(pcb: &mut udp_pcb, p: &mut pbuf,
                                 const dst_ip: &mut ip_addr_t, dst_port: u16,
                                 netif: &mut netif,  src_ip: &mut ip_addr_t);
pub fn             udp_sendto     (pcb: &mut udp_pcb, p: &mut pbuf,
                                 const dst_ip: &mut ip_addr_t, dst_port: u16);
pub fn             udp_send       (pcb: &mut udp_pcb, p: &mut pbuf);


pub fn             udp_sendto_if_chksum(pcb: &mut udp_pcb, p: &mut pbuf,
                                 const dst_ip: &mut ip_addr_t, dst_port: u16,
                                 netif: &mut netif, have_chksum: u8,
                                 chksum: u16);
pub fn             udp_sendto_chksum(pcb: &mut udp_pcb, p: &mut pbuf,
                                 const dst_ip: &mut ip_addr_t, dst_port: u16,
                                 have_chksum: u8, chksum: u16);
pub fn             udp_send_chksum(pcb: &mut udp_pcb, p: &mut pbuf,
                                 have_chksum: u8, chksum: u16);
pub fn             udp_sendto_if_src_chksum(pcb: &mut udp_pcb, p: &mut pbuf,
                                 const dst_ip: &mut ip_addr_t, dst_port: u16, netif: &mut netif,
                                 have_chksum: u8, chksum: u16,  src_ip: &mut ip_addr_t);


#define          udp_flags(pcb) ((pcb)->flags)
#define          udp_setflags(pcb, f)  ((pcb)->flags = (f))

#define          udp_set_flags(pcb, set_flags)     do { (pcb)->flags = ((pcb)->flags |  (set_flags)); } while(0)
#define          udp_clear_flags(pcb, clr_flags)   do { (pcb)->flags = ((pcb)->flags & (~(clr_flags) & 0xff)); } while(0)
#define          udp_is_flag_set(pcb, flag)        (((pcb)->flags & (flag)) != 0)

/* The following functions are the lower layer interface to UDP. */
pub fn              udp_input      (p: &mut pbuf, inp: &mut netif);

pub fn              udp_init       ();

/* for compatibility with older implementation */
#define udp_new_ip6() udp_new_ip_type(IPADDR_TYPE_V6)



#define udp_set_multicast_netif_addr(pcb, ip4addr) ip4_addr_copy((pcb)->mcast_ip4, *(ip4addr))
#define udp_get_multicast_netif_addr(pcb)          (&(pcb)->mcast_ip4)

#define udp_set_multicast_netif_index(pcb, idx)    ((pcb)->mcast_ifindex = (idx))
#define udp_get_multicast_netif_index(pcb)         ((pcb)->mcast_ifindex)
#define udp_set_multicast_ttl(pcb, value)          ((pcb)->mcast_ttl = (value))
#define udp_get_multicast_ttl(pcb)                 ((pcb)->mcast_ttl)



pub fn  udp_debug_print(udphdr: &mut udp_hdr);
#else
#define udp_debug_print(udphdr)


pub fn  udp_netif_ip_addr_changed(const ip_addr_t* old_addr,  ip_addr_t* new_addr);


}





