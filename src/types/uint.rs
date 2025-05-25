use core::clone::Clone;

use crate::crypto_bigint::prelude::*;
use crate::crypto_bigint::{Encoding, U256 as U256Lib};
use borsh::{BorshDeserialize, BorshSerialize};

use super::InterLiquidSdkError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct U256(U256Lib);

impl U256 {
    pub fn new(value: U256Lib) -> Self {
        Self(value)
    }

    pub fn inner_value(&self) -> &U256Lib {
        &self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero().into()
    }

    pub fn checked_add(&self, rhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_add(&rhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::Overflow);
        }
        Ok(U256(option.unwrap()))
    }

    pub fn checked_sub(&self, lhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_sub(&lhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::Underflow);
        }
        Ok(U256(option.unwrap()))
    }

    pub fn checked_mul(&self, rhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_mul(&rhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::Overflow);
        }
        Ok(U256(option.unwrap()))
    }

    pub fn checked_div(&self, rhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_div(&rhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::DivisionByZero);
        }
        Ok(U256(option.unwrap()))
    }

    pub fn powi(&self, exponent: u8) -> Result<U256, InterLiquidSdkError> {
        if exponent == 0 {
            return Ok(1.into());
        } else if exponent == 1 {
            return Ok(self.clone());
        } else {
            let mut val = self.checked_mul(self)?;

            for _ in 2..exponent {
                val = val.checked_mul(self)?;
            }

            return val;
        }
    }
}

impl BorshSerialize for U256 {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.to_le_bytes().serialize(writer)
    }
}

impl BorshDeserialize for U256 {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let amount_bytes = <U256Lib as Encoding>::Repr::deserialize_reader(reader)?;
        let amount = U256Lib::from_le_bytes(amount_bytes);

        Ok(U256(amount))
    }
}

impl From<U256Lib> for U256 {
    fn from(value: U256Lib) -> Self {
        U256(value)
    }
}

impl From<U256> for U256Lib {
    fn from(value: U256) -> Self {
        value.0
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        U256(U256Lib::from_u64(value))
    }
}
