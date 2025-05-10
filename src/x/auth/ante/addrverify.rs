use std::{collections::BTreeSet, marker::PhantomData};

use anyhow::anyhow;

use crate::{core::Context, tx::TxAnteHandler, types::InterLiquidSdkError, x::auth::ante::StdTx};

pub struct AddrVerifyAnteHandler<C: Context> {
    phantom: PhantomData<C>,
}

impl<C: Context> AddrVerifyAnteHandler<C> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<C: Context> TxAnteHandler<C, StdTx> for AddrVerifyAnteHandler<C> {
    fn handle(&self, ctx: &mut C, tx: &StdTx) -> Result<(), InterLiquidSdkError> {
        let unpacked_msgs = tx
            .body
            .msgs
            .iter()
            .map(|msg_any| ctx.msg_registry().unpack(msg_any))
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
