/*
 * @file
 * SNTP client API
 */

/*
 * Copyright (c) 2007-2009 Frédéric Bernon, Simon Goldschmidt
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
 * Author: Frédéric Bernon, Simon Goldschmidt
 *
 */

//

/* SNTP operating modes: default is to poll using unicast.
The mode has to be set before calling sntp_init(). */
pub const SNTP_OPMODE_POLL: u32 = 0;
pub const SNTP_OPMODE_LISTENONLY: u32 = 1;
// pub fn  sntp_setoperatingmode(operating_mode: u8);
// sntp_getoperatingmode: u8();

// pub fn  sntp_init();
// pub fn  sntp_stop();
// sntp_enabled: u8();

// pub fn  sntp_setserver(idx: u8,  addr: &mut LwipAddr);
// const sntp_getserver: &mut LwipAddr(idx: u8);

// sntp_getreachability: u8(idx: u8);

// pub fn  sntp_setservername(idx: u8, server: &String);
// sntp_getservername: &String(idx: u8);

// pub fn  sntp_servermode_dhcp(set_servers_from_dhcp: i32);
//  //  SNTP_GET_SERVERS_FROM_DHCP
// #define sntp_servermode_dhcp(x)
