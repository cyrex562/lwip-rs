//! Stack-internal timers implementation
//! This file includes timer callbacks for stack-internal timers as well as functions to set up or stop timers and check for expired timers.


use std::convert::TryInto;

use chrono::Duration;
use chrono::prelude::*;

use crate::core::context::LwipContext;
use crate::core::error::{LwipError, LwipErrorCodes};
use crate::core::error::LwipErrorCodes::ERR_INVALID_ARG;
use crate::core::timer::Timer;
use crate::packetbuffer::pbuf_h::PBUF_CHECK_FREE_OOSEQ;
use crate::tcp::tcp2::tcp_timer_handler;
use crate::tcp::tcp_priv_h::TCP_TMR_INTERVAL;

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
pub fn tcpip_tcp_timer(arg: &mut Vec<u8>) {
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
pub fn tcp_timer_needed() {
    /* timer is off but needed again? */
    if !tcpip_tcp_timer_active && (tcp_active_pcbs || tcp_tw_pcbs) {
        /* enable and start timer */
        tcpip_tcp_timer_active = 1;
        sys_timeout(TCP_TMR_INTERVAL, tcpip_tcp_timer, None);
    }
}


pub fn create_timer(
    ctx: &mut LwipContext,
    abs_time: i64,
    handler: TimerHandler,
    handler_name: &String,
    interval: i64,
    handler_arg: &mut Vec<u8>
) -> Result<(), LwipError> {
    let timeout = Timer {
        interval,
        handler,
        name: handler_name.clone(),
        last: abs_time,
        arg: handler_arg.clone(),
    };
    ctx.timers.push(timeout);
    Ok(())
}

/*
 *
 *
 * @param arg unused argument
 */

/// Timer callback function that calls cyclic.handler() and reschedules itself.
// pub fn lwip_cyclic_timer(ctx: &mut LwipContext, arg: &mut Vec<u8>) {
//     let now: u64;
//     let next_timeout_time: i64;
//     let cyclic: &mut Timer = arg;
//     cyclic.handler();
//     now = sys_now();
//     next_timeout_time = (current_timeout_due_time + cyclic.interval);  /* overflow handled by TIME_LESS_THAN macro */
//     if next_timeout_time < now {
//         /* timer would immediately expire again -> "overload" -> restart without any correction */
//         create_timer(ctx, (now + cyclic.interval), lwip_cyclic_timer, arg, &cyclic.name);
//     } else {
//         /* correct cyclic interval with handler execution delay and sys_check_timeouts jitter */
//         create_timer(ctx, next_timeout_time, lwip_cyclic_timer, arg, &cyclic.name);
//     }
// }

/// Initialize this module
pub fn sys_timeouts_init(ctx: &mut LwipContext) -> Result<(), LwipError> {
    unimplemented!()
}


/// Remove a timer from the context
pub fn remove_timer(ctx: &mut LwipContext, name: &String) -> Result<(), LwipError> {
    let mut out_index: isize = -1;
    for index in 0..ctx.timers.len() {
        let timer = Some(&ctx.timers[index]);
        if timer.handler_name == name {
            out_index = index.try_into().unwrap();
            break;
        }
    }

    match out_index {
        -1 => return Err(LwipError::new(LwipErrorCodes::ERR_INVALID_VAL, &format!("timer not found for name {}", name))),
        _ => {
            ctx.timers.remove(out_index as usize);
            Ok(())
        }
    }
}

/// function call to check timers; must be run periodically in the main loop
pub fn sys_check_timeouts(ctx: &mut LwipContext) {
    for timer in &mut ctx.timers {
        let mut now = Utc::now().naive_utc();
        let interval_dur: Duration = Duration::nanoseconds(timer.interval);
        let last_time = NaiveDateTime::from_timestamp(timer.last, 0);
        if (last_time - now) > interval_dur {
            timer.handler();
            timer.last = now.timestamp()
        }
    }
}

/// Reset all timer last time stamps
pub fn restart_timers(ctx: &mut LwipContext) {
    for timer in &mut ctx.timers {
        let now_ts = Utc::now().naive_utc().timestamp();
        timer.last = now_ts;
    }
}

/* Return the time left before the next timeout is due. If no timeouts are
 * enqueued, returns 0xffffffff
 */
pub fn get_next_due_timer(ctx: &mut LwipContext) -> Result<i64, LwipError> {
    if ctx.timers.is_empty() {
        return Err(LwipError::new(ERR_INVALID_ARG, "timers list is empty"));
    }
    let mut next: i64 = i64::MAX;
    for timer in &ctx.timers {
        let now_ts = Utc::now().naive_utc().timestamp();
        let time_left = now_ts - timer.last;
        if time_left < next {
            next = time_left
        }
    }

    Ok(next)
}

// static next_timeout: &mut sys_timeo;
// static current_timeout_due_time: u32;
// static tcpip_tcp_timer_active: i32;
