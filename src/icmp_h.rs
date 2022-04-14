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



// #include "lwip/arch.h"




pub const ICMP_ER: u32 = 0; /* echo reply */
pub const ICMP_DUR: u32 = 3; /* destination unreachable */
pub const ICMP_SQ: u32 = 4; /* source quench */
pub const ICMP_RD: u32 = 5; /* redirect */
pub const ICMP_ECHO: u32 = 8; /* echo */
pub const ICMP_TE: u32 = 11; /* time exceeded */
pub const ICMP_PP: u32 = 12; /* parameter problem */
pub const ICMP_TS: u32 = 13; /* timestamp */
pub const ICMP_TSR: u32 = 14; /* timestamp reply */
pub const ICMP_IRQ: u32 = 15; /* information request */
pub const ICMP_IR: u32 = 16; /* information reply */
pub const ICMP_AM: u32 = 17; /* address mask request */
pub const ICMP_AMR: u32 = 18; /* address mask reply */

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

e standard ICMP header (unspecified 32 bit data) */

struct icmp_hdr {
  PACK_STRUCT_FLD_8(u8_t type);
  PACK_STRUCT_FLD_8(u8_t code);
  PACK_STRUCT_FIELD(u16_t chksum);
  PACK_STRUCT_FIELD(u32_t data);
} PACK_STRUCT_STRUCT;

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"


/* Compatibility defines, old versions used to combine type and code to an u16_t */
#define ICMPH_TYPE(hdr) ((hdr)->type)
#define ICMPH_CODE(hdr) ((hdr)->code)
#define ICMPH_TYPE_SET(hdr, t) ((hdr)->type = (t))
#define ICMPH_CODE_SET(hdr, c) ((hdr)->code = (c))

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

is is the standard ICMP header only that the u32_t data
 *  is split to two u16_t like ICMP echo needs it.
 */

struct icmp_echo_hdr {
  PACK_STRUCT_FLD_8(u8_t type);
  PACK_STRUCT_FLD_8(u8_t code);
  PACK_STRUCT_FIELD(u16_t chksum);
  PACK_STRUCT_FIELD(u16_t id);
  PACK_STRUCT_FIELD(u16_t seqno);
} PACK_STRUCT_STRUCT;

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"





 /* LWIP_HDR_PROT_ICMP_H */
