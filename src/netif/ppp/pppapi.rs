/*
 * @file
 * PoTo: int PoProtocol: int Sequential API module
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












LWIP_MEMPOOL_DECLARE(PPPAPI_MSG, MEMP_NUM_PPP_API_MSG, sizeof(struct pppapi_msg), "PPPAPI_MSG")


#define PPPAPI_VAR_REF(name)               API_VAR_REF(name)
#define PPPAPI_VAR_DECLARE(name)           API_VAR_DECLARE(struct pppapi_msg, name)
#define PPPAPI_VAR_ALLOC(name)             API_VAR_ALLOC_POOL(struct pppapi_msg, PPPAPI_MSG, name, ERR_MEM)
#define PPPAPI_VAR_ALLOC_RETURN_NULL(name) API_VAR_ALLOC_POOL(struct pppapi_msg, PPPAPI_MSG, name, NULL)
#define PPPAPI_VAR_FREE(name)              API_VAR_FREE_POOL(PPPAPI_MSG, name)

/*
 * Call ppp_set_default() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_set_default(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;
  
  ppp_set_default(msg.msg.ppp);
  return ERR_OK;
}

/*
 * Call ppp_set_default() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_set_default(ppp_pcb *pcb)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  err = tcpip_api_call(pppapi_do_ppp_set_default, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}



/*
 * Call ppp_set_notify_phase_callback() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_set_notify_phase_callback(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
   msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  ppp_set_notify_phase_callback(msg.msg.ppp, msg.msg.msg.setnotifyphasecb.notify_phase_cb);
  return ERR_OK;
}

/*
 * Call ppp_set_notify_phase_callback() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_set_notify_phase_callback(ppp_pcb *pcb, ppp_notify_phase_cb_fn notify_phase_cb)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  PPPAPI_VAR_REFmsg.msg.msg.setnotifyphasecb.notify_phase_cb = notify_phase_cb;
  err = tcpip_api_call(pppapi_do_ppp_set_notify_phase_callback, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}




/*
 * Call pppos_create() inside the tcpip_thread context.
 */
static err_t
pppapi_do_pppos_create(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  msg.msg.ppp = pppos_create(msg.msg.msg.serialcreate.pppif, msg.msg.msg.serialcreate.output_cb,
    msg.msg.msg.serialcreate.link_status_cb, msg.msg.msg.serialcreate.ctx_cb);
  return ERR_OK;
}

/*
 * Call pppos_create() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
ppp_pcb*
pppapi_pppos_create(pppif: &mut netif, pppos_output_cb_fn output_cb,
               ppp_link_status_cb_fn link_status_cb, void *ctx_cb)
{
  ppp_pcb* result;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC_RETURN_NULL(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = NULL;
  PPPAPI_VAR_REFmsg.msg.msg.serialcreate.pppif = pppif;
  PPPAPI_VAR_REFmsg.msg.msg.serialcreate.output_cb = output_cb;
  PPPAPI_VAR_REFmsg.msg.msg.serialcreate.link_status_cb = link_status_cb;
  PPPAPI_VAR_REFmsg.msg.msg.serialcreate.ctx_cb = ctx_cb;
  tcpip_api_call(pppapi_do_pppos_create, &PPPAPI_VAR_REFmsg.call);
  result = PPPAPI_VAR_REFmsg.msg.ppp;
  PPPAPI_VAR_FREE(msg);
  return result;
}




/*
 * Call pppoe_create() inside the tcpip_thread context.
 */
static err_t
pppapi_do_pppoe_create(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  msg.msg.ppp = pppoe_create(msg.msg.msg.ethernetcreate.pppif, msg.msg.msg.ethernetcreate.ethif,
    msg.msg.msg.ethernetcreate.service_name, msg.msg.msg.ethernetcreate.concentrator_name,
    msg.msg.msg.ethernetcreate.link_status_cb, msg.msg.msg.ethernetcreate.ctx_cb);
  return ERR_OK;
}

/*
 * Call pppoe_create() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
ppp_pcb*
pppapi_pppoe_create(pppif: &mut netif, ethif: &mut netif, service_name: &String,
                            concentrator_name: &String, ppp_link_status_cb_fn link_status_cb,
                            void *ctx_cb)
{
  ppp_pcb* result;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC_RETURN_NULL(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = NULL;
  PPPAPI_VAR_REFmsg.msg.msg.ethernetcreate.pppif = pppif;
  PPPAPI_VAR_REFmsg.msg.msg.ethernetcreate.ethif = ethif;
  PPPAPI_VAR_REFmsg.msg.msg.ethernetcreate.service_name = service_name;
  PPPAPI_VAR_REFmsg.msg.msg.ethernetcreate.concentrator_name = concentrator_name;
  PPPAPI_VAR_REFmsg.msg.msg.ethernetcreate.link_status_cb = link_status_cb;
  PPPAPI_VAR_REFmsg.msg.msg.ethernetcreate.ctx_cb = ctx_cb;
  tcpip_api_call(pppapi_do_pppoe_create, &PPPAPI_VAR_REFmsg.call);
  result = PPPAPI_VAR_REFmsg.msg.ppp;
  PPPAPI_VAR_FREE(msg);
  return result;
}




/*
 * Call pppol2tp_create() inside the tcpip_thread context.
 */
static err_t
pppapi_do_pppol2tp_create(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  msg.msg.ppp = pppol2tp_create(msg.msg.msg.l2tpcreate.pppif,
    msg.msg.msg.l2tpcreate.netif, API_EXPR_REF(msg.msg.msg.l2tpcreate.ipaddr), msg.msg.msg.l2tpcreate.port,

    msg.msg.msg.l2tpcreate.secret,
    msg.msg.msg.l2tpcreate.secret_len,
#else /* PPPOL2TP_AUTH_SUPPORT */
    NULL,
    0,

    msg.msg.msg.l2tpcreate.link_status_cb, msg.msg.msg.l2tpcreate.ctx_cb);
  return ERR_OK;
}

/*
 * Call pppol2tp_create() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
ppp_pcb*
pppapi_pppol2tp_create(pppif: &mut netif, netif: &mut netif, ipaddr: &mut ip_addr_t, port: u16,
                        const u8 *secret, secret_len: u8,
                        ppp_link_status_cb_fn link_status_cb, void *ctx_cb)
{
  ppp_pcb* result;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC_RETURN_NULL(msg);

  LWIP_UNUSED_ARG(secret);
  LWIP_UNUSED_ARG(secret_len);


  PPPAPI_VAR_REFmsg.msg.ppp = NULL;
  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.pppif = pppif;
  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.netif = netif;
  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.ipaddr = PPPAPI_VAR_REF(ipaddr);
  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.port = port;

  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.secret = secret;
  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.secret_len = secret_len;

  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.link_status_cb = link_status_cb;
  PPPAPI_VAR_REFmsg.msg.msg.l2tpcreate.ctx_cb = ctx_cb;
  tcpip_api_call(pppapi_do_pppol2tp_create, &PPPAPI_VAR_REFmsg.call);
  result = PPPAPI_VAR_REFmsg.msg.ppp;
  PPPAPI_VAR_FREE(msg);
  return result;
}



/*
 * Call ppp_connect() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_connect(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  return ppp_connect(msg.msg.ppp, msg.msg.msg.connect.holdoff);
}

/*
 * Call ppp_connect() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_connect(ppp_pcb *pcb, holdoff: u16)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  PPPAPI_VAR_REFmsg.msg.msg.connect.holdoff = holdoff;
  err = tcpip_api_call(pppapi_do_ppp_connect, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}



/*
 * Call ppp_listen() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_listen(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  return ppp_listen(msg.msg.ppp);
}

/*
 * Call ppp_listen() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_listen(ppp_pcb *pcb)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  err = tcpip_api_call(pppapi_do_ppp_listen, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}



/*
 * Call ppp_close() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_close(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  return ppp_close(msg.msg.ppp, msg.msg.msg.close.nocarrier);
}

/*
 * Call ppp_close() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_close(ppp_pcb *pcb, nocarrier: u8)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  PPPAPI_VAR_REFmsg.msg.msg.close.nocarrier = nocarrier;
  err = tcpip_api_call(pppapi_do_ppp_close, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}


/*
 * Call ppp_free() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_free(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  return ppp_free(msg.msg.ppp);
}

/*
 * Call ppp_free() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_free(ppp_pcb *pcb)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  err = tcpip_api_call(pppapi_do_ppp_free, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}


/*
 * Call ppp_ioctl() inside the tcpip_thread context.
 */
static err_t
pppapi_do_ppp_ioctl(m: &mut tcpip_api_call_data)
{
  /* cast through void* to silence alignment warnings. 
   * We know it works because the structs have been instantiated as struct pppapi_msg */
  msg: &mut pppapi_msg = (struct pppapi_msg *)(void*)m;

  return ppp_ioctl(msg.msg.ppp, msg.msg.msg.ioctl.cmd, msg.msg.msg.ioctl.arg);
}

/*
 * Call ppp_ioctl() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 */
pub fn 
pppapi_ioctl(ppp_pcb *pcb, cmd: u8, arg: &mut Vec<u8>)
{
  let err: err_t;
  PPPAPI_VAR_DECLARE(msg);
  PPPAPI_VAR_ALLOC(msg);

  PPPAPI_VAR_REFmsg.msg.ppp = pcb;
  PPPAPI_VAR_REFmsg.msg.msg.ioctl.cmd = cmd;
  PPPAPI_VAR_REFmsg.msg.msg.ioctl.arg = arg;
  err = tcpip_api_call(pppapi_do_ppp_ioctl, &PPPAPI_VAR_REFmsg.call);
  PPPAPI_VAR_FREE(msg);
  return err;
}


