/*
 * @file
 * lwIP Error codes
 */
/*
 * Copyright (c) 2001-2004 Swedish Institute of Computer Science.
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

use std::fmt;

// TODO: define string messages
// TODO: enum?
#[derive(Debug)]
pub enum LwipErrorCodes {
    ERR_MEM = -1,
    // out of memory error
    ERR_BUF = -2,
    // buffer error
    ERR_TIMEOUT = -3,
    // timeout
    ERR_ROUTING = -4,
    // routing problem
    ERR_INVALID_VAL = -6,
    // illegal value
    ERR_WOULD_BLOCK = -7,
    // operation would block
    ERR_IN_USE = -8,
    // address in use
    ERR_IN_PROGRESS = -9,
    // already connecting
    ERR_CONNECTED = -10,
    // connection already established
    ERR_NO_CONN = -11,
    // not connected
    ERR_NETIF = -12,
    // low-level netif error
    ERR_CONN_ABORTED = -13,
    // connection aborted
    ERR_CONN_RESET = -14,
    // connection reset
    ERR_CONN_CLOSED = -15,
    // connection closed
    ERR_INVALID_ARG = -16,
    // illegal argument
    ERR_INVALID_STATE = -17, // invalid state
    ERR_NOT_FOUND = -18,
}

#[derive(Debug, Clone)]
pub struct LwipError {
    pub code: LwipErrorCodes,
    pub msg: String,
}

impl LwipError {
    pub fn new(code: LwipErrorCodes, msg: &str) -> LwipError {
        LwipError {
            code,
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for LwipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LwipError: code: {:?}, message: {}", self.code, self.msg)
    }
}
