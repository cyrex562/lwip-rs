/*
 * @file
 * Support for different processor and compiler architectures
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

// // #define LWIP_HDR_ARCH_H
// #define LITTLE_ENDIAN 1234
// #define BIG_ENDIAN 4321

/*
 * @defgroup compiler_abstraction Compiler/platform abstraction
 * @ingroup sys_layer
 * All defines related to this section must not be placed in lwipopts.h,
 * but in arch/cc.h!
 * If the compiler does not provide memset() this file must include a
 * definition of it, or include a file which defines it.
 * These options cannot be \#defined in lwipopts.h since they are not options
 * of lwIP itself, but options of the lwIP port to your system.
 * @{
 */

/* Define the byte order of the system.
 * Needed for conversion of network data to host byte order.
 * Allowed values: LITTLE_ENDIAN and BIG_ENDIAN
 */

// #define BYTE_ORDER LITTLE_ENDIAN

/* Define random number generator function of your system */
// // #define LWIP_RAND() ((u32)rand())


/* Platform specific diagnostic output.\n
 * Note the default implementation pulls in printf, which may
 * in turn pull in a lot of standard libary code. In resource-constrained 
 * systems, this should be defined to something less resource-consuming.
 */

// // #define LWIP_PLATFORM_DIAG(x) do {printf x;} while(0)

/* Platform specific assertion handling.\n
 * Note the default implementation pulls in printf, fflush and abort, which may
 * in turn pull in a lot of standard libary code. In resource-constrained 
 * systems, this should be defined to something less resource-consuming.
 */

// // #define LWIP_PLATFORM_ASSERT(x) do {printf("Assertion \"%s\" failed at line %d in %s\n", \
//                                      x, __LINE__, __FILE__); fflush(NULL); abort();} while(0)

/* Define this to 1 in arch/cc.h of your port if you do not want to
 * include stddef.h header to get usize. You need to typedef usize
 * by yourself in this case.
 */

// pub const LWIP_NO_STDDEF_H: u32 = 0;

/* Define this to 1 in arch/cc.h of your port if your compiler does not provide
 * the stdint.h header. You need to typedef the generic types listed in
 * lwip/arch.h yourself in this case (u8, u16...).
 */

// pub const LWIP_NO_STDINT_H: u32 = 0;

/* Define generic types used in lwIP */

/* stdint.h is C99 which should also provide support for 64-bit integers */

// // #define LWIP_HAVE_INT64 1

// typedef uint8_t   u8;
// typedef int8_t    s8_t;
// typedef uint16_t  u16;
// typedef int16_t   i16;
// typedef uint32_t  u32;
// typedef int32_t   i32;

// typedef uint64_t  u64_t;
// typedef int64_t   s64_t;

// typedef uintptr_t mem_ptr_t;

/* Define this to 1 in arch/cc.h of your port if your compiler does not provide
 * the inttypes.h header. You need to define the format strings listed in
 * lwip/arch.h yourself in this case (X8_F, U16_F...).
 */

pub const LWIP_NO_INTTYPES_H: u32 = 0;


/* Define (sn)printf formatters for these lwIP types */



// #define X8_F  "02" PRIx8


// #define U16_F PRIu16


// #define S16_F PRId16


// #define X16_F PRIx16


// #define U32_F PRIu32


// #define S32_F PRId32


// #define X32_F PRIx32


// #define SZT_F PRIuPTR



/* Define this to 1 in arch/cc.h of your port if your compiler does not provide
 * the limits.h header. You need to define the type limits yourself in this case
 * (e.g. INT_MAX, SSIZE_MAX).
 */

pub const LWIP_NO_LIMITS_H: u32 = 0;


/* Include limits.h? */




/* Do we need to define isize? This is a compatibility hack:
 * Unfortunately, this type seems to be unavailable on some systems (even if
 * sys/types or unistd.h are available).
 * Being like that, we define it to 'int' if SSIZE_MAX is not defined.
 */

/* If SSIZE_MAX is defined, unistd.h should provide the type as well */

pub const LWIP_NO_UNISTD_H: u32 = 0;




// #else /* SSIZE_MAX */
// typedef isize: int;
// #define SSIZE_MAX INT_MAX


/* some maximum values needed in lwip code */
pub const LWIP_UINT32_MAX: u32 = 0xffffffff;

/* Define this to 1 in arch/cc.h of your port if your compiler does not provide
 * the ctype.h header. If ctype.h is available, a few character functions
 * are mapped to the appropriate functions (lwip_islower, lwip_isdigit...), if
 * not, a private implementation is provided.
 */

pub const LWIP_NO_CTYPE_H: u32 = 0;


// TODO
// #define lwip_in_range(c, lo, up)  ((u8)(c) >= (lo) && (u8)(c) <= (up))
// #define lwip_isdigit(c)           lwip_in_range((c), '0', '9')
// #define lwip_isxdigit(c)          (lwip_isdigit(c) || lwip_in_range((c), 'a', 'f') || lwip_in_range((c), 'A', 'F'))
// #define lwip_islower(c)           lwip_in_range((c), 'a', 'z')
// #define lwip_isspace(c)           ((c) == ' ' || (c) == '\f' || (c) == '\n' || (c) == '\r' || (c) == '\t' || (c) == '\v')
// #define lwip_isupper(c)           lwip_in_range((c), 'A', 'Z')
// #define lwip_tolower(c)           (lwip_isupper(c) ? (c) - 'A' + 'a' : c)
// #define lwip_toupper(c)           (lwip_islower(c) ? (c) - 'a' + 'A' : c)
// #else

// #define lwip_isdigit(c)           isdigit((unsigned char)(c))
// #define lwip_isxdigit(c)          isxdigit((unsigned char)(c))
// #define lwip_islower(c)           islower((unsigned char)(c))
// #define lwip_isspace(c)           isspace((unsigned char)(c))
// #define lwip_isupper(c)           isupper((unsigned char)(c))
// #define lwip_tolower(c)           tolower((unsigned char)(c))
// #define lwip_toupper(c)           toupper((unsigned char)(c))


/* C++ const_cast<target_type>(val) equivalent to remove constness from a value (GCC -Wcast-qual) */

// TODO // #define LWIP_CONST_CAST(target_type, val) ((target_type)((ptrdiff_t)val))


/* Get rid of alignment cast warnings (GCC -Wcast-align) */

// TODO // #define LWIP_ALIGNMENT_CAST(target_type, val) LWIP_CONST_CAST(target_type, val)


/* Get rid of warnings related to pointer-to-numeric and vice-versa casts,
 * e.g. "conversion from 'u8' to 'void *' of greater size"
 */

// TODO // #define LWIP_PTR_NUMERIC_CAST(target_type, val) LWIP_CONST_CAST(target_type, val)


/* Avoid warnings/errors related to implicitly casting away packed attributes by doing a explicit cast */

// TODO // #define LWIP_PACKED_CAST(target_type, val) LWIP_CONST_CAST(target_type, val)


/* Allocates a memory buffer of specified size that is of sufficient size to align
 * its start address using LWIP_MEM_ALIGN.
 * You can declare your own version here e.g. to enforce alignment without adding
 * trailing padding bytes (see LWIP_MEM_ALIGN_BUFFER) or your own section placement
 * requirements.\n
 * e.g. if you use gcc and need 32 bit alignment:\n
 * \// #define LWIP_DECLARE_MEMORY_ALIGNED(variable_name, size) variable_name: u8[size] \_\_attribute\_\_((aligned(4)))\n
 * or more portable:\n
 * \// #define LWIP_DECLARE_MEMORY_ALIGNED(variable_name, size) u32 variable_name[(size + sizeof(u32) - 1) / sizeof(u32)]
 */

// TODO // #define LWIP_DECLARE_MEMORY_ALIGNED(variable_name, size) variable_name: u8[LWIP_MEM_ALIGN_BUFFER(size)]


/* Calculate memory size for an aligned buffer - returns the next highest
 * multiple of MEM_ALIGNMENT (e.g. LWIP_MEM_ALIGN_SIZE(3) and
 * LWIP_MEM_ALIGN_SIZE(4) will both yield 4 for MEM_ALIGNMENT == 4).
 */

// TODO // #define LWIP_MEM_ALIGN_SIZE(size) (((size) + MEM_ALIGNMENT - 1U) & ~(MEM_ALIGNMENT-1U))
pub fn LWIP_MEM_ALIGN_SIZE(size: usize) -> usize {
    (size + MEM_ALIGNMENT - 1) & !(MEM_ALIGNMENT - 1)
}


/* Calculate safe memory size for an aligned buffer when using an unaligned
 * type as storage. This includes a safety-margin on (MEM_ALIGNMENT - 1) at the
 * start (e.g. if buffer is u8[] and actual data will be u32*)
 */

// TODO // #define LWIP_MEM_ALIGN_BUFFER(size) (((size) + MEM_ALIGNMENT - 1U))


/* Align a memory pointer to the alignment defined by MEM_ALIGNMENT
 * so that ADDR % MEM_ALIGNMENT == 0
 */

// TODO // #define LWIP_MEM_ALIGN(addr) ((void *)(((mem_ptr_t)(addr) + MEM_ALIGNMENT - 1) & ~(mem_ptr_t)(MEM_ALIGNMENT-1)))



//


/* Packed structs support.
  * Placed BEFORE declaration of a packed struct.\n
  * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
  * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
  */

// #define


/* Packed structs support.
  * Placed AFTER declaration of a packed struct.\n
  * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
  * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
  */

// #define


/* Packed structs support.
  * Placed between end of declaration of a packed struct and trailing semicolon.\n
  * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
  * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
  */


// #define  __attribute__((packed))
// #else
// #define



/* Packed structs support.
  * Wraps u32 and members: u16.\n
  * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
  * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
  */

// TODO #define (x) x


/* Packed structs support.
  * Wraps members: u8, where some compilers warn that packing is not necessary.\n
  * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
  * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
  */

// TODO #define (x) (x)


/* Packed structs support.
  * Wraps members that are packed structs themselves, where some compilers warn that packing is not necessary.\n
  * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
  * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
  */

// TODO #define (x) (x)


/* PACK_STRUCT_USE_INCLUDES==1: Packed structs support using \#include files before and after struct to be packed.\n
 * The file included BEFORE the struct is "arch/bpstruct.h".\n
 * The file included AFTER the struct is "arch/epstruct.h".\n
 * This can be used to implement struct packing on MS Visual C compilers, see
 * the Win32 port in the lwIP contrib repository for reference.
 * For examples of packed struct declarations, see include/lwip/prot/ subfolder.\n
 * A port to GCC/clang is included in lwIP, if you use these compilers there is nothing to do here.
 */

// #define PACK_STRUCT_USE_INCLUDES

/* Eliminates compiler warning about unused arguments (GCC -Wextra -Wunused). */

// // #define LWIP_UNUSED_ARG(x) ()x


/* LWIP_PROVIDE_ERRNO==1: Let lwIP provide ERRNO values and the 'errno' variable.
 * If this is disabled, cc.h must either define 'errno', include <errno.h>,
 * define LWIP_ERRNO_STDINCLUDE to get <errno.h> included or
 * define LWIP_ERRNO_INCLUDE to <errno.h> or equivalent.
 */

// // #define LWIP_PROVIDE_ERRNO


/*
 * @}
 */


// }



