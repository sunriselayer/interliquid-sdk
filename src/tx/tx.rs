use std::any::Any;

use crate::types::{Address, InterLiquidSdkError};

pub trait Tx: Any {
    fn signer_address(&self) -> Result<Address, InterLiquidSdkError>;
    fn sign_bytes(&self) -> Result<Vec<u8>, InterLiquidSdkError>;
}
