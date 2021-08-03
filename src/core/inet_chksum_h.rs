/*
 * @file
 * IP checksum calculation functions
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

// #define LWIP_HDR_INET_CHKSUM_H






/* Swap the bytes in an u16: much like lwip_htons() for little-endian */

#define SWAP_BYTES_IN_WORD(w) (((w) & 0xff) << 8) | (((w) & 0xff00) >> 8)


/* Split an u32 in two u16s and add them up */

#define FOLD_U32T(u)          ((u32)(((u) >> 16) + ((u) & 0x0000ffffUL)))



/* Function-like macro: same as MEMCPY but returns the checksum of copied data
    as u16 */
# ifndef LWIP_CHKSUM_COPY
#  define LWIP_CHKSUM_COPY(dst, src, len) lwip_chksum_copy(dst, src, len)
#  ifndef LWIP_CHKSUM_COPY_ALGORITHM
#   define LWIP_CHKSUM_COPY_ALGORITHM 1
#  endif /* LWIP_CHKSUM_COPY_ALGORITHM */
# else /* LWIP_CHKSUM_COPY */
#  define LWIP_CHKSUM_COPY_ALGORITHM 0
# endif /* LWIP_CHKSUM_COPY */
#else /* LWIP_CHECKSUM_ON_COPY */
# define LWIP_CHKSUM_COPY_ALGORITHM 0






inet_chksum: u16(dataptr: &Vec<u8>, len: u16);
inet_chksum_pbuf: u16(p: &mut pbuf);

lwip_chksum_copy: u16(void *dst, src: &Vec<u8>, len: u16);



inet_chksum_pseudo: u16(p: &mut pbuf, proto: u8, proto_len: u16,
       const src: &mut ip4_addr, const dest: &mut ip4_addr);
inet_chksum_pseudo_partial: u16(p: &mut pbuf, proto: u8,
       proto_len: u16, chksum_len: u16, const src: &mut ip4_addr, const dest: &mut ip4_addr);



ip6_chksum_pseudo: u16(p: &mut pbuf, proto: u8, proto_len: u16,
       const src: &mut ip6_addr_t, const dest: &mut ip6_addr_t);
ip6_chksum_pseudo_partial: u16(p: &mut pbuf, proto: u8, proto_len: u16,
       chksum_len: u16, const src: &mut ip6_addr_t, const dest: &mut ip6_addr_t);



ip_chksum_pseudo: u16(p: &mut pbuf, proto: u8, proto_len: u16,
       const src: &mut ip_addr_t, const dest: &mut ip_addr_t);
ip_chksum_pseudo_partial: u16(p: &mut pbuf, proto: u8, proto_len: u16,
       chksum_len: u16, const src: &mut ip_addr_t, const dest: &mut ip_addr_t);


}




