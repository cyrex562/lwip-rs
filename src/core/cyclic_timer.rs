use crate::core::timeouts::LwipCyclicTimerHandler;

pub struct LwipCyclicTimer {
    //! This struct contains information about a stack-internal timer function
    //! that has to be called at a defined interval
    pub interval_ms: u64,
    pub handler: LwipCyclicTimerHandler,
    pub handler_name: String,
}

// typedef void (* LwipCyclicTimerHandler)();
type LwipCyclicTimerHandler = fn();
