/**
 * @file
 * Debug messages infrastructure
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

#define LWIP_HDR_DEBUG_H




/**
 * @defgroup debugging_levels LWIP_DBG_MIN_LEVEL and LWIP_DBG_TYPES_ON values
 * @ingroup lwip_opts_debugmsg
 * @{
 */

/** @name Debug level (LWIP_DBG_MIN_LEVEL)
 * @{
 */
/** Debug level: ALL messages*/
pub const LWIP_DBG_LEVEL_ALL: u32 = 0x00;
/** Debug level: Warnings. bad checksums, dropped packets, ... */
pub const LWIP_DBG_LEVEL_WARNING: u32 = 0x01;
/** Debug level: Serious. memory allocation failures, ... */
pub const LWIP_DBG_LEVEL_SERIOUS: u32 = 0x02;
/** Debug level: Severe */
pub const LWIP_DBG_LEVEL_SEVERE: u32 = 0x03;
/**
 * @}
 */

pub const LWIP_DBG_MASK_LEVEL: u32 = 0x03;
/* compatibility define only */
#define LWIP_DBG_LEVEL_OFF     LWIP_DBG_LEVEL_ALL

/** @name Enable/disable debug messages completely (LWIP_DBG_TYPES_ON)
 * @{
 */
/** flag for LWIP_DEBUGF to enable that debug message */
pub const LWIP_DBG_ON: u32 = 0x80;U
/** flag for LWIP_DEBUGF to disable that debug message */
pub const LWIP_DBG_OFF: u32 = 0x00;U
/**
 * @}
 */

/** @name Debug message types (LWIP_DBG_TYPES_ON)
 * @{
 */
/** flag for LWIP_DEBUGF indicating a tracing message (to follow program flow) */
pub const LWIP_DBG_TRACE: u32 = 0x40;U
/** flag for LWIP_DEBUGF indicating a state debug message (to follow module states) */
pub const LWIP_DBG_STATE: u32 = 0x20;U
/** flag for LWIP_DEBUGF indicating newly added code, not thoroughly tested yet */
pub const LWIP_DBG_FRESH: u32 = 0x10;U
/** flag for LWIP_DEBUGF to halt after printing this debug message */
pub const LWIP_DBG_HALT: u32 = 0x08;U
/**
 * @}
 */

/**
 * @}
 */

/**
 * @defgroup lwip_assertions Assertion handling
 * @ingroup lwip_opts_debug
 * @{
 */
/**
 * LWIP_NOASSERT: Disable LWIP_ASSERT checks:
 * To disable assertions define LWIP_NOASSERT in arch/cc.h.
 */

#define LWIP_NOASSERT
#undef LWIP_NOASSERT

/**
 * @}
 */


#define LWIP_ASSERT(message, assertion) do { if (!(assertion)) { \
  LWIP_PLATFORM_ASSERT(message); }} while(0)
#else  /* LWIP_NOASSERT */
#define LWIP_ASSERT(message, assertion)




#define LWIP_PLATFORM_ERROR(message) LWIP_PLATFORM_ASSERT(message)
#elif defined LWIP_DEBUG
#define LWIP_PLATFORM_ERROR(message) LWIP_PLATFORM_DIAG((message))
#else
#define LWIP_PLATFORM_ERROR(message)


/* if "expression" isn't true, then print "message" and execute "handler" expression */
#define LWIP_ERROR(message, expression, handler) do { if (!(expression)) { \
  LWIP_PLATFORM_ERROR(message); handler;}} while(0)


/** Enable debug message printing, but only if debug message type is enabled
 *  AND is of correct type AND is at least LWIP_DBG_LEVEL.
 */

#define LWIP_DEBUG
#undef LWIP_DEBUG



#define LWIP_DEBUGF(debug, message) do { \
                               if ( \
                                   ((debug) & LWIP_DBG_ON) && \
                                   ((debug) & LWIP_DBG_TYPES_ON) && \
                                   ((i16)((debug) & LWIP_DBG_MASK_LEVEL) >= LWIP_DBG_MIN_LEVEL)) { \
                                 LWIP_PLATFORM_DIAG(message); \
                                 if ((debug) & LWIP_DBG_HALT) { \
                                   while(1); \
                                 } \
                               } \
                             } while(0)

#else  /* LWIP_DEBUG */
#define LWIP_DEBUGF(debug, message)



