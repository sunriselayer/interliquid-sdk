mod keys;
mod msg_send;

use std::marker::PhantomData;

use borsh::BorshDeserialize;
use keys::{BALANCES, BANK, DENOMS};
pub use msg_send::*;

use crate::{
    state::StateManager,
    types::{InterLiquidSdkError, Token},
    utils::{key, KeySerializable},
};

pub struct Bank<S: StateManager> {
    phantom: PhantomData<S>,
}

impl<S: StateManager> Bank<S> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn get_balance(
        state: &mut S,
        address: &str,
        denom: &str,
    ) -> Result<Option<Token>, InterLiquidSdkError> {
        let key = key([BANK, BALANCES, &address.key(), DENOMS], &denom.key());
        let balance = state.get(&key)?;
        if let Some(balance) = balance {
            let token = Token::deserialize(&mut &balance[..])?;
            return Ok(Some(token));
        }
        Ok(None)
    }

    pub fn send(
        state: &mut S,
        from: &str,
        to: &str,
        amount: Vec<Token>,
    ) -> Result<(), InterLiquidSdkError> {
        Ok(())
    }
}
