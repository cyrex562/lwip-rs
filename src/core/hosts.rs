use crate::ip_address::IpAddress;

#[derive(Default,Clone,Debug)]
pub struct HostListEntry {
    pub name: String,
    pub ip_address: IpAddress,
}

impl HostListEntry {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            ip_address: IpAddress::new(),
        }
    }
}

pub struct HostsList {
    pub hosts: Vec<HostListEntry>
}
