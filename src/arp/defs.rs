use crate::defines::LwipAddr;
use crate::netif::netif_h::NetIfc;
use crate::packetbuffer::pbuf_h::PacketBuffer;

pub const ARP_AGE_REREQUEST_USED_UNICAST: u32 = (ARP_MAXAGE - 30);
pub const ARP_AGE_REREQUEST_USED_BROADCAST: u32 = (ARP_MAXAGE - 15);
pub const ARP_MAXPENDING: u32 = 5;

pub enum ArpState {
    EtharpStateEmpty = 0,
    EtharpStatePending,
    EtharpStateStable,
    EtharpStateStableRerequesting1,
    EtharpStateStableRerequesting2,
    EtharpStateStatic,
}

pub struct ArpEntry {
    /* Pointer to queue of pending outgoing packets on this ARP entry. */
    // pub q: etharp_q_entry,
    /* arp_queueing */
    /* Pointer to a single pending outgoing packet on this ARP entry. */
    pub q: PacketBuffer,
    pub ipaddr: LwipAddr,
    pub netif: NetIfc,
    pub ethaddr: LwipAddr,
    pub ctime: u64,
    pub state: ArpState,
}


pub const ETHARP_FLAG_TRY_HARD: u8 = 1;
pub const ETHARP_FLAG_FIND_ONLY: u8 = 2;
pub const ETHARP_FLAG_STATIC_ENTRY: u8 = 4;

// #define ARP_TMR_INTERVAL 1000
pub const ARP_TMR_INTERVAL: u32 = 1000;

pub struct etharp_q_entry {
    // next: &mut etharp_q_entry;
    p: &mut PacketBuffer,
}
