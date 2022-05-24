use crate::core::errors::LwipError;

pub mod ethernet;
pub mod core;
pub mod ipv4;
pub mod ipv6;
pub mod netif;
pub mod routing;
pub mod arp;
pub mod low_lvl_if;
pub mod timers;
pub mod ip;


// TODO: set up init
pub fn lwip_init() -> Result<(), LwipError> {
    // todo: configure netifs
    // todo: configure LwipContext
    // todo: configure hosts
    //
    todo!()
}
