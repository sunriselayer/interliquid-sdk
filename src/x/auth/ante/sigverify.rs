use anyhow::anyhow;

use crate::{
    core::Context,
    tx::{Tx, TxAnteHandler},
    types::InterLiquidSdkError,
    x::{
        auth::keeper::{AuthKeeper, AuthKeeperI},
        crypto::key::{VerifyingKey, VerifyingKeyTraitImpl},
    },
};

pub struct SigVerifyAnteHandler {
    auth_keeper: AuthKeeper,
}

impl SigVerifyAnteHandler {
    pub fn new(auth_keeper: AuthKeeper) -> Self {
        Self { auth_keeper }
    }
}

impl TxAnteHandler for SigVerifyAnteHandler {
    fn handle(&self, ctx: &mut dyn Context, tx: &Tx) -> Result<(), InterLiquidSdkError> {
        let mut account = match self.auth_keeper.get_account(ctx, &tx.auth_info.address)? {
            Some(account) => account,
            None => {
                return Err(InterLiquidSdkError::NotFound(anyhow!("account not found")));
            }
        };

        if account.nonce != tx.auth_info.nonce {
            return Err(InterLiquidSdkError::InvalidRequest(anyhow!(
                "nonce mismatch"
            )));
        }

        account.nonce += 1;
        self.auth_keeper
            .set_account(ctx, &tx.auth_info.address, &account)?;

        let verifying_key = match self.auth_keeper.get_verifying_key(
            ctx,
            &tx.auth_info.address,
            tx.auth_info.key_index,
        )? {
            Some(verifying_key) => verifying_key,
            None => {
                return Err(InterLiquidSdkError::NotFound(anyhow!(
                    "verifying key not found"
                )));
            }
        };

        let verifying_key = ctx
            .type_registry()
            .unpack_trait(&VerifyingKeyTraitImpl, &verifying_key)?;

        Ok(())
    }
}
