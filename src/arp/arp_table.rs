use crate::core::lwip_context::LwipContext;
use crate::core::mac_address::MacAddress;
use crate::errors::{LwipError, LwipErrorCode};
use crate::ipv4::ipv4_address::{ipv4_addr_is_any, Ipv4Address};
use crate::ipv4::ipv4_network::{ipv4_addr_is_broadcast, Ipv4Network};
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
    Static,
}

pub enum ArpEntryFlag {
    NotSet,
    TryHard = 1,
    FindOnly = 2,
    StaticEntry = 3,
}

#[derive(Debug, Clone, Default)]
pub struct ArpEntry {
    pub ip4_addr: Option<Ipv4Address>,
    pub mac_addr: Option<MacAddress>,
    pub netif_id: i64,
    pub ctime: i64,
    pub state: ArpState,
}

impl ArpEntry {
    pub fn new() -> Self {
        Self {
            ip4_addr: None,
            mac_addr: None,
            netif_id: -1,
            ctime: -1,
            state: ArpState::NotSet,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArpTable {
    table: Vec<ArpEntry>,
}

impl ArpTable {
    pub fn new() -> Self {
        Self {
            table: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
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
            if entry.state != ArpState::Empty && entry.state != ArpState::Static {
                entry.ctime += 1;
                if entry.state == ArpState::StableRerequesting1 {
                    entry.state = ArpState::StableRerequesting2;
                } else if entry.state == ArpState::StableRerequesting2 {
                    entry.state = ArpState::Stable
                } else if entry.state == ArpState::Pending {
                    send_arp_request(entry.netif_id, &entry.ip4_addr);
                }
            }
        });
        self.table.retain(|&mut entry| {
            if entry.state != ArpState::Empty && entry.state != ArpState::Static {
                if entry.ctime >= ARP_MAX_AGE || (entry.state == ArpState::Pending && entry.ctime >= ARP_MAX_PENDING) {
                    return false;
                }
            }
            return true;
        });
        Ok(())
    }

    pub fn find_entry_by_ip_addr(&self, ip4_addr: &Ipv4Address) -> Result<(usize, ArpEntry), LwipError> {
        let result = self.table.iter().position(|entry| { if entry.ip4_addr.is_some() {
         if entry.ip4_addr.unwrap() == ip4_addr {
             true
         }
        }
            false
        });
        if result.is_some() {
            let index = result.unwrap();
            let out_entry = self.table.get(index).unwrap().clone();
            Ok((index, out_entry))
        }
        Err(LwipError::new(LwipErrorCode::NotFound, "entry for ipv4 address {} not found".format(ip4_addr)))
    }

    pub fn update_entry(&mut self,
                        ctx: &mut LwipContext,
                        ip4_addr: &Ipv4Address,
                        mac_addr: &MacAddress,
                        static_entry: bool,
                        netif_id: i64) -> Result<(), LwipError> {
        let mut is_broadcast: bool = false ;
        for netif in ctx.netifs.iter() {
            for ip4net in netif.ipv4_nets.iter() {
                if ip4_addr == ip4net.broadcast_addr {
                    is_broadcast = true;
                    break;
                }
            }
        }

        if ipv4_addr_is_any(ip4_addr) ||is_broadcast || ip4v4_addr_is_multicast(ip4_addr) {
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "cannot add non-unicast address to arp table: {}".format(ip4_addr)));
        }

         let mut entry = ArpEntry {
                        ip4_addr: Some(ip4_addr.clone()),
                        mac_addr: Some(mac_addr.clone()),
                        netif_id: netif_id,
                        ctime: 0,
                        state: ArpState::NotSet
                    };

        let result = self.find_entry_by_ip_addr(ip4_addr);
        match result {
            Ok((idx, val)) => {
                self.table[idx] = entry;
            },
            Err(e) => {
                if e.code == LwipErrorCode::NotFound {

                    if static_entry {
                        entry.state = ArpState::Static
                    }
                    self.table.push(entry);
                } else {
                    return Err(LwipError::new(LwipErrorCode::OperationFailed, "arp table find operation failed: {}".format(e)));
                }
            }
        }
        Ok(())
    }

    pub fn update_entry_state(&mut self, index: usize, new_state: ArpState) -> Result<(), LwipError> {
        if index > self.table.len() {
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "index {} greater than table len {}".format(index, self.table.len())));
        }

        self.table[index].state = new_state;

        Ok(())
    }

    pub fn remove_arp_entry_by_ip_addr(&mut self, ip4_addr: &Ipv4Address) -> Result<(), LwipError> {
        let result = self.find_entry_by_ip_addr(ip4_addr);
        return match result {
            Ok((idx, _)) => {
                self.table.remove(idx);
                Ok(())
            },
            Err(e) => {
                Err(LwipError::new(LwipErrorCode::OperationFailed, "find arp entry failed: {}".format(e)))
            }
        }
    }

    pub fn remove_arp_entries_by_netif_id(&mut self, netif_id: i64) -> Result<(), LwipError> {
        self.table.retain(|entry| {
            entry.netif_id != netif_id
        });
        Ok(())
    }
    
    pub fn get_entry_by_index(&self, index: usize) -> Result<ArpEntry, LwipError> {
        if index > self.table.len() {
            return Err(LwipError::new(LwipErrorCode::InvalidArgument, "index value {} greater than length of arp table: {}".format(index, self.table.len())));
        }
        let result = self.table.get(index);
        return match result {
            Some(entry) => Ok(entry.clone()),
            None() => Err(LwipError::new(LwipErrorCode::OperationFailed, "failed to retrieve entry at index {}".format(index)))
        }
    }
}

