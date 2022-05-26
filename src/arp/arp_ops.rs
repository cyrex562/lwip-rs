use std::mem::size_of;
use crate::arp::arp_defines::ARP_OP_CODE_REPLY;
use crate::arp::arp_hdr::ArpMessage;
use crate::arp::arp_table::{ARP_AGE_REREQUEST_USED_BROADCAST, ARP_AGE_REREQUEST_USED_UNICAST, ArpState, ArpTable};
use crate::core::context::LwipContext;
use crate::core::errors::{LwipErrorCode, LwipError};
use crate::core::packet_buffer::{PacketBuffer, PayloadMapEntry, PayloadType};
use crate::ethernet::{ether_type::EtherType};
use crate::ethernet::hdr::EthernetHeader;
use crate::ipv4::addr::Ipv4Address;
use crate::netif::netif::NetworkInterface;
use crate::core::mac_address::MacAddress;

/// Respond to an ARP request. On ARP reply add entry to cache. Updates cache with
/// snooped address pairs. Should be called for incoming ARP packets.
pub fn arp_receive(ctx: &mut LwipContext,
                   table: &mut ArpTable,
                   pkt: &mut PacketBuffer,
                   src_netif: &NetworkInterface) -> Result<(), LwipError> {
    let arp_hdr: ArpMessage;
    let ether_hdr: EthernetHeader;
    // check if an entry in the payload map has an offset zero and is a link-layer header
    let result = pkt.payload_contains_header_type(PayloadType::Link);
    if result.is_err() {
        let err = result.err().unwrap();
        if err.code == LwipErrorCode::NotFound {
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "input packet does not contain an ethernet header"));
        } else {
            return Err(LwipError::new(LwipErrorCode::OperationFailed, "payload map entry lookup failed for packet: {}".format(err)));
        }
    }

    let pme = result.unwrap();
    let ether_hdr = EthernetHeader::from_u8_slice(pkt.payload[pme.offset .. pme.offset + pme.length]);
    if ether_hdr.ether_type != EtherType::ARP as u16 {
        return Err(LwipError::new(LwipErrorCode::InvalidArgument, "ethernet frame ethertype is not ARP: {}".format(ether_hdr.ether_type)));
    }
    let arp_msg = ArpMessage::from_u8_vec(pkt.payload.as_ref(), (pme.offset + pme.length) as usize);
    let arp_pme = PayloadMapEntry {
        offset: pme.offset + pme.length,
        length: arp_msg.get_arp_len() as isize,
        ptype: PayloadType::Other,
    };
    pkt.payload_map.push(arp_pme);

    match arp_msg.proto {
        ETHERTYPE_IPV4 => {
            let sender_ip4_addr = Ipv4Address::from_vec(arp_msg.sender_proto_addr.as_ref(), 0);
            let sender_hw_addr = MacAddress::from_vec(arp_msg.sender_hw_addr.as_ref(), 0);
            let tgt_ip4_addr = Ipv4Address::from_vec(arp_msg.target_proto_addr.as_ref(), 0);
            let tgt_hw_addr = MacAddress::from_vec(arp_msg.target_hw_addr.as_ref(), 0);
            let from_us: bool = src_netif.has_ip4_addr2(&sender_ip4_addr);
            let for_us: bool = src_netif.has_ip4_addr2(&tgt_ip4_addr);
            // if !from_us && !for_us {
            //     table.update_entry(ctx, &sender_ip_addr, &sender_hw_addr, false, src_netif.id);
            // }
            table.update_entry(ctx, &sender_ip4_addr, &sender_hw_addr, false, src_netif.id)?;
            match arp_msg.opcode {
                ARP_OP_CODE_REQ => {
                    if for_us && !from_us {
                        // get target ipv4 address for arp request
                        let mut tgt_ip_opt: Option<Ipv4Address> = None;
                        for net in src_netif.ipv4_nets.iter() {
                            if net.addr_in_net(&sender_ip4_addr) {
                                tgt_ip_opt = Some(net.local_address.clone());
                                break;
                            }
                        }
                        let tgt_ip: Ipv4Address = match tgt_ip_opt {
                            Some(x) => x,
                            None => {
                                // if we dont find a local ip, then we cant respond on this interface
                                return Err(LwipError::new(LwipErrorCode::InvalidData, "request arp message is from net that receiving network interface doesnt have an address configured"));
                            }
                        };

                        // send reply to request
                        // arp_send(ctx, src_netif, &src_netif.mac_address, &sender_hw_addr, &src_netif.mac_address, &tgt_ip, &sender_hw_addr, &sender_ip4_addr, ARP_OP_CODE_REPLY)?;
                    }
                }
                ARP_OP_CODE_REPLY => {
                    table.update_entry(ctx, &tgt_ip4_addr, &tgt_hw_addr, false, src_netif.id)?;
                }
                ARP_OP_CODE_NAK => {
                    return Err(LwipError::new(LwipErrorCode::Unimplemented, "unimplemented support for ARP op code NAK"));
                }
                _ => {
                    return Err(LwipError::new(LwipErrorCode::InvalidData, "Unsupported opcode for arp msg: {}".format(arp_msg.opcode)));
                }
            }
        }
        _ => {
            return Err(LwipError::new(LwipErrorCode::Unsupported, "unsupported arp msg proto: {}".format(arp_msg.proto)));
        }
    }

    Ok(())
}

pub fn send_to_arp_tbl_idx(ctx: &mut LwipContext, netif: &NetworkInterface, pkt: &mut PacketBuffer, index: usize) -> Result<(), LwipError> {
    if index >= ctx.arp_table.len() {
        return Err(LwipError::new(LwipErrorCode::InvalidArgument, "specified arp table index {} greater than length of table {}".format(index, ctx.arp_table.len())));
    }

    let entry = ctx.arp_table.get_entry_by_index(index)?;
    if entry.state == ArpState::Stable {
        if entry.ctime >= ARP_AGE_REREQUEST_USED_BROADCAST {
            // if arp_send(netif, entry.ip4_addr, None).is_ok() {
            //     ctx.arp_table.update_entry_state(index, ArpState::StableRerequesting1);
            // }
        } else if entry.ctime >= ARP_AGE_REREQUEST_USED_UNICAST {
            // if arp_send(netif, entry.ip_addr, entry.mac_addr) {
            //     ctx.arp_table.update_entry_state(index, ArpState::StableRerequesting1);
            // }
        }
    }

    // ethernet_send(netif, pkt, &netif.mac_address, &entry.mac_addr)
    todo!()
}

// arp_send(ctx, src_netif, &src_netif.mac_address, &sender_hw_addr, &src_netif.mac_address, &tgt_ip, &sender_hw_addr, &sender_ip4_addr, ARP_OP_CODE_REPLY)?;
pub fn arp_send(ctx: &mut LwipContext, netif: &NetworkInterface, pkt: &mut PacketBuffer, ip4_addr: &Ipv4Address) -> Result<(), LwipError> {


    Ok(())
}

pub fn arp_resolve_addr_for_pkt(netif: &NetworkInterface, pkt: &mut PacketBuffer, ip4_addr: &Ipv4Address) -> Result<(), LwipError> {
    let mut is_broadcast = false;
    for net in netif.ipv4_nets.iter() {
        if net.broadcast_addr == ip4_addr {
            is_broadcast = true;
            break;
        }
    }
    Ok(())
}
