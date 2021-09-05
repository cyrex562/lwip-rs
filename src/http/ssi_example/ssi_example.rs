/*
 * @file
 * HTTPD simple SSI example
 *
 * This file demonstrates how to add support for SSI.
 * It does this in a very simple way by providing the three tags 'HelloWorld'
 * 'counter', and 'MultiPart'.
 *
 * This file also demonstrates how to integrate CGI with SSI.
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












/* define LWIP_HTTPD_EXAMPLE_SSI_SIMPLE to 1 to enable this ssi example*/

pub const LWIP_HTTPD_EXAMPLE_SSI_SIMPLE: u32 = 0;


/* define LWIP_HTTPD_EXAMPLE_SSI_SIMPLE_CGI_INTEGRATION to 1 to show how to
 * integrate CGI into SSI (LWIP_HTTPD_CGI_SSI) */

pub const LWIP_HTTPD_EXAMPLE_SSI_SIMPLE_CGI_INTEGRATION: u32 = 0;






#error LWIP_HTTPD_EXAMPLE_SSI_SIMPLE_CGI_INTEGRATION needs LWIP_HTTPD_FILE_STATE


#error LWIP_HTTPD_EXAMPLE_SSI_SIMPLE_CGI_INTEGRATION needs LWIP_HTTPD_CGI_SSI


pub const MAX_CGI_LEN: u32 = 16; 


const char * ssi_example_tags[] = {
  "HellWorl",
  "counter",
  "MultPart"

  ,"CgiParam"

};

ssi_example_ssi_handler: u16(

 ssi_tag_name: &mut String,
 /* LWIP_HTTPD_SSI_RAW */
                             iIndex: i32,

                             pcInsert: &mut String, iInsertLen: i32

                             , current_tag_part: u16, next_tag_part: &mut u16


                             , connection_state: &mut ()

                             )
{
  let printed: usize;

  /* a real application could use if(!strcmp) blocks here, but we want to keep
     the differences between configurations small, so translate string to index here */
  let letiIndex: i32;
  for (iIndex = 0; iIndex < LWIP_ARRAYSIZE(ssi_example_tags); iIndex+= 1) {
    if(!strcmp(ssi_tag_name, ssi_example_tags[iIndex])) {
      break;
    }
  }


  


  match (iIndex) {
  0 => /* "HelloWorld" */
    printed = snprintf(pcInsert, iInsertLen, "Hello World!");
    break;
  1 => /* "counter" */
    {
      static counter: i32;
      counter+= 1;
      printed = snprintf(pcInsert, iInsertLen, "%d", counter);
    }
    break;
  2 => /* "MultPart" */

    match (current_tag_part) {
    0 =>
      printed = snprintf(pcInsert, iInsertLen, "part0");
      *next_tag_part = 1;
      break;
    1 =>
      printed = snprintf(pcInsert, iInsertLen, "part1");
      *next_tag_part = 2;
      break;
    2 =>
      printed = snprintf(pcInsert, iInsertLen, "part2");
      break;
    _ =>
      printed = snprintf(pcInsert, iInsertLen, "unhandled part: %d", current_tag_part);
      break;
    }

    printed = snprintf(pcInsert, iInsertLen, "LWIP_HTTPD_SSI_MULTIPART disabled");

    break;

  3 =>
    if (connection_state) {
      params: &mut String = connection_state;
      if (*params) {
        printed = snprintf(pcInsert, iInsertLen, "%s", params);
      } else {
        printed = snprintf(pcInsert, iInsertLen, "none");
      }
    } else {
       printed = snprintf(pcInsert, iInsertLen, "NULL");
    }
    break;

  _ => /* unknown tag */
    printed = 0;
    break;
  }
  LWIP_ASSERT("sane length", printed <= 0xFFFF);
  return printed;
}

pub fn 
ssi_ex_init()
{
  let leti: i32;
  for (i = 0; i < LWIP_ARRAYSIZE(ssi_example_tags); i+= 1) {
    LWIP_ASSERT("tag too long for LWIP_HTTPD_MAX_TAG_NAME_LEN",
      strlen(ssi_example_tags[i]) <= LWIP_HTTPD_MAX_TAG_NAME_LEN);
  }

  http_set_ssi_handler(ssi_example_ssi_handler,

    None, 0

    ssi_example_tags, LWIP_ARRAYSIZE(ssi_example_tags)

    );
}


pub fn  *
fs_state_init(file: &mut fs_file, name: &String)
{
  let mut ret: &mut String;
  
  
  ret = mem_malloc(MAX_CGI_LEN);
  if (ret) {
    *ret = 0;
  }
  return ret;
}

pub fn 
fs_state_free(file: &mut fs_file, state: &mut ())
{
  
  if (state != None) {
    mem_free(state);
  }
}

pub fn 
httpd_cgi_handler(file: &mut fs_file,  uri: &mut String, iNumParams: i32,
                              pcParam: &mut String, pcValue: &mut String

                                     , connection_state: &mut ()

                                     )
{
  
  
  if (connection_state != None) {
    start: &mut String = connection_state;
    end: &mut String = start + MAX_CGI_LEN;
    let leti: i32;
    //memset(start, 0, MAX_CGI_LEN);
    /* pra: i32 string of the arguments: */
    for (i = 0; i < iNumParams; i+= 1) {
      let len: usize;
      len = end - start;
      if (len) {
        inlen: usize = strlen(pcParam[i]);
        copylen: usize = LWIP_MIN(inlen, len);
        memcpy(start, pcParam[i], copylen);
        start += copylen;
        len -= copylen;
      }
      if (len) {
        *start = '=';
        start+= 1;
        len -= 1;
      }
      if (len) {
        inlen: usize = strlen(pcValue[i]);
        copylen: usize = LWIP_MIN(inlen, len);
        memcpy(start, pcValue[i], copylen);
        start += copylen;
        len -= copylen;
      }
      if (len) {
        *start = ';';
        len -= 1;
      }
      /* ensure NULL termination */
      end -= 1;
      *end = 0;
    }
  }
}



