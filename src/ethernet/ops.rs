use core::mem::size_of;
use lib_lwip_rs::core::mac_address::MacAddress;
use crate::core::context::LwipContext;
use crate::core::errors::LwipError;
use crate::core::packet_buffer::PacketBuffer;
use crate::ethernet::defines::ETH_HDR_LEN_NO_VLAN;
use crate::ethernet::hdr::EthernetHeader;
use crate::netif::netif::NetworkInterface;

/// Examine received packet, parse Ethernet Header, set type and offset of next layer
pub fn ether_process_rx(pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    let eth_hdr = EthernetHeader::from(pkt.payload[0..ETH_HDR_LEN_NO_VLAN]);
    match EtherType::from(eth_hdr.ether_type) {

    }
}

/// fix up ethernet header for packet, find correct netif, and queue in netif's tx buf
pub fn ether_send(ctx: &mut LwipContext, pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    todo!()
}


