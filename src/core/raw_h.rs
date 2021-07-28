/*
 * @file
 * raw API (to be used from TCPIP thread)\n
 * See also @ref raw_raw
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

#define LWIP_HDR_RAW_H












extern "C" {


pub const RAW_FLAGS_CONNECTED: u32 = 0x01;Upub const RAW_FLAGS_CONNECTED: u32 = 0x01;pub const RAW_FLAGS_CONNECTED: u32 = 0x01;
#define RAW_FLAGS_HDRINCL        0x02U
#define RAW_FLAGS_MULTICAST_LOOP 0x04U

struct raw_pcb;

/* Function prototype for raw pcb receive callback functions.
 * @param arg user supplied argument (raw_pcb.recv_arg)
 * @param pcb the raw_pcb which received data
 * @param p the packet buffer that was received
 * @param addr the remote IP address from which the packet was received
 * @return 1 if the packet was 'eaten' (aka. deleted),
 *         0 if the packet lives on
 * If returning 1, the callback is responsible for freeing the pbuf
 * if it's not used any more.
 */
typedef u8 (*raw_recv_fn)(arg: &mut Vec<u8>, pcb: &mut raw_pcb, p: &mut pbuf,
    const addr: &mut ip_addr_t);

/* the RAW protocol control block */
struct raw_pcb {
  /* Common members of all PCB types */
  IP_PCB;

  next: &mut raw_pcb;

  protocol: u8;
  flags: u8;


  /* outgoing network interface for multicast packets, by interface index (if nonzero) */
  mcast_ifindex: u8;
  /* TTL for outgoing multicast packets */
  mcast_ttl: u8;


  /* receive callback function */
  raw_recv_fn recv;
  /* user-supplied argument for the recv callback */
  void *recv_arg;

  /* fields for handling checksum computations as per RFC3542. */
  chksum_offset: u16;
  u8  chksum_reqd;

};

/* The following functions is the application layer interface to the
   RAW code. */
struct raw_pcb * raw_new        (proto: u8);
struct raw_pcb * raw_new_ip_type(type: u8, proto: u8);
pub fn              raw_remove     (pcb: &mut raw_pcb);
pub fn             raw_bind       (pcb: &mut raw_pcb, const ipaddr: &mut ip_addr_t);
pub fn              raw_bind_netif (pcb: &mut raw_pcb, const netif: &mut netif);
pub fn             raw_connect    (pcb: &mut raw_pcb, const ipaddr: &mut ip_addr_t);
pub fn              raw_disconnect (pcb: &mut raw_pcb);

pub fn             raw_sendto     (pcb: &mut raw_pcb, p: &mut pbuf, const ipaddr: &mut ip_addr_t);
pub fn             raw_sendto_if_src(pcb: &mut raw_pcb, p: &mut pbuf, const dst_ip: &mut ip_addr_t, netif: &mut netif, const src_ip: &mut ip_addr_t);
pub fn             raw_send       (pcb: &mut raw_pcb, p: &mut pbuf);

pub fn              raw_recv       (pcb: &mut raw_pcb, raw_recv_fn recv, void *recv_arg);

#define          raw_flags(pcb) ((pcb)->flags)
#define          raw_setflags(pcb,f)  ((pcb)->flags = (f))

#define          raw_set_flags(pcb, set_flags)     do { (pcb)->flags = (u8)((pcb)->flags |  (set_flags)); } while(0)
#define          raw_clear_flags(pcb, clr_flags)   do { (pcb)->flags = (u8)((pcb)->flags & (u8)(~(clr_flags) & 0xff)); } while(0)
#define          raw_is_flag_set(pcb, flag)        (((pcb)->flags & (flag)) != 0)

#define raw_init() /* Compatibility define, no init needed. */

/* for compatibility with older implementation */
#define raw_new_ip6(proto) raw_new_ip_type(IPADDR_TYPE_V6, proto)


#define raw_set_multicast_netif_index(pcb, idx) ((pcb)->mcast_ifindex = (idx))
#define raw_get_multicast_netif_index(pcb)      ((pcb)->mcast_ifindex)
#define raw_set_multicast_ttl(pcb, value)       ((pcb)->mcast_ttl = (value))
#define raw_get_multicast_ttl(pcb)              ((pcb)->mcast_ttl)



}





