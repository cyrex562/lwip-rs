pub enum IpAddressType {
    IPv4,
    IPv6,
}

pub struct IpAddress {
    address_bytes: Vec<u8>,
    address_type: IpAddressType,
}
