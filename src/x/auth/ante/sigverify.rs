use anyhow::anyhow;

use crate::{
    core::{MsgRegistry, SdkContext},
    tx::TxAnteHandler,
    types::InterLiquidSdkError,
    x::{
        auth::{
            ante::{SignDoc, StdTx},
            keeper::{AuthKeeper, AuthKeeperI},
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

impl TxAnteHandler<StdTx> for SigVerifyAnteHandler {
    fn handle(
        &self,
        ctx: &mut SdkContext,
        _msg_registry: &MsgRegistry,
        tx: &StdTx,
    ) -> Result<(), InterLiquidSdkError> {
        for (address, auth_info) in tx.auth_info.iter() {
            let mut account = match self.auth_keeper.get_account(ctx, address)? {
                Some(account) => account,
                None => {
                    return Err(InterLiquidSdkError::NotFound(anyhow!("account not found")));
                }
            };

            if account.nonce != auth_info.nonce {
                return Err(InterLiquidSdkError::InvalidRequest(anyhow!(
                    "nonce mismatch"
                )));
            }

            account.nonce += 1;
            self.auth_keeper.set_account(ctx, address, &account)?;

            let verifying_key =
                match self
                    .auth_keeper
                    .get_verifying_key(ctx, address, auth_info.key_index)?
                {
                    Some(verifying_key) => verifying_key,
                    None => {
                        return Err(InterLiquidSdkError::NotFound(anyhow!(
                            "verifying key not found"
                        )));
                    }
                };

            let verifying_key = self.crypto_keeper.unpack_verifying_key(&verifying_key)?;

            let sign_doc = SignDoc::new(&tx.body, &tx.auth_info, ctx.chain_id());

            let signature = tx
                .signature
                .get(address)
                .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                    "signature not found for address"
                )))?;

            verifying_key.verify(&sign_doc.to_bytes()?, &signature)?;
        }

        Ok(())
    }
}
