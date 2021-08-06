/*
 * @file
 * Functions to sync with TCPIP thread
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

// #define LWIP_HDR_TCPIP_H














/* The global semaphore to lock the stack. */
extern sys_mutex_t lock_tcpip_core;

/* Lock lwIP core mutex (needs @ref LWIP_TCPIP_CORE_LOCKING 1) */
#define LOCK_TCPIP_CORE()     sys_mutex_lock(&lock_tcpip_core)
/* Unlock lwIP core mutex (needs @ref LWIP_TCPIP_CORE_LOCKING 1) */
#define UNLOCK_TCPIP_CORE()   sys_mutex_unlock(&lock_tcpip_core)

#else /* LWIP_TCPIP_CORE_LOCKING */
#define LOCK_TCPIP_CORE()
#define UNLOCK_TCPIP_CORE()


struct pbuf;
struct netif;

/* Function prototype for the init_done function passed to tcpip_init */
typedef void (*tcpip_init_done_fn)(arg: &mut Vec<u8>);
/* Function prototype for functions passed to tcpip_callback() */
typedef void (*tcpip_callback_fn)(void *ctx);

/* Forward declarations */
struct tcpip_callback_msg;

pub fn    tcpip_init(tcpip_init_done_fn tcpip_init_done, arg: &mut Vec<u8>);

pub fn   tcpip_inpkt(p: &mut pbuf, inp: &mut netif, netif_input_fn input_fn);
pub fn   tcpip_input(p: &mut pbuf, inp: &mut netif);

pub fn   tcpip_try_callback(tcpip_callback_fn function, void *ctx);
pub fn   tcpip_callback(tcpip_callback_fn function, void *ctx);
/*  @ingroup lwip_os
 * @deprecated use tcpip_try_callback() or tcpip_callback() instead
 */
#define tcpip_callback_with_block(function, ctx, block) ((block != 0)? tcpip_callback(function, ctx) : tcpip_try_callback(function, ctx))

struct tcpip_callback_msg* tcpip_callbackmsg_new(tcpip_callback_fn function, void *ctx);
pub fn    tcpip_callbackmsg_delete(struct tcpip_callback_msg* msg);
pub fn   tcpip_callbackmsg_trycallback(struct tcpip_callback_msg* msg);
pub fn   tcpip_callbackmsg_trycallback_fromisr(struct tcpip_callback_msg* msg);

/* free pbufs or heap memory from another context without blocking */
pub fn   pbuf_free_callback(p: &mut pbuf);
pub fn   mem_free_callback(void *m);


pub fn   tcpip_timeout(msecs: u32, sys_timeout_handler h, arg: &mut Vec<u8>);
pub fn   tcpip_untimeout(sys_timeout_handler h, arg: &mut Vec<u8>);



tcpip_thread_poll_one: i32();



}





