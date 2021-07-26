/**
 * @file
 * HTTPD custom file system example for runtime generated files
 *
 * This file demonstrates how to add support for generated files to httpd.
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











/** define LWIP_HTTPD_EXAMPLE_GENERATEDFILES to 1 to enable this file system */

pub const LWIP_HTTPD_EXAMPLE_GENERATEDFILES: u32 = 0;





#error This needs LWIP_HTTPD_CUSTOM_FILES


#error This needs LWIP_HTTPD_DYNAMIC_HEADERS


/* This is the page we send. It's not generated, as you see.
 * Generating custom things instead of memcpy is left to your imagination :-)
 */
const char generated_html[] =
"<html>\n"
"<head><title>lwIP - A Lightweight TCP/IP Stack</title></head>\n"
" <body bgcolor=\"white\" text=\"black\">\n"
"  <table width=\"100%\">\n"
"   <tr valign=\"top\">\n"
"    <td width=\"80\">\n"
"     <a href=\"http://www.sics.se/\"><img src=\"/img/sics.gif\"\n"
"      border=\"0\" alt=\"SICS logo\" title=\"SICS logo\"></a>\n"
"    </td>\n"
"    <td width=\"500\">\n"
"     <h1>lwIP - A Lightweight TCP/IP Stack</h1>\n"
"     <h2>Generated page</h2>\n"
"     <p>This page might be generated in-memory at runtime</p>\n"
"    </td>\n"
"    <td>\n"
"    &nbsp;\n"
"    </td>\n"
"   </tr>\n"
"  </table>\n"
" </body>\n"
"</html>";


pub fn 
genfiles_ex_init(void)
{
  /* nothing to do here yet */
}

pub fn fs_open_custom(file: &mut fs_file, const char *name)
{
  /* this example only provides one file */
  if (!strcmp(name, "/generated.html")) {
    /* initialize fs_file correctly */
    memset(file, 0, sizeof(struct fs_file));
    file->pextension = mem_malloc(sizeof(generated_html));
    if (file->pextension != NULL) {
      /* instead of doing memcpy, you would generate e.g. a JSON here */
      memcpy(file->pextension, generated_html, sizeof(generated_html));
      file->data = (const char *)file->pextension;
      file->len = sizeof(generated_html) - 1; /* don't send the trailing 0 */
      file->index = file->len;
      /* allow persisteng connections */
      file->flags = FS_FILE_FLAGS_HEADER_PERSISTENT;
      return 1;
    }
  }
  return 0;
}

pub fn 
fs_close_custom(file: &mut fs_file)
{
  if (file && file->pextension) {
    mem_free(file->pextension);
    file->pextension = NULL;
  }
}


u8
fs_canread_custom(file: &mut fs_file)
{
  LWIP_UNUSED_ARG(file);
  /* This example does not use delayed/async reading */
  return 1;
}

u8
fs_wait_read_custom(file: &mut fs_file, fs_wait_cb callback_fn, void *callback_arg)
{
  LWIP_ASSERT("not implemented in this example configuration", 0);
  LWIP_UNUSED_ARG(file);
  LWIP_UNUSED_ARG(callback_fn);
  LWIP_UNUSED_ARG(callback_arg);
  /* Return
     - 1 if ready to read (at least one byte)
     - 0 if reading should be delayed (call 'tcpip_callback(callback_fn, callback_arg)' when ready) */
  return 1;
}

pub fn fs_read_async_custom(file: &mut fs_file, char *buffer, count: int, fs_wait_cb callback_fn, void *callback_arg)
{
  LWIP_ASSERT("not implemented in this example configuration", 0);
  LWIP_UNUSED_ARG(file);
  LWIP_UNUSED_ARG(buffer);
  LWIP_UNUSED_ARG(count);
  LWIP_UNUSED_ARG(callback_fn);
  LWIP_UNUSED_ARG(callback_arg);
  /* Return
     - FS_READ_EOF if all bytes have been read
     - FS_READ_DELAYED if reading is delayed (call 'tcpip_callback(callback_fn, callback_arg)' when done) */
  /* all bytes read already */
  return FS_READ_EOF;
}

#else /* LWIP_HTTPD_FS_ASYNC_READ */
pub fn fs_read_custom(file: &mut fs_file, char *buffer, count: int)
{
  LWIP_ASSERT("not implemented in this example configuration", 0);
  LWIP_UNUSED_ARG(file);
  LWIP_UNUSED_ARG(buffer);
  LWIP_UNUSED_ARG(count);
  /* Return
     - FS_READ_EOF if all bytes have been read
     - FS_READ_DELAYED if reading is delayed (call 'tcpip_callback(callback_fn, callback_arg)' when done) */
  /* all bytes read already */
  return FS_READ_EOF;
}




