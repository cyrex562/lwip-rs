/*
 * @file
 * SLIP Interface
 *
 */

/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * This file is built upon the file: src/arch/rtxc/netif/sioslip.c
 *
 * Author: Magnus Ivarsson <magnus.ivarsson(at)volvo.com>
 *         Simon Goldschmidt
 */

/*
 * @defgroup slipif SLIP
 * @ingroup netifs
 *
 * This is an arch independent SLIP netif. The specific serial hooks must be
 * provided by another file. They are sio_open, sio_read/sio_tryread and sio_send
 *
 * Usage: This netif can be used in three ways:\n
 *        1) For NO_SYS==0, an RX thread can be used which blocks on sio_read()
 *           until data is received.\n
 *        2) In your main loop, call slipif_poll() to check for new RX bytes,
 *           completed packets are fed into netif.input().\n
 *        3) Call slipif_received_byte[s]() from your serial RX ISR and
 *           slipif_process_rxqueue() from your main loop. ISR level decodes
 *           packets and puts completed packets on a queue which is fed into
 *           the stack from the main loop (needs SYS_LIGHTWEIGHT_PROT for
 *           pbuf_alloc to work on ISR level!).
 *
 */

pub const SLIP_END: u32 = 0xC0; //  0300: start and end of every packet 
pub const SLIP_ESC: u32 = 0xDB; //  0333: escape start (one byte escaped data follows) 
pub const SLIP_ESC_END: u32 = 0xDC; //  0334: following escape: original byte is 0xC0 (END) 
pub const SLIP_ESC_ESC: u32 = 0xDD; //  0335: following escape: original byte is 0xDB (ESC) 

//  Maximum packet size that is received by this netif 

pub const SLIP_MAX_SIZE: u32 = 1500;

/* Define this to the interface speed for SNMP
 * (sio_fd is the sio_fd_t returned by sio_open).
 * The default value of zero means 'unknown'.
 */

// #define SLIP_SIO_SPEED(sio_fd) 0

pub enum slipif_recv_state {
    SLIP_RECV_NORMAL,
    SLIP_RECV_ESCAPE,
}

pub struct slipif_priv {
    pub sd: sio_fd_t,
    //  q is the whole pbuf chain for a packet, p is the current pbuf in the chain 
    pub p: PacketBuffer,
    pub q: PacketBuffer,
    pub state: u8,
    pub i: u16,
    pub recved: u16,
    pub rxpackets: PacketBuffer,
}

/*
 * Send a pbuf doing the necessary SLIP encapsulation
 *
 * Uses the serial layer's sio_send()
 *
 * @param netif the lwip network interface structure for this slipif
 * @param p the pbuf chain packet to send
 * @return always returns ERR_OK since the serial layer does not provide return values
 */
pub fn slipif_output(netif: &mut NetIfc, p: &mut PacketBuffer) -> Result<(), LwipError> {
    let mut priv_if: &mut slipif_priv;
    let q: &mut PacketBuffer;
    let i: u16;
    let c: u8;

    LWIP_ASSERT("netif != NULL", (netif != None));
    LWIP_ASSERT("netif.state != NULL", (netif.state != None));
    LWIP_ASSERT("p != NULL", (p != None));

    //  LWIP_DEBUGF(SLIP_DEBUG, ("slipif_output: sending %"U16_F" bytes\n", p.tot_len));
    priv_if = netif.state;

    //  Send pbuf out on the serial I/O device. 
    //  Start with packet delimiter. 
    sio_send(SLIP_END, priv_if.sd);

    // for (q = p; q != None; q = q.next) {
    //   for (i = 0; i < q.len; i+= 1) {
    //     c = (q.payload)[i];
    //     match (c) {
    //       SLIP_END =>
    //         //  need to escape this byte (0xC0 -> 0xDB, 0xDC) 
    //         sio_send(SLIP_ESC, priv_if.sd);
    //         sio_send(SLIP_ESC_END, priv_if.sd);
    //         break;
    //       SLIP_ESC =>
    //         //  need to escape this byte (0xDB -> 0xDB, 0xDD) 
    //         sio_send(SLIP_ESC, priv_if.sd);
    //         sio_send(SLIP_ESC_ESC, priv_if.sd);
    //         break;
    //       _ =>
    //         //  normal byte - no need for escaping 
    //         sio_send(c, priv_if.sd);
    //         break;
    //     }
    //   }
    // }
    //  End with packet delimiter. 
    sio_send(SLIP_END, priv_if.sd);
    return Ok(());
}

/*
 * Send a pbuf doing the necessary SLIP encapsulation
 *
 * Uses the serial layer's sio_send()
 *
 * @param netif the lwip network interface structure for this slipif
 * @param p the pbuf chain packet to send
 * @param ipaddr the ip address to send the packet to (not used for slipif)
 * @return always returns ERR_OK since the serial layer does not provide return values
 */
pub fn slipif_output_v4(
    netif: &mut NetIfc,
    p: &mut PacketBuffer,
    ipaddr: &mut LwipAddr,
) -> Result<(), LwipError> {
    return slipif_output(netif, p);
}

/*
 * Send a pbuf doing the necessary SLIP encapsulation
 *
 * Uses the serial layer's sio_send()
 *
 * @param netif the lwip network interface structure for this slipif
 * @param p the pbuf chain packet to send
 * @param ipaddr the ip address to send the packet to (not used for slipif)
 * @return always returns ERR_OK since the serial layer does not provide return values
 */
pub fn slipif_output_v6(
    netif: &mut NetIfc,
    p: &mut PacketBuffer,
    ipaddr: &mut ip6_addr_t,
) -> Result<(), LwipError> {
    return slipif_output(netif, p);
}

/*
 * Handle the incoming SLIP stream character by character
 *
 * @param netif the lwip network interface structure for this slipif
 * @param c received character (multiple calls to this function will
 *        return a complete packet, NULL is returned before - used for polling)
 * @return The IP packet when SLIP_END is received
 */
pub fn slipif_rxbyte(netif: &mut NetIfc, c: u8) -> PacketBuffer {
    let mut priv_if: &mut slipif_priv;
    let t: &mut PacketBuffer;

    LWIP_ASSERT("netif != NULL", (netif != None));
    LWIP_ASSERT("netif.state != NULL", (netif.state != None));

    priv_if = netif.state;

    match (priv_if.state) {
        SLIP_RECV_NORMAL => match (c) {
            SLIP_END => {
                if (priv_if.recved > 0) {
                    //  Received whole packet. 
                    //  Trim the pbuf to the size of the received packet. 
                    pbuf_realloc(priv_if.q, priv_if.recved);

                    LINK_STATS_INC(link.recv);

                    //            LWIP_DEBUGF(SLIP_DEBUG, ("slipif: Got packet (%"U16_F" bytes)\n", priv_if.recved));
                    t = priv_if.q;
                    priv_if.p = priv_if.q = None;
                    priv_if.i = priv_if.recved = 0;
                    return t;
                }
                return None;
            }
            SLIP_ESC => {
                priv_if.state = SLIP_RECV_ESCAPE;
                return None;
            }
            _ => {}
        }, //  end match (c) 
        // break;
        SLIP_RECV_ESCAPE => {
            /* un-escape END or ESC bytes, leave other bytes
            (although that would be a protocol error) */
            match (c) {
                SLIP_ESC_END => {
                    c = SLIP_END;
                }

                SLIP_ESC_ESC => {
                    c = SLIP_ESC;
                }

                _ => {}
            }
            priv_if.state = SLIP_RECV_NORMAL;
        }
        _ => {}
    } //  end match (priv_if.state) 

    //  byte received, packet not yet completely received 
    if (priv_if.p == None) {
        //  allocate a new pbuf 
        //    LWIP_DEBUGF(SLIP_DEBUG, ("slipif_input: alloc\n"));
        priv_if.p = pbuf_alloc(
            PBUF_LINK,
            (PBUF_POOL_BUFSIZE - PBUF_LINK_HLEN - PBUF_LINK_ENCAPSULATION_HLEN),
            PBUF_POOL,
        );

        if (priv_if.p == None) {
            LINK_STATS_INC(link.drop);
            //      LWIP_DEBUGF(SLIP_DEBUG, ("slipif_input: no new pbuf! (DROP)\n"));
            //  don't process any further since we got no pbuf to receive to 
            return None;
        }

        if (priv_if.q != None) {
            //  'chain' the pbuf to the existing chain 
            pbuf_cat(priv_if.q, priv_if.p);
        } else {
            //  p is the first pbuf in the chain 
            priv_if.q = priv_if.p;
        }
    }

    //  this automatically drops bytes if > SLIP_MAX_SIZE 
    if ((priv_if.p != None) && (priv_if.recved <= SLIP_MAX_SIZE)) {
        (priv_if.p.payload)[priv_if.i] = c;
        priv_if.recved += 1;
        priv_if.i += 1;
        if (priv_if.i >= priv_if.p.len) {
            //  on to the next pbuf 
            priv_if.i = 0;
            if (priv_if.p.next != None && priv_if.p.next.len > 0) {
                //  p is a chain, on to the next in the chain 
                priv_if.p = priv_if.p.next;
            } else {
                /* p is a single pbuf, set it to NULL so next time a new
                 * pbuf is allocated */
                priv_if.p = None;
            }
        }
    }
    return None;
}

/* Like slipif_rxbyte, but passes completed packets to netif.input
 *
 * @param netif The lwip network interface structure for this slipif
 * @param c received character
 */
pub fn slipif_rxbyte_input(netif: &mut NetIfc, c: u8) {
    let p: &mut PacketBuffer;
    p = slipif_rxbyte(netif, c);
    if (p != None) {
        if (netif.input(p, netif) != ERR_OK) {
            pbuf_free(p);
        }
    }
}

/*
 * The SLIP input thread.
 *
 * Feed the IP layer with incoming packets
 *
 * @param nf the lwip network interface structure for this slipif
 */
pub fn slipif_loop_thread(nf: &mut Vec<u8>) {
    let c: u8;
    let netif: &mut NetIfc = nf;
    let priv_if: &mut slipif_priv = netif.state;

    loop {
        if (sio_read(priv_if.sd, &c, 1) > 0) {
            slipif_rxbyte_input(netif, c);
        }
    }
}

/*
 * @ingroup slipif
 * SLIP netif initialization
 *
 * Call the arch specific sio_open and remember
 * the opened device in the state field of the netif.
 *
 * @param netif the lwip network interface structure for this slipif
 * @return ERR_OK if serial line could be opened,
 *         ERR_MEM if no memory could be allocated,
 *         ERR_IF is serial line couldn't be opened
 *
 * @note If netif.state is interpreted as an serial: u8 port number.
 *
 */
pub fn slipif_init(netif: &mut NetIfc) {
    let mut priv_if: &mut slipif_priv;
    let sio_num: u8;

    LWIP_ASSERT("slipif needs an input callback", netif.input != None);

    //  netif.state contains serial port number 
    sio_num = LWIP_PTR_NUMERIC_CAST(u8, netif.state);

    //  LWIP_DEBUGF(SLIP_DEBUG, ("slipif_init: netif.num=%"U16_F"\n", sio_num));

    //  Allocate private data 
    priv_if = mem_malloc(sizeof(slipif_priv));
    if (!priv_if) {
        return ERR_MEM;
    }

    netif.name[0] = 's';
    netif.name[1] = 'l';

    netif.output = slipif_output_v4;

    netif.output_ip6 = slipif_output_v6;

    netif.mtu = SLIP_MAX_SIZE;

    //  Try to open the serial port. 
    priv_if.sd = sio_open(sio_num);
    if (!priv_if.sd) {
        //  Opening the serial port failed. 
        mem_free(priv_if);
        return ERR_IF;
    }

    //  Initialize private data 
    priv_if.p = None;
    priv_if.q = None;
    priv_if.state = SLIP_RECV_NORMAL;
    priv_if.i = 0;
    priv_if.recved = 0;

    priv_if.rxpackets = None;

    netif.state = priv_if;

    //  initialize the snmp variables and counters inside the NetIfc 
    MIB2_INIT_NETIF(netif, snmp_ifType_slip, SLIP_SIO_SPEED(priv_if.sd));

    //  Create a thread to poll the serial line. 
    sys_thread_new(
        SLIPIF_THREAD_NAME,
        slipif_loop_thread,
        netif,
        SLIPIF_THREAD_STACKSIZE,
        SLIPIF_THREAD_PRIO,
    );

    return Ok(());
}

/*
 * @ingroup slipif
 * Polls the serial device and feeds the IP layer with incoming packets.
 *
 * @param netif The lwip network interface structure for this slipif
 */
pub fn slipif_poll(netif: &mut NetIfc) {
    let c: u8;
    let mut priv_if: &mut slipif_priv;

    LWIP_ASSERT("netif != NULL", (netif != None));
    LWIP_ASSERT("netif.state != NULL", (netif.state != None));

    priv_if = netif.state;

    while (sio_tryread(priv_if.sd, &c, 1) > 0) {
        slipif_rxbyte_input(netif, c);
    }
}

/*
 * @ingroup slipif
 * Feeds the IP layer with incoming packets that were receive
 *
 * @param netif The lwip network interface structure for this slipif
 */
pub fn slipif_process_rxqueue(netif: &mut NetIfc) {
    let mut priv_if: &mut slipif_priv;
    SYS_ARCH_DECL_PROTECT(old_level);

    LWIP_ASSERT("netif != NULL", (netif != None));
    LWIP_ASSERT("netif.state != NULL", (netif.state != None));

    priv_if = netif.state;

    SYS_ARCH_PROTECT(old_level);
    while (priv_if.rxpackets != None) {
        let p: &mut PacketBuffer = priv_if.rxpackets;

        //  dequeue packet 
        let q: &mut PacketBuffer = p;
        while ((q.len != q.tot_len) && (q.next != None)) {
            q = q.next;
        }
        priv_if.rxpackets = q.next;
        q.next = None;
        //  SLIP_RX_QUEUE 
        priv_if.rxpackets = None;

        SYS_ARCH_UNPROTECT(old_level);
        if (netif.input(p, netif) != ERR_OK) {
            pbuf_free(p);
        }
        SYS_ARCH_PROTECT(old_level);
    }
    SYS_ARCH_UNPROTECT(old_level);
}

/* Like slipif_rxbyte, but queues completed packets.
 *
 * @param netif The lwip network interface structure for this slipif
 * @param data Received serial byte
 */
pub fn slipif_rxbyte_enqueue(netif: &mut NetIfc, data: u8) {
    let p: &mut PacketBuffer;
    let priv_if: &mut slipif_priv = netif.state;
    SYS_ARCH_DECL_PROTECT(old_level);

    p = slipif_rxbyte(netif, data);
    if (p != None) {
        SYS_ARCH_PROTECT(old_level);
        if (priv_if.rxpackets != None) {
            //  queue multiple pbufs 
            let q: &mut PacketBuffer = p;
            while (q.next != None) {
                q = q.next;
            }
            q.next = p;
        } else {
            //  SLIP_RX_QUEUE 
            pbuf_free(priv_if.rxpackets);
        }
        {
            priv_if.rxpackets = p;
        }
        SYS_ARCH_UNPROTECT(old_level);
    }
}

/*
 * @ingroup slipif
 * Process a received byte, completed packets are put on a queue that is
 * fed into IP through slipif_process_rxqueue().
 *
 * This function can be called from ISR if SYS_LIGHTWEIGHT_PROT is enabled.
 *
 * @param netif The lwip network interface structure for this slipif
 * @param data received character
 */
pub fn slipif_received_byte(netif: &mut NetIfc, data: u8) {
    LWIP_ASSERT("netif != NULL", (netif != None));
    LWIP_ASSERT("netif.state != NULL", (netif.state != None));
    slipif_rxbyte_enqueue(netif, data);
}

/*
 * @ingroup slipif
 * Process multiple received byte, completed packets are put on a queue that is
 * fed into IP through slipif_process_rxqueue().
 *
 * This function can be called from ISR if SYS_LIGHTWEIGHT_PROT is enabled.
 *
 * @param netif The lwip network interface structure for this slipif
 * @param data received character
 * @param len Number of received characters
 */
pub fn slipif_received_bytes(netif: &mut NetIfc, data: &mut Vec<u8>, len: u8) {
    let i: u8;
    let rxdata: &mut Vec<u8> = data;
    LWIP_ASSERT("netif != NULL", (netif != None));
    LWIP_ASSERT("netif.state != NULL", (netif.state != None));

    // for (i = 0; i < len; i+= 1, rxdata+= 1) {
    //   slipif_rxbyte_enqueue(netif, *rxdata);
    // }
}
