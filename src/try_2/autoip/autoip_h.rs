/*
 * @file
 * AutoIP protocol definitions
 */

/*
 *
 * Copyright (c) 2007 Dominik Spies <kontakt@dspies.de>
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
 * Author: Dominik Spies <kontakt@dspies.de>
 *
 * This is a AutoIP implementation for the lwIP TCP/IP stack. It aims to conform
 * with RFC 3927.
 *
 */

//

//  169.254.0.0
pub const AUTOIP_NET: u32 = 0xA9FE0000;
//  169.254.1.0
pub const AUTOIP_RANGE_START: u32 = (AUTOIP_NET | 0x0100);
//  169.254.254.255
pub const AUTOIP_RANGE_END: u32 = (AUTOIP_NET | 0xFEFF);

//  RFC 3927 Constants
pub const PROBE_WAIT: u32 = 1; //  second   (initial random delay)
pub const PROBE_MIN: u32 = 1; //  second   (minimum delay till repeated probe)
pub const PROBE_MAX: u32 = 2; //  seconds  (maximum delay till repeated probe)
pub const PROBE_NUM: u32 = 3; //           (number of probe packets)
pub const ANNOUNCE_NUM: u32 = 2; //           (number of announcement packets)
pub const ANNOUNCE_INTERVAL: u32 = 2; //  seconds  (time between announcement packets)
pub const ANNOUNCE_WAIT: u32 = 2; //  seconds  (delay before announcing)
pub const MAX_CONFLICTS: u32 = 10; //           (max conflicts before rate limiting)
pub const RATE_LIMIT_INTERVAL: u32 = 60; //  seconds  (delay between successive attempts)
pub const DEFEND_INTERVAL: u32 = 10; //  seconds  (min. wait between defensive ARPs)

//  AutoIP client states
pub enum autoip_state_enum {
    AUTOIP_STATE_OFF,        // 0
    AUTOIP_STATE_PROBING,    // 1
    AUTOIP_STATE_ANNOUNCING, // 2
    AUTOIP_STATE_BOUND,      // 3
}
