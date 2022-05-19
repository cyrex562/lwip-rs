pub const LWIP_IPV4_SRC_ROUTING: bool = true;
pub const IPV4_CLASS_A_NET: u32 = 0xff000000;
pub const IPV4_CLASS_A_NSHIFT: u32 = 24;
pub const IPV4_CLASS_A_HOST: u32 = 0xffffffff & !IPV4_CLASS_A_NET;
pub const IPV4_CLASS_A_MAX: u32 = 128;

pub const IPV4_CLASS_B_NET: u32 = 0xffff0000;
pub const IPV4_CLASS_B_NSHIFT: u32 = 16;
pub const IPV4_CLASS_B_HOST: u32 = 0xffffffff & !IPV4_CLASS_B_NET;
pub const IPV4_CLASS_B_MAX: u32 = 0xffff;

pub const IPV4_CLASS_C_NET: u32 = 0xffffff00;
pub const IPV4_CLASS_C_NSHIFT: u32 = 8;
pub const IPV4_CLASS_C_HOST: u32 = 0xffffffff & !IPV4_CLASS_C_NET;

pub const IPV4_CLASS_D_NET: u32 = 0xf0000000;
pub const IPV4_CLASS_D_NSHIFT: u32 = 28;
pub const IPV4_CLASS_D_HOST: u32 = 0x0fffffff;

pub const IPV4_LOOPBACK_NET: u32 = 127;

pub const IPV4_HDR_LEN: usize = 32;
pub const IPV4_MAX_HDR_LEN: usize = 60;
