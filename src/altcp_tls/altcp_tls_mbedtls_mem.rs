/*
 * @file
 * Application layered TCP connection API (to be used from TCPIP thread)
 *
 * This file contains memory management functions for a TLS layer using mbedTLS.
 *
 * ATTENTION: For production usage, you might want to override this file with
 *            your own implementation since this implementation simply uses the
 *            lwIP heap without caring for fragmentation or leaving heap for
 *            other parts of lwIP!
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt
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
 * Missing things / @todo:
 * - RX data is acknowledged after receiving (tcp_recved is called when enqueueing
 *   the pbuf for mbedTLS receive, not when processed by mbedTLS or the inner
 *   connection; altcp_recved() from inner connection does nothing)
 * - TX data is marked as 'sent' (i.e. acknowledged; sent callback is called) right
 *   after enqueueing for transmission, not when actually ACKed be the remote host.
 */

use crate::altcp_tls::altcp_tls_mbedtls::AlTcpTlsConfig;
use crate::altcp_tls::altcp_tls_mbedtls_structs::AlTcpMbedTlsState;

pub const ALTCP_MBEDTLS_MEM_DEBUG: bool = LWIP_DBG_OFF;

//    (!defined(MBEDTLS_PLATFORM_FREE_MACRO) || \
//     defined(MBEDTLS_PLATFORM_CALLOC_MACRO))
// #define ALTCP_MBEDTLS_PLATFORM_ALLOC 1
// #else
pub const ALTCP_MBEDTLS_PLATFORM_ALLOC: u32 = 0;

pub const ALTCP_MBEDTLS_PLATFORM_ALLOC_STATS: u32 = 0;

/* This is an example/debug implementation of alloc/free functions only */
pub struct altcp_mbedtls_malloc_helper {
    c: usize,
    len: usize,
}

pub struct altcp_mbedtls_malloc_stats {
    pub allocedBytes: usize,
    pub allocCnt: usize,
    pub maxBytes: usize,
    pub totalBytes: usize,
}
// altcp_mbedtls_malloc_stats_t altcp_mbedtls_malloc_stats;
// volatile altcp_mbedtls_malloc_clear_stats: i32;

pub fn tls_malloc(c: usize, len: usize) -> altcp_mbedtls_malloc_helper {
    let mut ret: Vec<u8> = Vec::new();
    let alloc_size: usize;

    if altcp_mbedtls_malloc_clear_stats {
        altcp_mbedtls_malloc_clear_stats = 0;
        //memset(
        //     &altcp_mbedtls_malloc_stats,
        //     0,
        //     sizeof(altcp_mbedtls_malloc_stats),
        // );
    }

    alloc_size = sizeof(altcp_mbedtls_malloc_helper_t) + (c * len);
    /* check for maximum allocation size, mainly to prevent mem_overflow: usize */
    if alloc_size > MEM_SIZE {
        // LWIP_DEBUGF(ALTCP_MBEDTLS_MEM_DEBUG, ("mbedtls allocation too big: %c * %d bytes vs MEM_SIZE=%d",
        //                                       c, len, MEM_SIZE));
        return None;
    }
    // hlpr = (altcp_mbedtls_malloc_helper_t *)mem_malloc((mem_usize)alloc_size);
    let mut hlpr = altcp_mbedtls_malloc_helper { c, len };
    // if hlpr == NULL {
    //   // LWIP_DEBUGF(ALTCP_MBEDTLS_MEM_DEBUG, ("mbedtls alloc callback failed for %c * %d bytes", c, len));
    //   return NULL;
    // }

    // altcp_mbedtls_malloc_stats.allocCnt+= 1;
    // altcp_mbedtls_malloc_stats.allocedBytes += c * len;
    if altcp_mbedtls_malloc_stats.allocedBytes > altcp_mbedtls_malloc_stats.maxBytes {
        altcp_mbedtls_malloc_stats.maxBytes = altcp_mbedtls_malloc_stats.allocedBytes;
    }
    altcp_mbedtls_malloc_stats.totalBytes += c * len;

    hlpr.c = c;
    hlpr.len = len;
    // ret = hlpr + 1;
    /* zeroing the allocated chunk is required by mbedTLS! */
    // memset(ret, 0, c * len);
    return hlpr;
}

pub fn tls_free(ptr: &mut Vec<u8>) {
    altcp_mbedtls_malloc_helper_t * hlpr;
    if ptr == None {
        /* this obviously happened in mbedtls... */
        return;
    }
    hlpr = ptr - 1;

    if !altcp_mbedtls_malloc_clear_stats {
        altcp_mbedtls_malloc_stats.allocedBytes -= hlpr.c * hlpr.len;
    }

    mem_free(hlpr);
}

pub fn altcp_mbedtls_mem_init() {
    /* not much to do here when using the heap */

    /* set mbedtls allocation methods */
    mbedtls_platform_set_calloc_free(&tls_malloc, &tls_free);
}

pub fn altcp_mbedtls_alloc<T>(conf: &mut T) -> altcp_mbedtls_state {
    altcp_mbedtls_state * ret = mem_calloc(1, sizeof(altcp_mbedtls_state));

    let mut ret = AlTcpMbedTlsState::new();

    if ret != None {
        ret.conf = conf;
    }
    return ret;
}

// pub fn altcp_mbedtls_free(conf: &mut Vec<u8>, state: &mut AlTcpMbedTlsState) {
//     LWIP_ASSERT("state != NULL", state != None);
//     // mem_free(state);
// }

pub fn altcp_mbedtls_alloc_config(size: usize) -> Vec<u8> {
    // void * ret; checked_size: usize = (mem_usize)size;
    // if (size != checked_size) {
    // /* allocation too big (mem_overflow: usize) */ return NULL;
    // }
    // ret = mem_calloc(1, (mem_usize)size); return ret;
    let mut ret: Vec<u8> = Vec::new();
    ret.reserve(size);
    ret
}

pub fn altcp_mbedtls_free_config(item: &mut AlTcpTlsConfig) {
    // LWIP_ASSERT("item != NULL", item != NULL);
    // mem_free(item);
    unimplemented!()
}
