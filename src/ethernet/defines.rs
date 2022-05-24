use crate::core::mac_address::MacAddress;
use crate::ethernet::ether_type::EtherType;
use crate::ethernet::mcast_addr::EthernetMulticastAddress;

pub const LWIP_ARP_FILTER_NETIF: u32 = 0;
pub const ETH_HWADDR_LEN: usize = 6;
pub const ETHARP_HWADDR_LEN: usize = ETH_HWADDR_LEN;
pub const ETH_PAD_SIZE: usize = 0;

pub const IPv4_MULTICAST_MAC_ADDR_OUI: [u8;3] = [0x01,0x00,0x5e];
pub const IPv6_MULTICAST_MAC_ADDR_OUI: [u8;3] = [0x33,0x33,0x00];

pub const ETH_HDR_LEN_NO_VLAN: usize = 14;
pub const ETH_HDR_LEN_VLAN: usize = 18;
pub const ETH_HDR_LEN_STAG: usize = 22;

pub const STP_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x0]), end_address: None, ethertype: EtherType::LengthFieldMax };
pub const LLDP_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x0]), end_address: None, ethertype: EtherType::LLDP };
pub const ETH_FLOW_CTRL_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x1]), end_address: None,ethertype: EtherType::MAC_Ctrl };
pub const SLOW_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x2]), end_address: None,ethertype: EtherType::Slow };
pub const PORT_AUTH_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x3]), end_address: None,ethertype: EtherType::IEEE_802_1_X };
pub const LLDP_ETHER_MAC_2: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x3]), end_address: None,ethertype: EtherType::LLDP };
pub const STP_PROV_BRDG_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x8]), end_address: None,ethertype: EtherType::LengthFieldMax };
pub const MVRP_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0xD]), end_address: None,ethertype: EtherType::MVRP };
pub const LLDP_ETHER_MAC_3: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0xE]), end_address: None,ethertype: EtherType::LLDP };
pub const PTP2_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0xe]), end_address: None,ethertype: EtherType::IEEE_1588_1 };
pub const MVRP_GARP_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x21]), end_address: None,ethertype: EtherType::MVRP };
pub const CFM_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x30]), end_address: Some(MacAddress::from([0x01,0x80,0xc2,0x0,0x0,0x3f])), ethertype: EtherType::ConnectivityFaultMgmt };
pub const PTP_ADV_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x1b,0x19,0x0,0x0,0x0]), end_address: None, ethertype: EtherType::IEEE_1588_1 };
pub const IP4_MCAST_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{ start_address: MacAddress::from([0x01,0x00,0x5e,0x0,0x0,0x0]), end_address: Some(MacAddress::from([0x01,0x00,0x5e,0x7f,0xff,0xff])), ethertype: EtherType::IPv4 };
pub const IP6_MCAST_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{
    start_address: MacAddress::from([0x33,0x33,0x0,0x0,0x0]),
    end_address: Some(MacAddress::from([0x33,0x33,0xff,0xff,0xff,0xff])),
    ethertype: EtherType::IPv6
};
// IEC 61850-8-1 GOOSE Type 1/1A, 0x88B8, 01-0C-CD-01-00-00 to 01-0C-CD-01-01-FF
// GSSE IEC 61850-8-1, 0x88B9, 01-0C-CD-02-00-00 to 01-0C-CD-02-01-FF
// IEC 61850 8-1 Multicast Sampled Values, 0x88BA, 01-0C-CD-04-00-00 to 01-0C-CD-04-01-FF
pub const CISCO_CDP_VTP_UDLD_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{
    start_address: MacAddress::from([0x01,0x00,0x0c,0xcc,0xcc,0xcc]),
    end_address: None,
    ethertype: EtherType::LengthFieldMax
};
pub const CISCO_SSTPA_ETHER_MAC: EthernetMulticastAddress = EthernetMulticastAddress{
    start_address: MacAddress::from([0x01,0x00,0x0c,0xcc,0xcc,0xcd]),
    end_address: None,
    ethertype: EtherType::LengthFieldMax
};

