use std::collections::BTreeMap;

use crate::types::{InterLiquidSdkError, Token};

use super::U256;

pub type Tokens = BTreeMap<String, U256>;

pub trait TokensI: Sized {
    fn validate(&self) -> Result<(), InterLiquidSdkError>;
    fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError>;
    fn checked_sub(self, rhs: &Self) -> Result<Self, InterLiquidSdkError>;
}

impl TokensI for Tokens {
    fn validate(&self) -> Result<(), InterLiquidSdkError> {
        for (denom, amount) in self.iter() {
            Token::validate_denom(denom)?;
            Token::validate_amount(amount)?;
        }

        Ok(())
    }

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
