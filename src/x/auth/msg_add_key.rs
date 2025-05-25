use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType, SerializableAny},
};

use super::{AuthKeeper, AuthKeeperI};

/// Message to add a new verifying key to an existing account.
/// This allows an account to have multiple keys for authentication.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgAddKey {
    /// The address of the account to add the key to.
    pub address: Address,
    /// The verifying key to add to the account.
    pub verifying_key: SerializableAny,
}

impl NamedSerializableType for MsgAddKey {
    const TYPE_NAME: &'static str = "Auth/MsgAddKey";
}

impl Msg for MsgAddKey {
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.address])
    }
}

impl AuthKeeper {
    /// Handles the MsgAddKey message by adding a new verifying key to the specified account.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `msg` - The message containing the address and verifying key to add
    pub fn msg_add_key(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgAddKey,
    ) -> Result<(), InterLiquidSdkError> {
        self.add_verifying_key(ctx, &msg.address, &msg.verifying_key)
    }
}
