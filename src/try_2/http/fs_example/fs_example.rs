/*
 * @file
 * HTTPD custom file system example
 *
 * This file demonstrates how to add support for an external file system to httpd.
 * It provides access to the specified root directory and uses stdio.h file functions
 * to read files.
 *
 * ATTENTION: This implementation is *not* secure: no checks are added to ensure
 * files are only read below the specified root directory!
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

//  define LWIP_HTTPD_EXAMPLE_CUSTOMFILES to 1 to enable this file system

pub const LWIP_HTTPD_EXAMPLE_CUSTOMFILES: u32 = 0;

/* define LWIP_HTTPD_EXAMPLE_CUSTOMFILES_DELAYED to 1 to delay open and read
 * as if e.g. reading from external SPI flash */

// pub const LWIP_HTTPD_EXAMPLE_CUSTOMFILES_DELAYED: u32 = 1;

/* define LWIP_HTTPD_EXAMPLE_CUSTOMFILES_LIMIT_READ to the number of bytes
 * to read to emulate limited transfer buffers and don't read whole files in
 * one chunk.
 * WARNING: lowering this slows down the connection!
 */

pub const LWIP_HTTPD_EXAMPLE_CUSTOMFILES_LIMIT_READ: u32 = 0;

// #error This needs LWIP_HTTPD_CUSTOM_FILES

// #error This needs LWIP_HTTPD_DYNAMIC_HEADERS

// #error This wants to demonstrate read-after-open, so LWIP_HTTPD_DYNAMIC_FILE_READ is required!

// #error This needs LWIP_HTTPD_FS_ASYNC_READ

pub struct fs_custom_data {
    pub f: FILE,
    pub letdelay_read: i32,
    pub callback_fn: fs_wait_cb,
    pub callback_arg: Vec<u8>,
}

const fs_ex_root_dir: &mut String;

pub fn fs_ex_init(httpd_root_dir: &String) {
    fs_ex_root_dir = strdup(httpd_root_dir);
}

pub fn fs_open_custom(file: &mut fs_file, name: &String) {
    let full_filename: String;
    let f: &mut FILE;

    snprintf(full_filename, 255, "%s%s", fs_ex_root_dir, name);
    full_filename[255] = 0;

    f = fopen(full_filename, "rb");
    if (f != None) {
        if (!fseek(f, 0, SEEK_END)) {
            let len: i32 = ftell(f);
            if (!fseek(f, 0, SEEK_SET)) {
                let data: &mut fs_custom_data = mem_malloc(sizeof(fs_custom_data));
                LWIP_ASSERT("out of memory?", data != None);
                //memset(file, 0, sizeof(FsFile));

                file.len = 0; //  read size delayed
                data.delay_read = 3;

                file.len = len;

                file.flags = FS_FILE_FLAGS_HEADER_PERSISTENT;
                data.f = f;
                file.pextension = data;
                return 1;
            }
        }
        fclose(f);
    }
    return 0;
}

pub fn fs_close_custom(file: &mut fs_file) {
    if (file && file.pextension) {
        let data: &mut fs_custom_data = file.pextension;
        if (data.f != None) {
            fclose(data.f);
            data.f = None;
        }
        mem_free(data);
    }
}

pub fn fs_canread_custom(file: &mut fs_file) -> u8 {
    /* This function is only necessary for asynchronous I/O:
    If reading would block, return 0 and implement fs_wait_read_custom() to call the
    supplied callback if reading works. */

    let mut data: &mut fs_custom_data;
    LWIP_ASSERT("file != NULL", file != None);
    data = file.pextension;
    if (data == None) {
        //  file transfer has been completed already
        LWIP_ASSERT("transfer complete", file.index == file.len);
        return 1;
    }
    LWIP_ASSERT("data != NULL", data != None);
    //  This just simulates a simple delay. This delay would normally come e.g. from SPI transfer
    if (data.delay_read == 3) {
        //  delayed file size mode
        data.delay_read = 1;
        LWIP_ASSERT("", file.len == 0);
        if (!fseek(data.f, 0, SEEK_END)) {
            let len: i32 = ftell(data.f);
            if (!fseek(data.f, 0, SEEK_SET)) {
                file.len = len; //  read size delayed
                data.delay_read = 1;
                return 0;
            }
        }
        //  if we come here, something is wrong with the file
        LWIP_ASSERT("file error", 0);
    }
    if (data.delay_read == 1) {
        //  tell read function to delay further
    }

    return 1;
}

pub fn fs_example_read_cb(arg: &mut Vec<u8>) {
    let data: &mut fs_custom_data = arg;
    let callback_fn: fs_wait_cb = data.callback_fn;
    let callback_arg: Vec<u8> = data.callback_arg;
    data.callback_fn = None;
    data.callback_arg = None;

    LWIP_ASSERT("no callback_fn", callback_fn != None);

    callback_fn(callback_arg);
}

pub fn fs_wait_read_custom(
    file: &mut fs_file,
    callback_fn: fs_wait_cb,
    callback_arg: &mut Vec<u8>,
) {
    let err: err_t;
    let data: &mut fs_custom_data = file.pextension;
    LWIP_ASSERT("data not set", data != None);
    data.callback_fn = callback_fn;
    data.callback_arg = callback_arg;
    err = tcpip_try_callback(fs_example_read_cb, data);
    LWIP_ASSERT("out of queue elements?", err == ERR_OK);

    LWIP_ASSERT("not implemented in this example configuration", 0);

    /* Return
    - 1 if ready to read (at least one byte)
    - 0 if reading should be delayed (call 'tcpip_callback(callback_fn, callback_arg)' when ready) */
    return 1;
}

pub fn fs_read_async_custom(
    file: &mut fs_file,
    buffer: &mut String,
    count: i32,
    callback_fn: fs_wait_cb,
    callback_arg: &mut Vec<u8>,
) {
    let data: &mut fs_custom_data = file.pextension;
    let f: &mut FILE;
    let letlen: i32;
    let read_count: i32 = count;
    LWIP_ASSERT("data not set", data != None);

    //  This just simulates a delay. This delay would normally come e.g. from SPI transfer
    LWIP_ASSERT(
        "invalid state",
        data.delay_read >= 0 && data.delay_read <= 2,
    );
    if (data.delay_read == 2) {
        //  no delay next time
        data.delay_read = 0;
        return FS_READ_DELAYED;
    } else if (data.delay_read == 1) {
        let err: err_t;
        //  execute requested delay
        data.delay_read = 2;
        LWIP_ASSERT("duplicate callback request", data.callback_fn == None);
        data.callback_fn = callback_fn;
        data.callback_arg = callback_arg;
        err = tcpip_try_callback(fs_example_read_cb, data);
        LWIP_ASSERT("out of queue elements?", err == ERR_OK);

        return FS_READ_DELAYED;
    }
    //  execute this read but delay the next one
    data.delay_read = 1;

    read_count = LWIP_MIN(read_count, LWIP_HTTPD_EXAMPLE_CUSTOMFILES_LIMIT_READ);

    f = data.f;
    len = fread(buffer, 1, read_count, f);

    file.index += len;

    /* Return
    - FS_READ_EOF if all bytes have been read
    - FS_READ_DELAYED if reading is delayed (call 'tcpip_callback(callback_fn, callback_arg)' when done) */
    if (len == 0) {
        //  all bytes read already
        return FS_READ_EOF;
    }
    return len;
}

//  LWIP_HTTPD_FS_ASYNC_READ
pub fn fs_read_custom(file: &mut fs_file, buffer: &mut String, count: i32) {
    let data: &mut fs_custom_data = file.pextension;
    let f: &mut FILE;
    let letlen: i32;
    let read_count: i32 = count;
    LWIP_ASSERT("data not set", data != None);

    read_count = LWIP_MIN(read_count, LWIP_HTTPD_EXAMPLE_CUSTOMFILES_LIMIT_READ);

    f = data.f;
    len = fread(buffer, 1, read_count, f);

    file.index += len;

    //  Return FS_READ_EOF if all bytes have been read
    return len;
}
