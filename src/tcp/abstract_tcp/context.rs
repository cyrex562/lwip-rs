use crate::altcp::altcp::altcp_close;
use crate::altcp::altcp_tcp::altcp_tcp_new_ip_type;
use crate::altcp::altcp_tls_mbedtls::altcp_tls_wrap;
use crate::altcp::altcp_tls_mbedtls_structs::AlTcpMbedTlsState;
use crate::core::error::{ERR_VAL, LwipError};
use crate::tcp::tcp2_h::TcpContext;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct AlTcpContext {
    pub tcp_ctx: TcpContext,
    pub al_tcp_proxy_conn_state: AlTcpProxyConnectState,

    // pub functions: AltcpFunctions,
    pub inner_conn_key : u32,
    // TODO: figure out how to handle self-referencing inner struct
    // arg: &mut Vec<u8>;
    // pub arg: Option<T>,
    // state: &mut Vec<u8>;
    pub state: Option<AlTcpMbedTlsState>,
    //  application callbacks 
    // AltcpAcceptFn     accept;
    // pub accept: Option<AltcpAcceptFn>,
    // AltcpConnectedFn  connected;
    // pub connected: Option<AltcpConnectedFn>,
    // AltcpRecvFn       recv;
    // pub recv: Option<AltcpRecvFn>,
    // AltcpSentFn       sent;
    // pub sent: Option<AltcpSentFn>,
    // AltcpPollFn       poll;
    // pub poll: Option<AltcpPollFn>,
    // AltcpErrFn        err;
    // pub err: Option<AltcpErrFn>,
    // pollinterval: u8;
    pub pollinterval: u64,
}

impl AlTcpContext {
    pub fn new<T>() -> AlTcpContext {
        AlTcpContext {
            inner_conn_key: 0,
            tcp_ctx: TcpContext::new(),
            // functions: NetOperations::new(),
            // arg: None,
            state: some(AlTcpMbedTlsState::new()),
            // accept: None,
            // connected: None,
            // recv: None,
            // sent: None,
            // poll: None,
            // err: None,
            pollinterval: 0,
            al_tcp_proxy_conn_state: ()
        }
    }
}

/// This standard allocator function creates an abstract_tcp pcb for TLS over TCP
pub fn altcp_tls_new(config: &mut altcp_tls_config, ip_type: u8) -> Result<AlTcpContext, LwipError>{
    let mut inner_conn = match altcp_tcp_new_ip_type(ip_type) {
        Ok(x) => x,
        Err(e) => return Err(LwipError::new(ERR_VAL, ""))
    };

    return match altcp_tls_wrap(config, &mut inner_conn) {
        Ok(x) => Ok(x),
        Err(e) => {
            altcp_close(&mut inner_conn);
            Err(e)}
    }
}

pub fn altcp_tls_alloc(arg: &mut Vec<u8>, ip_type: u8) -> Result<AlTcpContext, LwipError> {
    altcp_tls_new(arg, ip_type)
}
