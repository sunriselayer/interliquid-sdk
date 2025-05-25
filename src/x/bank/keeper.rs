use super::keys::{BALANCES, BANK};

use crate::{
    core::Context,
    types::{Address, InterLiquidSdkError, Tokens, TokensI, U256},
    utils::{IndexedMap, KeyPrefixTupleOne},
};

/// Interface for bank module keeper functionality.
/// Defines the core banking operations for managing account balances and token transfers.
pub trait BankKeeperI: Send + Sync {
    /// Retrieves the balance of a specific denomination for an address.
    ///
    /// # Arguments
    /// * `ctx` - The context for state access
    /// * `address` - The account address to query
    /// * `denom` - The token denomination to query
    ///
    /// # Returns
    /// * `Ok(Some(U256))` - The balance amount if found
    /// * `Ok(None)` - If no balance exists for the denomination
    /// * `Err` - If an error occurs during state access
    fn get_balance(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        denom: &str,
    ) -> Result<Option<U256>, InterLiquidSdkError>;

    /// Retrieves all token balances for an address.
    ///
    /// # Arguments
    /// * `ctx` - The context for state access
    /// * `address` - The account address to query
    ///
    /// # Returns
    /// * `Ok(Tokens)` - A collection of all token balances for the address
    /// * `Err` - If an error occurs during state access
    fn get_all_balances(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
    ) -> Result<Tokens, InterLiquidSdkError>;

    /// Transfers tokens from one address to another.
    ///
    /// # Arguments
    /// * `ctx` - The context for state access
    /// * `from` - The sender's address
    /// * `to` - The recipient's address
    /// * `tokens` - The collection of tokens to transfer
    ///
    /// # Returns
    /// * `Ok(())` - If the transfer succeeds
    /// * `Err` - If insufficient balance or other error occurs
    fn send(
        &self,
        ctx: &mut dyn Context,
        from: &Address,
        to: &Address,
        tokens: &Tokens,
    ) -> Result<(), InterLiquidSdkError>;
}

/// The bank module keeper responsible for managing account balances.
/// Stores balances as a mapping from (address, denomination) to amount.
pub struct BankKeeper {
    /// Indexed map storing balances with composite key of (address, denomination)
    balances: IndexedMap<(Address, String), U256>,
}

impl BankKeeper {
    /// Creates a new instance of BankKeeper.
    ///
    /// # Returns
    /// A new BankKeeper with initialized balance storage
    pub fn new() -> Self {
        Self {
            balances: IndexedMap::new([BANK, BALANCES]),
        }
    }

    /// Adds the specified amount to an account's balance for a given denomination.
    ///
    /// # Arguments
    /// * `ctx` - The context for state access
    /// * `address` - The account address to credit
    /// * `denom` - The token denomination
    /// * `amount` - The amount to add
    ///
    /// # Returns
    /// * `Ok(())` - If the balance is successfully updated
    /// * `Err` - If overflow or state access error occurs
    fn add_balance(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        denom: &str,
        amount: &U256,
    ) -> Result<(), InterLiquidSdkError> {
        let balance = self
            .balances
            .get(ctx.state_manager_mut(), (address, denom))?;

        let new_balance = match balance {
            Some(balance) => balance.checked_add(amount)?,
            None => amount.clone(),
        };

        self.balances
            .set(ctx.state_manager_mut(), (address, denom), &new_balance)?;

        Ok(())
    }

    /// Subtracts the specified amount from an account's balance for a given denomination.
    ///
    /// # Arguments
    /// * `ctx` - The context for state access
    /// * `address` - The account address to debit
    /// * `denom` - The token denomination
    /// * `amount` - The amount to subtract
    ///
    /// # Returns
    /// * `Ok(())` - If the balance is successfully updated
    /// * `Err(Underflow)` - If insufficient balance
    /// * `Err` - If other state access error occurs
    fn sub_balance(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        denom: &str,
        amount: &U256,
    ) -> Result<(), InterLiquidSdkError> {
        let balance = self
            .balances
            .get(ctx.state_manager_mut(), (address, denom))?;

        let new_balance = match balance {
            Some(balance) => balance.checked_sub(amount)?,
            None => {
                return Err(InterLiquidSdkError::Underflow);
            }
        };

        self.balances
            .set(ctx.state_manager_mut(), (address, denom), &new_balance)?;

        Ok(())
    }
}

impl BankKeeperI for BankKeeper {
    fn get_balance(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        denom: &str,
    ) -> Result<Option<U256>, InterLiquidSdkError> {
        let balance = self
            .balances
            .get(ctx.state_manager_mut(), (address, denom))?;

        Ok(balance)
    }

    fn get_all_balances(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
    ) -> Result<Tokens, InterLiquidSdkError> {
        let mut tokens = Tokens::new();

        for result in self.balances.iter(
            ctx.state_manager_mut(),
            KeyPrefixTupleOne::<Address, String>::new(address),
        ) {
            let ((_address, denom), amount) = result?;
            tokens.insert(denom, amount);
        }

        Ok(tokens)
    }

    fn send(
        &self,
        ctx: &mut dyn Context,
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
