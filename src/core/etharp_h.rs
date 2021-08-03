/*
 * @file
 * ARP protocol definitions
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

// #define LWIP_HDR_PROT_ETHARP_H









#define ETHARP_HWADDR_LEN     ETH_HWADDR_LEN


/*
 * struct ip4_addr_wordaligned is used in the definition of the ARP packet format in
 * order to support compilers that don't have structure packing.
 */

#  include "arch/bpstruct.h"


struct ip4_addr_wordaligned {
  (addrw: u16[2]);
} ;


#  include "arch/epstruct.h"


/* MEMCPY-like copying of IP addresses where addresses are known to be
 * 16-bit-aligned if the port is correctly configured (so a port could define
 * this to copying 2 u16's) - no NULL-pointer-checking needed. */

#define IPADDR_WORDALIGNED_COPY_TO_ip4_addr(dest, src) SMEMCPY(dest, src, sizeof(ip4_addr))


 /* MEMCPY-like copying of IP addresses where addresses are known to be
 * 16-bit-aligned if the port is correctly configured (so a port could define
 * this to copying 2 u16's) - no NULL-pointer-checking needed. */

#define IPADDR_WORDALIGNED_COPY_FROM_ip4_addr(dest, src) SMEMCPY(dest, src, sizeof(ip4_addr))



#  include "arch/bpstruct.h"


/* the ARP message, see RFC 826 ("Packet format") */
struct etharp_hdr {
  (hwtype: u16);
  (proto: u16);
  (u8  hwlen);
  (u8  protolen);
  (opcode: u16);
  (struct eth_addr shwaddr);
  (struct ip4_addr_wordaligned sipaddr);
  (struct eth_addr dhwaddr);
  (struct ip4_addr_wordaligned dipaddr);
} ;


#  include "arch/epstruct.h"


#define SIZEOF_ETHARP_HDR 28

/* ARP message types (opcodes) */
enum etharp_opcode {
  ARP_REQUEST = 1,
  ARP_REPLY   = 2
};


}



