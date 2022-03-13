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































pub struct pcapif {
  pub pd: &mut pcap_t,
  pub sem: sys_sem_t,
  pub pkt: Vec<u8>,
  pub len: u32,
  pub lasttime: u32,
  pub p: PacketBuffer,
  pub ethaddr: LwipAddr,
}

static errbuf: [u8;PCAP_ERRBUF_SIZE];

// -----------------------------------------------------------------------------------
pub fn pcapif_output(netif: &mut NetIfc, p: &mut PacketBuffer,
	      ipaddr: &mut LwipAddr)
{
 return Ok(());
}
// -----------------------------------------------------------------------------------
pub fn
timeout(arg: &mut Vec<u8>)
{
  let mut netif: &mut NetIfc;
  let mut pcapif: &mut pcapif;
  let p: &mut PacketBuffer;
  let mut ethhdr: &mut eth_hdr;
  
  netif = arg;
  pcapif = netif.state;
  ethhdr = pcapif.pkt;

  
  if (lwip_htons(ethhdr.ether_type) != ETHTYPE_IP ||
     ip_lookup(pcapif.pkt + 14, netif)) {
    
    //  We allocate a pbuf chain of pbufs from the pool. 
    p = pbuf_alloc(PBUF_LINK, pcapif.len, PBUF_POOL);
    
    if (p != None) {
      pbuf_take(p, pcapif.pkt, pcapif.len);

      ethhdr = p.payload;
      match (lwip_htons(ethhdr.ether_type)) {
      //  IP or ARP packet? 
      ETHTYPE_IP |
      ETHTYPE_ARP |

      //  PPPoE packet? 
      ETHTYPE_PPPOEDISC |
      ETHTYPE_PPPOE =>{

        //  full packet send to tcpip_thread to process 
        if (netif.input(p, netif) != ERR_OK) {
//          LWIP_DEBUGF(NETIF_DEBUG, ("ethernetif_input: IP input error\n"));
          pbuf_free(p);
          p = None;
        }}
        
      _ =>{
        pbuf_free(p);}
        
      }
    }
  } else {
    printf("ip_lookup dropped\n");
  }

  sys_sem_signal(&pcapif.sem);
}
// -----------------------------------------------------------------------------------
pub fn
callback(u_arg: &mut String,  hdr: &mut pcap_pkthdr,  u_pkt: &mut String)
{
  let mut netif: &mut NetIfc;
  let mut pcapif: &mut pcapif;
  let time: u32;
  let lasttime;
  
  netif = arg;
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
// -----------------------------------------------------------------------------------
pub fn
pcapif_thread(arg: &mut Vec<u8>)
{
  let mut netif: &mut NetIfc;
  let mut pcapif: &mut pcapif;
  netif = arg;
  pcapif = netif.state;

  loop {
    pcap_loop(pcapif.pd, 1, callback, netif);
    sys_sem_wait(&pcapif.sem);
    if (pcapif.p != None) {
      netif.input(pcapif.p, netif);
    }
  }
}
// -----------------------------------------------------------------------------------
pub fn 
pcapif_init(netif: &mut NetIfc)
{
  let mut p: &mut pcapif;
    
  p = malloc(sizeof(pcapif));
  if (p == None){
      return ERR_MEM;}
  netif.state = p;
  netif.name[0] = 'p';
  netif.name[1] = 'c';
  netif.output = pcapif_output;

  p.pd = pcap_open_offline("pcapdump", errbuf);
  if (p.pd == None) {
    printf("pcapif_init: failed %s\n", errbuf);
    return ERR_IF;
  }

  if(sys_sem_new(&p.sem, 0) != ERR_OK) {
    // LWIP_ASSERT("Failed to create semaphore", 0);
  }
  p.p = None;
  p.lasttime = 0;
  
  sys_thread_new("pcapif_thread", pcapif_thread, netif, DEFAULT_THREAD_STACKSIZE, DEFAULT_THREAD_PRIO);
 return Ok(());
}
// -----------------------------------------------------------------------------------

