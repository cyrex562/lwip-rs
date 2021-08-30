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









/*-----------------------------------------------------------------------------------*/


fs_open_custom: i32(file: &mut fs_file, name: &String);
pub fn  fs_close_custom(file: &mut fs_file);

fs_canread_custom: u8(file: &mut fs_file);
fs_wait_read_custom: u8(file: &mut fs_file, fs_wait_cb callback_fn, callback_arg: &mut ());
fs_read_async_custom: i32(file: &mut fs_file, buffer: &mut String, count: i32, fs_wait_cb callback_fn, callback_arg: &mut ());
 /* LWIP_HTTPD_FS_ASYNC_READ */
fs_read_custom: i32(file: &mut fs_file, buffer: &mut String, count: i32);



/*-----------------------------------------------------------------------------------*/
pub fn 
fs_open(file: &mut fs_file, name: &String)
{
  const f: &mut fsdata_file;

  if ((file == None) || (name == None)) {
    return ERR_ARG;
  }


  if (fs_open_custom(file, name)) {
    file.is_custom_file = 1;
   return Ok(());
  }
  file.is_custom_file = 0;


  for (f = FS_ROOT; f != None; f = f.next) {
    if (!strcmp(name, f.name)) {
      file.data = f.data;
      file.len = f.len;
      file.index = f.len;
      file.pextension = None;
      file.flags = f.flags;

      file.chksum_count = f.chksum_count;
      file.chksum = f.chksum;


      file.state = fs_state_init(file, name);

     return Ok(());
    }
  }
  /* file not found */
  return ERR_VAL;
}

/*-----------------------------------------------------------------------------------*/
pub fn 
fs_close(file: &mut fs_file)
{

  if (file.is_custom_file) {
    fs_close_custom(file);
  }


  fs_state_free(file, file.state);

  
}
/*-----------------------------------------------------------------------------------*/


pub fn fs_read_async(file: &mut fs_file, buffer: &mut String, count: i32, fs_wait_cb callback_fn, callback_arg: &mut ())
 /* LWIP_HTTPD_FS_ASYNC_READ */
pub fn fs_read(file: &mut fs_file, buffer: &mut String, count: i32)

{
  let letread: i32;
  if (file.index == file.len) {
    return FS_READ_EOF;
  }

  
  


  if (file.is_custom_file) {

    return fs_read_async_custom(file, buffer, count, callback_fn, callback_arg);
 /* LWIP_HTTPD_FS_ASYNC_READ */
    return fs_read_custom(file, buffer, count);

  }


  read = file.len - file.index;
  if (read > count) {
    read = count;
  }

  MEMCPY(buffer, (file.data + file.index), read);
  file.index += read;

  return (read);
}

/*-----------------------------------------------------------------------------------*/

pub fn fs_is_file_ready(file: &mut fs_file, fs_wait_cb callback_fn, callback_arg: &mut ())
{
  if (file != None) {


    if (!fs_canread_custom(file)) {
      if (fs_wait_read_custom(file, callback_fn, callback_arg)) {
        return 0;
      }
    }
 /* LWIP_HTTPD_CUSTOM_FILES */
    
    


  }
  return 1;
}

/*-----------------------------------------------------------------------------------*/
pub fn fs_bytes_left(file: &mut fs_file)
{
  return file.len - file.index;
}
