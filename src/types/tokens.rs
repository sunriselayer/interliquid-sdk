use std::collections::BTreeMap;

use crate::types::{InterLiquidSdkError, Token};

use super::U256;

/// A collection of tokens stored as a map from denomination to amount.
/// Uses BTreeMap to maintain sorted order of denominations.
pub type Tokens = BTreeMap<String, U256>;

/// Trait defining operations for token collections.
pub trait TokensI: Sized {
    /// Validates all tokens in the collection.
    fn validate(&self) -> Result<(), InterLiquidSdkError>;
    
    /// Performs checked addition of two token collections.
    fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError>;
    
    /// Performs checked subtraction of two token collections.
    fn checked_sub(self, rhs: &Self) -> Result<Self, InterLiquidSdkError>;
}

impl TokensI for Tokens {
    /// Validates all tokens in the collection.
    /// Checks that all denominations are non-empty and all amounts are non-zero.
    ///
    /// # Errors
    /// Returns an error if any token has an invalid denomination or zero amount
    fn validate(&self) -> Result<(), InterLiquidSdkError> {
        for (denom, amount) in self.iter() {
            Token::validate_denom(denom)?;
            Token::validate_amount(amount)?;
        }

        Ok(())
    }

    /// Performs checked addition of two token collections.
    /// Tokens with the same denomination are added together.
    /// Tokens that exist in only one collection are included in the result.
    ///
    /// # Arguments
    /// * `rhs` - The token collection to add
    ///
    /// # Returns
    /// A new token collection with the combined tokens
    ///
    /// # Errors
    /// Returns an error if validation fails or if addition overflows
    fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError> {
        self.validate()?;
        rhs.validate()?;

        let mut lhs = self;

        for (denom, amount_rhs) in rhs.iter() {
            let amount_lhs = lhs.get_mut(denom);

            if amount_lhs.is_none() {
                lhs.insert(denom.to_string(), amount_rhs.clone());
            } else {
                let new_amount = amount_lhs.unwrap().checked_add(&amount_rhs)?;
                lhs.insert(denom.to_string(), new_amount);
            }
        }

        Ok(lhs)
    }

    /// Performs checked subtraction of two token collections.
    /// Subtracts tokens in `rhs` from the corresponding tokens in `self`.
    ///
    /// # Arguments
    /// * `rhs` - The token collection to subtract
    ///
    /// # Returns
    /// A new token collection with the subtracted amounts
    ///
    /// # Errors
    /// Returns `Underflow` if a token in `rhs` doesn't exist in `self` or if subtraction underflows
    fn checked_sub(self, rhs: &Self) -> Result<Self, InterLiquidSdkError> {
        self.validate()?;
        rhs.validate()?;

        let mut lhs = self;

        for (denom, amount_rhs) in rhs.iter() {
            let amount_lhs = lhs.get_mut(denom);

            if amount_lhs.is_none() {
                return Err(InterLiquidSdkError::Underflow);
            }

            let new_amount = amount_lhs.unwrap().checked_sub(&amount_rhs)?;

            lhs.insert(denom.to_string(), new_amount);
        }

        Ok(lhs)
    }
}
