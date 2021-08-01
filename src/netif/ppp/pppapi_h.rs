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
  ppp_pcb *ppp;
  union {

    struct {
      ppp_notify_phase_cb_fn notify_phase_cb;
    } setnotifyphasecb;


    struct {
      pppif: &mut netif;
      pppos_output_cb_fn output_cb;
      ppp_link_status_cb_fn link_status_cb;
      void *ctx_cb;
    } serialcreate;


    struct {
      pppif: &mut netif;
      ethif: &mut netif;
      service_name: String;
      concentrator_name: String;
      ppp_link_status_cb_fn link_status_cb;
      void *ctx_cb;
    } ethernetcreate;


    struct {
      pppif: &mut netif;
      netif: &mut netif;
      API_MSG_M_DEF_C(ip_addr_t, ipaddr);
      port: u16;

      const u8 *secret;
      secret_len: u8;

      ppp_link_status_cb_fn link_status_cb;
      void *ctx_cb;
    } l2tpcreate;

    struct {
      holdoff: u16;
    } connect;
    struct {
      nocarrier: u8;
    } close;
    struct {
      cmd: u8;
      arg: &mut Vec<u8>;
    } ioctl;
  } msg;
};

struct pppapi_msg {
  struct tcpip_api_call_data call;
  struct pppapi_msg_msg msg;
};

/* API for application */
pub fn  pppapi_set_default(ppp_pcb *pcb);

pub fn  pppapi_set_notify_phase_callback(ppp_pcb *pcb, ppp_notify_phase_cb_fn notify_phase_cb);


ppp_pcb *pppapi_pppos_create(pppif: &mut netif, pppos_output_cb_fn output_cb, ppp_link_status_cb_fn link_status_cb, void *ctx_cb);


ppp_pcb *pppapi_pppoe_create(pppif: &mut netif, ethif: &mut netif, const char *service_name,
                                const char *concentrator_name, ppp_link_status_cb_fn link_status_cb,
                                void *ctx_cb);


ppp_pcb *pppapi_pppol2tp_create(pppif: &mut netif, netif: &mut netif, ipaddr: &mut ip_addr_t, port: u16,
                            const u8 *secret, secret_len: u8,
                            ppp_link_status_cb_fn link_status_cb, void *ctx_cb);

pub fn  pppapi_connect(ppp_pcb *pcb, holdoff: u16);

pub fn  pppapi_listen(ppp_pcb *pcb);

pub fn  pppapi_close(ppp_pcb *pcb, nocarrier: u8);
pub fn  pppapi_free(ppp_pcb *pcb);
pub fn  pppapi_ioctl(ppp_pcb *pcb, cmd: u8, arg: &mut Vec<u8>);


}





