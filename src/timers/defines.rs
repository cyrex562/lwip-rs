pub const MAX_TIMEOUT: u32 = 0x7fffffff;
pub const SLEEP_TIME_INFINITE: u32 = 0xffffffff;

#[derive(Debug,Clone,Default)]
pub struct SysTimeo {
    pub time: u64,
    pub h: fn(arg: &mut Vec<u8>),
    pub arg: Vec<u8>,
    pub handler_name: String,
}

#[derive(Debug,Clone,Default)]
pub struct LwipCyclicHandler {
    pub interval_ms: u32,
    pub handler: LwipCyclicTimerHandler,
    pub handler_name: String
}

