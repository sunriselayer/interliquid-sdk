use crate::{
    core::{Context, MsgRegistry},
    types::InterLiquidSdkError,
};

use super::Tx;

/// The handler which is called before Tx's Msg executions.
///
/// Ante handlers are responsible for performing pre-processing checks before
/// message execution, such as signature verification, fee deduction, and
/// other validation logic.
pub trait TxAnteHandler<TX: Tx>: Send + Sync {
    /// Handles pre-transaction processing.
    ///
    /// # Parameters
    /// - `ctx`: The mutable context for state access and modifications
    /// - `msg_registry`: Registry containing message type information
    /// - `tx`: The transaction to be processed
    ///
    /// # Returns
    /// - `Ok(())` if pre-processing succeeds
    /// - `Err(InterLiquidSdkError)` if validation fails
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}

/// The handler which is called after Tx's Msg executions.
///
/// Post handlers are responsible for performing post-processing actions after
/// message execution, such as event emission, cleanup, or additional state
/// modifications based on the execution results.
pub trait TxPostHandler<TX: Tx>: Send + Sync {
    /// Handles post-transaction processing.
    ///
    /// # Parameters
    /// - `ctx`: The mutable context for state access and modifications
    /// - `msg_registry`: Registry containing message type information
    /// - `tx`: The transaction that was processed
    ///
    /// # Returns
    /// - `Ok(())` if post-processing succeeds
    /// - `Err(InterLiquidSdkError)` if post-processing fails
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}
