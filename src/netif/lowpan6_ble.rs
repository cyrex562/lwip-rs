/*
 * @file
 * 6LowPAN over BLE output for IPv6 (RFC7668).
*/

/*
 * Copyright (c) 2017 Benjamin Aigner
 * Copyright (c) 2015 Inico Technologies Ltd. , Author: Ivan Delamer <delamer@inicotech.com>
 * 
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
 * Author: Benjamin Aigner <aignerb@technikum-wien.at>
 * 
 * Based on the original 6lowpan implementation of lwIP ( @see 6lowpan.c)
 */


/*
 * @defgroup rfc7668if 6LoWPAN over BLE (RFC7668)
 * @ingroup netifs
 * This file implements a RFC7668 implementation for 6LoWPAN over
 * Bluetooth Low Energy. The specification is very similar to 6LoWPAN,
 * so most of the code is re-used.
 * Compared to 6LoWPAN, much functionality is already implemented in
 * lower BLE layers (fragmenting, session management,...).
 *
 * Usage:
 * - add this netif
 *   - don't add IPv4 addresses (no IPv4 support in RFC7668), pass 'NULL','NULL','NULL'
 *   - use the BLE to EUI64 conversation util to create an IPv6 link-local address from the BLE MAC (@ref ble_addr_to_eui64)
 *   - input function: @ref rfc7668_input
 * - set the link output function, which transmits output data to an established L2CAP channel
 * - If data arrives (HCI event "L2CAP_DATA_PACKET"):
 *   - allocate a @ref PBUF_RAW buffer
 *   - let the pbuf struct poto: i32 the incoming data or copy it to the buffer
 *   - call netif.input
 *
 * @todo:
 * - further testing
 * - support compression contexts
 * - support multiple addresses
 * - support multicast
 * - support neighbor discovery
 */



















/* context memory, containing IPv6 addresses */
static rfc7668_context: ip6_addr_t[LWIP_6LOWPAN_NUM_CONTEXTS];

#define rfc7668_context NULL


static struct lowpan6_link_addr rfc7668_local_addr;
static struct lowpan6_link_addr rfc7668_peer_addr;

/*
 * @ingroup rfc7668if
 *  convert BT address to EUI64 addr
 * 
 * This method converts a Bluetooth MAC address to an EUI64 address,
 * which is used within IPv6 communication
 * 
 * @param dst IPv6 destination space
 * @param src BLE MAC address source
 * @param public_addr If the LWIP_RFC7668_LINUX_WORKAROUND_PUBLIC_ADDRESS
 * option is set, bit 0x02 will be set if param=0 (no public addr); cleared otherwise
 * 
 * @see LWIP_RFC7668_LINUX_WORKAROUND_PUBLIC_ADDRESS
 */
pub fn 
ble_addr_to_eui64(uint8_t *dst,  uint8_t *src, public_addr: i32)
{
  /* according to RFC7668 ch 3.2.2. */
  memcpy(dst, src, 3);
  dst[3] = 0xFF;
  dst[4] = 0xFE;
  memcpy(&dst[5], &src[3], 3);

  if(public_addr) {
    dst[0] &= ~0x02;
  } else {
    dst[0] |= 0x02;
  }

  

}

/*
 * @ingroup rfc7668if
 *  convert EUI64 address to Bluetooth MAC addr
 * 
 * This method converts an EUI64 address to a Bluetooth MAC address,
 * 
 * @param dst BLE MAC address destination
 * @param src IPv6 source
 * 
 */
pub fn 
eui64_to_ble_addr(uint8_t *dst,  uint8_t *src)
{
  /* according to RFC7668 ch 3.2.2. */
  memcpy(dst,src,3);
  memcpy(&dst[3],&src[5],3);
}

/* Set an address used for stateful compression.
 * This expects an address of 6 or 8 bytes.
 */
static err_t
rfc7668_set_addr(addr: &mut lowpan6_link_addr,  in_addr: &mut Vec<u8>, in_addr_len: usize, is_mac_48: i32, is_public_addr: i32)
{
  if ((in_addr == NULL) || (addr == NULL)) {
    return ERR_VAL;
  }
  if (is_mac_48) {
    if (in_addr_len != 6) {
      return ERR_VAL;
    }
    addr.addr_len = 8;
    ble_addr_to_eui64(addr.addr, in_addr, is_public_addr);
  } else {
    if (in_addr_len != 8) {
      return ERR_VAL;
    }
    addr.addr_len = 8;
    memcpy(addr.addr, in_addr, 8);
  }
  return ERR_OK;
}


/* Set the local address used for stateful compression.
 * This expects an address of 8 bytes.
 */
pub fn 
rfc7668_set_local_addr_eui64(netif: &mut netif,  local_addr: &mut Vec<u8>, local_addr_len: usize)
{
  /* netif not used for now, the address is stored globally... */
  
  return rfc7668_set_addr(&rfc7668_local_addr, local_addr, local_addr_len, 0, 0);
}

/* Set the local address used for stateful compression.
 * This expects an address of 6 bytes.
 */
pub fn 
rfc7668_set_local_addr_mac48(netif: &mut netif,  local_addr: &mut Vec<u8>, local_addr_len: usize, is_public_addr: i32)
{
  /* netif not used for now, the address is stored globally... */
  
  return rfc7668_set_addr(&rfc7668_local_addr, local_addr, local_addr_len, 1, is_public_addr);
}

/* Set the peer address used for stateful compression.
 * This expects an address of 8 bytes.
 */
pub fn 
rfc7668_set_peer_addr_eui64(netif: &mut netif,  peer_addr: &mut Vec<u8>, peer_addr_len: usize)
{
  /* netif not used for now, the address is stored globally... */
  
  return rfc7668_set_addr(&rfc7668_peer_addr, peer_addr, peer_addr_len, 0, 0);
}

/* Set the peer address used for stateful compression.
 * This expects an address of 6 bytes.
 */
pub fn 
rfc7668_set_peer_addr_mac48(netif: &mut netif,  peer_addr: &mut Vec<u8>, peer_addr_len: usize, is_public_addr: i32)
{
  /* netif not used for now, the address is stored globally... */
  
  return rfc7668_set_addr(&rfc7668_peer_addr, peer_addr, peer_addr_len, 1, is_public_addr);
}

/* Encapsulate IPv6 frames for BLE transmission
 * 
 * This method implements the IPv6 header compression:
 *  *) According to RFC6282
 *  *) See Figure 2, contains base format of bit positions
 *  *) Fragmentation not necessary (done at L2CAP layer of BLE)
 * @note Currently the pbuf allocation uses 256 bytes. If longer packets are used (possible due to MTU=1480Bytes), increase it here!
 * 
 * @param p Pbuf struct, containing the payload data
 * @param netif Output network interface. Should be of RFC7668 type
 * 
 * @return Same as netif.output.
 */
static err_t
rfc7668_compress(netif: &mut netif, p: &mut pbuf)
{
  p_frag: &mut pbuf;
  remaining_len: u16;
  buffer: &mut Vec<u8>;
  lowpan6_header_len: u8;
  hidden_header_len: u8;
  let err: err_t;

  LWIP_ASSERT("lowpan6_frag: netif.linkoutput not set", netif.linkoutput != NULL);



  /* We'll use a dedicated pbuf for building BLE fragments.
   * We'll over-allocate it by the bytes saved for header compression.
   */
  p_frag = pbuf_alloc(PBUF_RAW, p.tot_len, PBUF_RAM);
  if (p_frag == NULL) {
    MIB2_STATS_NETIF_INC(netif, ifoutdiscards);
    return ERR_MEM;
  }
  LWIP_ASSERT("this needs a pbuf in one piece", p_frag.len == p_frag.tot_len);

  /* Write IP6 header (with IPHC). */
  buffer = (u8*)p_frag.payload;

  err = lowpan6_compress_headers(netif, p.payload, p.len, buffer, p_frag.len,
    &lowpan6_header_len, &hidden_header_len, rfc7668_context, &rfc7668_local_addr, &rfc7668_peer_addr);
  if (err != ERR_OK) {
    MIB2_STATS_NETIF_INC(netif, ifoutdiscards);
    pbuf_free(p_frag);
    return err;
  }
  pbuf_remove_header(p, hidden_header_len);

  /* Calculate remaining packet length */
  remaining_len = p.tot_len;

  /* Copy IPv6 packet */
  pbuf_copy_partial(p, buffer + lowpan6_header_len, remaining_len, 0);

  /* Calculate frame length */
  p_frag.len = p_frag.tot_len = remaining_len + lowpan6_header_len;

  /* send the packet */
  MIB2_STATS_NETIF_ADD(netif, ifoutoctets, p_frag.tot_len);
  LWIP_DEBUGF(LWIP_LOWPAN6_DEBUG|LWIP_DBG_TRACE, ("rfc7668_output: sending packet %p\n", p));
  err = netif.linkoutput(netif, p_frag);

  pbuf_free(p_frag);

  return err;
 /* LWIP_6LOWPAN_IPHC */
  /* 6LoWPAN over BLE requires IPHC! */
  return ERR_IF;

}

/*
 * @ingroup rfc7668if
 * Set context id IPv6 address
 *
 * Store one IPv6 address to a given context id.
 *
 * @param idx Context id
 * @param context IPv6 addr for this context
 *
 * @return ERR_OK (if everything is fine), ERR_ARG (if the context id is out of range), ERR_VAL (if contexts disabled)
 */
pub fn 
rfc7668_set_context(idx: u8,  context: &mut ip6_addr_t)
{

  /* check if the ID is possible */
  if (idx >= LWIP_6LOWPAN_NUM_CONTEXTS) {
    return ERR_ARG;
  }
  /* copy IPv6 address to context storage */
  ip6_addr_set(&rfc7668_context[idx], context);
  return ERR_OK;

  
  
  return ERR_VAL;

}

/*
 * @ingroup rfc7668if
 * Compress outgoing IPv6 packet and pass it on to netif.linkoutput
 *
 * @param netif The lwIP network interface which the IP packet will be sent on.
 * @param q The pbuf(s) containing the IP packet to be sent.
 * @param ip6addr The IP address of the packet destination.
 *
 * @return See rfc7668_compress
 */
pub fn 
rfc7668_output(netif: &mut netif, q: &mut pbuf,  ip6addr: &mut ip6_addr_t)
{
  /* dst ip6addr is not used here, we only have one peer */
  LWIP_UNUSED_ARGip6addr;

  return rfc7668_compress(netif, q);
}

/*
 * @ingroup rfc7668if
 * Process a received raw payload from an L2CAP channel
 *
 * @param p the received packet, p.payload pointing to the
 *        IPv6 header (maybe compressed)
 * @param netif the network interface on which the packet was received
 * 
 * @return ERR_OK if everything was fine
 */
pub fn 
rfc7668_input(struct pbuf * p, netif: &mut netif)
{
  u8 * puc;

  MIB2_STATS_NETIF_ADD(netif, ifinoctets, p.tot_len);

  /* Load first header byte */
  puc = (u8*)p.payload;
  
  /* no IP header compression */
  if (*puc == 0x41) {
    LWIP_DEBUGF(LWIP_LOWPAN6_DECOMPRESSION_DEBUG, ("Completed packet, removing dispatch: 0x%2x \n", *puc));
    /* This is a complete IPv6 packet, just skip header byte. */
    pbuf_remove_header(p, 1);
  /* IPHC header compression */
  } else if ((*puc & 0xe0 )== 0x60) {
    LWIP_DEBUGF(LWIP_LOWPAN6_DECOMPRESSION_DEBUG, ("Completed packet, decompress dispatch: 0x%2x \n", *puc));
    /* IPv6 headers are compressed using IPHC. */
    p = lowpan6_decompress(p, 0, rfc7668_context, &rfc7668_peer_addr, &rfc7668_local_addr);
    /* if no pbuf is returned, handle as discarded packet */
    if (p == NULL) {
      MIB2_STATS_NETIF_INC(netif, ifindiscards);
      return ERR_OK;
    }
  /* invalid header byte, discard */
  } else {
    LWIP_DEBUGF(LWIP_LOWPAN6_DECOMPRESSION_DEBUG, ("Completed packet, discarding: 0x%2x \n", *puc));
    MIB2_STATS_NETIF_INC(netif, ifindiscards);
    pbuf_free(p);
    return ERR_OK;
  }
  /* @todo: distinguish unicast/multicast */
  MIB2_STATS_NETIF_INC(netif, ifinucastpkts);


  {
    i: u16;
    LWIP_DEBUGF(LWIP_RFC7668_IP_UNCOMPRESSED_DEBUG, ("IPv6 payload:\n"));
    for (i = 0; i < p.len; i+= 1) {
      if ((i%4)==0) {
        LWIP_DEBUGF(LWIP_RFC7668_IP_UNCOMPRESSED_DEBUG, ("\n"));
      }
      LWIP_DEBUGF(LWIP_RFC7668_IP_UNCOMPRESSED_DEBUG, ("%2X ", *((uint8_t *)p.payload+i)));
    }
    LWIP_DEBUGF(LWIP_RFC7668_IP_UNCOMPRESSED_DEBUG, ("\np.len: %d\n", p.len));
  }

  /* pass data to ip6_input */
  return ip6_input(p, netif);
}

/*
 * @ingroup rfc7668if
 * Initialize the netif
 * 
 * No flags are used (broadcast not possible, not ethernet, ...)
 * The shortname for this netif is "BT"
 *
 * @param netif the network interface to be initialized as RFC7668 netif
 * 
 * @return ERR_OK if everything went fine
 */
pub fn 
rfc7668_if_init(netif: &mut netif)
{
  netif.name[0] = 'b';
  netif.name[1] = 't';
  /* local function as IPv6 output */
  netif.output_ip6 = rfc7668_output;

  MIB2_INIT_NETIF(netif, snmp_ifType_other, 0);

  /* maximum transfer unit, set according to RFC7668 ch2.4 */
  netif.mtu = 1280;

  /* no flags set (no broadcast, ethernet,...)*/
  netif.flags = 0;

  /* everything fine */
  return ERR_OK;
}


/*
 * Pass a received packet to tcpip_thread for input processing
 *
 * @param p the received packet, p.payload pointing to the
 *          IEEE 802.15.4 header.
 * @param inp the network interface on which the packet was received
 * 
 * @return see @ref tcpip_inpkt, same return values
 */
pub fn 
tcpip_rfc7668_input(p: &mut pbuf, inp: &mut netif)
{
  /* send data to upper layer, return the result */
  return tcpip_inpkt(p, inp, rfc7668_input);
}



