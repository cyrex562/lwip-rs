/*
 *
 * @file tftp_server.c
 *
 * @author   Logan Gunthorpe <logang@deltatee.com>
 *           Dirk Ziegelmeier <dziegel@gmx.de>
 *
 * @brief    Trivial File Transfer Protocol (RFC 1350)
 *
 * Copyright (c) Deltatee Enterprises Ltd. 2013
 * All rights reserved.
 *
 */

/*
 * Redistribution and use in source and binary forms, with or without
 * modification,are permitted provided that the following conditions are met:
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
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Author: Logan Gunthorpe <logang@deltatee.com>
 *         Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */

/*
 * @defgroup tftp TFTP server
 * @ingroup apps
 *
 * This is simple TFTP server for the lwIP raw API.
 */









pub const TFTP_MAX_PAYLOAD_SIZE: u32 = 512; 
pub const TFTP_HEADER_LENGTH: u32 = 4; 

pub const TFTP_RRQ: u32 = 1; 
pub const TFTP_WRQ: u32 = 2; 
pub const TFTP_DATA: u32 = 3; 
pub const TFTP_ACK: u32 = 4; 
pub const TFTP_ERROR: u32 = 5; 

pub enum tftp_error {
  TFTP_ERROR_FILE_NOT_FOUND    = 1,
  TFTP_ERROR_ACCESS_VIOLATION  = 2,
  TFTP_ERROR_DISK_FULL         = 3,
  TFTP_ERROR_ILLEGAL_OPERATION = 4,
  TFTP_ERROR_UNKNOWN_TRFR_ID   = 5,
  TFTP_ERROR_FILE_EXISTS       = 6,
  TFTP_ERROR_NO_SUCH_USER      = 7
}



pub struct tftp_state {
 pub ctx: tftp_context,
  pub handle: Vec<u8>,
  pub last_data: PacketBuffer,
  pub upcb: udp_pcb,
  pub addr: LwipAddr,
  pub port: u16,
  pub lettimer: i32,
  pub letlast_pkt: i32,
  pub blknum: u16,
  pub retries: u8,
  pub mode_write: u8,
}

// static struct tftp_state tftp_state;

// pub fn tftp_tmr(arg: &mut Vec<u8>);

pub fn
close_handle()
{
  tftp_state.port = 0;
  ip_addr_set_any(0, &tftp_state.addr);

  if (tftp_state.last_data != None) {
    pbuf_free(tftp_state.last_data);
    tftp_state.last_data = None;
  }

  sys_untimeout(tftp_tmr, None);

  if (tftp_state.handle) {
    tftp_state.ctx.close(tftp_state.handle);
    tftp_state.handle = None;
//    LWIP_DEBUGF(TFTP_DEBUG | LWIP_DBG_STATE, ("tftp: closing\n"));
  }
}

pub fn
send_error( addr: &mut LwipAddr, port: u16, code: tftp_error, str: &String)
{
  let str_length: i32 = strlen(str);
  let p: &mut PacketBuffer;
  let mut payload: &mut u16;

  p = pbuf_alloc(PBUF_TRANSPORT, (TFTP_HEADER_LENGTH + str_length + 1), PBUF_RAM);
  if (p == None) {
    return;
  }

  payload =  p.payload;
  payload[0] = PP_HTONS(TFTP_ERROR);
  payload[1] = lwip_htons(code);
  MEMCPY(&payload[2], str, str_length + 1);

  udp_sendto(tftp_state.upcb, p, addr, port);
  pbuf_free(p);
}

pub fn
send_ack(blknum: u16)
{
  let p: &mut PacketBuffer;
  let mut payload: &mut u16;

  p = pbuf_alloc(PBUF_TRANSPORT, TFTP_HEADER_LENGTH, PBUF_RAM);
  if (p == None) {
    return;
  }
  payload =  p.payload;

  payload[0] = PP_HTONS(TFTP_ACK);
  payload[1] = lwip_htons(blknum);
  udp_sendto(tftp_state.upcb, p, &tftp_state.addr, tftp_state.port);
  pbuf_free(p);
}

pub fn
resend_data()
{
  let p: &mut PacketBuffer = pbuf_alloc(PBUF_TRANSPORT, tftp_state.last_data.len, PBUF_RAM);
  if (p == None) {
    return;
  }

  if (pbuf_copy(p, tftp_state.last_data) != ERR_OK) {
    pbuf_free(p);
    return;
  }

  udp_sendto(tftp_state.upcb, p, &tftp_state.addr, tftp_state.port);
  pbuf_free(p);
}

pub fn
send_data()
{
  let mut payload: &mut u16;
  let letret: i32;

  if (tftp_state.last_data != None) {
    pbuf_free(tftp_state.last_data);
  }

  tftp_state.last_data = pbuf_alloc(PBUF_TRANSPORT, TFTP_HEADER_LENGTH + TFTP_MAX_PAYLOAD_SIZE, PBUF_RAM);
  if (tftp_state.last_data == None) {
    return;
  }

  payload =  tftp_state.last_data.payload;
  payload[0] = PP_HTONS(TFTP_DATA);
  payload[1] = lwip_htons(tftp_state.blknum);

  ret = tftp_state.ctx.read(tftp_state.handle, &payload[2], TFTP_MAX_PAYLOAD_SIZE);
  if (ret < 0) {
    send_error(&tftp_state.addr, tftp_state.port, TFTP_ERROR_ACCESS_VIOLATION, "Error occured while reading the file.");
    close_handle();
    return;
  }

  pbuf_realloc(tftp_state.last_data, (TFTP_HEADER_LENGTH + ret));
  resend_data();
}

pub fn
recv(arg: &mut Vec<u8>, upcb: &mut udp_pcb, p: &mut PacketBuffer,  addr: &mut LwipAddr, port: u16)
{
  let sbuf: &mut u16 =  p.payload;
  let letopcode: i32;

  
  

  if (((tftp_state.port != 0) && (port != tftp_state.port)) ||
      (!ip_addr_isany_val(tftp_state.addr) && !ip_addr_cmp(&tftp_state.addr, addr))) {
    send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "Only one connection at a time is supported");
    pbuf_free(p);
    return;
  }

  opcode = sbuf[0];

  tftp_state.last_pkt = tftp_state.timer;
  tftp_state.retries = 0;


  match (PP_HTONS(opcode)) {
    TFTP_RRQ | //  fall through 
    TFTP_WRQ => {
      let tftp_None: char = 0;
      let filename: String;
      let mode: String;
      let filename_end_offset: u16;
      let mode_end_offset: u16;

      if (tftp_state.handle != None) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "Only one connection at a time is supported");
        
      }

      sys_timeout(TFTP_TIMER_MSECS, tftp_tmr, None);

      //  find \0 in pbuf -> end of filename string 
      filename_end_offset = pbuf_memfind(p, &tftp_None, sizeof(tftp_None), 2);
      if ((filename_end_offset - 1) > sizeof(filename)) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "Filename too long/not NULL terminated");
        
      }
      pbuf_copy_partial(p, filename, filename_end_offset - 1, 2);

      //  find \0 in pbuf -> end of mode string 
      mode_end_offset = pbuf_memfind(p, &tftp_None, sizeof(tftp_None), filename_end_offset + 1);
      if ((mode_end_offset - filename_end_offset) > sizeof(mode)) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "Mode too long/not NULL terminated");
        
      }
      pbuf_copy_partial(p, mode, mode_end_offset - filename_end_offset, filename_end_offset + 1);

      tftp_state.handle = tftp_state.ctx.open(filename, mode, opcode == PP_HTONS(TFTP_WRQ));
      tftp_state.blknum = 1;

      if (!tftp_state.handle) {
        send_error(addr, port, TFTP_ERROR_FILE_NOT_FOUND, "Unable to open requested file.");
        
      }

//      LWIP_DEBUGF(TFTP_DEBUG | LWIP_DBG_STATE, ("tftp: %s request from ", (opcode == PP_HTONS(TFTP_WRQ)) ? "write" : "read"));
      ip_addr_debug_print(TFTP_DEBUG | LWIP_DBG_STATE, addr);
//      LWIP_DEBUGF(TFTP_DEBUG | LWIP_DBG_STATE, (" for '%s' mode '%s'\n", filename, mode));

      ip_addr_copy(tftp_state.addr, *addr);
      tftp_state.port = port;

      if (opcode == PP_HTONS(TFTP_WRQ)) {
        tftp_state.mode_write = 1;
        send_ack(0);
      } else {
        tftp_state.mode_write = 0;
        send_data();
      }

      
    }

    TFTP_DATA=> {
      let letret: i32;
      let blknum: u16;

      if (tftp_state.handle == None) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "No connection");
        
      }

      if (tftp_state.mode_write != 1) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "Not a write connection");
        
      }

      blknum = lwip_ntohs(sbuf[1]);
      if (blknum == tftp_state.blknum) {
        pbuf_remove_header(p, TFTP_HEADER_LENGTH);

        ret = tftp_state.ctx.write(tftp_state.handle, p);
        if (ret < 0) {
          send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "error writing file");
          close_handle();
        } else {
          send_ack(blknum);
        }

        if (p.tot_len < TFTP_MAX_PAYLOAD_SIZE) {
          close_handle();
        } else {
          tftp_state.blknum+= 1;
        }
      } else if ((blknum + 1) == tftp_state.blknum) {
        //  retransmit of previous block, ack again (casting to to: u16 care for overflow) 
        send_ack(blknum);
      } else {
        send_error(addr, port, TFTP_ERROR_UNKNOWN_TRFR_ID, "Wrong block number");
      }
      
    }

    TFTP_ACK => {
      let blknum: u16;
      let letlastpkt: i32;

      if (tftp_state.handle == None) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "No connection");
        
      }

      if (tftp_state.mode_write != 0) {
        send_error(addr, port, TFTP_ERROR_ACCESS_VIOLATION, "Not a read connection");
        
      }

      blknum = lwip_ntohs(sbuf[1]);
      if (blknum != tftp_state.blknum) {
        send_error(addr, port, TFTP_ERROR_UNKNOWN_TRFR_ID, "Wrong block number");
        
      }

      lastpkt = 0;

      if (tftp_state.last_data != None) {
        lastpkt = tftp_state.last_data.tot_len != (TFTP_MAX_PAYLOAD_SIZE + TFTP_HEADER_LENGTH);
      }

      if (!lastpkt) {
        tftp_state.blknum+= 1;
        send_data();
      } else {
        close_handle();
      }

      
    }

    _ =>{
      send_error(addr, port, TFTP_ERROR_ILLEGAL_OPERATION, "Unknown operation");
      }
  }

  pbuf_free(p);
}

pub fn
tftp_tmr(arg: &mut Vec<u8>)
{
  

  tftp_state.timer+= 1;

  if (tftp_state.handle == None) {
    return;
  }

  sys_timeout(TFTP_TIMER_MSECS, tftp_tmr, None);

  if ((tftp_state.timer - tftp_state.last_pkt) > (TFTP_TIMEOUT_MSECS / TFTP_TIMER_MSECS)) {
    if ((tftp_state.last_data != None) && (tftp_state.retries < TFTP_MAX_RETRIES)) {
//      LWIP_DEBUGF(TFTP_DEBUG | LWIP_DBG_STATE, ("tftp: timeout, retrying\n"));
      resend_data();
      tftp_state.retries+= 1;
    } else {
//      LWIP_DEBUGF(TFTP_DEBUG | LWIP_DBG_STATE, ("tftp: timeout\n"));
      close_handle();
    }
  }
}

/* @ingroup tftp
 * Initialize TFTP server.
 * @param ctx TFTP callback struct
 */
pub fn 
tftp_init( ctx: &mut tftp_context)
{
  let ret: err_t;

  //  LWIP_ASSERT_CORE_LOCKED(); is checked by udp_new() 
  let pcb: &mut udp_pcb = udp_new_ip_type(IPADDR_TYPE_ANY);
  if (pcb == None) {
    return ERR_MEM;
  }

  ret = udp_bind(pcb, IP_ANY_TYPE, TFTP_PORT);
  if (ret != ERR_OK) {
    udp_remove(pcb);
    return ret;
  }

  tftp_state.handle    = None;
  tftp_state.port      = 0;
  tftp_state.ctx       = ctx;
  tftp_state.timer     = 0;
  tftp_state.last_data = None;
  tftp_state.upcb      = pcb;

  udp_recv(pcb, recv, None);

 return Ok(());
}

/* @ingroup tftp
 * Deinitialize ("turn off") TFTP server.
 */
pub fn  tftp_cleanup()
{
  LWIP_ASSERT("Cleanup called on non-initialized TFTP", tftp_state.upcb != None);
  udp_remove(tftp_state.upcb);
  close_handle();
  //memset(&tftp_state, 0, sizeof(tftp_state));
}


