use core::result::Result;
use core::result::Result::{Err, Ok};

#[derive(Default,Debug,Clone)]
pub struct VlanTag {
    /// TPID: 16 bits: (0x8100)
    pub tpid: u16,
    /// TCI:
    ///   PCP: 3 bits: class-of-service
    ///   DEI: 1 bit: drop-eligible indicator
    ///   VID: 12-bits VLAN tag
    pub tci: u16,
}

impl VlanTag {
    pub fn set_pcp(&mut self, new_pcp_val: u8) -> Result<(), LwipError> {
        if new_pcp_val > 0b111 {
            return Err(LwipError::new(InvalidArgument, format!("invalid value for new pcp val: {}", new_pcp_val).as_str()))
        }
        self.tci = self.tci & new_pcp_val << 13;
        Ok(())
    }

    pub fn get_pcp(&self) -> u16 {
        self.tci & 0xe000 //0b1110000000000000
    }

    pub fn set_dei(&mut self, dei: u8) -> Result<(), LwipError> {
        if dei > 1 {
            return Err(LwipError::new(InvalidArgument, format!("invalid value for new dei val: {}", dei).as_str()))
        }

        self.tci = self.tci & dei << 12;

        Ok(())
    }

    pub fn get_dei(&self) -> u16 {
        self.tci & 0x1000 // 0b0001000000000000
    }

    pub fn set_vid(&mut self, vid: u16) {
        self.tci = self.tci | vid;
    }

    pub fn get_vid(&self) -> u16 {
        self.tci & 0x0fff
    }
}

// END OF FILE
