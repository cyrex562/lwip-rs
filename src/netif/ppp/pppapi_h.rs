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


// #define LWIP_PPPAPI_H

















struct pppapi_msg_msg {
  ppp: &mut ppp_pcb;
  union {

    struct {
      ppp_notify_phase_cb_fn notify_phase_cb;
    } setnotifyphasecb;


    struct {
      pppif: &mut NetIfc;
      pppos_output_cb_fn output_cb;
      ppp_link_status_cb_fn link_status_cb;
      ctx_cb: &mut ();
    } serialcreate;


    struct {
      pppif: &mut NetIfc;
      ethif: &mut NetIfc;
      let service_name: String;
      let concentrator_name: String;
      ppp_link_status_cb_fn link_status_cb;
      ctx_cb: &mut ();
    } ethernetcreate;


    struct {
      pppif: &mut NetIfc;
      netif: &mut NetIfc;
      API_MSG_M_DEF_C(LwipAddr, ipaddr);
      let port: u16;

      const secret: &mut Vec<u8>;
      let secret_len: u8;

      ppp_link_status_cb_fn link_status_cb;
      ctx_cb: &mut ();
    } l2tpcreate;

    struct {
      let holdoff: u16;
    } connect;
    struct {
      let nocarrier: u8;
    } close;
    struct {
      let cmd: u8;
      arg: &mut Vec<u8>;
    } ioctl;
  } msg;
};

struct pppapi_msg {
  let call: tcpip_api_call_data;
  let msg: pppapi_msg_msg;
};

/* API for application */
pub fn  pppapi_set_default(pcb: &mut ppp_pcb);

pub fn  pppapi_set_notify_phase_callback(pcb: &mut ppp_pcb, ppp_notify_phase_cb_fn notify_phase_cb);


pppapi_pppos_create: &mut ppp_pcb(pppif: &mut NetIfc, pppos_output_cb_fn output_cb, ppp_link_status_cb_fn link_status_cb, ctx_cb: &mut ());


pppapi_pppoe_create: &mut ppp_pcb(pppif: &mut NetIfc, ethif: &mut NetIfc, service_name: &String,
                                concentrator_name: &String, ppp_link_status_cb_fn link_status_cb,
                                ctx_cb: &mut ());


pppapi_pppol2tp_create: &mut ppp_pcb(pppif: &mut NetIfc, netif: &mut NetIfc, ipaddr: &mut LwipAddr, port: u16,
                            const secret: &mut Vec<u8>, secret_len: u8,
                            ppp_link_status_cb_fn link_status_cb, ctx_cb: &mut ());

pub fn  pppapi_connect(pcb: &mut ppp_pcb, holdoff: u16);

pub fn  pppapi_listen(pcb: &mut ppp_pcb);

pub fn  pppapi_close(pcb: &mut ppp_pcb, nocarrier: u8);
pub fn  pppapi_free(pcb: &mut ppp_pcb);
pub fn  pppapi_ioctl(pcb: &mut ppp_pcb, cmd: u8, arg: &mut Vec<u8>);


}





