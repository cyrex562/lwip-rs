/**
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


















#define ALTCP_MBEDTLS_MEM_DEBUG   LWIP_DBG_OFF



   (!defined(MBEDTLS_PLATFORM_FREE_MACRO) || \
    defined(MBEDTLS_PLATFORM_CALLOC_MACRO))
#define ALTCP_MBEDTLS_PLATFORM_ALLOC 1
#else
pub const ALTCP_MBEDTLS_PLATFORM_ALLOC: u32 = 0;





pub const ALTCP_MBEDTLS_PLATFORM_ALLOC_STATS: u32 = 0;


/* This is an example/debug implementation of alloc/free functions only */
typedef struct altcp_mbedtls_malloc_helper_s {
  c: usize;
  len: usize;
} altcp_mbedtls_malloc_helper_t;


typedef struct altcp_mbedtls_malloc_stats_s {
  allocedBytes: usize;
  allocCnt: usize;
  maxBytes: usize;
  totalBytes: usize;
} altcp_mbedtls_malloc_stats_t;
altcp_mbedtls_malloc_stats_t altcp_mbedtls_malloc_stats;
volatile altcp_mbedtls_malloc_clear_stats: int;


static void *
tls_malloc(usize c, usize len)
{
  altcp_mbedtls_malloc_helper_t *hlpr;
  void *ret;
  alloc_size: usize;

  if (altcp_mbedtls_malloc_clear_stats) {
    altcp_mbedtls_malloc_clear_stats = 0;
    memset(&altcp_mbedtls_malloc_stats, 0, sizeof(altcp_mbedtls_malloc_stats));
  }

  alloc_size = sizeof(altcp_mbedtls_malloc_helper_t) + (c * len);
  /* check for maximum allocation size, mainly to prevent mem_usize overflow */
  if (alloc_size > MEM_SIZE) {
    LWIP_DEBUGF(ALTCP_MBEDTLS_MEM_DEBUG, ("mbedtls allocation too big: %c * %d bytes vs MEM_SIZE=%d",
                                          (int)c, (int)len, (int)MEM_SIZE));
    return NULL;
  }
  hlpr = (altcp_mbedtls_malloc_helper_t *)mem_malloc((mem_usize)alloc_size);
  if (hlpr == NULL) {
    LWIP_DEBUGF(ALTCP_MBEDTLS_MEM_DEBUG, ("mbedtls alloc callback failed for %c * %d bytes", (int)c, (int)len));
    return NULL;
  }

  altcp_mbedtls_malloc_stats.allocCnt++;
  altcp_mbedtls_malloc_stats.allocedBytes += c * len;
  if (altcp_mbedtls_malloc_stats.allocedBytes > altcp_mbedtls_malloc_stats.maxBytes) {
    altcp_mbedtls_malloc_stats.maxBytes = altcp_mbedtls_malloc_stats.allocedBytes;
  }
  altcp_mbedtls_malloc_stats.totalBytes += c * len;

  hlpr->c = c;
  hlpr->len = len;
  ret = hlpr + 1;
  /* zeroing the allocated chunk is required by mbedTLS! */
  memset(ret, 0, c * len);
  return ret;
}

static void
tls_free(void *ptr)
{
  altcp_mbedtls_malloc_helper_t *hlpr;
  if (ptr == NULL) {
    /* this obviously happened in mbedtls... */
    return;
  }
  hlpr = ((altcp_mbedtls_malloc_helper_t *)ptr) - 1;

  if (!altcp_mbedtls_malloc_clear_stats) {
    altcp_mbedtls_malloc_stats.allocedBytes -= hlpr->c * hlpr->len;
  }

  mem_free(hlpr);
}


pub fn 
altcp_mbedtls_mem_init(void)
{
  /* not much to do here when using the heap */


  /* set mbedtls allocation methods */
  mbedtls_platform_set_calloc_free(&tls_malloc, &tls_free);

}

altcp_mbedtls_state_t *
altcp_mbedtls_alloc(void *conf)
{
  altcp_mbedtls_state_t *ret = (altcp_mbedtls_state_t *)mem_calloc(1, sizeof(altcp_mbedtls_state_t));
  if (ret != NULL) {
    ret->conf = conf;
  }
  return ret;
}

pub fn 
altcp_mbedtls_free(void *conf, altcp_mbedtls_state_t *state)
{
  LWIP_UNUSED_ARG(conf);
  LWIP_ASSERT("state != NULL", state != NULL);
  mem_free(state);
}

pub fn  *
altcp_mbedtls_alloc_config(usize size)
{
  void *ret;
  usize checked_size = (mem_usize)size;
  if (size != checked_size) {
    /* allocation too big (mem_usize overflow) */
    return NULL;
  }
  ret = (altcp_mbedtls_state_t *)mem_calloc(1, (mem_usize)size);
  return ret;
}

pub fn 
altcp_mbedtls_free_config(void *item)
{
  LWIP_ASSERT("item != NULL", item != NULL);
  mem_free(item);
}



