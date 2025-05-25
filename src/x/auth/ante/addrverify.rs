use std::collections::BTreeSet;

use anyhow::anyhow;

use crate::{
    core::{Context, MsgRegistry, TxAnteHandler},
    types::InterLiquidSdkError,
    x::auth::ante::StdTx,
};

/// An ante handler that verifies all message signers have corresponding auth info in the transaction.
/// This ensures that every address that needs to sign a message is properly authenticated.
pub struct AddrVerifyAnteHandler {}

impl AddrVerifyAnteHandler {
    /// Creates a new AddrVerifyAnteHandler instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl TxAnteHandler<StdTx> for AddrVerifyAnteHandler {
    /// Verifies that all message signers have auth info in the transaction.
    /// 
    /// # Arguments
    /// * `_ctx` - The execution context (unused)
    /// * `msg_registry` - Registry to unpack message types
    /// * `tx` - The transaction to verify
    /// 
    /// # Errors
    /// Returns an error if any signer address is missing from the auth info.
    fn handle(
        &self,
        _ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &StdTx,
    ) -> Result<(), InterLiquidSdkError> {
        let unpacked_msgs = tx
            .body
            .msgs
            .iter()
            .map(|msg_any| msg_registry.unpack(msg_any))
            .collect::<Result<Vec<_>, InterLiquidSdkError>>()?;

        let signers = unpacked_msgs
            .iter()
            .flat_map(|msg| msg.signer_addresses())
            .collect::<BTreeSet<_>>();

        for signer in signers {
            if !tx.auth_info.contains_key(&signer) {
                return Err(InterLiquidSdkError::Unauthorized(anyhow!(
                    "address is not contained in auth info"
                )));
            }
        }

        Ok(())
    }
}
