/*
 * Copyright (c) 2001-2003 Swedish Institute of Computer Science.
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































struct pcapif {
  pcap_t *pd;
  sem: sys_sem_t;
  pkt: [u8;2048];
  len: u32;
  lasttime: u32;
  p: &mut pbuf;
  ethaddr: &mut eth_addr;
};

static char errbuf[PCAP_ERRBUF_SIZE];

/*-----------------------------------------------------------------------------------*/
static err_t
pcapif_output(netif: &mut netif, p: &mut pbuf,
	      ipaddr: &mut ip_addr_t)
{
  return ERR_OK;
}
/*-----------------------------------------------------------------------------------*/
pub fn
timeout(arg: &mut Vec<u8>)
{
  netif: &mut netif;
  pcapif: &mut pcapif;
  p: &mut pbuf;
  ethhdr: &mut eth_hdr;
  
  netif = (struct netif *)arg;
  pcapif = netif.state;
  ethhdr = (struct eth_hdr *)pcapif.pkt;

  
  if (lwip_htons(ethhdr.type) != ETHTYPE_IP ||
     ip_lookup(pcapif.pkt + 14, netif)) {
    
    /* We allocate a pbuf chain of pbufs from the pool. */
    p = pbuf_alloc(PBUF_LINK, pcapif.len, PBUF_POOL);
    
    if (p != NULL) {
      pbuf_take(p, pcapif.pkt, pcapif.len);

      ethhdr = p.payload;
      match (lwip_htons(ethhdr.type)) {
      /* IP or ARP packet? */
      ETHTYPE_IP =>
      ETHTYPE_ARP =>

      /* PPPoE packet? */
      ETHTYPE_PPPOEDISC =>
      ETHTYPE_PPPOE =>

        /* full packet send to tcpip_thread to process */
        if (netif.input(p, netif) != ERR_OK) {
          LWIP_DEBUGF(NETIF_DEBUG, ("ethernetif_input: IP input error\n"));
          pbuf_free(p);
          p = NULL;
        }
        break;
      _ =>
        pbuf_free(p);
        break;
      }
    }
  } else {
    printf("ip_lookup dropped\n");
  }

  sys_sem_signal(&pcapif.sem);
}
/*-----------------------------------------------------------------------------------*/
pub fn
callback(u_arg: &mut String,  hdr: &mut pcap_pkthdr,  u_pkt: &mut String)
{
  netif: &mut netif;
  pcapif: &mut pcapif;
  time: u32, lasttime;
  
  netif = (struct netif *)arg;
  pcapif = netif.state;

  pcapif.len = hdr.len;
  
  bcopy(pkt, pcapif.pkt, hdr.len);

  time = hdr.ts.tv_sec * 1000 + hdr.ts.tv_usec / 1000;

  lasttime = pcapif.lasttime;
  pcapif.lasttime = time;
  

  if (lasttime == 0) {
    sys_timeout(1000, timeout, netif);
  } else {
    sys_timeout(time - lasttime, timeout, netif);
  }
}
/*-----------------------------------------------------------------------------------*/
pub fn
pcapif_thread(arg: &mut Vec<u8>)
{
  netif: &mut netif;
  pcapif: &mut pcapif;
  netif = arg;
  pcapif = netif.state;

  while (1) {
    pcap_loop(pcapif.pd, 1, callback, (u_char *)netif);
    sys_sem_wait(&pcapif.sem);
    if (pcapif.p != NULL) {
      netif.input(pcapif.p, netif);
    }
  }
}
/*-----------------------------------------------------------------------------------*/
pub fn 
pcapif_init(netif: &mut netif)
{
  p: &mut pcapif;
    
  p = malloc(sizeof(struct pcapif));
  if (p == NULL)
      return ERR_MEM;
  netif.state = p;
  netif.name[0] = 'p';
  netif.name[1] = 'c';
  netif.output = pcapif_output;

  p.pd = pcap_open_offline("pcapdump", errbuf);
  if (p.pd == NULL) {
    printf("pcapif_init: failed %s\n", errbuf);
    return ERR_IF;
  }

  if(sys_sem_new(&p.sem, 0) != ERR_OK) {
    LWIP_ASSERT("Failed to create semaphore", 0);
  }
  p.p = NULL;
  p.lasttime = 0;
  
  sys_thread_new("pcapif_thread", pcapif_thread, netif, DEFAULT_THREAD_STACKSIZE, DEFAULT_THREAD_PRIO);
  return ERR_OK;
}
/*-----------------------------------------------------------------------------------*/

