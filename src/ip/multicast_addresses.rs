use std::net::Ipv4Addr;
use crate::ip::defs::Ipv4AddressRange;

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


