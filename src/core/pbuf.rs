/*
 * @file
 * Packet buffer management
 */

/*
 * @defgroup pbuf Packet buffers (PBUF)
 * @ingroup infrastructure
 *
 * Packets are built from the pbuf data structure. It supports dynamic
 * memory allocation for packet contents or can reference externally
 * managed packet contents both in RAM and ROM. Quick allocation for
 * incoming packets is provided through pools with fixed sized pbufs.
 *
 * A packet may span over multiple pbufs, chained as a singly linked
 * list. This is called a "pbuf chain".
 *
 * Multiple packets may be queued, also using this singly linked list.
 * This is called a "packet queue".
 *
 * So, a packet queue consists of one or more pbuf chains, each of
 * which consist of one or more pbufs. CURRENTLY, PACKET QUEUES ARE
 * NOT SUPPORTED!!! Use helper structs to queue multiple packets.
 *
 * The differences between a pbuf chain and a packet queue are very
 * precise but subtle.
 *
 * The last pbuf of a packet has a .tot_len field that equals the
 * .len field. It can be found by traversing the list. If the last
 * pbuf of a packet has a .next field other than NULL, more packets
 * are on the queue.
 *
 * Therefore, looping through a pbuf of a single packet, has an
 * loop end condition (tot_len == p.len), NOT (next == NULL).
 *
 * Example of custom pbuf usage: @ref zerocopyrx
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

use std::mem::size_of;

use crate::core::pbuf_h::PacketBuffer;

// # define SIZEOF_STRUCT_PBUF        LWIP_MEM_ALIGN_SIZE(sizeof(use crate::core::arch_h::LWIP_MEM_ALIGN_SIZE;
// use crate::core::def_h::NULL;
// use crate::core::pbuf_h::{pbuf_custom, PBUF_FLAG_IS_CUSTOM, PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP};
//
// PacketBuffer))
pub fn SIZEOF_STRUCT_PBUF() -> usize {
    pbuf.size_of()
}
/* Since the pool is created in memp, PBUF_POOL_BUFSIZE will be automatically
aligned there. Therefore, PBUF_POOL_BUFSIZE_ALIGNED can be used here. */
pub const PBUF_POOL_BUFSIZE_ALIGNED: usize = LWIP_MEM_ALIGN_SIZE(PBUF_POOL_BUFSIZE);

// static const PacketBuffer * pbuf_skip_const( const in: &mut pbuf, in_offset: u16, out_offset: &mut u16);

// # define PBUF_POOL_IS_EMPTY() // # else /* !LWIP_TCP || !TCP_QUEUE_OOSEQ || !PBUF_POOL_FREE_OOSEQ */

pub fn PBUF_POOL_FREE_OOSEQ_QUEUE_CALL() {
    if (tcpip_try_callback(pbuf_free_ooseq_callback, None) != ERR_OK) {
        SYS_ARCH_PROTECT(old_level);
        pbuf_free_ooseq_pending = 0;
        SYS_ARCH_UNPROTECT(old_level);
    }
}

// volatile pbuf_free_ooseq_pending: u8; # define PBUF_POOL_IS_EMPTY() pbuf_pool_is_empty()

/*
 * Attempt to reclaim some memory from queued out-of-sequence TCP segments
 * if we run out of pool pbufs. It's better to give priority to new packets
 * if we're running out.
 *
 * This must be done in the correct thread context therefore this function
 * can only be used with NO_SYS=0 and through tcpip_callback.
 */

pub fn pbuf_free_ooseq() {
    let pcb: &mut tcp_pcb;
    SYS_ARCH_SET(pbuf_free_ooseq_pending, 0);

    // TODO:
    // for (pcb = tcp_active_pcbs; NULL != pcb; pcb = pcb.next) {
    //   if (pcb.ooseq != NULL) {
    //     /* Free the ooseq pbufs of one PCB only */
    //     LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_free_ooseq: freeing out-of-sequence pbufs\n"));
    //     tcp_free_ooseq(pcb);
    //     return;
    //   }
    // }
}

/*
 * Just a callback function for tcpip_callback() that calls pbuf_free_ooseq().
 */
pub fn pbuf_free_ooseq_callback(arg: &mut Vec<u8>) {
    //
    pbuf_free_ooseq();
}

/* Queue a call to pbuf_free_ooseq if not already queued. */
pub fn pbuf_pool_is_empty() {
    SYS_ARCH_SET(pbuf_free_ooseq_pending, 1);
    // #else /* PBUF_POOL_FREE_OOSEQ_QUEUE_CALL */
    let queued: u8;
    SYS_ARCH_DECL_PROTECT(old_level);
    SYS_ARCH_PROTECT(old_level);
    queued = pbuf_free_ooseq_pending;
    pbuf_free_ooseq_pending = 1;
    SYS_ARCH_UNPROTECT(old_level);

    if (!queued) {
        /* queue a call to pbuf_free_ooseq if not already queued */
        PBUF_POOL_FREE_OOSEQ_QUEUE_CALL();
    }
}

/* Initialize members of PacketBuffer after allocation */
pub fn pbuf_init_alloced_pbuf(
    p: &mut pbuf,
    payload: &mut Vec<u8>,
    tot_len: u16,
    len: usize,
    ptype: pbuf_type,
    flags: u8,
) {
    // p.next = NULL;
    p.payload = payload;
    p.tot_len = tot_len;
    p.len = len;
    p.type_internal = ptype;
    p.flags = flags;
    p.pbuf_ref = 1;
    p.if_idx = NETIF_NO_INDEX;
}

/*
 * @ingroup pbuf
 * Allocates a pbuf of the given type (possibly a chain for PBUF_POOL type).
 *
 * The actual memory allocated for the pbuf is determined by the
 * layer at which the pbuf is allocated and the requested size
 * (from the size parameter).
 *
 * @param layer header size
 * @param length size of the pbuf's payload
 * @param type this parameter decides how and where the pbuf
 * should be allocated as follows:
 *
 * - PBUF_RAM: buffer memory for pbuf is allocated as one large
 *             chunk. This includes protocol headers as well.
 * - PBUF_ROM: no buffer memory is allocated for the pbuf, even for
 *             protocol headers. Additional headers must be prepended
 *             by allocating another pbuf and chain in to the front of
 *             the ROM pbuf. It is assumed that the memory used is really
 *             similar to ROM in that it is immutable and will not be
 *             changed. Memory which is dynamic should generally not
 *             be attached to PBUF_ROM pbufs. Use PBUF_REF instead.
 * - PBUF_REF: no buffer memory is allocated for the pbuf, even for
 *             protocol headers. It is assumed that the pbuf is only
 *             being used in a single thread. If the pbuf gets queued,
 *             then pbuf_take should be called to copy the buffer.
 * - PBUF_POOL: the pbuf is allocated as a pbuf chain, with pbufs from
 *              the pbuf pool that is allocated during pbuf_init().
 *
 * @return the allocated pbuf. If multiple pbufs where allocated, this
 * is the first pbuf of a pbuf chain.
 */
pub fn pbuf_alloc(layer: pbuf_layer, length: u16, ptype: pbuf_type) -> pbuf {
    let mut p: &mut pbuf;
    let mut offset: u16 = layer;
    // LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_alloc(length=%"U16_F")\n", length));

    match ptype {
        PBUF_REF | PBUF_ROM => {
            p = pbuf_alloc_reference(None, length, ptype);
        }
        PBUF_POOL => {
            let q: &mut pbuf;
            let last: &mut PacketBuffer;
            let rem_len: u16; /* remaining length */
            p = None;
            last = None;
            rem_len = length;
            loop {
                let qlen: u16;
                q = memp_malloc(MEMP_PBUF_POOL);
                if (q == None) {
                    PBUF_POOL_IS_EMPTY();
                    /* free chain so far allocated */
                    if (p) {
                        pbuf_free(p);
                    }
                    /* bail out unsuccessfully */
                    return None;
                }
                qlen = LWIP_MIN(
                    rem_len,
                    (PBUF_POOL_BUFSIZE_ALIGNED - LWIP_MEM_ALIGN_SIZE(offset)),
                );
                pbuf_init_alloced_pbuf(
                    q,
                    LWIP_MEM_ALIGN((q + SIZEOF_STRUCT_PBUF + offset)),
                    rem_len,
                    qlen,
                    ptype,
                    0,
                );
                // LWIP_ASSERT("pbuf_alloc: pbuf q.payload properly aligned",
                //             (q.payload % MEM_ALIGNMENT) == 0);
                // LWIP_ASSERT("PBUF_POOL_BUFSIZE must be bigger than MEM_ALIGNMENT",
                //             (PBUF_POOL_BUFSIZE_ALIGNED - LWIP_MEM_ALIGN_SIZE(offset)) > 0 );
                if (p == None) {
                    /* allocated head of pbuf chain (into p) */
                    p = q;
                } else {
                    /* make previous pbuf poto: i32 this pbuf */
                    last.next = q;
                }
                last = q;
                rem_len = (rem_len - qlen);
                offset = 0;
                if !(rem_len > 0) {
                    break;
                }
            }
        }
        PBUF_RAM => {
            let payload_len: u16 = (LWIP_MEM_ALIGN_SIZE(offset) + LWIP_MEM_ALIGN_SIZE(length));
            let alloc_len = (LWIP_MEM_ALIGN_SIZE(SIZEOF_STRUCT_PBUF) + payload_len);

            /* bug #50040: Check for integer overflow when calculating alloc_len */
            if (payload_len < LWIP_MEM_ALIGN_SIZE(length))
                || (alloc_len < LWIP_MEM_ALIGN_SIZE(length))
            {
                return None;
            }

            /* If pbuf is to be allocated in RAM, allocate memory for it. */
            p = mem_malloc(alloc_len);
            if (p == None) {
                return None;
            }
            pbuf_init_alloced_pbuf(
                p,
                LWIP_MEM_ALIGN((p + SIZEOF_STRUCT_PBUF + offset)),
                length,
                length,
                ptype,
                0,
            );
            LWIP_ASSERT(
                "pbuf_alloc: pbuf.payload properly aligned",
                (p.payload % MEM_ALIGNMENT) == 0,
            );
        }
        _ => {
            LWIP_ASSERT("pbuf_alloc: erroneous type", 0);
            return None;
        }
    }
    // LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_alloc(length=%"U16_F") == %p\n", length, p));
    return p;
}

/*
 * @ingroup pbuf
 * Allocates a pbuf for referenced data.
 * Referenced data can be volatile (PBUF_REF) or long-lived (PBUF_ROM).
 *
 * The actual memory allocated for the pbuf is determined by the
 * layer at which the pbuf is allocated and the requested size
 * (from the size parameter).
 *
 * @param payload referenced payload
 * @param length size of the pbuf's payload
 * @param type this parameter decides how and where the pbuf
 * should be allocated as follows:
 *
 * - PBUF_ROM: It is assumed that the memory used is really
 *             similar to ROM in that it is immutable and will not be
 *             changed. Memory which is dynamic should generally not
 *             be attached to PBUF_ROM pbufs. Use PBUF_REF instead.
 * - PBUF_REF: It is assumed that the pbuf is only
 *             being used in a single thread. If the pbuf gets queued,
 *             then pbuf_take should be called to copy the buffer.
 *
 * @return the allocated pbuf.
 */
pub fn pbuf_alloc_reference(payload: &mut Vec<u8>, length: u16, ptype: pbuf_type) -> pbuf {
    let p: &mut pbuf;
    LWIP_ASSERT(
        "invalid pbuf_type",
        (ptype == PBUF_REF) || (ptype == PBUF_ROM),
    ); /* only allocate memory for the pbuf structure */
    p = memp_malloc(MEMP_PBUF);
    if p == None {
        // LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
        // ("pbuf_alloc_reference: Could not allocate MEMP_PBUF for PBUF_%s.\n",
        // (type == PBUF_ROM) ? "ROM": "REF"));
        return None;
    }
    pbuf_init_alloced_pbuf(p, payload, length, length, ptype, 0);
    return p;
}

/*
 * @ingroup pbuf
 * Initialize a custom pbuf (already allocated).
 * Example of custom pbuf usage: @ref zerocopyrx
 *
 * @param l header size
 * @param length size of the pbuf's payload
 * @param type type of the pbuf (only used to treat the pbuf accordingly, as
 *        this function allocates no memory)
 * @param p pointer to the custom pbuf to initialize (already allocated)
 * @param payload_mem pointer to the buffer that is used for payload and headers,
 *        must be at least big enough to hold 'length' plus the header size,
 *        may be NULL if set later.
 *        ATTENTION: The caller is responsible for correct alignment of this buffer!!
 * @param payload_mem_len the size of the 'payload_mem' buffer, must be at least
 *        big enough to hold 'length' plus the header size
 */
pub fn pbuf_alloced_custom(
    l: pbuf_layer,
    length: u16,
    ptype: pbuf_type,
    p: &mut pbuf_custom,
    payload: &mut Vec<u8>,
    payload_mem_len: usize,
) -> Optional<pbuf> {
    let offset: u16 = l;
    let payload: Vec<u8>;
    // LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_alloced_custom(length=%"U16_F")\n", length));

    if LWIP_MEM_ALIGN_SIZE(offset as usize) + length > payload_mem_len as usize {
        // LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_LEVEL_WARNING, ("pbuf_alloced_custom(length=%"U16_F") buffer too short\n", length));
        return None;
    }

    if payload_mem != None {
        // payload = payload_mem + LWIP_MEM_ALIGN_SIZE(offset as usize);
        payload.reserve(payload_mem + LWIP_MEM_ALIGN_SIZE(offset as usize));
    } else {
        payload.clear();
    }
    pbuf_init_alloced_pbuf(
        &mut p.pbuf,
        &mut payload,
        length,
        length,
        ptype,
        PBUF_FLAG_IS_CUSTOM as u8,
    );
    return &p.pbuf;
}

/*
 * @ingroup pbuf
 * Shrink a pbuf chain to a desired length.
 *
 * @param p pbuf to shrink.
 * @param new_len desired new length of pbuf chain
 *
 * Depending on the desired length, the first few pbufs in a chain might
 * be skipped and left unchanged. The new last pbuf in the chain will be
 * resized, and any remaining pbufs will be freed.
 *
 * @note If the pbuf is ROM/REF, only the .tot_len and .len fields are adjusted.
 * @note May not be called on a packet queue.
 *
 * @note Despite its name, pbuf_realloc cannot grow the size of a pbuf (chain).
 */
pub fn pbuf_realloc(p: &mut pbuf, new_len: u16) {
    let q: &mut pbuf;
    let rem_len: u16; /* remaining length */
    let rem_len: u16;
    let shrink: u16;

    LWIP_ASSERT("pbuf_realloc: p != NULL", p != None);

    /* desired length larger than current length? */
    if new_len >= p.tot_len {
        /* enlarging not yet supported */
        return;
    }

    /* the pbuf chain grows by (new_len - p.tot_len) bytes
     * (which may be negative in case of shrinking) */
    shrink = (p.tot_len - new_len);

    /* first, step over any pbufs that should remain in the chain */
    rem_len = new_len;
    q = p;
    /* should this pbuf be kept? */
    while rem_len > q.len {
        /* decrease remaining length by pbuf length */
        rem_len = (rem_len - q.len);
        /* decrease total length indicator */
        q.tot_len = (q.tot_len - shrink);
        /* proceed to next pbuf in chain */
        q = q.next;
        LWIP_ASSERT("pbuf_realloc: q != NULL", q != None);
    }
    /* we have now reached the new last pbuf (in q) */
    /* rem_len == desired length for pbuf q */

    /* shrink allocated memory for PBUF_RAM */
    /* (other types merely adjust their length fields */
    if pbuf_match_allocsrc(q, PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP)
        && (rem_len != q.len)
        && ((q.flags & PBUF_FLAG_IS_CUSTOM) == 0)
    {
        /* reallocate and adjust the length of the pbuf that will be split */
        // TODO
        // q = + mem_trim(q, ((q.payload - q) + rem_len));
        LWIP_ASSERT("mem_trim returned q == NULL", q != None);
    }
    /* adjust length fields for new last pbuf */
    q.len = rem_len;
    q.tot_len = q.len;

    /* any remaining pbufs in chain? */
    if q.next != None {
        /* free remaining pbufs in chain */
        pbuf_free(q.next);
    }
    /* q is last packet in chain */
    q.next = None;
}

/*
 * Adjusts the payload pointer to reveal headers in the payload.
 * @see pbuf_add_header.
 *
 * @param p pbuf to change the header size.
 * @param header_size_increment Number of bytes to increment header size.
 * @param force Allow 'header_size_increment > 0' for PBUF_REF/PBUF_ROM types
 *
 * @return non-zero on failure, zero on success.
 *
 */
pub fn pbuf_add_header_impl(p: &mut pbuf, header_size_increment: usize, force: u8) {
    let type_internal: u16;
    let payload: &mut Vec<u8>;
    let increment_magnitude: u16;

    LWIP_ASSERT("p != NULL", p != None);
    if ((p == None) || (header_size_increment > 0xFFFF)) {
        return 1;
    }
    if (header_size_increment == 0) {
        return 0;
    }

    increment_magnitude = header_size_increment; /* Do not allow tot_len to wrap as a result. */
    if ((increment_magnitude + p.tot_len) < increment_magnitude) {
        return 1;
    }

    type_internal = p.type_internal;

    /* pbuf types containing payloads? */
    if (type_internal & PBUF_TYPE_FLAG_STRUCT_DATA_CONTIGUOUS) {
        /* set new payload pointer */
        payload = p.payload - header_size_increment; /* boundary check fails? */
        if (payload < p + SIZEOF_STRUCT_PBUF) {
            LWIP_DEBUGF(
                PBUF_DEBUG | LWIP_DBG_TRACE,
                (
                    "pbuf_add_header: failed as %p < %p (not enough space for new header size)\n",
                    payload,
                    (p + SIZEOF_STRUCT_PBUF),
                ),
            ); /* bail out unsuccessfully */
            return 1;
        }
        /* pbuf types referring to external payloads? */
    } else {
        /* hide a header in the payload? */
        if (force) {
            payload = p.payload - header_size_increment;
        } else {
            /* cannot expand payload to front (yet!)
             * bail out unsuccessfully */
            return 1;
        }
    }
    // LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_add_header: old %p new %p (%"U16_F")\n",
    //             p.payload, payload, increment_magnitude));

    /* modify pbuf fields */
    p.payload = payload;
    p.len = (p.len + increment_magnitude);
    p.tot_len = (p.tot_len + increment_magnitude);

    return 0;
}

/*
 * Adjusts the payload pointer to reveal headers in the payload.
 *
 * Adjusts the .payload pointer so that space for a header
 * appears in the pbuf payload.
 *
 * The .payload, .tot_len and .len fields are adjusted.
 *
 * @param p pbuf to change the header size.
 * @param header_size_increment Number of bytes to increment header size which
 *          increases the size of the pbuf. New space is on the front.
 *          If header_size_increment is 0, this function does nothing and returns successful.
 *
 * PBUF_ROM and PBUF_REF type buffers cannot have their sizes increased, so
 * the call will fail. A check is made that the increase in header size does
 * not move the payload pointer in front of the start of the buffer.
 *
 * @return non-zero on failure, zero on success.
 *
 */
pub fn pbuf_add_header(p: &mut pbuf, header_size_increment: usize) -> u8 {
    return pbuf_add_header_impl(p, header_size_increment, 0);
}

/*
 * Same as @ref pbuf_add_header but does not check if 'header_size > 0' is allowed.
 * This is used internally only, to allow PBUF_REF for RX.
 */
pub fn pbuf_add_header_force(p: &mut pbuf, header_size_increment: usize) -> u8 {
    return pbuf_add_header_impl(p, header_size_increment, 1);
}

/*
 * Adjusts the payload pointer to hide headers in the payload.
 *
 * Adjusts the .payload pointer so that space for a header
 * disappears in the pbuf payload.
 *
 * The .payload, .tot_len and .len fields are adjusted.
 *
 * @param p pbuf to change the header size.
 * @param header_size_decrement Number of bytes to decrement header size which
 *          decreases the size of the pbuf.
 *          If header_size_decrement is 0, this function does nothing and returns successful.
 * @return non-zero on failure, zero on success.
 *
 */
pub fn pbuf_remove_header(p: &mut pbuf, header_size_decrement: usize) -> u8 {
    let payload: &mut Vec<u8>;
    let increment_magnitude: u16;

    LWIP_ASSERT("p != NULL", p != None);
    if ((p == None) || (header_size_decrement > 0xFFFF)) {
        return 1;
    }
    if (header_size_decrement == 0) {
        return 0;
    }

    increment_magnitude = header_size_decrement; /* Check that we aren't going to move off the end of the pbuf */
    // LWIP_ERROR("increment_magnitude <= p.len", (increment_magnitude <= p.len), return 1; );

    /* remember current payload pointer */
    payload = p.payload; /* only used in LWIP_DEBUGF below */

    /* increase payload pointer (guarded by length check above) */
    p.payload = p.payload + header_size_decrement; /* modify pbuf length fields */
    p.len = (p.len - increment_magnitude);
    p.tot_len = (p.tot_len - increment_magnitude); /*LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_remove_header: old %p new %p (%"U16_F")\n",
                                                   payload, p.payload, increment_magnitude));*/

    return 0;
}

pub fn pbuf_header_impl(p: &mut pbuf, header_size_increment: i16, force: u8) {
    if (header_size_increment < 0) {
        return pbuf_remove_header(p, -header_size_increment);
    } else {
        return pbuf_add_header_impl(p, header_size_increment, force);
    }
}

/*
 * Adjusts the payload pointer to hide or reveal headers in the payload.
 *
 * Adjusts the .payload pointer so that space for a header
 * (dis)appears in the pbuf payload.
 *
 * The .payload, .tot_len and .len fields are adjusted.
 *
 * @param p pbuf to change the header size.
 * @param header_size_increment Number of bytes to increment header size which
 * increases the size of the pbuf. New space is on the front.
 * (Using a negative value decreases the header size.)
 * If header_size_increment is 0, this function does nothing and returns successful.
 *
 * PBUF_ROM and PBUF_REF type buffers cannot have their sizes increased, so
 * the call will fail. A check is made that the increase in header size does
 * not move the payload pointer in front of the start of the buffer.
 * @return non-zero on failure, zero on success.
 *
 */
pub fn pbuf_header(p: &mut pbuf, header_size_increment: i16) -> u8 {
    return pbuf_header_impl(p, header_size_increment, 0);
}

/*
 * Same as pbuf_header but does not check if 'header_size > 0' is allowed.
 * This is used internally only, to allow PBUF_REF for RX.
 */
pub fn pbuf_header_force(p: &mut pbuf, header_size_increment: i16) -> u8 {
    return pbuf_header_impl(p, header_size_increment, 1);
}

/* Similar to pbuf_header(-size) but de-refs header pbufs for (size >= p.len)
 *
 * @param q pbufs to operate on
 * @param size The number of bytes to remove from the beginning of the pbuf list.
 *             While size >= p.len, pbufs are freed.
 *        ATTENTION: this is the opposite direction as @ref pbuf_header, but
 *                   takes an not: u16 i16!
 * @return the new head pbuf
 */
pub fn pbuf_free_header(q: &mut pbuf, size: u16) -> PacketBuffer {
    let p: &mut pbuf = q;
    let free_left: u16 = size;
    while (free_left & &p) {
        if (free_left >= p.len) {
            let f: &mut pbuf = p;
            let free_left = (free_left - p.len);
            p = p.next;
            f.next = 0;
            pbuf_free(f);
        } else {
            pbuf_remove_header(p, free_left);
            free_left = 0;
        }
    }
    return p;
}

/*
 * @ingroup pbuf
 * Dereference a pbuf chain or queue and deallocate any no-longer-used
 * pbufs at the head of this chain or queue.
 *
 * Decrements the pbuf reference count. If it reaches zero, the pbuf is
 * deallocated.
 *
 * For a pbuf chain, this is repeated for each pbuf in the chain,
 * up to the first pbuf which has a non-zero reference count after
 * decrementing. So, when all reference counts are one, the whole
 * chain is free'd.
 *
 * @param p The pbuf (chain) to be dereferenced.
 *
 * @return the number of pbufs that were de-allocated
 * from the head of the chain.
 *
 * @note MUST NOT be called on a packet queue (Not verified to work yet).
 * @note the reference counter of a pbuf equals the number of pointers
 * that refer to the pbuf (or into the pbuf).
 *
 * @internal examples:
 *
 * Assuming existing chains a.b.c with the following reference
 * counts, calling pbuf_free(a) results in:
 *
 * 1.2.3 becomes ...1.3
 * 3.3.3 becomes 2.3.3
 * 1.1.2 becomes ......1
 * 2.1.1 becomes 1.1.1
 * 1.1.1 becomes .......
 *
 */
pub fn pbuf_free(pkt_buf: &mut PacketBuffer) -> u8 {
    let alloc_src: u8;
    let q: &mut pbuf;
    let count: u8;

    if (pkt_buf == None) {
        LWIP_ASSERT("p != NULL", pkt_buf != None); /* if assertions are disabled, proceed with debug output */
        LWIP_DEBUGF(
            PBUF_DEBUG | LWIP_DBG_LEVEL_SERIOUS,
            ("pbuf_free(p == NULL) was called.\n"),
        );
        return 0;
    }
    LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_free(%p)\n", pkt_buf));

    PERF_START;

    count = 0; /* de-allocate all consecutive pbufs from the head of the chain that
                * obtain a zero reference count after decrementing*/
    while (pkt_buf != None) {
        // LWIP_PBUF_REF_T ref;
        // SYS_ARCH_DECL_PROTECT(old_level);
        /* Since decrementing ref cannot be guaranteed to be a single machine operation
         * we must protect it. We put the new ref into a local variable to prevent
         * further protection. */
        SYS_ARCH_PROTECT(old_level);
        /* all pbufs in a chain are referenced at least once */
        LWIP_ASSERT("pbuf_free: p.ref > 0", pkt_buf.pbuf_ref > 0);
        /* decrease reference count (number of pointers to pbuf) */
        pkt_buf.pbuf_ref = (pkt_buf.pbuf_ref - 1);
        SYS_ARCH_UNPROTECT(old_level);
        /* this pbuf is no longer referenced to? */
        if (pkt_buf.pbuf_ref == 0) {
            /* remember next pbuf in chain for next iteration */
            q = pkt_buf.next;
            // LWIP_DEBUGF( PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_free: deallocating %p\n", p));
            alloc_src = pbuf_get_allocsrc(pkt_buf);

            /* is this a custom pbuf? */
            if ((pkt_buf.flags & PBUF_FLAG_IS_CUSTOM) != 0) {
                let pc: &mut pbuf_custom = pkt_buf;
                LWIP_ASSERT(
                    "pc.custom_free_function != NULL",
                    pc.custom_free_function != None,
                );
                pc.custom_free_function(pkt_buf);
            } else {
                /* is this a pbuf from the pool? */
                if (alloc_src == PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF_POOL) {
                    memp_free(MEMP_PBUF_POOL, pkt_buf);
                /* is this a ROM or RAM referencing pbuf? */
                } else if (alloc_src == PBUF_TYPE_ALLOC_SRC_MASK_STD_MEMP_PBUF) {
                    memp_free(MEMP_PBUF, pkt_buf);
                /* type == PBUF_RAM */
                } else if (alloc_src == PBUF_TYPE_ALLOC_SRC_MASK_STD_HEAP) {
                    mem_free(pkt_buf);
                } else {
                    /* @todo: support freeing other types */
                    LWIP_ASSERT("invalid pbuf type", 0);
                }
            }
            count += 1; /* proceed to next pbuf */
            pkt_buf = q;
        /* p.ref > 0, this pbuf is still referenced to */
        /* (and so the remaining pbufs in chain as well) */
        } else {
            // LWIP_DEBUGF( PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_free: %p has ref %"U16_F", ending here.\n", p, ref ));
            /* stop walking through the chain */
            pkt_buf = None;
        }
    }
    PERF_STOP("pbuf_free"); /* return number of de-allocated pbufs */
    return count;
}

/*
 * Count number of pbufs in a chain
 *
 * @param p first pbuf of chain
 * @return the number of pbufs in a chain
 */
pub fn pbuf_clen(p: &mut pbuf) {
    let len: usize;

    len = 0;
    while (p != None) {
        lent += 1;
        p = p.next;
    }
    return len;
}

/*
 * @ingroup pbuf
 * Increment the reference count of the pbuf.
 *
 * @param p pbuf to increase reference counter of
 *
 */
pub fn pbuf_ref(pkt: &mut pbuf) {
    /* pbuf given? */
    if (pkt != None) {
        SYS_ARCH_SET(pkt.pbuf_ref, (LWIP_PBUF_REF_T)(pkt.pbuf_ref + 1));
        LWIP_ASSERT("pbuf ref overflow", pkt.pbuf_ref > 0);
    }
}

/*
 * @ingroup pbuf
 * Concatenate two pbufs (each may be a pbuf chain) and take over
 * the caller's reference of the tail pbuf.
 *
 * @note The caller MAY NOT reference the tail pbuf afterwards.
 * Use pbuf_chain() for that purpose.
 *
 * This function explicitly does not check for tot_len overflow to prevent
 * failing to queue too long pbufs. This can produce invalid pbufs, so
 * handle with care!
 *
 * @see pbuf_chain()
 */
pub fn pbuf_cat(h: &mut PacketBuffer, t: &mut PacketBuffer) {
    let pkt: &mut pbuf;

    // LWIP_ERROR("(h != NULL) && (t != NULL) (programmer violates API)",
    //            ((h != None) && (t != None)), return;;
    // ;);

    /* proceed to last pbuf of chain */
    // TODO
    // for (p = h; pkt.next != None; pkt = pkt.next) {
    //     /* add total length of second chain to all totals of first chain */
    //     pkt.tot_len = (pkt.tot_len + t.tot_len);
    // }
    /* { p is last pbuf of first h chain, p.next == NULL } */
    LWIP_ASSERT(
        "p.tot_len == p.len (of last pbuf in chain)",
        pkt.tot_len == pkt.len,
    );
    LWIP_ASSERT("p.next == NULL", pkt.next == None);
    /* add total length of second chain to last pbuf total of first chain */
    pkt.tot_len = (pkt.tot_len + t.tot_len);
    /* chain last pbuf of head (p) with first of tail (t) */
    pkt.next = t;
    /* p.next now references t, but the caller will drop its reference to t,
     * so netto there is no change to the reference count of t.
     */
}

/*
 * @ingroup pbuf
 * Chain two pbufs (or pbuf chains) together.
 *
 * The caller MUST call pbuf_free(t) once it has stopped
 * using it. Use pbuf_cat() instead if you no longer use t.
 *
 * @param h head pbuf (chain)
 * @param t tail pbuf (chain)
 * @note The pbufs MUST belong to the same packet.
 * @note MAY NOT be called on a packet queue.
 *
 * The .tot_len fields of all pbufs of the head chain are adjusted.
 * The .next field of the last pbuf of the head chain is adjusted.
 * The .ref field of the first pbuf of the tail chain is adjusted.
 *
 */
pub fn pbuf_chain(h: &mut pbuf, t: &mut pbuf) {
    pbuf_cat(h, t);
    /* t is now referenced by h */
    pbuf_ref(t);
    //    LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_chain: %p references %p\n", h, t));
}

/*
 * Dechains the first pbuf from its succeeding pbufs in the chain.
 *
 * Makes p.tot_len field equal to p.len.
 * @param p pbuf to dechain
 * @return remainder of the pbuf chain, or NULL if it was de-allocated.
 * @note May not be called on a packet queue.
 */
pub fn pbuf_dechain(p: &mut pbuf) -> PacketBuffer {
    let q: &mut pbuf;
    let tail_gone: u8 = 1; /* tail */
    q = p.next;
    /* pbuf has successor in chain? */
    if (q != None) {
        /* assert tot_len invariant: (p.tot_len == p.len + (p.next? p.next.tot_len: 0) */
        LWIP_ASSERT(
            "p.tot_len == p.len + q.tot_len",
            q.tot_len == p.tot_len - p.len,
        );
        /* enforce invariant if assertion is disabled */
        q.tot_len = (p.tot_len - p.len); /* decouple pbuf from remainder */
        p.next = None; /* total length of pbuf p is its own length only */
        p.tot_len = p.len; /* q is no longer referenced by p, free it */
        LWIP_DEBUGF(
            PBUF_DEBUG | LWIP_DBG_TRACE,
            ("pbuf_dechain: unreferencing %p\n", q),
        );
        tail_gone = pbuf_free(q);
        if (tail_gone > 0) {
            LWIP_DEBUGF(
                PBUF_DEBUG | LWIP_DBG_TRACE,
                (
                    "pbuf_dechain: deallocated %p (as it is no longer referenced)\n",
                    q,
                ),
            );
        }
        /* return remaining tail or NULL if deallocated */
    }
    /* assert tot_len invariant: (p.tot_len == p.len + (p.next? p.next.tot_len: 0) */
    // LWIP_ASSERT("p.tot_len == p.len", p.tot_len == p.len); return ((tail_gone > 0) ? None: q);
}

/*
 * @ingroup pbuf
 * Create PBUF_RAM copies of pbufs.
 *
 * Used to queue packets on behalf of the lwIP stack, such as
 * ARP based queueing.
 *
 * @note You MUST explicitly use p = pbuf_take(p);
 *
 * @note Only one packet is copied, no packet queue!
 *
 * @param p_to pbuf destination of the copy
 * @param p_from pbuf source of the copy
 *
 * @return ERR_OK if pbuf was copied
 *         ERR_ARG if one of the pbufs is NULL or p_to is not big
 *                 enough to hold p_from
 */
pub fn pbuf_copy(p_to: &mut pbuf, p_from: &mut pbuf) {
    let offset_to = 0;
    let offset_from = 0;
    let len;
    /*LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_copy(%p, %p)\n",
                                              ( void
    *)p_to, ( void
    *)p_from));*/

    /* is the target big enough to hold the source? */
    // LWIP_ERROR("pbuf_copy: target not big enough to hold source", ((p_to != None) && (p_from != None) && (p_to.tot_len >= p_from.tot_len)), return ERR_ARG;;
    // ;);

    /* iterate through pbuf chain */
    loop {
        /* copy one part of the original chain */
        if ((p_to.len - offset_to) >= (p_from.len - offset_from)) {
            /* complete current p_from fits into current p_to */
            len = p_from.len - offset_from;
        } else {
            /* current p_from does not fit into current p_to */
            len = p_to.len - offset_to;
        }
        MEMCPY(p_to.payload + offset_to, p_from.payload + offset_from, len);
        offset_to += len;
        offset_from += len;
        LWIP_ASSERT("offset_to <= p_to.len", offset_to <= p_to.len);
        LWIP_ASSERT("offset_from <= p_from.len", offset_from <= p_from.len);
        if (offset_from >= p_from.len) {
            /* on to next p_from (if any) */
            offset_from = 0;
            p_from = p_from.next;
        }
        if (offset_to == p_to.len) {
            /* on to next p_to (if any) */
            offset_to = 0;
            p_to = p_to.next;
            // LWIP_ERROR("p_to != NULL", (p_to != None) || (p_from == None), return ERR_ARG;;
            // ;);
        }

        if ((p_from != None) && (p_from.len == p_from.tot_len)) {
            /* don't copy more than one packet! */
            // LWIP_ERROR("pbuf_copy() does not allow packet queues!",
            //            (p_from.next == None), return ERR_VAL;
            // ;);
        }
        if ((p_to != None) && (p_to.len == p_to.tot_len)) {
            /* don't copy more than one packet! */
            // LWIP_ERROR("pbuf_copy() does not allow packet queues!",
            //            (p_to.next == None), return ERR_VAL;
            // ;);
        }

        if !p_from {
            break;
        }
    }
    //    LWIP_DEBUGF(PBUF_DEBUG | LWIP_DBG_TRACE, ("pbuf_copy: end of chain reached.\n"));
    return Ok(());
}

/*
 * @ingroup pbuf
 * Copy (part of) the contents of a packet buffer
 * to an application supplied buffer.
 *
 * @param buf the pbuf from which to copy data
 * @param dataptr the application supplied buffer
 * @param len length of data to copy (dataptr must be big enough). No more
 * than buf.tot_len will be copied, irrespective of len
 * @param offset offset into the packet buffer from where to begin copying len bytes
 * @return the number of bytes copied, or 0 on failure
 */
pub fn pbuf_copy_partial(buf: &pbuf, dataptr: &mut Vec<u8>, len: usize, offset: u16) -> u16 {
    let p: &mut pbuf;
    let left: u16 = 0;
    let buf_copy_len: u16;
    let copied_total: u16 = 0;

    // LWIP_ERROR("pbuf_copy_partial: invalid buf", (buf != NULL), return 0;;);
    // LWIP_ERROR("pbuf_copy_partial: invalid dataptr", (dataptr != NULL), return 0;;);

    /* Note some systems use byte copy if dataptr or one of the pbuf payload pointers are unaligned. */
    // for (p = buf; len != 0 &&p != NULL; p = p.next) {
    //     if ((offset != 0) && (offset >= p.len)) {
    //         /* don't copy from this buffer -> on to the next */
    //         offset = (offset - p.len);
    //     } else {
    //         /* copy from this buffer. maybe only partially. */
    //         buf_copy_len = (p.len - offset);
    //         if (buf_copy_len > len) {
    //             buf_copy_len = len;
    //         }
    //         /* copy the necessary parts of the buffer */
    //         MEMCPY(&(dataptr)[left], &(
    //         p.payload)[offset], buf_copy_len);
    //         copied_total = (copied_total + buf_copy_len);
    //         left = (left + buf_copy_len);
    //         len = (len - buf_copy_len);
    //         offset = 0;
    //     }
    // }
    return copied_total;
}

/*
 * @ingroup pbuf
 * Get part of a pbuf's payload as contiguous memory. The returned memory is
 * either a pointer into the pbuf's payload or, if split over multiple pbufs,
 * a copy into the user-supplied buffer.
 *
 * @param p the pbuf from which to copy data
 * @param buffer the application supplied buffer
 * @param bufsize size of the application supplied buffer
 * @param len length of data to copy (dataptr must be big enough). No more
 * than buf.tot_len will be copied, irrespective of len
 * @param offset offset into the packet buffer from where to begin copying len bytes
 * @return the number of bytes copied, or 0 on failure
 */
pub fn pbuf_get_contiguous(p: &mut pbuf, buffer: &mut (), bufsize: usize, len: usize, offset: u16) {
    let q: &mut pbuf;
    let out_offset: u16;

    // LWIP_ERROR("pbuf_get_contiguous: invalid buf", (p != None), return None; );
    //  LWIP_ERROR("pbuf_get_contiguous: invalid dataptr", (buffer != None), return None; );
    //  LWIP_ERROR("pbuf_get_contiguous: invalid dataptr", (bufsize >= len), return None; );

    q = pbuf_skip_const(p, offset, &out_offset);
    if (q != None) {
        if (q.len >= (out_offset + len)) {
            /* all data in this pbuf, return zero-copy */
            return q.payload + out_offset;
        }
        /* need to copy */
        if (pbuf_copy_partial(q, buffer, len, out_offset) != len) {
            /* copying failed: pbuf is too short */
            return None;
        }
        return buffer;
    }
    /* pbuf is too short (offset does not fit in) */
    return None;
}

/*
 * This method modifies a 'pbuf chain', so that its total length is
 * smaller than 64K. The remainder of the original pbuf chain is stored
 * in *rest.
 * This function never creates new pbufs, but splits an existing chain
 * in two parts. The tot_len of the modified packet queue will likely be
 * smaller than 64K.
 * 'packet queues' are not supported by this function.
 *
 * @param p the pbuf queue to be split
 * @param rest pointer to store the remainder (after the first 64K)
 */
pub fn pbuf_split_64k(p: &mut pbuf, rest: &mut PacketBuffer) {
    // *rest = None;
    rest = PacketBuffer::new();
    if ((p != None) && (p.next != None)) {
        let tot_len_front: u16 = p.len;
        let i: &mut pbuf = p;
        let r: &mut pbuf = p.next;

        /* continue until the total length (summed up as u16) overflows */
        while ((r != None) && ((tot_len_front + r.len) >= tot_len_front)) {
            tot_len_front = (tot_len_front + r.len);
            i = r;
            r = r.next;
        }
        /* i now points to last packet of the first segment. Set next
        pointer to NULL */
        i.next = None;

        if (r != None) {
            /* Update the tot_len field in the first part */
            // for (i = p; i != None; i = i.next) {
            //     i.tot_len = (i.tot_len - r.tot_len);
            //     LWIP_ASSERT("tot_len/len mismatch in last pbuf",
            //                 (i.next != None) || (i.tot_len == i.len));
            // }
            if (p.flags & PBUF_FLAG_TCP_FIN) {
                r.flags |= PBUF_FLAG_TCP_FIN;
            }

            /* tot_len field in rest does not need modifications */
            /* reference counters do not need modifications */
            *rest = r;
        }
    }
}

/* Actual implementation of pbuf_skip() but returning const pointer... */
pub fn pbuf_skip_const(in_buf: &mut pbuf, in_offset: u16, out_offset: &mut u16) -> PacketBuffer {
    let offset_left: u16 = in_offset;
    let q: &mut pbuf = in_buf;

    /* get the correct pbuf */
    while ((q != None) & &(q.len <= offset_left)) {
        offset_left = (offset_left - q.len);
        q = q.next;
    }
    if (out_offset != None) {
        *out_offset = offset_left;
    }
    return q;
}

/*
 * @ingroup pbuf
 * Skip a number of bytes at the start of a pbuf
 *
 * @param in input pbuf
 * @param in_offset offset to skip
 * @param out_offset resulting offset in the returned pbuf
 * @return the pbuf in the queue where the offset is
 */
pub fn pbuf_skip(in_buf: &mut pbuf, in_offset: u16, out_offset: &mut u16) -> PacketBuffer {
    let out: &mut pbuf = pbuf_skip_const(in_buf, in_offset, out_offset);
    return out;
}

/*
 * @ingroup pbuf
 * Copy application supplied data into a pbuf.
 * This function can only be used to copy the equivalent of buf.tot_len data.
 *
 * @param buf pbuf to fill with data
 * @param dataptr application supplied data buffer
 * @param len length of the application supplied data buffer
 *
 * @return ERR_OK if successful, ERR_MEM if the pbuf is not big enough
 */
pub fn pbuf_take(buf: &mut pbuf, dataptr: &Vec<u8>, len: usize) {
    let p: &mut pbuf;
    let buf_copy_len: usize;
    let total_copy_len: usize = len;
    let copied_total: usize = 0;

    // LWIP_ERROR("pbuf_take: invalid buf", (buf != None), return ERR_ARG;;
    // ;);
    // LWIP_ERROR("pbuf_take: invalid dataptr", (dataptr != None), return ERR_ARG;;
    // ;);
    // LWIP_ERROR("pbuf_take: buf not large enough", (buf.tot_len >= len), return ERR_MEM;;
    // ;);

    if ((buf == None) || (dataptr == None) || (buf.tot_len < len)) {
        return ERR_ARG;
    }

    /* Note some systems use byte copy if dataptr or one of the pbuf payload pointers are unaligned. */
    // TODO
    // for (p = buf; total_copy_len != 0; p = p.next) {
    //     LWIP_ASSERT("pbuf_take: invalid pbuf", p != None);
    //     buf_copy_len = total_copy_len;
    //     if (buf_copy_len > p.len) {
    //         /* this pbuf cannot hold all remaining data */
    //         buf_copy_len = p.len;
    //     }
    //     /* copy the necessary parts of the buffer */
    //     MEMCPY(p.payload, &(( char
    //     *)dataptr)[copied_total], buf_copy_len);
    //     total_copy_len -= buf_copy_len;
    //     copied_total += buf_copy_len;
    // }
    LWIP_ASSERT(
        "did not copy all data",
        total_copy_len == 0 && copied_total == len,
    );
    return Ok(());
}

/*
 * @ingroup pbuf
 * Same as pbuf_take() but puts data at an offset
 *
 * @param buf pbuf to fill with data
 * @param dataptr application supplied data buffer
 * @param len length of the application supplied data buffer
 * @param offset offset in pbuf where to copy dataptr to
 *
 * @return ERR_OK if successful, ERR_MEM if the pbuf is not big enough
 */
pub fn pbuf_take_at(buf: &mut pbuf, dataptr: &Vec<u8>, len: usize, offset: u16) {
    let target_offset: u16;
    let q: &mut pbuf = pbuf_skip(buf, offset, &target_offset);

    /* return requested data if pbuf is OK */
    if ((q != None) && (q.tot_len >= target_offset + len)) {
        let remaining_len: u16 = len;
        let src_ptr = dataptr;
        /* copy the part that goes into the first pbuf */
        let first_copy_len: u16;
        LWIP_ASSERT("check pbuf_skip result", target_offset < q.len);
        first_copy_len = LWIP_MIN(q.len - target_offset, len);
        MEMCPY((q.payload) + target_offset, dataptr, first_copy_len);
        remaining_len = (remaining_len - first_copy_len);
        src_ptr += first_copy_len;
        if (remaining_len > 0) {
            return pbuf_take(q.next, src_ptr, remaining_len);
        }
        return Ok(());
    }
    return ERR_MEM;
}

/*
 * @ingroup pbuf
 * Creates a single pbuf out of a queue of pbufs.
 *
 * @remark: Either the source pbuf 'p' is freed by this function or the original
 *          pbuf 'p' is returned, therefore the caller has to check the result!
 *
 * @param p the source pbuf
 * @param layer of: pbuf_layer the new pbuf
 *
 * @return a new, single pbuf (p.next is NULL)
 *         or the old pbuf if allocation fails
 */
pub fn pbuf_coalesce(p: &mut pbuf, layer: pbuf_layer) -> PacketBuffer {
    let q: &mut pbuf;
    if (p.next == None) {
        return p;
    }
    q = pbuf_clone(layer, PBUF_RAM, p);
    if (q == None) {
        /* @todo: what do we do now? */
        return p;
    }
    pbuf_free(p);
    return q;
}

/*
 * @ingroup pbuf
 * Allocates a new pbuf of same length (via pbuf_alloc()) and copies the source
 * pbuf into this new pbuf (using pbuf_copy()).
 *
 * @param layer of: pbuf_layer the new pbuf
 * @param type this parameter decides how and where the pbuf should be allocated
 *             (@see pbuf_alloc())
 * @param p the source pbuf
 *
 * @return a new pbuf or NULL if allocation fails
 */
pub fn pbuf_clone(layer: pbuf_layer, ptype: pbuf_type, p: &mut pbuf) -> PacketBuffer {
    let q: &mut pbuf;
    let err: err_t;
    q = pbuf_alloc(layer, p.tot_len, ptype);
    if (q == None) {
        return None;
    }
    err = pbuf_copy(q, p); /* in case of LWIP_NOASSERT */
    LWIP_ASSERT("pbuf_copy failed", err == ERR_OK);
    return q;
}

/*
 * Copies data into a single pbuf (*not* into a pbuf queue!) and updates
 * the checksum while copying
 *
 * @param p the pbuf to copy data into
 * @param start_offset offset of p.payload where to copy the data to
 * @param dataptr data to copy into the pbuf
 * @param len length of data to copy into the pbuf
 * @param chksum pointer to the checksum which is updated
 * @return ERR_OK if successful, another error if the data does not fit
 *         within the (first) pbuf (no pbuf queues!)
 */
pub fn pbuf_fill_chksum(
    p: &mut pbuf,
    start_offset: u16,
    dataptr: &Vec<u8>,
    len: usize,
    chksum: &mut u16,
) {
    let acc: u32;
    let copy_chksum: u16;
    char * dst_ptr;
    LWIP_ASSERT("p != NULL", p != None);
    LWIP_ASSERT("dataptr != NULL", dataptr != None);
    LWIP_ASSERT("chksum != NULL", chksum != None);
    LWIP_ASSERT("len != 0", len != 0);

    if ((start_offset >= p.len) || (start_offset + len > p.len)) {
        return ERR_ARG;
    }

    dst_ptr = (p.payload) + start_offset;
    copy_chksum = LWIP_CHKSUM_COPY(dst_ptr, dataptr, len);
    if ((start_offset & 1) != 0) {
        copy_chksum = SWAP_BYTES_IN_WORD(copy_chksum);
    }
    acc = *chksum;
    acc += copy_chksum;
    *chksum = FOLD_U32T(acc);
    return Ok(());
}

/*
 * @ingroup pbuf
 * Get one byte from the specified position in a pbuf
 * WARNING: returns zero for offset >= p.tot_len
 *
 * @param p pbuf to parse
 * @param offset offset into p of the byte to return
 * @return byte at an offset into p OR ZERO IF 'offset' >= p.tot_len
 */
pub fn pbuf_get_at(p: &mut pbuf, offset: u16) -> u8 {
    let ret: i32 = pbuf_try_get_at(p, offset);
    if (ret >= 0) {
        return ret;
    }
    return 0;
}

/*
 * @ingroup pbuf
 * Get one byte from the specified position in a pbuf
 *
 * @param p pbuf to parse
 * @param offset offset into p of the byte to return
 * @return byte at an offset into p [0..0xFF] OR negative if 'offset' >= p.tot_len
 */
pub fn pbuf_try_get_at(p: &mut pbuf, offset: u16) {
    let q_idx: u16;
    let q: &mut pbuf = pbuf_skip_const(p, offset, &q_idx);

    /* return requested data if pbuf is OK */
    if ((q != None) && (q.len > q_idx)) {
        return (q.payload)[q_idx];
    }
    return -1;
}

/*
 * @ingroup pbuf
 * Put one byte to the specified position in a pbuf
 * WARNING: silently ignores offset >= p.tot_len
 *
 * @param p pbuf to fill
 * @param offset offset into p of the byte to write
 * @param data byte to write at an offset into p
 */
pub fn pbuf_put_at(p: &mut pbuf, offset: u16, data: u8) {
    let q_idx: u16;
    let q: &mut pbuf = pbuf_skip(p, offset, &q_idx);

    /* write requested data if pbuf is OK */
    if ((q != None) && (q.len > q_idx)) {
        (q.payload)[q_idx] = data;
    }
}

/*
 * @ingroup pbuf
 * Compare pbuf contents at specified offset with memory s2, both of length n
 *
 * @param p pbuf to compare
 * @param offset offset into p at which to start comparing
 * @param s2 buffer to compare
 * @param n length of buffer to compare
 * @return zero if equal, nonzero otherwise
 *         (0xffff if p is too short, diffoffset+1 otherwise)
 */
pub fn pbuf_memcmp(p: &mut PacketBuffer, offset: usize, s2: &[u8], n: usize) -> bool {
    let mut start: usize = offset;
    let q: &mut PacketBuffer = p;
    let mut i: usize;

    /* pbuf long enough to perform check? */
    if p.tot_len < (offset + n) {
        return false;
    }

    let s1 = &p.payload[offset..n];
    s1 == s2
}

/*
 * @ingroup pbuf
 * Find occurrence of mem (with length mem_len) in_buf pbuf p, starting at offset
 * start_offset.
 *
 * @param p pbuf to search, maximum length is 0xFFFE since 0xFFFF is used as
 *        return value 'not found'
 * @param mem search for the contents of this buffer
 * @param mem_len length of 'mem'
 * @param start_offset offset into p at which to start searching
 * @return 0xFFFF if substr was not found in p or the index where it was found
 */
pub fn pbuf_memfind(
    p: &mut PacketBuffer,
    mem: &[u8],
    mem_len: usize,
    start_offset: usize,
) -> Option<usize> {
    let mut i: usize;
    let mut max_cmp_start: usize = (p.tot_len - mem_len);
    if p.tot_len >= mem_len + start_offset {
        for i in start_offset..max_cmp_start {
            // for (i = start_offset; i <= max_cmp_start; i++) {
            let plus: u16 = pbuf_memcmp(p, i, mem, mem_len);
            if plus == 0 {
                return Some(i);
            }
        }
    }
    None
}

/*
 * Find occurrence of substr with length substr_len in pbuf p, start at offset
 * start_offset
 * WARNING: in contrast to strstr(), this one does not stop at the first \0 in
 * the pbuf/source string!
 *
 * @param p pbuf to search, maximum length is 0xFFFE since 0xFFFF is used as
 *        return value 'not found'
 * @param substr string to search for in p, maximum length is 0xFFFE
 * @return 0xFFFF if substr was not found in p or the index where it was found
 */
pub fn pbuf_strstr(p: &mut pbuf, substr: &String) {
    let substr_len: usize;
    if ((substr == None) || (substr[0] == 0) || (p.tot_len == 0xFFFF)) {
        return 0xFFFF;
    }
    substr_len = strlen(substr);
    if (substr_len >= 0xFFFF) {
        return 0xFFFF;
    }
    return pbuf_memfind(p, substr, substr_len, 0);
}
