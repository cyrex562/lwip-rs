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

// #define LWIP_HDR_APPS_FS_H








#define FS_READ_EOF     -1
#define FS_READ_DELAYED -2


struct fsdata_chksum {
  offset: u32;
  chksum: u16;
  len: u16;
};


pub const FS_FILE_FLAGS_HEADER_INCLUDED: u32 = 0x01;pub const FS_FILE_FLAGS_HEADER_INCLUDED: u32 = 0x01;pub const FS_FILE_FLAGS_HEADER_INCLUDED: u32 = 0x01;pub const FS_FILE_FLAGS_HEADER_INCLUDED: u32 = 0x01;
#define FS_FILE_FLAGS_HEADER_PERSISTENT   0x02
#define FS_FILE_FLAGS_HEADER_HTTPVER_1_1  0x04
#define FS_FILE_FLAGS_SSI                 0x08

/* Define FS_FILE_EXTENSION_T_DEFINED if you have typedef'ed to your private
 * pointer type (defaults to 'void' so the default usage is 'void*')
 */

typedef void fs_file_extension;


struct fs_file {
  data: String;
  len: i32;
  index: i32;
  /* pextension is free for implementations to hold private (extensional)
     arbitrary data, e.g. holding some file state or file system handle */
  fs_file_extension *pextension;

  const chksum: &mut fsdata_chksum;
  chksum_count: u16;

  flags: u8;

  is_custom_file: u8;


  void *state;

};


typedef void (*fs_wait_cb)(arg: &mut Vec<u8>);


pub fn  fs_open(file: &mut fs_file, name: &String);
pub fn  fs_close(file: &mut fs_file);


fs_read_async: i32(file: &mut fs_file, char *buffer, count: i32, fs_wait_cb callback_fn, void *callback_arg);
#else /* LWIP_HTTPD_FS_ASYNC_READ */
fs_read: i32(file: &mut fs_file, char *buffer, count: i32);



fs_is_file_ready: i32(file: &mut fs_file, fs_wait_cb callback_fn, void *callback_arg);

fs_bytes_left: i32(file: &mut fs_file);


/* This user-defined function is called when a file is opened. */
pub fn  *fs_state_init(file: &mut fs_file, name: &String);
/* This user-defined function is called when a file is closed. */
pub fn  fs_state_free(file: &mut fs_file, void *state);


struct fsdata_file {
  const next: &mut fsdata_file;
  const unsigned char *name;
  const unsigned char *data;
  len: i32;
  flags: u8;

  chksum_count: u16;
  const chksum: &mut fsdata_chksum;

};


}



