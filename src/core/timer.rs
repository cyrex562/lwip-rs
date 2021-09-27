use crate::core::timeouts::LwipCyclicTimerHandler;
use crate::core::context::LwipContext;

pub struct Timer {
    //! This struct contains information about a stack-internal timer function
    //! that has to be called at a defined interval
    pub interval: i64,
    pub handler: TimerHandler,
    pub name: String,
    pub arg: Vec<u8>,
    pub last: i64,
}

// typedef void (* LwipCyclicTimerHandler)();
type TimerHandler = fn(arg: &mut Vec<u8>);
