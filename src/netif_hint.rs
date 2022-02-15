pub struct NetifHint {
    addr_hint: u8,
    /** VLAN hader is set if this is >= 0 (but must be <= 0xFFFF) */
    tci: i32,
}
