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

// static NetIfc netif;

// #define NETIF_ADDRS ipaddr, netmask, gw,
// pub fn  init_default_netif( ipaddr: &mut ip4_addr,  netmask: &mut ip4_addr,  gw: &mut ip4_addr)

// #define NETIF_ADDRS
pub fn init_default_netif() {
    netif_add(&netif, NETIF_ADDRS, None, pcapif_init, netif_input);
    //  NO_SYS 
    netif_add(&netif, NETIF_ADDRS, None, pcapif_init, tcpip_input);

    netif_set_default(&netif);
}

pub fn default_netif_poll() {
    //  check for packets and link status
    pcapif_poll(&netif);
    /* When pcapif_poll comes back, there are not packets, so sleep to
    prevent 100% CPU load. Don't do this in an embedded system since it
    increases latency! */
    sys_msleep(1);
    //  !PCAPIF_RX_USE_THREAD 
    sys_msleep(50);
}

pub fn default_netif_shutdown() {
    //  release the pcap library... 
    pcapif_shutdown(&netif);
}
