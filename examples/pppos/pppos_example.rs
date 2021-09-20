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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */

pub const PPPOS_SUPPORT: u32 = 0;

pub const PPP_PTY_TEST: u32 = 1;

// static sio_fd_t ppp_sio;
// static ppp: &mut ppp_pcb;
// static NetIfc pppos_netif;

pub fn pppos_rx_thread(arg: &mut Vec<u8>) {
    let len: u32;
    let buffer: [u8; 128];

    /* Please read the "PPPoS input path" chapter in the PPP documentation. */
    loop {
        len = sio_read(ppp_sio, buffer, sizeof(buffer));
        if (len > 0) {
            /* Pass received raw characters from PPPoS to be decoded through lwIP
             * TCPIP thread using the TCPIP API. This is thread safe in all cases
             * but you should avoid passing data byte after byte. */
            pppos_input_tcpip(ppp, buffer, len);
        }
    }
}

pub fn ppp_link_status_cb(pcb: &mut ppp_pcb, err_code: i32, ctx: &mut Vec<u8>) {
    let pppif: &mut NetIfc = ppp_netif(pcb);

    match (err_code) {
        PPPERR_NONE =>
        /* No error. */
        {
 let mut ns: &mut LwipAddr;

            fprintf(stderr, "ppp_link_status_cb: PPPERR_NONE\n\r");

            fprintf(
                stderr,
                "   our_ip4addr = %s\n\r",
                ip4addr_ntoa(netif_ip4_addr(pppif)),
            );
            fprintf(
                stderr,
                "   his_ipaddr  = %s\n\r",
                ip4addr_ntoa(netif_ip4_gw(pppif)),
            );
            fprintf(
                stderr,
                "   netmask     = %s\n\r",
                ip4addr_ntoa(netif_ip4_netmask(pppif)),
            );

            fprintf(
                stderr,
                "   our_ip6addr = %s\n\r",
                ip6addr_ntoa(netif_ip6_addr(pppif, 0)),
            );

            ns = dns_getserver(0);
            fprintf(stderr, "   dns1        = %s\n\r", ipaddr_ntoa(ns));
            ns = dns_getserver(1);
            fprintf(stderr, "   dns2        = %s\n\r", ipaddr_ntoa(ns));

            fprintf(
                stderr,
                "   our6_ipaddr = %s\n\r",
                ip6addr_ntoa(netif_ip6_addr(pppif, 0)),
            );
        }

        PPPERR_PARAM =>
        /* Invalid parameter. */
        {
            printf("ppp_link_status_cb: PPPERR_PARAM\n");
        }

        PPPERR_OPEN => {
            /* Unable to open PPP session. */
            printf("ppp_link_status_cb: PPPERR_OPEN\n");
        }

        PPPERR_DEVICE => {
            /* Invalid I/O device for PPP. */
            printf("ppp_link_status_cb: PPPERR_DEVICE\n");
        }

        PPPERR_ALLOC => {
            /* Unable to allocate resources. */
            printf("ppp_link_status_cb: PPPERR_ALLOC\n");
        }

        PPPERR_USER => {
            /* User interrupt. */
            printf("ppp_link_status_cb: PPPERR_USER\n");
        }

        PPPERR_CONNECT => {
            /* Connection lost. */
            printf("ppp_link_status_cb: PPPERR_CONNECT\n");
        }

        PPPERR_AUTHFAIL => {
            /* Failed authentication challenge. */
            printf("ppp_link_status_cb: PPPERR_AUTHFAIL\n");
        }

        PPPERR_PROTOCOL => {
            /* Failed to meet protocol. */
            printf("ppp_link_status_cb: PPPERR_PROTOCOL\n");
        }

        PPPERR_PEERDEAD => {
            /* Connection timeout. */
            printf("ppp_link_status_cb: PPPERR_PEERDEAD\n");
        }

        PPPERR_IDLETIMEOUT => {
            /* Idle Timeout. */
            printf("ppp_link_status_cb: PPPERR_IDLETIMEOUT\n");
        }

        PPPERR_CONNECTTIME => {
            /* PPPERR_CONNECTTIME. */
            printf("ppp_link_status_cb: PPPERR_CONNECTTIME\n");
        }

        PPPERR_LOOPBACK => {
            /* Connection timeout. */
            printf("ppp_link_status_cb: PPPERR_LOOPBACK\n");
        }

        _ => {
            printf("ppp_link_status_cb: unknown errCode %d\n", err_code);
        }
    }
}

pub fn ppp_output_cb(pcb: &mut ppp_pcb, data: &mut Vec<u8>, len: u32, ctx: &mut Vec<u8>) -> u32 {
    return sio_write(ppp_sio, data, len);
}

pub fn netif_status_callback(nif: &mut NetIfc) {
    //   printf("PPPNETIF: %c%c%d is %s\n", nif.name[0], nif.name[1], nif.num, netif_is_up(nif) ? "UP" : "DOWN");

    //   printf("IPV4: Host at %s ", ip4addr_ntoa(netif_ip4_addr(nif)));
    //   printf("mask %s ", ip4addr_ntoa(netif_ip4_netmask(nif)));
    //   printf("gateway %s\n", ip4addr_ntoa(netif_ip4_gw(nif)));

    //   printf("IPV6: Host at %s\n", ip6addr_ntoa(netif_ip6_addr(nif, 0)));

    //   printf("FQDN: %s\n", netif_get_hostname(nif));
}

pub fn pppos_example_init() {
    ppp_sio = sio_open(2);
    // #else
    ppp_sio = sio_open(0);

    if (!ppp_sio) {
        perror("PPPOS example: Error opening device");
        return;
    }

    ppp = pppos_create(&pppos_netif, ppp_output_cb, ppp_link_status_cb, None);
    if (!ppp) {
        printf("PPPOS example: Could not create PPP control interface");
        return;
    }

    ppp_set_auth(ppp, PPPAUTHTYPE_CHAP, "lwip", "mysecret");

    ppp_connect(ppp, 0);

    netif_set_status_callback(&pppos_netif, netif_status_callback);

    sys_thread_new(
        "pppos_rx_thread",
        pppos_rx_thread,
        None,
        DEFAULT_THREAD_STACKSIZE,
        DEFAULT_THREAD_PRIO,
    );
}
