pub const ICMP_ER: u32 = 0; //  echo reply 
pub const ICMP_DUR: u32 = 3; //  destination unreachable 
pub const ICMP_SQ: u32 = 4; //  source quench 
pub const ICMP_RD: u32 = 5; //  redirect 
pub const ICMP_ECHO: u32 = 8; //  echo 
pub const ICMP_TE: u32 = 11; //  time exceeded 
pub const ICMP_PP: u32 = 12; //  parameter problem 
pub const ICMP_TS: u32 = 13; //  timestamp 
pub const ICMP_TSR: u32 = 14; //  timestamp reply 
pub const ICMP_IRQ: u32 = 15; //  information request 
pub const ICMP_IR: u32 = 16; //  information reply 
pub const ICMP_AM: u32 = 17; //  address mask request 
pub const ICMP_AMR: u32 = 18; //  address mask reply 

pub struct IcmpEchoHeader {
    pub echo_type: u8,
    pub code: u8,
    pub chksum: u16,
    pub id: u16,
    pub seqno: u16,
}

impl IcmpEchoHeader {
    pub fn ICMPH_TYPE(&mut self) -> u8 {
        self.echo_type
    }
    pub fn ICMP_CODE(&mut self) -> u8 {
        self.code
    }
    pub fn ICMPH_TYPE_SET(&mut self, t: u8) {
        self.echo_type = t
    }
    pub fn ICMPH_CODE_SET(&mut self, c: u8) {
        self.code = c
    }
}
