use std::net::SocketAddr;

use crate::core::options::DNS_MAX_NAME_LENGTH;
use crate::sockets::sockets_h::sockaddr_storage;

//  Errors used by the DNS API functions, h_errno can be one of them 
pub const EAI_NONAME: u32 = 200;
pub const EAI_SERVICE: u32 = 201;
pub const EAI_FAIL: u32 = 202;
pub const EAI_MEMORY: u32 = 203;
pub const EAI_FAMILY: u32 = 204;
pub const HOST_NOT_FOUND: u32 = 210;
pub const NO_DATA: u32 = 211;
pub const NO_RECOVERY: u32 = 212;
pub const TRY_AGAIN: u32 = 213;

//  input flags for struct addrinfo 
pub const AI_PASSIVE: u32 = 0x01;
pub const AI_CANONNAME: u32 = 0x02;
pub const AI_NUMERICHOST: u16 = 0x04;
pub const AI_NUMERICSERV: u16 = 0x08;
pub const AI_V4MAPPED: u16 = 0x10;
pub const AI_ALL: u16 = 0x20;
pub const AI_ADDRCONFIG: u16 = 0x40;

pub struct Hostent {
    pub h_name: String, //  Official name of the host. 
    pub h_aliases: Vec<String>, /* A pointer to an array of pointers to alternative host names,
                        terminated by a null pointer. */
    pub h_addrtype: i32, //  Address type. 
    pub h_length: usize, //  The length, in bytes, of the address. 
    pub h_addr_list: Vec<String>, /* A pointer to an array of pointers to network addresses (in
                                                    network byte order) for the host, terminated by a null pointer. */
                         // #define h_addr h_addr_list[0] //  for backward compatibility 
}

pub struct AddrInfo {
    pub ai_flags: i32,       //  Input flags. 
    pub ai_family: i32,      //  Address family of socket. 
    pub ai_socktype: i32,    //  Socket type. 
    pub ai_protocol: i32,    //  Protocol of socket. 
    pub ai_addrlen: usize,   //  Length of socket address. 
    pub ai_addr: SocketAddr, //  Socket address of socket. 
    pub ai_canonname: String, //  Canonical name of service location. 
                             //     struct addrinfo  *ai_next;       //  Pointer to next in list. 
}

pub const NETDB_ELEM_SIZE: usize =
    (sizeof(addrinfo) + sizeof(sockaddr_storage) + DNS_MAX_NAME_LENGTH + 1);
