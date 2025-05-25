use core::cmp::Ord;

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

    fn align_precision(
        mut lhs: Decimal,
        mut rhs: Decimal,
    ) -> Result<(Decimal, Decimal), InterLiquidSdkError> {
        let max_precision = lhs.precision.max(rhs.precision);

        if lhs.precision < max_precision {
            let diff = max_precision - lhs.precision;
            let ten: U256 = 10u64.into();
            lhs = lhs.checked_mul(&ten.powi(diff)?)?;
        }

        if rhs.precision < max_precision {
            let diff = max_precision - rhs.precision;
            let ten: U256 = 10u64.into();
            rhs = rhs.checked_mul(&ten.powi(diff)?)?;
        }
    }

    pub fn checked_add(mut self, mut rhs: Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let (lhs, rhs) = Self::align_precision(self, rhs)?;

        Ok(Self::new(lhs.value.checked_add(&rhs.value)?, lhs.precision))
    }

    pub fn checked_sub(mut self, mut rhs: Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let (lhs, rhs) = Self::align_precision(self, rhs)?;

        Ok(Self::new(lhs.value.checked_sub(&rhs.value)?, lhs.precision))
    }

    pub fn checked_mul(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let new_value = self.value.checked_mul(&rhs.value)?;
        let new_precision = self.precision + rhs.precision;

        return Ok(Self::new(new_value, new_precision));
    }

    pub fn checked_div(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let mut new_value = self.value.checked_div(&rhs.value)?;
        let new_precision = self.precision as i16 - rhs.precision as i16;

        if new_precision < 0 {
            let ten: U256 = 10u64.into();
            new_value = new_value.checked_mul(&ten.powi(new_precision.abs())?)?;
        }

        return Ok(Self::new(new_value, new_precision));
    }
}
