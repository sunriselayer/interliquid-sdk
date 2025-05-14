use std::collections::BTreeSet;

use crate::sha2::{Digest, Sha256};
use anyhow::anyhow;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType, SerializableAny},
};

use super::{types::Account, AuthKeeper, AuthKeeperI};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgCreateAccount {
    pub creator: Address,
    pub address_seed: Vec<u8>,
    pub verifying_key: SerializableAny,
}

impl NamedSerializableType for MsgCreateAccount {
    fn type_name() -> &'static str {
        "Auth/MsgCreateAccount"
    }
}

impl Msg for MsgCreateAccount {
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.creator])
    }
}

impl AuthKeeper {
    pub fn msg_create_account(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgCreateAccount,
    ) -> Result<(), InterLiquidSdkError> {
        let mut hasher = Sha256::new();
        hasher.update(&msg.creator);
        hasher.update(&msg.address_seed);
        let address: Address = hasher.finalize().into();

        if let Some(_) = self.get_account(ctx, &address)? {
            return Err(InterLiquidSdkError::AlreadyExists(anyhow!(
                "account already exists for the calculated address"
            )));
        }

        let account = Account::new(address);
        self.set_account(ctx, &address, &account)?;

        self.add_verifying_key(ctx, &address, &msg.verifying_key)?;

        Ok(())
    }
}
