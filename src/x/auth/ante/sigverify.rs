use std::any::Any;

use anyhow::anyhow;

use crate::{
    core::Context,
    tx::{Tx, TxAnteHandler},
    types::InterLiquidSdkError,
    x::{
        auth::{
            keeper::{AuthKeeper, AuthKeeperI},
            tx::{SignDoc, StdTx},
        },
        crypto::keeper::{CryptoKeeper, CryptoKeeperI},
    },
};

pub struct SigVerifyAnteHandler {
    auth_keeper: AuthKeeper,
    crypto_keeper: CryptoKeeper,
}

impl SigVerifyAnteHandler {
    pub fn new(auth_keeper: AuthKeeper, crypto_keeper: CryptoKeeper) -> Self {
        Self {
            auth_keeper,
            crypto_keeper,
        }
    }
}

impl TxAnteHandler for SigVerifyAnteHandler {
    fn handle(&self, ctx: &mut dyn Context, tx: &Box<dyn Tx>) -> Result<(), InterLiquidSdkError> {
        let tx = tx.downcast::<StdTx>().unwrap();

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

        let verifying_key = self
            .crypto_keeper
            .unpack_verifying_key(ctx, &verifying_key)?;

        let sign_doc = SignDoc::new(&tx.body, &tx.auth_info, ctx.chain_id());

        verifying_key.verify(&sign_doc.to_bytes()?, &tx.signature)?;

        Ok(())
    }
}
