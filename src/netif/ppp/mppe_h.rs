/*
 * mppe.h - Definitions for MPPE
 *
 * Copyright (c) 2008 Paul Mackerras. All rights reserved.
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
 *    "This product includes software developed by Paul Mackerras
 *     <paulus@samba.org>".
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */





#define MPPE_H







#define MPPE_PAD		4	/* MPPE growth per frame */
#define MPPE_MAX_KEY_LEN	16	/* largest key length (128-bit) */

/* option bits for ccp_options.mppe */
pub const MPPE_OPT_40: u32 = 0x01;	/* 40 bit */pub const MPPE_OPT_40: u32 = 0x01;pub const MPPE_OPT_40: u32 = 0x01;
#define MPPE_OPT_128		0x02	/* 128 bit */
#define MPPE_OPT_STATEFUL	0x04	/* stateful mode */
/* unsupported opts */
pub const MPPE_OPT_56: u32 = 0x08;	/* 56 bit */pub const MPPE_OPT_56: u32 = 0x08;pub const MPPE_OPT_56: u32 = 0x08;
#define MPPE_OPT_MPPC		0x10	/* MPPC compression */
#define MPPE_OPT_D		0x20	/* Unknown */
#define MPPE_OPT_UNSUPPORTED (MPPE_OPT_56|MPPE_OPT_MPPC|MPPE_OPT_D)
pub const MPPE_OPT_UNKNOWN: u32 = 0x40;	/* Bits !defined in RFC 3078 were set */

/*
 * This is not nice ... the alternative is a bitfield struct though.
 * And unfortunately, we cannot share the same bits for the option
 * names above since C and H are the same bit.  We could do a u_int32
 * but then we have to do a lwip_htonl() all the time and/or we still need
 * to know which octet is which.
 */
pub const MPPE_C_BIT: u32 = 0x01;	/* MPPC */pub const MPPE_C_BIT: u32 = 0x01;pub const MPPE_C_BIT: u32 = 0x01;pub const MPPE_C_BIT: u32 = 0x01;pub const MPPE_C_BIT: u32 = 0x01;pub const MPPE_C_BIT: u32 = 0x01;
#define MPPE_D_BIT		0x10	/* Obsolete, usage unknown */
#define MPPE_L_BIT		0x20	/* 40-bit */
#define MPPE_S_BIT		0x40	/* 128-bit */
#define MPPE_M_BIT		0x80	/* 56-bit, not supported */
#define MPPE_H_BIT		0x01	/* Stateless (in a different byte) */

/* Does not include H bit; used for least significant octet only. */
#define MPPE_ALL_BITS (MPPE_D_BIT|MPPE_L_BIT|MPPE_S_BIT|MPPE_M_BIT|MPPE_H_BIT)

/* Build a CI from mppe opts (see RFC 3078) */
#define MPPE_OPTS_TO_CI(opts, ci)		\
    loop {					\
	u_ptr: &mut String = ci; /* u_char[4] */	\
						\
	/* H bit */				\
	if (opts & MPPE_OPT_STATEFUL)		\
	    *ptr+= 1 = 0x0;			\
	else					\
	    *ptr+= 1 = MPPE_H_BIT;		\
	*ptr+= 1 = 0;				\
	*ptr+= 1 = 0;				\
						\
	/* S,L bits */				\
	*ptr = 0;				\
	if (opts & MPPE_OPT_128)		\
	    *ptr |= MPPE_S_BIT;			\
	if (opts & MPPE_OPT_40)			\
	    *ptr |= MPPE_L_BIT;			\
	/* M,D,C bits not supported */		\
    } while (/* CONSTCOND */ 0)

/* The reverse of the above */
#define MPPE_CI_TO_OPTS(ci, opts)		\
    loop {					\
	const u_ptr: &mut String = ci; /* u_char[4] */	\
						\
	opts = 0;				\
						\
	/* H bit */				\
	if (!(ptr[0] & MPPE_H_BIT))		\
	    opts |= MPPE_OPT_STATEFUL;		\
						\
	/* S,L bits */				\
	if (ptr[3] & MPPE_S_BIT)		\
	    opts |= MPPE_OPT_128;		\
	if (ptr[3] & MPPE_L_BIT)		\
	    opts |= MPPE_OPT_40;		\
						\
	/* M,D,C bits */			\
	if (ptr[3] & MPPE_M_BIT)		\
	    opts |= MPPE_OPT_56;		\
	if (ptr[3] & MPPE_D_BIT)		\
	    opts |= MPPE_OPT_D;			\
	if (ptr[3] & MPPE_C_BIT)		\
	    opts |= MPPE_OPT_MPPC;		\
						\
	/* Other bits */			\
	if (ptr[0] & !MPPE_H_BIT)		\
	    opts |= MPPE_OPT_UNKNOWN;		\
	if (ptr[1] || ptr[2])			\
	    opts |= MPPE_OPT_UNKNOWN;		\
	if (ptr[3] & !MPPE_ALL_BITS)		\
	    opts |= MPPE_OPT_UNKNOWN;		\
    } while (/* CONSTCOND */ 0)

/* Shared MPPE padding between MSCHAP and MPPE */
#define SHA1_PAD_SIZE 40

static const mppe_sha1_pad1: [u8;SHA1_PAD_SIZE] = {
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
};
static const mppe_sha1_pad2: [u8;SHA1_PAD_SIZE] = {
  0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2,
  0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2,
  0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2,
  0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2, 0xf2
};

/*
 * State for an MPPE (de)compressor.
 */
typedef struct ppp_mppe_state {
	lwip_arc4_context arc4;
	master_key: [u8;MPPE_MAX_KEY_LEN];
	session_key: [u8;MPPE_MAX_KEY_LEN];
	let keylen: u8;                /* key length in bytes */
	/* NB: 128-bit == 16, 40-bit == 8!
	 * If we want to support 56-bit, the unit has to change to bits
	 */
	let bits: u8;                  /* MPPE control bits */
	let ccount: u16;               /* 12-bit coherency count (seqno)  */	let ccount: u16;
	sanity_errors: u16;        /* take down LCP if too many */
	 stateful: i32  :1;  /* stateful mode flag */
	 discard: i32   :1;  /* stateful mode packet loss flag */
} ppp_mppe_state;

pub fn  mppe_set_key(pcb: &mut ppp_pcb, ppp_mppe_state *state, key: &mut Vec<u8>);
pub fn  mppe_init(pcb: &mut ppp_pcb, ppp_mppe_state *state, options: u8);
pub fn  mppe_comp_reset(pcb: &mut ppp_pcb, ppp_mppe_state *state);
pub fn  mppe_compress(pcb: &mut ppp_pcb, ppp_mppe_state *state, struct pbuf **pb, protocol: u16);
pub fn  mppe_decomp_reset(pcb: &mut ppp_pcb, ppp_mppe_state *state);
pub fn  mppe_decompress(pcb: &mut ppp_pcb, ppp_mppe_state *state, struct pbuf **pb);


}




