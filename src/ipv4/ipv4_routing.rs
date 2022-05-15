use crate::errors::{LwipError, LwipErrorCode};
use crate::ipv4::ipv4_address::{ipv4_addr_is_link_local, Ipv4Address};
use crate::ipv4::ipv4_hdr::{IPV4_DONT_FRAG_FLAG, Ipv4Header};
use crate::netif::netif::NetworkInterface;
use crate::packet_buffer::PacketBuffer;

/// check routing stuff to see if a source-based routing policy exists and spit out the appropriate network interface on which to transmit the packet
pub fn get_netif_for_ip4_src_rte(src: &Ipv4Address, dst: &Ipv4Address) -> Result<&NetworkInterface, LwipError> {
    todo!()
}

/// find the appropriate netif for a given ip address. Search the list of network interfaces, until a match is found, e.g. the masked IP address of the network interface equals the masked IP address given to the function
/// TODO handle multicast routes as well
pub fn get_netif_for_ip4_dest(dst: &Ipv4Address) -> Result<&NetworkInterface, LwipError> {
    // TODO: iterate over network interfaces and routing tables looking for the correct route
    // TODO: handle loopback addresses

    todo!()
}

pub fn ipv4_can_forward(pkt: &PacketBuffer) -> bool {
    // TODO: dont route link-layer broadcasts
    // TODO: down route link-layer multicasts
    // TODO: dont forward experimental packets
    // TODO: dont route loopback packets
    todo!()
}

pub fn get_dst_netif_for_ip4_pkt(ctx: &mut LwipContext, ip_hdr: &Ipv4Header) -> Result<&NetworkInterface, LwipError> {
    todo!()
}

pub fn ipv4_forward(pkt: &mut PacketBuffer, ip_hdr: &mut Ipv4Header, rx_netif: &NetworkInterface) -> Result<(), LwipError> {
    let dst_ip_addr = Ipv4Address::from_u32(ip_hdr.dst_addr);

    if !ipv4_can_forward(pkt) {
        return Err(LwipError::new(LwipErrorCode::InvalidData, "cant forward packet"));
    }

    // do not forward link-local addresses
    if ipv4_addr_is_link_local(&dst_ip_addr) {
        return Err(LwipError::new(LwipErrorCode::InvalidData, "cant forward link layer packet"));
    }

    let dst_netif = get_dst_netif_for_ip4_pkt(ctx, ip_hdr)?;

    // do not forward packets to original interface
    if dst_netif == rx_netif {
        return Err(LwipError::new(LwipErrorCode::InvalidOperation, "cant forward packet to src interface"));
    }

    // decrement the packets TTL
    ip_hdr.ttl -= 1;

    // send ICMP packet if TTL == 0
    if ip_hdr.ttl == 0 {
        // dont send ICMP messages in response to ICMP messages
        if ip_hdr.proto != IP_PROTO_ICMP {
            icmp_time_exceeded(pkt, ICMP_TE_TTL)
        }
        return Err(LwipError::new(LwipErrorCode::InvalidOperation, "TTL of packet is zero"));
    }

    // if checksum offload is enabled then set the checksum to zero
    if dst_netif.offload_ipv4_checksum {
        ip_hdr.checksum = 0;
    } else {
        // update the IP checksum
        if ip_hdr.checksum >= (0xffffu16 - 0x100u16).to_le() {
            ip_hdr.checksum = ip_hdr.checksum + ((0x100u16).to_le() + 1);
        } else {
            ip_hdr.checksum = ip_hdr.checksum + ((0x100u16).to_le());
        }
    }

    // perform fragmentation of ip packet as needed
    if dst_netif.mtu > 0 && (pkt.tot_len > (dst_netif.mtu as usize)) {
        if ip_hdr.frag_off & IPV4_DONT_FRAG_FLAG == 0 {
            ipv4_fragment(pkt, dst_netif, )
        }
    }

    // TODO: update SNMP/normal stats

    Ok(())
}
