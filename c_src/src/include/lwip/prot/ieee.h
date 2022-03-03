/**
 * @file
 * IEEE assigned numbers
 *
 * @defgroup ieee IEEE assigned numbers
 * @ingroup infrastructure
 */

/*
 * Copyright (c) 2017 Dirk Ziegelmeier.
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */







/**
 * @ingroup ieee
 * A list of often ethtypes (although lwIP does not use all of them).
 */
enum lwip_ieee_eth_type {
  /** Internet protocol v4 */
  ETHTYPE_IP        = 0x0800,
  /** Address resolution protocol */
  ETHTYPE_ARP       = 0x0806,
  /** Wake on lan */
  ETHTYPE_WOL       = 0x0842,
  /** RARP */
  ETHTYPE_RARP      = 0x8035,
  /** Virtual local area network */
  ETHTYPE_VLAN      = 0x8100,
  /** Internet protocol v6 */
  ETHTYPE_IPV6      = 0x86DD,
  /** PPP Over Ethernet Discovery Stage */
  ETHTYPE_PPPOEDISC = 0x8863,
  /** PPP Over Ethernet Session Stage */
  ETHTYPE_PPPOE     = 0x8864,
  /** Jumbo Frames */
  ETHTYPE_JUMBO     = 0x8870,
  /** Process field network */
  ETHTYPE_PROFINET  = 0x8892,
  /** Ethernet for control automation technology */
  ETHTYPE_ETHERCAT  = 0x88A4,
  /** Link layer discovery protocol */
  ETHTYPE_LLDP      = 0x88CC,
  /** Serial real-time communication system */
  ETHTYPE_SERCOS    = 0x88CD,
  /** Media redundancy protocol */
  ETHTYPE_MRP       = 0x88E3,
  /** Precision time protocol */
  ETHTYPE_PTP       = 0x88F7,
  /** Q-in-Q, 802.1ad */
  ETHTYPE_QINQ      = 0x9100
};




 /* LWIP_HDR_PROT_IEEE_H */
