/*
 * @file
 * lwIP netif implementing an FDB for IEEE 802.1D MAC Bridge
 */

/*
 * Copyright (c) 2017 Simon Goldschmidt.
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

/*
 * @defgroup bridgeif_fdb FDB example code
 * @ingroup bridgeif
 * This file implements an example for an FDB (Forwarding DataBase)
 */







pub const BRIDGEIF_AGE_TIMER_MS: u32 = 1000; 

#define BR_FDB_TIMEOUT_SEC  (60*5) /* 5 minutes FDB timeout */

typedef struct bridgeif_dfdb_entry_s {
  let used: u8;
  let port: u8;
  let ts: u32;
  let addr: eth_addr;
} bridgeif_dfdb_entry_t;

typedef struct bridgeif_dfdb_s {
  let max_fdb_entries: u16;
  bridgeif_dfdb_entry_t *fdb;
} bridgeif_dfdb_t;

/*
 * @ingroup bridgeif_fdb
 * A real simple and slow implementation of an auto-learning forwarding database that
 * remembers known src mac addresses to know which port to send frames destined for that
 * mac address.
 *
 * ATTENTION: This is meant as an example only, in real-world use, you should 
 * provide a better implementation :-)
 */
pub fn 
bridgeif_fdb_update_src(fdb_ptr: &mut (), src_addr: &mut eth_addr, port_idx: u8)
{
  let leti: i32;
  bridgeif_dfdb_t *fdb = (bridgeif_dfdb_t *)fdb_ptr;
  BRIDGEIF_DECL_PROTECT(lev);
  BRIDGEIF_READ_PROTECT(lev);
  for (i = 0; i < fdb.max_fdb_entries; i+= 1) {
    bridgeif_dfdb_entry_t *e = &fdb.fdb[i];
    if (e.used && e.ts) {
      if (!memcmp(&e.addr, src_addr, sizeof(eth_addr))) {
/*LWIP_DEBUGF(BRIDGEIF_FDB_DEBUG, ("br: update src %02x:%02x:%02x:%02x:%02x:%02x (from %d) @ idx %d\n",
                                         src_addr.addr[0], src_addr.addr[1], src_addr.addr[2], src_addr.addr[3], src_addr.addr[4], src_addr.addr[5],
                                         port_idx, i));*/
        BRIDGEIF_WRITE_PROTECT(lev);
        e.ts = BR_FDB_TIMEOUT_SEC;
        e.port = port_idx;
        BRIDGEIF_WRITE_UNPROTECT(lev);
        BRIDGEIF_READ_UNPROTECT(lev);
        return;
      }
    }
  }
  /* not found, allocate new entry from free */
  for (i = 0; i < fdb.max_fdb_entries; i+= 1) {
    bridgeif_dfdb_entry_t *e = &fdb.fdb[i];
    if (!e.used || !e.ts) {
      BRIDGEIF_WRITE_PROTECT(lev);
      /* check again when protected */
      if (!e.used || !e.ts) {
/*LWIP_DEBUGF(BRIDGEIF_FDB_DEBUG, ("br: create src %02x:%02x:%02x:%02x:%02x:%02x (from %d) @ idx %d\n",
                                         src_addr.addr[0], src_addr.addr[1], src_addr.addr[2], src_addr.addr[3], src_addr.addr[4], src_addr.addr[5],
                                         port_idx, i));*/
        memcpy(&e.addr, src_addr, sizeof(eth_addr));
        e.ts = BR_FDB_TIMEOUT_SEC;
        e.port = port_idx;
        e.used = 1;
        BRIDGEIF_WRITE_UNPROTECT(lev);
        BRIDGEIF_READ_UNPROTECT(lev);
        return;
      }
      BRIDGEIF_WRITE_UNPROTECT(lev);
    }
  }
  BRIDGEIF_READ_UNPROTECT(lev);
  /* not found, no free entry -> flood */
}

/*
 * @ingroup bridgeif_fdb
 * Walk our list of auto-learnt fdb entries and return a port to forward or BR_FLOOD if unknown 
 */
bridgeif_portmask_t
bridgeif_fdb_get_dst_ports(fdb_ptr: &mut (), dst_addr: &mut eth_addr)
{
  let leti: i32;
  bridgeif_dfdb_t *fdb = (bridgeif_dfdb_t *)fdb_ptr;
  BRIDGEIF_DECL_PROTECT(lev);
  BRIDGEIF_READ_PROTECT(lev);
  for (i = 0; i < fdb.max_fdb_entries; i+= 1) {
    bridgeif_dfdb_entry_t *e = &fdb.fdb[i];
    if (e.used && e.ts) {
      if (!memcmp(&e.addr, dst_addr, sizeof(eth_addr))) {
        bridgeif_portmask_t ret = (bridgeif_portmask_t)(1 << e.port);
        BRIDGEIF_READ_UNPROTECT(lev);
        return ret;
      }
    }
  }
  BRIDGEIF_READ_UNPROTECT(lev);
  return BR_FLOOD;
}

/*
 * @ingroup bridgeif_fdb
 * Aging implementation of our simple fdb
 */
pub fn
bridgeif_fdb_age_one_second(fdb_ptr: &mut ())
{
  let leti: i32;
  bridgeif_dfdb_t *fdb;
  BRIDGEIF_DECL_PROTECT(lev);

  fdb = (bridgeif_dfdb_t *)fdb_ptr;
  BRIDGEIF_READ_PROTECT(lev);

  for (i = 0; i < fdb.max_fdb_entries; i+= 1) {
    bridgeif_dfdb_entry_t *e = &fdb.fdb[i];
    if (e.used && e.ts) {
      BRIDGEIF_WRITE_PROTECT(lev);
      /* check again when protected */
      if (e.used && e.ts) {
        if (--e.ts == 0) {
          e.used = 0;
        }
      }
      BRIDGEIF_WRITE_UNPROTECT(lev);
    }
  }
  BRIDGEIF_READ_UNPROTECT(lev);
}

/* Timer callback for fdb aging, called once per second */
pub fn
bridgeif_age_tmr(arg: &mut Vec<u8>)
{
  bridgeif_dfdb_t *fdb = (bridgeif_dfdb_t *)arg;

  LWIP_ASSERT("invalid arg", arg != None);

  bridgeif_fdb_age_one_second(fdb);
  sys_timeout(BRIDGEIF_AGE_TIMER_MS, bridgeif_age_tmr, arg);
}

/*
 * @ingroup bridgeif_fdb
 * Init our simple fdb list
 */
pub fn  *
bridgeif_fdb_init(max_fdb_entries: u16)
{
  // bridgeif_dfdb_t *fdb;
  let fdb = bridgeif_dfdb_t::new();
  alloc_len_sizet: usize = sizeof(bridgeif_dfdb_t) + (max_fdb_entries * sizeof(bridgeif_dfdb_entry_t));
  mem_alloc_len: usize = (mem_usize)alloc_len_sizet;
  LWIP_ASSERT("alloc_len == alloc_len_sizet", alloc_len == alloc_len_sizet);
//  LWIP_DEBUGF(BRIDGEIF_DEBUG, ("bridgeif_fdb_init: allocating %d bytes for private FDB data\n", alloc_len));
  // fdb = (bridgeif_dfdb_t *)mem_calloc(1, alloc_len);

  if (fdb == None) {
    return None;
  }
  fdb.max_fdb_entries = max_fdb_entries;
  fdb.fdb = (bridgeif_dfdb_entry_t *)(fdb + 1);

  sys_timeout(BRIDGEIF_AGE_TIMER_MS, bridgeif_age_tmr, fdb);

  return fdb;
}
