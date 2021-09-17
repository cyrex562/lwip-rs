/*
 * ccp.h - Definitions for PPP Compression Control Protocol.
 *
 * Copyright (c) 1994-2002 Paul Mackerras. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * 3. Redistributions of any form whatsoever must retain the following
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
 *
 * $Id: ccp.h,v 1.12 2004/11/04 10:02:26 paulus Exp $
 */

/*
 * CCP codes.
 */

pub const CCP_CONFREQ: u32 = 1; 
pub const CCP_CONFACK: u32 = 2; 
pub const CCP_TERMREQ: u32 = 5; 
pub const CCP_TERMACK: u32 = 6; 
pub const CCP_RESETREQ: u32 = 14; 
pub const CCP_RESETACK: u32 = 15; 

/*
 * Max # bytes for a CCP option
 */

pub const CCP_MAX_OPTION_LENGTH: u32 = 32; 

/*
 * Parts of a CCP packet.
 */

#define CCP_CODE(dp)		((dp)[0])
#define CCP_ID(dp)		((dp)[1])
#define CCP_LENGTH(dp)		(((dp)[2] << 8) + (dp)[3])
pub const CCP_HDRLEN: u32 = 4; 

#define CCP_OPT_CODE(dp)	((dp)[0])
#define CCP_OPT_LENGTH(dp)	((dp)[1])
pub const CCP_OPT_MINLEN: u32 = 2; 


/*
 * Definitions for BSD-Compress.
 */

pub const CI_BSD_COMPRESS: u32 = 21; 	/* config. option for BSD-Compress */
pub const CI_BSD_COMPRESS: u32 = 21; 
pub const CILEN_BSD_COMPRESS: u32 = 3;	/* length of config. option */

/* Macros for handling the 3rd byte of the BSD-Compress config option. */
#define BSD_NBITS(x)		((x) & 0x1F)	/* number of bits requested */
#define BSD_VERSION(x)		((x) >> 5)	/* version of option format */
pub const BSD_CURRENT_VERSION: u32 = 1; 		/* current version number */
#define BSD_MAKE_OPT(v, n)	(((v) << 5) | (n))

pub const BSD_MIN_BITS: u32 = 9; 	/* smallest code size supported */
pub const BSD_MIN_BITS: u32 = 9; 
pub const BSD_MAX_BITS: u32 = 15;	/* largest code size supported */



/*
 * Definitions for Deflate.
 */

pub const CI_DEFLATE: u32 = 26; 	/* config option for Deflate */
pub const CI_DEFLATE: u32 = 26; 
pub const CI_DEFLATE: u32 = 26; 
pub const CI_DEFLATE_DRAFT: u32 = 2; 
pub const CILEN_DEFLATE: u32 = 4; 
pub const CI_DEFLATE: u32 = 26; 
pub const DEFLATE_MIN_SIZE: u32 = 9;
pub const DEFLATE_MAX_SIZE: u32 = 15;
pub const DEFLATE_METHOD_VAL: u32 = 8;
#define DEFLATE_SIZE(x)		(((x) >> 4) + 8)
#define DEFLATE_METHOD(x)	((x) & 0x0F)
#define DEFLATE_MAKE_OPT(w)	((((w) - 8) << 4) + DEFLATE_METHOD_VAL)
pub const DEFLATE_CHK_SEQUENCE: u32 = 0;



/*
 * Definitions for MPPE.
 */

pub const CI_MPPE: u32 = 18;       /* config option for MPPE */
pub const CI_MPPE: u32 = 18; 
pub const CILEN_MPPE: u32 = 6;      /* length of config option */



/*
 * Definitions for other, as yet unsupported, compression methods.
 */

pub const CI_PREDICTOR_1: u32 = 1; 	/* config option for Predictor-1 */
pub const CI_PREDICTOR_1: u32 = 1; 
pub const CI_PREDICTOR_1: u32 = 1; 
pub const CI_PREDICTOR_1: u32 = 1; 
pub const CILEN_PREDICTOR_1: u32 = 2;	/* length of its config option */
pub const CI_PREDICTOR_2: u32 = 2;	/* config option for Predictor-2 */
pub const CILEN_PREDICTOR_2: u32 = 2;	/* length of its config option */


typedef struct ccp_options {

     deflate: i32          :1; /* do Deflate? */
     deflate_correct: i32  :1; /* use correct code for deflate? */
     deflate_draft: i32    :1; /* use draft RFC code for deflate? */


     bsd_compress: i32     :1; /* do BSD Compress? */


     predictor_1: i32      :1; /* do Predictor-1? */
     predictor_2: i32      :1; /* do Predictor-2? */



    let mppe: u8;			/* MPPE bitfield */


    u_short bsd_bits;		/* # bits/code for BSD Compress */


    u_short deflate_size;	/* lg(window size) for Deflate */

    let method: u8;		/* code for chosen compression method */
} ccp_options;

extern const struct protent ccp_protent;

pub fn  ccp_resetrequest(pcb: &mut ppp_pcb);  /* Issue a reset-request. */


}




