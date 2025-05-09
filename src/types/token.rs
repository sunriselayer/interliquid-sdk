use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{InterLiquidSdkError, U256};

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Token {
    pub denom: String,
    pub amount: U256,
}

impl Token {
    pub fn new(denom: String, amount: U256) -> Self {
        Self { denom, amount }
    }

    pub(crate) fn validate_denom(denom: &str) -> Result<(), InterLiquidSdkError> {
        if denom.is_empty() {
            return Err(InterLiquidSdkError::InvalidDenom);
        }
        Ok(())
    }

    pub(crate) fn validate_amount(amount: &U256) -> Result<(), InterLiquidSdkError> {
        if amount.is_zero() {
            return Err(InterLiquidSdkError::ZeroAmount);
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), InterLiquidSdkError> {
        Self::validate_denom(&self.denom)?;
        Self::validate_amount(&self.amount)
    }

    pub fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError> {
        if self.denom != rhs.denom {
            return Err(InterLiquidSdkError::DenomMismatch);
        }

        Ok(Self {
            denom: self.denom,
            amount: self.amount.checked_add(&rhs.amount)?,
        })
    }

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
