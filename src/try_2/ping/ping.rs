/*
 * @file
 * Ping sender module
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

/*
 * This is an example of a "ping" sender (with raw API and socket API).
 * It can be used as a start poto: i32 maintain opened a network connection, or
 * like a network "watchdog" for your device.
 *
 */























/*
 * PING_DEBUG: Enable debugging for PING.
 */

pub const PING_DEBUG: u32 = LWIP_DBG_ON;


//  ping receive timeout - in milliseconds 

pub const PING_RCV_TIMEO: u32 = 1000; 


//  ping delay - in milliseconds 

pub const PING_DELAY: u32 = 1000; 


//  ping identifier - must fit on a u16 

pub const PING_ID: u32 = 0xAFAF;


//  ping additional data size to include in the packet 

pub const PING_DATA_SIZE: u32 = 32; 


//  ping result action - no default action 

// pub const PING_RESULT(ping_ok)


//  ping variables 
// static const ping_target: &mut LwipAddr;
// static ping_seq_num: u16;

// static ping_time: u32;


// static ping_pcb: &mut raw_pcb;


//  Prepare a echo ICMP request 
pub fn
ping_prepare_echo( iecho: &mut icmp_echo_hdr, len: usize)
{
  let i: usize;
  let data_len: usize = len - sizeof(icmp_echo_hdr);

  ICMPH_TYPE_SET(iecho, ICMP_ECHO);
  ICMPH_CODE_SET(iecho, 0);
  iecho.chksum = 0;
  iecho.id     = PING_ID;
  iecho.seqno  = lwip_htons(ping_seq_num += 1);

  //  fill the additional data buffer with some data 
  // TODO
  // for(i = 0; i < data_len; i+= 1) {
  //   (iecho)[sizeof(icmp_echo_hdr) + i] = (char)i;
  // }

  iecho.chksum = inet_chksum(iecho, len);
}



//  Ping using the socket ip 
pub fn ping_send(s: i32,  addr: &mut LwipAddr) -> Result<(), LwipError>
{
  let err: i32;
  let mut iecho: &mut icmp_echo_hdr;
  let to: sockaddr_storage;
  let ping_size: usize = sizeof(icmp_echo_hdr) + PING_DATA_SIZE;
  LWIP_ASSERT("ping_size is too big", ping_size <= 0xffff);


  if(IP_IS_V6(addr) && !ip6_addr_isipv4mappedipv6(ip_2_ip6(addr))) {
    //  todo: support ICMP6 echo 
    return ERR_VAL;
  }


  iecho = mem_malloc(ping_size);
  if (!iecho) {
    return ERR_MEM;
  }

  ping_prepare_echo(iecho, ping_size);
  

  if(IP_IS_V4(addr)) {
    let to4: &mut sockaddr_in = &to;
    to4.sin_len    = sizeof(to4);
    to4.sin_family = AF_INET;
    inet_addr_from_ip4addr(&to4.sin_addr, ip_2_ip4(addr));
  }



  if(IP_IS_V6(addr)) {
    let to6: &mut sockaddr_in6 = &to;
    to6.sin6_len    = sizeof(to6);
    to6.sin6_family = AF_INET6;
    inet6_addr_from_ip6addr(&to6.sin6_addr, ip_2_ip6(addr));
  }


  err = lwip_sendto(s, iecho, ping_size, 0, &to, sizeof(to));

  mem_free(iecho);

  // return (err ? ERR_OK : ERR_VAL);
  return err;
}

pub fn
ping_recv(s: i32)
{
  let buf: String;
  let len: i32;
  let from: sockaddr_storage;
  let fromlen: i32 = sizeof(from);

  while((len = lwip_recvfrom(s, buf, sizeof(buf), 0, &from, &fromlen)) > 0) {
    if (len >= (sizeof(ip_hdr)+sizeof(icmp_echo_hdr))) {
      let fromaddr: LwipAddr;
      //memset(&fromaddr, 0, sizeof(fromaddr));


      if(from.ss_family == AF_INET) {
        let from4: &mut sockaddr_in = &from;
        inet_addr_to_ip4addr(ip_2_ip4(&fromaddr), &from4.sin_addr);
        IP_SET_TYPE_VAL(fromaddr, IPADDR_TYPE_V4);
      }



      if(from.ss_family == AF_INET6) {
        let from6: &mut sockaddr_in6 = &from;
        inet6_addr_to_ip6addr(ip_2_ip6(&fromaddr), &from6.sin6_addr);
        IP_SET_TYPE_VAL(fromaddr, IPADDR_TYPE_V6);
      }

      
//      LWIP_DEBUGF( PING_DEBUG, ("ping: recv "));
      ip_addr_debug_print_val(PING_DEBUG, fromaddr);
//      LWIP_DEBUGF( PING_DEBUG, (" %"U32_F" ms\n", (sys_now() - ping_time)));

      //  todo: support ICMP6 echo 

      if (IP_IS_V4_VAL(fromaddr)) {
        let mut iphdr: &mut ip_hdr;
        let mut iecho: &mut icmp_echo_hdr;

        iphdr = buf;
        iecho = (buf + (IPH_HL(iphdr) * 4));
        if ((iecho.id == PING_ID) && (iecho.seqno == lwip_htons(ping_seq_num))) {
          //  do some ping result processing 
          PING_RESULT((ICMPH_TYPE(iecho) == ICMP_ER));
          return;
        } else {
//          LWIP_DEBUGF( PING_DEBUG, ("ping: drop\n"));
        }
      }

    }
    fromlen = sizeof(from);
  }

  if (len == 0) {
//    LWIP_DEBUGF( PING_DEBUG, ("ping: recv - %"U32_F" ms - timeout\n", (sys_now()-ping_time)));
  }

  //  do some ping result processing 
  PING_RESULT(0);
}

pub fn
ping_thread(arg: &mut Vec<u8>)
{
  let s: i32;
  let ret: i32;


  let timeout: i32 = PING_RCV_TIMEO;

  let timeout: timeval;
  timeout.tv_sec = PING_RCV_TIMEO/1000;
  timeout.tv_usec = (PING_RCV_TIMEO%1000)*1000;

  


  if(IP_IS_V4(ping_target) || ip6_addr_isipv4mappedipv6(ip_2_ip6(ping_target))) {
    s = lwip_socket(AF_INET6, SOCK_RAW, IP_PROTO_ICMP);
  } else {
    s = lwip_socket(AF_INET6, SOCK_RAW, IP6_NEXTH_ICMP6);
  }

  s = lwip_socket(AF_INET,  SOCK_RAW, IP_PROTO_ICMP);

  if (s < 0) {
    return;
  }

  ret = lwip_setsockopt(s, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
  LWIP_ASSERT("setting receive timeout failed", ret == 0);
  

  loop {
    if (ping_send(s, ping_target) == ERR_OK) {
//      LWIP_DEBUGF( PING_DEBUG, ("ping: send "));
      ip_addr_debug_print(PING_DEBUG, ping_target);
//      LWIP_DEBUGF( PING_DEBUG, ("\n"));


      ping_time = sys_now();

      ping_recv(s);
    } else {
//      LWIP_DEBUGF( PING_DEBUG, ("ping: send "));
      ip_addr_debug_print(PING_DEBUG, ping_target);
//      LWIP_DEBUGF( PING_DEBUG, (" - error\n"));
    }
    sys_msleep(PING_DELAY);
  }
}

 //  PING_USE_SOCKETS 

//  Ping using the raw ip 
pub fn ping_recv(arg: &mut Vec<u8>, pcb: &mut raw_pcb, p: &mut PacketBuffer,  addr: &mut LwipAddr)
{
  let mut iecho: &mut icmp_echo_hdr;
  
  
  
  LWIP_ASSERT("p != NULL", p != None);

  if ((p.tot_len >= (PBUF_IP_HLEN + sizeof(icmp_echo_hdr))) &&
      pbuf_remove_header(p, PBUF_IP_HLEN) == 0) {
    iecho = p.payload;

    if ((iecho.id == PING_ID) && (iecho.seqno == lwip_htons(ping_seq_num))) {
//      LWIP_DEBUGF( PING_DEBUG, ("ping: recv "));
      ip_addr_debug_print(PING_DEBUG, addr);
//      LWIP_DEBUGF( PING_DEBUG, (" %"U32_F" ms\n", (sys_now()-ping_time)));

      //  do some ping result processing 
      PING_RESULT(1);
      pbuf_free(p);
      return 1; //  eat the packet 
    }
    //  not eaten, restore original packet 
    pbuf_add_header(p, PBUF_IP_HLEN);
  }

  return 0; //  don't eat the packet 
}

pub fn
ping_send(raw: &mut raw_pcb,  addr: &mut LwipAddr)
{
  let p: &mut PacketBuffer;
  let mut iecho: &mut icmp_echo_hdr;
  let ping_size: usize = sizeof(icmp_echo_hdr) + PING_DATA_SIZE;

//  LWIP_DEBUGF( PING_DEBUG, ("ping: send "));
  ip_addr_debug_print(PING_DEBUG, addr);
//  LWIP_DEBUGF( PING_DEBUG, ("\n"));
  LWIP_ASSERT("ping_size <= 0xffff", ping_size <= 0xffff);

  p = pbuf_alloc(PBUF_IP, ping_size, PBUF_RAM);
  if (!p) {
    return;
  }
  if ((p.len == p.tot_len) && (p.next == None)) {
    iecho = p.payload;

    ping_prepare_echo(iecho, ping_size);

    raw_sendto(raw, p, addr);

    ping_time = sys_now();

  }
  pbuf_free(p);
}

pub fn
ping_timeout(arg: &mut Vec<u8>)
{
  let pcb: &mut raw_pcb = arg;

  LWIP_ASSERT("ping_timeout: no pcb given!", pcb != None);

  ping_send(pcb, ping_target);

  sys_timeout(PING_DELAY, ping_timeout, pcb);
}

pub fn
ping_raw_init()
{
  ping_pcb = raw_new(IP_PROTO_ICMP);
  LWIP_ASSERT("ping_pcb != NULL", ping_pcb != None);

  raw_recv(ping_pcb, ping_recv, None);
  raw_bind(ping_pcb, IP_ADDR_ANY);
  sys_timeout(PING_DELAY, ping_timeout, ping_pcb);
}

pub fn 
ping_send_now()
{
  LWIP_ASSERT("ping_pcb != NULL", ping_pcb != None);
  ping_send(ping_pcb, ping_target);
}



pub fn 
ping_init( ping_addr: &mut LwipAddr)
{
  ping_target = ping_addr;


  sys_thread_new("ping_thread", ping_thread, None, DEFAULT_THREAD_STACKSIZE, DEFAULT_THREAD_PRIO);
 //  PING_USE_SOCKETS 
  ping_raw_init();

}


