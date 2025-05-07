use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    state::StateManager,
    tx::Msg,
    types::{InterLiquidSdkError, NamedSerializableType, Token},
};

use super::BankKeeper;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MsgSend {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Token>,
}

impl NamedSerializableType for MsgSend {
    fn type_name() -> String {
        "Bank/MsgSend".to_string()
    }
}

impl Msg for MsgSend {}
