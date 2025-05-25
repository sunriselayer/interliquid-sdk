use crate::{
    core::{Context, MsgRegistry},
    types::InterLiquidSdkError,
};

use super::Tx;

/// The handler which is called before Tx's Msg executions.
pub trait TxAnteHandler<TX: Tx>: Send + Sync {
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}

/// The handler which is called after Tx's Msg executions.
pub trait TxPostHandler<TX: Tx>: Send + Sync {
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}
