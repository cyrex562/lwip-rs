/*
 * @file
 * RTP client/server module
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














/* This is an example of a "RTP" client/server based on a MPEG4 bitstream (with socket API).
 */

/*
 * RTP_DEBUG: Enable debugging for RTP.
 */



/* RTP stream port */

#define RTP_STREAM_PORT             4000


/* RTP stream multicast address as IPv4 address in "u32" format */

#define RTP_STREAM_ADDRESS          inet_addr("232.0.0.0")


/* RTP send delay - in milliseconds */

#define RTP_SEND_DELAY              40


/* RTP receive timeout - in milliseconds */

#define RTP_RECV_TIMEOUT            2000


/* RTP stats display period - in received packets */

#define RTP_RECV_STATS              50


/* RTP macro to let the application process the data */

#define RTP_RECV_PROCESSING(p,s)


/* RTP packet/payload size */
#define RTP_PACKET_SIZE             1500
#define RTP_PAYLOAD_SIZE            1024

/* RTP header constants */
pub const RTP_VERSION: u32 = 0x80;
#define RTP_TIMESTAMP_INCREMENT     3600
pub const RTP_SSRC: u32 = 0;
#define RTP_PAYLOADTYPE             96
pub const RTP_MARKER_MASK: u32 = 0x80;

/* RTP message header */




struct rtp_hdr {
  (u8  version);
  (u8  payloadtype);
  seqNum: u16,
  timestamp: u32,
  ssrc: u32,
} ;





/* RTP packets */
static rtp_send_packet: [u8;RTP_PACKET_SIZE];
static rtp_recv_packet: [u8;RTP_PACKET_SIZE];

/*
 * RTP send packets
 */
pub fn
rtp_send_packets( sock: i32, struct sockaddr_in* to)
{
  struct rtp_hdr* rtphdr;
  u8*           rtp_payload;
  usize          rtp_payload_size;
  usize          rtp_data_index;

  /* prepare RTP packet */
  rtphdr = (struct rtp_hdr*)rtp_send_packet;
  rtphdr.version     = RTP_VERSION;
  rtphdr.payloadtype = 0;
  rtphdr.ssrc        = PP_HTONL(RTP_SSRC);
  rtphdr.timestamp   = lwip_htonl(lwip_ntohl(rtphdr.timestamp) + RTP_TIMESTAMP_INCREMENT);

  /* send RTP stream packets */
  rtp_data_index = 0;
  loop {
    rtp_payload      = rtp_send_packet+sizeof(struct rtp_hdr);
    rtp_payload_size = LWIP_MIN(RTP_PAYLOAD_SIZE, sizeof(rtp_data) - rtp_data_index);

    MEMCPY(rtp_payload, rtp_data + rtp_data_index, rtp_payload_size);

    /* set MARKER bit in RTP header on the last packet of an image */
    if ((rtp_data_index + rtp_payload_size) >= sizeof(rtp_data)) {
      rtphdr.payloadtype = RTP_PAYLOADTYPE | RTP_MARKER_MASK;
    } else {
      rtphdr.payloadtype = RTP_PAYLOADTYPE;
    }

    /* send RTP stream packet */
    if (lwip_sendto(sock, rtp_send_packet, sizeof(struct rtp_hdr) + rtp_payload_size,
        0, to, sizeof(struct sockaddr)) >= 0) {
      rtphdr.seqNum  = lwip_htons((lwip_ntohs(rtphdr.seqNum) + 1));
      rtp_data_index += rtp_payload_size;
    } else {
//      LWIP_DEBUGF(RTP_DEBUG, ("rtp_sender: not sendto==%i\n", errno));
    }
  }while (rtp_data_index < sizeof(rtp_data));
}

/*
 * RTP send thread
 */
pub fn
rtp_send_thread(arg: &mut Vec<u8>)
{
  int                sock;
  local: sockaddr_in;
  to: sockaddr_in;
  u32              rtp_stream_address;

  

  /* initialize RTP stream address */
  rtp_stream_address = RTP_STREAM_ADDRESS;

  /* if we got a valid RTP stream address... */
  if (rtp_stream_address != 0) {
    /* create new socket */
    sock = lwip_socket(AF_INET, SOCK_DGRAM, 0);
    if (sock >= 0) {
      /* prepare local address */
      memset(&local, 0, sizeof(local));
      local.sin_family      = AF_INET;
      local.sin_port        = PP_HTONS(INADDR_ANY);
      local.sin_addr.s_addr = PP_HTONL(INADDR_ANY);

      /* bind to local address */
      if (lwip_bind(sock, &local, sizeof(local)) == 0) {
        /* prepare RTP stream address */
        memset(&to, 0, sizeof(to));
        to.sin_family      = AF_INET;
        to.sin_port        = PP_HTONS(RTP_STREAM_PORT);
        to.sin_addr.s_addr = rtp_stream_address;

        /* send RTP packets */
        memset(rtp_send_packet, 0, sizeof(rtp_send_packet));
        loop {
          rtp_send_packets( sock, &to);
          sys_msleep(RTP_SEND_DELAY);
        }
      }

      /* close the socket */
      lwip_close(sock);
    }
  }
}

/*
 * RTP recv thread
 */
pub fn
rtp_recv_thread(arg: &mut Vec<u8>)
{
  int                sock;
  local: sockaddr_in;
  from: sockaddr_in;
  int                fromlen;
  struct ip_mreq     ipmreq;
  struct rtp_hdr*    rtphdr;
  u32              rtp_stream_address;
  int                timeout;
  int                result;
  int                recvrtppackets  = 0;
  int                lostrtppackets  = 0;
  u16              lastrtpseq = 0;

  

  /* initialize RTP stream address */
  rtp_stream_address = RTP_STREAM_ADDRESS;

  /* if we got a valid RTP stream address... */
  if (rtp_stream_address != 0) {
    /* create new socket */
    sock = lwip_socket(AF_INET, SOCK_DGRAM, 0);
    if (sock >= 0) {
      /* prepare local address */
      memset(&local, 0, sizeof(local));
      local.sin_family      = AF_INET;
      local.sin_port        = PP_HTONS(RTP_STREAM_PORT);
      local.sin_addr.s_addr = PP_HTONL(INADDR_ANY);

      /* bind to local address */
      if (lwip_bind(sock, &local, sizeof(local)) == 0) {
        /* set recv timeout */
        timeout = RTP_RECV_TIMEOUT;
        result = lwip_setsockopt(sock, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
        if (result) {
//          LWIP_DEBUGF(RTP_DEBUG, ("rtp_recv_thread: setsockopt(SO_RCVTIMEO) failed: errno=%d\n", errno));
        }

        /* prepare multicast "ip_mreq" struct */
        ipmreq.imr_multiaddr.s_addr = rtp_stream_address;
        ipmreq.imr_interface.s_addr = PP_HTONL(INADDR_ANY);

        /* join multicast group */
        if (lwip_setsockopt(sock, IPPROTO_IP, IP_ADD_MEMBERSHIP, &ipmreq, sizeof(ipmreq)) == 0) {
          /* receive RTP packets */
          while(1) {
            fromlen = sizeof(from);
            result  = lwip_recvfrom(sock, rtp_recv_packet, sizeof(rtp_recv_packet), 0,
              &from, (socklen_t *)&fromlen);
            if ((result > 0) && (result >= sizeof(struct rtp_hdr))) {
              recved: usize = result;
              rtphdr = (struct rtp_hdr *)rtp_recv_packet;
              recvrtppackets+= 1;
              if ((lastrtpseq == 0) || ((lastrtpseq + 1) == lwip_ntohs(rtphdr.seqNum))) {
                RTP_RECV_PROCESSING((rtp_recv_packet + sizeof(rtp_hdr)), (recved-sizeof(rtp_hdr)));
                 /* just in case... */
              } else {
                lostrtppackets+= 1;
              }
              lastrtpseq = lwip_ntohs(rtphdr.seqNum);
              if ((recvrtppackets % RTP_RECV_STATS) == 0) {
//                LWIP_DEBUGF(RTP_DEBUG, ("rtp_recv_thread: recv %6i packet(s) / lost %4i packet(s) (%.4f%%)...\n", recvrtppackets, lostrtppackets, (lostrtppackets*100.0)/recvrtppackets));
              }
            } else {
//              LWIP_DEBUGF(RTP_DEBUG, ("rtp_recv_thread: recv timeout...\n"));
            }
          }

          /* leave multicast group */
          /* TODO: this code is never reached
          result = lwip_setsockopt(sock, IPPROTO_IP, IP_DROP_MEMBERSHIP, &ipmreq, sizeof(ipmreq));
          if (result) {
//            LWIP_DEBUGF(RTP_DEBUG, ("rtp_recv_thread: setsockopt(IP_DROP_MEMBERSHIP) failed: errno=%d\n", errno));
          }*/
        }
      }

      /* close the socket */
      lwip_close(sock);
    }
  }
}

pub fn 
rtp_init()
{
  sys_thread_new("rtp_send_thread", rtp_send_thread, NULL, DEFAULT_THREAD_STACKSIZE, DEFAULT_THREAD_PRIO);
  sys_thread_new("rtp_recv_thread", rtp_recv_thread, NULL, DEFAULT_THREAD_STACKSIZE, DEFAULT_THREAD_PRIO);
}


