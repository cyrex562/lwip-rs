use crate::core::context::LwipContext;
use crate::core::error::LwipError;

#[derive(Clone, Debug, Default)]
pub struct DesContext {

}

impl DesContext {
    pub fn new() -> DesContext {
        DesContext::default()
    }
}

pub fn des_setkey_enc(ctx: &mut LwipContext, des_key: &[u8;8]) -> Result<(), LwipError> {
    unimplemented!()
}

pub fn des_crypt_ecb(ctx: &mut LwipContext, challenge: &String, response: &[u8]) -> Result<(), LwipError> {
    unimplemented!()
}
