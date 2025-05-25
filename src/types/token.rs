use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{InterLiquidSdkError, U256};

/// Represents a token with a denomination and amount.
/// Used for handling different types of tokens/currencies in the system.
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Token {
    /// The denomination or identifier of the token (e.g., "uatom", "usdc")
    pub denom: String,
    /// The amount of the token
    pub amount: U256,
}

impl Token {
    /// Creates a new Token instance.
    ///
    /// # Arguments
    /// * `denom` - The denomination of the token
    /// * `amount` - The amount of the token
    pub fn new(denom: String, amount: U256) -> Self {
        Self { denom, amount }
    }

    /// Validates that a denomination is valid (non-empty).
    ///
    /// # Arguments
    /// * `denom` - The denomination to validate
    ///
    /// # Errors
    /// Returns `InvalidDenom` if the denomination is empty
    pub(crate) fn validate_denom(denom: &str) -> Result<(), InterLiquidSdkError> {
        if denom.is_empty() {
            return Err(InterLiquidSdkError::InvalidDenom);
        }
        Ok(())
    }

    /// Validates that an amount is valid (non-zero).
    ///
    /// # Arguments
    /// * `amount` - The amount to validate
    ///
    /// # Errors
    /// Returns `ZeroAmount` if the amount is zero
    pub(crate) fn validate_amount(amount: &U256) -> Result<(), InterLiquidSdkError> {
        if amount.is_zero() {
            return Err(InterLiquidSdkError::ZeroAmount);
        }
        Ok(())
    }

    /// Validates the token by checking both denomination and amount.
    ///
    /// # Errors
    /// Returns an error if either the denomination is empty or the amount is zero
    pub fn validate(&self) -> Result<(), InterLiquidSdkError> {
        Self::validate_denom(&self.denom)?;
        Self::validate_amount(&self.amount)
    }

    /// Performs checked addition of two tokens.
    ///
    /// # Arguments
    /// * `rhs` - The token to add
    ///
    /// # Returns
    /// A new token with the sum of the amounts
    ///
    /// # Errors
    /// Returns `DenomMismatch` if the denominations don't match, or `Overflow` if the addition overflows
    pub fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError> {
        if self.denom != rhs.denom {
            return Err(InterLiquidSdkError::DenomMismatch);
        }

        Ok(Self {
            denom: self.denom,
            amount: self.amount.checked_add(&rhs.amount)?,
        })
    }

    /// Performs checked subtraction of two tokens.
    ///
    /// # Arguments
    /// * `lhs` - The token to subtract
    ///
    /// # Returns
    /// A new token with the difference of the amounts
    ///
    /// # Errors
    /// Returns `DenomMismatch` if the denominations don't match, or `Underflow` if the subtraction underflows
    pub fn checked_sub(self, lhs: &Self) -> Result<Self, InterLiquidSdkError> {
        if self.denom != lhs.denom {
            return Err(InterLiquidSdkError::DenomMismatch);
        }

        Ok(Self {
            denom: self.denom,
            amount: self.amount.checked_sub(&lhs.amount)?,
        })
    }
}
