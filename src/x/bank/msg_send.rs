use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::Context,
    tx::Msg,
    types::{InterLiquidSdkError, NamedSerializableType, Token, Tokens},
};

use super::{BankKeeper, BankKeeperI};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MsgSend {
    pub from_address: String,
    pub to_address: String,
    pub tokens: Tokens,
}

impl NamedSerializableType for MsgSend {
    fn type_name() -> &'static str {
        "Bank/MsgSend"
    }
}

impl Msg for MsgSend {}

impl BankKeeper {
    pub fn msg_send(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgSend,
    ) -> Result<(), InterLiquidSdkError> {
        self.send(ctx, &msg.from_address, &msg.to_address, &msg.tokens)
    }
}
