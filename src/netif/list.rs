use std::collections::HashMap;
use crate::network_interface::NetworkInterface;

pub struct NetworkInterfaceList {
    pub interfaces: HashMap<u32, NetworkInterface>,
    pub next: u32,
}

