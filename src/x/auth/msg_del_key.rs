use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType},
};

use super::{AuthKeeper, AuthKeeperI};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgDelKey {
    pub address: Address,
    pub key_index: u64,
}

impl NamedSerializableType for MsgDelKey {
    fn type_name() -> &'static str {
        "Auth/MsgDelKey"
    }
}

impl Msg for MsgDelKey {
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.address])
    }
}

impl AuthKeeper {
    pub fn msg_del_key(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgDelKey,
    ) -> Result<(), InterLiquidSdkError> {
        self.del_verifying_key(ctx, &msg.address, msg.key_index)
    }
}
