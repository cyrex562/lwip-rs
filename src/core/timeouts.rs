use std::io::empty;
use crate::core::context::LwipContext;

pub const MAX_TIMEOUT: u32 = 0x7fffffff;
pub const SLEEP_TIME_INFINITE: u32 = 0xffffffff;

pub type SysTimeoutHanlder = fn(&mut LwipContext, arg: &mut Vec<u8>);

#[derive(Debug,Clone,Default)]
pub struct SysTimeo {
    pub time: u64,
    pub h: SysTimeoutHanlder,
    pub arg: Vec<u8>,
    pub handler_name: String,
}

#[derive(Debug,Clone,Default)]
pub struct LwipCyclicHandler {
    pub interval_ms: u32,
    pub handler: LwipCyclicTimerHandler,
    pub handler_name: String
}



/// Timer callback function that calls tcp_tmr and reschedules itself
pub fn tcpip_tcp_timer(ctx: &mut LwipContext, tcp_active_pcbs: &mut Vec<TcpPcb>, tcp_tw_pcbs: &mut Vec<TcpPcb>) {
    tcp_tmr();
    if !tcp_active_pcbs.is_empty() || !tcp_tw_pcbs.is_empty() {
        sys_timeout(ctx.options.tcp_timer_interval, tcpip_tcp_timer, None);
    } else {
        ctx.tcpip_tcp_timer_active = false;
    }
}

/// Called from TCP_REG when registering a new PCB; the reason is to have the TCP timer only running when there are active or time-wait PCBs
pub fn tcp_timer_needed(ctx: &mut LwipContext) {
    if !ctx.tcpip_tcp_timer_active && (!ctx.tcp_active_pcbs.is_empty() || !ctx.tcp_tw_pcbs.is_empty()) {
        ctx.tcpip_tcp_timer_active = true;
        sys_timeout(ctx.options.tcp_timer_interval, tcpip_tcp_timer, None);
    }
}

pub fn sys_timeout_abs(abs_time: u64, handler: SysTimeoutHandler, arg: &mut Vec<u8>, handler_name: &String) {
    let timeout = SysTimeout {
        h: handler,
        arg: arg.to_owned(),
        time: abs_time,
        handler_name: hander_name.clone(),
    };

    // todo: check next timer in a list to see if its less, and swap positions; put timer in the sorted position correct.
}


pub fn lwip_cyclic_timer(arg: &mut Vec<u8>) {
    let mut now = 0i64;
    let mut next_time = 0i64;
    let cyclic: LwipCyclicTimer = LwipCyclicTimer::from(arg);
    // now = get time for now todo:
    // get next time out time todo:
    // todo: next timeout time less than now then put new timer in now + next interval, otherwise put in next slot
}

pub fn sys_timeouts_init(ctx: &mut LwipContext) {
    // todo iterate list of cyclic timers and schedule with sys_timeout
    //  for (i = (LWIP_TCP ? 1 : 0); i < LWIP_ARRAYSIZE(lwip_cyclic_timers); i++) {
    //     /* we have to cast via size_t to get rid of const warning
    //       (this is OK as cyclic_timer() casts back to const* */
    //     sys_timeout(lwip_cyclic_timers[i].interval_ms, lwip_cyclic_timer, LWIP_CONST_CAST(void *, &lwip_cyclic_timers[i]));
    //   }
}

/// Create a one-shot timer. timeouts arew processed while waiting for a message using
/// sys_timeouts_mbox_fetch and by calling sys_check_timeouts;
pub fn sys_timeout_debug(msecs: u32, handler: SysTimeoutHandler, arg: &mut Vec<u8>, handler_name: &String) {
    let mut next_timeout_time: u64 = 0;
    let mut now: u64 = 0; // todo: get current system time (since epoch)
    next_timeout_time =  now + msecs;
    sys_timeout_abs(next_timeout_time, handler, arg, handler_name)
}


/**
 * Go through timeout list (for this task only) and remove the first matching
 * entry (subsequent entries remain untouched), even though the timeout has not
 * triggered yet.
 *
 * @param handler callback function that would be called by the timeout
 * @param arg callback argument that would be passed to handler
*/

/// Go through the timeout list and remove the first matching entry
pub fn sys_untimeout(ctx: &mut LwipContext, handler: SysTimeoutHandler, arg: &mut Vec<u8>) {
    // todo: iterate over timeouts list and remove the first matching entry based on matching the handler
}


pub fn sys_check_timeouts(ctx: &mut LwipContext) {
    let mut now = 0u64;
    // todo: get current epoch time
   // todo: iterate over each timer, check it, and call its callback function if it expired.
}

/// Rebase the timeout times to the current time
pub fn sys_restart_timeouts(ctx: &mut LwipContext)  {
    let mut now: u64 = 0;
    let mut base: u64 = 0;
    let t: &mut SysTimeo;
    // todo get current time for now
    // todo set base to next timeout's time val
    // todo iterate over timers and update them with time = (time -base) + now

}


/// get the time left before the next timeout is due
pub fn sys_timeouts_sleeptime(ctx: &mut LwipContext) -> Option<u64> {
    let mut now = 0u64;
    let mut time_left = 0u64;
    // todo set now to current system time
    // todo iterate over timers, and return the amount of time left before the next timer expires
    Some(time_left)
}
