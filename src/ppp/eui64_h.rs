/*
 * eui64.h - EUI64 routines for IPv6CP.
 *
 * Copyright (c) 1999 Tommi Komulainen.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * 4. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by Tommi Komulainen
 *     <Tommi.Komulainen@iki.fi>".
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * $Id: eui64.h,v 1.6 2002/12/04 23:03:32 paulus Exp $
*/

/*
 * @todo:
 *
 * Maybe this should be done by processing struct in6_addr directly...
 */

#[derive(PartialEq, Clone)]
pub struct eui64_t {
    pub e8: [u8; 8],
}

impl eui64_t {
    pub fn new() -> eui64_t {
        eui64_t { e8: [0; 8] }
    }

    pub fn e16(&self) -> [u16; 4] {
        unimplemented!()
    }

    pub fn e32(&self) -> [u32; 2] {
        unimplemented!()
    }

    pub fn iszero(&self) -> bool {
        unimplemented!()
    }

    pub fn zero(&self) {
        unimplemented!()
    }

    pub fn magic(&self) {
        // e.e32[0] = magic();	\
        // e.e32[1] = magic();	\
        // e.e8[0] &= !2;	\
        unimplemented!()
    }

    pub fn magic_nz(&self, x: i32) -> bool {
        unimplemented!()
    }

    pub fn magic_ne(&self, x: i32) -> bool {
        unimplemented!()
    }

    pub fn get(&self, ll: u64, cp: u64) {
        unimplemented!()
        /*
        eui64_copy((*cp), (ll));	\
                (cp) += sizeof(eui64_t);	\
        */
    }

    pub fn put(&self, ll: u64, cp: u64) {
        unimplemented!()
        /*
        eui64_copy((ll), (*cp));	\
                (cp) += sizeof(eui64_t);	\
        */
    }

    pub fn set32(&self, l: u32) {
        unimplemented!()
        /*
        e.e32[0] = 0;		\
                e.e32[1] = lwip_htonl(l);	\
        */
    }

    pub fn setlo32(&self, l: u32) {
        self::set32(l)
    }

    pub fn ntoa(&self) -> String {
        unimplemented!()
    }
}
