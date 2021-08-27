/*
 * @file
 * memory pools lwIP internal implementations (do not use in application code)
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


// #define LWIP_HDR_MEMP_PRIV_H













/* MEMP_SIZE: save space for struct memp and for sanity check */
#define MEMP_SIZE          (LWIP_MEM_ALIGN_SIZE(sizeof(struct memp)) + MEM_SANITY_REGION_BEFORE_ALIGNED)
#define MEMP_ALIGN_SIZE(x) (LWIP_MEM_ALIGN_SIZE(x) + MEM_SANITY_REGION_AFTER_ALIGNED)

 /* MEMP_OVERFLOW_CHECK */

/* No sanity checks
 * We don't need to preserve the struct memp while not allocated, so we
 * can save a little space and set MEMP_SIZE to 0.
 */
pub const MEMP_SIZE: u32 = 0;
#define MEMP_ALIGN_SIZE(x) (LWIP_MEM_ALIGN_SIZE(x))




struct memp {
  next: &mut memp;

  let file: String;
  let letline: i32;

};



/* Use a helper type to get the start and end of the user "memory pools" for mem_malloc */
typedef enum {
    /* Get the first (via:
       MEMP_POOL_HELPER_START = ( 1*MEMP_POOL_A + 0*MEMP_POOL_B + 0*MEMP_POOL_C + 0)*/
    MEMP_POOL_HELPER_FIRST = (
// #define /* LWIP_MEMPOOL(name,num,size,desc) */
// #define LWIP_MALLOC_MEMPOOL_START 1
// #define LWIP_MALLOC_MEMPOOL(num, size) * MEMP_POOL_##size + 0
// #define LWIP_MALLOC_MEMPOOL_END

    ) ,
    /* Get the last (via:
       MEMP_POOL_HELPER_END = ( 0 + MEMP_POOL_A*0 + MEMP_POOL_B*0 + MEMP_POOL_C*1) */
    MEMP_POOL_HELPER_LAST = (
// #define /* LWIP_MEMPOOL(name,num,size,desc) */
// #define LWIP_MALLOC_MEMPOOL_START
// #define LWIP_MALLOC_MEMPOOL(num, size) 0 + MEMP_POOL_##size *
// #define LWIP_MALLOC_MEMPOOL_END 1

    )
} memp_pool_helper_t;

/* The actual start and stop values are here (cast them over)
   We use this helper type and these defines so we can avoid using const memp_t values */
#define MEMP_POOL_FIRST ((memp_t) MEMP_POOL_HELPER_FIRST)
#define MEMP_POOL_LAST   ((memp_t) MEMP_POOL_HELPER_LAST)


/* Memory pool descriptor */
struct memp_desc {

  /* Textual description */
  let desc: String;


  /* Statistics */
  stats: &mut stats_mem;


  /* Element size */
  let size: u16;


  /* Number of elements */
  let num: u16;

  /* Base address */
  base: &mut Vec<u8>;

  /* First free element of each pool. Elements form a linked list. */
  struct memp **tab;

};


#define DECLARE_LWIP_MEMPOOL_DESC(desc) (desc),

#define DECLARE_LWIP_MEMPOOL_DESC(desc)



// #define LWIP_MEMPOOL_DECLARE_STATS_INSTANCE(name) static struct stats_mem name;
// #define LWIP_MEMPOOL_DECLARE_STATS_REFERENCE(name) &name,

// #define LWIP_MEMPOOL_DECLARE_STATS_INSTANCE(name)
// #define LWIP_MEMPOOL_DECLARE_STATS_REFERENCE(name)


pub fn  memp_init_pool(const desc: &mut memp_desc);


pub fn  *memp_malloc_pool_fn(const struct memp_desc* desc,  char* file,  line: i32);
#define memp_malloc_pool(d) memp_malloc_pool_fn((d), __FILE__, __LINE__)

pub fn  *memp_malloc_pool(const desc: &mut memp_desc);

pub fn   memp_free_pool(const struct memp_desc* desc, mem: &mut ());


}



