use std::marker::PhantomData;

use super::keys::{BALANCES, BANK};

use crate::{
    state::StateManager,
    types::{InterLiquidSdkError, Token},
    utils::IndexedMap,
};

pub trait BankKeeperI<S: StateManager> {
    fn get_balance(
        &self,
        state: &mut S,
        address: &str,
        denom: &str,
    ) -> Result<Option<Token>, InterLiquidSdkError>;

    fn send(
        &self,
        state: &mut S,
        from: &str,
        to: &str,
        amount: Vec<Token>,
    ) -> Result<(), InterLiquidSdkError>;
}

pub struct BankKeeper<S: StateManager> {
    balances: IndexedMap<(String, String), Token>,
    phantom: PhantomData<S>,
}

impl<S: StateManager> BankKeeper<S> {
    pub fn new() -> Self {
        Self {
            balances: IndexedMap::new([BANK, BALANCES]),
            phantom: PhantomData,
        }
    }
}
impl<S: StateManager> BankKeeperI<S> for BankKeeper<S> {
    fn get_balance(
        &self,
        state: &mut S,
        address: &str,
        denom: &str,
    ) -> Result<Option<Token>, InterLiquidSdkError> {
        let balance = self
            .balances
            .get(state, &(address.to_string(), denom.to_string()))?;

        Ok(balance)
    }

    fn send(
        &self,
        state: &mut S,
        from: &str,
        to: &str,
        amount: Vec<Token>,
    ) -> Result<(), InterLiquidSdkError> {
        Ok(())
    }
}
