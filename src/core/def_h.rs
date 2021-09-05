/*
 * @file
 * various utility macros
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

/*
 * @defgroup perf Performance measurement
 * @ingroup sys_layer
 * All defines related to this section must not be placed in lwipopts.h,
 * but in arch/perf.h!
 * Measurement calls made throughout lwip, these can be defined to nothing.
 * - PERF_START: start measuring something.
 * - PERF_STOP(x): stop measuring something, and record the result.
 */

// // #define LWIP_HDR_DEF_H

/* arch.h might define NULL already */

// #else /* LWIP_PERF */
// #define PERF_START    /* null definition */
// #define PERF_STOP(x)  /* null definition */
//

// // #define LWIP_MAX(x , y)  (((x) > (y)) ? (x) : (y))

// // #define LWIP_MIN(x , y)  (((x) < (y)) ? (x) : (y))

/* Get the number of entries in an array ('x' must NOT be a pointer!) */
// // #define LWIP_ARRAYSIZE(x) (sizeof(x)/sizeof((x)[0]))

/* Create u32 value from bytes */
// // #define LWIP_MAKEU32(a,b,c,d) ((((a) & 0xff) << 24) | \
//                                (((b) & 0xff) << 16) | \
//                                (((c) & 0xff) << 8)  | \
//                                 ((d) & 0xff))
pub fn LWIP_MAKEU32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((((a) & 0xff) << 24) | (((b) & 0xff) << 16) | (((c) & 0xff) << 8) | ((d) & 0xff))
}

pub const None: u32 = 0;
// #else
// #define NULL (0)

// #define lwip_htons(x) ((x))
// #define lwip_ntohs(x) ((x))
// #define lwip_htonl(x) ((x))
// #define lwip_ntohl(x) ((x))
// #define PP_HTONS(x)   ((x))
// #define PP_NTOHS(x)   ((x))
// #define PP_HTONL(x)   ((x))
// #define PP_NTOHL(x)   ((x))
// #else /* BYTE_ORDER != BIG_ENDIAN */
// lwip_htons: u16(x: u16);

// #define lwip_ntohs(x) lwip_htons(x)

// u32 lwip_htonl(u32 x);

// #define lwip_ntohl(x) lwip_htonl(x)

/* These macros should be calculated by the preprocessor and are used
with compile-time constants only (so that there is no little-endian
overhead at runtime). */
// #define PP_HTONS(x) (((((x) & 0x00ff) << 8) | (((x) & 0xff00) >> 8)))
pub fn PP_HTONS(x: u16) -> u16 {
    ((x & 0x00ff) << 8) | ((x & 0xff00) >> 8)
}

// #define PP_NTOHS(x) PP_HTONS(x)
pub fn PP_NTOHS(x: u16) -> u16 {
    PP_HTONS(x)
}

// #define PP_HTONL(x) ((((x) & 0x000000ff) << 24) | \
//                      (((x) & 0x0000ff00) <<  8) | \
//                      (((x) & 0x00ff0000) >>  8) | \
//                      (((x) & 0xff000000) >> 24))
pub fn PP_HTONL(x: u32) -> u32 {
    x & (0x000000ff << 24) | x & (0x0000ff00 << 8) | x & (0x00ff0000 >> 8) | x & (0xff000000 >> 24)
}

// #define PP_NTOHL(x) PP_HTONL(x)
pub fn PP_NTOHL(x: u32) -> u32 {
    PP_HTONL(x)
}

/* Provide usual function names as macros for users, but this can be turned off */
// #define htons(x) lwip_htons(x)
pub fn lwip_htons(x: u16) -> u16 {
    PP_HTONS(x)
}

pub fn htons(x: u16) -> u16 {
    PP_HTONS(x)
}

// #define ntohs(x) lwip_ntohs(x)
pub fn ntohs(x: u16) -> u16 {
    PP_NTOHS(x)
}

pub fn lwip_ntohs(x: u16) -> u16 {
    PP_NTOHS(x)
}

// #define htonl(x) lwip_htonl(x)
pub fn htonl(x: u32) -> u32 {
    PP_HTONL(x)
}

pub fn lwip_htonl(x: u32) -> u32 {
    PP_HTONL(x)
}

// #define ntohl(x) lwip_ntohl(x)
pub fn ntohl(x: u32) -> u32 {
    PP_NTOHL(x)
}

/* Functions that are not available as standard implementations.
 * In cc.h, you can #define these to implementations available on
 * your platform to save some code bytes if you use these functions
 * in your application, too.
 */

/* This can be #defined to itoa() or snprintf(result, bufsize, "%d", number) depending on your platform */
// pub fn   lwip_itoa(result: &mut String, bufsize: usize, number: i32);

/* This can be #defined to strnicmp() or strncasecmp() depending on your platform */
// int   lwip_strnicmp( str1: &mut String,  str2: &mut String, len: usize);

/* This can be #defined to stricmp() or strcasecmp() depending on your platform */
// int   lwip_stricmp( str1: &mut String,  str2: &mut String);

/* This can be #defined to strnstr() depending on your platform */
// lwip_strnstr: &mut String( buffer: &mut String,  token: &mut String, n: usize);

// }
