use crate::{
    state::StateManager,
    tx::Message,
    types::{InterLiquidSdkError, Token},
};

pub struct MsgSend {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Token>,
}

impl<S: StateManager> Message<S> for MsgSend {
    fn apply(&self, state: &mut S) -> Result<(), InterLiquidSdkError> {
        Ok(())
    }
}
