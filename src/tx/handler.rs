use crate::{core::Context, types::InterLiquidSdkError};

use super::Tx;

pub trait TxAnteHandler<C: Context, TX: Tx> {
    fn handle(&self, ctx: &mut C, tx: &TX) -> Result<(), InterLiquidSdkError>;
}

pub trait TxPostHandler<C: Context, TX: Tx> {
    fn handle(&self, ctx: &mut C, tx: &TX) -> Result<(), InterLiquidSdkError>;
}
