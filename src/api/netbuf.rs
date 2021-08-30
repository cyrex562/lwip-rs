/*
 * @file
 * Network buffer management
 *
 * @defgroup netbuf Network buffers
 * @ingroup netconn
 * Network buffer descriptor for @ref netconn. Based on @ref pbuf internally
 * to avoid copying data around.\n
 * Buffers must not be shared accross multiple threads, all functions except
 * netbuf_new() and netbuf_delete() are not thread-safe.
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

/*
 * @ingroup netbuf
 * Create (allocate) and initialize a new netbuf.
 * The netbuf doesn't yet contain a packet buffer!
 *
 * @return a pointer to a new netbuf
 *         NULL on lack of memory
 */
pub fn netbuf_new() -> netbuf {
    let buf: &mut netbuf;

    buf = memp_malloc(MEMP_NETBUF);
    if (buf != None) {
        //memset(buf, 0, sizeof(netbuf));
    }
    return buf;
}

/*
 * @ingroup netbuf
 * Deallocate a netbuf allocated by netbuf_new().
 *
 * @param buf pointer to a netbuf allocated by netbuf_new()
 */
pub fn netbuf_delete(buf: &mut netbuf) {
    if (buf != None) {
        if (buf.p != None) {
            pbuf_free(buf.p);
            buf.p = buf.ptr = None;
        }
        memp_free(MEMP_NETBUF, buf);
    }
}

/*
 * @ingroup netbuf
 * Allocate memory for a packet buffer for a given netbuf.
 *
 * @param buf the netbuf for which to allocate a packet buffer
 * @param size the size of the packet buffer to allocate
 * @return pointer to the allocated memory
 *         NULL if no memory could be allocated
 */
pub fn netbuf_alloc(buf: &mut netbuf, size: u16) -> Vec<u8> {
    // LWIP_ERROR("netbuf_alloc: invalid buf", (buf != NULL), return NULL;);

    /* Deallocate any previously allocated memory. */
    if (buf.p != None) {
        pbuf_free(buf.p);
    }
    buf.p = pbuf_alloc(PBUF_TRANSPORT, size, PBUF_RAM);
    if (buf.p == None) {
        return None;
    }
    LWIP_ASSERT("check that first pbuf can hold size", (buf.p.len >= size));
    buf.ptr = buf.p;
    return buf.p.payload;
}

/*
 * @ingroup netbuf
 * Free the packet buffer included in a netbuf
 *
 * @param buf pointer to the netbuf which contains the packet buffer to free
 */
pub fn netbuf_free(buf: &mut netbuf) {
    // LWIP_ERROR("netbuf_free: invalid buf", (buf != NULL), return;);
    if (buf.p != None) {
        pbuf_free(buf.p);
    }
    buf.p = buf.ptr = None;

    buf.flags = 0;
    buf.toport_chksum = 0;
}

/*
 * @ingroup netbuf
 * Let a netbuf reference existing (non-volatile) data.
 *
 * @param buf netbuf which should reference the data
 * @param dataptr pointer to the data to reference
 * @param size size of the data
 * @return ERR_OK if data is referenced
 *         ERR_MEM if data couldn't be referenced due to lack of memory
 */
pub fn netbuf_ref(buf: &mut netbuf, dataptr: &Vec<u8>, size: u16) {
    // LWIP_ERROR("netbuf_ref: invalid buf", (buf != NULL), return ERR_ARG;);
    if (buf.p != None) {
        pbuf_free(buf.p);
    }
    buf.p = pbuf_alloc(PBUF_TRANSPORT, 0, PBUF_REF);
    if (buf.p == None) {
        buf.ptr = None;
        return ERR_MEM;
    }
    (buf.p).payload = dataptr;
    buf.p.len = buf.p.tot_len = size;
    buf.ptr = buf.p;
   return Ok(());
}

/*
 * @ingroup netbuf
 * Chain one netbuf to another (@see pbuf_chain)
 *
 * @param head the first netbuf
 * @param tail netbuf to chain after head, freed by this function, may not be reference after returning
 */
pub fn netbuf_chain(head: &mut netbuf, tail: &mut netbuf) {
    // LWIP_ERROR("netbuf_chain: invalid head", (head != NULL), return;);
    // LWIP_ERROR("netbuf_chain: invalid tail", (tail != NULL), return;);
    pbuf_cat(head.p, tail.p);
    head.ptr = head.p;
    memp_free(MEMP_NETBUF, tail);
}

/*
 * @ingroup netbuf
 * Get the data pointer and length of the data inside a netbuf.
 *
 * @param buf netbuf to get the data from
 * @param dataptr pointer to a void pointer where to store the data pointer
 * @param len pointer to an where: u16 the length of the data is stored
 * @return ERR_OK if the information was retrieved,
 *         ERR_BUF on error.
 */
pub fn netbuf_data(buf: &mut netbuf, dataptr: &mut Vec<u8>, len: &mut u16) {
    // LWIP_ERROR("netbuf_data: invalid buf", (buf != NULL), return ERR_ARG;);
    // LWIP_ERROR("netbuf_data: invalid dataptr", (dataptr != NULL), return ERR_ARG;);
    // LWIP_ERROR("netbuf_data: invalid len", (len != NULL), return ERR_ARG;);

    if (buf.ptr == None) {
        return ERR_BUF;
    }
    *dataptr = buf.ptr.payload;
    *len = buf.ptr.len;
   return Ok(());
}

/*
 * @ingroup netbuf
 * Move the current data pointer of a packet buffer contained in a netbuf
 * to the next part.
 * The packet buffer itself is not modified.
 *
 * @param buf the netbuf to modify
 * @return -1 if there is no next part
 *         1  if moved to the next part but now there is no next part
 *         0  if moved to the next part and there are still more parts
 */
pub fn netbuf_next(buf: &mut netbuf) -> s8 {
    // LWIP_ERROR("netbuf_next: invalid buf", (buf != NULL), return -1;);
    if (buf.ptr.next == None) {
        return -1;
    }
    buf.ptr = buf.ptr.next;
    if (buf.ptr.next == None) {
        return 1;
    }
    return 0;
}

/*
 * @ingroup netbuf
 * Move the current data pointer of a packet buffer contained in a netbuf
 * to the beginning of the packet.
 * The packet buffer itself is not modified.
 *
 * @param buf the netbuf to modify
 */
pub fn netbuf_first(buf: &mut netbuf) {
    // LWIP_ERROR("netbuf_first: invalid buf", (buf != NULL), return;);
    buf.ptr = buf.p;
}
