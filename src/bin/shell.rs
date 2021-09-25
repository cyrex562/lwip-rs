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






















// #define NEWLINE "\r\n"
 /* WIN32 */
// #define NEWLINE "\n"


/* Define this to 1 if you want to echo back all received characters
 * (e.g. so they are displayed on a remote telnet)
 */

pub const SHELL_ECHO: u32 = 0;


pub const BUFSIZE: usize =             1024;
// static  buffer: [u8;BUFSIZE];

struct command {
  pub conn: &mut netconn,
  pub nargs: u8,
  pub args: String,
}





pub const ESUCCESS: u32 = 0;
pub const ESYNTAX: i32 = -1;
pub const ETOOFEW: i32 =  -2;
pub const ETOOMANY: i32 = -3;
pub const ECLOSED: i32 = -4;

pub const NCONNS: u32 = 10;
// static conns: &mut netconn[NCONNS];

/* help_msg is split into 3 strings to prevent exceeding the C89 maximum length of 509 per string */
pub const help_msg1: String = r#"Available commands:
open [IP address] [TCP port]: opens a TCP connection to the specified address.
lstn [TCP port]: sets up a server on the specified port.
acpt [connection #]: waits for an incoming connection request
send [connection #] [message]: sends a message on a TCP connection.
udpc [local UDP port] [IP address] [remote port]: opens a UDP "connection".
udpl [local UDP port] [IP address] [remote port]: opens a UDP-Lite "connection"."#.to_string();
pub const help_msg2: String = r#"udpn [local UDP port] [IP address] [remote port]: opens a UDP "connection" without checksums.
udpb [local port] [remote port]: opens a UDP broadcast "connection".
usnd [connection #] [message]: sends a message on a UDP connection.
recv [connection #]: recieves data on a TCP or UDP connection.
clos [connection #]: closes a TCP or UDP connection.
stat: prints out lwIP statistics.
idxtoname [index]: outputs interface name from index.
nametoidx [name]: outputs interface index from name."#;
pub const help_msg3: String = r#"
"gethostnm [name]: outputs IP address of host.
quit: quits"#.to_string();


pub const padding_10spaces: String = r#"          "#;

// #define PROTOCOL_STATS (LINK_STATS && ETHARP_STATS && IPFRAG_STATS && IP_STATS && ICMP_STATS && UDP_STATS && TCP_STATS)


pub const shell_stat_proto_names: [String;8] = [

  "LINK      ".to_string(),


  "ETHARP    ".to_string(),


  "IP_FRAG   ".to_string(),


  "IP        ".to_string(),


  "ICMP      ".to_string(),


  "UDP       ".to_string(),


  "TCP       ".to_string(),

  "last".to_string()
];

// pub const  shell_stat_proto_stats: [stats_proto;7] = {

//   &lwip_stats.link,


//   &lwip_stats.etharp,


//   &lwip_stats.ip_frag,


//   &lwip_stats.ip,


//   &lwip_stats.icmp,


//   &lwip_stats.udp,


//   &lwip_stats.tcp,

// };
pub const num_protostats: usize = sizeof(shell_stat_proto_stats)/sizeof(stats_proto);

pub const stat_msgs_proto: [String;12] = [
  " * transmitted ".to_string(),
  "           * received ".to_string(),
  "             forwarded ".to_string(),
  "           * dropped ".to_string(),
  "           * checksum errors ".to_string(),
  "           * length errors ".to_string(),
  "           * memory errors ".to_string(),
  "             routing errors ".to_string(),
  "             protocol errors ".to_string(),
  "             option errors ".to_string(),
  "           * misc errors ".to_string(),
  "             cache hits ".to_string()
];



/*-----------------------------------------------------------------------------------*/
pub fn
sendstr(str: &String, conn: &mut netconn)
{
  netconn_write(conn, str, strlen(str), NETCONN_NOCOPY);
}
/*-----------------------------------------------------------------------------------*/
pub fn com_open(com: &mut command)
{
  let ipaddr: LwipAddr;
  let port: u16;
  let i: i32;
  let err: err_t;
  let tmp: i32;

  if (ipaddr_aton(com.args[0], &ipaddr) == -1) {
    sendstr(strerror(errno), com.conn);
    return ESYNTAX;
  }
  tmp = strtol(com.args[1], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  port = tmp;

  /* Find the first unused connection in conns. */
  // for(i = 0; i < NCONNS && conns[i] != NULL; i+= 1);

  if (i == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Opening connection to ", com.conn);
  netconn_write(com.conn, com.args[0], strlen(com.args[0]), NETCONN_COPY);
  sendstr(":", com.conn);
  netconn_write(com.conn, com.args[1], strlen(com.args[1]), NETCONN_COPY);
  sendstr(NEWLINE, com.conn);

  conns[i] = netconn_new(NETCONN_TCP);
  if (conns[i] == None) {
    sendstr("Could not create connection identifier (out of memory)."NEWLINE, com.conn);
    return ESUCCESS;
  }
  err = netconn_connect(conns[i], &ipaddr, port);
  if (err != ERR_OK) {
    fprintf(stderr, "error %s"NEWLINE, lwip_strerr(err));
    sendstr("Could not connect to remote host: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    netconn_delete(conns[i]);
    conns[i] = None;
    return ESUCCESS;
  }

  sendstr("Opened connection, connection identifier is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, i);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_lstn(com: &mut command)
{
  let port: u16;
  let i: i32;
  let err: err_t;
  let tmp: i32;

  tmp = strtol(com.args[0], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  port = tmp;

  /* Find the first unused connection in conns. */
  // for(i = 0; i < NCONNS && conns[i] != NULL; i+= 1);

  if (i == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Opening a listening connection on port ", com.conn);
  netconn_write(com.conn, com.args[0], strlen(com.args[0]), NETCONN_COPY);
  sendstr(NEWLINE, com.conn);

  conns[i] = netconn_new(NETCONN_TCP);
  if (conns[i] == None) {
    sendstr("Could not create connection identifier (out of memory)."NEWLINE, com.conn);
    return ESUCCESS;
  }
  
  err = netconn_bind(conns[i], IP_ADDR_ANY, port);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not bind: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }
  
  err = netconn_listen(conns[i]);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not listen: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Opened connection, connection identifier is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, i);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
/*-----------------------------------------------------------------------------------*/
pub fn com_clos(com: &mut command)
{
  let i: i32;
  let err: err_t;
  
  i = strtol(com.args[0], None, 10);

  if (i > NCONNS) {
    sendstr("Connection identifier too high."NEWLINE, com.conn);
    return ESUCCESS;
  }
  if (conns[i] == None) {
    sendstr("Connection identifier not in use."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_close(conns[i]);
  if (err != ERR_OK) {
    sendstr("Could not close connection: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Connection closed."NEWLINE, com.conn);
  netconn_delete(conns[i]);
  conns[i] = None;
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_acpt(com: &mut command)
{
  let i: i32;
  let j;
  let err: err_t;

  /* Find the first unused connection in conns. */
  // for(j = 0; j < NCONNS && conns[j] != NULL; j+= 1);

  if (j == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  i = strtol(com.args[0], None, 10);

  if (i > NCONNS) {
    sendstr("Connection identifier too high."NEWLINE, com.conn);
    return ESUCCESS;
  }
  if (conns[i] == None) {
    sendstr("Connection identifier not in use."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_accept(conns[i], &conns[j]);
  
  if (err != ERR_OK) {
    sendstr("Could not accept connection: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Accepted connection, connection identifier for new connection is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, j);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);

  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/

pub fn
com_stat_write_mem(conn: &mut netconn, elem: &mut stats_mem, i: i32)
{
  let len: usize;
  let buf: String;
  let slen: usize;


  
  slen = strlen(elem.name);
  netconn_write(conn, elem.name, slen, NETCONN_COPY);
 /*  LWIP_DEBUG */
  len = sprintf(buf, "%d", i);
  slen = strlen(buf);
  netconn_write(conn, buf, slen, NETCONN_COPY);

  if(slen < 10) {
    netconn_write(conn, padding_10spaces, 10-slen, NETCONN_COPY);
  }

  // len = sprintf(buf, " * available %"MEM_SIZE_F NEWLINE, elem.avail);
  netconn_write(conn, buf, len, NETCONN_COPY);
  // len = sprintf(buf, "           * used %"MEM_SIZE_F NEWLINE, elem.used);
  netconn_write(conn, buf, len, NETCONN_COPY);
  // len = sprintf(buf, "           * high water mark %"MEM_SIZE_F NEWLINE, elem.max);
  netconn_write(conn, buf, len, NETCONN_COPY);
  // len = sprintf(buf, "           * errors %"STAT_COUNTER_F NEWLINE, elem.err);
  netconn_write(conn, buf, len, NETCONN_COPY);
  // len = sprintf(buf, "           * illegal %"STAT_COUNTER_F NEWLINE, elem.illegal);
  netconn_write(conn, buf, len, NETCONN_COPY);
}
pub fn
com_stat_write_sys(conn: &mut netconn, elem: &mut stats_syselem, name: &String)
{
  let len: usize;
  let buf: String;
  let slen: usize = strlen(name);

  netconn_write(conn, name, slen, NETCONN_COPY);
  if(slen < 10) {
    netconn_write(conn, padding_10spaces, 10-slen, NETCONN_COPY);
  }

  // len = sprintf(buf, " * used %"STAT_COUNTER_F NEWLINE, elem.used);
  netconn_write(conn, buf, len, NETCONN_COPY);
  // len = sprintf(buf, "           * high water mark %"STAT_COUNTER_F NEWLINE, elem.max);
  netconn_write(conn, buf, len, NETCONN_COPY);
  // len = sprintf(buf, "           * errors %"STAT_COUNTER_F NEWLINE, elem.err);
  netconn_write(conn, buf, len, NETCONN_COPY);
}
pub fn com_stat(com: &mut command)
{

  let i: usize;


  let k: usize;
  let buf: String;
  let len: usize;

  /* protocol stats, @todo: add IGMP */
  // for(i = 0; i < num_protostats; i+= 1) {
  //   s: usize = sizeof(stats_proto)/sizeof(STAT_COUNTER);
  //   STAT_COUNTER *c = &shell_stat_proto_stats[i].xmit;
  //   LWIP_ASSERT("stats not in sync", s == sizeof(stat_msgs_proto)/sizeof);
  //   netconn_write(com.conn, shell_stat_proto_names[i], strlen(shell_stat_proto_names[i]), NETCONN_COPY);
  //   for(k = 0; k < s; k+= 1) {
  //     len = sprintf(buf, "%s%"STAT_COUNTER_F NEWLINE, stat_msgs_proto[k], c[k]);
  //     netconn_write(com.conn, buf, len, NETCONN_COPY);
  //   }
  // }


  com_stat_write_mem(com.conn, &lwip_stats.mem, -1);


  // for(i = 0; i < MEMP_MAX; i+= 1) {
  //   com_stat_write_mem(com.conn, lwip_stats.memp[i], -1);
  // }


  com_stat_write_sys(com.conn, &lwip_stats.sys.sem,   "SEM       ");
  com_stat_write_sys(com.conn, &lwip_stats.sys.mutex, "MUTEX     ");
  com_stat_write_sys(com.conn, &lwip_stats.sys.mbox,  "MBOX      ");


  return ESUCCESS;
}

/*-----------------------------------------------------------------------------------*/
pub fn com_send(com: &mut command)
{
  let i: i32;
  let err: err_t;
  let len: usize;
  
  i = strtol(com.args[0], None, 10);

  if (i > NCONNS) {
    sendstr("Connection identifier too high."NEWLINE, com.conn);
    return ESUCCESS;
  }

  if (conns[i] == None) {
    sendstr("Connection identifier not in use."NEWLINE, com.conn);
    return ESUCCESS;
  }

  len = strlen(com.args[1]);
  com.args[1][len] = '\r';
  com.args[1][len + 1] = '\n';
  com.args[1][len + 2] = 0;
  
  err = netconn_write(conns[i], com.args[1], len + 3, NETCONN_COPY);
  if (err != ERR_OK) {
    sendstr("Could not send data: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }
  
  sendstr("Data enqueued for sending."NEWLINE, com.conn);
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_recv(com: &mut command)
{
  let i: i32;
  let err: err_t;
  let buf: &mut netbuf;
  let len: usize;
  
  i = strtol(com.args[0], None, 10);

  if (i > NCONNS) {
    sendstr("Connection identifier too high."NEWLINE, com.conn);
    return ESUCCESS;
  }

  if (conns[i] == None) {
    sendstr("Connection identifier not in use."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_recv(conns[i], &buf);
  if (err == ERR_OK) {
      
    netbuf_copy(buf, buffer, BUFSIZE);
    len = netbuf_len(buf);
    sendstr("Reading from connection:"NEWLINE, com.conn);
    netconn_write(com.conn, buffer, len, NETCONN_COPY);
    netbuf_delete(buf);
  } else {
    sendstr("EOF."NEWLINE, com.conn);
  }
  err = netconn_err(conns[i]);
  if (err != ERR_OK) {
    sendstr("Could not receive data: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_udpc(com: &mut command)
{
  let ipaddr: LwipAddr;
  let lport: u16;
  let rport;
  let i: i32;
  let err: err_t;
  let tmp: i32;

  tmp = strtol(com.args[0], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  lport = tmp;
  if (ipaddr_aton(com.args[1], &ipaddr) == -1) {
    sendstr(strerror(errno), com.conn);
    return ESYNTAX;
  }
  tmp = strtol(com.args[2], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  rport = tmp;

  /* Find the first unused connection in conns. */
  // for(i = 0; i < NCONNS && conns[i] != NULL; i+= 1);

  if (i == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Setting up UDP connection from port ", com.conn);
  netconn_write(com.conn, com.args[0], strlen(com.args[0]), NETCONN_COPY);
  sendstr(" to ", com.conn);
  netconn_write(com.conn, com.args[1], strlen(com.args[1]), NETCONN_COPY);
  sendstr(":", com.conn);
  netconn_write(com.conn, com.args[2], strlen(com.args[2]), NETCONN_COPY);
  sendstr(NEWLINE, com.conn);

  conns[i] = netconn_new(NETCONN_UDP);
  if (conns[i] == None) {
    sendstr("Could not create connection identifier (out of memory)."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_connect(conns[i], &ipaddr, rport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not connect to remote host: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_bind(conns[i], IP_ADDR_ANY, lport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not bind: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Connection set up, connection identifier is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, i);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_udpl(com: &mut command)
{
  let ipaddr: LwipAddr;
  let lport: u16;
  let rport;
  let i: i32;
  let err: err_t;
  let tmp: i32;

  tmp = strtol(com.args[0], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  lport = tmp;
  if (ipaddr_aton(com.args[1], &ipaddr) == -1) {
    sendstr(strerror(errno), com.conn);
    return ESYNTAX;
  }
  tmp = strtol(com.args[2], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  rport = tmp;

  /* Find the first unused connection in conns. */
  // for(i = 0; i < NCONNS && conns[i] != NULL; i+= 1);

  if (i == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Setting up UDP-Lite connection from port ", com.conn);
  netconn_write(com.conn, com.args[0], strlen(com.args[0]), NETCONN_COPY);
  sendstr(" to ", com.conn);
  netconn_write(com.conn, com.args[1], strlen(com.args[1]), NETCONN_COPY);
  sendstr(":", com.conn);
  netconn_write(com.conn, com.args[2], strlen(com.args[2]), NETCONN_COPY);
  sendstr(NEWLINE, com.conn);

  conns[i] = netconn_new(NETCONN_UDPLITE);
  if (conns[i] == None) {
    sendstr("Could not create connection identifier (out of memory)."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_connect(conns[i], &ipaddr, rport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not connect to remote host: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_bind(conns[i], IP_ADDR_ANY, lport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not bind: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Connection set up, connection identifier is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, i);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_udpn(com: &mut command)
{
  let ipaddr: LwipAddr;
  let lport: u16; 
  let rport;
  let i: i32;
  let err: err_t;
  let tmp: i32;

  tmp = strtol(com.args[0], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  lport = tmp;
  if (ipaddr_aton(com.args[1], &ipaddr) == -1) {
    sendstr(strerror(errno), com.conn);
    return ESYNTAX;
  }
  tmp = strtol(com.args[2], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  rport = tmp;

  /* Find the first unused connection in conns. */
  // for(i = 0; i < NCONNS && conns[i] != NULL; i+= 1);

  if (i == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Setting up UDP connection without checksums from port ", com.conn);
  netconn_write(com.conn, com.args[0], strlen(com.args[0]), NETCONN_COPY);
  sendstr(" to ", com.conn);
  netconn_write(com.conn, com.args[1], strlen(com.args[1]), NETCONN_COPY);
  sendstr(":", com.conn);
  netconn_write(com.conn, com.args[2], strlen(com.args[2]), NETCONN_COPY);
  sendstr(NEWLINE, com.conn);

  conns[i] = netconn_new(NETCONN_UDPNOCHKSUM);
  if (conns[i] == None) {
    sendstr("Could not create connection identifier (out of memory)."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_connect(conns[i], &ipaddr, rport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not connect to remote host: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_bind(conns[i], IP_ADDR_ANY, lport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not bind: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Connection set up, connection identifier is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, i);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_udpb(com: &mut command)
{
  let ipaddr: LwipAddr;

  let lport: u16;

  let rport: u16;
  let i: i32;
  let err: err_t;
  let tmp: i32;

  tmp = strtol(com.args[0], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }

  lport = tmp;

  if (ipaddr_aton(com.args[1], &ipaddr) == -1) {
    sendstr(strerror(errno), com.conn);
    return ESYNTAX;
  }
  tmp = strtol(com.args[2], None, 10);
  if((tmp < 0) || (tmp > 0xffff)) {
    sendstr("Invalid port number."NEWLINE, com.conn);
    return ESUCCESS;
  }
  rport = tmp;

  /* Find the first unused connection in conns. */
  // for(i = 0; i < NCONNS && conns[i] != NULL; i+= 1);

  if (i == NCONNS) {
    sendstr("No more connections available, sorry."NEWLINE, com.conn);
    return ESUCCESS;
  }

  sendstr("Setting up UDP broadcast connection from port ", com.conn);
  netconn_write(com.conn, com.args[0], strlen(com.args[0]), NETCONN_COPY);
  sendstr(" to ", com.conn);
  netconn_write(com.conn, com.args[1], strlen(com.args[1]), NETCONN_COPY);
  sendstr(NEWLINE, com.conn);

  conns[i] = netconn_new(NETCONN_UDP);
  if (conns[i] == None) {
    sendstr("Could not create connection identifier (out of memory)."NEWLINE, com.conn);
    return ESUCCESS;
  }

  err = netconn_connect(conns[i], &ipaddr, rport);
  if (err != ERR_OK) {
    netconn_delete(conns[i]);
    conns[i] = None;
    sendstr("Could not connect to remote host: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }


  if (IP_IS_V6_VAL(ipaddr)) {
    err = netconn_bind(conns[i], &ip_addr_broadcast, lport);
    if (err != ERR_OK) {
      netconn_delete(conns[i]);
      conns[i] = None;
      sendstr("Could not bind: ", com.conn);

      sendstr(lwip_strerr(err), com.conn);

      sendstr("(debugging must be turned on for error message to appear)", com.conn);

      sendstr(NEWLINE, com.conn);
      return ESUCCESS;
    }
  }


  sendstr("Connection set up, connection identifier is ", com.conn);
  snprintf(buffer, sizeof(buffer), "%d"NEWLINE, i);
  netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_usnd(com: &mut command)
{
  let i: i32;
  let err: err_t;
  let buf: &mut netbuf;
  let mem: &mut String;
  let len: usize;
  let tmp: usize;
  
  i = strtol(com.args[0], None, 10);

  if (i > NCONNS) {
    sendstr("Connection identifier too high."NEWLINE, com.conn);
    return ESUCCESS;
  }

  if (conns[i] == None) {
    sendstr("Connection identifier not in use."NEWLINE, com.conn);
    return ESUCCESS;
  }
  tmp = strlen(com.args[1]) + 1;
  if (tmp > 0xffff) {
    sendstr("Invalid length."NEWLINE, com.conn);
    return ESUCCESS;
  }
  len = tmp;

  buf = netbuf_new();
  mem = netbuf_alloc(buf, len);
  if (mem == None) {
    sendstr("Could not allocate memory for sending."NEWLINE, com.conn);
    return ESUCCESS;
  }
  strncpy(mem, com.args[1], len);
  err = netconn_send(conns[i], buf);
  netbuf_delete(buf);
  if (err != ERR_OK) {
    sendstr("Could not send data: ", com.conn);

    sendstr(lwip_strerr(err), com.conn);

    sendstr("(debugging must be turned on for error message to appear)", com.conn);

    sendstr(NEWLINE, com.conn);
    return ESUCCESS;
  }
  
  sendstr("Data sent."NEWLINE, com.conn);
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------------*/
pub fn com_idxtoname(com: &mut command)
{
  let i = strtol(com.args[0], None, 10);

  if (lwip_if_indextoname(i, buffer)) {
    netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
    sendstr(NEWLINE, com.conn);
  } else {
    snprintf(buffer, sizeof(buffer), "if_indextoname() failed: %d"NEWLINE, errno);
    netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  }
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn com_nametoidx(com: &mut command)
{
   let idx = lwip_if_nametoindex(com.args[0]);

  if (idx) {
    snprintf(buffer, sizeof(buffer), "%u"NEWLINE, idx);
    netconn_write(com.conn, buffer, strlen(buffer), NETCONN_COPY);
  } else {
    sendstr("No interface found"NEWLINE, com.conn);
  }
  return ESUCCESS;
}

/*-----------------------------------------------------------------------------------*/

pub fn com_gethostbyname(com: &mut command)
{
  let addr: LwipAddr;
  let err = netconn_gethostbyname(com.args[0], &addr);

  if (err == ERR_OK) {
    if (ipaddr_ntoa_r(&addr, buffer, sizeof(buffer))) {
      sendstr("Host found: ", com.conn);
      sendstr(buffer, com.conn);
      sendstr(NEWLINE, com.conn);
    } else {
        sendstr("ipaddr_ntoa_r failed", com.conn);
    }
  } else {
    sendstr("No host found"NEWLINE, com.conn);
  }
  return ESUCCESS;
}

/*-----------------------------------------------------------------------------------*/
pub fn com_help(com: &mut command)
{
  sendstr(&help_msg1, com.conn);
  sendstr(&help_msg2, com.conn);
  sendstr(&help_msg3, com.conn);
  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn parse_command(com: &mut command, len: u32)
{
  let i: u16;
  let bufp: u16;
  
  if (strncmp(buffer, "open", 4) == 0) {
    com.exec = com_open;
    com.nargs = 2;
  } else if (strncmp(buffer, "lstn", 4) == 0) {
    com.exec = com_lstn;
    com.nargs = 1;
  } else if (strncmp(buffer, "acpt", 4) == 0) {
    com.exec = com_acpt;
    com.nargs = 1;
  } else if (strncmp(buffer, "clos", 4) == 0) {
    com.exec = com_clos;
    com.nargs = 1;

  } else if (strncmp(buffer, "stat", 4) == 0) {
    com.exec = com_stat;
    com.nargs = 0;

  } else if (strncmp(buffer, "send", 4) == 0) {
    com.exec = com_send;
    com.nargs = 2;
  } else if (strncmp(buffer, "recv", 4) == 0) {
    com.exec = com_recv;
    com.nargs = 1;
  } else if (strncmp(buffer, "udpc", 4) == 0) {
    com.exec = com_udpc;
    com.nargs = 3;
  } else if (strncmp(buffer, "udpb", 4) == 0) {
    com.exec = com_udpb;
    com.nargs = 2;
  } else if (strncmp(buffer, "udpl", 4) == 0) {
    com.exec = com_udpl;
    com.nargs = 3;
  } else if (strncmp(buffer, "udpn", 4) == 0) {
    com.exec = com_udpn;
    com.nargs = 3;
  } else if (strncmp(buffer, "usnd", 4) == 0) {
    com.exec = com_usnd;
    com.nargs = 2;

  } else if (strncmp(buffer, "idxtoname", 9) == 0) {
    com.exec = com_idxtoname;
    com.nargs = 1;
  } else if (strncmp(buffer, "nametoidx", 9) == 0) {
    com.exec = com_nametoidx;
    com.nargs = 1;


  } else if (strncmp(buffer, "gethostnm", 9) == 0) {
    com.exec = com_gethostbyname;
    com.nargs = 1;

  } else if (strncmp(buffer, "help", 4) == 0) {
    com.exec = com_help;
    com.nargs = 0;
  } else if (strncmp(buffer, "quit", 4) == 0) {
    printf("quit"NEWLINE);
    return ECLOSED;
  } else {
    return ESYNTAX;
  }

  if (com.nargs == 0) {
    return ESUCCESS;
  }
  bufp = 0;
  // for(; bufp < len && buffer[bufp] != ' '; bufp+= 1);
  // for(i = 0; i < 10; i+= 1) {
  //   for(; bufp < len && buffer[bufp] == ' '; bufp+= 1);
  //   if (buffer[bufp] == '\r' ||
  //      buffer[bufp] == '\n') {
  //     buffer[bufp] = 0;
  //     if (i < com.nargs - 1) {
  //       return ETOOFEW;
  //     }
  //     if (i > com.nargs - 1) {
  //       return ETOOMANY;
  //     }
  //     break;
  //   }    
  //   if (bufp > len) {
  //     return ETOOFEW;
  //   }    
  //   com.args[i] = &buffer[bufp];
  //   for(; bufp < len && buffer[bufp] != ' ' && buffer[bufp] != '\r' &&
  //     buffer[bufp] != '\n'; bufp+= 1) {
  //     if (buffer[bufp] == '\\') {
  //       buffer[bufp] = ' ';
  //     }
  //   }
  //   if (bufp > len) {
  //     return ESYNTAX;
  //   }
  //   buffer[bufp] = 0;
  //   bufp+= 1;
  //   if (i == com.nargs - 1) {
  //     break;
  //   }

  // }

  return ESUCCESS;
}
/*-----------------------------------------------------------------------------------*/
pub fn shell_error(err: i8, conn: &mut netconn)
{
  match (err) {
  ESYNTAX =>     sendstr("## Syntax error"NEWLINE, conn),
  ETOOFEW =>     sendstr("## Too few arguments to command given"NEWLINE, conn),
  ETOOMANY =>     sendstr("## Too many arguments to command given"NEWLINE, conn),
  ECLOSED =>     sendstr("## Connection closed"NEWLINE, conn),
  _ => {}
    /* unknown error, don't assert here */
    
  }
}
/*-----------------------------------------------------------------------------------*/
pub fn
prompt(conn: &mut netconn)
{
  sendstr("> ", conn);
}  
/*-----------------------------------------------------------------------------------*/
pub fn
shell_main(conn: &mut netconn)
{
  let p: &mut PacketBuffer;
  let len: usize = 0;
  let  cur_len;
  let com: command;
  let err: i8;
  let i: i32;
  let ret: err_t;
  let echomem: &mut Vec<u8>;


  loop {
    ret = netconn_recv_tcp_pbuf(conn, &p);
    if (ret == ERR_OK) {
      pbuf_copy_partial(p, &buffer[len], (BUFSIZE - len), 0);
      cur_len = p.tot_len;
      len = (len + cur_len);
      if ((len < cur_len) || (len > BUFSIZE)) {
        len = BUFSIZE;
      }

      echomem = mem_malloc(cur_len);
      if (echomem != None) {
        pbuf_copy_partial(p, echomem, cur_len, 0);
        netconn_write(conn, echomem, cur_len, NETCONN_COPY);
        mem_free(echomem);
      }

      pbuf_free(p);
      if (((len > 0) && ((buffer[len-1] == '\r') || (buffer[len-1] == '\n'))) ||
          (len >= BUFSIZE)) {
        if (buffer[0] != 0xff && 
           buffer[1] != 0xfe) {
          err = parse_command(&com, len);
          if (err == ESUCCESS) {
            com.conn = conn;
            err = com.exec(&com);
          }
          if (err == ECLOSED) {
            printf("Closed"NEWLINE);
            shell_error(err, conn);
            // goto close;
          }
          if (err != ESUCCESS) {
            shell_error(err, conn);
          }
        } else {
          // sendstr(NEWLINE NEWLINE
          //         "lwIP simple interactive shell."NEWLINE
          //         "(c) Copyright 2001, Swedish Institute of Computer Science."NEWLINE
          //         "Written by Adam Dunkels."NEWLINE
          //         "For help, try the \"help\" command."NEWLINE, conn);
        }
        if (ret == ERR_OK) {
          prompt(conn);
        }
        len = 0;
      }
    }
    if ret != ERR_OK {
      break;
    }
  } 
  printf("err %s"NEWLINE, lwip_strerr(ret));

// close:
  netconn_close(conn);

  // for(i = 0; i < NCONNS; i+= 1) {
  //   if (conns[i] != NULL) {
  //     netconn_delete(conns[i]);
  //   }
  //   conns[i] = NULL;
  // }
}
/*-----------------------------------------------------------------------------------*/
pub fn
shell_thread(arg: &mut Vec<u8>)
{
  let conn: &mut netconn;
  let newconn: &mut netconn;
  let err: err_t;

  conn = netconn_new(NETCONN_TCP_IPV6);
  // LWIP_ERROR("shell: invalid conn", (conn != NULL), return;);
  err = netconn_bind(conn, IP6_ADDR_ANY, 23);
 /* LWIP_IPV6 */
  conn = netconn_new(NETCONN_TCP);
  // LWIP_ERROR("shell: invalid conn", (conn != NULL), return;);
  err = netconn_bind(conn, IP_ADDR_ANY, 23);

  // LWIP_ERROR("shell: netconn_bind failed", (err == ERR_OK), netconn_delete(conn); return;);
  err = netconn_listen(conn);
  // LWIP_ERROR("shell: netconn_listen failed", (err == ERR_OK), netconn_delete(conn); return;);

  loop {
    err = netconn_accept(conn, &newconn);
    if (err == ERR_OK) {
      shell_main(newconn);
      netconn_delete(newconn);
    }
  }
}
/*-----------------------------------------------------------------------------------*/
pub fn 
shell_init()
{
  sys_thread_new("shell_thread", shell_thread, None, DEFAULT_THREAD_STACKSIZE, DEFAULT_THREAD_PRIO);
}


