use std::convert::TryFrom;
use crate::arp::ops::etharp_input;
use crate::core::error::LwipError;
use crate::core::error::LwipErrorCodes::{ERR_BUF, ERR_INVALID_VAL};
use crate::ethernet::defs::{EthernetHeader, ETH_HDR_LEN, EthernetVlanHeader, ETH_VLAN_HDR_LEN, MacAddress, ETH_HWADDR_LEN};
use crate::ethernet::ether_types::EtherType;
use crate::ip::ip42::ip4_input;
use crate::ip::ip62::ip6_input;
use crate::netif::defs::{NETIF_FLAG_ETHARP, NetworkInterface};
use crate::packetbuffer::pbuf::{pbuf_add_header, pbuf_free, pbuf_remove_header};
use crate::packetbuffer::pbuf_h::{PacketBuffer, PacketBufferContentType, PacketBufferLayer, PBUF_FLAG_LLBCAST, PBUF_FLAG_LLMCAST};
use log::{debug,error,log_enabled,info, Level};
use crate::ethernet::multicast_addresses::mac_address_is_multicast;
use crate::nd6::nd62::nd6_get_next_hop_addr_or_queue;

pub fn ethernet_input(pkt_buf: &mut PacketBuffer, netif: &mut NetworkInterface) -> Result<(), LwipError> {
    let mut next_hdr_offset: isize = ETH_HDR_LEN as isize;

    if pkt_buf.len <= ETH_HDR_LEN {
        // TODO: increment error and drop packet counts
        return Err(LwipError::new(ERR_INVALID_VAL, "ethernet header in buffer less than standard header length"));
    }

    if pkt_buf.netif_id == -1 {
        pkt_buf.netif_id = netif.id;
    }

    //  points to packet payload, which starts with an Ethernet header 
    let eth_hdr = EthernetHeader::from_slice(&pkt_buf.buffer);

    debug!("ethernet_input: {:#?}", &eth_hdr);

    let eth_type = eth_hdr.ether_type.into();
    if eth_type == EtherType::Vlan {
            let vlan: EthernetVlanHeader = EthernetVlanHeader::from_slice(&pkt_buf.buffer[ETH_HDR_LEN..]);
            next_hdr_offset = (ETH_HDR_LEN + ETH_VLAN_HDR_LEN) as isize;
            if pkt_buf.len <= ETH_HDR_LEN + ETH_VLAN_HDR_LEN {
                // TODO: increment error and drop packet counts
                //  a packet with only an ethernet/vlan header (or less) is not valid for us
                return Err(LwipError::new(ERR_INVALID_VAL, "packet not long enough"))
            }

            // TODO: check if the packet's VLAN ID matches the one our port is tagged for, if it is continue; if not, then ignore the packet
        }

    // TODO: LWIP_ARP_FILTER_NETIF_FN(pkt_buf, netif, lwip_htons(eth_type));
    let dest_addr = MacAddress::from_slice(&eth_hdr.dst_addr);
    if dest_addr.is_multicast() {
        if mac_address_is_multicast(&eth_hdr.dst_addr) {
            pkt_buf.flags |= PBUF_FLAG_LLMCAST;
        }
    }
    let etype =  EtherType::try_from(eth_hdr.ether_type)?;
    match etype {
        //  IP packet? 
        EtherType::IPv4 => {
            pkt_buf.contents_map.push(PacketBufferLayer{offset: next_hdr_offset, content_type: PacketBufferContentType::Ipv4});
            ip4_input(pkt_buf, netif);
        }
        EtherType::ARP => {
            pkt_buf.contents_map.push(PacketBufferLayer{offset: next_hdr_offset, content_type: PacketBufferContentType::Arp});
            etharp_input(pkt_buf, netif);
        }
        EtherType::PppoeDisc => {
            //  PPP Over Ethernet Discovery Stage
            pkt_buf.contents_map.push(PacketBufferLayer{offset: next_hdr_offset, content_type: PacketBufferContentType::PppoeDisc});
            pppoe_disc_input(netif, pkt_buf);
        }
        EtherType::PppoeSession => {
            //  PPP Over Ethernet Session Stage
            pkt_buf.contents_map.push(PacketBufferLayer{offset: next_hdr_offset, content_type: PacketBufferContentType::PppoeSession});
            pppoe_data_input(netif, pkt_buf);
        }
        EtherType::Ipv6  => {
            pkt_buf.contents_map.push(PacketBufferLayer{offset: next_hdr_offset, content_type: PacketBufferContentType::Ipv6});
            ip6_input(pkt_buf, netif);

        }
        _ => {
            return Err(LwipError::new(ERR_INVALID_VAL, &*format!("unhandled proto: {:?}", etype)))
        }
    }

    // This means the pbuf is freed or consumed, so the caller doesn't have to free it again
    pbuf_free(pkt_buf);
    return Ok(());
}

pub fn ethernet_output(
    netif: &mut NetworkInterface,
    p: &mut PacketBuffer,
    src: &MacAddress,
    dst: &MacAddress,
    ether_type: EtherType,
    vlan_hdr: Option<&EthernetVlanHeader>,
) -> Result<(), LwipError>{
    let mut out_eth_hdr: EthernetHeader = EthernetHeader::new();
    let mut eth_type_be: u16 = lwip_htons(ether_type);

    if ether_type == EtherType::Vlan {
    
    }


    let vlan_prio_vid = LWIP_HOOK_VLAN_SET(netif, p, src, dst, &ether_type);
    if vlan_prio_vid >= 0 {
        let mut vlanhdr: &mut eth_vlan_hdr;

        LWIP_ASSERT("prio_vid must be <= 0xFFFF", vlan_prio_vid <= 0xFFFF);

        if pbuf_add_header(p, SIZEOF_ETH_HDR + SIZEOF_VLAN_HDR) != 0 {
            // goto pbuf_header_failed;
        }
        vlanhdr = ((p.payload) + SIZEOF_ETH_HDR);
        vlanhdr.tpid = eth_type_be;
        vlanhdr.prio_vid = lwip_htons(vlan_prio_vid);

        eth_type_be = PP_HTONS(ETHTYPE_VLAN);
    } else {
        if pbuf_add_header(p, SIZEOF_ETH_HDR) != 0 {
            // goto pbuf_header_failed;
        }
    }

    ethhdr = p.payload;
    ethhdr.ether_type = eth_type_be;
    SMEMCPY(&ethhdr.dest, dst, ETH_HWADDR_LEN);
    SMEMCPY(&ethhdr.src, src, ETH_HWADDR_LEN);

    //  send the packet 
    return netif.linkoutput(netif, p);
}


pub fn eth_addr_cmp(addr1: &[u8; 6], addr2: &[u8; 6]) -> bool {
    addr1 == addr2
}

pub fn ethip6_output(netif: &mut NetworkInterface, q: &mut PacketBuffer, ip6addr: &mut ip6_addr_t) -> Result<(), LwipError> {
    let mut dest: MacAddress = MacAddress::default();
    let mut hwaddr: MacAddress = MacAddress::default();
    let result: err_t;

    //  The destination IP address must be properly zoned from here on down.
    IP6_ADDR_ZONECHECK_NETIF(ip6addr, netif);

    //  multicast destination IP address?
    if ip6_addr_ismulticastip6addr {
        //  Hash IP multicast address to MAC address.
        dest.addr[0] = 0x33;
        dest.addr[1] = 0x33;
        dest.addr[2] = (&(ip6addr.addr[3]))[0];
        dest.addr[3] = (&(ip6addr.addr[3]))[1];
        dest.addr[4] = (&(ip6addr.addr[3]))[2];
        dest.addr[5] = (&(ip6addr.addr[3]))[3];

        //  Send out.
        return ethernet_output(netif, q, &netif.hwaddr, &dest, ETHTYPE_IPV6, None);
    }

    //  Ask ND6 what to do with the packet.
    result = nd6_get_next_hop_addr_or_queue(netif, q, ip6addr, &hwaddr);
    if (result != ERR_OK) {
        return result;
    }

    //  If no hardware address is returned, nd6 has queued the packet for later.
    if (hwaddr == None) {
       return Ok(());
    }

    //  Send out the packet using the returned hardware address.
    SMEMCPY(dest.addr, hwaddr, 6);
    return ethernet_output(netif, q, &netif.hwaddr, &dest, ETHTYPE_IPV6, None);
}
