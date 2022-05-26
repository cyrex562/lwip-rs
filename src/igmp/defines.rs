use crate::ipv4::addr::Ipv4Address;

pub const IGMP_TTL: u32 = 1;
pub const IGMP_MINLEN: usize = 8;
pub const ROUTER_ALERT: u16 = 0x9404;
pub const ROUTER_ALERTLEN: u32 = 4;

pub const IGMP_MEMB_QUERY: u32 = 0x11;
// Membership query        //
pub const IGMP_V1_MEMB_REPORT: u32 = 0x12;
// Ver. 1 membership report//
pub const IGMP_V2_MEMB_REPORT: u32 = 0x16;
// Ver. 2 membership report//
pub const IGMP_LEAVE_GROUP: u32 = 0x17; // Leave-group message     //

pub const IGMP_GROUP_NON_MEMBER: u32 = 0;
pub const IGMP_GROUP_DELAYING_MEMBER: u32 = 1;
pub const IGMP_GROUP_IDLE_MEMBER: u32 = 2;

pub const IGMP_TMR_INTERVAL: u32 = 100; /* Milliseconds */
pub const IGMP_V1_DELAYING_MEMBER_TMR: u32 =   (1000/IGMP_TMR_INTERVAL);
pub const IGMP_JOIN_DELAYING_MEMBER_TMR: u32 = (500 /IGMP_TMR_INTERVAL);

#[derive(Debug, Clone, Default)]
pub struct IgmpMsg {
    // PACK_STRUCT_FLD_8(u8_t         igmp_msgtype);
    pub igmp_msg_type: u8,
    // PACK_STRUCT_FLD_8(u8_t         igmp_maxresp);
    pub igmp_max_resp: u8,
    // PACK_STRUCT_FIELD(u16_t        igmp_checksum);
    pub igmp_checksum: u16,
    // PACK_STRUCT_FLD_S(ip4_addr_p_t igmp_group_address);
    pub igmp_group_address: Ipv4Address,
}


/**
 * igmp group structure - there is
 * a list of groups for each interface
 * these should really be linked from the interface, but
 * if we keep them separate we will not affect the lwip original code
 * too much
 *
 * There will be a group for the all systems group address but this
 * will not run the state machine as it is used to kick off reports
 * from all the other groups
 */
#[derive(Default, Debug, Clone)]
pub struct IgmpGroup {
    /** next link */
    // struct igmp_group *next;
    /** multicast address */
    // ip4_addr_t         group_address;
    pub grp_addr: Ipv4Address,
    /** signifies we were the last person to report */
    // last_reporter_flag: u8;
    pub last_reporter_flag: u8,
    /** current state of the group */
    // group_state: u8;
    pub group_state: u8,
    /** timer for reporting, negative is OFF */
    // timer: u16;
    pub timer: u16,
    /** counter of simultaneous uses */
    // use: u8
    pub uses: u8,
}
