/*
 * @file
 * TCPIP API internal implementations (do not use in application code)
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

// #define LWIP_HDR_TCPIP_PRIV_H

// PacketBuffer;
// NetIfc;

// #define (name)               (*(name))
// #define API_VAR_DECLARE(type, name)     type * name
// #define API_VAR_ALLOC_EXT(type, pool, name, errorblock) loop { \
//                                           name = (type *)memp_malloc(pool); \
//                                           if (name == None) { \
//                                             errorblock; \
//                                           } \
//                                         } while(0)
// #define API_VAR_ALLOC(type, pool, name, errorval) API_VAR_ALLOC_EXT(type, pool, name, return errorval)
// #define API_VAR_ALLOC_POOL(type, pool, name, errorval) loop { \
//                                           name = (type *)LWIP_MEMPOOL_ALLOC(pool); \
//                                           if (name == None) { \
//                                             return errorval; \
//                                           } \
//                                         } while(0)
// #define API_VAR_FREE(pool, name)        memp_free(pool, name)
// #define API_VAR_FREE_POOL(pool, name)   LWIP_MEMPOOL_FREE(pool, name)
// #define API_EXPR_REF(expr)              (&(expr))

// #define API_EXPR_REF_SEM(expr)          (expr)

// #define API_EXPR_REF_SEM(expr)          API_EXPR_REF(expr)

// #define API_EXPR_DEREF(expr)            expr
// #define API_MSG_M_DEF(m)                m
// #define API_MSG_M_DEF_C(t, m)           t m
/* LWIP_MPU_COMPATIBLE */
// #define (name)               name
// #define API_VAR_DECLARE(type, name)     type name
// #define API_VAR_ALLOC_EXT(type, pool, name, errorblock)
// #define API_VAR_ALLOC(type, pool, name, errorval)
// #define API_VAR_ALLOC_POOL(type, pool, name, errorval)
// #define API_VAR_FREE(pool, name)
// #define API_VAR_FREE_POOL(pool, name)
// #define API_EXPR_REF(expr)              expr
// #define API_EXPR_REF_SEM(expr)          API_EXPR_REF(expr)
// #define API_EXPR_DEREF(expr)            (*(expr))
// #define API_MSG_M_DEF(m)                *m
// #define API_MSG_M_DEF_C(t, m)           const t * m

// pub fn  tcpip_send_msg_wait_sem(fn: tcpip_callback_fn , apimsg: &mut (), sys_sem_t* sem);

pub struct tcpip_api_call_data {
    pub err: err_t,

    pub sem: sys_sem_t,

    /* !LWIP_TCPIP_CORE_LOCKING */
    pub dummy: u8, /* avoid empty struct :-( */
}

// typedef err_t (*tcpip_api_call_fn)(struct tcpip_api_call_data* call);
type tcpip_api_call_fn = fn(call: &mut tcpip_api_call_data);

// pub fn  tcpip_api_call(tcpip_api_call_fn fn, call: &mut tcpip_api_call_data);

enum tcpip_msg_type {
    TCPIP_MSG_API,
    TCPIP_MSG_API_CALL,

    TCPIP_MSG_INPKT,

    TCPIP_MSG_TIMEOUT,
    TCPIP_MSG_UNTIMEOUT,

    TCPIP_MSG_CALLBACK,
    TCPIP_MSG_CALLBACK_STATIC,
}

pub struct tcpip_msg {
    pub msg_type: tcpip_msg_type,
    pub function: tcpip_callback_fn,
    pub api_func: tcpip_apip_call_fn,
    pub arg: tcpip_api_call_data,
    pub sem: sys_sem_t,
    pub msg: Vec<u8>,
    pub p: PacketBuffer,
    pub netif: NetIfc,
    pub input_func: netif_input_fn,
    pub ctx: Vec<u8>,
}
