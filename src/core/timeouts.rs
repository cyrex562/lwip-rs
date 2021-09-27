//! Stack-internal timers implementation
//! This file includes timer callbacks for stack-internal timers as well as functions to set up or stop timers and check for expired timers.


use crate::core::cyclic_timer::LwipCyclicTimer;
use crate::core::system_timeout::{SystemTimeout, SysTimeoutHandler};
use crate::tcp::tcp2::tcp_timer_handler;
use crate::tcp::tcp_priv_h::TCP_TMR_INTERVAL;
use crate::core::context::LwipContext;

/// Returned by sys_timeouts_sleeptime() to indicate there is no timer, so we can sleep forever.
pub const SYS_TIMEOUTS_SLEEPTIME_INFINITE: u64 = u64::MAX;
pub const LWIP_MAX_TIMEOUT: u64 = u64::MAX;

/* Function prototype for a stack-internal timer function that has to be
 * called at a defined interval */


/* Function prototype for a timeout callback function. Register such a function
 * using sys_timeout().
 *
 * @param arg Additional argument to pass to the function - set up by sys_timeout()
 */


/*
 * Timer callback function that calls tcp_tmr() and reschedules itself.
 *
 * @param arg unused argument
 */
pub fn
tcpip_tcp_timer(arg: &mut Vec<u8>)
{
  

  /* call TCP timer handler */
  tcp_timer_handler();
  /* timer still needed? */
  if tcp_active_pcbs || tcp_tw_pcbs {
    /* restart timer */
    sys_timeout(TCP_TMR_INTERVAL, tcpip_tcp_timer, None);
  } else {
    /* disable timer */
    tcpip_tcp_timer_active = 0;
  }
}

/*
 * Called from TCP_REG when registering a new PCB:
 * the reason is to have the TCP timer only running when
 * there are active (or time-wait) PCBs.
 */
pub fn 
tcp_timer_needed()
{
  LWIP_ASSERT_CORE_LOCKED();

  /* timer is off but needed again? */
  if !tcpip_tcp_timer_active && (tcp_active_pcbs || tcp_tw_pcbs) {
    /* enable and start timer */
    tcpip_tcp_timer_active = 1;
    sys_timeout(TCP_TMR_INTERVAL, tcpip_tcp_timer, None);
  }
}


pub fn sys_timeout_abs(
    abs_time: u64,
    handler: SysTimeoutHandler,
    arg: &mut LwipCyclicTimer,
    handler_name: &String
)
{
  let mut timeout: SystemTimeout = SystemTimeout::new();
  let mut t: SystemTimeout = SystemTimeout::new();
    // TODO:
  // timeout.next = None;
  timeout.h = handler;
  timeout.arg = arg.clone();
  timeout.time = abs_time;
  timeout.handler_name = handler_name.clone();
    // TODO:
  // if (next_timeout == None) {
  //   next_timeout = timeout;
  //   return;
  // }
    // TODO:
  // if (TIME_LESS_THAN(timeout.time, next_timeout.time)) {
  //   timeout.next = next_timeout;
  //   next_timeout = timeout;
  // } else {
  //   // TODO:
  //   // for (t = next_timeout; t != None; t = t.next) {
  //   //   if ((t.next == None) || TIME_LESS_THAN(timeout.time, t.next.time)) {
  //   //     timeout.next = t.next;
  //   //     t.next = timeout;
  //   //     break;
  //   //   }
  //   // }
  // }
}

/*
 *
 *
 * @param arg unused argument
 */

/// Timer callback function that calls cyclic.handler() and reschedules itself.
pub fn lwip_cyclic_timer(arg: &mut LwipCyclicTimer)
{
  let now: u64;
  let next_timeout_time: u64;
 let cyclic: &mut LwipCyclicTimer = arg;
  cyclic.handler();
  now = sys_now();
  next_timeout_time = (current_timeout_due_time + cyclic.interval_ms);  /* overflow handled by TIME_LESS_THAN macro */
  if next_timeout_time < now {
    /* timer would immediately expire again -> "overload" -> restart without any correction */
    sys_timeout_abs((now + cyclic.interval_ms), lwip_cyclic_timer, arg, &cyclic.handler_name);
  } else {
    /* correct cyclic interval with handler execution delay and sys_check_timeouts jitter */
    sys_timeout_abs(next_timeout_time, lwip_cyclic_timer, arg, &cyclic.handler_name);
  }
}

/* Initialize this module */
pub fn  sys_timeouts_init(ctx: &mut LwipContext)
{
  let i: usize;
    for timer in &mut ctx.lwip_cyclic_timers {
        sys_timeout_abs(timer.interval_ms, timer.handler, timer, &timer.handler_name)
    }
}

/*
 * Create a one-shot timer (aka timeout). Timeouts are processed in the
 * following cases:
 * - while waiting for a message using sys_timeouts_mbox_fetch()
 * - by calling sys_check_timeouts() (NO_SYS==1 only)
 *
 * @param msecs time in milliseconds after that the timer should expire
 * @param handler callback function to call when msecs have elapsed
 * @param arg argument to pass to the callback function
 */

pub fn 
sys_timeout_debug(
    msecs: u64,
    handler: SysTimeoutHandler,
    arg: &mut LwipCyclicTimer,
    handler_name: &String
)
{
  let next_timeout_time: u64;
  next_timeout_time = (sys_now() + msecs); /* overflow handled by TIME_LESS_THAN macro */
  sys_timeout_abs(next_timeout_time, handler, arg, handler_name);
}

/*
 * Go through timeout list (for this task only) and remove the first matching
 * entry (subsequent entries remain untouched), even though the timeout has not
 * triggered yet.
 *
 * @param handler callback function that would be called by the timeout
 * @param arg callback argument that would be passed to handler
*/
pub fn 
sys_untimeout(ctx: &mut LwipContext, name: &String)
{
    let mut timer: Option<&LwipCyclicTimer> = None;
    for i in 0..ctx.lwip_cyclic_timers.len() {
        timer = Some(&ctx.lwip_cyclic_timers[i]);
        if timer.handler_name == name {
            break;
        }
    }

    match timer {
        Some(t) => ctx.lwip_cyclic_timers.remove(index)
    }

  let prev_t: &mut SystemTimeout;
  let t: SystemTimeout;

  LWIP_ASSERT_CORE_LOCKED();

  if (next_timeout == None) {
    return;
  }

  // for (t = next_timeout, prev_t = None; t != None; prev_t = t, t = t.next) {
  //   if ((t.h == handler) && (t.arg == arg)) {
  //     /* We have a match */
  //     /* Unlink from previous in list */
  //     if (prev_t == None) {
  //       next_timeout = t.next;
  //     } else {
  //       prev_t.next = t.next;
  //     }
  //     memp_free(MEMP_SYS_TIMEOUT, t);
  //     return;
  //   }
  // }
  return;
}

/*
 * @ingroup lwip_nosys
 * Handle timeouts for NO_SYS==1 (i.e. without using
 * tcpip_thread/sys_timeouts_mbox_fetch(). Uses sys_now() to call timeout
 * handler functions when timeouts expire.
 *
 * Must be called periodically from your main loop.
 */
pub fn 
sys_check_timeouts()
{
  let now: u32;

  LWIP_ASSERT_CORE_LOCKED();

  /* Process only timers expired at the start of the function. */
  now = sys_now();

  loop {
    let mut tmptimeout: &mut SystemTimeout;
    let handler: SysTimeoutHandler;
    let arg: &mut Vec<u8>;

    PBUF_CHECK_FREE_OOSEQ();

    tmptimeout = next_timeout;
    if (tmptimeout == None) {
      return;
    }

    if (TIME_LESS_THAN(now, tmptimeout.time)) {
      return;
    }

    /* Timeout has expired */
    next_timeout = tmptimeout.next;
    handler = tmptimeout.h;
    arg = tmptimeout.arg;
    current_timeout_due_time = tmptimeout.time;

    if (handler != None) {
/*LWIP_DEBUGF(TIMERS_DEBUG, ("sct calling h=%s t=%"U32_F" arg=%p\n",
                                 tmptimeout.handler_name, sys_now() - tmptimeout.time, arg));*/
    }

    memp_free(MEMP_SYS_TIMEOUT, tmptimeout);
    if (handler != None) {
      handler(arg);
    }
    LWIP_TCPIP_THREAD_ALIVE();

    /* Repeat until all expired timers have been called */
  } 
}

/* Rebase the timeout times to the current time.
 * This is necessary if sys_check_timeouts() hasn't been called for a long
 * time (e.g. while saving energy) to prevent all timer functions of that
 * period being called.
 */
pub fn 
sys_restart_timeouts()
{
  let now: u32;
  let base: u32;
  let mut t: &mut SystemTimeout;

  if (next_timeout == None) {
    return;
  }

  now = sys_now();
  base = next_timeout.time;

  // TODO
  // for (t = next_timeout; t != None; t = t.next) {
  //   t.time = (t.time - base) + now;
  // }
}

/* Return the time left before the next timeout is due. If no timeouts are
 * enqueued, returns 0xffffffff
 */
pub fn sys_timeouts_sleeptime() -> u32
{
  let now: u32;

  LWIP_ASSERT_CORE_LOCKED();

  if (next_timeout == None) {
    return SYS_TIMEOUTS_SLEEPTIME_INFINITE;
  }
  now = sys_now();
  if (TIME_LESS_THAN(next_timeout.time, now)) {
    return 0;
  } else {
    let ret: u32 = (next_timeout.time - now);
    LWIP_ASSERT("invalid sleeptime", ret <= LWIP_MAX_TIMEOUT);
    return ret;
  }
}

 /* LWIP_TIMERS && !LWIP_TIMERS_CUSTOM */
/* Satisfy the TCP code which calls this function */
pub fn 
tcp_timer_needed()
{
}

/* This array contains all stack-internal cyclic timers. To get the number of
 * timers, use lwip_num_cyclic_timers */
// TODO: include in context
// extern const struct LwipCyclicTimer lwip_cyclic_timers[];

/* The one and only timeout list */
// static next_timeout: &mut sys_timeo;

// static current_timeout_due_time: u32;

/* global variable that shows if the tcp timer is currently scheduled or not */
// static tcpip_tcp_timer_active: i32;
