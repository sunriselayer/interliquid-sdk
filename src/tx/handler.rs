use crate::{core::Context, types::InterLiquidSdkError};

use super::Tx;

pub trait TxAnteHandler<TX: Tx> {
    fn handle(&self, ctx: &mut dyn Context, tx: &TX) -> Result<(), InterLiquidSdkError>;
}

pub trait TxPostHandler<TX: Tx> {
    fn handle(&self, ctx: &mut dyn Context, tx: &TX) -> Result<(), InterLiquidSdkError>;
}
