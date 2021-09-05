/*
 * @file
 * Dynamic memory manager
 *
 * This is a lightweight replacement for the standard C library malloc().
 *
 * If you want to use the standard C library malloc() instead, define
 * MEM_LIBC_MALLOC to 1 in your lwipopts.h
 *
 * To let mem_malloc() use pools (prevents fragmentation and is much faster than
 * a heap but might waste some memory), define MEM_USE_POOLS to 1, define
 * MEMP_USE_CUSTOM_POOLS to 1 and create a file "lwippools.h" that includes a list
 * of pools like this (more pools can be added between _START and _END):
 *
 * Define three pools with sizes 256, 512, and 1512 bytes
 * LWIP_MALLOC_MEMPOOL_START
 * LWIP_MALLOC_MEMPOOL(20, 256)
 * LWIP_MALLOC_MEMPOOL(10, 512)
 * LWIP_MALLOC_MEMPOOL(5, 1512)
 * LWIP_MALLOC_MEMPOOL_END
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
 *         Simon Goldschmidt
 *
 */

/* This is overridable for tests only... */

// #define LWIP_MEM_ILLEGAL_FREE(msg)         LWIP_ASSERT(msg, 0)

// #define MEM_STATS_INC_LOCKED(x)         SYS_ARCH_LOCKED(MEM_STATS_INC(x))
// #define MEM_STATS_INC_USED_LOCKED(x, y) SYS_ARCH_LOCKED(MEM_STATS_INC_USED(x, y))
// #define MEM_STATS_DEC_USED_LOCKED(x, y) SYS_ARCH_LOCKED(MEM_STATS_DEC_USED(x, y))

// #define MEM_SANITY_OFFSET   MEM_SANITY_REGION_BEFORE_ALIGNED
// #define MEM_SANITY_OVERHEAD (MEM_SANITY_REGION_BEFORE_ALIGNED + MEM_SANITY_REGION_AFTER_ALIGNED)

pub const MEM_SANITY_OFFSET: u32 = 0;
pub const MEM_SANITY_OFFSET: u32 = 0;
pub const MEM_SANITY_OVERHEAD: u32 = 0;

/*
 * Check if a mep element was victim of an overflow or underflow
 * (e.g. the restricted area after/before it has been altered)
 *
 * @param p the mem element to check
 * @param size allocated size of the element
 * @param descr1 description of the element source shown on error
 * @param descr2 description of the element source shown on error
 */
pub fn mem_overflow_check_raw(p: &mut (), size: usize, descr1: &String, descr2: &String) {
    let k: u16;
    let m: &mut Vec<u8>;

    m = p + size;
    // for (k = 0; k < MEM_SANITY_REGION_AFTER_ALIGNED; k+= 1) {
    //   if (m[k] != 0xcd) {
    //     let errstr: String;
    //     snprintf(errstr, sizeof(errstr), "detected mem overflow in %s%s", descr1, descr2);
    //     LWIP_ASSERT(errstr, 0);
    //   }
    // }

    m = p - MEM_SANITY_REGION_BEFORE_ALIGNED;
    // for (k = 0; k < MEM_SANITY_REGION_BEFORE_ALIGNED; k+= 1) {
    //   if (m[k] != 0xcd) {
    //     let errstr: String;
    //     snprintf(errstr, sizeof(errstr), "detected mem underflow in %s%s", descr1, descr2);
    //     LWIP_ASSERT(errstr, 0);
    //   }
    // }
}

/*
 * Initialize the restricted area of a mem element.
 */
pub fn mem_overflow_init_raw(p: &mut (), size: usize) {
    let m: &mut Vec<u8>;

    m = p - MEM_SANITY_REGION_BEFORE_ALIGNED;
    //memset(m, 0xcd, MEM_SANITY_REGION_BEFORE_ALIGNED);

    m = p + size;
    //memset(m, 0xcd, MEM_SANITY_REGION_AFTER_ALIGNED);

    /* MEM_SANITY_REGION_BEFORE_ALIGNED > 0 || MEM_SANITY_REGION_AFTER_ALIGNED > 0 */
}

/* mem_init is not used when using pools instead of a heap or using
 * C library malloc().
 */
pub fn mem_init() {}

/* mem_trim is not used when using pools instead of a heap or using
 * C library malloc(): we can't free part of a pool element and the stack
 * support mem_trim() to return a different pointer
 */
pub fn mem_trim(mem: &mut (), mem_size: usize) {
    return mem;
}

/* lwIP heap implemented using C library malloc() */

/* in case C library malloc() needs extra protection,
 * allow these defines to be overridden.
 */

// #define mem_clib_free free

// #define mem_clib_malloc malloc

// #define mem_clib_calloc calloc

// #define MEM_LIBC_STATSHELPER_SIZE LWIP_MEM_ALIGN_SIZE(sizeof)

// pub const MEM_LIBC_STATSHELPER_SIZE: u32 = 0;

/*
 * Allocate a block of memory with a minimum of 'size' bytes.
 *
 * @param size is the minimum size of the requested block in bytes.
 * @return pointer to allocated memory or NULL if no free memory was found.
 *
 * Note that the returned value must always be aligned (as defined by MEM_ALIGNMENT).
 */
// pub fn  *
// mem_malloc(mem_size: usize)
// {
//   ret: &mut () = mem_clib_malloc(size + MEM_LIBC_STATSHELPER_SIZE);
//   if (ret == None) {
//     MEM_STATS_INC_LOCKED(err);
//   } else {
//     LWIP_ASSERT("malloc() must return aligned memory", LWIP_MEM_ALIGN(ret) == ret);

//     *(mem_usize *)ret = size;
//     ret = ret + MEM_LIBC_STATSHELPER_SIZE;
//     MEM_STATS_INC_USED_LOCKED(used, size);

//   }
//   return ret;
// }

/* Put memory back on the heap
 *
 * @param rmem is the pointer as returned by a previous call to mem_malloc()
 */
// pub fn
// mem_free(rmem: &mut ())
// {
//   LWIP_ASSERT("rmem != NULL", (rmem != None));
//   LWIP_ASSERT("rmem == MEM_ALIGN(rmem)", (rmem == LWIP_MEM_ALIGN(rmem)));

//   rmem = rmem - MEM_LIBC_STATSHELPER_SIZE;
//   MEM_STATS_DEC_USED_LOCKED(used, *(mem_usize *)rmem);

//   mem_clib_free(rmem);
// }

// #elif MEM_USE_POOLS

/* lwIP heap implemented with different sized pools */

/*
 * Allocate memory: determine the smallest pool that is big enough
 * to contain an element of 'size' and get an element from that pool.
 *
 * @param size the size in bytes of the memory needed
 * @return a pointer to the allocated memory or NULL if the pool is empty
 */
// pub fn  *
// mem_malloc(mem_size: usize)
// {
//   ret: &mut ();
//   element: &mut memp_malloc_helper = None;
//   memp_t poolnr;
//   mem_required_size: usize = size + LWIP_MEM_ALIGN_SIZE(sizeof(memp_malloc_helper));

//   // for (poolnr = MEMP_POOL_FIRST; poolnr <= MEMP_POOL_LAST; poolnr = (memp_t)(poolnr + 1)) {
//   //   /* is this pool big enough to hold an element of the required size
//   //      plus a struct memp_malloc_helper that saves the pool this element came from? */
//   //   if (required_size <= memp_pools[poolnr].size) {
//   //     element = memp_malloc(poolnr);
//   //     if (element == NULL) {
//   //       /* No need to DEBUGF or ASSERT: This error is already taken care of in memp.c */
//   //       /* Try a bigger pool if this one is empty! */
//   //       if (poolnr < MEMP_POOL_LAST) {
//   //         continue;
//   //       }

//   //       MEM_STATS_INC_LOCKED(err);
//   //       return NULL;
//   //     }
//   //     break;
//   //   }
//   // }
//   if (poolnr > MEMP_POOL_LAST) {
//     LWIP_ASSERT("mem_malloc(): no pool is that big!", 0);
//     MEM_STATS_INC_LOCKED(err);
//     return None;
//   }

//   /* save the pool number this element came from */
//   element.poolnr = poolnr;
//   /* and return a pointer to the memory directly after the struct memp_malloc_helper */
//   ret = element + LWIP_MEM_ALIGN_SIZE(sizeof(memp_malloc_helper));

//   /* truncating to is: u16 safe because struct memp_desc::size is u16 */
//   element.size = size;
//   MEM_STATS_INC_USED_LOCKED(used, element.size);

//   /* initialize unused memory (diff between requested size and selected pool's size) */
//   //memset(ret + size, 0xcd, memp_pools[poolnr].size - size);

//   return ret;
// }

/*
 * Free memory previously allocated by mem_malloc. Loads the pool number
 * and calls memp_free with that pool number to put the element back into
 * its pool
 *
 * @param rmem the memory element to free
 */
// pub fn
// mem_free(rmem: &mut ())
// {
//   hmem: &mut memp_malloc_helper;

//   LWIP_ASSERT("rmem != NULL", (rmem != None));
//   LWIP_ASSERT("rmem == MEM_ALIGN(rmem)", (rmem == LWIP_MEM_ALIGN(rmem)));

//   /* get the original struct memp_malloc_helper */
//   /* cast through to: &mut Vec<u8> get rid of alignment warnings */
//   hmem = (struct memp_malloc_helper *)(rmem - LWIP_MEM_ALIGN_SIZE(sizeof(memp_malloc_helper)));

//   LWIP_ASSERT("hmem != NULL", (hmem != None));
//   LWIP_ASSERT("hmem == MEM_ALIGN(hmem)", (hmem == LWIP_MEM_ALIGN(hmem)));
//   LWIP_ASSERT("hmem.poolnr < MEMP_MAX", (hmem.poolnr < MEMP_MAX));

//   MEM_STATS_DEC_USED_LOCKED(used, hmem.size);

//   {
//     let i: u16;
//     LWIP_ASSERT("MEM_USE_POOLS: invalid chunk size",
//                 hmem.size <= memp_pools[hmem.poolnr].size);
//     /* check that unused memory remained untouched (diff between requested size and selected pool's size) */
//     // for (i = hmem.size; i < memp_pools[hmem.poolnr].size; i+= 1) {
//     //   data: u8 = *(rmem + i);
//     //   LWIP_ASSERT("MEM_USE_POOLS: mem overflow detected", data == 0xcd);
//     // }
//   }

//   /* and put it in the pool we saved earlier */
//   memp_free(hmem.poolnr, hmem);
// }

/* MEM_USE_POOLS */
/* lwIP replacement for your libc malloc() */

/*
 * The heap is made up as a list of structs of this type.
 * This does not have to be aligned since for getting its size,
 * we only use the macro SIZEOF_STRUCT_MEM, which automatically aligns.
 */
// struct mem {
//   /* index (-> ram[next]) of the next struct */
//   let mem_next: usize;
//   /* index (-> ram[prev]) of the previous struct */
//   let mem_prev: usize;
//   /* 1: this area is used; 0: this area is unused */
//   let used: u8;

//   /* this keeps track of the user allocation size for guard checks */
//   let mem_user_size: usize;

// };

/* All allocated blocks will be MIN_SIZE bytes big, at least!
 * MIN_SIZE can be overridden to suit your needs. Smaller values save space,
 * larger values could prevent too small blocks to fragment the RAM too much. */

// pub const MIN_SIZE: u32 = 12;

/* some alignment macros: we define them here for better source code layout */
// #define MIN_SIZE_ALIGNED     LWIP_MEM_ALIGN_SIZE(MIN_SIZE)
// #define SIZEOF_STRUCT_MEM    LWIP_MEM_ALIGN_SIZE(sizeof(mem))
// #define MEM_SIZE_ALIGNED     LWIP_MEM_ALIGN_SIZE(MEM_SIZE)

/* If you want to relocate the heap to external memory, simply define
 * LWIP_RAM_HEAP_POINTER as a void-pointer to that location.
 * If so, make sure the memory at that location is big enough (see below on
 * how that space is calculated). */

/* the heap. we need one struct mem at the end and some room for alignment */
// LWIP_DECLARE_MEMORY_ALIGNED(ram_heap, MEM_SIZE_ALIGNED + (2 * SIZEOF_STRUCT_MEM));
// #define LWIP_RAM_HEAP_POINTER ram_heap

/* pointer to the heap (ram_heap): for alignment, ram is now a pointer instead of an array */
// static ram: &mut Vec<u8>;
/* the last entry, always unused! */
// static ram_end: &mut mem;

/* concurrent access protection */

// static sys_mutex_t mem_mutex;

// static volatile mem_free_count: u8;

/* Allow mem_free from other (e.g. interrupt) context */
// #define LWIP_MEM_FREE_DECL_PROTECT()  SYS_ARCH_DECL_PROTECT(lev_free)
// #define LWIP_MEM_FREE_PROTECT()       SYS_ARCH_PROTECT(lev_free)
// #define LWIP_MEM_FREE_UNPROTECT()     SYS_ARCH_UNPROTECT(lev_free)
// #define LWIP_MEM_ALLOC_DECL_PROTECT() SYS_ARCH_DECL_PROTECT(lev_alloc)
// #define LWIP_MEM_ALLOC_PROTECT()      SYS_ARCH_PROTECT(lev_alloc)
// #define LWIP_MEM_ALLOC_UNPROTECT()    SYS_ARCH_UNPROTECT(lev_alloc)
// #define LWIP_MEM_LFREE_VOLATILE       volatile

/* LWIP_ALLOW_MEM_FREE_FROM_OTHER_CONTEXT */

/* Protect the heap only by using a mutex */
// #define LWIP_MEM_FREE_DECL_PROTECT()
// #define LWIP_MEM_FREE_PROTECT()    sys_mutex_lock(&mem_mutex)
// #define LWIP_MEM_FREE_UNPROTECT()  sys_mutex_unlock(&mem_mutex)
/* mem_malloc is protected using mutex AND LWIP_MEM_ALLOC_PROTECT */
// #define LWIP_MEM_ALLOC_DECL_PROTECT()
// #define LWIP_MEM_ALLOC_PROTECT()
// #define LWIP_MEM_ALLOC_UNPROTECT()
// #define LWIP_MEM_LFREE_VOLATILE

/* pointer to the lowest free block, this is used for faster search */
// static struct mem * LWIP_MEM_LFREE_VOLATILE lfree;

// pub fn mem_sanity();
// #define MEM_SANITY() mem_sanity()
// #define MEM_SANITY()

// pub fn
// mem_overflow_init_element(mem: &mut mem, mem_user_size: usize)
// {
//   p: &mut () = mem + SIZEOF_STRUCT_MEM + MEM_SANITY_OFFSET;
//   mem.user_size = user_size;
//   mem_overflow_init_raw(p, user_size);
// }

// pub fn
// mem_overflow_check_element(mem: &mut mem)
// {
//   p: &mut () = mem + SIZEOF_STRUCT_MEM + MEM_SANITY_OFFSET;
//   mem_overflow_check_raw(p, mem.user_size, "heap", "");
// }
/* MEM_OVERFLOW_CHECK */
// #define mem_overflow_init_element(mem, size)
// #define mem_overflow_check_element(mem)

// static struct mem *
// ptr_to_mem(mem_ptr: usize)
// {
//   return (struct mem *)&ram[ptr];
// }

// pub fn mem_to_ptr(mem: &mut ())
// {
//   return (mem - ram);
// }

/*
 * "Plug holes" by combining adjacent empty struct mems.
 * After this function is through, there should not exist
 * one empty struct mem pointing to another empty struct mem.
 *
 * @param mem this points to a struct mem which just has been freed
 * @internal this function is only called by mem_free() and mem_trim()
 *
 * This assumes access to the heap is protected by the calling function
 * already.
 */
// pub fn
// plug_holes(mem: &mut mem)
// {
//   nmem: &mut mem;
//   pmem: &mut mem;

//   LWIP_ASSERT("plug_holes: mem >= ram", mem >= ram);
//   LWIP_ASSERT("plug_holes: mem < ram_end", mem < ram_end);
//   LWIP_ASSERT("plug_holes: mem.used == 0", mem.used == 0);

//   /* plug hole forward */
//   LWIP_ASSERT("plug_holes: mem.next <= MEM_SIZE_ALIGNED", mem.next <= MEM_SIZE_ALIGNED);

//   nmem = ptr_to_mem(mem.next);
//   if (mem != nmem && nmem.used == 0 && nmem != ram_end) {
//     /* if mem.next is unused and not end of ram, combine mem and mem.next */
//     if (lfree == nmem) {
//       lfree = mem;
//     }
//     mem.next = nmem.next;
//     if (nmem.next != MEM_SIZE_ALIGNED) {
//       ptr_to_mem(nmem.next).prev = mem_to_ptr(mem);
//     }
//   }

//   /* plug hole backward */
//   pmem = ptr_to_mem(mem.prev);
//   if (pmem != mem && pmem.used == 0) {
//     /* if mem.prev is unused, combine mem and mem.prev */
//     if (lfree == mem) {
//       lfree = pmem;
//     }
//     pmem.next = mem.next;
//     if (mem.next != MEM_SIZE_ALIGNED) {
//       ptr_to_mem(mem.next).prev = mem_to_ptr(pmem);
//     }
//   }
// }

/*
 * Zero the heap and initialize start, end and lowest-free
 */
// pub fn
// mem_init()
// {
//   mem: &mut mem;

//   LWIP_ASSERT("Sanity check alignment",
//               (SIZEOF_STRUCT_MEM & (MEM_ALIGNMENT - 1)) == 0);

//   /* align the heap */
//   ram = LWIP_MEM_ALIGN(LWIP_RAM_HEAP_POINTER);
//   /* initialize the start of the heap */
//   mem = (struct mem *)ram;
//   mem.next = MEM_SIZE_ALIGNED;
//   mem.prev = 0;
//   mem.used = 0;
//   /* initialize the end of the heap */
//   ram_end = ptr_to_mem(MEM_SIZE_ALIGNED);
//   ram_end.used = 1;
//   ram_end.next = MEM_SIZE_ALIGNED;
//   ram_end.prev = MEM_SIZE_ALIGNED;
//   MEM_SANITY();

//   /* initialize the lowest-free pointer to the start of the heap */
//   lfree = (struct mem *)ram;

//   MEM_STATS_AVAIL(avail, MEM_SIZE_ALIGNED);

//   if (sys_mutex_new(&mem_mutex) != ERR_OK) {
//     LWIP_ASSERT("failed to create mem_mutex", 0);
//   }
// }

/* Check if a struct mem is correctly linked.
 * If not, double-free is a possible reason.
 */
// pub fn mem_link_valid(mem: &mut mem)
// {
//   nmem: &mut mem, *pmem;
//   let mem_rmem_idx: usize;
//   rmem_idx = mem_to_ptr(mem);
//   nmem = ptr_to_mem(mem.next);
//   pmem = ptr_to_mem(mem.prev);
//   if ((mem.next > MEM_SIZE_ALIGNED) || (mem.prev > MEM_SIZE_ALIGNED) ||
//       ((mem.prev != rmem_idx) && (pmem.next != rmem_idx)) ||
//       ((nmem != ram_end) && (nmem.prev != rmem_idx))) {
//     return 0;
//   }
//   return 1;
// }

// pub fn
// mem_sanity()
// {
//   mem: &mut mem;
//   let last_used: u8;

//   /* begin with first element here */
//   mem = (struct mem *)ram;
//   LWIP_ASSERT("heap element used valid", (mem.used == 0) || (mem.used == 1));
//   last_used = mem.used;
//   LWIP_ASSERT("heap element prev ptr valid", mem.prev == 0);
//   LWIP_ASSERT("heap element next ptr valid", mem.next <= MEM_SIZE_ALIGNED);
//   LWIP_ASSERT("heap element next ptr aligned", LWIP_MEM_ALIGN(ptr_to_mem(mem.next) == ptr_to_mem(mem.next)));

//   /* check all elements before the end of the heap */
//   for (mem = ptr_to_mem(mem.next);
//        (mem > ram) && (mem < ram_end);
//        mem = ptr_to_mem(mem.next)) {
//     LWIP_ASSERT("heap element aligned", LWIP_MEM_ALIGN(mem) == mem);
//     LWIP_ASSERT("heap element prev ptr valid", mem.prev <= MEM_SIZE_ALIGNED);
//     LWIP_ASSERT("heap element next ptr valid", mem.next <= MEM_SIZE_ALIGNED);
//     LWIP_ASSERT("heap element prev ptr aligned", LWIP_MEM_ALIGN(ptr_to_mem(mem.prev) == ptr_to_mem(mem.prev)));
//     LWIP_ASSERT("heap element next ptr aligned", LWIP_MEM_ALIGN(ptr_to_mem(mem.next) == ptr_to_mem(mem.next)));

//     if (last_used == 0) {
//       /* 2 unused elements in a row? */
//       LWIP_ASSERT("heap element unused?", mem.used == 1);
//     } else {
//       LWIP_ASSERT("heap element unused member", (mem.used == 0) || (mem.used == 1));
//     }

//     LWIP_ASSERT("heap element link valid", mem_link_valid(mem));

//     /* used/unused altering */
//     last_used = mem.used;
//   }
//   LWIP_ASSERT("heap end ptr sanity", mem == ptr_to_mem(MEM_SIZE_ALIGNED));
//   LWIP_ASSERT("heap element used valid", mem.used == 1);
//   LWIP_ASSERT("heap element prev ptr valid", mem.prev == MEM_SIZE_ALIGNED);
//   LWIP_ASSERT("heap element next ptr valid", mem.next == MEM_SIZE_ALIGNED);
// }

/*
 * Put a struct mem back on the heap
 *
 * @param rmem is the data portion of a struct mem as returned by a previous
 *             call to mem_malloc()
 */
// pub fn
// mem_free(rmem: &mut ())
// {
//   mem: &mut mem;
//   LWIP_MEM_FREE_DECL_PROTECT();

//   if (rmem == None) {
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_LEVEL_SERIOUS, ("mem_free(p == NULL) was called.\n"));
//     return;
//   }
//   if (((rmem) & (MEM_ALIGNMENT - 1)) != 0) {
//     LWIP_MEM_ILLEGAL_FREE("mem_free: sanity check alignment");
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SEVERE, ("mem_free: sanity check alignment\n"));
//     /* protect mem stats from concurrent access */
//     MEM_STATS_INC_LOCKED(illegal);
//     return;
//   }

//   /* Get the corresponding struct mem: */
//   /* cast through to: &mut Vec<u8> get rid of alignment warnings */
//   mem = (struct mem *)(rmem - (SIZEOF_STRUCT_MEM + MEM_SANITY_OFFSET));

//   if (mem < ram || rmem + MIN_SIZE_ALIGNED > ram_end) {
//     LWIP_MEM_ILLEGAL_FREE("mem_free: illegal memory");
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SEVERE, ("mem_free: illegal memory\n"));
//     /* protect mem stats from concurrent access */
//     MEM_STATS_INC_LOCKED(illegal);
//     return;
//   }

//   mem_overflow_check_element(mem);

//   /* protect the heap from concurrent access */
//   LWIP_MEM_FREE_PROTECT();
//   /* mem has to be in a used state */
//   if (!mem.used) {
//     LWIP_MEM_ILLEGAL_FREE("mem_free: illegal memory: double free");
//     LWIP_MEM_FREE_UNPROTECT();
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SEVERE, ("mem_free: illegal memory: double free?\n"));
//     /* protect mem stats from concurrent access */
//     MEM_STATS_INC_LOCKED(illegal);
//     return;
//   }

//   if (!mem_link_valid(mem)) {
//     LWIP_MEM_ILLEGAL_FREE("mem_free: illegal memory: non-linked: double free");
//     LWIP_MEM_FREE_UNPROTECT();
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SEVERE, ("mem_free: illegal memory: non-linked: double free?\n"));
//     /* protect mem stats from concurrent access */
//     MEM_STATS_INC_LOCKED(illegal);
//     return;
//   }

//   /* mem is now unused. */
//   mem.used = 0;

//   if (mem < lfree) {
//     /* the newly freed struct is now the lowest */
//     lfree = mem;
//   }

//   MEM_STATS_DEC_USED(used, mem.next - ((mem - ram)));

//   /* finally, see if prev or next are free also */
//   plug_holes(mem);
//   MEM_SANITY();

//   mem_free_count = 1;

//   LWIP_MEM_FREE_UNPROTECT();
// }

/*
 * Shrink memory returned by mem_malloc().
 *
 * @param rmem pointer to memory allocated by mem_malloc the is to be shrinked
 * @param new_size required size after shrinking (needs to be smaller than or
 *                equal to the previous size)
 * @return for compatibility reasons: is always == rmem, at the moment
 *         or NULL if newsize is > old size, in which case rmem is NOT touched
 *         or freed!
 */
// pub fn  *
// mem_trim(rmem: &mut (), mem_new_size: usize)
// {
//   mem_size: usize, newsize;
//   mem_ptr: usize, ptr2;
//   mem: &mut mem, *mem2;
//   /* use the FREE_PROTECT here: it protects with sem OR SYS_ARCH_PROTECT */
//   LWIP_MEM_FREE_DECL_PROTECT();

//   /* Expand the size of the allocated memory region so that we can
//      adjust for alignment. */
//   newsize = LWIP_MEM_ALIGN_SIZE(new_size);
//   if (newsize < MIN_SIZE_ALIGNED) {
//     /* every data block must be at least MIN_SIZE_ALIGNED long */
//     newsize = MIN_SIZE_ALIGNED;
//   }

//   newsize += MEM_SANITY_REGION_BEFORE_ALIGNED + MEM_SANITY_REGION_AFTER_ALIGNED;

//   if ((newsize > MEM_SIZE_ALIGNED) || (newsize < new_size)) {
//     return None;
//   }

//   LWIP_ASSERT("mem_trim: legal memory", rmem >= ram &&
//               rmem < ram_end);

//   if (rmem < ram || rmem >= ram_end) {
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SEVERE, ("mem_trim: illegal memory\n"));
//     /* protect mem stats from concurrent access */
//     MEM_STATS_INC_LOCKED(illegal);
//     return rmem;
//   }
//   /* Get the corresponding struct mem ... */
//   /* cast through to: &mut Vec<u8> get rid of alignment warnings */
//   mem = (struct mem *)(rmem - (SIZEOF_STRUCT_MEM + MEM_SANITY_OFFSET));

//   mem_overflow_check_element(mem);

//   /* ... and its offset pointer */
//   ptr = mem_to_ptr(mem);

//   size = ((mem.next - ptr) - (SIZEOF_STRUCT_MEM + MEM_SANITY_OVERHEAD));
//   LWIP_ASSERT("mem_trim can only shrink memory", newsize <= size);
//   if (newsize > size) {
//     /* not supported */
//     return None;
//   }
//   if (newsize == size) {
//     /* No change in size, simply return */
//     return rmem;
//   }

//   /* protect the heap from concurrent access */
//   LWIP_MEM_FREE_PROTECT();

//   mem2 = ptr_to_mem(mem.next);
//   if (mem2.used == 0) {
//     /* The next struct is unused, we can simply move it at little */
//     let mem_next: usize;
//     LWIP_ASSERT("invalid next ptr", mem.next != MEM_SIZE_ALIGNED);
//     /* remember the old next pointer */
//     next = mem2.next;
//     /* create new struct mem which is moved directly after the shrinked mem */
//     ptr2 = (ptr + SIZEOF_STRUCT_MEM + newsize);
//     if (lfree == mem2) {
//       lfree = ptr_to_mem(ptr2);
//     }
//     mem2 = ptr_to_mem(ptr2);
//     mem2.used = 0;
//     /* restore the next pointer */
//     mem2.next = next;
//     /* link it back to mem */
//     mem2.prev = ptr;
//     /* link mem to it */
//     mem.next = ptr2;
//     /* last thing to restore linked list: as we have moved mem2,
//      * let 'mem2.next.prev' poto: i32 mem2 again. but only if mem2.next is not
//      * the end of the heap */
//     if (mem2.next != MEM_SIZE_ALIGNED) {
//       ptr_to_mem(mem2.next).prev = ptr2;
//     }
//     MEM_STATS_DEC_USED(used, (size - newsize));
//     /* no need to plug holes, we've already done that */
//   } else if (newsize + SIZEOF_STRUCT_MEM + MIN_SIZE_ALIGNED <= size) {
//     /* Next struct is used but there's room for another struct mem with
//      * at least MIN_SIZE_ALIGNED of data.
//      * Old size ('size') must be big enough to contain at least 'newsize' plus a struct mem
//      * ('SIZEOF_STRUCT_MEM') with some data ('MIN_SIZE_ALIGNED').
//      * @todo we could leave out MIN_SIZE_ALIGNED. We would create an empty
//      *       region that couldn't hold data, but when mem.next gets freed,
//      *       the 2 regions would be combined, resulting in more free memory */
//     ptr2 = (ptr + SIZEOF_STRUCT_MEM + newsize);
//     LWIP_ASSERT("invalid next ptr", mem.next != MEM_SIZE_ALIGNED);
//     mem2 = ptr_to_mem(ptr2);
//     if (mem2 < lfree) {
//       lfree = mem2;
//     }
//     mem2.used = 0;
//     mem2.next = mem.next;
//     mem2.prev = ptr;
//     mem.next = ptr2;
//     if (mem2.next != MEM_SIZE_ALIGNED) {
//       ptr_to_mem(mem2.next).prev = ptr2;
//     }
//     MEM_STATS_DEC_USED(used, (size - newsize));
//     /* the original mem.next is used, so no need to plug holes! */
//   }
//   /* else {
//     next struct mem is used but size between mem and mem2 is not big enough
//     to create another struct mem
//     -> don't do anyhting.
//     -> the remaining space stays unused since it is too small
//   } */
//   mem_overflow_init_element(mem, new_size);

//   MEM_SANITY();

//   mem_free_count = 1;

//   LWIP_MEM_FREE_UNPROTECT();
//   return rmem;
// }

/*
 * Allocate a block of memory with a minimum of 'size' bytes.
 *
 * @param size_in is the minimum size of the requested block in bytes.
 * @return pointer to allocated memory or NULL if no free memory was found.
 *
 * Note that the returned value will always be aligned (as defined by MEM_ALIGNMENT).
 */
// pub fn  *
// mem_malloc(mem_size_in: usize)
// {
//   mem_ptr: usize, ptr2, size;
//   mem: &mut mem, *mem2;

//   local_mem_free_count: u8 = 0;

//   LWIP_MEM_ALLOC_DECL_PROTECT();

//   if (size_in == 0) {
//     return None;
//   }

//   /* Expand the size of the allocated memory region so that we can
//      adjust for alignment. */
//   size = LWIP_MEM_ALIGN_SIZE(size_in);
//   if (size < MIN_SIZE_ALIGNED) {
//     /* every data block must be at least MIN_SIZE_ALIGNED long */
//     size = MIN_SIZE_ALIGNED;
//   }

//   size += MEM_SANITY_REGION_BEFORE_ALIGNED + MEM_SANITY_REGION_AFTER_ALIGNED;

//   if ((size > MEM_SIZE_ALIGNED) || (size < size_in)) {
//     return None;
//   }

//   /* protect the heap from concurrent access */
//   sys_mutex_lock(&mem_mutex);
//   LWIP_MEM_ALLOC_PROTECT();

//   /* run as long as a mem_free disturbed mem_malloc or mem_trim */
//   loop {
//     local_mem_free_count = 0;

//     /* Scan through the heap searching for a free block that is big enough,
//      * beginning with the lowest free block.
//      */
//     for (ptr = mem_to_ptr(lfree); ptr < MEM_SIZE_ALIGNED - size;
//          ptr = ptr_to_mem(ptr).next) {
//       mem = ptr_to_mem(ptr);

//       mem_free_count = 0;
//       LWIP_MEM_ALLOC_UNPROTECT();
//       /* allow mem_free or mem_trim to run */
//       LWIP_MEM_ALLOC_PROTECT();
//       if (mem_free_count != 0) {
//         /* If mem_free or mem_trim have run, we have to restart since they
//            could have altered our current struct mem. */
//         local_mem_free_count = 1;
//         break;
//       }

//       if ((!mem.used) &&
//           (mem.next - (ptr + SIZEOF_STRUCT_MEM)) >= size) {
//         /* mem is not used and at least perfect fit is possible:
//          * mem.next - (ptr + SIZEOF_STRUCT_MEM) gives us the 'user data size' of mem */
//         if (mem.next - (ptr + SIZEOF_STRUCT_MEM) >= (size + SIZEOF_STRUCT_MEM + MIN_SIZE_ALIGNED)) {
//           /* (in addition to the above, we test if another struct mem (SIZEOF_STRUCT_MEM) containing
//            * at least MIN_SIZE_ALIGNED of data also fits in the 'user data space' of 'mem')
//            * -> split large block, create empty remainder,
//            * remainder must be large enough to contain MIN_SIZE_ALIGNED data: if
//            * mem.next - (ptr + (2*SIZEOF_STRUCT_MEM)) == size,
//            * struct mem would fit in but no data between mem2 and mem2.next
//            * @todo we could leave out MIN_SIZE_ALIGNED. We would create an empty
//            *       region that couldn't hold data, but when mem.next gets freed,
//            *       the 2 regions would be combined, resulting in more free memory
//            */
//           ptr2 = (ptr + SIZEOF_STRUCT_MEM + size);
//           LWIP_ASSERT("invalid next ptr",ptr2 != MEM_SIZE_ALIGNED);
//           /* create mem2 struct */
//           mem2 = ptr_to_mem(ptr2);
//           mem2.used = 0;
//           mem2.next = mem.next;
//           mem2.prev = ptr;
//           /* and insert it between mem and mem.next */
//           mem.next = ptr2;
//           mem.used = 1;

//           if (mem2.next != MEM_SIZE_ALIGNED) {
//             ptr_to_mem(mem2.next).prev = ptr2;
//           }
//           MEM_STATS_INC_USED(used, (size + SIZEOF_STRUCT_MEM));
//         } else {
//           /* (a mem2 struct does no fit into the user data space of mem and mem.next will always
//            * be used at this point: if not we have 2 unused structs in a row, plug_holes should have
//            * take care of this).
//            * -> near fit or exact fit: do not split, no mem2 creation
//            * also can't move mem.next directly behind mem, since mem.next
//            * will always be used at this point!
//            */
//           mem.used = 1;
//           MEM_STATS_INC_USED(used, mem.next - mem_to_ptr(mem));
//         }

// mem_malloc_adjust_lfree:

//         if (mem == lfree) {
//           cur: &mut mem = lfree;
//           /* Find next free block after mem and update lowest free pointer */
//           while (cur.used && cur != ram_end) {

//             mem_free_count = 0;
//             LWIP_MEM_ALLOC_UNPROTECT();
//             /* prevent high interrupt latency... */
//             LWIP_MEM_ALLOC_PROTECT();
//             if (mem_free_count != 0) {
//               /* If mem_free or mem_trim have run, we have to restart since they
//                  could have altered our current struct mem or lfree. */
//               // goto mem_malloc_adjust_lfree;
//             }

//             cur = ptr_to_mem(cur.next);
//           }
//           lfree = cur;
//           LWIP_ASSERT("mem_malloc: !lfree.used", ((lfree == ram_end) || (!lfree.used)));
//         }
//         LWIP_MEM_ALLOC_UNPROTECT();
//         sys_mutex_unlock(&mem_mutex);
//         LWIP_ASSERT("mem_malloc: allocated memory not above ram_end.",
//                     mem + SIZEOF_STRUCT_MEM + size <= ram_end);
//         LWIP_ASSERT("mem_malloc: allocated memory properly aligned.",
//                     (mem + SIZEOF_STRUCT_MEM) % MEM_ALIGNMENT == 0);
//         LWIP_ASSERT("mem_malloc: sanity check alignment",
//                     ((mem) & (MEM_ALIGNMENT - 1)) == 0);

//         mem_overflow_init_element(mem, size_in);

//         MEM_SANITY();
//         return mem + SIZEOF_STRUCT_MEM + MEM_SANITY_OFFSET;
//       }
//     }

//     /* if we got interrupted by a mem_free, try again */
//   } while (local_mem_free_count != 0);

//   MEM_STATS_INC(err);
//   LWIP_MEM_ALLOC_UNPROTECT();
//   sys_mutex_unlock(&mem_mutex);
// //  LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("mem_malloc: could not allocate %"S16_F" bytes\n", size));
//   return None;
// }

// pub fn  *
// mem_calloc(mem_count: usize, mem_size: usize)
// {
//   return mem_clib_calloc(count, size);
// }

/* MEM_LIBC_MALLOC && (!LWIP_STATS || !MEM_STATS) */
/*
 * Contiguously allocates enough space for count objects that are size bytes
 * of memory each and returns a pointer to the allocated memory.
 *
 * The allocated memory is filled with bytes of value zero.
 *
 * @param count number of objects to allocate
 * @param size size of the objects to allocate
 * @return pointer to allocated memory / NULL pointer if there is an error
 */
// pub fn  *
// mem_calloc(mem_count: usize, mem_size: usize)
// {
//   p: &mut ();
//   alloc_size: usize = count * size;

//   if (alloc_size != alloc_size) {
// //    LWIP_DEBUGF(MEM_DEBUG | LWIP_DBG_LEVEL_SERIOUS, ("mem_calloc: could not allocate %"SZT_F" bytes\n", alloc_size));
//     return None;
//   }

//   /* allocate 'count' objects of size 'size' */
//   p = mem_malloc(alloc_size);
//   if (p) {
//     /* zero the memory */
//     //memset(p, 0, alloc_size);
//   }
//   return p;
// }
