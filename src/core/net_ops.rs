use crate::core::defines::LwipAddr;
use crate::core::error::LwipError;
use crate::tcp::tcpbase_h::TcpState;

pub type AcceptFunc = fn(arg: &mut Vec<u8>, new_conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type ConnectedCallbackFun = fn(arg: &mut Vec<u8>, conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type SentFunc = fn(arg: &mut Vec<u8>, conn: &mut Vec<u8>, len: usize) -> Result<(), LwipError>;
pub type ErrFunc = fn(arg: &mut Vec<u8>) -> Result<(), LwipError>;
pub type NewFunc = fn(arg: &mut Vec<u8>, ip_type: u8) -> Result<Vec<u8>, LwipError>;
pub type SetPollFunc = fn(conn: &mut Vec<u8>, interval: u64);
pub type RecvFunc = fn(conn: &mut Vec<u8>, len: usize);
pub type BindFunc = fn(conn: &mut Vec<u8>, addr: &mut LwipAddr, port: u16) -> Result<(), LwipError>;
pub type ConnectFunc = fn(conn: &mut Vec<u8>, addr: &mut LwipAddr, port:u16, connected_cb: ConnectedCallbackFun) -> Result<(), LwipError>;
pub type ListenFunc = fn(conn: &mut Vec<u8>, backlog: usize) -> Result<(), LwipError>;
pub type AbortFunc = fn(conn: &mut Vec<u8>);
pub type CloseFunc = fn(conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type ShutdownFunc = fn(conn: &mut Vec<u8>, shut_tx: i32, shut_rx: i32) -> Result<(), LwipError>;
pub type WriteFunc = fn(conn: &mut Vec<u8>, dataptr: &Vec<u8>, len: usize, flags: u32) -> Result<(), LwipError>;
pub type OutputFunc = fn(conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type MssFunc = fn(conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type SndbufFunc = fn(conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type SndqueuelenFunc = fn(conn: &mut Vec<u8>) -> Result<(), LwipError>;
pub type NagleDisableFunc = fn(conn: &mut Vec<u8>);
pub type NagleEnableFunc = fn(conn: &mut Vec<u8>);
pub type NagleDisabledFunc = fn(conn: &mut Vec<u8>) -> Result<bool, LwipError>;
pub type SetPrioFunc = fn(conn: &mut Vec<u8>, prio: u8);
pub type DeallocFunc = fn(conn: &mut Vec<u8>);
pub type GetTcpAddrInfoFunc = fn(conn: &mut Vec<u8>, local: i32, addr: &mut LwipAddr, port: &mut u16) -> Result<(), LwipError>;
pub type GetIpFunc = fn(conn: &mut Vec<u8>, local: i32) -> Result<LwipAddr, LwipError>;
pub type GetPortFunc = fn(conn: &mut Vec<u8>, local: i32) -> Result<u16, LwipError>;
pub type GetTcpStateFunc = fn(conn: &mut Vec<u8>) -> Result<TcpState, LwipError>;

#[derive(Default, Debug, Clone)]
pub struct NetOperations {
    pub accept: Option<AcceptFunc>,
    pub connected: Option<ConnectedCallbackFun>,
    pub sent: Option<SentFunc>,
    pub err: Option<ErrFunc>,
    pub new: Option<NewFunc>,
    pub set_poll: Option<SetPollFunc>,
    pub received: Option<RecvFunc>,
    pub bind: Option<BindFunc>,
    pub connect: Option<ConnectFunc>,
    pub listen: Option<ListenFunc>,
    pub abort: Option<AbortFunc>,
    pub close: Option<CloseFunc>,
    pub shutdown: Option<ShutdownFunc>,
    pub write: Option<WriteFunc>,
    pub output: Option<OutputFunc>,
    pub mss: Option<MssFunc>,
    pub sndbuf: Option<SndbufFunc>,
    pub sndqueuelen: Option<SndqueuelenFunc>,
    pub nagle_disable: Option<NagleDisableFunc>,
    pub nagle_enable: Option<NagleEnableFunc>,
    pub nagle_disabled: Option<NagleDisabledFunc>,
    pub setprio: Option<SetPrioFunc>,
    pub dealloc: Option<DeallocFunc>,
    pub get_tcp_addrinfo: Option<GetTcpAddrInfoFunc>,
    pub get_ip: Option<GetIpFunc>,
    pub get_port: Option<GetPortFunc>,
    pub get_tcp_state: Option<GetTcpStateFunc>,
}

impl NetOperations {
    pub fn new() -> NetOperations {
        NetOperations{
            ..Default::default()
        }
    }
}
