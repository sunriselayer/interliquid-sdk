use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{InterLiquidSdkError, U256};

#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Decimal {
    value: U256,
    precision: u8,
}

impl Decimal {
    pub fn new(value: U256, precision: u8) -> Self {
        Self { value, precision }
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn checked_add(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        todo!()
    }

    pub fn checked_sub(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        todo!()
    }

    pub fn checked_mul(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        todo!()
    }

    pub fn checked_div(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        todo!()
    }
}
