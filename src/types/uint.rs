use borsh::{BorshDeserialize, BorshSerialize};
use crypto_bigint::prelude::*;
use crypto_bigint::{Encoding, U256 as U256Lib};

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
