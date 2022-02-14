/**
 * @file
 * MDNS responder
 */

 /*
 * Copyright (c) 2015 Verisure Innovation AB
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
 * Author: Erik Ekman <erik@kryo.se>
 * Author: Jasper Verschueren <jasper.verschueren@apart-audio.com>
 *
 */






/**
 * @defgroup mdns_opts Options
 * @ingroup mdns
 * @{
 */

/**
 * LWIP_MDNS_RESPONDER==1: Turn on multicast DNS module. UDP must be available for MDNS
 * transport. IGMP is needed for IPv4 multicast.
 */

pub const LWIP_MDNS_RESPONDER: u32 = 0;
 /* LWIP_MDNS_RESPONDER */

/** The maximum number of services per netif */

pub const MDNS_MAX_SERVICES: u32 = 1;


/** The minimum delay between probes in ms. RFC 6762 require 250ms.
 * In noisy WiFi environment, adding 30-50ms to this value help a lot for
 * a successful Apple BCT tests.
 */

pub const MDNS_PROBE_DELAY_MS: u32 = 250;


/** The maximum number of received packets stored in chained list of known
 * answers for pending truncated questions. This value define the size of
 * the MDNS_PKTS mempool.
 * Up to MDNS_MAX_STORED_PKTS pbuf can be stored in addition to TC questions
 * that are pending.
 */

pub const MDNS_MAX_STORED_PKTS: u32 = 4;


/** Payload size allocated for each outgoing UDP packet. Will be allocated with
 * PBUF_RAM and freed after packet was sent.
 * According to RFC 6762, there is no reason to retain the 512 bytes restriction
 * for link-local multicast packet.
 * 512 bytes isn't enough when 2 services need to be probed.
 */

#define MDNS_OUTPUT_PACKET_SIZE      ((MDNS_MAX_SERVICES == 1) ? 512 : 1450)


/** MDNS_RESP_USENETIF_EXTCALLBACK==1: register an ext_callback on the netif
 * to automatically restart probing/announcing on status or address change.
 */

pub const MDNS_RESP_USENETIF_EXTCALLBACK: u32 = LWIP_NETIF_EXT_STATUS_CALLBACK;


/**
 * LWIP_MDNS_SEARCH==1: Turn on search over multicast DNS module.
 */

pub const LWIP_MDNS_SEARCH: u32 = 1;


/** The maximum number of running requests */

pub const MDNS_MAX_REQUESTS: u32 = 2;


/**
 * MDNS_DEBUG: Enable debugging for multicast DNS.
 */

pub const MDNS_DEBUG: u32 = LWIP_DBG_OFF;


/**
 * @}
 */

 /* LWIP_HDR_APPS_MDNS_OPTS_H */
