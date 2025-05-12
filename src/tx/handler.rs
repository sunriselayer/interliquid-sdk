use crate::{
    core::{Context, MsgRegistry},
    types::InterLiquidSdkError,
};

use super::Tx;

pub trait TxAnteHandler<TX: Tx>: Send + Sync {
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}

pub trait TxPostHandler<TX: Tx>: Send + Sync {
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}
