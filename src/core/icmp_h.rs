/**
 * @file
 * ICMP protocol definitions
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

#define LWIP_HDR_PROT_ICMP_H




extern "C" {


#define ICMP_ER   0    /* echo reply */
#define ICMP_DUR  3    /* destination unreachable */
#define ICMP_SQ   4    /* source quench */
#define ICMP_RD   5    /* redirect */
#define ICMP_ECHO 8    /* echo */
#define ICMP_TE  11    /* time exceeded */
#define ICMP_PP  12    /* parameter problem */
#define ICMP_TS  13    /* timestamp */
#define ICMP_TSR 14    /* timestamp reply */
#define ICMP_IRQ 15    /* information request */
#define ICMP_IR  16    /* information reply */
#define ICMP_AM  17    /* address mask request */
#define ICMP_AMR 18    /* address mask reply */


#  include "arch/bpstruct.h"

/** This is the standard ICMP header only that the u32 data
 *  is split to two like: u16 ICMP echo needs it.
 *  This header is also used for other ICMP types that do not
 *  use the data part.
 */
PACK_STRUCT_BEGIN
struct icmp_echo_hdr {
  PACK_STRUCT_FLD_8(type: u8);
  PACK_STRUCT_FLD_8(code: u8);
  PACK_STRUCT_FIELD(chksum: u16);
  PACK_STRUCT_FIELD(id: u16);
  PACK_STRUCT_FIELD(seqno: u16);
} PACK_STRUCT_STRUCT;
PACK_STRUCT_END

#  include "arch/epstruct.h"


/* Compatibility defines, old versions used to combine type and code to an u16 */
#define ICMPH_TYPE(hdr) ((hdr)->type)
#define ICMPH_CODE(hdr) ((hdr)->code)
#define ICMPH_TYPE_SET(hdr, t) ((hdr)->type = (t))
#define ICMPH_CODE_SET(hdr, c) ((hdr)->code = (c))


}



