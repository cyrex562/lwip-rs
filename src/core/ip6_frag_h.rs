/*
 * @file
 *
 * IPv6 fragmentation and reassembly.
 */

/*
 * Copyright (c) 2010 Inico Technologies Ltd.
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
 * Author: Ivan Delamer <delamer@inicotech.com>
 *
 *
 * Please coordinate changes and requests with Ivan Delamer
 * <delamer@inicotech.com>
 */

#define LWIP_HDR_IP6_FRAG_H








extern "C" {





/* The IPv6 reassembly timer interval in milliseconds. */
#define IP6_REASS_TMR_INTERVAL 1000

/* IP6_FRAG_COPYHEADER==1: for platforms where sizeof(void*) > 4, "struct
 * ip6_reass_helper" is too large to be stored in the IPv6 fragment header, and
 * will bleed into the header before it, which may be the IPv6 header or an
 * extension header. This means that for each first fragment packet, we need to
 * 1) make a copy of some IPv6 header fields (src+dest) that we need later on,
 * just in case we do overwrite part of the IPv6 header, and 2) make a copy of
 * the header data that we overwrote, so that we can restore it before either
 * completing reassembly or sending an ICMPv6 reply. The last part is true even
 * if this setting is disabled, but if it is enabled, we need to save a bit
 * more data (up to the size of a pointer) because we overwrite more. */

pub const IPV6_FRAG_COPYHEADER: u32 = 0;


/* With IPV6_FRAG_COPYHEADER==1, a helper structure may (or, depending on the
 * presence of extensions, may not) overwrite part of the IP header. Therefore,
 * we copy the fields that we need from the IP header for as long as the helper
 * structure may still be in place. This is easier than temporarily restoring
 * those fields in the IP header each time we need to perform checks on them. */

#define IPV6_FRAG_SRC(ipr) ((ipr)->src)
#define IPV6_FRAG_DEST(ipr) ((ipr)->dest)
#else /* IPV6_FRAG_COPYHEADER */
#define IPV6_FRAG_SRC(ipr) ((ipr)->iphdr.src)
#define IPV6_FRAG_DEST(ipr) ((ipr)->iphdr.dest)


/* IPv6 reassembly helper struct.
 * This is exported because memp needs to know the size.
 */
struct ip6_reassdata {
  next: &mut ip6_reassdata;
  p: &mut pbuf;
  iphdr: &mut ip6_hdr; /* pointer to the first (original) IPv6 header */

  ip6_addr_p_t src; /* copy of the source address in the IP header */
  ip6_addr_p_t dest; /* copy of the destination address in the IP header */
  /* This buffer (for the part of the original header that we overwrite) will
   * be slightly oversized, but we cannot compute the exact size from here. */
  orig_hdr: u8[sizeof(struct ip6_frag_hdr) + sizeof(void*)];
#else /* IPV6_FRAG_COPYHEADER */
  /* In this case we still need the buffer, for sending ICMPv6 replies. */
  orig_hdr: u8[sizeof(struct ip6_frag_hdr)];

  identification: u32;
  datagram_len: u16;
  nexth: u8;
  timer: u8;

  src_zone: u8; /* zone of original packet's source address */
  dest_zone: u8; /* zone of original packet's destination address */

};

#define ip6_reass_init() /* Compatibility define */
pub fn  ip6_reass_tmr(void);
ip6_reass: &mut pbuf(p: &mut pbuf);






#define LWIP_PBUF_CUSTOM_REF_DEFINED
/* A custom pbuf that holds a reference to another pbuf, which is freed
 * when this custom pbuf is freed. This is used to create a custom PBUF_REF
 * that points into the original pbuf. */
struct pbuf_custom_ref {
  /* 'base class' */
  struct pbuf_custom pc;
  /* pointer to the original pbuf that is referenced */
  original: &mut pbuf;
};


pub fn  ip6_frag(p: &mut pbuf, netif: &mut netif, const ip6_addr_t *dest);





}



