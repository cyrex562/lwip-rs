use std::net::Ipv4Addr;
use crate::ip::defs::{Ipv4Address, Ipv4AddressRange, Ipv6Address, Ipv6AddressRange};

// from https://en.wikipedia.org/wiki/Multicast_address

pub const LOCAL_SUBNET_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange {
    start: Ipv4Address::from([224,0,0,0]),
    end: Ipv4Address::from([224,0,0,255])
};

pub const INTERNET_CTRL_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange {
    start: Ipv4Address::from([224,0,1,0]),
    end: Ipv4Address::from([224,0,1,255])
};

pub const AD_HOC_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [224,0,2,0], [224,0,255,255]
);

pub const RESERVED_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [224,1,0,0], [224,1,255,255]
);

pub const AD_HOC_2_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [224,3,0,0], [224,4,255,255]
);

pub const RESERVED_2_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [225,0,0,0], [231,255,255,255]
);

pub const SRC_SPEC_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
  [232,0,0,0], [232,255,255,255]
);

pub const  GLOP_ADDR_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [233,0,0,0], [233,251,255,255]
);

pub const AD_HOC_BLOCK_3_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [233,252,0,0], [233,255,255,255]
);

pub const UCAST_PREFIX_BASE_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [234,0,0,0], [234,255,255,255]
);

pub const RSRVD_3_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [235,0,0,0], [238,255,255,255]
);

pub const ADMIN_SCOPE_MCAST_ADDR_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [239,0,0,0], [239,255,255,255]
);

pub const BASE_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,0);
pub const ALL_HOSTS_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,1);
pub const ALL_RTR_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,2);
pub const DVMRP_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,4);
pub const OSPF_ALL_RTR_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,5);
pub const OSPF_ALL_DESIG_RTR_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,6);
pub const RIP_V2_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,9);
pub const EIGRP_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,10);
pub const PIM_V2_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,13);
pub const VRRP_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,18);
pub const IS_IS_IP_MCAST_IPV4_ADDR: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [224,0,0,19], [224,0,0,21]
);
pub const IGMP_V3_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224, 0, 0, 22);
pub const HSRP_V2_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,102);
pub const PTP_V2_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,107);
pub const MDNS_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,251);
pub const LLMNR_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,252);
pub const TEREDO_TUN_CLNT_DISCO_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,0,253);
pub const NTP_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,1);
pub const SVC_LOC_PROTO_V1_GEN_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,22);
pub const SVC_LOC_PROTO_V1_DIR_AGENT_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,35);
pub const AUTO_RP_ANNOUNCE_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,39);
pub const AUTO_RP_DISCO_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,40);
pub const H323_GATEKEEPER_DISCO_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,41);
pub const PTP_V1_MSG_MCAST_IPV4_RANGE: Ipv4AddressRange = Ipv4AddressRange::from_arrays(
    [224,0,1,129], [224,0,1,132]
);
pub const PTP_V2_MSG_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(224,0,1,129);
pub const SSDP_MCAST_IPV4_ADDR: Ipv4Address = Ipv4Address::from_bytes(239,255,255,250);
pub const SLP_V2_ADDR: Ipv4Address = Ipv4Address::from_bytes(239,255,255,253);

// TODO: bit-pack multicast address using fn
// old: 8 - prefix, 4 - flags, 4 - scope, 112 - group id
// new: 8 - prefix, 4 - ff1, 4 - scope, 4 - ff2, 4- reserved, 8 - plen , 64 - network prefix, 32 - group_id

pub const IPV6_MCAST_ADDR_FMT: Ipv6Address = Ipv6Address::from_bytes(0xff, 0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0);

// TODO: multicast address scope; https://en.wikipedia.org/wiki/Multicast_address

pub const LOC_NET_NODE_SEG_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 1);
pub const LOC_NET_RTR_SEG_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 2);
pub const OSPF_V3_ALL_SPF_RTR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 5);
pub const OSPF_V3_ALL_DR_RTR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 6);
pub const IS_IS_RTR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 8);
pub const RIP_RTR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 9);
pub const EIGRP_RTR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 0xa);
pub const PIM_RTR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 0xd);
pub const VRRP_V3_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 0x12);
pub const MLD_V2_REPORT_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,0, 0x16);
pub const DHCP_V6_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,1, 2);
pub const LLMNR_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 2,0,0,0,0,0,0,0,0,0,0,0,0,1, 3);
pub const DHCP_SRV_MCAST_IPV6: Ipv6Address = Ipv6Address::from_bytes(0xff, 5,0,0,0,0,0,0,0,0,0,0,0,0,1, 3);
// TODO: SSDP ff0x::c
// TODO: MDNS ff0x::fb
// TODO: NTP ff0x::101
// TODO: Net Info Svc ff0x::108
// TODO: PTPv2 Msg ff0x::181
// TODO: PTPv2 peer delay ffo2::6b
// TODO: Used for experiments ff0x::114

// TODO: source-specific multicast https://en.wikipedia.org/wiki/Source-specific_multicast

