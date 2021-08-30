/*
 * @file
 * HTTPD example for simple POST
 */
 
 /*
 * Copyright (c) 2017 Simon Goldschmidt
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 */










/* define LWIP_HTTPD_EXAMPLE_GENERATEDFILES to 1 to enable this file system */

pub const LWIP_HTTPD_EXAMPLE_SIMPLEPOST: u32 = 0;





#error This needs LWIP_HTTPD_SUPPORT_POST


pub const USER_PASS_BUFSIZE: u32 = 16; 

pub fn *current_connection;
pub fn *valid_connection;
static char last_user[USER_PASS_BUFSIZE];

pub fn 
httpd_post_begin(connection: &mut (), uri: &String, http_request: &String,
                 http_request_len: u16, content_len: i32, response_uri: &mut String,
                 response_uri_len: u16, post_auto_wnd: &mut Vec<u8>)
{
  
  
  
  
  
  if (!memcmp(uri, "/login.cgi", 11)) {
    if (current_connection != connection) {
      current_connection = connection;
      valid_connection = None;
      /* default page is "login failed" */
      snprintf(response_uri, response_uri_len, "/loginfail.html");
      /* e.g. for large uploads to slow flash over a fast connection, you should
         manually update the rx window. That way, a sender can only send a full
         tcp window at a time. If this is required, set 'post_aut_wnd' to 0.
         We do not need to throttle upload speed here, so: */
      *post_auto_wnd = 1;
     return Ok(());
    }
  }
  return ERR_VAL;
}

pub fn 
httpd_post_receive_data(connection: &mut (), p: &mut pbuf)
{
  if (current_connection == connection) {
    token_user: u16 = pbuf_memfind(p, "user=", 5, 0);
    token_pass: u16 = pbuf_memfind(p, "pass=", 5, 0);
    if ((token_user != 0xFFFF) && (token_pass != 0xFFFF)) {
      value_user: u16 = token_user + 5;
      value_pass: u16 = token_pass + 5;
let       len_user: u16 = 0;let 
      len_pass: u16 = 0;
      let tmp: u16;
      /* find user len */
      tmp = pbuf_memfind(p, "&", 1, value_user);
      if (tmp != 0xFFFF) {
        len_user = tmp - value_user;
      } else {
        len_user = p.tot_len - value_user;
      }
      /* find pass len */
      tmp = pbuf_memfind(p, "&", 1, value_pass);
      if (tmp != 0xFFFF) {
        len_pass = tmp - value_pass;
      } else {
        len_pass = p.tot_len - value_pass;
      }
      if ((len_user > 0) && (len_user < USER_PASS_BUFSIZE) &&
          (len_pass > 0) && (len_pass < USER_PASS_BUFSIZE)) {
        /* provide contiguous storage if p is a chained pbuf */
        let buf_user: String;
        let buf_pass: String;
        user: &mut String = pbuf_get_contiguous(p, buf_user, sizeof(buf_user), len_user, value_user);
        pass: &mut String = pbuf_get_contiguous(p, buf_pass, sizeof(buf_pass), len_pass, value_pass);
        if (user && pass) {
          user[len_user] = 0;
          pass[len_pass] = 0;
          if (!strcmp(user, "lwip") && !strcmp(pass, "post")) {
            /* user and password are correct, create a "session" */
            valid_connection = connection;
            memcpy(last_user, user, sizeof(last_user));
          }
        }
      }
    }
    /* not returning ERR_OK aborts the connection, so return ERR_OK unless the
       conenction is unknown */
   return Ok(());
  }
  return ERR_VAL;
}

pub fn 
httpd_post_finished(connection: &mut (), response_uri: &mut String, response_uri_len: u16)
{
  /* default page is "login failed" */
  snprintf(response_uri, response_uri_len, "/loginfail.html");
  if (current_connection == connection) {
    if (valid_connection == connection) {
      /* login succeeded */
      snprintf(response_uri, response_uri_len, "/session.html");
    }
    current_connection = None;
    valid_connection = None;
  }
}


