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
 * This file is part of and a contribution to the lwIP TCP/IP stack.
 *
 * Credits go to Adam Dunkels (and the current maintainers) of this software.
 *
 * Christiaan Simons rewrote this file to get a more stable echo example.
 */

/*
 * @file
 * TCP echo server example using raw API.
 *
 * Echos all bytes sent by connecting client,
 * and passively closes when client is done.
 *
 */









static tcpecho_raw_pcb: &mut TcpContext;

enum tcpecho_raw_states
{
  ES_NONE = 0,
  ES_ACCEPTED,
  ES_RECEIVED,
  ES_CLOSING
}

pub struct tcpecho_raw_state
{
  pub state: u8,
  pub retries: u8,
  pub pcb: &mut TcpContext,
  //  pbuf (chain) to recycle 
  pub p: &mut PacketBuffer,
}

pub fn
tcpecho_raw_free(es: &mut tcpecho_raw_state)
{
  if (es != None) {
    if (es.p) {
      //  free the buffer chain if present 
      pbuf_free(es.p);
    }

    mem_free(es);
  }  
}

pub fn
tcpecho_raw_close(tpcb: &mut TcpContext, es: &mut tcpecho_raw_state)
{
  tcp_arg(tpcb, None);
  tcp_sent(tpcb, None);
  tcp_recv(tpcb, None);
  tcp_err(tpcb, None);
  tcp_poll(tpcb, None, 0);

  tcpecho_raw_free(es);

  tcp_close(tpcb);
}

pub fn
tcpecho_raw_send(tpcb: &mut TcpContext, es: &mut tcpecho_raw_state)
{
  let ptr: &mut PacketBuffer;
  let wr_err: err_t = ERR_OK;
 
  while ((wr_err == ERR_OK) &&
         (es.p != None) &&
         (es.p.len <= tcp_sndbuf(tpcb))) {
    ptr = es.p;

    //  enqueue data for transmission 
    wr_err = tcp_write(tpcb, ptr.payload, ptr.len, 1);
    if (wr_err == ERR_OK) {
      let plen: u16;

      plen = ptr.len;
      //  continue with next pbuf in chain (if any) 
      es.p = ptr.next;
      if(es.p != None) {
        //  new reference! 
        pbuf_ref(es.p);
      }
      //  chop first pbuf from chain 
      pbuf_free(ptr);
      //  we can read more data now 
      tcp_recved(tpcb, plen);
    } else if(wr_err == ERR_MEM) {
      //  we are low on memory, try later / harder, defer to poll 
      es.p = ptr;
    } else {
      //  other problem ?? 
    }
  }
}

pub fn
tcpecho_raw_error(arg: &mut Vec<u8>, err: err_t)
{
  let es: &mut tcpecho_raw_state;

  

  es = arg;

  tcpecho_raw_free(es);
}

pub fn tcpecho_raw_poll(arg: &mut Vec<u8>, tpcb: &mut TcpContext) -> Result<(), LwipError>
{
  let ret_err: err_t;
  let es: &mut tcpecho_raw_state;

  es = arg;
  if (es != None) {
    if (es.p != None) {
      //  there is a remaining pbuf (chain)  
      tcpecho_raw_send(tpcb, es);
    } else {
      //  no remaining pbuf (chain)  
      if(es.state == ES_CLOSING) {
        tcpecho_raw_close(tpcb, es);
      }
    }
    ret_err = ERR_OK;
  } else {
    //  nothing to be done 
    tcp_abort(tpcb);
    ret_err = ERR_ABRT;
  }
  return ret_err;
}

pub fn tcpecho_raw_sent(arg: &mut Vec<u8>, tpcb: &mut TcpContext, len: usize) -> Result<(), LwipError>
{
  let es: &mut tcpecho_raw_state;

  

  let es = arg;
  es.retries = 0;
  
  if(es.p != None) {
    //  still got pbufs to send 
    tcp_sent(tpcb, tcpecho_raw_sent);
    tcpecho_raw_send(tpcb, es);
  } else {
    //  no more pbufs to send 
    if(es.state == ES_CLOSING) {
      tcpecho_raw_close(tpcb, es);
    }
  }
 return Ok(());
}

pub fn tcpecho_raw_recv(arg: &mut Vec<u8>, tpcb: &mut TcpContext, p: &mut PacketBuffer, err: err_t) -> Result<(), LwipError>
{
  let es: &mut tcpecho_raw_state;
  let ret_err: err_t;

  LWIP_ASSERT("arg != NULL",arg != None);
  es = arg;
  if (p == None) {
    //  remote host closed connection 
    es.state = ES_CLOSING;
    if(es.p == None) {
      //  we're done sending, close it 
      tcpecho_raw_close(tpcb, es);
    } else {
      //  we're not done yet 
      tcpecho_raw_send(tpcb, es);
    }
    ret_err = ERR_OK;
  } else if(err != ERR_OK) {
    //  cleanup, for unknown reason 
    if (p != None) {
      pbuf_free(p);
    }
    ret_err = err;
  }
  else if(es.state == ES_ACCEPTED) {
    //  first data chunk in p.payload 
    es.state = ES_RECEIVED;
    //  store reference to incoming pbuf (chain) 
    es.p = p;
    tcpecho_raw_send(tpcb, es);
    ret_err = ERR_OK;
  } else if (es.state == ES_RECEIVED) {
    //  read some more data 
    if(es.p == None) {
      es.p = p;
      tcpecho_raw_send(tpcb, es);
    } else {
      let ptr: &mut PacketBuffer;

      //  chain pbufs to the end of what we recv'ed previously  
      ptr = es.p;
      pbuf_cat(ptr,p);
    }
    ret_err = ERR_OK;
  } else {
    //  unkown es.state, trash data  
    tcp_recved(tpcb, p.tot_len);
    pbuf_free(p);
    ret_err = ERR_OK;
  }
  return ret_err;
}

pub fn tcpecho_raw_accept(arg: &mut Vec<u8>, newpcb: &mut TcpContext, err: err_t) -> Result<(), LwipError>
{
  let ret_err: err_t;
  let es: &mut tcpecho_raw_state;

  
  if ((err != ERR_OK) || (newpcb == None)) {
    return ERR_VAL;
  }

  /* Unless this pcb should have NORMAL priority, set its priority now.
     When running out of pcbs, low priority pcbs can be aborted to create
     new pcbs of higher priority. */
  tcp_setprio(newpcb, TCP_PRIO_MIN);

  es = mem_malloc(sizeof(tcpecho_raw_state));
  if (es != None) {
    es.state = ES_ACCEPTED;
    es.pcb = newpcb;
    es.retries = 0;
    es.p = None;
    //  pass newly allocated es to our callbacks 
    tcp_arg(newpcb, es);
    tcp_recv(newpcb, tcpecho_raw_recv);
    tcp_err(newpcb, tcpecho_raw_error);
    tcp_poll(newpcb, tcpecho_raw_poll, 0);
    tcp_sent(newpcb, tcpecho_raw_sent);
    ret_err = ERR_OK;
  } else {
    ret_err = ERR_MEM;
  }
  return ret_err;
}

pub fn 
tcpecho_raw_init()
{
  tcpecho_raw_pcb = tcp_new_ip_type(IPADDR_TYPE_ANY);
  if (tcpecho_raw_pcb != None) {
    let err: err_t;

    err = tcp_bind(tcpecho_raw_pcb, IP_ANY_TYPE, 7);
    if (err == ERR_OK) {
      tcpecho_raw_pcb = tcp_listen(tcpecho_raw_pcb);
      tcp_accept(tcpecho_raw_pcb, tcpecho_raw_accept);
    } else {
      //  abort? output diagnostic? 
    }
  } else {
    //  abort? output diagnostic? 
  }
}


