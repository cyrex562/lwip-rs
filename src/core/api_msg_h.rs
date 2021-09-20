use crate::core::api_h::{NetConnDesc, netvector};
use crate::core::err_h::LwipError;
use crate::defines::LwipAddr;

/* Note: Netconn API is always available when sockets are enabled -
 * sockets are implemented on top of them */

// TODO:
// #define API_MSG_M_DEF_SEM(m)  *m
// #else
// #define API_MSG_M_DEF_SEM(m)  API_MSG_M_DEF(m)

// #else /* LWIP_MPU_COMPATIBLE */
// #define API_MSG_M_DEF_SEM(m)  API_MSG_M_DEF(m)

/* For the netconn API, these values are use as a bitmask! */
// #define NETCONN_SHUT_RD   1
pub const NETCONN_SHUT_RD: u32 = 1;
// #define NETCONN_SHUT_WR   2
pub const NETCONN_SHUT_WR: u32 = 2;
// #define NETCONN_SHUT_RDWR (NETCONN_SHUT_RD | NETCONN_SHUT_WR)
pub const NETCONN_SHUT_RWDR: u32 = NETCONN_SHUT_RD | NETCONN_SHUT_WR;

pub struct ApiMessageN {
    proto: u8,
}

pub struct ApiMessageBc {
    // API_MSG_M_DEF_C(LwipAddr, ipaddr),
    ipaddr: LwipAddr,
    port: u16,
    if_idx: u8,
}

pub struct ApiMessageAd {
    // API_MSG_M_DEF(ipaddr): LwipAddr,
    ipaddr: LwipAddr,
    // API_MSG_M_DEF: u16(port),
    port: u16,
    local: u8,
}

pub struct ApiMessageW {
    /* current vector to write */
    vector: netvector,
    /* number of unwritten vectors */
    vector_cnt: u16,
    /* offset into current vector */
    vector_off: usize,
    /* total length across vectors */
    len: usize,
    /* offset into total length/output of bytes written when err == ERR_OK */
    offset: usize,
    apiflags: u8,

    time_started: u32,
}

pub struct ApiMessageR {
    len: usize,
}

pub struct ApiMsgSd {
    shut: u8,

    time_started: u32,
    // #else /* LWIP_SO_SNDTIMEO || LWIP_SO_LINGER */
    polls_left: u8,
}

pub struct ApiMessageJl {
    // API_MSG_M_DEF_C(LwipAddr, multiaddr),
    multiaddr: LwipAddr,
    // API_MSG_M_DEF_C(LwipAddr, netif_addr),
    netif_addr: LwipAddr,
    if_idx: u8,
    join_or_leave: netconn_igmp,
}

pub struct ApiMessageLb {
    backlog: u8,
}

/* IP addresses and port numbers are expected to be in
 * the same byte order as in the corresponding pcb.
 */
/* This struct includes everything that is necessary to execute a function
for a netconn in another thread context (mainly used to process netconns
in the tcpip_thread context to be thread safe). */
pub struct ApiMessage {
    /* The netconn which to process - always needed: it includes the semaphore
    which is used to block the application thread until the function finished. */
    pub conn: NetConnDesc,
    /* The return value of the function executed in tcpip_thread. */
    pub err: LwipError,
    /* Depending on the executed function, one of these union members is used */
    /* used for lwip_netconn_do_send */
    pub b: netbuf,
    /* used for lwip_netconn_do_newconn */
    pub n: ApiMessageN,
    /* used for lwip_netconn_do_bind and lwip_netconn_do_connect */
    pub bc: ApiMessageBc,
    /* used for lwip_netconn_do_getaddr */
    pub ad: ApiMessageAd,
    /* used for lwip_netconn_do_write */
    pub w: ApiMessageW,
    /* used for lwip_netconn_do_recv */
    pub r: ApiMessageR,
    /* used for lwip_netconn_do_close (/shutdown) */
    pub sd: ApiMsgSd,
    /* used for lwip_netconn_do_join_leave_group */
    pub jl: ApiMessageJl,
    pub lb: ApiMessageLb,
    pub op_completed_sem: sys_sem_t,
}

// TODO:
// // #define LWIP_API_MSG_SEM(msg)          ((msg).op_completed_sem)
// #else /* LWIP_NETCONN_SEM_PER_THREAD */
// // #define LWIP_API_MSG_SEM(msg)          (&(msg).conn.op_completed)

/* As lwip_netconn_do_gethostbyname requires more arguments but doesn't require a netconn,
it has its own struct (to avoid struct api_msg getting bigger than necessary).
lwip_netconn_do_gethostbyname must be called using tcpip_callback instead of tcpip_apimsg
(see netconn_gethostbyname). */
struct DnsApiMessage {
    /* Hostname to query or dotted IP address string */
    // name: char[DNS_MAX_NAME_LENGTH];
    // #else /* LWIP_MPU_COMPATIBLE */
    pub name: String,
    /* The resolved address is stored here */
    // API_MSG_M_DEF(addr): LwipAddr,
    pub addr: LwipAddr,
    /* Type of resolve call */
    pub dns_addrtype: u8,
    /* This semaphore is posted when the name is resolved, the application thread
    should wait on it. */
    // API_MSG_M_DEF_SEM(sem): sys_sem_t,
    pub sem: sys_sem_t,
    /* Errors are given back here */
    // API_MSG_M_DEF(err): err_t,
    pub err: LwipError,
}

// lwip_netconn_is_deallocated_msg: i32(msg: &mut Vec<u8>);

// lwip_netconn_is_err_msg: i32(msg: &mut Vec<u8>, err: &mut err_t);
// pub fn  lwip_netconn_do_newconn         (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_delconn         (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_bind            (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_bind_if         (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_connect         (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_disconnect      (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_listen          (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_send            (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_recv            (m: &mut Vec<u8>);
//
// pub fn  lwip_netconn_do_accepted        (m: &mut Vec<u8>);
//
// pub fn  lwip_netconn_do_write           (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_getaddr         (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_close           (m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_shutdown        (m: &mut Vec<u8>);
//
// pub fn  lwip_netconn_do_join_leave_group(m: &mut Vec<u8>);
// pub fn  lwip_netconn_do_join_leave_group_netif(m: &mut Vec<u8>);
//
// pub fn  lwip_netconn_do_gethostbyname(arg: &mut Vec<u8>);

// struct netconn* netconn_alloc(enum netconn_type t, netconn_callback callback);
// pub fn  netconn_free(conn: &mut netconn);

/* netifapi related lwIP internal definitions */

// #define NETIFAPI_IPADDR_DEF(type, m)  type m
// #else /* LWIP_MPU_COMPATIBLE */
// #define NETIFAPI_IPADDR_DEF(type, m)  const type * m

// typedef void (*netifapi_void_fn)(netif: &mut NetIfc); type netifapi_void_fn = fn(netif: &mut NetIfc);
// typedef err_t (*netifapi_errt_fn)(netif: &mut NetIfc); type netifapi_errt_fn = fn(netif: &mut NetIfc);

pub struct NetifApiMsgAdd {
    pub ipaddr: LwipAddr,
    pub netmask: LwipAddr,
    pub gw: LwipAddr,
}

pub struct NetifApiMsgCommon {
    pub voidfunc: netifapi_void_fn,
    pub errtfunc: netifapi_errt_fn,
}

pub struct NetifApiMsg {
    pub call: tcpip_api_call_data,
    pub netif: netif,
    pub add: NetifApiMsgAdd,
    pub common: NetifApiMsgCommon,
    pub name: String,
    pub index: u8,
}
