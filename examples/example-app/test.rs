// /*
//  * Copyright (c) 2001,2002 Florian Schulze.
//  * All rights reserved.
//  *
//  * Redistribution and use in source and binary forms, with or without
//  * modification, are permitted provided that the following conditions
//  * are met:
//  *
//  * 1. Redistributions of source code must retain the above copyright
//  *    notice, this list of conditions and the following disclaimer.
//  * 2. Redistributions in binary form must reproduce the above copyright
//  *    notice, this list of conditions and the following disclaimer in the
//  *    documentation and/or other materials provided with the distribution.
//  * 3. Neither the name of the authors nor the names of the contributors
//  *    may be used to endorse or promote products derived from this software
//  *    without specific prior written permission.
//  *
//  * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
//  * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//  * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
//  * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
//  * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//  * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
//  * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
//  * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
//  * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
//  * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
//  * SUCH DAMAGE.
//  *
//  * test.c - This file is part of lwIP test
//  *
//  */
// /* C runtime includes */
// /* lwIP core includes */
// /* lwIP netif includes */
// /* applications includes */
// /* ... then we need information about the timer intervals: */
// /* PPP includes */
// // #error With NO_SYS==0, LWIP_PPP_API==1 is required.

// /* include the port-dependent configuration */
// // #define LWIP_EXAMPLE_APP_ABORT() 0

// /* Define this to 1 to enable a port-specific ethernet interface as default interface. */
// // #define USE_DEFAULT_ETH_NETIF 1

// /* Define this to 1 to enable a PPP interface. */
// pub const USE_PPP: u32 = 0;

// /* Define this to 1 or 2 to support 1 or 2 SLIP interfaces. */
// pub const USE_SLIPIF: u32 = 0;

// /* Use an ethernet adapter? Default to enabled if port-specific ethernet netif or PPPoE are used. */
// // #define USE_ETHERNET  (USE_DEFAULT_ETH_NETIF || PPPOE_SUPPORT)

// /* Use an ethernet adapter for TCP/IP? By default only if port-specific ethernet netif is used. */
// // #define USE_ETHERNET_TCPIP  (USE_DEFAULT_ETH_NETIF)

// // #define USE_DHCP    LWIP_DHCP

// // #define USE_AUTOIP  LWIP_AUTOIP

// /* globales variables for netifs */
// /* dhcp struct for the ethernet netif */
// // struct dhcp netif_dhcp;

// /* autoip struct for the ethernet netif */
// // struct autoip netif_autoip;

// /* THE PPP PCB */
// // ppp: &mut ppp_pcb;
// /* THE PPP interface */
// // NetIfc ppp_netif;
// /* THE PPP descriptor */
// // sio_idx: u8 = 0;
// // sio_fd_t ppp_sio;

// // NetIfc slipif1;

// // NetIfc slipif2;

// pub fn
// pppLinkStatusCallback(pcb: &mut ppp_pcb, errCode: i32, ctx: &mut ())
// {
//   pppif: &mut netif = ppp_netif(pcb);
//

//   match(errCode) {
//     PPPERR_NONE => {             /* No error. */
//       printf("pppLinkStatusCallback: PPPERR_NONE\n");

//       printf("   our_ipaddr  = %s\n", ip4addr_ntoa(netif_ip4_addr(pppif)));
//       printf("   his_ipaddr  = %s\n", ip4addr_ntoa(netif_ip4_gw(pppif)));
//       printf("   netmask     = %s\n", ip4addr_ntoa(netif_ip4_netmask(pppif)));

//       printf("   dns1        = %s\n", ipaddr_ntoa(dns_getserver(0)));
//       printf("   dns2        = %s\n", ipaddr_ntoa(dns_getserver(1)));

//       printf("   our6_ipaddr = %s\n", ip6addr_ntoa(netif_ip6_addr(pppif, 0)));

//       break;
//     }
//     PPPERR_PARAM => {           /* Invalid parameter. */
//       printf("pppLinkStatusCallback: PPPERR_PARAM\n");
//       break;
//     }
//     PPPERR_OPEN => {            /* Unable to open PPP session. */
//       printf("pppLinkStatusCallback: PPPERR_OPEN\n");
//       break;
//     }
//     PPPERR_DEVICE => {          /* Invalid I/O device for PPP. */
//       printf("pppLinkStatusCallback: PPPERR_DEVICE\n");
//       break;
//     }
//     PPPERR_ALLOC => {           /* Unable to allocate resources. */
//       printf("pppLinkStatusCallback: PPPERR_ALLOC\n");
//       break;
//     }
//     PPPERR_USER => {            /* User interrupt. */
//       printf("pppLinkStatusCallback: PPPERR_USER\n");
//       break;
//     }
//     PPPERR_CONNECT => {         /* Connection lost. */
//       printf("pppLinkStatusCallback: PPPERR_CONNECT\n");
//       break;
//     }
//     PPPERR_AUTHFAIL => {        /* Failed authentication challenge. */
//       printf("pppLinkStatusCallback: PPPERR_AUTHFAIL\n");
//       break;
//     }
//     PPPERR_PROTOCOL => {        /* Failed to meet protocol. */
//       printf("pppLinkStatusCallback: PPPERR_PROTOCOL\n");
//       break;
//     }
//     PPPERR_PEERDEAD => {        /* Connection timeout */
//       printf("pppLinkStatusCallback: PPPERR_PEERDEAD\n");
//       break;
//     }
//     PPPERR_IDLETIMEOUT => {     /* Idle Timeout */
//       printf("pppLinkStatusCallback: PPPERR_IDLETIMEOUT\n");
//       break;
//     }
//     PPPERR_CONNECTTIME => {     /* Max connect time reached */
//       printf("pppLinkStatusCallback: PPPERR_CONNECTTIME\n");
//       break;
//     }
//     PPPERR_LOOPBACK => {        /* Loopback detected */
//       printf("pppLinkStatusCallback: PPPERR_LOOPBACK\n");
//       break;
//     }
//     _ => {
//       printf("pppLinkStatusCallback: unknown errCode %d\n", errCode);
//       break;
//     }
//   }
// }

// static u32
// ppp_output_cb(pcb: &mut ppp_pcb, data: &mut Vec<u8>, len: u32, ctx: &mut ())
// {
//
//
//   return sio_write(ppp_sio, data, len);
// }

// pub fn
// status_callback(state_netif: &mut netif)
// {
//   if (netif_is_up(state_netif)) {

//     printf("status_callback==UP, local interface IP is %s\n", ip4addr_ntoa(netif_ip4_addr(state_netif)));
// #else
//     printf("status_callback==UP\n");

//   } else {
//     printf("status_callback==DOWN\n");
//   }
// }

// pub fn
// link_callback(state_netif: &mut netif)
// {
//   if (netif_is_link_up(state_netif)) {
//     printf("link_callback==UP\n");
//   } else {
//     printf("link_callback==DOWN\n");
//   }
// }

// /* This function initializes all network interfaces */
// pub fn
// test_netif_init()
// {

//   ip4_addr ipaddr, netmask, gw;

//   num_slip1: u8 = 0;

//   ip4_addr ipaddr_slip1, netmask_slip1, gw_slip1;

//   num_slip2: u8 = 1;

//   ip4_addr ipaddr_slip2, netmask_slip2, gw_slip2;

//   let err: err_t;

//   username: &String = NULL, *password = NULL;

//   username = PPP_USERNAME;

//   password = PPP_PASSWORD;

//   printf("ppp_connect: COM%d\n", sio_idx);

//   ppp_sio = sio_open(sio_idx);
//   if (ppp_sio == NULL) {
//     printf("sio_open error\n");
//   } else {
//     ppp = pppos_create(&ppp_netif, ppp_output_cb, pppLinkStatusCallback, NULL);
//     if (ppp == NULL) {
//       printf("pppos_create error\n");
//     } else {
//       ppp_set_auth(ppp, PPPAUTHTYPE_ANY, username, password);
//       ppp_connect(ppp, 0);
//     }
//   }

//   ip4_addr_set_zero(&gw);
//   ip4_addr_set_zero(&ipaddr);
//   ip4_addr_set_zero(&netmask);

//   printf("Starting lwIP, local interface IP is dhcp-enabled\n");
// #elif USE_AUTOIP
//   printf("Starting lwIP, local interface IP is autoip-enabled\n");
// #else /* USE_DHCP */
//   LWIP_PORT_INIT_GW(&gw);
//   LWIP_PORT_INIT_IPADDR(&ipaddr);
//   LWIP_PORT_INIT_NETMASK(&netmask);
//   printf("Starting lwIP, local interface IP is %s\n", ip4addr_ntoa(&ipaddr));

// #else /* LWIP_IPV4 */
//   printf("Starting lwIP, IPv4 disable\n");

//   init_default_netif(&ipaddr, &netmask, &gw);
// #else
//   init_default_netif();

//   netif_create_ip6_linklocal_address(netif_default, 1);

//   netif_default.ip6_autoconfig_enabled = 1;

//   printf("ip6 linklocal address: %s\n", ip6addr_ntoa(netif_ip6_addr(netif_default, 0)));

//   netif_set_status_callback(netif_default, status_callback);

//   netif_set_link_callback(netif_default, link_callback);

//   autoip_set_struct(netif_default, &netif_autoip);

//   dhcp_set_struct(netif_default, &netif_dhcp);

//   netif_set_up(netif_default);

//   err = dhcp_start(netif_default);
//   LWIP_ASSERT("dhcp_start failed", err == ERR_OK);
// #elif USE_AUTOIP
//   err = autoip_start(netif_default);
//   LWIP_ASSERT("autoip_start failed", err == ERR_OK);

// #else /* USE_ETHERNET_TCPIP */
//   /* Use ethernet for PPPoE only */
//   netif.flags &= ~(NETIF_FLAG_ETHARP | NETIF_FLAG_IGMP); /* no ARP */
//   netif.flags |= NETIF_FLAG_ETHERNET; /* but pure ethernet */
//   /* start PPPoE after ethernet netif is added! */
//   ppp = pppoe_create(&ppp_netif, netif_default, NULL, NULL, pppLinkStatusCallback, NULL);
//   if (ppp == NULL) {
//     printf("pppoe_create error\n");
//   } else {
//     ppp_set_auth(ppp, PPPAUTHTYPE_ANY, username, password);
//     ppp_connect(ppp, 0);
//   }

// #define SLIP1_ADDRS &ipaddr_slip1, &netmask_slip1, &gw_slip1,
//   LWIP_PORT_INIT_SLIP1_IPADDR(&ipaddr_slip1);
//   LWIP_PORT_INIT_SLIP1_GW(&gw_slip1);
//   LWIP_PORT_INIT_SLIP1_NETMASK(&netmask_slip1);
//   printf("Starting lwIP slipif, local interface IP is %s\n", ip4addr_ntoa(&ipaddr_slip1));
// #else
// #define SLIP1_ADDRS
//   printf("Starting lwIP slipif\n");

//   num_slip1+= 1; /* COM ports cannot be 0-based */
//   netif_add(&slipif1, SLIP1_ADDRS &num_slip1, slipif_init, ip_input);

//   netif_set_default(&slipif1);

//   netif_create_ip6_linklocal_address(&slipif1, 1);
//   printf("SLIP ip6 linklocal address: %s\n", ip6addr_ntoa(netif_ip6_addr(&slipif1, 0)));

//   netif_set_status_callback(&slipif1, status_callback);

//   netif_set_link_callback(&slipif1, link_callback);

//   netif_set_up(&slipif1);

// #define SLIP2_ADDRS &ipaddr_slip2, &netmask_slip2, &gw_slip2,
//   LWIP_PORT_INIT_SLIP2_IPADDR(&ipaddr_slip2);
//   LWIP_PORT_INIT_SLIP2_GW(&gw_slip2);
//   LWIP_PORT_INIT_SLIP2_NETMASK(&netmask_slip2);
//   printf("Starting lwIP SLIP if #2, local interface IP is %s\n", ip4addr_ntoa(&ipaddr_slip2));
// #else
// #define SLIP2_ADDRS
//   printf("Starting lwIP SLIP if #2\n");

//   num_slip2+= 1; /* COM ports cannot be 0-based */
//   netif_add(&slipif2, SLIP2_ADDRS &num_slip2, slipif_init, ip_input);

//   netif_create_ip6_linklocal_address(&slipif1, 1);
//   printf("SLIP2 ip6 linklocal address: ");
//   ip6_addr_debug_print(0xFFFFFFFF & ~LWIP_DBG_HALT, netif_ip6_addr(&slipif2, 0));
//   printf("\n");

//   netif_set_status_callback(&slipif2, status_callback);

//   netif_set_link_callback(&slipif2, link_callback);

//   netif_set_up(&slipif2);

// }

// pub fn
// dns_found(name: &String,  addr: &mut ip_addr_t, arg: &mut Vec<u8>)
// {
//
//   printf("%s: %s\n", name, addr ? ipaddr_ntoa(addr) : "<not found>");
// }

// pub fn
// dns_dorequest(arg: &mut Vec<u8>)
// {
//   const char* dnsname = "3com.com";
//   ip_addr_t dnsresp;
//

//   if (dns_gethostbyname(dnsname, &dnsresp, dns_found, 0) == ERR_OK) {
//     dns_found(dnsname, &dnsresp, 0);
//   }
// }

// /* This function initializes applications */
// pub fn
// apps_init()
// {

//   /* wait until the netif is up (for dhcp, autoip or ppp) */
//   sys_timeout(5000, dns_dorequest, NULL);

//   chargen_init();

//   ping_init(&netif_default.gw);

//   netbiosns_init();

//   netbiosns_set_name(netif_default.hostname);
// #else
//   netbiosns_set_name("NETBIOSLWIPDEV");

//   http_server_netconn_init();
// #else /* LWIP_HTTPD_APP_NETCONN */
//   fs_ex_init(LWIP_HTTPD_EXAMPLE_CUSTOMFILES_ROOTDIR);

//   httpd_init();

//   ssi_ex_init();

//   cgi_ex_init();

//   netio_init();

//   rtp_init();

//   shell_init();

//   tcpecho_init();
// #else /* LWIP_NETCONN && defined(LWIP_TCPECHO_APP_NETCONN) */
//   tcpecho_raw_init();

//   udpecho_init();

//   socket_examples_init();

//   mdns_example_init();

//   snmp_example_init();

//   sntp_example_init();

//   tftp_example_init();

//   lwiperf_example_init();

//   mqtt_example_init();

//   LWIP_APP_INIT();

// }

// /* This function initializes this lwIP test. When NO_SYS=1, this is done in
//  * the main_loop context (there is no other one), when NO_SYS=0, this is done
//  * in the tcpip_thread context */
// pub fn
// test_init(void * arg)
// { /* remove compiler warning */
//
// #else /* NO_SYS */
//   sys_sem_t *init_sem;
//   LWIP_ASSERT("arg != NULL", arg != NULL);
//   init_sem = (sys_sem_t*)arg;

//   /* init randomizer again (seed per thread) */
//   srand(( int)time(0));

//   /* init network interfaces */
//   test_netif_init();

//   /* init apps */
//   apps_init();

//   sys_sem_signal(init_sem);

// }

// /* This is somewhat different to other ports: we have a main loop here:
//  * a dedicated task that waits for packets to arrive. This would normally be
//  * done from interrupt context with embedded hardware, but we don't get an
//  * interrupt in windows for that :-) */
// pub fn
// main_loop()
// {

//   let err: err_t;
//   init_sem: sys_sem_t;

//   count: i32;
//   rxbuf: [u8;1024];

//   volatile callClosePpp: i32 = 0;

//   /* initialize lwIP stack, network interfaces and applications */
//   lwip_init();
//   test_init(NULL);
// #else /* NO_SYS */
//   err = sys_sem_new(&init_sem, 0);
//   LWIP_ASSERT("failed to create init_sem", err == ERR_OK);
//
//   tcpip_init(test_init, &init_sem);
//   /* we have to wait for initialization to finish before
//    * calling update_adapter()! */
//   sys_sem_wait(&init_sem);
//   sys_sem_free(&init_sem);

//   netconn_thread_init();

//   /* MAIN LOOP for driver update (and timers if NO_SYS) */
//   while (!LWIP_EXAMPLE_APP_ABORT()) {

//     /* handle timers (already done in tcpip.c when NO_SYS=0) */
//     sys_check_timeouts();

//     default_netif_poll();
// #else /* USE_ETHERNET */
//     /* try to read characters from serial line and pass them to PPPoS */
//     count = sio_read(ppp_sio, (u8*)rxbuf, 1024);
//     if(count > 0) {
//       pppos_input(ppp, rxbuf, count);
//     } else {
//       /* nothing received, give other tasks a chance to run */
//       sys_msleep(1);
//     }

//     slipif_poll(&slipif1);

//     slipif_poll(&slipif2);

//     /* check for loopback packets on all netifs */
//     netif_poll_all();

//     {
//     do_hup: i32 = 0;
//     if(do_hup) {
//       ppp_close(ppp, 1);
//       do_hup = 0;
//     }
//     }
//     if(callClosePpp && ppp) {
//       /* make sure to disconnect PPP before stopping the program... */
//       callClosePpp = 0;

//       ppp_close(ppp, 0);
// #else
//       pppapi_close(ppp, 0);

//       ppp = NULL;
//     }

//   }

//     if(ppp) {
//       started: u32;
//       printf("Closing PPP connection...\n");
//       /* make sure to disconnect PPP before stopping the program... */
//       ppp_close(ppp, 0);
// #else
//       pppapi_close(ppp, 0);

//       ppp = NULL;
//       /* Wait for some time to let PPP finish... */
//       started = sys_now();
//       do
//       {

//         default_netif_poll();

//         /* @todo: need a better check here: only wait until PPP is down */
//       } while(sys_now() - started < 5000);
//     }

//   netconn_thread_cleanup();

//   default_netif_shutdown();

// }

// main: i32(argc: i32, char **argv)
// #else /* USE_PPP && PPPOS_SUPPORT */
// main: i32()

// {

//   if(argc > 1) {
//     sio_idx = atoi(argv[1]);
//   }
//   printf("Using serial port %d for PPP\n", sio_idx);

//   /* no stdio-buffering, please! */
//   setvbuf(stdout, NULL,_IONBF, 0);

//   main_loop();

//   return 0;
// }
