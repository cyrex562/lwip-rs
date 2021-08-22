/*
 * @file
 * Network Poto: i32 PoProtocol: i32 over Serial header file.
 *
 */

/*
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
 */





#define PPPOS_H










/* PPP packet parser states.  Current state indicates operation yet to be
 * completed. */
enum {
  PDIDLE = 0,  /* Idle state - waiting. */
  PDSTART,     /* Process start flag. */
  PDADDRESS,   /* Process address field. */
  PDCONTROL,   /* Process control field. */
  PDPROTOCOL1, /* Process protocol field 1. */
  PDPROTOCOL2, /* Process protocol field 2. */
  PDDATA       /* Process data byte. */
};

/* PPPoS serial output callback function prototype */
typedef u32 (*pppos_output_cb_fn)(pcb: &mut ppp_pcb, data: &mut Vec<u8>, len: u32, ctx: &mut ());

/*
 * Extended asyncmap - allows any character to be escaped.
 */
typedef ext_accm: [u8;32];

/*
 * PPPoS interface control block.
 */
typedef struct pppos_pcb_s pppos_pcb;
struct pppos_pcb_s {
  /* -- below are data that will NOT be cleared between two sessions */
  ppp: &mut ppp_pcb;                    /* PPP PCB */
  pppos_output_cb_fn output_cb;    /* PPP serial output callback */

  /* -- below are data that will be cleared between two sessions
   *
   * last_xmit must be the first member of cleared members, because it is
   * used to know which part must not be cleared.
   */
  let last_xmit: u32;                 /* Time of last transmission. */
  ext_accm out_accm;               /* Async-Ctl-Char-Map for output. */

  /* flags */
   open: i32            :1; /* Set if PPPoS is open */
   pcomp: i32           :1; /* Does peer accept protocol compression? */
   accomp: i32          :1; /* Does peer accept addr/ctl compression? */

  /* PPPoS rx */
  ext_accm in_accm;                /* Async-Ctl-Char-Map for input. */
  in_head: &mut pbuf, *in_tail;  /* The input packet. */
  let in_protocol: u16;               /* The input protocol code. */  let in_protocol: u16;
  in_fcs: u16;                    /* Input Frame Check Sequence value. */
  let in_state: u8;                   /* The input process state. */  let in_state: u8;
  in_escaped: u8;                 /* Escape next character. */
};

/* Create a new PPPoS session. */
pppos_create: &mut ppp_pcb(pppif: &mut NetIfc, pppos_output_cb_fn output_cb,
       ppp_link_status_cb_fn link_status_cb, ctx_cb: &mut ());


/* Pass received raw characters to PPPoS to be decoded through lwIP TCPIP thread. */
pub fn  pppos_input_tcpip(ppp: &mut ppp_pcb, s: &mut Vec<u8>, l: i32);


/* PPP over Serial: this is the input function to be called for received data. */
pub fn  pppos_input(ppp: &mut ppp_pcb, u8* data, len: i32);


/*
 * Functions called from lwIP
 * DO NOT CALL FROM lwIP USER APPLICATION.
 */

pub fn  pppos_input_sys(p: &mut pbuf, inp: &mut NetIfc);



}




