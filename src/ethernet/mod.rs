// #define IFNAME0 'e'
// #define IFNAME1 'n'

mod defines;
pub mod ether_type;
pub mod vlan;
pub mod hdr;
pub mod ops;
pub mod mcast_addr;

use std::mem::size_of;
use crate::core::mac_address::MacAddress;
use crate::errors::LwipError;
use crate::errors::LwipErrorCode::InvalidArgument;



 /* 0 */
