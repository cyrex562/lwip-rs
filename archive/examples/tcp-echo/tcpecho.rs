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

// -----------------------------------------------------------------------------------
pub fn tcpecho_thread(arg: &mut Vec<u8>) {
    let conn: netconn;
    let newconn: netconn;
    let err: err_t;

    //  Create a new connection identifier. 
    //  Bind connection to well known port number 7. 

    conn = netconn_new(NETCONN_TCP_IPV6);
    netconn_bind(conn, IP6_ADDR_ANY, 7);
    //  LWIP_IPV6 
    conn = netconn_new(NETCONN_TCP);
    netconn_bind(conn, IP_ADDR_ANY, 7);

    // LWIP_ERROR("tcpecho: invalid conn", (conn != NULL), return;);

    //  Tell connection to go into listening mode. 
    netconn_listen(conn);

    loop {
        //  Grab new connection. 
        err = netconn_accept(conn, &newconn);
        // printf("accepted new connection %p\n", newconn);
        //  Process the new connection. 
        if (err == ERR_OK) {
            let buf: &mut netbuf;
            let data: &mut Vec<u8>;
            let len: usize;

            while ((err = netconn_recv(newconn, &buf)) == ERR_OK) {
                // printf("Recved\n");
                loop {
                    netbuf_data(buf, &data, &len);
                    err = netconn_write(newconn, data, len, NETCONN_COPY);

                    if (err != ERR_OK) {
                        printf("tcpecho: netconn_write: error \"%s\"\n", lwip_strerr(err));
                    }
                    if netbuf_next(buf) == 0 {
                        break;
                    }
                }
                netbuf_delete(buf);
            }
            // printf("Got EOF, looping\n");
            //  Close connection and discard connection identifier. 
            netconn_close(newconn);
            netconn_delete(newconn);
        }
    }
}
// -----------------------------------------------------------------------------------
pub fn tcpecho_init() {
    sys_thread_new(
        "tcpecho_thread",
        tcpecho_thread,
        None,
        DEFAULT_THREAD_STACKSIZE,
        DEFAULT_THREAD_PRIO,
    );
}
// -----------------------------------------------------------------------------------
