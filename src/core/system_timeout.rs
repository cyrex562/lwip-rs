use crate::core::cyclic_timer::LwipCyclicTimer;

// typedef void (* SysTimeoutHandler)(arg: &mut Vec<u8>);
pub type SysTimeoutHandler = fn(arg: &mut LwipCyclicTimer);

pub struct SystemTimeout {
    // let mut next: &mut sys_timeo;
    pub time: u64,
    pub h: SysTimeoutHandler,
    pub arg: Vec<u8>,
    pub handler_name: String,
}
