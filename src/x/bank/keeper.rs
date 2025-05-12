use std::marker::PhantomData;

use super::keys::{BALANCES, BANK};

use crate::{
    core::Context,
    types::{Address, InterLiquidSdkError, Tokens, TokensI, U256},
    utils::{IndexedMap, KeyPrefixTupleOne},
};

pub trait BankKeeperI<C: Context>: Send {
    fn get_balance(
        &self,
        ctx: &mut C,
        address: &Address,
        denom: &str,
    ) -> Result<Option<U256>, InterLiquidSdkError>;

    fn get_all_balances(
        &self,
        ctx: &mut C,
        address: &Address,
    ) -> Result<Tokens, InterLiquidSdkError>;

    fn send(
        &self,
        ctx: &mut C,
        from: &Address,
        to: &Address,
        tokens: &Tokens,
    ) -> Result<(), InterLiquidSdkError>;
}

pub struct BankKeeper<C: Context> {
    balances: IndexedMap<(Address, String), U256>,
    phantom: PhantomData<C>,
}

impl<C: Context> BankKeeper<C> {
    pub fn new() -> Self {
        Self {
            balances: IndexedMap::new([BANK, BALANCES]),
            phantom: PhantomData,
        }
    }

    fn add_balance(
        &self,
        ctx: &mut C,
        address: &Address,
        denom: &str,
        amount: &U256,
    ) -> Result<(), InterLiquidSdkError> {
        let balance = self.balances.get(ctx.state_manager(), (address, denom))?;

        let new_balance = match balance {
            Some(balance) => balance.checked_add(amount)?,
            None => amount.clone(),
        };

        self.balances
            .set(ctx.state_manager(), (address, denom), &new_balance)?;

        Ok(())
    }

    fn sub_balance(
        &self,
        ctx: &mut C,
        address: &Address,
        denom: &str,
        amount: &U256,
    ) -> Result<(), InterLiquidSdkError> {
        let balance = self.balances.get(ctx.state_manager(), (address, denom))?;

        let new_balance = match balance {
            Some(balance) => balance.checked_sub(amount)?,
            None => {
                return Err(InterLiquidSdkError::Underflow);
            }
        };

        self.balances
            .set(ctx.state_manager(), (address, denom), &new_balance)?;

        Ok(())
    }
}

impl<C: Context> BankKeeperI<C> for BankKeeper<C> {
    fn get_balance(
        &self,
        ctx: &mut C,
        address: &Address,
        denom: &str,
    ) -> Result<Option<U256>, InterLiquidSdkError> {
        let balance = self.balances.get(ctx.state_manager(), (address, denom))?;

        Ok(balance)
    }

    fn get_all_balances(
        &self,
        ctx: &mut C,
        address: &Address,
    ) -> Result<Tokens, InterLiquidSdkError> {
        let mut tokens = Tokens::new();

        for result in self.balances.iter(
            ctx.state_manager(),
            KeyPrefixTupleOne::<Address, String>::new(address),
        ) {
            let ((_address, denom), amount) = result?;
            tokens.insert(denom, amount);
        }

        Ok(tokens)
    }

    fn send(
        &self,
        ctx: &mut C,
        from: &Address,
        to: &Address,
        tokens: &Tokens,
    ) -> Result<(), InterLiquidSdkError> {
        tokens.validate()?;
        for (denom, amount) in tokens {
            self.sub_balance(ctx, from, denom, amount)?;
            self.add_balance(ctx, to, denom, amount)?;
        }

        Ok(())
    }
}
