use crate::errors::{LwipError, LwipErrorCode};
use crate::ipv4::ipv4_address::Ipv4Address;
use crate::mac_address::MacAddress;

pub const ARP_AGE_REREQUEST_USED_UNICAST: i64 = ARP_MAXAGE - 30;
pub const ARP_AGE_REREQUEST_USED_BROADCAST: i64 = ARP_MAXAGE - 15;
pub const ARP_MAX_PENDING: i64 = 5;

pub enum ArpState {
    NotSet,
    Empty,
    Pending,
    Stable,
    StableRerequesting1,
    StableRerequesting2,
    Static
}

pub enum ArpEntryFlag {
    NotSet,
    TryHard = 1,
    FindOnly = 2,
    StaticEntry = 3,
}

#[derive(Debug,Clone,Default)]
pub struct ArpEntry {
    ip4_addr: Option<Ipv4Address>,
    mac_addr: Option<MacAddress>,
    netif_id: i64,
    ctime: i64,
    state: ArpState,
}

impl ArpEntry {
    pub fn new() -> Self {
        Self {
            ip4_addr: None,
            mac_addr: None,
            netif_id: -1,
            ctime: -1,
            state: ArpState::NotSet
        }
    }
}

#[derive(Debug,Clone,Default)]
pub struct ArpTable {
    table: Vec<ArpEntry>,
}

impl ArpTable {
    pub fn new () -> Self {
        Self {
            table: Vec::new()
        }
    }

    pub fn free_entry(&mut self, index: usize) -> Result<(), LwipError> {
        if self.table.len() < index {
            Err(LwipError::new(LwipErrorCode::InvalidArgument, "index {} larger then  length of table".format(index)))
        }
        let result = self.table.remove(index);
        Ok(())
    }

    pub fn clear_expired_entries(&mut self) -> Result<(), LwipError> {
        self.table.iter_mut().map(|entry| {
            if entry.state
            entry.ctime += 1

        });
        self.table.retain(|&mut entry| {
            if entry.state != ArpState::Empty && entry.state != ArpState::Static {
                if entry.ctime >= ARP_MAX_AGE || (entry.state == ArpState::Pending && entry.ctime >= ARP_MAX_PENDING) {
                    return false
                }
            }
            return true
        });

        for entry in self.table.iter_mut() {
            entry.
        }

        self.table.iter_mut().

    }
}

