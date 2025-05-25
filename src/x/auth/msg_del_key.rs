use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType},
};

use super::{AuthKeeper, AuthKeeperI};

/// Message to delete a verifying key from an account.
/// This removes a specific key by its index from the account's key set.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgDelKey {
    /// The address of the account to delete the key from.
    pub address: Address,
    /// The index of the key to delete.
    pub key_index: u64,
}

impl NamedSerializableType for MsgDelKey {
    const TYPE_NAME: &'static str = "Auth/MsgDelKey";
}

impl Msg for MsgDelKey {
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.address])
    }
}

impl AuthKeeper {
    /// Handles the MsgDelKey message by removing a verifying key from the specified account.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `msg` - The message containing the address and key index to delete
    pub fn msg_del_key(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgDelKey,
    ) -> Result<(), InterLiquidSdkError> {
        self.del_verifying_key(ctx, &msg.address, msg.key_index)
    }
}
