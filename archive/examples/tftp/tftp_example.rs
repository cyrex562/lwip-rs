/*
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */

pub fn tftp_open(fname: &mut String, mode: &mut String, is_write: u8) {
    // 

    if (is_write) {
        return fopen(fname, "wb");
    } else {
        return fopen(fname, "rb");
    }
}

pub fn tftp_close(handle: &mut Vec<u8>) {
    // fclose(handle);
    unimplemented!()
}

pub fn tftp_read(handle: &mut Vec<u8>, buf: &mut Vec<u8>, bytes: i32) -> i32 {
    let ret: i32 = fread(buf, 1, bytes, handle);
    if (ret <= 0) {
        return -1;
    }
    return ret;
}

pub fn tftp_write(handle: &mut Vec<u8>, p: &mut PacketBuffer) -> i32 {
    while (p != None) {
        if (fwrite(p.payload, 1, p.len, handle) != p.len) {
            return -1;
        }
        p = p.next;
    }

    return 0;
}

// static const struct tftp_context tftp = {
//   tftp_open,
//   tftp_close,
//   tftp_read,
//   tftp_write
// };

pub fn tftp_example_init() {
    tftp_init(&tftp);
}
