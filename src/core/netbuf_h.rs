/**
 * @file
 * netbuf API (for netconn API)
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

#define LWIP_HDR_NETBUF_H




/* Note: Netconn API is always available when sockets are enabled -
 * sockets are implemented on top of them */






extern "C" {


/** This netbuf has dest-addr/port set */
pub const NETBUF_FLAG_DESTADDR: u32 = 0x01;
/** This netbuf includes a checksum */
pub const NETBUF_FLAG_CHKSUM: u32 = 0x02;

/** "Network buffer" - contains data and addressing info */
struct netbuf {
  p: &mut pbuf, *ptr;
  ip_addr_t addr;
  port: u16;

  flags: u8;
  toport_chksum: u16;

  ip_addr_t toaddr;


};

/* Network buffer functions: */
struct netbuf *   netbuf_new      (void);
pub fn               netbuf_delete   (buf: &mut netbuf);
pub fn  *            netbuf_alloc    (buf: &mut netbuf, size: u16);
pub fn               netbuf_free     (buf: &mut netbuf);
pub fn              netbuf_ref      (buf: &mut netbuf,
                                   dataptr: &Vec<u8>, size: u16);
pub fn               netbuf_chain    (head: &mut netbuf, tail: &mut netbuf);

pub fn              netbuf_data     (buf: &mut netbuf,
                                   void **dataptr, len: &mut u16);
s8_t              netbuf_next     (buf: &mut netbuf);
pub fn               netbuf_first    (buf: &mut netbuf);


#define netbuf_copy_partial(buf, dataptr, len, offset) \
  pbuf_copy_partial((buf)->p, (dataptr), (len), (offset))
#define netbuf_copy(buf,dataptr,len) netbuf_copy_partial(buf, dataptr, len, 0)
#define netbuf_take(buf, dataptr, len) pbuf_take((buf)->p, dataptr, len)
#define netbuf_len(buf)              ((buf)->p->tot_len)
#define netbuf_fromaddr(buf)         (&((buf)->addr))
#define netbuf_set_fromaddr(buf, fromaddr) ip_addr_set(&((buf)->addr), fromaddr)
#define netbuf_fromport(buf)         ((buf)->port)

#define netbuf_destaddr(buf)         (&((buf)->toaddr))
#define netbuf_set_destaddr(buf, destaddr) ip_addr_set(&((buf)->toaddr), destaddr)

#define netbuf_destport(buf)         (((buf)->flags & NETBUF_FLAG_DESTADDR) ? (buf)->toport_chksum : 0)
#else /* LWIP_CHECKSUM_ON_COPY */
#define netbuf_destport(buf)         ((buf)->toport_chksum)



#define netbuf_set_chksum(buf, chksum) do { (buf)->flags = NETBUF_FLAG_CHKSUM; \
                                            (buf)->toport_chksum = chksum; } while(0)



}





