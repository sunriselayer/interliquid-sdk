use crate::{core::Context, types::InterLiquidSdkError};

use super::Tx;

pub trait TxAnteHandler {
    fn handle(&self, ctx: &mut dyn Context, tx: &Tx) -> Result<(), InterLiquidSdkError>;
}

pub trait TxPostHandler {
    fn handle(&self, ctx: &mut dyn Context, tx: &Tx) -> Result<(), InterLiquidSdkError>;
}
