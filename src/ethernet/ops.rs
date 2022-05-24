use core::mem::size_of;
use lib_lwip_rs::core::mac_address::MacAddress;
use crate::core::context::LwipContext;
use crate::core::errors::{LwipError, LwipErrorCode};
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
        EtherType::NotSet => return Err(LwipError::new(LwipErrorCode::InvalidData, "ethertype field not set")),
        EtherType::IPv4 => {
            todo!()
        }
        EtherType::ARP => {
            todo!()
        }
        EtherType::VLAN_C_TAG => {
            todo!()
        }
        EtherType::VLAN_S_TAG => unimplemented!(),
        EtherType::IPv6 => {
            todo!()
        }
        EtherType::LLDP => unimplemented!(),
        EtherType::TRILL => unimplemented!(),
        EtherType::L2_IS_IS => unimplemented!(),
        EtherType::RARP => unimplemented!(),
        EtherType::PPP => unimplemented!(),
        EtherType::MPLS => unimplemented!(),
        EtherType::MPLS_UPSTREAM_LABEL => unimplemented!(),
        EtherType::PPPoE_Discovery => unimplemented!(),
        EtherType::PPPoE_Session => unimplemented!(),
        EtherType::IEEE_802_1_X => unimplemented!(),
        EtherType::MACSEC => unimplemented!(),
        EtherType::PBB_INST_TAG => unimplemented!(),
        EtherType::MVRP => unimplemented!(),
        EtherType::MMRP => unimplemented!(),
        EtherType::IEEE_802_11_R => unimplemented!(),
        EtherType::TRILL_FGL => unimplemented!(),
        EtherType::TRILL_BR => unimplemented!(),
        EtherType::LOWPAN_ENCAP => unimplemented!(),
        EtherType::GRE_X => unimplemented!(),
        EtherType::HD_BASE_T_CMP => unimplemented!(),
        EtherType::RDMAoE => unimplemented!(),
        EtherType::IEEE_1722_2016 => unimplemented!(),
        EtherType::MAC_Ctrl => unimplemented!(),
        EtherType::LocalExperimental2 => unimplemented!(),
        EtherType::SRP => unimplemented!(),
        EtherType::ECP => unimplemented!(),
        EtherType::MDCP => unimplemented!(),
        EtherType::ROHC => unimplemented!(),
        EtherType::IEEE_1588_1 => unimplemented!(),
        EtherType::LocalExperimental1 => unimplemented!(),
        EtherType::FlowFilteringTag => unimplemented!(),
        EtherType::MIRP => unimplemented!(),
        EtherType::IEEE_80211_MP => unimplemented!(),
        EtherType::LLC_Encap => unimplemented!(),
        EtherType::RoCE => unimplemented!(),
        EtherType::MRP => unimplemented!(),
        EtherType::IPX => unimplemented!(),
        EtherType::IPX2 => unimplemented!(),
        EtherType::EoIB => unimplemented!(),
        EtherType::FCoE => unimplemented!(),
        EtherType::BCN => unimplemented!(),
        EtherType::RTAG => unimplemented!(),
        EtherType::CNM => unimplemented!(),
        EtherType::CN_TAG => unimplemented!(),
        EtherType::DRCP => unimplemented!(),
        EtherType::RSNA_PRE_AUTH => unimplemented!(),
        EtherType::SCSIoE => unimplemented!(),
        EtherType::DCE => unimplemented!(),
        EtherType::T_TAG => unimplemented!(),
        EtherType::VLC => unimplemented!(),
        EtherType::MIS => unimplemented!(),
        EtherType::MPLS_ENCAP => unimplemented!(),
        EtherType::sFlow => unimplemented!(),
        EtherType::LWAAP => unimplemented!(),
        EtherType::GFP => unimplemented!(),
        EtherType::Slow => unimplemented!(),
        EtherType::BridgePortExt => unimplemented!(),
        EtherType::MacStatus => unimplemented!(),
        EtherType::CongestionIsolationMsg => unimplemented!(),
        EtherType::ConnectivityFaultMgmt => unimplemented!(),
        EtherType::RIST => unimplemented!(),
        _ => {
            return Err(LwipError::new(LwipErrorCode::InvalidOperation, "unhandled EtherTYpe: {}".format()))
        }
    }
    }

    Ok(())

}

/// fix up ethernet header for packet, find correct netif, and queue in netif's tx buf
pub fn ether_send(ctx: &mut LwipContext, pkt: &mut PacketBuffer) -> Result<(), LwipError> {
    todo!()
}


