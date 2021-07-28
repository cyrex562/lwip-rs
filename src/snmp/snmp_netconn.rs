/*
 * @file
 * SNMP netconn frontend.
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 */













/* SNMP netconn API worker thread */
pub fn
snmp_netconn_thread(arg: &mut Vec<u8>)
{
  conn: &mut netconn;
  buf: &mut netbuf;
  let err: err_t;
  LWIP_UNUSED_ARG(arg);

  /* Bind to SNMP port with default IP address */

  conn = netconn_new(NETCONN_UDP_IPV6);
  netconn_bind(conn, IP6_ADDR_ANY, LWIP_IANA_PORT_SNMP);
#else /* LWIP_IPV6 */
  conn = netconn_new(NETCONN_UDP);
  netconn_bind(conn, IP4_ADDR_ANY, LWIP_IANA_PORT_SNMP);

  LWIP_ERROR("snmp_netconn: invalid conn", (conn != NULL), return;);

  snmp_traps_handle = conn;

  do {
    err = netconn_recv(conn, &buf);

    if (err == ERR_OK) {
      snmp_receive(conn, buf.p, &buf.addr, buf.port);
    }

    if (buf != NULL) {
      netbuf_delete(buf);
    }
  } while (1);
}

pub fn 
snmp_sendto(void *handle, p: &mut pbuf, const dst: &mut ip_addr_t, port: u16)
{
  result: err_t;
  struct netbuf buf;

  memset(&buf, 0, sizeof(buf));
  buf.p = p;
  result = netconn_sendto((struct netconn *)handle, &buf, dst, port);

  return result;
}

u8
snmp_get_local_ip_for_dst(void *handle, const dst: &mut ip_addr_t, result: &mut ip_addr_t)
{
  conn: &mut netconn = (struct netconn *)handle;
  dst_if: &mut netif;
  const dst_ip: &mut ip_addr_t;

  LWIP_UNUSED_ARG(conn); /* unused in case of IPV4 only configuration */

  ip_route_get_local_ip(&conn.pcb.udp.local_ip, dst, dst_if, dst_ip);

  if ((dst_if != NULL) && (dst_ip != NULL)) {
    ip_addr_copy(*result, *dst_ip);
    return 1;
  } else {
    return 0;
  }
}

/*
 * Starts SNMP Agent.
 */
pub fn 
snmp_init(void)
{
  LWIP_ASSERT_CORE_LOCKED();
  sys_thread_new("snmp_netconn", snmp_netconn_thread, NULL, SNMP_STACK_SIZE, SNMP_THREAD_PRIO);
}


