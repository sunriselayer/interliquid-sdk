use std::collections::BTreeSet;

use crate::sha2::{Digest, Sha256};
use anyhow::anyhow;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType, SerializableAny},
};

use super::{types::Account, AuthKeeper, AuthKeeperI};

/// Message to create a new account on the blockchain.
/// The account address is deterministically derived from the creator's address and a seed.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgCreateAccount {
    /// The address creating the new account.
    pub creator: Address,
    /// Seed data used to deterministically generate the new account address.
    pub address_seed: Vec<u8>,
    /// The initial verifying key for the new account.
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
    /// Handles the MsgCreateAccount message by creating a new account.
    /// The account address is derived by hashing the creator address and seed.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `msg` - The message containing account creation parameters
    /// 
    /// # Errors
    /// Returns an error if an account already exists at the calculated address.
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
