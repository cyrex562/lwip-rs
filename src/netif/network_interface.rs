use std::collections::HashMap;
use std::time::SystemTime;
use chrono::prelude::*;
use std::time::UNIX_EPOCH;
use log::debug;
use crate::core::mac_address::MacAddress;
use crate::core::packet_buffer::PacketBuffer;
use crate::ipv4_acd::AcdStateInfo;
use crate::errors::{LwipError, LwipErrorCode};
use crate::errors::LwipErrorCode::{InvalidArgument, NotSet};
use crate::ip::ip_input;
use crate::ip_address::{IpAddress, IPV4_ADDR_ANY};
use crate::ipv4::ipv4_address::Ipv4Address;
use crate::ipv4::ipv4_network::Ipv4Network;
use crate::ipv6::ip6_addr::Ipv6Address;
use crate::mac_address::MacAddress;
use crate::mac_filter::MacFilterOps;
use crate::netif_hint::NetifHint;
use crate::packet_buffer::PacketBuffer;
use crate::queue::Queue;

pub const NETIF_REPORT_TYPE_IPV4: u8 = 0x01;
pub const NETIF_REPORT_TYPE_IPV6: u8 = 0x02;

pub enum LwipInternalNetifClientDataIndex {
    LwipNetifClientDataIndexDhcp,
    LwipNetifClientDataIndexAutoip,
    LwipNetifClientDataIndexAcd,
    LwipNetifClientDataIndexIgmp,
    LwipNetifClientDataIndexDhcp6,
    LwipNetifClientDataIndexMld6,
    PNetifClientDataIndexMax,
}

pub const NETIF_CHECKSUM_GEN_IP: u32 = 0x0001;
pub const NETIF_CHECKSUM_GEN_UDP: u32 = 0x0002;
pub const NETIF_CHECKSUM_GEN_TCP: u32 = 0x0004;
pub const NETIF_CHECKSUM_GEN_ICMP: u32 = 0x0008;
pub const NETIF_CHECKSUM_GEN_ICMP6: u32 = 0x0010;
pub const NETIF_CHECKSUM_CHECK_IP: u32 = 0x0100;
pub const NETIF_CHECKSUM_CHECK_UDP: u32 = 0x0200;
pub const NETIF_CHECKSUM_CHECK_TCP: u32 = 0x0400;
pub const NETIF_CHECKSUM_CHECK_ICMP: u32 = 0x0800;
pub const NETIF_CHECKSUM_CHECK_ICMP6: u32 = 0x1000;
pub const NETIF_CHECKSUM_ENABLE_ALL: u32 = 0xFFFF;
pub const NETIF_CHECKSUM_DISABLE_ALL: u32 = 0x0000;

#[derive(Debug, Clone, Default)]
pub struct NetifIpv6AddressContext {
    address: Ipv6Address,
    state: u8,
    valid_life: u32,
    preferred_life: u32,
}

#[derive(Debug, Clone, Default)]
pub struct NetifIpv4AddressContext {
    address: Ipv4Address,
    netmask: Ipv4Address,
}

pub enum NetworkInterfaceType {
    /// Ethernet device that may or may not process ARP or other traffic, such as PPPoE
    Ethernet,
    NotSet,
}

pub struct NetifIgmpMacFilter {
    pub grp: IgmpGroup,
    pub action: MacFilterAction,
}

pub struct NetifMldMacFilter {
    pub grp: MldGroup,
    pub action: MacFilterAction,
}

/// Generic data structure used for all lwIP network interfaces.
#[derive(Debug, Clone, Default)]
pub struct NetworkInterface {
    pub id: i64,
    /// a list of assigned MAC addresses
    pub mac_address: MacAddress,
    /// a list of assigned IPv4 addresses
    pub ipv4_nets: Vec<Ipv4Network>,
    /// a list of assigned IPv6 addresses
    pub ipv6_nets: Vec<NetifIpv6AddressContext>,
    /// the type of network interface
    pub interface_type: NetworkInterfaceType,
    /// MTU
    pub mtu: u16,
    /// name of the interface
    pub name: String,
    /// is this netif enabled for IPv6 autoconfiguration
    pub ip6_autoconfig_enabled: bool,
    /// Number of Router Solicitation messages that remain to be sent.
    pub rtr_sol_cnt: u8,
    /// type of link
    pub link_type: u8,
    /// speed of link in bits per sec
    pub link_speed: i64,
    /// timestamp at last change made (up/down)
    pub last_state_change_ts: Option<DateTime<UTC>>,
    /// a table of mac filters
    pub igmp_mac_filters: HashMap<u32, NetifIgmpMacFilter>,
    /// a table of MLD MAC filters
    pub mld_mac_filters: HashMap<u32, NetifMldMacFilter>,
    /// packets to transmit
    pub tx_queue: Queue<PacketBuffer>,
    /// received packets
    pub rx_queue: Queue<PacketBuffer>,
    /// Used if the original scheduling failed.
    pub reschedule_poll: bool,
    /// whether the link is enabled and can process traffic
    pub up: bool,
    /// active link
    pub link_up: bool,
    /// whether or not the device processes arp packets
    pub etharp: bool,
    /// whether or not the device processes IGMP packets
    pub igmp: bool,
    /// whether or not the device has broadcast capability
    pub broadcast: bool,
    /// whether or not the device has MLD6 capability
    pub mld6: bool,
}

impl NetworkInterface {
    pub fn new() -> Self {
        let dt = Utc::now();
        Self {
            id: dt.timestamp_millis(),
            mac_addresses: Vec::new(),
            ipv4_nets: Vec::new(),
            ipv6_nets: Vec::new(),
            interface_type: NetworkInterfaceType::NotSet,
            mtu: 1500,
            name: "".to_string(),
            ip6_autoconfig_enabled: false,
            rtr_sol_cnt: 0,
            link_type: 0,
            link_speed: -1,
            last_state_change_ts: None,
            igmp_mac_filters: HashMap::new(),
            mld_mac_filters: HashMap::new(),
            tx_queue: Queue::new(),
            rx_queue: Queue::new(),
            reschedule_poll: false,
            up: false,
            link_up: false,
            etharp: false,
            igmp: false,
            broadcast: false,
            mld6: false,
        }
    }

    pub fn low_level_init(&mut self, hwaddr: &MacAddress, mtu: u16) {
        self.mac_address = hwaddr.clone();
        self.mtu = mtu;
        self.broadcast = true;
        self.etharp = true;
        self.link_up = true;
        // TODO: if the MLD Mac Filter is set, add a filter?
    }

    pub fn init(&mut self) -> Result<(), LwipError> {
        todo!();
        Ok(())
    }

    pub fn input(&mut self) -> Result<PacketBuffer, LwipError> {
        let p = self.rx_queue.pop();
        Ok(p)
    }

    pub fn output(&mut self, p: &PacketBuffer) -> Result<(), LwipError> {
        self.tx_queue.push(p);
        Ok(())
    }

    pub fn add_igmp_mac_filter(&mut self, grp: &IgmpGroup, action: MacFilterAction) -> Result<(), LwipError> {
        todo!()
    }

    pub fn rem_igmp_mac_filter(&mut self, grp: &IgmpGroup) -> Result<(), LwipError> {
        todo!()
    }

    pub fn add_mld_mac_filter(&mut self, grp: &MldGroup, action: MacFilterAction) -> Result<(), LwipError> {
        todo!()
    }

    pub fn rem_mld_mac_filter(&mut self, grp: &MldGroup, action: MacFilterAction) -> Result<(), LwipError> {
        todo!()
    }

    pub fn reg_status_cb_fn(&mut self, func: NetifStatusCallbackFunc) -> Result<u32, LwipError> {
        todo!()
    }

    pub fn unreg_status_cb_fn(&mut self, id: u32) -> Result<(), LwipError> {
        todo!()
    }

    pub fn reg_link_state_chg_cb_fn(&mut self, func: NetifLinkStateChgFunc) -> Result<u32, LwipError> {
        todo!()
    }

    pub fn unreg_link_state_chg_cb_fn(&mut self, id: u32) -> Result<(), LwipError> {
        todo!()
    }

    pub fn has_ip4_addr(&self, addr: &Ipv4Network) -> bool {
        self.ipv4_nets.contains(addr)
    }
}

pub enum LwipNetifStateChange {
    None,
    Added,
    Removed,
    LinkChanged,
    StatusChanged,
    Ipv4AddressChanged,
    Ipv4GatewayChanged,
    Ipv4NetmaskChanged,
    Ipv4SettingsChanged,
    Ipv6Set,
    Ipv6AddrStateChanged,
    Ipv4AddrValid,
}


#[derive(Clone, Debug, Default)]
pub struct NetifExtCallbackArgs {
    state: u8,
    old_address: IpAddress,
    old_netmask: IpAddress,
    old_gateway: IpAddress,
    address_index: u32,
    old_state: u8,
    address: IpAddress,
}

impl NetifExtCallbackArgs {
    pub fn new() -> Self {
        Self::default()
    }
}

type NetifExtCallbackFn = fn(netif: &mut NetworkInterface, reason: u16, args: &NetifExtCallbackArgs);

pub struct NetifExtCallback {
    callback_fn: NetifExtCallbackFn,
    next: u32,
}



