/*
 * @file
 * Ethernet Interface Skeleton
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

/*
 * This file is a skeleton for developing Ethernet network interface
 * drivers for lwIP. Add code to the low_level functions and do a
 * search-and-replace for the word "ethernetif" to replace it with
 * something that better describes your network interface.
 */

//  Define those to better describe your network interface. 
pub const IFNAME0: String = 'e'.to_string();
pub const IFNAME1: String = 'n'.to_string();

/*
 * Helper struct to hold private data used to operate your ethernet interface.
 * Keeping the ethernet address of the MAC in this struct is not necessary
 * as it is already kept in the NetIfc.
 * But this is only an example, anyway...
 */
pub struct ethernetif {
    pub ethaddr: eth_addr,
    //  Add whatever per-interface state that is needed here. 
}

//  Forward declarations. 
fn ethernetif_input(netif: &mut NetIfc);

/*
 * In this function, the hardware should be initialized.
 * Called from ethernetif_init().
 *
 * @param netif the already initialized lwip network interface structure
 *        for this ethernetif
 */
pub fn low_level_init(netif: &mut NetIfc) {
    let ethernetif: &mut ethernetif = netif.state;

    //  set MAC hardware address length 
    netif.hwaddr_len = ETHARP_HWADDR_LEN;

    //  set MAC hardware address 
    netif.hwaddr[0] = 0;

    netif.hwaddr[5] = 0;

    //  maximum transfer unit 
    netif.mtu = 1500;

    //  device capabilities 
    //  don't set NETIF_FLAG_ETHARP if this device is not an ethernet one 
    netif.flags = NETIF_FLAG_BROADCAST | NETIF_FLAG_ETHARP | NETIF_FLAG_LINK_UP;

    /*
     * For hardware/netifs that implement MAC filtering.
     * All-nodes link-local is handled by default, so we must let the hardware know
     * to allow multicast packets in.
     * Should set mld_mac_filter previously. */
    if (netif.mld_mac_filter != None) {
        let ip6_allnodes_ll: ip6_addr_t;
        ip6_addr_set_allnodes_linklocal(&ip6_allnodes_ll);
        netif.mld_mac_filter(netif, &ip6_allnodes_ll, NETIF_ADD_MAC_FILTER);
    }

    //  Do whatever else is needed to initialize interface. 
}

/*
 * This function should do the actual transmission of the packet. The packet is
 * contained in the pbuf that is passed to the function. This pbuf
 * might be chained.
 *
 * @param netif the lwip network interface structure for this ethernetif
 * @param p the MAC packet to send (e.g. IP packet including MAC addresses and type)
 * @return ERR_OK if the packet could be sent
 *         an value: err_t if the packet couldn't be sent
 *
 * @note Returning ERR_MEM here if a DMA queue of your MAC is full can lead to
 *       strange results. You might consider waiting for space in the DMA queue
 *       to become available since the stack doesn't retry to send a packet
 *       dropped because of memory failure (except for the TCP timers).
 */

pub fn low_level_output(netif: &mut NetIfc, p: &mut PacketBuffer) -> Result<(), LwipError> {
    let ethernetif: &mut ethernetif = netif.state;
    let q: &mut PacketBuffer;

    // initiate transfer();

    pbuf_remove_header(p, ETH_PAD_SIZE); //  drop the padding word 

    // for (q = p; q != NULL; q = q.next) {
    //   /* Send the data from the pbuf to the interface, one pbuf at a
    //      time. The size of the data in each pbuf is kept in the .len
    //      variable. */
    //   send data from(q.payload, q.len);
    // }

    // signal that packet should be sent();

    MIB2_STATS_NETIF_ADD(netif, ifoutoctets, p.tot_len);
    if ((p.payload)[0] & 1) {
        //  broadcast or multicast packet
        MIB2_STATS_NETIF_INC(netif, ifoutnucastpkts);
    } else {
        //  unicast packet 
        MIB2_STATS_NETIF_INC(netif, ifoutucastpkts);
    }
    //  increase ifoutdiscards or ifouterrors on error 

    pbuf_add_header(p, ETH_PAD_SIZE); //  reclaim the padding word 

    LINK_STATS_INC(link.xmit);

   return Ok(());
}

/*
 * Should allocate a pbuf and transfer the bytes of the incoming
 * packet from the interface into the pbuf.
 *
 * @param netif the lwip network interface structure for this ethernetif
 * @return a pbuf filled with the received packet (including MAC header)
 *         NULL on memory error
 */
pub fn low_level_input(netif: &mut NetIfc) -> PacketBuffer {
    let ethernetif: &mut ethernetif = netif.state;
    let p: &mut PacketBuffer;
    let q: &mut PacketBuffer;
    let len: usize;

    /* Obtain the size of the packet and put it into the "len"
    variable. */
    len = 0;

    len += ETH_PAD_SIZE; //  allow room for Ethernet padding 

    //  We allocate a pbuf chain of pbufs from the pool. 
    p = pbuf_alloc(PBUF_RAW, len, PBUF_POOL);

    if (p != None) {
        pbuf_remove_header(p, ETH_PAD_SIZE); //  drop the padding word 

        /* We iterate over the pbuf chain until we have read the entire
         * packet into the pbuf. */
        // for (q = p; q != NULL; q = q.next) {
        //   /* Read enough bytes to fill this pbuf in the chain. The
        //    * available data in the pbuf is given by the q.len
        //    * variable.
        //    * This does not necessarily have to be a memcpy, you can also preallocate
        //    * pbufs for a DMA-enabled MAC and after receiving truncate it to the
        //    * actually received size. In this case, ensure the tot_len member of the
        //    * pbuf is the sum of the chained pbuf len members.
        //    */
        //   read data into(q.payload, q.len);
        // }
        // acknowledge that packet has been read();

        MIB2_STATS_NETIF_ADD(netif, ifinoctets, p.tot_len);
        if ((p.payload)[0] & 1) {
            //  broadcast or multicast packet
            MIB2_STATS_NETIF_INC(netif, ifinnucastpkts);
        } else {
            //  unicast packet
            MIB2_STATS_NETIF_INC(netif, ifinucastpkts);
        }

        pbuf_add_header(p, ETH_PAD_SIZE); //  reclaim the padding word 

        LINK_STATS_INC(link.recv);
    } else {
        // drop packet();
        LINK_STATS_INC(link.memerr);
        LINK_STATS_INC(link.drop);
        MIB2_STATS_NETIF_INC(netif, ifindiscards);
    }

    return p;
}

/*
 * This function should be called when a packet is ready to be read
 * from the interface. It uses the function low_level_input() that
 * should handle the actual reception of bytes from the network
 * interface. Then the type of the received packet is determined and
 * the appropriate input function is called.
 *
 * @param netif the lwip network interface structure for this ethernetif
 */
pub fn ethernetif_input(netif: &mut NetIfc) {
    let ethernetif: &mut ethernetif;
    let ethhdr: &mut eth_hdr;
    let p: &mut PacketBuffer;

    ethernetif = netif.state;

    //  move received packet into a new pbuf 
    p = low_level_input(netif);
    //  if no packet could be read, silently ignore this 
    if (p != None) {
        //  pass all packets to ethernet_input, which decides what packets it supports 
        if (netif.input(p, netif) != ERR_OK) {
//            LWIP_DEBUGF(NETIF_DEBUG, ("ethernetif_input: IP input error\n"));
            pbuf_free(p);
            p = None;
        }
    }
}

/*
 * Should be called at the beginning of the program to set up the
 * network interface. It calls the function low_level_init() to do the
 * actual setup of the hardware.
 *
 * This function should be passed as a parameter to netif_add().
 *
 * @param netif the lwip network interface structure for this ethernetif
 * @return ERR_OK if the loopif is initialized
 *         ERR_MEM if private data couldn't be allocated
 *         any other on: err_t error
 */
pub fn ethernetif_init(netif: &mut NetIfc) {
    let ethernetif: &mut ethernetif;

    // LWIP_ASSERT("netif != NULL", (netif != None));

    ethernetif = mem_malloc(sizeof(ethernetif));
    if (ethernetif == None) {
//        LWIP_DEBUGF(NETIF_DEBUG, ("ethernetif_init: out of memory\n"));
        return ERR_MEM;
    }

    //  Initialize interface hostname 
    netif.hostname = "lwip";

    /*
     * Initialize the snmp variables and counters inside the NetIfc.
     * The last argument should be replaced with your link speed, in units
     * of bits per second.
     */
    MIB2_INIT_NETIF(
        netif,
        snmp_ifType_ethernet_csmacd,
        LINK_SPEED_OF_YOUR_NETIF_IN_BPS,
    );

    netif.state = ethernetif;
    netif.name[0] = IFNAME0;
    netif.name[1] = IFNAME1;
    /* We directly use etharp_output() here to save a function call.
     * You can instead declare your own function an call etharp_output()
     * from it if you have to do some checks before sending (e.g. if link
     * is available...) */

    netif.output = etharp_output;

    netif.output_ip6 = ethip6_output;

    netif.linkoutput = low_level_output;

    ethernetif.ethaddr = &(netif.hwaddr[0]);

    //  initialize the hardware 
    low_level_init(netif);

   return Ok(());
}
