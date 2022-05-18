pub fn lwip_make_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    let x = [a,b,c,d];
    u32::from_le_bytes(x)
}

