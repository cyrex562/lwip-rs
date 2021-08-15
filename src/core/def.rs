/*
* @file
* Common functions used throughout the stack.
*
* These are reference implementations of the byte swapping functions.
* Again with the aim of being simple, correct and fully portable.
* Byte swapping is the second thing you would want to optimize. You will
* need to port it to your architecture and in your cc.h:
*
* \#define lwip_htons(x) your_htons
* \#define lwip_htonl(x) your_htonl
*
* Note lwip_ntohs() and lwip_ntohl() are merely references to the htonx counterparts.
*
* If you \#define them to htons() and htonl(), you should
* \// #define LWIP_DONT_PROVIDE_BYTEORDER_FUNCTIONS to prevent lwIP from
* defining htonx/ntohx compatibility macros.

* @defgroup sys_nonstandard Non-standard functions
* @ingroup sys_layer
* lwIP provides default implementations for non-standard functions.
* These can be mapped to OS functions to reduce code footprif: i32 desired.
* All defines related to this section must not be placed in lwipopts.h,
* but in arch/cc.h!
* These options cannot be \#defined in lwipopts.h since they are not options
* of lwIP itself, but options of the lwIP port to your system.
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
 * Author: Simon Goldschmidt
 *
 */

/*
 * Convert an from: u16 host- to network byte order.
 *
 * @param n in: u16 host byte order
 * @return n in network byte order
 */
pub fn lwip_htons(n: u16) {
    return PP_HTONS(n);
}

/*
 * Convert an u32 from host- to network byte order.
 *
 * @param n u32 in host byte order
 * @return n in network byte order
 */
pub fn lwip_htonl(n: u32) -> u32 {
    return PP_HTONL(n);
}

/*
 * @ingroup sys_nonstandard
 * lwIP default implementation for strnstr() non-standard function.
 * This can be \#defined to strnstr() depending on your platform port.
 */
pub fn lwip_strnstr(buffer: &String, token: &String, n: usize) -> Option<&String> {
    let p: String;
    let tokenlen = token.len();
    if (tokenlen == 0) {
        return buffer;
    }
    // for (p = buffer; *p && (p + tokenlen <= buffer + n); p+= 1) {
    //   if ((*p == *token) && (strncmp(p, token, tokenlen) == 0)) {
    //     return LWIP_CONST_CAST(char *, p);
    //   }
    // }
    return None;
}

/*
 * @ingroup sys_nonstandard
 * lwIP default implementation for stricmp() non-standard function.
 * This can be \#defined to stricmp() depending on your platform port.
 */
pub fn lwip_stricmp(str1: &String, str2: &String) -> bool {
    // char c1, c2;

    // loop {
    //   c1 = *str1+= 1;
    //   c2 = *str2+= 1;
    //   if (c1 != c2) {
    //     char c1_upc = c1 | 0x20;
    //     if ((c1_upc >= 'a') && (c1_upc <= 'z')) {
    //       /* characters are not equal an one is in the alphabet range:
    //       downcase both chars and check again */
    //       char c2_upc = c2 | 0x20;
    //       if (c1_upc != c2_upc) {
    //         /* still not equal */
    //         /* don't care for < or > */
    //         return 1;
    //       }
    //     } else {
    //       /* characters are not equal but none is in the alphabet range */
    //       return 1;
    //     }
    //   }
    // } while (c1 != 0);
    // return 0;
    str1 == str2
}

/*
 * @ingroup sys_nonstandard
 * lwIP default implementation for strnicmp() non-standard function.
 * This can be \#defined to strnicmp() depending on your platform port.
 */
pub fn lwip_strnicmp(str1: &String, str2: &String, len: usize) -> bool {
    if len > str1.len() || len > str2.len() {
        return false;
    }
    // char c1, c2;

    // loop {
    //   c1 = *str1+= 1;
    //   c2 = *str2+= 1;
    //   if (c1 != c2) {
    //     char c1_upc = c1 | 0x20;
    //     if ((c1_upc >= 'a') && (c1_upc <= 'z')) {
    //       /* characters are not equal an one is in the alphabet range:
    //       downcase both chars and check again */
    //       char c2_upc = c2 | 0x20;
    //       if (c1_upc != c2_upc) {
    //         /* still not equal */
    //         /* don't care for < or > */
    //         return 1;
    //       }
    //     } else {
    //       /* characters are not equal but none is in the alphabet range */
    //       return 1;
    //     }
    //   }
    //   len -= 1;
    // } while ((len != 0) && (c1 != 0));
    // return 0;
    str1[..len] == str2[..len]
}

/*
 * @ingroup sys_nonstandard
 * lwIP default implementation for itoa() non-standard function.
 * This can be \#defined to itoa() or snprintf(result, bufsize, "%d", number) depending on your platform port.
 */
pub fn lwip_itoa(result: &mut String, bufsize: usize, number: i32) {
    // res: &mut String = result;
    // tmp: &mut String = result + bufsize - 1;
    // n: i32 = (number >= 0) ? number : -number;

    // /* handle invalid bufsize */
    // if (bufsize < 2) {
    //   if (bufsize == 1) {
    //     *result = 0;
    //   }
    //   return;
    // }

    // /* First, add sign */
    // if (number < 0) {
    //   *res+= 1 = '-';
    // }
    // /* Then create the string from the end and stop if buffer full,
    //    and ensure output string is zero terminated */
    // *tmp = 0;
    // while ((n != 0) && (tmp > res)) {
    //   char val = (char)('0' + (n % 10));
    //   tmp -= 1;
    //   *tmp = val;
    //   n = n / 10;
    // }
    // if (n) {
    //   /* buffer is too small */
    //   *result = 0;
    //   return;
    // }
    // if (*tmp == 0) {
    //   /* Nothing added? */
    //   *res+= 1 = '0';
    //   *res+= 1 = 0;
    //   return;
    // }
    // /* move from temporary buffer to output buffer (sign is not moved) */
    // memmove(res, tmp, ((result + bufsize) - tmp));
    *result = number.to_string()
}
