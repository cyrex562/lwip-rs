/// Swap the bytes in an u16: much like lwip_htons() for little-endian
pub fn swap_bytes_in_word(w: u16) -> u16 { (((w) & 0xff) << 8) | (((w) & 0xff00) >> 8)}

/// Split an u32 in two u16s and add them up
pub fn fold_u32(u: u32) -> u32 {
    (((u) >> 16) + ((u) & 0x0000ffff))
}
