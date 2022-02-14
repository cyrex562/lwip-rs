use crate::altcp::callbacks::AltcpNewFn;

pub struct AltcpAllocatorT {
    alloc: AltcpNewFn,
    arg: Vec<u8>,
}
