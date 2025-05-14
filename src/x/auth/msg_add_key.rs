use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType, SerializableAny},
};

use super::{AuthKeeper, AuthKeeperI};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgAddKey {
    pub address: Address,
    pub verifying_key: SerializableAny,
}

impl NamedSerializableType for MsgAddKey {
    fn type_name() -> &'static str {
        "Auth/MsgAddKey"
    }
}

impl Msg for MsgAddKey {
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.address])
    }
}

impl<'a> AuthKeeper<'a> {
    pub fn msg_add_key(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgAddKey,
    ) -> Result<(), InterLiquidSdkError> {
        self.add_verifying_key(ctx, &msg.address, &msg.verifying_key)
    }
}
