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













// struct pbuf;
// struct netif;


#define (name)               (*(name))
#define API_VAR_DECLARE(type, name)     type * name
#define API_VAR_ALLOC_EXT(type, pool, name, errorblock) do { \
                                          name = (type *)memp_malloc(pool); \
                                          if (name == NULL) { \
                                            errorblock; \
                                          } \
                                        } while(0)
#define API_VAR_ALLOC(type, pool, name, errorval) API_VAR_ALLOC_EXT(type, pool, name, return errorval)
#define API_VAR_ALLOC_POOL(type, pool, name, errorval) do { \
                                          name = (type *)LWIP_MEMPOOL_ALLOC(pool); \
                                          if (name == NULL) { \
                                            return errorval; \
                                          } \
                                        } while(0)
#define API_VAR_FREE(pool, name)        memp_free(pool, name)
#define API_VAR_FREE_POOL(pool, name)   LWIP_MEMPOOL_FREE(pool, name)
#define API_EXPR_REF(expr)              (&(expr))

#define API_EXPR_REF_SEM(expr)          (expr)

#define API_EXPR_REF_SEM(expr)          API_EXPR_REF(expr)

#define API_EXPR_DEREF(expr)            expr
#define API_MSG_M_DEF(m)                m
#define API_MSG_M_DEF_C(t, m)           t m
 /* LWIP_MPU_COMPATIBLE */
#define (name)               name
#define API_VAR_DECLARE(type, name)     type name
#define API_VAR_ALLOC_EXT(type, pool, name, errorblock)
#define API_VAR_ALLOC(type, pool, name, errorval)
#define API_VAR_ALLOC_POOL(type, pool, name, errorval)
#define API_VAR_FREE(pool, name)
#define API_VAR_FREE_POOL(pool, name)
#define API_EXPR_REF(expr)              expr
#define API_EXPR_REF_SEM(expr)          API_EXPR_REF(expr)
#define API_EXPR_DEREF(expr)            (*(expr))
#define API_MSG_M_DEF(m)                *m
#define API_MSG_M_DEF_C(t, m)           const t * m


pub fn  tcpip_send_msg_wait_sem(tcpip_callback_fn fn, apimsg: &mut (), sys_sem_t* sem);

struct tcpip_api_call_data
{

  let err: err_t;

  sem: sys_sem_t;

 /* !LWIP_TCPIP_CORE_LOCKING */
  dummy: u8; /* avoid empty struct :-( */

};
typedef err_t (*tcpip_api_call_fn)(struct tcpip_api_call_data* call);
pub fn  tcpip_api_call(tcpip_api_call_fn fn, call: &mut tcpip_api_call_data);

enum tcpip_msg_type {

  TCPIP_MSG_API,
  TCPIP_MSG_API_CALL,


  TCPIP_MSG_INPKT,


  TCPIP_MSG_TIMEOUT,
  TCPIP_MSG_UNTIMEOUT,

  TCPIP_MSG_CALLBACK,
  TCPIP_MSG_CALLBACK_STATIC
};

struct tcpip_msg {
  enum tcpip_msg_type type;
  union {

    struct {
      tcpip_callback_fn function;
      void* msg;
    } api_msg;
    struct {
      tcpip_api_call_fn function;
      arg: &mut tcpip_api_call_data;
      sys_sem_t *sem;
    } api_call;


    struct {
      p: &mut pbuf;
      netif: &mut netif;
      netif_input_fn input_fn;
    } inp;

    struct {
      tcpip_callback_fn function;
      ctx: &mut ();
    } cb;

    struct {
      msecs: u32;
      sys_timeout_handler h;
      arg: &mut Vec<u8>;
    } tmo;

  } msg;
};


}





