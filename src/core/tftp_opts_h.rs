/*
 *
 * @file tftp_opts.h
 *
 * @author   Logan Gunthorpe <logang@deltatee.com>
 *
 * @brief    Trivial File Transfer Protocol (RFC 1350) implementation options
 *
 * Copyright (c) Deltatee Enterprises Ltd. 2013
 * All rights reserved.
 *
 */

/* 
 * Redistribution and use in source and binary forms, with or without
 * modification,are permitted provided that the following conditions are met:
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
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Author: Logan Gunthorpe <logang@deltatee.com>
 *
 */


// #define LWIP_HDR_APPS_TFTP_OPTS_H




/*
 * @defgroup tftp_opts Options
 * @ingroup tftp
 * @{
 */

/*
 * Enable TFTP debug messages
 */

pub const TFTP_DEBUG: u32 = LWIP_DBG_OFF;


/*
 * TFTP server port
 */

pub const TFTP_PORT: u32 = LWIP_IANA_PORT_TFTP;


/*
 * TFTP timeout
 */

pub const TFTP_TIMEOUT_MSECS: u32 = 10000; 


/*
 * Max. number of retries when a file is read from server
 */

pub const TFTP_MAX_RETRIES: u32 = 5; 


/*
 * TFTP timer cyclic interval
 */

#define TFTP_TIMER_MSECS      (TFTP_TIMEOUT_MSECS / 10)


/*
 * Max. length of TFTP filename
 */

pub const TFTP_MAX_FILENAME_LEN: u32 = 20; 


/*
 * Max. length of TFTP mode
 */

pub const TFTP_MAX_MODE_LEN: u32 = 7; 


/*
 * @}
 */


