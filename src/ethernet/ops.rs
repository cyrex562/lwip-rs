use core::mem::size_of;
use lib_lwip_rs::core::mac_address::MacAddress;
use crate::core::context::LwipContext;
use crate::core::errors::LwipError;
use crate::core::mac_address::MacAddress;
use crate::core::packet_buffer::PacketBuffer;
use crate::ethernet::defines::ETH_HDR_LEN_NO_VLAN;
use crate::ethernet::ether_type::EtherType;
use crate::ethernet::hdr::EthernetHeader;
use crate::netif::netif::NetworkInterface;

pub fn get_src_mac(buffer: &[u8], hdr_start_off: usize) -> MacAddress {
    MacAddress::from(buffer[hdr_start_off..hdr_start_off +6])
}

pub fn get_dst_mac(buffer: &[u8], hdr_start_off:usize) -> MacAddress {
    MacAddress::from(buffer[hdr_start_off +6..hdr_start_off +12])
}

pub fn get_ether_type(buffer: &[u8], hdr_start_off: usize) -> EtherType {

}

/// Examine received packet, parse Ethernet Header, set type and offset of next layer
pub fn ether_process_rx(pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    let eth_hdr = EthernetHeader::from(pkt.payload[0..ETH_HDR_LEN_NO_VLAN]);
    if eth_hdr.ether_type <= EtherType::LengthFieldMax as u16 {

    } else {
        let ether_type =  EtherType::try_from(eth_hdr.ether_type)?;
    match ether_type {
        EtherType::NotSet => {}
        EtherType::LengthFieldMax => {}
        EtherType::IPv4 => {

        }
        EtherType::ARP => {}
        EtherType::VLAN_C_TAG => {}
        EtherType::VLAN_S_TAG => {}
        EtherType::IPv6 => {}
        EtherType::LLDP => {}
        EtherType::TRILL => {}
        EtherType::L2_IS_IS => {}
        EtherType::RARP => {}
        EtherType::PPP => {}
        EtherType::MPLS => {}
        EtherType::MPLS_UPSTREAM_LABEL => {}
        EtherType::PPPoE_Discovery => {}
        EtherType::PPPoE_Session => {}
        EtherType::IEEE_802_1_X => {}
        EtherType::MACSEC => {}
        EtherType::PBB_INST_TAG => {}
        EtherType::MVRP => {}
        EtherType::MMRP => {}
        EtherType::IEEE_802_11_R => {}
        EtherType::TRILL_FGL => {}
        EtherType::TRILL_BR => {}
        EtherType::LOWPAN_ENCAP => {}
        EtherType::GRE_X => {}
        EtherType::HD_BASE_T_CMP => {}
        EtherType::RDMAoE => {}
        EtherType::IEEE_1722_2016 => {}
        EtherType::MAC_Ctrl => {}
        EtherType::LocalExperimental2 => {}
        EtherType::SRP => {}
        EtherType::ECP => {}
        EtherType::MDCP => {}
        EtherType::ROHC => {}
        EtherType::IEEE_1588_1 => {}
        EtherType::LocalExperimental1 => {}
        EtherType::FlowFilteringTag => {}
        EtherType::MIRP => {}
        EtherType::IEEE_80211_MP => {}
        EtherType::LLC_Encap => {}
        EtherType::RoCE => {}
        EtherType::MRP => {}
        EtherType::IPX => {}
        EtherType::IPX2 => {}
        EtherType::EoIB => {}
        EtherType::FCoE => {}
        EtherType::BCN => {}
        EtherType::RTAG => {}
        EtherType::CNM => {}
        EtherType::CN_TAG => {}
        EtherType::DRCP => {}
        EtherType::RSNA_PRE_AUTH => {}
        EtherType::SCSIoE => {}
        EtherType::DCE => {}
        EtherType::T_TAG => {}
        EtherType::VLC => {}
        EtherType::MIS => {}
        EtherType::MPLS_ENCAP => {}
        EtherType::sFlow => {}
        EtherType::LWAAP => {}
        EtherType::GFP => {}
        EtherType::Slow => {}
        EtherType::BridgePortExt => {}
        EtherType::MacStatus => {}
        EtherType::CongestionIsolationMsg => {}
        EtherType::ConnectivityFaultMgmt => {}
        EtherType::RIST => {}
    }
    }

    Ok(())

}

/// fix up ethernet header for packet, find correct netif, and queue in netif's tx buf
pub fn ether_send(ctx: &mut LwipContext, pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    todo!()
}


