/*
 * @file
 * Management Information Base II (RFC1213) IP objects and functions.
 */

/*
 * Copyright (c) 2006 Axon Digital Design B.V., The Netherlands.
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *         Christiaan Simons <christiaan.simons@axon.tv>
 */















#define SYNC_NODE_NAME(node_name) node_name ## _synced
#define CREATE_LWIP_SYNC_NODE(oid, node_name) \
   static const struct snmp_threadsync_node node_name ## _synced = SNMP_CREATE_THREAD_SYNC_NODE(oid, &node_name.node, &snmp_mib2_lwip_locks);

#define SYNC_NODE_NAME(node_name) node_name
#define CREATE_LWIP_SYNC_NODE(oid, node_name)



/* --- ip .1.3.6.1.2.1.4 ----------------------------------------------------- */

pub fn ip_get_value(instance: &mut snmp_node_instance, value: &mut ())
{
  i32 *sint_ptr = (i32 *)value;
  u32 *uint_ptr = (u32 *)value;

  match (instance.node.oid) {
    1 => /* ipForwarding */

      /* forwarding */
      *sint_ptr = 1;

      /* not-forwarding */
      *sint_ptr = 2;

      return sizeof(*sint_ptr);
    2 => /* ipDefaultTTL */
      *sint_ptr = IP_DEFAULT_TTL;
      return sizeof(*sint_ptr);
    3 => /* ipInReceives */
      *uint_ptr = STATS_GET(mib2.ipinreceives);
      return sizeof(*uint_ptr);
    4 => /* ipInHdrErrors */
      *uint_ptr = STATS_GET(mib2.ipinhdrerrors);
      return sizeof(*uint_ptr);
    5 => /* ipInAddrErrors */
      *uint_ptr = STATS_GET(mib2.ipinaddrerrors);
      return sizeof(*uint_ptr);
    6 => /* ipForwDatagrams */
      *uint_ptr = STATS_GET(mib2.ipforwdatagrams);
      return sizeof(*uint_ptr);
    7 => /* ipInUnknownProtos */
      *uint_ptr = STATS_GET(mib2.ipinunknownprotos);
      return sizeof(*uint_ptr);
    8 => /* ipInDiscards */
      *uint_ptr = STATS_GET(mib2.ipindiscards);
      return sizeof(*uint_ptr);
    9 => /* ipInDelivers */
      *uint_ptr = STATS_GET(mib2.ipindelivers);
      return sizeof(*uint_ptr);
    10 => /* ipOutRequests */
      *uint_ptr = STATS_GET(mib2.ipoutrequests);
      return sizeof(*uint_ptr);
    11 => /* ipOutDiscards */
      *uint_ptr = STATS_GET(mib2.ipoutdiscards);
      return sizeof(*uint_ptr);
    12 => /* ipOutNoRoutes */
      *uint_ptr = STATS_GET(mib2.ipoutnoroutes);
      return sizeof(*uint_ptr);
    13 => /* ipReasmTimeout */

      *sint_ptr = IP_REASS_MAXAGE;

      *sint_ptr = 0;

      return sizeof(*sint_ptr);
    14 => /* ipReasmReqds */
      *uint_ptr = STATS_GET(mib2.ipreasmreqds);
      return sizeof(*uint_ptr);
    15 => /* ipReasmOKs */
      *uint_ptr = STATS_GET(mib2.ipreasmoks);
      return sizeof(*uint_ptr);
    16 => /* ipReasmFails */
      *uint_ptr = STATS_GET(mib2.ipreasmfails);
      return sizeof(*uint_ptr);
    17 => /* ipFragOKs */
      *uint_ptr = STATS_GET(mib2.ipfragoks);
      return sizeof(*uint_ptr);
    18 => /* ipFragFails */
      *uint_ptr = STATS_GET(mib2.ipfragfails);
      return sizeof(*uint_ptr);
    19 => /* ipFragCreates */
      *uint_ptr = STATS_GET(mib2.ipfragcreates);
      return sizeof(*uint_ptr);
    23 => /* ipRoutingDiscards: not supported -> always 0 */
      *uint_ptr = 0;
      return sizeof(*uint_ptr);
    _ =>
//      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("ip_get_value(): unknown id: %"S32_F"\n", instance.node.oid));
      break;
  }

  return 0;
}

/*
 * Test ip object value before setting.
 *
 * @param instance node instance
 * @param len return value space (in bytes)
 * @param value points to (varbind) space to copy value from.
 *
 * @note we allow set if the value matches the hardwired value,
 *   otherwise return badvalue.
 */
pub fn ip_set_test(instance: &mut snmp_node_instance, len: usize, value: &mut ())
{
  snmp_ret: err_t = SNMP_ERR_WRONGVALUE;
  i32 *sint_ptr = (i32 *)value;

  
  match (instance.node.oid) {
    1 => /* ipForwarding */

      /* forwarding */
      if (*sint_ptr == 1)

      /* not-forwarding */
      if (*sint_ptr == 2)

      {
        ret = SNMP_ERR_NOERROR;
      }
      break;
    2 => /* ipDefaultTTL */
      if (*sint_ptr == IP_DEFAULT_TTL) {
        ret = SNMP_ERR_NOERROR;
      }
      break;
    _ =>
//      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("ip_set_test(): unknown id: %"S32_F"\n", instance.node.oid));
      break;
  }

  return ret;
}

pub fn ip_set_value(instance: &mut snmp_node_instance, len: usize, value: &mut ())
{
  
  
  
  /* nothing to do here because in set_test we only accept values being the same as our own stored value -> no need to store anything */
  return SNMP_ERR_NOERROR;
}

/* --- ipAddrTable --- */

/* list of allowed value ranges for incoming OID */
static const struct snmp_oid_range ip_AddrTable_oid_ranges[] = {
  { 0, 0xff }, /* IP A */
  { 0, 0xff }, /* IP B */
  { 0, 0xff }, /* IP C */
  { 0, 0xff }  /* IP D */
};

pub fn ip_AddrTable_get_cell_value_core(netif: &mut NetIfc,  u32 *column, union snmp_variant_value *value, u32 *value_len)
{
  

  match (*column) {
    1 => /* ipAdEntAddr */
      value.u32 = netif_ip4_addr(netif).addr;
      break;
    2 => /* ipAdEntIfIndex */
      value.u32 = netif_to_num(netif);
      break;
    3 => /* ipAdEntNetMask */
      value.u32 = netif_ip4_netmask(netif).addr;
      break;
    4 => /* ipAdEntBcastAddr */
      /* lwIP oddity, there's no broadcast
         address in the netif we can rely on */
      value.u32 = IPADDR_BROADCAST & 1;
      break;
    5 => /* ipAdEntReasmMaxSize */

      /* @todo The theoretical maximum is IP_REASS_MAX_PBUFS * size of the pbufs,
       * but only if receiving one fragmented packet at a time.
       * The current solution is to calculate for 2 simultaneous packets...
       */
      value.u32 = (IP_HLEN + ((IP_REASS_MAX_PBUFS / 2) *
                               (PBUF_POOL_BUFSIZE - PBUF_LINK_ENCAPSULATION_HLEN - PBUF_LINK_HLEN - IP_HLEN)));

      /* @todo returning MTU would be a bad thing and
          returning a wild guess like '576' isn't good either */
      value.u32 = 0;

      break;
    _ =>
      return SNMP_ERR_NOSUCHINSTANCE;
  }

  return SNMP_ERR_NOERROR;
}

pub fn ip_AddrTable_get_cell_value(const u32 *column,  u32 *row_oid, row_oid_len: u8, union snmp_variant_value *value, u32 *value_len)
{
  let mut if_addr: LwipAddr;
  netif: &mut NetIfc;

  /* check if incoming OID length and if values are in plausible range */
  if (!snmp_oid_in_range(row_oid, row_oid_len, ip_AddrTable_oid_ranges, LWIP_ARRAYSIZE(ip_AddrTable_oid_ranges))) {
    return SNMP_ERR_NOSUCHINSTANCE;
  }

  /* get IP from incoming OID */
  snmp_oid_to_ip4(&row_oid[0], &ip); /* we know it succeeds because of oid_in_range check above */

  /* find netif with requested ip */
  NETIF_FOREACH(netif) {
    if (ip4_addr_cmp(&ip, netif_ip4_addr(netif))) {
      /* fill in object properties */
      return ip_AddrTable_get_cell_value_core(netif, column, value, value_len);
    }
  }

  /* not found */
  return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn ip_AddrTable_get_next_cell_instance_and_value(const u32 *column, row_oid: &mut snmp_obj_id, union snmp_variant_value *value, u32 *value_len)
{
  netif: &mut NetIfc;
  let state: snmp_next_oid_state;
  result_temp: u32[LWIP_ARRAYSIZE(ip_AddrTable_oid_ranges)];

  /* init struct to search next oid */
  snmp_next_oid_init(&state, row_oid.id, row_oid.len, result_temp, LWIP_ARRAYSIZE(ip_AddrTable_oid_ranges));

  /* iterate over all possible OIDs to find the next one */
  NETIF_FOREACH(netif) {
    test_oid: u32[LWIP_ARRAYSIZE(ip_AddrTable_oid_ranges)];
    snmp_ip4_to_oid(netif_ip4_addr(netif), &test_oid[0]);

    /* check generated OID: is it a candidate for the next one? */
    snmp_next_oid_check(&state, test_oid, LWIP_ARRAYSIZE(ip_AddrTable_oid_ranges), netif);
  }

  /* did we find a next one? */
  if (state.status == SNMP_NEXT_OID_STATUS_SUCCESS) {
    snmp_oid_assign(row_oid, state.next_oid, state.next_oid_len);
    /* fill in object properties */
    return ip_AddrTable_get_cell_value_core((NetIfc *)state.reference, column, value, value_len);
  }

  /* not found */
  return SNMP_ERR_NOSUCHINSTANCE;
}

/* --- ipRouteTable --- */

/* list of allowed value ranges for incoming OID */
static const struct snmp_oid_range ip_RouteTable_oid_ranges[] = {
  { 0, 0xff }, /* IP A */
  { 0, 0xff }, /* IP B */
  { 0, 0xff }, /* IP C */
  { 0, 0xff }, /* IP D */
};

pub fn ip_RouteTable_get_cell_value_core(netif: &mut NetIfc, default_route: u8,  u32 *column, union snmp_variant_value *value, u32 *value_len)
{
  match (*column) {
    1 => /* ipRouteDest */
      if (default_route) {
        /* default rte has 0.0.0.0 dest */
        value.u32 = IP4_ADDR_ANY4.addr;
      } else {
        /* netifs have netaddress dest */
        let mut if_addr: LwipAddr;
        ip4_addr_get_network(&tmp, netif_ip4_addr(netif), netif_ip4_netmask(netif));
        value.u32 = tmp.addr;
      }
      break;
    2 => /* ipRouteIfIndex */
      value.u32 = netif_to_num(netif);
      break;
    3 => /* ipRouteMetric1 */
      if (default_route) {
        value.s32 = 1; /* default */
      } else {
        value.s32 = 0; /* normal */
      }
      break;
    4 => /* ipRouteMetric2 */
    5 => /* ipRouteMetric3 */
    6 => /* ipRouteMetric4 */
      value.s32 = -1; /* none */
      break;
    7 => /* ipRouteNextHop */
      if (default_route) {
        /* default rte: gateway */
        value.u32 = netif_ip4_gw(netif).addr;
      } else {
        /* other rtes: netif ip_addr  */
        value.u32 = netif_ip4_addr(netif).addr;
      }
      break;
    8 => /* ipRouteType */
      if (default_route) {
        /* default rte is indirect */
        value.u32 = 4; /* indirect */
      } else {
        /* other rtes are direct */
        value.u32 = 3; /* direct */
      }
      break;
    9 => /* ipRouteProto */
      /* locally defined routes */
      value.u32 = 2; /* local */
      break;
    10 => /* ipRouteAge */
      /* @todo (sysuptime - timestamp last change) / 100 */
      value.u32 = 0;
      break;
    11 => /* ipRouteMask */
      if (default_route) {
        /* default rte use 0.0.0.0 mask */
        value.u32 = IP4_ADDR_ANY4.addr;
      } else {
        /* other rtes use netmask */
        value.u32 = netif_ip4_netmask(netif).addr;
      }
      break;
    12 => /* ipRouteMetric5 */
      value.s32 = -1; /* none */
      break;
    13 => /* ipRouteInfo */
      value.const_ptr = snmp_zero_dot_zero.id;
      *value_len = snmp_zero_dot_zero.len * sizeof;
      break;
    _ =>
      return SNMP_ERR_NOSUCHINSTANCE;
  }

  return SNMP_ERR_NOERROR;
}

pub fn ip_RouteTable_get_cell_value(const u32 *column,  u32 *row_oid, row_oid_len: u8, union snmp_variant_value *value, u32 *value_len)
{
  let mut if_addr: LwipAddr;
  netif: &mut NetIfc;

  /* check if incoming OID length and if values are in plausible range */
  if (!snmp_oid_in_range(row_oid, row_oid_len, ip_RouteTable_oid_ranges, LWIP_ARRAYSIZE(ip_RouteTable_oid_ranges))) {
    return SNMP_ERR_NOSUCHINSTANCE;
  }

  /* get IP and port from incoming OID */
  snmp_oid_to_ip4(&row_oid[0], &test_ip); /* we know it succeeds because of oid_in_range check above */

  /* default route is on default netif */
  if (ip4_addr_isany_val(test_ip) && (netif_default != None)) {
    /* fill in object properties */
    return ip_RouteTable_get_cell_value_core(netif_default, 1, column, value, value_len);
  }

  /* find netif with requested route */
  NETIF_FOREACH(netif) {
    let mut if_addr: LwipAddr;
    ip4_addr_get_network(&dst, netif_ip4_addr(netif), netif_ip4_netmask(netif));

    if (ip4_addr_cmp(&dst, &test_ip)) {
      /* fill in object properties */
      return ip_RouteTable_get_cell_value_core(netif, 0, column, value, value_len);
    }
  }

  /* not found */
  return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn ip_RouteTable_get_next_cell_instance_and_value(const u32 *column, row_oid: &mut snmp_obj_id, union snmp_variant_value *value, u32 *value_len)
{
  netif: &mut NetIfc;
  let state: snmp_next_oid_state;
  result_temp: u32[LWIP_ARRAYSIZE(ip_RouteTable_oid_ranges)];
  test_oid: u32[LWIP_ARRAYSIZE(ip_RouteTable_oid_ranges)];

  /* init struct to search next oid */
  snmp_next_oid_init(&state, row_oid.id, row_oid.len, result_temp, LWIP_ARRAYSIZE(ip_RouteTable_oid_ranges));

  /* check default route */
  if (netif_default != None) {
    snmp_ip4_to_oid(IP4_ADDR_ANY4, &test_oid[0]);
    snmp_next_oid_check(&state, test_oid, LWIP_ARRAYSIZE(ip_RouteTable_oid_ranges), netif_default);
  }

  /* iterate over all possible OIDs to find the next one */
  NETIF_FOREACH(netif) {
    let mut if_addr: LwipAddr;
    ip4_addr_get_network(&dst, netif_ip4_addr(netif), netif_ip4_netmask(netif));

    /* check generated OID: is it a candidate for the next one? */
    if (!ip4_addr_isany_val(dst)) {
      snmp_ip4_to_oid(&dst, &test_oid[0]);
      snmp_next_oid_check(&state, test_oid, LWIP_ARRAYSIZE(ip_RouteTable_oid_ranges), netif);
    }
  }

  /* did we find a next one? */
  if (state.status == SNMP_NEXT_OID_STATUS_SUCCESS) {
    let mut if_addr: LwipAddr;
    snmp_oid_to_ip4(&result_temp[0], &dst);
    snmp_oid_assign(row_oid, state.next_oid, state.next_oid_len);
    /* fill in object properties */
    return ip_RouteTable_get_cell_value_core((NetIfc *)state.reference, ip4_addr_isany_val(dst), column, value, value_len);
  } else {
    /* not found */
    return SNMP_ERR_NOSUCHINSTANCE;
  }
}


/* --- ipNetToMediaTable --- */

/* list of allowed value ranges for incoming OID */
static const struct snmp_oid_range ip_NetToMediaTable_oid_ranges[] = {
  { 1, 0xff }, /* IfIndex */
  { 0, 0xff }, /* IP A    */
  { 0, 0xff }, /* IP B    */
  { 0, 0xff }, /* IP C    */
  { 0, 0xff }  /* IP D    */
};

pub fn ip_NetToMediaTable_get_cell_value_core(arp_table_index: usize,  u32 *column, union snmp_variant_value *value, u32 *value_len)
{
  ip: &mut ip4_addr;
  netif: &mut NetIfc;
  ethaddr: &mut eth_addr;

  etharp_get_entry(arp_table_index, &ip, &netif, &ethaddr);

  /* value */
  match (*column) {
    1 => /* atIfIndex / ipNetToMediaIfIndex */
      value.u32 = netif_to_num(netif);
      break;
    2 => /* atPhysAddress / ipNetToMediaPhysAddress */
      value.ptr = ethaddr;
      *value_len = sizeof(*ethaddr);
      break;
    3 => /* atNetAddress / ipNetToMediaNetAddress */
      value.u32 = ip.addr;
      break;
    4 => /* ipNetToMediaType */
      value.u32 = 3; /* dynamic*/
      break;
    _ =>
      return SNMP_ERR_NOSUCHINSTANCE;
  }

  return SNMP_ERR_NOERROR;
}

pub fn ip_NetToMediaTable_get_cell_value(const u32 *column,  u32 *row_oid, row_oid_len: u8, union snmp_variant_value *value, u32 *value_len)
{
  let mut if_addr: LwipAddr;
  let netif_index: u8;
  let i: usize;

  /* check if incoming OID length and if values are in plausible range */
  if (!snmp_oid_in_range(row_oid, row_oid_len, ip_NetToMediaTable_oid_ranges, LWIP_ARRAYSIZE(ip_NetToMediaTable_oid_ranges))) {
    return SNMP_ERR_NOSUCHINSTANCE;
  }

  /* get IP from incoming OID */
  netif_index = row_oid[0];
  snmp_oid_to_ip4(&row_oid[1], &ip_in); /* we know it succeeds because of oid_in_range check above */

  /* find requested entry */
  for (i = 0; i < ARP_TABLE_SIZE; i+= 1) {
    ip: &mut ip4_addr;
    netif: &mut NetIfc;
    ethaddr: &mut eth_addr;

    if (etharp_get_entry(i, &ip, &netif, &ethaddr)) {
      if ((netif_index == netif_to_num(netif)) && ip4_addr_cmp(&ip_in, ip)) {
        /* fill in object properties */
        return ip_NetToMediaTable_get_cell_value_core(i, column, value, value_len);
      }
    }
  }

  /* not found */
  return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn ip_NetToMediaTable_get_next_cell_instance_and_value(const u32 *column, row_oid: &mut snmp_obj_id, union snmp_variant_value *value, u32 *value_len)
{
  let i: usize;
  let state: snmp_next_oid_state;
  result_temp: u32[LWIP_ARRAYSIZE(ip_NetToMediaTable_oid_ranges)];

  /* init struct to search next oid */
  snmp_next_oid_init(&state, row_oid.id, row_oid.len, result_temp, LWIP_ARRAYSIZE(ip_NetToMediaTable_oid_ranges));

  /* iterate over all possible OIDs to find the next one */
  for (i = 0; i < ARP_TABLE_SIZE; i+= 1) {
    ip: &mut ip4_addr;
    netif: &mut NetIfc;
    ethaddr: &mut eth_addr;

    if (etharp_get_entry(i, &ip, &netif, &ethaddr)) {
      test_oid: u32[LWIP_ARRAYSIZE(ip_NetToMediaTable_oid_ranges)];

      test_oid[0] = netif_to_num(netif);
      snmp_ip4_to_oid(ip, &test_oid[1]);

      /* check generated OID: is it a candidate for the next one? */
      snmp_next_oid_check(&state, test_oid, LWIP_ARRAYSIZE(ip_NetToMediaTable_oid_ranges), LWIP_PTR_NUMERIC_CAST(void *, i));
    }
  }

  /* did we find a next one? */
  if (state.status == SNMP_NEXT_OID_STATUS_SUCCESS) {
    snmp_oid_assign(row_oid, state.next_oid, state.next_oid_len);
    /* fill in object properties */
    return ip_NetToMediaTable_get_cell_value_core(LWIP_PTR_NUMERIC_CAST(usize, state.reference), column, value, value_len);
  }

  /* not found */
  return SNMP_ERR_NOSUCHINSTANCE;
}



static const struct snmp_scalar_node ip_Forwarding      = SNMP_SCALAR_CREATE_NODE(1, SNMP_NODE_INSTANCE_READ_WRITE, SNMP_ASN1_TYPE_INTEGER, ip_get_value, ip_set_test, ip_set_value);
static const struct snmp_scalar_node ip_DefaultTTL      = SNMP_SCALAR_CREATE_NODE(2, SNMP_NODE_INSTANCE_READ_WRITE, SNMP_ASN1_TYPE_INTEGER, ip_get_value, ip_set_test, ip_set_value);
static const struct snmp_scalar_node ip_InReceives      = SNMP_SCALAR_CREATE_NODE_READONLY(3, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_InHdrErrors     = SNMP_SCALAR_CREATE_NODE_READONLY(4, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_InAddrErrors    = SNMP_SCALAR_CREATE_NODE_READONLY(5, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_ForwDatagrams   = SNMP_SCALAR_CREATE_NODE_READONLY(6, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_InUnknownProtos = SNMP_SCALAR_CREATE_NODE_READONLY(7, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_InDiscards      = SNMP_SCALAR_CREATE_NODE_READONLY(8, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_InDelivers      = SNMP_SCALAR_CREATE_NODE_READONLY(9, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_OutRequests     = SNMP_SCALAR_CREATE_NODE_READONLY(10, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_OutDiscards     = SNMP_SCALAR_CREATE_NODE_READONLY(11, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_OutNoRoutes     = SNMP_SCALAR_CREATE_NODE_READONLY(12, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_ReasmTimeout    = SNMP_SCALAR_CREATE_NODE_READONLY(13, SNMP_ASN1_TYPE_INTEGER, ip_get_value);
static const struct snmp_scalar_node ip_ReasmReqds      = SNMP_SCALAR_CREATE_NODE_READONLY(14, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_ReasmOKs        = SNMP_SCALAR_CREATE_NODE_READONLY(15, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_ReasmFails      = SNMP_SCALAR_CREATE_NODE_READONLY(16, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_FragOKs         = SNMP_SCALAR_CREATE_NODE_READONLY(17, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_FragFails       = SNMP_SCALAR_CREATE_NODE_READONLY(18, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_FragCreates     = SNMP_SCALAR_CREATE_NODE_READONLY(19, SNMP_ASN1_TYPE_COUNTER, ip_get_value);
static const struct snmp_scalar_node ip_RoutingDiscards = SNMP_SCALAR_CREATE_NODE_READONLY(23, SNMP_ASN1_TYPE_COUNTER, ip_get_value);

static const struct snmp_table_simple_col_def ip_AddrTable_columns[] = {
  { 1, SNMP_ASN1_TYPE_IPADDR,  SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipAdEntAddr */
  { 2, SNMP_ASN1_TYPE_INTEGER, SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipAdEntIfIndex */
  { 3, SNMP_ASN1_TYPE_IPADDR,  SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipAdEntNetMask */
  { 4, SNMP_ASN1_TYPE_INTEGER, SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipAdEntBcastAddr */
  { 5, SNMP_ASN1_TYPE_INTEGER, SNMP_VARIANT_VALUE_TYPE_U32 }  /* ipAdEntReasmMaxSize */
};

static const struct snmp_table_simple_node ip_AddrTable = SNMP_TABLE_CREATE_SIMPLE(20, ip_AddrTable_columns, ip_AddrTable_get_cell_value, ip_AddrTable_get_next_cell_instance_and_value);

static const struct snmp_table_simple_col_def ip_RouteTable_columns[] = {
  {  1, SNMP_ASN1_TYPE_IPADDR,    SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteDest */
  {  2, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteIfIndex */
  {  3, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_S32 }, /* ipRouteMetric1 */
  {  4, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_S32 }, /* ipRouteMetric2 */
  {  5, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_S32 }, /* ipRouteMetric3 */
  {  6, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_S32 }, /* ipRouteMetric4 */
  {  7, SNMP_ASN1_TYPE_IPADDR,    SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteNextHop */
  {  8, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteType */
  {  9, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteProto */
  { 10, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteAge */
  { 11, SNMP_ASN1_TYPE_IPADDR,    SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipRouteMask */
  { 12, SNMP_ASN1_TYPE_INTEGER,   SNMP_VARIANT_VALUE_TYPE_S32 }, /* ipRouteMetric5 */
  { 13, SNMP_ASN1_TYPE_OBJECT_ID, SNMP_VARIANT_VALUE_TYPE_PTR }  /* ipRouteInfo */
};

static const struct snmp_table_simple_node ip_RouteTable = SNMP_TABLE_CREATE_SIMPLE(21, ip_RouteTable_columns, ip_RouteTable_get_cell_value, ip_RouteTable_get_next_cell_instance_and_value);



static const struct snmp_table_simple_col_def ip_NetToMediaTable_columns[] = {
  {  1, SNMP_ASN1_TYPE_INTEGER,      SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipNetToMediaIfIndex */
  {  2, SNMP_ASN1_TYPE_OCTET_STRING, SNMP_VARIANT_VALUE_TYPE_PTR }, /* ipNetToMediaPhysAddress */
  {  3, SNMP_ASN1_TYPE_IPADDR,       SNMP_VARIANT_VALUE_TYPE_U32 }, /* ipNetToMediaNetAddress */
  {  4, SNMP_ASN1_TYPE_INTEGER,      SNMP_VARIANT_VALUE_TYPE_U32 }  /* ipNetToMediaType */
};

static const struct snmp_table_simple_node ip_NetToMediaTable = SNMP_TABLE_CREATE_SIMPLE(22, ip_NetToMediaTable_columns, ip_NetToMediaTable_get_cell_value, ip_NetToMediaTable_get_next_cell_instance_and_value);



/* the following nodes access variables in LWIP stack from SNMP worker thread and must therefore be synced to LWIP (TCPIP) thread */
CREATE_LWIP_SYNC_NODE( 1, ip_Forwarding)
CREATE_LWIP_SYNC_NODE( 2, ip_DefaultTTL)
CREATE_LWIP_SYNC_NODE( 3, ip_InReceives)
CREATE_LWIP_SYNC_NODE( 4, ip_InHdrErrors)
CREATE_LWIP_SYNC_NODE( 5, ip_InAddrErrors)
CREATE_LWIP_SYNC_NODE( 6, ip_ForwDatagrams)
CREATE_LWIP_SYNC_NODE( 7, ip_InUnknownProtos)
CREATE_LWIP_SYNC_NODE( 8, ip_InDiscards)
CREATE_LWIP_SYNC_NODE( 9, ip_InDelivers)
CREATE_LWIP_SYNC_NODE(10, ip_OutRequests)
CREATE_LWIP_SYNC_NODE(11, ip_OutDiscards)
CREATE_LWIP_SYNC_NODE(12, ip_OutNoRoutes)
CREATE_LWIP_SYNC_NODE(13, ip_ReasmTimeout)
CREATE_LWIP_SYNC_NODE(14, ip_ReasmReqds)
CREATE_LWIP_SYNC_NODE(15, ip_ReasmOKs)
CREATE_LWIP_SYNC_NODE(15, ip_ReasmFails)
CREATE_LWIP_SYNC_NODE(17, ip_FragOKs)
CREATE_LWIP_SYNC_NODE(18, ip_FragFails)
CREATE_LWIP_SYNC_NODE(19, ip_FragCreates)
CREATE_LWIP_SYNC_NODE(20, ip_AddrTable)
CREATE_LWIP_SYNC_NODE(21, ip_RouteTable)

CREATE_LWIP_SYNC_NODE(22, ip_NetToMediaTable)

CREATE_LWIP_SYNC_NODE(23, ip_RoutingDiscards)

static const const: &mut snmp_node ip_nodes[] = {
  &SYNC_NODE_NAMEip_Forwarding.node.node,
  &SYNC_NODE_NAMEip_DefaultTTL.node.node,
  &SYNC_NODE_NAMEip_InReceives.node.node,
  &SYNC_NODE_NAMEip_InHdrErrors.node.node,
  &SYNC_NODE_NAMEip_InAddrErrors.node.node,
  &SYNC_NODE_NAMEip_ForwDatagrams.node.node,
  &SYNC_NODE_NAMEip_InUnknownProtos.node.node,
  &SYNC_NODE_NAMEip_InDiscards.node.node,
  &SYNC_NODE_NAMEip_InDelivers.node.node,
  &SYNC_NODE_NAMEip_OutRequests.node.node,
  &SYNC_NODE_NAMEip_OutDiscards.node.node,
  &SYNC_NODE_NAMEip_OutNoRoutes.node.node,
  &SYNC_NODE_NAMEip_ReasmTimeout.node.node,
  &SYNC_NODE_NAMEip_ReasmReqds.node.node,
  &SYNC_NODE_NAMEip_ReasmOKs.node.node,
  &SYNC_NODE_NAMEip_ReasmFails.node.node,
  &SYNC_NODE_NAMEip_FragOKs.node.node,
  &SYNC_NODE_NAMEip_FragFails.node.node,
  &SYNC_NODE_NAMEip_FragCreates.node.node,
  &SYNC_NODE_NAMEip_AddrTable.node.node,
  &SYNC_NODE_NAMEip_RouteTable.node.node,

  &SYNC_NODE_NAMEip_NetToMediaTable.node.node,

  &SYNC_NODE_NAMEip_RoutingDiscards.node.node
};

const struct snmp_tree_node snmp_mib2_ip_root = SNMP_CREATE_TREE_NODE(4, ip_nodes);


/* --- at .1.3.6.1.2.1.3 ----------------------------------------------------- */


/* at node table is a subset of ip_nettomedia table (same rows but less columns) */
static const struct snmp_table_simple_col_def at_Table_columns[] = {
  { 1, SNMP_ASN1_TYPE_INTEGER,      SNMP_VARIANT_VALUE_TYPE_U32 }, /* atIfIndex */
  { 2, SNMP_ASN1_TYPE_OCTET_STRING, SNMP_VARIANT_VALUE_TYPE_PTR }, /* atPhysAddress */
  { 3, SNMP_ASN1_TYPE_IPADDR,       SNMP_VARIANT_VALUE_TYPE_U32 }  /* atNetAddress */
};

static const struct snmp_table_simple_node at_Table = SNMP_TABLE_CREATE_SIMPLE(1, at_Table_columns, ip_NetToMediaTable_get_cell_value, ip_NetToMediaTable_get_next_cell_instance_and_value);

/* the following nodes access variables in LWIP stack from SNMP worker thread and must therefore be synced to LWIP (TCPIP) thread */
CREATE_LWIP_SYNC_NODE(1, at_Table)

static const const: &mut snmp_node at_nodes[] = {
  &SYNC_NODE_NAMEat_Table.node.node
};

const struct snmp_tree_node snmp_mib2_at_root = SNMP_CREATE_TREE_NODE(3, at_nodes);



