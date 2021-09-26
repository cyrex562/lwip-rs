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

// #define LWIP_HDR_RAW_H















pub const RAW_FLAGS_CONNECTED: u32 = 0x01;
pub const RAW_FLAGS_HDRINCL: u32 = 0x02;
pub const RAW_FLAGS_MULTICAST_LOOP: u32 = 0x04; 

// RawPcb;

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
type raw_recv_fn = fn(arg: &mut Vec<u8>, pcb: &mut RawPcb, p: &mut PacketBuffer,
 addr: &mut LwipAddr)->u8;

/* the RAW protocol control block */
pub struct RawPcb {
  /* Common members of all PCB types */
  pub ip_pcb: IP_PCB,
  // let mut next: &mut RawPcb;
  pub protocol: u8,
  pub flags: u8,


  /* outgoing network interface for multicast packets, by interface index (if nonzero) */
  pub mcast_ifindex: u8,
  /* TTL for outgoing multicast packets */
  pub mcast_ttl: u8,


  /* receive callback function */
  pub recv: raw_recv_fn,
  /* user-supplied argument for the recv callback */
  pub recv_arg: &mut Vec<u8>,

  /* fields for handling checksum computations as per RFC3542. */
  pub chksum_offset: u16,
  pub chksum_reqd: u8,
}

/* The following functions is the application layer interface to the
   RAW code. */
// RawPcb * raw_new        (proto: u8);
// RawPcb * raw_new_ip_type(type: u8, proto: u8);
// pub fn              raw_remove     (pcb: &mut RawPcb);
// pub fn             raw_bind       (pcb: &mut RawPcb,  ipaddr: &mut LwipAddr);
// pub fn              raw_bind_netif (pcb: &mut RawPcb,  netif: &mut NetIfc);
// pub fn             raw_connect    (pcb: &mut RawPcb,  ipaddr: &mut LwipAddr);
// pub fn              raw_disconnect (pcb: &mut RawPcb);

// pub fn             raw_sendto     (pcb: &mut RawPcb, p: &mut PacketBuffer,  ipaddr: &mut LwipAddr);
// pub fn             raw_sendto_if_src(pcb: &mut RawPcb, p: &mut PacketBuffer,  dst_ip: &mut LwipAddr, netif: &mut NetIfc,  src_ip: &mut LwipAddr);
// pub fn             raw_send       (pcb: &mut RawPcb, p: &mut PacketBuffer);

// pub fn              raw_recv       (pcb: &mut RawPcb, raw_recv_fn recv, recv_arg: &mut Vec<u8>);

// #define          raw_flags(pcb) ((pcb).flags)
// #define          raw_setflags(pcb,f)  ((pcb).flags = (f))

// #define          raw_set_flags(pcb, set_flags)     loop { (pcb).flags = ((pcb).flags |  (set_flags)); } while(0)
// #define          raw_clear_flags(pcb, clr_flags)   loop { (pcb).flags = ((pcb).flags & (!(clr_flags) & 0xff)); } while(0)
// #define          raw_is_flag_set(pcb, flag)        (((pcb).flags & (flag)) != 0)

// #define raw_init() /* Compatibility define, no init needed. */

// /* for compatibility with older implementation */
// #define raw_new_ip6(proto) raw_new_ip_type(IPADDR_TYPE_V6, proto)


// #define raw_set_multicast_netif_index(pcb, idx) ((pcb).mcast_ifindex = (idx))
// #define raw_get_multicast_netif_index(pcb)      ((pcb).mcast_ifindex)
// #define raw_set_multicast_ttl(pcb, value)       ((pcb).mcast_ttl = (value))
// #define raw_get_multicast_ttl(pcb)              ((pcb).mcast_ttl)






