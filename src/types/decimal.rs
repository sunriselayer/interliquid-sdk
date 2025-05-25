use core::cmp::Ord;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{InterLiquidSdkError, U256};

/// Represents a decimal number with arbitrary precision.
/// The decimal is stored as a U256 value with a specified precision (number of decimal places).
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Decimal {
    /// The raw integer value of the decimal
    value: U256,
    /// The number of decimal places (precision)
    precision: u8,
}

impl Decimal {
    /// Creates a new Decimal with the specified value and precision.
    ///
    /// # Arguments
    /// * `value` - The raw integer value
    /// * `precision` - The number of decimal places
    pub fn new(value: U256, precision: u8) -> Self {
        Self { value, precision }
    }

    /// Checks if the decimal value is zero.
    ///
    /// # Returns
    /// true if the value is zero, false otherwise
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// Aligns two decimals to have the same precision by scaling up the one with lower precision.
    ///
    /// # Arguments
    /// * `lhs` - The left-hand side decimal
    /// * `rhs` - The right-hand side decimal
    ///
    /// # Returns
    /// A tuple of the two decimals with aligned precision
    ///
    /// # Errors
    /// Returns an error if scaling causes overflow
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

        Ok((lhs, rhs))
    }

    /// Performs checked addition of two decimals.
    ///
    /// # Arguments
    /// * `rhs` - The decimal to add
    ///
    /// # Returns
    /// The sum of the two decimals with aligned precision
    ///
    /// # Errors
    /// Returns an error if the addition overflows
    pub fn checked_add(mut self, mut rhs: Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let (lhs, rhs) = Self::align_precision(self, rhs)?;

        Ok(Self::new(lhs.value.checked_add(&rhs.value)?, lhs.precision))
    }

    /// Performs checked subtraction of two decimals.
    ///
    /// # Arguments
    /// * `rhs` - The decimal to subtract
    ///
    /// # Returns
    /// The difference of the two decimals with aligned precision
    ///
    /// # Errors
    /// Returns an error if the subtraction underflows
    pub fn checked_sub(mut self, mut rhs: Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let (lhs, rhs) = Self::align_precision(self, rhs)?;

        Ok(Self::new(lhs.value.checked_sub(&rhs.value)?, lhs.precision))
    }

    /// Performs checked multiplication of two decimals.
    ///
    /// # Arguments
    /// * `rhs` - The decimal to multiply by
    ///
    /// # Returns
    /// The product of the two decimals with combined precision
    ///
    /// # Errors
    /// Returns an error if the multiplication overflows
    pub fn checked_mul(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let new_value = self.value.checked_mul(&rhs.value)?;
        let new_precision = self.precision + rhs.precision;

        return Ok(Self::new(new_value, new_precision));
    }

    /// Performs checked division of two decimals.
    ///
    /// # Arguments
    /// * `rhs` - The decimal to divide by
    ///
    /// # Returns
    /// The quotient of the two decimals with adjusted precision
    ///
    /// # Errors
    /// Returns an error if dividing by zero or if the operation overflows
    pub fn checked_div(&self, rhs: &Decimal) -> Result<Decimal, InterLiquidSdkError> {
        let mut new_value = self.value.checked_div(&rhs.value)?;
        let new_precision = self.precision as i16 - rhs.precision as i16;

        if new_precision < 0 {
            let ten: U256 = 10u64.into();
            new_value = new_value.checked_mul(&ten.powi(new_precision.abs() as u8)?)?;
        }

        return Ok(Self::new(new_value, new_precision as u8));
    }
}
