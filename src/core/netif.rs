/*
 * @file
 * lwIP network interface abstraction
 *
 * @defgroup netif Network interface (NETIF)
 * @ingroup callbackstyle_api
 *
 * @defgroup netif_ip4 IPv4 address handling
 * @ingroup netif
 *
 * @defgroup netif_ip6 IPv6 address handling
 * @ingroup netif
 *
 * @defgroup netif_cd Client data handling
 * Store data  on a netif for application usage.
 * @see @ref LWIP_NUM_NETIF_CLIENT_DATA
 * @ingroup netif
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
 */












































#define NETIF_STATUS_CALLBACK(n) do{ if (n.status_callback) { (n.status_callback)(n); }}while(0)

#define NETIF_STATUS_CALLBACK(n)



#define NETIF_LINK_CALLBACK(n) do{ if (n.link_callback) { (n.link_callback)(n); }}while(0)

#define NETIF_LINK_CALLBACK(n)



static netif_ext_callback_t *ext_callback;



netif_list: &mut netif;

netif__ => &mut netif;

#define netif_index_to_num(index)   ((index) - 1)
static netif_num: u8;


static netif_client_id: u8;


pub const NETIF_REPORT_TYPE_IPV4: u32 = 0x01;pub const NETIF_REPORT_TYPE_IPV4: u32 = 0x01;
#define NETIF_REPORT_TYPE_IPV6  0x02
pub fn netif_issue_reports(netif: &mut netif, report_type: u8);


static netif_null_output_ip6: err_t(netif: &mut netif, p: &mut pbuf,  ipaddr: &mut ip6_addr_t);


static netif_null_output_ip4: err_t(netif: &mut netif, p: &mut pbuf,  ipaddr: &mut ip4_addr);




static netif_loop_output_ipv4: err_t(netif: &mut netif, p: &mut pbuf,  addr: &mut ip4_addr);


static netif_loop_output_ipv6: err_t(netif: &mut netif, p: &mut pbuf,  addr: &mut ip6_addr_t);



static NetIfc loop_netif;

/*
 * Initialize a lwip network interface structure for a loopback interface
 *
 * @param netif the lwip network interface structure for this loopif
 * @return ERR_OK if the loopif is initialized
 *         ERR_MEM if private data couldn't be allocated
 */
pub fn netif_loopif_init(netif: &mut netif) -> Result<(), LwipError>
{
  LWIP_ASSERT("netif_loopif_init: invalid netif", netif != NULL);

  /* initialize the snmp variables and counters inside the NetIfc
   * ifSpeed: no assumption can be made!
   */
  MIB2_INIT_NETIF(netif, snmp_ifType_softwareLoopback, 0);

  netif.name[0] = 'l';
  netif.name[1] = 'o';

  netif.output = netif_loop_output_ipv4;


  netif.output_ip6 = netif_loop_output_ipv6;


  netif_set_flags(netif, NETIF_FLAG_IGMP);

  NETIF_SET_CHECKSUM_CTRL(netif, NETIF_CHECKSUM_DISABLE_ALL);
  return ERR_OK;
}


pub fn 
netif_init()
{


#define LOOPIF_ADDRINIT &loop_ipaddr, &loop_netmask, &loop_gw,
  ip4_addr loop_ipaddr, loop_netmask, loop_gw;
  IP4_ADDR(&loop_gw, 127, 0, 0, 1);
  IP4_ADDR(&loop_ipaddr, 127, 0, 0, 1);
  IP4_ADDR(&loop_netmask, 255, 0, 0, 0);
 /* LWIP_IPV4 */
#define LOOPIF_ADDRINIT



  netif_add(&loop_netif, LOOPIF_ADDRINIT NULL, netif_loopif_init, ip_input);
  /* NO_SYS */
  netif_add(&loop_netif, LOOPIF_ADDRINIT NULL, netif_loopif_init, tcpip_input);



  IP_ADDR6_HOST(loop_netif.ip6_addr, 0, 0, 0, 0x00000001);
  loop_netif.ip6_addr_state[0] = IP6_ADDR_VALID;


  netif_set_link_up(&loop_netif);
  netif_set_up(&loop_netif);


}

/*
 * @ingroup lwip_nosys
 * Forwards a received packet for input processing with
 * ethernet_input() or ip_input() depending on netif flags.
 * Don't call directly, pass to netif_add() and call
 * netif.input().
 * Only works if the netif driver correctly sets
 * NETIF_FLAG_ETHARP and/or NETIF_FLAG_ETHERNET flag!
 */
pub fn 
netif_input(p: &mut pbuf, inp: &mut netif)
{
  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ASSERT("netif_input: invalid pbuf", p != NULL);
  LWIP_ASSERT("netif_input: invalid netif", inp != NULL);


  if (inp.flags & (NETIF_FLAG_ETHARP | NETIF_FLAG_ETHERNET)) {
    return ethernet_input(p, inp);
  } else

    return ip_input(p, inp);
}

/*
 * @ingroup netif
 * Add a network interface to the list of lwIP netifs.
 *
 * Same as @ref netif_add but without IPv4 addresses
 */
NetIfc *
netif_add_noaddr(netif: &mut netif, state: &mut (), netif_init_fn init, netif_input_fn input)
{
  return netif_add(netif,

                   NULL, NULL, NULL,

                   state, init, input);
}

/*
 * @ingroup netif
 * Add a network interface to the list of lwIP netifs.
 *
 * @param netif a pre-allocated netif structure
 * @param ipaddr IP address for the new netif
 * @param netmask network mask for the new netif
 * @param gw default gateway IP address for the new netif
 * @param state opaque data passed to the new netif
 * @param init callback function that initializes the interface
 * @param input callback function that is called to pass
 * ingress packets up in the protocol layer stack.\n
 * It is recommended to use a function that passes the input directly
 * to the stack (netif_input(), NO_SYS=1 mode) or via sending a
 * message to TCPIP thread (tcpip_input(), NO_SYS=0 mode).\n
 * These functions use netif flags NETIF_FLAG_ETHARP and NETIF_FLAG_ETHERNET
 * to decide whether to forward to ethernet_input() or ip_input().
 * In other words, the functions only work when the netif
 * driver is implemented correctly!\n
 * Most members of NetIfc should be be initialized by the
 * netif init function = netif driver (init parameter of this function).\n
 * IPv6: Don't forget to call netif_create_ip6_linklocal_address() after
 * setting the MAC address in NetIfc.hwaddr
 * (IPv6 requires a link-local address).
 *
 * @return netif, or NULL if failed.
 */
NetIfc *
netif_add(netif: &mut netif,

          const ipaddr: &mut ip4_addr,  netmask: &mut ip4_addr,  gw: &mut ip4_addr,

          state: &mut (), netif_init_fn init, netif_input_fn input)
{

  s8_t i;


  LWIP_ASSERT_CORE_LOCKED();


  if (netif_default != NULL) {
    LWIP_ASSERT("single netif already set", 0);
    return NULL;
  }


  LWIP_ERROR("netif_add: invalid netif", netif != NULL, return NULL);
  LWIP_ERROR("netif_add: No init function given", init != NULL, return NULL);


  if (ipaddr == NULL) {
    ipaddr = ip_2_ip4(IP4_ADDR_ANY);
  }
  if (netmask == NULL) {
    netmask = ip_2_ip4(IP4_ADDR_ANY);
  }
  if (gw == NULL) {
    gw = ip_2_ip4(IP4_ADDR_ANY);
  }

  /* reset new interface configuration state */
  ip_addr_set_zero_ip4(&netif.ip_addr);
  ip_addr_set_zero_ip4(&netif.netmask);
  ip_addr_set_zero_ip4(&netif.gw);
  netif.output = netif_null_output_ip4;


  for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
    ip_addr_set_zero_ip6(&netif.ip6_addr[i]);
    netif.ip6_addr_state[i] = IP6_ADDR_INVALID;

    netif.ip6_addr_valid_life[i] = IP6_ADDR_LIFE_STATIC;
    netif.ip6_addr_pref_life[i] = IP6_ADDR_LIFE_STATIC;

  }
  netif.output_ip6 = netif_null_output_ip6;

  NETIF_SET_CHECKSUM_CTRL(netif, NETIF_CHECKSUM_ENABLE_ALL);
  netif.mtu = 0;
  netif.flags = 0;

  memset(netif.client_data, 0, sizeof(netif.client_data));



  /* IPv6 address autoconfiguration not enabled by default */
  netif.ip6_autoconfig_enabled = 0;

  nd6_restart_netif(netif);


  netif.status_callback = NULL;


  netif.link_callback = NULL;


  netif.igmp_mac_filter = NULL;


  netif.mld_mac_filter = NULL;


  netif.loop_first = NULL;
  netif.loop_last = NULL;


  /* remember netif specific state information data */
  netif.state = state;
  netif.num = netif_num;
  netif.input = input;

  NETIF_RESET_HINTS(netif);

  netif.loop_cnt_current = 0;



  netif_set_addr(netif, ipaddr, netmask, gw);


  /* call user specified initialization function for netif */
  if (init(netif) != ERR_OK) {
    return NULL;
  }

  /* Initialize the MTU for IPv6 to the one set by the netif driver.
     This can be updated later by RA. */
  netif.mtu6 = netif.mtu;



  /* Assign a unique netif number in the range [0..254], so that (num+1) can
     serve as an interface index that fits in a u8.
     We assume that the new netif has not yet been added to the list here.
     This algorithm is O(n^2), but that should be OK for lwIP.
     */
  {
    netif2: &mut netif;
    num_netifs: i32;
    loop {
      if (netif.num == 255) {
        netif.num = 0;
      }
      num_netifs = 0;
      for (netif2 = netif_list; netif2 != NULL; netif2 = netif2.next) {
        LWIP_ASSERT("netif already added", netif2 != netif);
        num_netifs+= 1;
        LWIP_ASSERT("too many netifs, max. supported number is 255", num_netifs <= 255);
        if (netif2.num == netif.num) {
          netif.num+= 1;
          break;
        }
      }
    } while (netif2 != NULL);
  }
  if (netif.num == 254) {
    netif_num = 0;
  } else {
    netif_num = (netif.num + 1);
  }

  /* add this netif to the list */
  netif.next = netif_list;
  netif_list = netif;

  mib2_netif_added(netif);


  /* start IGMP processing */
  if (netif.flags & NETIF_FLAG_IGMP) {
    igmp_start(netif);
  }


  LWIP_DEBUGF(NETIF_DEBUG, ("netif: added interface %c%c IP",
                            netif.name[0], netif.name[1]));

  LWIP_DEBUGF(NETIF_DEBUG, (" addr "));
  ip4_addr_debug_print(NETIF_DEBUG, ipaddr);
  LWIP_DEBUGF(NETIF_DEBUG, (" netmask "));
  ip4_addr_debug_print(NETIF_DEBUG, netmask);
  LWIP_DEBUGF(NETIF_DEBUG, (" gw "));
  ip4_addr_debug_print(NETIF_DEBUG, gw);

  LWIP_DEBUGF(NETIF_DEBUG, ("\n"));

  netif_invoke_ext_callback(netif, LWIP_NSC_NETIF_ADDED, NULL);

  return netif;
}

pub fn
netif_do_ip_addr_changed(const old_addr: &mut ip_addr_t,  new_addr: &mut ip_addr_t)
{

  tcp_netif_ip_addr_changed(old_addr, new_addr);


  udp_netif_ip_addr_changed(old_addr, new_addr);


  raw_netif_ip_addr_changed(old_addr, new_addr);

}


static int
netif_do_set_ipaddr(netif: &mut netif,  ipaddr: &mut ip4_addr, old_addr: &mut ip_addr_t)
{
  LWIP_ASSERT("invalid pointer", ipaddr != NULL);
  LWIP_ASSERT("invalid pointer", old_addr != NULL);

  /* address is actually being changed? */
  if (ip4_addr_cmp(ipaddr, netif_ip4_addr(netif)) == 0) {
    ip_addr_t new_addr;
    *ip_2_ip4(&new_addr) = *ipaddr;
    IP_SET_TYPE_VAL(new_addr, IPADDR_TYPE_V4);

    ip_addr_copy(*old_addr, *netif_ip_addr4(netif));

    LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_STATE, ("netif_set_ipaddr: netif address being changed\n"));
    netif_do_ip_addr_changed(old_addr, &new_addr);

    mib2_remove_ip4(netif);
    mib2_remove_route_ip4(0, netif);
    /* set new IP address to netif */
    ip4_addr_set(ip_2_ip4(&netif.ip_addr), ipaddr);
    IP_SET_TYPE_VAL(netif.ip_addr, IPADDR_TYPE_V4);
    mib2_add_ip4(netif);
    mib2_add_route_ip4(0, netif);

    netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV4);

    NETIF_STATUS_CALLBACK(netif);
    return 1; /* address changed */
  }
  return 0; /* address unchanged */
}

/*
 * @ingroup netif_ip4
 * Change the IP address of a network interface
 *
 * @param netif the network interface to change
 * @param ipaddr the new IP address
 *
 * @note call netif_set_addr() if you also want to change netmask and
 * default gateway
 */
pub fn 
netif_set_ipaddr(netif: &mut netif,  ipaddr: &mut ip4_addr)
{
  ip_addr_t old_addr;

  LWIP_ERROR("netif_set_ipaddr: invalid netif", netif != NULL, return);

  /* Don't propagate NULL pointer (IPv4 ANY) to subsequent functions */
  if (ipaddr == NULL) {
    ipaddr = IP4_ADDR_ANY4;
  }

  LWIP_ASSERT_CORE_LOCKED();

  if (netif_do_set_ipaddr(netif, ipaddr, &old_addr)) {

    netif_ext_callback_args_t args;
    args.ipv4_changed.old_address = &old_addr;
    netif_invoke_ext_callback(netif, LWIP_NSC_IPV4_ADDRESS_CHANGED, &args);

  }
}

static int
netif_do_set_netmask(netif: &mut netif,  netmask: &mut ip4_addr, old_nm: &mut ip_addr_t)
{
  /* address is actually being changed? */
  if (ip4_addr_cmp(netmask, netif_ip4_netmask(netif)) == 0) {

    LWIP_ASSERT("invalid pointer", old_nm != NULL);
    ip_addr_copy(*old_nm, *netif_ip_netmask4(netif));

    

    mib2_remove_route_ip4(0, netif);
    /* set new netmask to netif */
    ip4_addr_set(ip_2_ip4(&netif.netmask), netmask);
    IP_SET_TYPE_VAL(netif.netmask, IPADDR_TYPE_V4);
    mib2_add_route_ip4(0, netif);
    LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE, ("netif: netmask of interface %c%c set to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                netif.name[0], netif.name[1],
                ip4_addr1_16(netif_ip4_netmask(netif)),
                ip4_addr2_16(netif_ip4_netmask(netif)),
                ip4_addr3_16(netif_ip4_netmask(netif)),
                ip4_addr4_16(netif_ip4_netmask(netif))));
    return 1; /* netmask changed */
  }
  return 0; /* netmask unchanged */
}

/*
 * @ingroup netif_ip4
 * Change the netmask of a network interface
 *
 * @param netif the network interface to change
 * @param netmask the new netmask
 *
 * @note call netif_set_addr() if you also want to change ip address and
 * default gateway
 */
pub fn 
netif_set_netmask(netif: &mut netif,  netmask: &mut ip4_addr)
{

  ip_addr_t old_nm_val;
  old_nm: &mut ip_addr_t = &old_nm_val;

  old_nm: &mut ip_addr_t = NULL;

  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ERROR("netif_set_netmask: invalid netif", netif != NULL, return);

  /* Don't propagate NULL pointer (IPv4 ANY) to subsequent functions */
  if (netmask == NULL) {
    netmask = IP4_ADDR_ANY4;
  }

  if (netif_do_set_netmask(netif, netmask, old_nm)) {

    netif_ext_callback_args_t args;
    args.ipv4_changed.old_netmask = old_nm;
    netif_invoke_ext_callback(netif, LWIP_NSC_IPV4_NETMASK_CHANGED, &args);

  }
}

static int
netif_do_set_gw(netif: &mut netif,  gw: &mut ip4_addr, old_gw: &mut ip_addr_t)
{
  /* address is actually being changed? */
  if (ip4_addr_cmp(gw, netif_ip4_gw(netif)) == 0) {

    LWIP_ASSERT("invalid pointer", old_gw != NULL);
    ip_addr_copy(*old_gw, *netif_ip_gw4(netif));

    


    ip4_addr_set(ip_2_ip4(&netif.gw), gw);
    IP_SET_TYPE_VAL(netif.gw, IPADDR_TYPE_V4);
    LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE, ("netif: GW address of interface %c%c set to %"U16_F".%"U16_F".%"U16_F".%"U16_F"\n",
                netif.name[0], netif.name[1],
                ip4_addr1_16(netif_ip4_gw(netif)),
                ip4_addr2_16(netif_ip4_gw(netif)),
                ip4_addr3_16(netif_ip4_gw(netif)),
                ip4_addr4_16(netif_ip4_gw(netif))));
    return 1; /* gateway changed */
  }
  return 0; /* gateway unchanged */
}

/*
 * @ingroup netif_ip4
 * Change the default gateway for a network interface
 *
 * @param netif the network interface to change
 * @param gw the new default gateway
 *
 * @note call netif_set_addr() if you also want to change ip address and netmask
 */
pub fn 
netif_set_gw(netif: &mut netif,  gw: &mut ip4_addr)
{

  ip_addr_t old_gw_val;
  old_gw: &mut ip_addr_t = &old_gw_val;

  old_gw: &mut ip_addr_t = NULL;

  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ERROR("netif_set_gw: invalid netif", netif != NULL, return);

  /* Don't propagate NULL pointer (IPv4 ANY) to subsequent functions */
  if (gw == NULL) {
    gw = IP4_ADDR_ANY4;
  }

  if (netif_do_set_gw(netif, gw, old_gw)) {

    netif_ext_callback_args_t args;
    args.ipv4_changed.old_gw = old_gw;
    netif_invoke_ext_callback(netif, LWIP_NSC_IPV4_GATEWAY_CHANGED, &args);

  }
}

/*
 * @ingroup netif_ip4
 * Change IP address configuration for a network interface (including netmask
 * and default gateway).
 *
 * @param netif the network interface to change
 * @param ipaddr the new IP address
 * @param netmask the new netmask
 * @param gw the new default gateway
 */
pub fn 
netif_set_addr(
  netif: &mut netif, 
  ipaddr: &mut ip4_addr, 
  netmask: &mut ip4_addr,
  gw: &mut ip4_addr)
{

  netif_nsc_reason_t change_reason = LWIP_NSC_NONE;
  netif_ext_callback_args_t cb_args;
  ip_addr_t old_nm_val;
  ip_addr_t old_gw_val;
  old_nm: &mut ip_addr_t = &old_nm_val;
  old_gw: &mut ip_addr_t = &old_gw_val;
  // old_nm: &mut ip_addr_t = NULL;
  // old_gw: &mut ip_addr_t = NULL;

  ip_addr_t old_addr;
  remove: i32;

  LWIP_ASSERT_CORE_LOCKED();

  /* Don't propagate NULL pointer (IPv4 ANY) to subsequent functions */
  if (ipaddr == NULL) {
    ipaddr = IP4_ADDR_ANY4;
  }
  if (netmask == NULL) {
    netmask = IP4_ADDR_ANY4;
  }
  if (gw == NULL) {
    gw = IP4_ADDR_ANY4;
  }

  remove = ip4_addr_isany(ipaddr);
  if (remove) {
    /* when removing an address, we have to remove it *before* changing netmask/gw
       to ensure that tcp RST segment can be sent correctly */
    if (netif_do_set_ipaddr(netif, ipaddr, &old_addr)) {

      change_reason |= LWIP_NSC_IPV4_ADDRESS_CHANGED;
      cb_args.ipv4_changed.old_address = &old_addr;

    }
  }
  if (netif_do_set_netmask(netif, netmask, old_nm)) {

    change_reason |= LWIP_NSC_IPV4_NETMASK_CHANGED;
    cb_args.ipv4_changed.old_netmask = old_nm;

  }
  if (netif_do_set_gw(netif, gw, old_gw)) {

    change_reason |= LWIP_NSC_IPV4_GATEWAY_CHANGED;
    cb_args.ipv4_changed.old_gw = old_gw;

  }
  if (!remove) {
    /* set ipaddr last to ensure netmask/gw have been set when status callback is called */
    if (netif_do_set_ipaddr(netif, ipaddr, &old_addr)) {

      change_reason |= LWIP_NSC_IPV4_ADDRESS_CHANGED;
      cb_args.ipv4_changed.old_address = &old_addr;

    }
  }


  if (change_reason != LWIP_NSC_NONE) {
    change_reason |= LWIP_NSC_IPV4_SETTINGS_CHANGED;
    netif_invoke_ext_callback(netif, change_reason, &cb_args);
  }

}


/*
 * @ingroup netif
 * Remove a network interface from the list of lwIP netifs.
 *
 * @param netif the network interface to remove
 */
pub fn 
netif_remove(netif: &mut netif)
{

  i: i32;


  LWIP_ASSERT_CORE_LOCKED();

  if (netif == NULL) {
    return;
  }

  netif_invoke_ext_callback(netif, LWIP_NSC_NETIF_REMOVED, NULL);


  if (!ip4_addr_isany_val(*netif_ip4_addr(netif))) {
    netif_do_ip_addr_changed(netif_ip_addr4(netif), NULL);
  }


  /* stop IGMP processing */
  if (netif.flags & NETIF_FLAG_IGMP) {
    igmp_stop(netif);
  }




  for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
    if (ip6_addr_isvalid(netif_ip6_addr_state(netif, i))) {
      netif_do_ip_addr_changed(netif_ip_addr6(netif, i), NULL);
    }
  }

  /* stop MLD processing */
  mld6_stop(netif);


  if (netif_is_up(netif)) {
    /* set netif down before removing (call callback function) */
    netif_set_down(netif);
  }

  mib2_remove_ip4(netif);

  /* this netif is default? */
  if (netif_default == netif) {
    /* reset default netif */
    netif_set_default(NULL);
  }

  /*  is it the first netif? */
  if (netif_list == netif) {
    netif_list = netif.next;
  } else {
    /*  look for netif further down the list */
    tmp_netif: &mut netif;
    NETIF_FOREACH(tmp_netif) {
      if (tmp_netif.next == netif) {
        tmp_netif.next = netif.next;
        break;
      }
    }
    if (tmp_netif == NULL) {
      return; /* netif is not on the list */
    }
  }

  mib2_netif_removed(netif);

  if (netif.remove_callback) {
    netif.remove_callback(netif);
  }

  LWIP_DEBUGF( NETIF_DEBUG, ("netif_remove: removed netif\n") );
}

/*
 * @ingroup netif
 * Set a network interface as the default network interface
 * (used to output all packets for which no specific route is found)
 *
 * @param netif the default network interface
 */
pub fn 
netif_set_default(netif: &mut netif)
{
  LWIP_ASSERT_CORE_LOCKED();

  if (netif == NULL) {
    /* remove default route */
    mib2_remove_route_ip4(1, netif);
  } else {
    /* install default route */
    mib2_add_route_ip4(1, netif);
  }
  netif_default = netif;
  LWIP_DEBUGF(NETIF_DEBUG, ("netif: setting default interface %c%c\n",
                            netif ? netif.name[0] : '\'', netif ? netif.name[1] : '\''));
}

/*
 * @ingroup netif
 * Bring an interface up, available for processing
 * traffic.
 */
pub fn 
netif_set_up(netif: &mut netif)
{
  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ERROR("netif_set_up: invalid netif", netif != NULL, return);

  if (!(netif.flags & NETIF_FLAG_UP)) {
    netif_set_flags(netif, NETIF_FLAG_UP);

    MIB2_COPY_SYSUPTIME_TO(&netif.ts);

    NETIF_STATUS_CALLBACK(netif);


    {
      netif_ext_callback_args_t args;
      args.status_changed.state = 1;
      netif_invoke_ext_callback(netif, LWIP_NSC_STATUS_CHANGED, &args);
    }


    netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV4 | NETIF_REPORT_TYPE_IPV6);

    nd6_restart_netif(netif);

  }
}

/* Send ARP/IGMP/MLD/RS events, e.g. on link-up/netif-up or addr-change
 */
pub fn
netif_issue_reports(netif: &mut netif, report_type: u8)
{
  LWIP_ASSERT("netif_issue_reports: invalid netif", netif != NULL);

  /* Only send reports when both link and admin states are up */
  if (!(netif.flags & NETIF_FLAG_LINK_UP) ||
      !(netif.flags & NETIF_FLAG_UP)) {
    return;
  }


  if ((report_type & NETIF_REPORT_TYPE_IPV4) &&
      !ip4_addr_isany_val(*netif_ip4_addr(netif))) {

    /* For Ethernet network interfaces, we would like to send a "gratuitous ARP" */
    if (netif.flags & (NETIF_FLAG_ETHARP)) {
      etharp_gratuitous(netif);
    }



    /* resend IGMP memberships */
    if (netif.flags & NETIF_FLAG_IGMP) {
      igmp_report_groups(netif);
    }

  }



  if (report_type & NETIF_REPORT_TYPE_IPV6) {

    /* send mld memberships */
    mld6_report_groups(netif);

  }

}

/*
 * @ingroup netif
 * Bring an interface down, disabling any traffic processing.
 */
pub fn 
netif_set_down(netif: &mut netif)
{
  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ERROR("netif_set_down: invalid netif", netif != NULL, return);

  if (netif.flags & NETIF_FLAG_UP) {

    {
      netif_ext_callback_args_t args;
      args.status_changed.state = 0;
      netif_invoke_ext_callback(netif, LWIP_NSC_STATUS_CHANGED, &args);
    }


    netif_clear_flags(netif, NETIF_FLAG_UP);
    MIB2_COPY_SYSUPTIME_TO(&netif.ts);


    if (netif.flags & NETIF_FLAG_ETHARP) {
      etharp_cleanup_netif(netif);
    }



    nd6_cleanup_netif(netif);


    NETIF_STATUS_CALLBACK(netif);
  }
}


/*
 * @ingroup netif
 * Set callback to be called when interface is brought up/down or address is changed while up
 */
pub fn 
netif_set_status_callback(netif: &mut netif, netif_status_callback_fn status_callback)
{
  LWIP_ASSERT_CORE_LOCKED();

  if (netif) {
    netif.status_callback = status_callback;
  }
}



/*
 * @ingroup netif
 * Set callback to be called when the interface has been removed
 */
pub fn 
netif_set_remove_callback(netif: &mut netif, netif_status_callback_fn remove_callback)
{
  LWIP_ASSERT_CORE_LOCKED();

  if (netif) {
    netif.remove_callback = remove_callback;
  }
}


/*
 * @ingroup netif
 * Called by a driver when its link goes up
 */
pub fn 
netif_set_link_up(netif: &mut netif)
{
  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ERROR("netif_set_link_up: invalid netif", netif != NULL, return);

  if (!(netif.flags & NETIF_FLAG_LINK_UP)) {
    netif_set_flags(netif, NETIF_FLAG_LINK_UP);


    dhcp_network_changed(netif);



    autoip_network_changed(netif);


    netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV4 | NETIF_REPORT_TYPE_IPV6);

    nd6_restart_netif(netif);


    NETIF_LINK_CALLBACK(netif);

    {
      netif_ext_callback_args_t args;
      args.link_changed.state = 1;
      netif_invoke_ext_callback(netif, LWIP_NSC_LINK_CHANGED, &args);
    }

  }
}

/*
 * @ingroup netif
 * Called by a driver when its link goes down
 */
pub fn 
netif_set_link_down(netif: &mut netif)
{
  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ERROR("netif_set_link_down: invalid netif", netif != NULL, return);

  if (netif.flags & NETIF_FLAG_LINK_UP) {
    netif_clear_flags(netif, NETIF_FLAG_LINK_UP);
    NETIF_LINK_CALLBACK(netif);

    {
      netif_ext_callback_args_t args;
      args.link_changed.state = 0;
      netif_invoke_ext_callback(netif, LWIP_NSC_LINK_CHANGED, &args);
    }

  }
}


/*
 * @ingroup netif
 * Set callback to be called when link is brought up/down
 */
pub fn 
netif_set_link_callback(netif: &mut netif, netif_status_callback_fn link_callback)
{
  LWIP_ASSERT_CORE_LOCKED();

  if (netif) {
    netif.link_callback = link_callback;
  }
}



/*
 * @ingroup netif
 * Send an IP packet to be received on the same netif (loopif-like).
 * The pbuf is simply copied and handed back to netif.input.
 * In multithreaded mode, this is done directly since netif.input must put
 * the packet on a queue.
 * In callback mode, the packet is put on an internal queue and is fed to
 * netif.input by netif_poll().
 *
 * @param netif the lwip network interface structure
 * @param p the (IP) packet to 'send'
 * @return ERR_OK if the packet has been sent
 *         ERR_MEM if the pbuf used to copy the packet couldn't be allocated
 */
pub fn 
netif_loop_output(netif: &mut netif, p: &mut pbuf)
{
  r: &mut pbuf;
  let err: err_t;
  last: &mut pbuf;

  clen: u16 = 0;

  /* If we have a loopif, SNMP counters are adjusted for it,
   * if not they are adjusted for 'netif'. */


  stats_if: &mut netif = &loop_netif;
 /* LWIP_HAVE_LOOPIF */
  stats_if: &mut netif = netif;



  schedule_poll: u8 = 0;

  SYS_ARCH_DECL_PROTECT(lev);

  LWIP_ASSERT("netif_loop_output: invalid netif", netif != NULL);
  LWIP_ASSERT("netif_loop_output: invalid pbuf", p != NULL);

  /* Allocate a new pbuf */
  r = pbuf_alloc(PBUF_LINK, p.tot_len, PBUF_RAM);
  if (r == NULL) {
    LINK_STATS_INC(link.memerr);
    LINK_STATS_INC(link.drop);
    MIB2_STATS_NETIF_INC(stats_if, ifoutdiscards);
    return ERR_MEM;
  }

  clen = pbuf_clen(r);
  /* check for overflow or too many pbuf on queue */
  if (((netif.loop_cnt_current + clen) < netif.loop_cnt_current) ||
      ((netif.loop_cnt_current + clen) > LWIP_MIN(LWIP_LOOPBACK_MAX_PBUFS, 0xFFFF))) {
    pbuf_free(r);
    LINK_STATS_INC(link.memerr);
    LINK_STATS_INC(link.drop);
    MIB2_STATS_NETIF_INC(stats_if, ifoutdiscards);
    return ERR_MEM;
  }
  netif.loop_cnt_current = (netif.loop_cnt_current + clen);


  /* Copy the whole pbuf queue p into the single pbuf r */
  if ((err = pbuf_copy(r, p)) != ERR_OK) {
    pbuf_free(r);
    LINK_STATS_INC(link.memerr);
    LINK_STATS_INC(link.drop);
    MIB2_STATS_NETIF_INC(stats_if, ifoutdiscards);
    return err;
  }

  /* Put the packet on a linked list which gets emptied through calling
     netif_poll(). */

  /* let last poto: i32 the last pbuf in chain r */
  for (last = r; last.next != NULL; last = last.next) {
    /* nothing to do here, just get to the last pbuf */
  }

  SYS_ARCH_PROTECT(lev);
  if (netif.loop_first != NULL) {
    LWIP_ASSERT("if first != NULL, last must also be != NULL", netif.loop_last != NULL);
    netif.loop_last.next = r;
    netif.loop_last = last;
  } else {
    netif.loop_first = r;
    netif.loop_last = last;

    /* No existing packets queued, schedule poll */
    schedule_poll = 1;

  }
  SYS_ARCH_UNPROTECT(lev);

  LINK_STATS_INC(link.xmit);
  MIB2_STATS_NETIF_ADD(stats_if, ifoutoctets, p.tot_len);
  MIB2_STATS_NETIF_INC(stats_if, ifoutucastpkts);


  /* For multithreading environment, schedule a call to netif_poll */
  if (schedule_poll) {
    tcpip_try_callback((tcpip_callback_fn)netif_poll, netif);
  }


  return ERR_OK;
}



pub fn netif_loop_output_ipv4(netif: &mut netif, p: &mut pbuf,  addr: &mut ip4_addr) -> Result<(), LwipError>
{
  
  return netif_loop_output(netif, p);
}



pub fn netif_loop_output_ipv6(netif: &mut netif, p: &mut pbuf,  addr: &mut ip6_addr_t) -> Result<(), LwipError>
{
  
  return netif_loop_output(netif, p);
}




/*
 * Call netif_poll() in the main loop of your application. This is to prevent
 * reentering non-reentrant functions like tcp_input(). Packets passed to
 * netif_loop_output() are put on a list that is passed to netif.input() by
 * netif_poll().
 */
pub fn 
netif_poll(netif: &mut netif)
{
  /* If we have a loopif, SNMP counters are adjusted for it,
   * if not they are adjusted for 'netif'. */


  stats_if: &mut netif = &loop_netif;
 /* LWIP_HAVE_LOOPIF */
  stats_if: &mut netif = netif;


  SYS_ARCH_DECL_PROTECT(lev);

  LWIP_ASSERT("netif_poll: invalid netif", netif != NULL);

  /* Get a packet from the list. With SYS_LIGHTWEIGHT_PROT=1, this is protected */
  SYS_ARCH_PROTECT(lev);
  while (netif.loop_first != NULL) {
    in: &mut pbuf, *in_end;

    clen: u8 = 1;


    in = in_end = netif.loop_first;
    while (in_end.len != in_end.tot_len) {
      LWIP_ASSERT("bogus pbuf: len != tot_len but next == NULL!", in_end.next != NULL);
      in_end = in_end.next;

      clen+= 1;

    }

    /* adjust the number of pbufs on queue */
    LWIP_ASSERT("netif.loop_cnt_current underflow",
                ((netif.loop_cnt_current - clen) < netif.loop_cnt_current));
    netif.loop_cnt_current = (netif.loop_cnt_current - clen);


    /* 'in_end' now points to the last pbuf from 'in' */
    if (in_end == netif.loop_last) {
      /* this was the last pbuf in the list */
      netif.loop_first = netif.loop_last = NULL;
    } else {
      /* pop the pbuf off the list */
      netif.loop_first = in_end.next;
      LWIP_ASSERT("should not be null since first != last!", netif.loop_first != NULL);
    }
    /* De-queue the pbuf from its successors on the 'loop_' list. */
    in_end.next = NULL;
    SYS_ARCH_UNPROTECT(lev);

    in.if_idx = netif_get_index(netif);

    LINK_STATS_INC(link.recv);
    MIB2_STATS_NETIF_ADD(stats_if, ifinoctets, in.tot_len);
    MIB2_STATS_NETIF_INC(stats_if, ifinucastpkts);
    /* loopback packets are always IP packets! */
    if (ip_input(in, netif) != ERR_OK) {
      pbuf_free(in);
    }
    SYS_ARCH_PROTECT(lev);
  }
  SYS_ARCH_UNPROTECT(lev);
}


/*
 * Calls netif_poll() for every netif on the netif_list.
 */
pub fn 
netif_poll_all()
{
  netif: &mut netif;
  /* loop through netifs */
  NETIF_FOREACH(netif) {
    netif_poll(netif);
  }
}




/*
 * @ingroup netif_cd
 * Allocate an index to store data in client_data member of NetIfc.
 * Returned value is an index in mentioned array.
 * @see LWIP_NUM_NETIF_CLIENT_DATA
 */
u8
netif_alloc_client_data_id()
{
  result: u8 = netif_client_id;
  netif_client_id+= 1;

  LWIP_ASSERT_CORE_LOCKED();


#error LWIP_NUM_NETIF_CLIENT_DATA must be <= 256

  LWIP_ASSERT("Increase LWIP_NUM_NETIF_CLIENT_DATA in lwipopts.h", result < LWIP_NUM_NETIF_CLIENT_DATA);
  return (result + LWIP_NETIF_CLIENT_DATA_INDEX_MAX);
}



/*
 * @ingroup netif_ip6
 * Change an IPv6 address of a network interface
 *
 * @param netif the network interface to change
 * @param addr_idx index of the IPv6 address
 * @param addr6 the new IPv6 address
 *
 * @note call netif_ip6_addr_set_state() to set the address valid/temptative
 */
pub fn 
netif_ip6_addr_set(netif: &mut netif, s8_t addr_idx,  addr6: &mut ip6_addr_t)
{
  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ASSERT("netif_ip6_addr_set: invalid netif", netif != NULL);
  LWIP_ASSERT("netif_ip6_addr_set: invalid addr6", addr6 != NULL);

  netif_ip6_addr_set_parts(netif, addr_idx, addr6.addr[0], addr6.addr[1],
                           addr6.addr[2], addr6.addr[3]);
}

/*
 * Change an IPv6 address of a network interface (internal version taking 4 * u32)
 *
 * @param netif the network interface to change
 * @param addr_idx index of the IPv6 address
 * @param i0 word0 of the new IPv6 address
 * @param i1 word1 of the new IPv6 address
 * @param i2 word2 of the new IPv6 address
 * @param i3 word3 of the new IPv6 address
 */
pub fn 
netif_ip6_addr_set_parts(netif: &mut netif, s8_t addr_idx, i0: u32, i1: u32, i2: u32, i3: u32)
{
  ip_addr_t old_addr;
  ip_addr_t new_ipaddr;
  LWIP_ASSERT_CORE_LOCKED();
  LWIP_ASSERT("netif != NULL", netif != NULL);
  LWIP_ASSERT("invalid index", addr_idx < LWIP_IPV6_NUM_ADDRESSES);

  ip6_addr_copy(*ip_2_ip6(&old_addr), *netif_ip6_addr(netif, addr_idx));
  IP_SET_TYPE_VAL(old_addr, IPADDR_TYPE_V6);

  /* address is actually being changed? */
  if ((ip_2_ip6(&old_addr).addr[0] != i0) || (ip_2_ip6(&old_addr).addr[1] != i1) ||
      (ip_2_ip6(&old_addr).addr[2] != i2) || (ip_2_ip6(&old_addr).addr[3] != i3)) {
    LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_STATE, ("netif_ip6_addr_set: netif address being changed\n"));

    IP_ADDR6(&new_ipaddr, i0, i1, i2, i3);
    ip6_addr_assign_zone(ip_2_ip6(&new_ipaddr), IP6_UNICAST, netif);

    if (ip6_addr_isvalid(netif_ip6_addr_state(netif, addr_idx))) {
      netif_do_ip_addr_changed(netif_ip_addr6(netif, addr_idx), &new_ipaddr);
    }
    /* @todo: remove/readd mib2 ip6 entries? */

    ip_addr_copy(netif.ip6_addr[addr_idx], new_ipaddr);

    if (ip6_addr_isvalid(netif_ip6_addr_state(netif, addr_idx))) {
      netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV6);
      NETIF_STATUS_CALLBACK(netif);
    }


    {
      netif_ext_callback_args_t args;
      args.ipv6_set.addr_index  = addr_idx;
      args.ipv6_set.old_address = &old_addr;
      netif_invoke_ext_callback(netif, LWIP_NSC_IPV6_SET, &args);
    }

  }

  LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE, ("netif: IPv6 address %d of interface %c%c set to %s/0x%"X8_F"\n",
              addr_idx, netif.name[0], netif.name[1], ip6addr_ntoa(netif_ip6_addr(netif, addr_idx)),
              netif_ip6_addr_state(netif, addr_idx)));
}

/*
 * @ingroup netif_ip6
 * Change the state of an IPv6 address of a network interface
 * (INVALID, TEMPTATIVE, PREFERRED, DEPRECATED, where TEMPTATIVE
 * includes the number of checks done, see ip6_addr.h)
 *
 * @param netif the network interface to change
 * @param addr_idx index of the IPv6 address
 * @param state the new IPv6 address state
 */
pub fn 
netif_ip6_addr_set_state(netif: &mut netif, s8_t addr_idx, state: u8)
{
  old_state: u8;
  LWIP_ASSERT_CORE_LOCKED();
  LWIP_ASSERT("netif != NULL", netif != NULL);
  LWIP_ASSERT("invalid index", addr_idx < LWIP_IPV6_NUM_ADDRESSES);

  old_state = netif_ip6_addr_state(netif, addr_idx);
  /* state is actually being changed? */
  if (old_state != state) {
    old_valid: u8 = old_state & IP6_ADDR_VALID;
    new_valid: u8 = state & IP6_ADDR_VALID;
    LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_STATE, ("netif_ip6_addr_set_state: netif address state being changed\n"));


    /* Reevaluate solicited-node multicast group membership. */
    if (netif.flags & NETIF_FLAG_MLD6) {
      nd6_adjust_mld_membership(netif, addr_idx, state);
    }


    if (old_valid && !new_valid) {
      /* address about to be removed by setting invalid */
      netif_do_ip_addr_changed(netif_ip_addr6(netif, addr_idx), NULL);
      /* @todo: remove mib2 ip6 entries? */
    }
    netif.ip6_addr_state[addr_idx] = state;

    if (!old_valid && new_valid) {
      /* address added by setting valid */
      /* This is a good moment to check that the address is properly zoned. */
      IP6_ADDR_ZONECHECK_NETIF(netif_ip6_addr(netif, addr_idx), netif);
      /* @todo: add mib2 ip6 entries? */
      netif_issue_reports(netif, NETIF_REPORT_TYPE_IPV6);
    }
    if ((old_state & ~IP6_ADDR_TENTATIVE_COUNT_MASK) !=
        (state     & ~IP6_ADDR_TENTATIVE_COUNT_MASK)) {
      /* address state has changed -> call the callback function */
      NETIF_STATUS_CALLBACK(netif);
    }


    {
      netif_ext_callback_args_t args;
      args.ipv6_addr_state_changed.addr_index = addr_idx;
      args.ipv6_addr_state_changed.old_state  = old_state;
      args.ipv6_addr_state_changed.address    = netif_ip_addr6(netif, addr_idx);
      netif_invoke_ext_callback(netif, LWIP_NSC_IPV6_ADDR_STATE_CHANGED, &args);
    }

  }
  LWIP_DEBUGF(NETIF_DEBUG | LWIP_DBG_TRACE | LWIP_DBG_STATE, ("netif: IPv6 address %d of interface %c%c set to %s/0x%"X8_F"\n",
              addr_idx, netif.name[0], netif.name[1], ip6addr_ntoa(netif_ip6_addr(netif, addr_idx)),
              netif_ip6_addr_state(netif, addr_idx)));
}

/*
 * Checks if a specific local address is present on the netif and returns its
 * index. Depending on its state, it may or may not be assigned to the
 * interface (as per RFC terminology).
 *
 * The given address may or may not be zoned (i.e., have a zone index other
 * than IP6_NO_ZONE). If the address is zoned, it must have the correct zone
 * for the given netif, or no match will be found.
 *
 * @param netif the netif to check
 * @param ip6addr the IPv6 address to find
 * @return >= 0: address found, this is its index
 *         -1: address not found on this netif
 */
s8_t
netif_get_ip6_addr_match(netif: &mut netif,  ip6addr: &mut ip6_addr_t)
{
  s8_t i;

  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ASSERT("netif_get_ip6_addr_match: invalid netif", netif != NULL);
  LWIP_ASSERT("netif_get_ip6_addr_match: invalid ip6addr", ip6addr != NULL);


  if (ip6_addr_has_zoneip6addr && !ip6_addr_test_zone(ip6addr, netif)) {
    return -1; /* wrong zone, no match */
  }


  for (i = 0; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
    if (!ip6_addr_isinvalid(netif_ip6_addr_state(netif, i)) &&
        ip6_addr_cmp_zoneless(netif_ip6_addr(netif, i), ip6addr)) {
      return i;
    }
  }
  return -1;
}

/*
 * @ingroup netif_ip6
 * Create a link-local IPv6 address on a netif (stored in slot 0)
 *
 * @param netif the netif to create the address on
 * @param from_mac_48bit if != 0, assume hwadr is a 48-bit MAC address (std conversion)
 *                       if == 0, use hwaddr directly as interface ID
 */
pub fn 
netif_create_ip6_linklocal_address(netif: &mut netif, from_mac_48bit: u8)
{
  i: u8, addr_index;

  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ASSERT("netif_create_ip6_linklocal_address: invalid netif", netif != NULL);

  /* Link-local prefix. */
  ip_2_ip6(&netif.ip6_addr[0]).addr[0] = PP_HTONL(0xfe800000);
  ip_2_ip6(&netif.ip6_addr[0]).addr[1] = 0;

  /* Generate interface ID. */
  if (from_mac_48bit) {
    /* Assume hwaddr is a 48-bit IEEE 802 MAC. Convert to EUI-64 address. Complement Group bit. */
    ip_2_ip6(&netif.ip6_addr[0]).addr[2] = lwip_htonl((((u32)(netif.hwaddr[0] ^ 0x02)) << 24) |
        ((u32)(netif.hwaddr[1]) << 16) |
        ((u32)(netif.hwaddr[2]) << 8) |
        (0xff));
    ip_2_ip6(&netif.ip6_addr[0]).addr[3] = lwip_htonl((u32)(0xfeul << 24) |
        ((u32)(netif.hwaddr[3]) << 16) |
        ((u32)(netif.hwaddr[4]) << 8) |
        (netif.hwaddr[5]));
  } else {
    /* Use hwaddr directly as interface ID. */
    ip_2_ip6(&netif.ip6_addr[0]).addr[2] = 0;
    ip_2_ip6(&netif.ip6_addr[0]).addr[3] = 0;

    addr_index = 3;
    for (i = 0; (i < 8) && (i < netif.hwaddr_len); i+= 1) {
      if (i == 4) {
        addr_index -= 1;
      }
      ip_2_ip6(&netif.ip6_addr[0]).addr[addr_index] |= lwip_htonl(((u32)(netif.hwaddr[netif.hwaddr_len - i - 1])) << (8 * (i & 0x03)));
    }
  }

  /* Set a link-local zone. Even though the zone is implied by the owning
   * netif, setting the zone anyway has two important conceptual advantages:
   * 1) it avoids the need for a ton of exceptions in internal code, allowing
   *    e.g. ip6_addr_cmp() to be used on local addresses;
   * 2) the properly zoned address is visible externally, e.g. when any outside
   *    code enumerates available addresses or uses one to bind a socket.
   * Any external code unaware of address scoping is likely to just ignore the
   * zone field, so this should not create any compatibility problems. */
  ip6_addr_assign_zone(ip_2_ip6(&netif.ip6_addr[0]), IP6_UNICAST, netif);

  /* Set address state. */

  /* Will perform duplicate address detection (DAD). */
  netif_ip6_addr_set_state(netif, 0, IP6_ADDR_TENTATIVE);

  /* Consider address valid. */
  netif_ip6_addr_set_state(netif, 0, IP6_ADDR_PREFERRED);

}

/*
 * @ingroup netif_ip6
 * This function allows for the easy addition of a new IPv6 address to an interface.
 * It takes care of finding an empty slot and then sets the address tentative
 * (to make sure that all the subsequent processing happens).
 *
 * @param netif netif to add the address on
 * @param ip6addr address to add
 * @param chosen_idx if != NULL, the chosen IPv6 address index will be stored here
 */
pub fn 
netif_add_ip6_address(netif: &mut netif,  ip6addr: &mut ip6_addr_t, s8_t *chosen_idx)
{
  s8_t i;

  LWIP_ASSERT_CORE_LOCKED();

  LWIP_ASSERT("netif_add_ip6_address: invalid netif", netif != NULL);
  LWIP_ASSERT("netif_add_ip6_address: invalid ip6addr", ip6addr != NULL);

  i = netif_get_ip6_addr_match(netif, ip6addr);
  if (i >= 0) {
    /* Address already added */
    if (chosen_idx != NULL) {
      *chosen_idx = i;
    }
    return ERR_OK;
  }

  /* Find a free slot. The first one is reserved for link-local addresses. */
  for (i = ip6_addr_islinklocalip6addr ? 0 : 1; i < LWIP_IPV6_NUM_ADDRESSES; i+= 1) {
    if (ip6_addr_isinvalid(netif_ip6_addr_state(netif, i))) {
      ip_addr_copy_from_ip6(netif.ip6_addr[i], *ip6addr);
      ip6_addr_assign_zone(ip_2_ip6(&netif.ip6_addr[i]), IP6_UNICAST, netif);
      netif_ip6_addr_set_state(netif, i, IP6_ADDR_TENTATIVE);
      if (chosen_idx != NULL) {
        *chosen_idx = i;
      }
      return ERR_OK;
    }
  }

  if (chosen_idx != NULL) {
    *chosen_idx = -1;
  }
  return ERR_VAL;
}

/* Dummy IPv6 output function for netifs not supporting IPv6
 */
pub fn netif_null_output_ip6(netif: &mut netif, p: &mut pbuf,  ipaddr: &mut ip6_addr_t) -> Result<(), LwipError>
{
  
  
  

  return ERR_IF;
}



/* Dummy IPv4 output function for netifs not supporting IPv4
 */
pub fn netif_null_output_ip4(netif: &mut netif, p: &mut pbuf,  ipaddr: &mut ip4_addr) -> Result<(), LwipError>
{
  
  
  

  return ERR_IF;
}


/*
* @ingroup netif
* Return the interface index for the netif with name
* or NETIF_NO_INDEX if not found/on error
*
* @param name the name of the netif
*/
u8
netif_name_to_index(name: &String)
{
  netif: &mut netif = netif_find(name);
  if (netif != NULL) {
    return netif_get_index(netif);
  }
  /* No name found, return invalid index */
  return NETIF_NO_INDEX;
}

/*
* @ingroup netif
* Return the interface name for the netif matching index
* or NULL if not found/on error
*
* @param idx the interface index of the netif
* @param name char buffer of at least NETIF_NAMESIZE bytes
*/
char *
netif_index_to_name(idx: u8, name: &mut String)
{
  netif: &mut netif = netif_get_by_index(idx);

  if (netif != NULL) {
    name[0] = netif.name[0];
    name[1] = netif.name[1];
    lwip_itoa(&name[2], NETIF_NAMESIZE - 2, netif_index_to_num(idx));
    return name;
  }
  return NULL;
}

/*
* @ingroup netif
* Return the interface for the netif index
*
* @param idx index of netif to find
*/
NetIfc *
netif_get_by_index(idx: u8)
{
  netif: &mut netif;

  LWIP_ASSERT_CORE_LOCKED();

  if (idx != NETIF_NO_INDEX) {
    NETIF_FOREACH(netif) {
      if (idx == netif_get_index(netif)) {
        return netif; /* found! */
      }
    }
  }

  return NULL;
}

/*
 * @ingroup netif
 * Find a network interface by searching for its name
 *
 * @param name the name of the netif (like netif.name) plus concatenated number
 * in ascii representation (e.g. 'en0')
 */
NetIfc *
netif_find(name: &String)
{
  netif: &mut netif;
  num: u8;

  LWIP_ASSERT_CORE_LOCKED();

  if (name == NULL) {
    return NULL;
  }

  num = atoi(&name[2]);

  NETIF_FOREACH(netif) {
    if (num == netif.num &&
        name[0] == netif.name[0] &&
        name[1] == netif.name[1]) {
      LWIP_DEBUGF(NETIF_DEBUG, ("netif_find: found %c%c\n", name[0], name[1]));
      return netif;
    }
  }
  LWIP_DEBUGF(NETIF_DEBUG, ("netif_find: didn't find %c%c\n", name[0], name[1]));
  return NULL;
}


/*
 * @ingroup netif
 * Add extended netif events listener
 * @param callback pointer to listener structure
 * @param fn callback function
 */
pub fn 
netif_add_ext_callback(netif_ext_callback_t *callback, netif_ext_callback_fn fn)
{
  LWIP_ASSERT_CORE_LOCKED();
  LWIP_ASSERT("callback must be != NULL", callback != NULL);
  LWIP_ASSERT("fn must be != NULL", fn != NULL);

  callback.callback_fn = fn;
  callback.next        = ext_callback;
  ext_callback          = callback;
}

/*
 * @ingroup netif
 * Remove extended netif events listener
 * @param callback pointer to listener structure
 */
pub fn 
netif_remove_ext_callback(netif_ext_callback_t* callback)
{
  netif_ext_callback_t *last, *iter;

  LWIP_ASSERT_CORE_LOCKED();
  LWIP_ASSERT("callback must be != NULL", callback != NULL);

  if (ext_callback == NULL) {
    return;
  }

  if (callback == ext_callback) {
    ext_callback = ext_callback.next;
  } else {
    last = ext_callback;
    for (iter = ext_callback.next; iter != NULL; last = iter, iter = iter.next) {
      if (iter == callback) {
        LWIP_ASSERT("last != NULL", last != NULL);
        last.next = callback.next;
        callback.next = NULL;
        return;
      }
    }
  }
}

/*
 * Invoke extended netif status event
 * @param netif netif that is affected by change
 * @param reason change reason
 * @param args depends on reason, see reason description
 */
pub fn 
netif_invoke_ext_callback(netif: &mut netif, netif_nsc_reason_t reason,  netif_ext_callback_args_t *args)
{
  netif_ext_callback_t *callback = ext_callback;

  LWIP_ASSERT("netif must be != NULL", netif != NULL);

  while (callback != NULL) {
    callback.callback_fn(netif, reason, args);
    callback = callback.next;
  }
}

