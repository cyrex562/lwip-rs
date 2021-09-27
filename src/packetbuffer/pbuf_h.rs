/*
 * @file
 * pbuf API
 */
#![allow(non_snake_case)]

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

// #define LWIP_HDR_PBUF_H

//

/* LWIP_SUPPORT_CUSTOM_PBUF==1: Custom pbufs behave much like their pbuf type
 * but they are allocated by external code (initialised by calling
 * pbuf_alloced_custom()) and when pbuf_free gives up their last reference, they
 * are freed by calling pbuf_custom.custom_free_function().
 * Currently, the pbuf_custom code is only needed for one specific configuration
 * of ip_frag, unless required by external driver/application code. */

// #define LWIP_SUPPORT_CUSTOM_PBUF ((ip_frag && !LWIP_NETIF_TX_SINGLE_PBUF) || (LWIP_IPV6 && LWIP_IPV6_FRAG))

/* @ingroup pbuf
 * PBUF_NEEDS_COPY(p): return a boolean value indicating whether the given
 * pbuf needs to be copied in order to be kept around beyond the current call
 * stack without risking being corrupted. The default setting provides safety:
 * it will make a copy iof any pbuf chain that does not consist entirely of
 * PBUF_ROM type pbufs. For setups with zero-copy support, it may be redefined
 * to evaluate to true in all cases, for example. However, doing so also has an
 * effect on the application side: any buffers that are *not* copied must also
 * *not* be reused by the application after passing them to lwIP. For example,
 * when setting PBUF_NEEDS_COPY to (0), after using udp_send() with a PBUF_RAM
 * pbuf, the application must free the pbuf immediately, rather than reusing it
 * for other purposes. For more background information on this, see tasks #6735
 * and #7896, and bugs #11400 and #49914. */

use crate::core::options::{PBUF_LINK_ENCAPSULATION_HLEN, PBUF_LINK_HLEN};
use crate::packetbuffer::pbuf::pbuf_free_ooseq;

// #define PBUF_NEEDS_COPY(p)  ((p).type_internal & PBUF_TYPE_FLAG_DATA_VOLATILE)
pub fn PBUF_NEEDS_COPY(p: PacketBuffer) -> bool {
    p.type_internal & PBUF_TYPE_FLAG_DATA_VOLATILE != 0
}

/* @todo: We need a mechanism to prevent wasting memory in every pbuf
(TCP vs. UDP, IPv4 vs. IPv6: UDP/IPv4 packets may waste up to 28 bytes) */

//#define PBUF_TRANSPORT_HLEN 20
pub const PBUF_TRANSPORT_HLEN: u32 = 20;

// #define PBUF_IP_HLEN        40
pub const PBUF_IP_HLEN: u32 = 40;
// #else
// #define PBUF_IP_HLEN        20

/*
 * @ingroup pbuf
 * Enumeration of pbuf layers
 */
// pub enum pbuf_layer{
//   /* Includes spare room for transport layer header, e.g. UDP header.
//    * Use this if you intend to pass the pbuf to functions like udp_send().
//    */
//   PBUF_TRANSPORT = pbuf_link_encapsulation_hlen + pbuf_link_hlen + PBUF_IP_HLEN + PBUF_TRANSPORT_HLEN,
//   /* Includes spare room for IP header.
//    * Use this if you intend to pass the pbuf to functions like raw_send().
//    */
//   PBUF_IP = pbuf_link_encapsulation_hlen + pbuf_link_hlen + PBUF_IP_HLEN,
//   /* Includes spare room for link layer header (ethernet header).
//    * Use this if you intend to pass the pbuf to functions like ethernet_output().
//    * @see pbuf_link_hlen
//    */
//   PBUF_LINK = pbuf_link_encapsulation_hlen + pbuf_link_hlen,
//   /* Includes spare room for additional encapsulation header before ethernet
//    * headers (e.g. 802.11).
//    * Use this if you intend to pass the pbuf to functions like netif.linkoutput().
//    * @see pbuf_link_encapsulation_hlen
//    */
//   PBUF_RAW_TX = pbuf_link_encapsulation_hlen,
//   /* Use this for input packets in a netif driver when calling netif.input()
//    * in the most common case - ethernet-layer netif driver. */
//   PBUF_RAW = 0
// }

pub const PBUF_TRANSPORT: usize =
    PBUF_LINK_ENCAPSULATION_HLEN + PBUF_LINK_HLEN + PBUF_IP_HLEN + PBUF_TRANSPORT_HLEN;

pub const PBUF_IP: usize = PBUF_LINK_ENCAPSULATION_HLEN + PBUF_LINK_HLEN + PBUF_IP_HLEN;

pub const PBUF_LINK: usize = PBUF_LINK_ENCAPSULATION_HLEN + PBUF_LINK_HLEN;

pub const PBUF_RAW_TX: usize = PBUF_LINK_ENCAPSULATION_HLEN;

pub const PBUF_RAW: u32 = 0;

/* Base flags for pbuf_type definitions: */

/* Indicates that the payload directly follows the PacketBuffer.
 *  This makes @ref pbuf_header work in both directions. */
pub const PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS: u32 = 0x80;
/* Indicates the data stored in this pbuf can change. If this pbuf needs
 * to be queued, it must be copied/duplicated. */
pub const PBUF_TYPE_FLAG_DATA_VOLATILE: u32 = 0x40;
/* 4 bits are reserved for 16 allocation sources (e.g. heap, pool1, pool2, etc)
 * Internally, we use: 0=heap, 1=MEMP_PBUF, 2=MEMP_PBUF_POOL -> 13 types free*/
pub const PBUF_TYPE_ALLOC_SRC_MASK: u32 = 0x0F;
/* Indicates this pbuf is used for RX (if not set, indicates use for TX).
 * This information can be used to keep some spare RX buffers e.g. for
 * receiving TCP ACKs to unblock a connection) */
pub const PBUF_ALLOC_FLAG_RX: u32 = 0x0100;
/* Indicates the application needs the pbuf payload to be in one piece */
pub const PBUF_ALLOC_FLAG_DATA_CONTIGUOUS: u32 = 0x0200;

pub const PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP: u32 = 0x00;

// #define PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF      0x01
pub const PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF: u32 = 0x01;

// #define PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL 0x02
pub const PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL: u32 = 0x02;

/* First pbuf allocation type for applications */
pub const PBUF_TYPE_ALLOC_SRC_MASK_APP_MIN: u32 = 0x03;
/* Last pbuf allocation type for applications */
// #define PBUF_TYPE_ALLOC_SRC_MASK_APP_MAX            PBUF_TYPE_ALLOC_SRC_MASK
pub const PBUF_TYPE_ALLOC_SRC_MASK_APP_MAX: u32 = PBUF_TYPE_ALLOC_SRC_MASK;

/*
 * @ingroup pbuf
 * Enumeration of pbuf types
 */
// typedef enum {
//   /* pbuf data is stored in RAM, used for TX mostly, PacketBuffer and its payload
//       are allocated in one piece of contiguous memory (so the first payload byte
//       can be calculated from PacketBuffer).
//       pbuf_alloc() allocates PBUF_RAM pbufs as unchained pbufs (although that might
//       change in future versions).
//       This should be used for all OUTGOING packets (TX).*/
//   PBUF_RAM = (PBUF_ALLOC_FLAG_DATA_CONTIGUOUS | PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS | PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP),
//   /* pbuf data is stored in ROM, i.e. PacketBuffer and its payload are located in
//       totally different memory areas. Since it points to ROM, payload does not
//       have to be copied when queued for transmission. */
//   PBUF_ROM = PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF,
//   /* pbuf comes from the pbuf pool. Much like PBUF_ROM but payload might change
//       so it has to be duplicated when queued before transmitting, depending on
//       who has a 'ref' to it. */
//   PBUF_REF = (PBUF_TYPE_FLAG_DATA_VOLATILE | PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF),
//   /* pbuf payload refers to RAM. This one comes from a pool and should be used
//       for RX. Payload can be chained (scatter-gather RX) but like PBUF_RAM, struct
//       pbuf and its payload are allocated in one piece of contiguous memory (so
//       the first payload byte can be calculated from PacketBuffer).
//       Don't use this for TX, if the pool becomes empty e.g. because of TCP queuing,
//       you are unable to receive TCP acks! */
//   PBUF_POOL = (PBUF_ALLOC_FLAG_RX | PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS | PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL)
// } pbuf_type;

pub const PBUF_RAM: u32 = (PBUF_ALLOC_FLAG_DATA_CONTIGUOUS
    | PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS
    | PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP);

pub const PBUF_ROM: u32 = PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF;

pub const PBUF_REF: u32 = (PBUF_TYPE_FLAG_DATA_VOLATILE | PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF);

pub const PBUF_POOL: u32 = (PBUF_ALLOC_FLAG_RX
    | PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS
    | PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL);

/* indicates this packet's data should be immediately passed to the application */
pub const PBUF_FLAG_PUSH: u32 = 0x01;
/* indicates this is a custom pbuf: PacketBuffer_free calls pbuf_custom.custom_free_function()
when the last reference is released (plus custom PBUF_RAM cannot be trimmed) */
pub const PBUF_FLAG_IS_CUSTOM: u32 = 0x02;
/* indicates this pbuf is UDP multicast to be looped back */
pub const PBUF_FLAG_MCASTLOOP: u32 = 0x04;
/* indicates this pbuf was received as link-level broadcast */
pub const PBUF_FLAG_LLBCAST: u32 = 0x08;
/* indicates this pbuf was received as link-level multicast */
pub const PBUF_FLAG_LLMCAST: u32 = 0x10;
/* indicates this pbuf includes a TCP FIN flag */
pub const PBUF_FLAG_TCP_FIN: u32 = 0x20;

/* Main packet buffer struct */
pub struct PacketBuffer {
    /* next pbuf in singly linked pbuf chain */
    // next: &mut PacketBuffer;

    /* pointer to the actual data in the buffer */
    // payload: &mut Vec<u8>;
    pub spayload: Vec<u8>,

    /*
     * total length of this buffer and all next buffers in chain
     * belonging to the same packet.
     *
     * For non-queue packet chains this is the invariant:
     * p.tot_len == p.len + (p.next? p.next.tot_len: 0)
     */
    pub tot_len: usize,

    /* length of this buffer */
    pub len: usize,

    /* a bit field indicating pbuf type and allocation sources
      (see PBUF_TYPE_FLAG_*, PBUF_ALLOC_FLAG_* and PBUF_TYPE_ALLOC_SRC_MASK)
    */
    pub type_internal: u8,

    /* misc flags */
    pub flags: u8,

    /*
     * the reference count always equals the number of pointers
     * that refer to this pbuf. This can be pointers from an application,
     * the stack itself, or pbuf.next pointers from a chain.
     */
    pub pbuf_ref: LWIP_PBUF_REF_T,

    /* For incoming packets, this contains the input netif's index */
    pub if_idx: usize,
}

/* Helper struct for const-correctness only.
 * The only meaning of this one is to provide a const payload pointer
 * for PBUF_ROM type.
 */
pub struct pbuf_rom {
    /* next pbuf in singly linked pbuf chain */
    // next: &mut PacketBuffer;

    /* pointer to the actual data in the buffer */
    payload: Vec<u8>,
}

/* Prototype for a function to free a custom pbuf */
// typedef void (*pbuf_free_custom_fn)(p: &mut PacketBuffer);
type pbuf_free_custom_fn = fn(p: &mut PacketBuffer);

/* A custom pbuf: like a pbuf, but following a function pointer to free it. */
pub struct pbuf_custom {
    /* The actual pbuf */
    pub pbuf: PacketBuffer,
    /* This function is called when pbuf_free deallocates this pbuf(_custom) */
    pub custom_free_function: PacketBuffer_free_custom_fn,
}

/* Define this to 0 to prevent freeing ooseq pbufs when the PBUF_POOL is empty */

// #define PBUF_POOL_FREE_OOSEQ 1
pub const PBUF_POOL_FREE_OOSEQ: u32 = 1;

// extern volatile pbuf_free_ooseq_pending: u8;
// pub fn  pbuf_free_ooseq();
/* When not using sys_check_timeouts(), call PBUF_CHECK_FREE_OOSEQ()
at regular intervals from main level to check if ooseq pbufs need to be
freed! */
// #define PBUF_CHECK_FREE_OOSEQ() loop { if(pbuf_free_ooseq_pending) { \
//   /* pbuf_alloc() reported PBUF_POOL to be empty -> try to free some \
//      ooseq queued pbufs now */ \
//   pbuf_free_ooseq(); }}while(0)
// // #else /* LWIP_TCP && TCP_QUEUE_OOSEQ && NO_SYS && PBUF_POOL_FREE_OOSEQ */
//   /* Otherwise declare an empty PBUF_CHECK_FREE_OOSEQ */
//   #define PBUF_CHECK_FREE_OOSEQ()
pub fn PBUF_CHECK_FREE_OOSEQ() {
    if pbuf_free_ooseq_pending {
        pbuf_free_ooseq();
    }
}

/* Initializes the pbuf module. This call is empty for now, but may not be in future. */
// #define pbuf_init()
pub fn pbuf_init() {
    unimplemented!()
}

// pbuf_alloc: &mut PacketBuffer(l: PacketBuffer_layer, length: u16, pbuf_type type);
// pbuf_alloc_reference: &mut PacketBuffer(payload: &mut Vec<u8>, length: u16, pbuf_type type);

// pbuf_alloced_custom: &mut PacketBuffer(l: PacketBuffer_layer, length: u16, pbuf_type type,
//                                  p: &mut PacketBuffer_custom, payload: &mut Vec<u8>_mem,
//                                  payload_mem_len: u16);

// pub fn  pbuf_realloc(p: &mut PacketBuffer, size: u16);
// #define pbuf_get_allocsrc(p)          ((p).type_internal & PBUF_TYPE_ALLOC_SRC_MASK)
pub fn pbuf_get_allocsrc(p: &mut PacketBuffer) -> u32 {
    (p.type_internal & PBUF_TYPE_ALLOC_SRC_MASK) as u32
}

// #define pbuf_match_allocsrc(p, type)  (pbuf_get_allocsrc(p) == ((type) & PBUF_TYPE_ALLOC_SRC_MASK))
pub fn pbuf_match_allocsrc(p: &mut PacketBuffer, ptype: PacketBuffer_type) -> bool {
    pbuf_get_allocsrc(p) == (ptype & PBUF_TYPE_ALLOC_SRC_MASK)
}

// #define pbuf_match_type(p, type)      pbuf_match_allocsrc(p, type)
pub fn pbuf_match_type(p: &mut PacketBuffer, ptype: PacketBuffer_type) -> bool {
    pbuf_match_allocsrc(p, ptype)
}

// pbuf_header: u8(p: &mut PacketBuffer, header_size: i16);
// pbuf_header_force: u8(p: &mut PacketBuffer, header_size: i16);
// pbuf_add_header: u8(p: &mut PacketBuffer, header_size_increment: usize);
// pbuf_add_header_force: u8(p: &mut PacketBuffer, header_size_increment: usize);
// pbuf_remove_header: u8(p: &mut PacketBuffer, header_size: usize);
// pbuf_free_header: &mut PacketBuffer(q: &mut PacketBuffer, size: u16);
// pub fn  pbuf_ref(p: &mut PacketBuffer);
// pbuf_free: u8(p: &mut PacketBuffer);
// pbuf_clen: u16( p: &mut PacketBuffer);
// pub fn  pbuf_cat(head: &mut PacketBuffer, tail: &mut PacketBuffer);
// pub fn  pbuf_chain(head: &mut PacketBuffer, tail: &mut PacketBuffer);
// pbuf_dechain: &mut PacketBuffer(p: &mut PacketBuffer);
// pub fn  pbuf_copy(p_to: &mut PacketBuffer,  p_from: &mut PacketBuffer);
// pbuf_copy_partial: u16( p: &mut PacketBuffer, dataptr: &mut Vec<u8>, len: u16, offset: u16);
// pub fn  *pbuf_get_contiguous( p: &mut PacketBuffer, buffer: &mut Vec<u8>, bufsize: usize, len: u16, offset: u16);
// pub fn  pbuf_take(buf: &mut PacketBuffer, dataptr: &Vec<u8>, len: u16);
// pub fn  pbuf_take_at(buf: &mut PacketBuffer, dataptr: &Vec<u8>, len: u16, offset: u16);
// pbuf_skip: &mut PacketBuffer(in: &mut PacketBuffer, in_offset: u16, u16* out_offset);
// pbuf_coalesce: &mut PacketBuffer(p: &mut PacketBuffer, layer: PacketBuffer_layer);
// pbuf_clone: &mut PacketBuffer(l: PacketBuffer_layer, pbuf_type type, p: &mut PacketBuffer);
// pub fn  pbuf_fill_chksum(p: &mut PacketBuffer, start_offset: u16, dataptr: &Vec<u8>,
//                        len: u16, chksum: &mut u16);
// pub fn  pbuf_split_64k(p: &mut PacketBuffer, rest: &mut Vec<PacketBuffer>);
// pbuf_get_at: u8( p: &mut PacketBuffer, offset: u16);
// pbuf_try_get_at: i32( p: &mut PacketBuffer, offset: u16);
// pub fn  pbuf_put_at(p: &mut PacketBuffer, offset: u16, data: u8);
// pbuf_memcmp: u16( p: &mut PacketBuffer, offset: u16,  s2: &mut Vec<u8>, n: u16);
// pbuf_memfind: u16( p: &mut PacketBuffer,  mem: &mut Vec<u8>, mem_len: u16, start_offset: u16);
// pbuf_strstr: u16( p: &mut PacketBuffer,  substr: &mut String);
// }
