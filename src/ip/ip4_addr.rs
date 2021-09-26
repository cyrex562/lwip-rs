/*
 * @file
 * This is the IPv4 address tools implementation.
 *
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

/* used by IP4_ADDR_ANY and IP_ADDR_BROADCAST in ip_addr.h */
// const LwipAddr ip_addr_any = IPADDR4_INIT(IPADDR_ANY);
// const LwipAddr ip_addr_broadcast = IPADDR4_INIT(IPADDR_BROADCAST);

/*
 * Determine if an address is a broadcast address on a network interface
 *
 * @param addr address to be checked
 * @param netif the network interface against which the address is checked
 * @return returns non-zero if the address is a broadcast address
 */
pub fn ip4_addr_isbroadcast_u32(addr: u32, netif: &mut NetIfc) -> u8 {
    let mut if_addr: LwipAddr;
    ip4_addr_set_u32(&ipaddr, addr);

    /* all ones (broadcast) or all zeroes (old skool broadcast) */
    if ((!addr == IPADDR_ANY) || (addr == IPADDR_ANY)) {
        return 1;
        /* no broadcast support on this network interface? */
    } else if ((netif.flags & NETIF_FLAG_BROADCAST) == 0) {
        /* the given address cannot be a broadcast address
         * nor can we check against any broadcast addresses */
        return 0;
        /* address matches network interface address exactly? => no broadcast */
    } else if (addr == ip4_addr_get_u32(netif_ip4_addr(netif))) {
        return 0;
        /*  on the same (sub) network... */
    } else if (ip4_addr_netcmp(&ipaddr, netif_ip4_addr(netif), netif_ip4_netmask(netif))
             /* ...and host identifier bits are all ones? =>... */
             && ((addr & !ip4_addr_get_u32(netif_ip4_netmask(netif))) ==
                 (IPADDR_BROADCAST & !ip4_addr_get_u32(netif_ip4_netmask(netif)))))
    {
        /* => network broadcast address */
        return 1;
    } else {
        return 0;
    }
}

/* Checks if a netmask is valid (starting with ones, then only zeros)
 *
 * @param netmask the IPv4 netmask to check (in network byte order!)
 * @return 1 if the netmask is valid, 0 if it is not
 */
pub fn ip4_addr_netmask_valid(netmask: u32) -> u8 {
    let mask: u32;
    let nm_hostorder: u32 = lwip_htonl(netmask);

    /* first, check for the first zero */
    // for (mask = 1 << 31 ; mask != 0; mask >>= 1) {
    //   if ((nm_hostorder & mask) == 0) {
    //     break;
    //   }
    // }
    /* then check that there is no one */
    // for (; mask != 0; mask >>= 1) {
    //   if ((nm_hostorder & mask) != 0) {
    //     /* there is a one after the first zero -> invalid */
    //     return 0;
    //   }
    // }
    /* no one after the first zero -> valid */
    return 1;
}

/*
 * Ascii internet address interpretation routine.
 * The value returned is in network order.
 *
 * @param cp IP address in ascii representation (e.g. "127.0.0.1")
 * @return ip address in network order
 */
pub fn ipaddr_addr(cp: &String) -> LwipAddr {
    unimplemented!()
}

/*
 * Check whether "cp" is a valid ascii representation
 * of an Internet address and convert to a binary address.
 * Returns 1 if the address is valid, 0 if not.
 * This replaces inet_addr, the return value from which
 * cannot distinguish between failure and a local broadcast address.
 *
 * @param cp IP address in ascii representation (e.g. "127.0.0.1")
 * @param addr pointer to which to save the ip address in network order
 * @return 1 if cp could be converted to addr, 0 on failure
 */
pub fn ip4addr_aton(cp: &String) -> LwipAddr {
    let val: u32;
    let base: u8;
    let c: u8;
    let parts: [u32; 4];
    u32 * pp = parts;

    unimplemented!()
}

/*
 * Convert numeric IP address into decimal dotted ASCII representation.
 * returns ptr to static buffer; not reentrant!
 *
 * @param addr ip address in network order to convert
 * @return pointer to a global static (!) buffer that holds the ASCII
 *         representation of addr
 */
pub fn ip4addr_ntoa(addr: &LwipAddr) -> String {
    return ip4addr_ntoa_r(addr);
}

/*
 * Same as ip4addr_ntoa, but reentrant since a user-supplied buffer is used.
 *
 * @param addr ip address in network order to convert
 * @param buf target buffer where the string is stored
 * @param buflen length of buf
 * @return either pointer to buf which now holds the ASCII
 *         representation of addr or NULL if buf was too small
 */
pub fn ip4addr_ntoa_r(addr: &LwipAddr) -> String {
    unimplemented!()
}
