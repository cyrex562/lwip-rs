/*
 * Copyright (c) 2018 Simon Goldschmidt
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 * To use the hooks in this file, make sure this file is included in LWIP_HOOK_FILENAME
 * and define these hooks:
 *
 * // #define LWIP_HOOK_TCP_INPACKET_PCB(pcb, hdr, optlen, opt1len, opt2, p) tcp_md5_check_inpacket(pcb, hdr, optlen, opt1len, opt2, p)
 * // #define LWIP_HOOK_TCP_OPT_LENGTH_SEGMENT(pcb, internal_len)            tcp_md5_get_additional_option_length(pcb, internal_len)
 * // #define LWIP_HOOK_TCP_ADD_TX_OPTIONS(p, hdr, pcb, opts)                tcp_md5_add_tx_options(p, hdr, pcb,  opts)
 *
 * // #define LWIP_HOOK_SOCKETS_SETSOCKOPT(s, sock, level, optname, optval, optlen, err) tcp_md5_setsockopt_hook(sock, level, optname, optval, optlen, err)
 */


// #define LWIP_HDR_CONTRIB_ADDONS_TCP_MD5_H












/* setsockopt definitions and structs: */

/* This is the optname (for level = IPPROTO_TCP) */

#define TCP_MD5SIG 14


#define TCP_MD5SIG_MAXKEYLEN 80

/* This is the optval type */
struct tcp_md5sig {
  struct  sockaddr_storage tcpm_addr;
  u16   __tcpm_pad1;
  u16   tcpm_keylen;
  u32   __tcpm_pad2;
  u8    tcpm_key[TCP_MD5SIG_MAXKEYLEN];
};

/* socket setsockopt hook: */
tcp_md5_setsockopt_hook: int(sock: &mut lwip_sock, level: int, optname: int, optval: &Vec<u8>, optlen: u32, int *err);

/* Internal hook functions */
pub fn  tcp_md5_init();
pub fn  tcp_md5_check_inpacket(struct tcp_pcb* pcb, hdr: &mut tcp_hdr, optlen: u16, opt1len: u16, u8 *opt2, p: &mut pbuf);
tcp_md5_get_additional_option_length: u8(const pcb: &mut tcp_pcb, internal_option_length: u8);
u32 *tcp_md5_add_tx_options(p: &mut pbuf, hdr: &mut tcp_hdr, const pcb: &mut tcp_pcb, u32 *opts);


}



