/*****************************************************************************
* pppoe.h - PPP Over Ethernet implementation for lwIP.
*
* Copyright (c) 2006 by Marc Boucher, Services Informatiques (MBSI) inc.
*
* The authors hereby grant permission to use, copy, modify, distribute,
* and license this software and its documentation for any purpose, provided
* that existing copyright notices are retained in all copies and that this
* notice and the following disclaimer are included verbatim in any 
* distributions. No written agreement, license, or royalty fee is required
* for any of the authorized uses.
*
* THIS SOFTWARE IS PROVIDED BY THE CONTRIBUTORS *AS IS* AND ANY EXPRESS OR
* IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
* OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. 
* IN NO EVENT SHALL THE CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
* INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
* NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
* DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
* THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
* (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
* THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*
******************************************************************************
* REVISION HISTORY
*
* 06-01-01 Marc Boucher <marc@mbsi.ca>
*   Ported to lwIP.
*****************************************************************************/



/* based on NetBSD: if_pppoe.c,v 1.64 2006/01/31 23:50:15 martin Exp */

/*-
 * Copyright (c) 2002 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Martin Husemann <martin@NetBSD.org>.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *        This product includes software developed by the NetBSD
 *        Foundation, Inc. and its contributors.
 * 4. Neither the name of The NetBSD Foundation nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
// #include "netif/ppp/ppp_opts.h"
// #if PPP_SUPPORT && PPPOE_SUPPORT /* don't build if not configured for use in lwipopts.h */




// #include "ppp.h"
// #include "lwip/etharp.h"




#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
struct pppoehdr {
  PACK_STRUCT_FLD_8(u8_t vertype);
  PACK_STRUCT_FLD_8(u8_t code);
  PACK_STRUCT_FIELD(u16_t session);
  PACK_STRUCT_FIELD(u16_t plen);
} PACK_STRUCT_STRUCT;

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"


#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/bpstruct.h"

TRUCT_BEGIN
struct pppoetag {
  PACK_STRUCT_FIELD(u16_t tag);
  PACK_STRUCT_FIELD(u16_t len);
} PACK_STRUCT_STRUCT;

#ifdef PACK_STRUCT_USE_INCLUDES
#  include "arch/epstruct.h"



pub const PPPOE_STATE_INITIAL: u32 = 0;
pub const PPPOE_STATE_PADI_SENT: u32 = 1;
pub const PPPOE_STATE_PADR_SENT: u32 = 2;
pub const PPPOE_STATE_SESSION: u32 = 3;
/* passive */
pub const PPPOE_STATE_PADO_SENT: u32 = 1;

pub const PPPOE_HEADERLEN: u32 = sizeof;(struct pppoehdr)
pub const PPPOE_VERTYPE: u32 = 0x11;    /* VER=1, TYPE = 1 */

pub const PPPOE_TAG_EOL: u32 = 0x0000;  /* end of list */
pub const PPPOE_TAG_SNAME: u32 = 0x0101;  /* service name */
pub const PPPOE_TAG_ACNAME: u32 = 0x0102;  /* access concentrator name */
pub const PPPOE_TAG_HUNIQUE: u32 = 0x0103;  /* host unique */
pub const PPPOE_TAG_ACCOOKIE: u32 = 0x0104;  /* AC cookie */
pub const PPPOE_TAG_VENDOR: u32 = 0x0105;  /* vendor specific */
pub const PPPOE_TAG_RELAYSID: u32 = 0x0110;  /* relay session id */
pub const PPPOE_TAG_SNAME_ERR: u32 = 0x0201;  /* service name error */
pub const PPPOE_TAG_ACSYS_ERR: u32 = 0x0202;  /* AC system error */
pub const PPPOE_TAG_GENERIC_ERR: u32 = 0x0203;  /* gerneric error */

pub const PPPOE_CODE_PADI: u32 = 0x09;    /* Active Discovery Initiation */
pub const PPPOE_CODE_PADO: u32 = 0x07;    /* Active Discovery Offer */
pub const PPPOE_CODE_PADR: u32 = 0x19;    /* Active Discovery Request */
pub const PPPOE_CODE_PADS: u32 = 0x65;    /* Active Discovery Session confirmation */
pub const PPPOE_CODE_PADT: u32 = 0xA7;    /* Active Discovery Terminate */


pub const PPPOE_MAX_AC_COOKIE_LEN: u32 = 64;


struct pppoe_softc {
  struct pppoe_softc *next;
  struct netif *sc_ethif;      /* ethernet interface we are using */
  ppp_pcb *pcb;                /* PPP PCB */

  struct eth_addr sc_dest;     /* hardware address of concentrator */
  sc_session: u16;            /* PPPoE session id */
  sc_state: u8;               /* discovery phase or session connected */

// #if PPPOE_SCNAME_SUPPORT
  const char *sc_service_name;      /* if != NULL: requested name of service */
  const char *sc_concentrator_name; /* if != NULL: requested concentrator id */
 /* PPPOE_SCNAME_SUPPORT */
 sc_ac_cookie[PPPOE_MAX_AC_COOKIE_LEN]; /* content of AC cookie we must echo back */
  sc_ac_cookie_len: u8;       /* length of cookie data */
#ifdef PPPOE_SERVER
  u8_t *sc_hunique;            /* content of host unique we must echo back */
  sc_hunique_len: u8;         /* length of host unique */

 sc_padi_retried;        /* number of PADI retries already done */
  sc_padr_retried: u8;        /* number of PADR retries already done */
};


#define pppoe_init() /* compatibility define, no initialization needed */

ppp_pcb *pppoe_create(struct netif *pppif,
       struct netif *ethif,
       const char *service_name, const char *concentrator_name,
       ppp_link_status_cb_fn link_status_cb, void *ctx_cb);

/*
 * Functions called from lwIP
 * DO NOT CALL FROM lwIP USER APPLICATION.
 */
void pppoe_disc_input(struct netif *netif, struct pbuf *p);
void pppoe_data_input(struct netif *netif, struct pbuf *p);




 /* PPP_OE_H */

 /* PPP_SUPPORT && PPPOE_SUPPORT */
