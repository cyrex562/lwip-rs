/*
* @file
* Dynamic pool memory manager
*
* lwIP has dedicated pools for many structures (netconn, protocol control blocks,
* packet buffers, ...). All these pools are managed here.
*
* @defgroup mempool Memory pools
* @ingroup infrastructure
* Custom memory pools

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

/* Make sure we include everything we need for size calculation required by memp_std.h */

/* needed by default MEMP_NUM_SYS_TIMEOUT */

// #define /* LWIP_MEMPOOL(name,num,size,desc) LWIP_MEMPOOL_DECLARE(name,num,size,desc) */

// const const: &mut memp_desc memp_pools[MEMP_MAX] = {
// // #define /* LWIP_MEMPOOL(name,num,size,desc) */ &memp_ ## name,

// };

// #undef MEMP_OVERFLOW_CHECK
/* MEMP_OVERFLOW_CHECK >= 2 does not work with MEMP_MEM_MALLOC, use 1 instead */
// pub const MEMP_OVERFLOW_CHECK: u32 = 1;

/*
 * Check that memp-lists don't form a circle, using "Floyd's cycle-finding algorithm".
 */
// pub fn memp_sanity(const desc: &mut memp_desc)
// {
//   t: &mut memp, *h;

//   t = *desc.tab;
//   if (t != None) {
//     for (h = t.next; (t != None) && (h != None); t = t.next,
//          h = ((h.next != None) ? h.next.next : None)) {
//       if (t == h) {
//         return 0;
//       }
//     }
//   }

//   return 1;
// }

/*
 * Check if a memp element was victim of an overflow or underflow
 * (e.g. the restricted area after/before it has been altered)
 *
 * @param p the memp element to check
 * @param desc the pool p comes from
 */
// pub fn
// memp_overflow_check_element(p: &mut memp,  desc: &mut memp_desc)
// {
//   mem_overflow_check_raw(p + MEMP_SIZE, desc.size, "pool ", desc.desc);
// }

/*
 * Initialize the restricted area of on memp element.
 */
// pub fn
// memp_overflow_init_element(p: &mut memp,  desc: &mut memp_desc)
// {
//   mem_overflow_init_raw(p + MEMP_SIZE, desc.size);
// }

/*
 * Do an overflow check for all elements in every pool.
 *
 * @see memp_overflow_check_element for a description of the check
 */
// pub fn
// memp_overflow_check_all()
// {
//   i: u16, j;
//   p: &mut memp;
//   SYS_ARCH_DECL_PROTECT(old_level);
//   SYS_ARCH_PROTECT(old_level);

//   for (i = 0; i < MEMP_MAX; += 1i) {
//     p = (struct memp *)LWIP_MEM_ALIGN(memp_pools[i].base);
//     for (j = 0; j < memp_pools[i].num; += 1j) {
//       memp_overflow_check_element(p, memp_pools[i]);
//       p = LWIP_ALIGNMENT_CAST(struct memp *, (p + MEMP_SIZE + memp_pools[i].size + MEM_SANITY_REGION_AFTER_ALIGNED));
//     }
//   }
//   SYS_ARCH_UNPROTECT(old_level);
// }

/*
 * Initialize custom memory pool.
 * Related functions: memp_malloc_pool, memp_free_pool
 *
 * @param desc pool to initialize
 */
// pub fn
// memp_init_pool(const desc: &mut memp_desc)
// {

//   let leti: i32;
//   memp: &mut memp;

//   *desc.tab = None;
//   memp = (struct memp *)LWIP_MEM_ALIGN(desc.base);

//   /* force memset on pool memory */
//   //memset(memp, 0, desc.num * (MEMP_SIZE + desc.size

//                                        + MEM_SANITY_REGION_AFTER_ALIGNED

//                                       ));

//   /* create a linked list of memp elements */
//   for (i = 0; i < desc.num; += 1i) {
//     memp.next = *desc.tab;
//     *desc.tab = memp;

//     memp_overflow_init_element(memp, desc);

//     /* cast through void* to get rid of alignment warnings */
//     memp = (struct memp *)(memp + MEMP_SIZE + desc.size

//                                    + MEM_SANITY_REGION_AFTER_ALIGNED

//                                   );
//   }

//   desc.stats.avail = desc.num;

//   desc.stats.name  = desc.desc;

// }

/*
 * Initializes lwIP built-in pools.
 * Related functions: memp_malloc, memp_free
 *
 * Carves out memp_memory into linked lists for each pool-type.
 */
// pub fn
// memp_init()
// {
//   let i: u16;

//   /* for every pool: */
//   for (i = 0; i < LWIP_ARRAYSIZE(memp_pools); i+= 1) {
//     memp_init_pool(memp_pools[i]);

//     lwip_stats.memp[i] = memp_pools[i].stats;

//   }

//   /* check everything a first time to see if it worked */
//   memp_overflow_check_all();

// }

// pub fn *

// do_memp_malloc_pool(const desc: &mut memp_desc)

// do_memp_malloc_pool_fn(const desc: &mut memp_desc, file: &String,  line: i32)

// {
//   memp: &mut memp;
//   SYS_ARCH_DECL_PROTECT(old_level);

//   memp = (struct memp *)mem_malloc(MEMP_SIZE + MEMP_ALIGN_SIZE(desc.size));
//   SYS_ARCH_PROTECT(old_level);
//  /* MEMP_MEM_MALLOC */
//   SYS_ARCH_PROTECT(old_level);

//   memp = *desc.tab;

//   if (memp != None) {

//     memp_overflow_check_element(memp, desc);

//     *desc.tab = memp.next;

//     memp.next = None;

//     memp.file = file;
//     memp.line = line;

//     memp_overflow_init_element(memp, desc);

//     LWIP_ASSERT("memp_malloc: memp properly aligned",
//                 ((mem_ptr_t)memp % MEM_ALIGNMENT) == 0);

//     desc.stats.used+= 1;
//     if (desc.stats.used > desc.stats.max) {
//       desc.stats.max = desc.stats.used;
//     }

//     SYS_ARCH_UNPROTECT(old_level);
//     /* cast through u8* to get rid of alignment warnings */
//     return (memp + MEMP_SIZE);
//   } else {

//     desc.stats.err+= 1;

//     SYS_ARCH_UNPROTECT(old_level);
// //    LWIP_DEBUGF(MEMP_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("memp_malloc: out of memory in pool %s\n", desc.desc));
//   }

//   return None;
// }

/*
 * Get an element from a custom pool.
 *
 * @param desc the pool to get an element from
 *
 * @return a pointer to the allocated memory or a NULL pointer on error
 */
// pub fn  *

// memp_malloc_pool(const desc: &mut memp_desc)

// memp_malloc_pool_fn(const desc: &mut memp_desc, file: &String,  line: i32)

// {
//   LWIP_ASSERT("invalid pool desc", desc != None);
//   if (desc == None) {
//     return None;
//   }

//   return do_memp_malloc_pool(desc);

//   return do_memp_malloc_pool_fn(desc, file, line);

// }

/*
 * Get an element from a specific pool.
 *
 * @param type the pool to get an element from
 *
 * @return a pointer to the allocated memory or a NULL pointer on error
 */
// pub fn  *

// memp_malloc(memp_t type)

// memp_malloc_fn(memp_t type, file: &String,  line: i32)

// {
//   memp: &mut ();
//   LWIP_ERROR("memp_malloc: type < MEMP_MAX", (type < MEMP_MAX), return None;);

//   memp_overflow_check_all();

//   memp = do_memp_malloc_pool(memp_pools[type]);

//   memp = do_memp_malloc_pool_fn(memp_pools[type], file, line);

//   return memp;
// }

// pub fn
// do_memp_free_pool(const desc: &mut memp_desc, mem: &mut ())
// {
//   memp: &mut memp;
//   SYS_ARCH_DECL_PROTECT(old_level);

//   LWIP_ASSERT("memp_free: mem properly aligned",
//               ((mem_ptr_t)mem % MEM_ALIGNMENT) == 0);

//   /* cast through void* to get rid of alignment warnings */
//   memp = (struct memp *)(mem - MEMP_SIZE);

//   SYS_ARCH_PROTECT(old_level);

//   memp_overflow_check_element(memp, desc);

//   desc.stats.used -= 1;

//   SYS_ARCH_UNPROTECT(old_level);
//   mem_free(memp);
//  /* MEMP_MEM_MALLOC */
//   memp.next = *desc.tab;
//   *desc.tab = memp;

//   LWIP_ASSERT("memp sanity", memp_sanity(desc));

//   SYS_ARCH_UNPROTECT(old_level);

// }

/*
 * Put a custom pool element back into its pool.
 *
 * @param desc the pool where to put mem
 * @param mem the memp element to free
 */
// pub fn
// memp_free_pool(const desc: &mut memp_desc, mem: &mut ())
// {
//   LWIP_ASSERT("invalid pool desc", desc != None);
//   if ((desc == None) || (mem == None)) {
//     return;
//   }

//   do_memp_free_pool(desc, mem);
// }

/*
 * Put an element back into its pool.
 *
 * @param type the pool where to put mem
 * @param mem the memp element to free
 */
// pub fn
// memp_free(memp_t type, mem: &mut ())
// {

//   old_first: &mut memp;

//   LWIP_ERROR("memp_free: type < MEMP_MAX", (type < MEMP_MAX), return;);

//   if (mem == None) {
//     return;
//   }

//   memp_overflow_check_all();

//   old_first = *memp_pools[type].tab;

//   do_memp_free_pool(memp_pools[type], mem);

//   if (old_first == None) {
//     LWIP_HOOK_MEMP_AVAILABLE(type);
//   }

// }
