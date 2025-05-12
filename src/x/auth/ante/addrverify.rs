use std::collections::BTreeSet;

use anyhow::anyhow;

use crate::{
    core::{MsgRegistry, SdkContext},
    tx::TxAnteHandler,
    types::InterLiquidSdkError,
    x::auth::ante::StdTx,
};

pub struct AddrVerifyAnteHandler {}

impl AddrVerifyAnteHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl TxAnteHandler<StdTx> for AddrVerifyAnteHandler {
    fn handle(
        &self,
        _ctx: &mut SdkContext,
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
