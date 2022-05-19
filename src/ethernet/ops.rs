use core::mem::size_of;
use lib_lwip_rs::core::mac_address::MacAddress;
use crate::core::context::LwipContext;
use crate::core::errors::LwipError;
use crate::core::packet_buffer::PacketBuffer;
use crate::netif::netif::NetworkInterface;

/// Examine received packet, parse Ethernet Header, set type and offset of next layer
pub fn ether_recv(ctx: &mut LwipContext, netif: &mut NetworkInterface, pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    todo!()
}

/// fix up ethernet header for packet, find correct netif, and queue in netif's tx buf
pub fn ether_send(ctx: &mut LwipContext, pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    todo!()
}
