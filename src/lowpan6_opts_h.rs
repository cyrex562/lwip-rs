/**
 * @file
 * 6LowPAN options list
 */

/*
 * Copyright (c) 2015 Inico Technologies Ltd.
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




// #include "lwip/opt.h"

/** LWIP_6LOWPAN_NUM_CONTEXTS: define the number of compression
 * contexts per netif type
 */

pub const LWIP_6LOWPAN_NUM_CONTEXTS: u32 = 10; /** LWIP_6LOWPAN_INFER_SHORT_ADDRESS: set this to 0 to disable creating
 * short addresses for matching addresses (debug only)
 */

pub const LWIP_6LOWPAN_INFER_SHORT_ADDRESS: u32 = 1; /** LWIP_6LOWPAN_IPHC: set this to 0 to disable IP header compression as per
 * RFC 6282 (which is mandatory for BLE)
 */

pub const LWIP_6LOWPAN_IPHC: u32 = 1; /** Set this to 1 if your IEEE 802.15.4 interface can calculate and check the
 * CRC in hardware. This means TX packets get 2 zero bytes added on transmission
 * which are to be filled with the CRC.
 */

pub const LWIP_6LOWPAN_802154_HW_CRC: u32 = 0; /** If LWIP_6LOWPAN_802154_HW_CRC==0, this can override the default slow
 * implementation of the CRC used for 6LoWPAN over IEEE 802.15.4 (which uses
 * a shift register).
 */

#define LWIP_6LOWPAN_CALC_CRC(buf, len)  lowpan6_calc_crc(buf, len)


/** Debug level for 6LoWPAN in general */

#define LWIP_LOWPAN6_DEBUG               LWIP_DBG_OFF


/** Debug level for 6LoWPAN over IEEE 802.15.4 */

#define LWIP_LOWPAN6_802154_DEBUG        LWIP_DBG_OFF


/** LWIP_LOWPAN6_IP_COMPRESSED_DEBUG: enable compressed IP frame
 * output debugging
 */

#define LWIP_LOWPAN6_IP_COMPRESSED_DEBUG      LWIP_DBG_OFF


/** LWIP_LOWPAN6_DECOMPRESSION_DEBUG: enable decompression debug output
 */

#define LWIP_LOWPAN6_DECOMPRESSION_DEBUG      LWIP_DBG_OFF


/** LWIP_RFC7668_IP_UNCOMPRESSED_DEBUG: enable decompressed IP frame
 * output debugging */

#define LWIP_RFC7668_IP_UNCOMPRESSED_DEBUG    LWIP_DBG_OFF


/** LWIP_RFC7668_LINUX_WORKAROUND_PUBLIC_ADDRESS:
 * Currently, the linux kernel driver for 6lowpan sets/clears a bit in
 * the address, depending on the BD address (either public or not).
 * Might not be RFC7668 conform, so you may select to do that (=1) or
 * not (=0) */

pub const LWIP_RFC7668_LINUX_WORKAROUND_PUBLIC_ADDRESS: u32 = 1; /* LWIP_HDR_LOWPAN6_OPTS_H */
