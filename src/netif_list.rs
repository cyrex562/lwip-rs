use crate::network_interface::NetworkInterface;

pub struct NetworkInterfaceList {
    pub interfaces: Vec<NetworkInterface>,
    pub next: u32,
}

