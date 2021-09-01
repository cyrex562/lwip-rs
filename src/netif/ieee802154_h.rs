/*
 * @file
 * Definitions for IEEE 802.15.4 MAC frames
 */

/*
 * Copyright (c) 2018 Simon Goldschmidt.
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 */

// #define LWIP_HDR_NETIF_IEEE802154_H











/* General MAC frame format
 * This shows the full featured header, mainly for documentation.
 * Some fields are omitted or shortened to achieve frame compression.
 */
struct ieee_802154_hdr {
  /* See IEEE_802154_FC_* defines */
  frame_control: u16,
  /* Sequence number is omitted if IEEE_802154_FC_SEQNO_SUPPR is set in frame_control */
  (u8  sequence_number);
  /* Destination PAN ID is omitted if Destination Addressing Mode is 0 */
  destination_pan_id: u16,
  /* Destination Address is omitted if Destination Addressing Mode is 0 */
  (destination_address: [u8;8]);
  /* Source PAN ID is omitted if Source Addressing Mode is 0
      or if IEEE_802154_FC_PANID_COMPR is set in frame control*/
  source_pan_id: u16,
  /* Source Address is omitted if Source Addressing Mode is 0 */
  (source_address: [u8;8]);
  /* The rest is variable */
} ;





/* Addressing modes (2 bits) */
pub const IEEE_802154_ADDR_MODE_NO_ADDR: u32 = 0x00; /* PAN ID and address fields are not present */pub const IEEE_802154_ADDR_MODE_NO_ADDR: u32 = 0x00;pub const IEEE_802154_ADDR_MODE_NO_ADDR: u32 = 0x00;pub const IEEE_802154_ADDR_MODE_NO_ADDR: u32 = 0x00;
pub const IEEE_802154_ADDR_MODE_RESERVED: u32 = 0; x01 /* Reserved */pub const IEEE_802154_ADDR_MODE_RESERVED: u32 = 0; pub const IEEE_802154_ADDR_MODE_RESERVED: u32 = 0; 
pub const IEEE_802154_ADDR_MODE_SHORT: u32 = 0x02; /* Address field contains a short address (16 bit) */
pub const IEEE_802154_ADDR_MODE_EXT: u32 = 0x03; /* Address field contains an extended address (64 bit) */

/* IEEE 802.15.4 Frame Control definitions (2 bytes; see IEEE 802.15.4-2015 ch. 7.2.1) */
pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007; /* bits 0..2: Frame Type */pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;pub const IEEE_802154_FC_FT_MASK: u32 = 0x0007;
pub const IEEE_802154_FC_FT_BEACON: u32 = 0; x00pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; pub const IEEE_802154_FC_FT_BEACON: u32 = 0; 
pub const IEEE_802154_FC_FT_DATA: u32 = 0x01;
pub const IEEE_802154_FC_FT_ACK: u32 = 0x02;
pub const IEEE_802154_FC_FT_MAC_CMD: u32 = 0x03;
pub const IEEE_802154_FC_FT_RESERVED: u32 = 0x04;
pub const IEEE_802154_FC_FT_MULTIPURPOSE: u32 = 0x05;
pub const IEEE_802154_FC_FT_FRAG: u32 = 0x06;
pub const IEEE_802154_FC_FT_EXT: u32 = 0x07;
pub const IEEE_802154_FC_SEC_EN: u32 = 0x0008; /* bit 3: Security Enabled */
pub const IEEE_802154_FC_FRAME_PEND: u32 = 0x0010; /* bit 4: Frame Pending */
pub const IEEE_802154_FC_ACK_REQ: u32 = 0x0020; /* bit 5: AR (ACK required) */
pub const IEEE_802154_FC_PANID_COMPR: u32 = 0x0040; /* bit 6: PAN ID Compression (src and dst are equal, src PAN ID omitted) */
pub const IEEE_802154_FC_RESERVED: u32 = 0x0080;
pub const IEEE_802154_FC_SEQNO_SUPPR: u32 = 0x0100; /* bit 8: Sequence Number Suppression */
pub const IEEE_802154_FC_IE_PRESENT: u32 = 0x0200; /* bit 9: IE Present */
pub const IEEE_802154_FC_DST_ADDR_MODE_MASK: u32 = 0x0c00; /* bits 10..11: Destination Addressing Mode */
#define IEEE_802154_FC_DST_ADDR_MODE_NO_ADDR   (IEEE_802154_ADDR_MODE_NO_ADDR << 10)
#define IEEE_802154_FC_DST_ADDR_MODE_SHORT     (IEEE_802154_ADDR_MODE_SHORT << 10)
#define IEEE_802154_FC_DST_ADDR_MODE_EXT       (IEEE_802154_ADDR_MODE_EXT << 10)
pub const IEEE_802154_FC_FRAME_VERSION_MASK: u32 = 0x3000; /* bits 12..13: Frame Version */
#define IEEE_802154_FC_FRAME_VERSION_GET(x)    (((x) & IEEE_802154_FC_FRAME_VERSION_MASK) >> 12)
pub const IEEE_802154_FC_SRC_ADDR_MODE_MASK: u32 = 0xc000; /* bits 14..15: Source Addressing Mode */
#define IEEE_802154_FC_SRC_ADDR_MODE_SHORT     (IEEE_802154_ADDR_MODE_SHORT << 14)
#define IEEE_802154_FC_SRC_ADDR_MODE_EXT       (IEEE_802154_ADDR_MODE_EXT << 14)


}



