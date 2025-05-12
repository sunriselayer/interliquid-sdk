use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::Context,
    tx::Msg,
    types::{Address, InterLiquidSdkError, NamedSerializableType, Tokens},
};

use super::{BankKeeper, BankKeeperI};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MsgSend {
    pub from_address: Address,
    pub to_address: Address,
    pub tokens: Tokens,
}

impl NamedSerializableType for MsgSend {
    fn type_name() -> &'static str {
        "Bank/MsgSend"
    }
}

impl Msg for MsgSend {
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.from_address])
    }
}

impl BankKeeper {
    pub fn msg_send(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgSend,
    ) -> Result<(), InterLiquidSdkError> {
        self.send(ctx, &msg.from_address, &msg.to_address, &msg.tokens)
    }
}
