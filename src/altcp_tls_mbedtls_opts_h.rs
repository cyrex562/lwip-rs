/**
 * @file
 * Application layered TCP/TLS connection API (to be used from TCPIP thread)
 *
 * This file contains options for an mbedtls port of the TLS layer.
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







/** LWIP_ALTCP_TLS_MBEDTLS==1: use mbedTLS for TLS support for altcp API
 * mbedtls include directory must be reachable via include search path
 */

pub const LWIP_ALTCP_TLS_MBEDTLS: u32 = 0;


/** Configure debug level of this file */

pub const ALTCP_MBEDTLS_DEBUG: u32 = LWIP_DBG_OFF;


/** Configure lwIP debug level of the mbedTLS library */

pub const ALTCP_MBEDTLS_LIB_DEBUG: u32 = LWIP_DBG_OFF;


/** Configure minimum internal debug level of the mbedTLS library */

pub const ALTCP_MBEDTLS_LIB_DEBUG_LEVEL_MIN: u32 = 0;


/** Enable the basic session cache
 * ATTENTION: Using a session cache can lower security by reusing keys!
 */

pub const ALTCP_MBEDTLS_USE_SESSION_CACHE: u32 = 0;


/** Maximum cache size of the basic session cache */

pub const ALTCP_MBEDTLS_SESSION_CACHE_SIZE: u32 = 30;


/** Set a session timeout in seconds for the basic session cache  */

#define ALTCP_MBEDTLS_SESSION_CACHE_TIMEOUT_SECONDS   (60 * 60)


/** Use session tickets to speed up connection setup (needs
 * MBEDTLS_SSL_SESSION_TICKETS enabled in mbedTLS config).
 * ATTENTION: Using session tickets can lower security by reusing keys!
 */

pub const ALTCP_MBEDTLS_USE_SESSION_TICKETS: u32 = 0;


/** Session ticket cipher */

pub const ALTCP_MBEDTLS_SESSION_TICKET_CIPHER: u32 = MBEDTLS_CIPHER_AES_256_GCM;


/** Maximum timeout for session tickets */

#define ALTCP_MBEDTLS_SESSION_TICKET_TIMEOUT_SECONDS  (60 * 60 * 24)


/** Certificate verification mode: MBEDTLS_SSL_VERIFY_NONE, MBEDTLS_SSL_VERIFY_OPTIONAL (default),
 * MBEDTLS_SSL_VERIFY_REQUIRED (recommended)*/

pub const ALTCP_MBEDTLS_AUTHMODE: u32 = MBEDTLS_SSL_VERIFY_OPTIONAL;


 /* LWIP_ALTCP */

 /* LWIP_HDR_ALTCP_TLS_OPTS_H */
