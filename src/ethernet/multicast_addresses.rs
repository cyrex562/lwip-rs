use crate::ethernet::defs::{Eui48, MacAddress, MacAddressRange};

pub const CISCO_CDP_MCAST_ADDR: Eui48 = [0x1, 0x0, 0xc, 0xcc, 0xcc, 0xcc];
pub const CISCO_VTP_MCAST_ADDR: Eui48 = [0x1, 0x0, 0xc, 0xcc, 0xcc, 0xcc];
pub const CISCO_UDLD_MCAST_ADDR: Eui48 = [0x1, 0x0, 0xc, 0xcc, 0xcc, 0xcc];
pub const CISCO_SHARED_STP_MCAST_ADDR: Eui48 = [0x1, 0x0, 0xc, 0xcc, 0xcc, 0xcd];
pub const CISCO_CGMP_MCAST_ADDR: Eui48 = [0x1, 0x0, 0xc, 0xdd, 0xdd, 0xdd];
pub const HUGHES_TERM_SRV_SW_DL: Eui48 = [0x1, 0x0, 0x10, 0x0, 0x0, 0x20];
pub const HUGHES_TER_SRV_SW_REQ: Eui48 = [0x01, 0x00, 0x10, 0xff, 0xff, 0x20];
pub const STP_BRIDGE_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x0];
pub const LLDP_EXTRA_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x0];
pub const ETHER_OAM_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x00, 0x00, 0x2];
pub const ETHER_FLOW_CTRL_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x1];
pub const ETHER_LACP_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x2];
pub const ETHER_PORT_AUTH_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x3];
pub const LLDP_EXTRA_2_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x3];
pub const STP_PROV_BRIDGE_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x8];
pub const MVRP_PROV_BRIDGE_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0xd];
pub const LLDP_PRI_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0xe];
pub const PTP_O_ETHER_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0xe];
pub const GVRP_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x21];
pub const MVRP_MCAST_ADDR: Eui48 = [0x1, 0x80, 0xc2, 0x0, 0x0, 0x21];
pub const ETHER_BCAST_ADDR: MacAddress = MacAddress::from_bytes(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
pub const ETHER_CFM_PROTO_MCAST_ADDR_RANGE: MacAddressRange = MacAddressRange {
    start: [0x1, 0x80, 0xc2, 0x0, 0x0, 0x30],
    end: [0x1, 0x80, 0xc2, 0x0, 0x0, 0x3f],
};
pub const PTP_O_ETHER_SPEC: Eui48 = [0x1, 0xb, 0x19, 0x0, 0x0, 0x0];
// use low 23 bits of ipv4 address in the ethernet address
pub const IPV4_MCAST_MAC_ADDR_RANGE: MacAddressRange = MacAddressRange {
    start: [0x1, 0x0, 0x5e, 0x0, 0x0, 0x0],
    end: [0x1, 0x0, 0x5e, 0x7f, 0xff, 0xff],
};
// use low 32 ibts of ipv6 address in the ethernet address
pub const IPV6_MCAST_MAC_ADDR_RANGE: MacAddressRange = MacAddressRange {
    start: [0x33, 0x33, 0x0, 0x0, 0x0, 0x0],
    end: [0x33, 0x33, 0xff, 0xff, 0xff, 0xff],
};
pub const IEC61850_8_1_GOOSE_MAC_ADDR_RANGE: MacAddressRange = MacAddressRange {
    start: [0x1, 0xc, 0xcd, 0x1, 0x0, 0x0],
    end: [0x1, 0xc, 0xcd, 0x1, 0x1, 0xff],
};
pub const IEC61850_8_1_GSSE_MAC_ADDR_RANGE: MacAddressRange = MacAddressRange {
    start: [0x1, 0xc, 0xcd, 0x2, 0x0, 0x0],
    end: [0x1, 0xc, 0xcd, 0x2, 0x1, 0xff],
};
pub const IEC61850_8_1_MCAST_SAMP_VAL_MAC_ADDR_RANGE: MacAddressRange = MacAddressRange {
    start: [0x1, 0xc, 0xcd, 0x4, 0x0, 0x0],
    end: [0x1, 0xc, 0xcd, 0x4, 0x1, 0xff],
};

pub fn mac_addr_in_range(addr: &eui48, range: &MacAddressRange) -> bool {
    let addr_val: u64 = u64::from_le_bytes([0,0,addr[0], addr[1], addr[2], addr[3], addr[4], addr[5]]);
    let start_val: u64 = u64::from_le_bytes([0,0,range.start[0],range.start[1],range.start[2],range.start[3],range.start[4],range.start[5]]);
    let end_val: u64 = u64::from_le_bytes([0,0,range.end[0],range.end[1],range.end[2],range.end[3],range.end[4],range.end[5]]);
    start_val <= addr_val && addr_val <= end_val
}

pub fn mac_address_is_multicast(addr: &eui48) -> bool {
    if addr == CISCO_CDP_MCAST_ADDR {
        true
    }
    if addr == CISCO_VTP_MCAST_ADDR {
        true
    }
    if addr == CISCO_UDLD_MCAST_ADDR {
        true
    }
    if addr == CISCO_SHARED_STP_MCAST_ADDR {
        true
    }
    if addr == CISCO_CGMP_MCAST_ADDR {
        true
    }
    if addr == HUGHES_TERM_SRV_SW_DL {
        true
    }
    if addr == HUGHES_TER_SRV_SW_REQ {
        true
    }
    if addr == STP_BRIDGE_MCAST_ADDR {
        true
    }
    if addr == LLDP_EXTRA_MCAST_ADDR {
        true
    }
    if addr == ETHER_OAM_MCAST_ADDR {
        true
    }
    if addr == ETHER_FLOW_CTRL_MCAST_ADDR {
        true
    }
    if addr == ETHER_LACP_MCAST_ADDR {
        true
    }
    if addr == ETHER_PORT_AUTH_MCAST_ADDR {
        true
    }
    if addr == LLDP_EXTRA_2_MCAST_ADDR {
        true
    }
    if addr == STP_PROV_BRIDGE_MCAST_ADDR {
        true
    }
    if addr == MVRP_PROV_BRIDGE_MCAST_ADDR {
        true
    }
    if addr == LLDP_PRI_MCAST_ADDR {
        true
    }
    if addr == PTP_O_ETHER_MCAST_ADDR {
        true
    }
    if addr == GVRP_MCAST_ADDR {
        true
    }
    if addr == MVRP_MCAST_ADDR {
        true
    }
    if addr == ETHER_BCAST_ADDR {
        true
    }
    if addr == PTP_O_ETHER_SPEC { true}
    if mac_addr_in_range(addr, &ETHER_CFM_PROTO_MCAST_ADDR_RANGE) {true}
    if mac_addr_in_range(addr, &IPV4_MCAST_MAC_ADDR_RANGE) {true}
    if mac_addr_in_range(addr, &IPV6_MCAST_MAC_ADDR_RANGE){true}
    if mac_addr_in_range(addr, &IEC61850_8_1_GOOSE_MAC_ADDR_RANGE){true}
    if mac_addr_in_range(addr,&IEC61850_8_1_GSSE_MAC_ADDR_RANGE){true}
    if mac_addr_in_range(addr, &IEC61850_8_1_MCAST_SAMP_VAL_MAC_ADDR_RANGE) {true}
    false
}
