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
            let multiplier = ten.powi(diff)?;
            lhs.value = lhs.value.checked_mul(&multiplier)?;
            lhs.precision = max_precision;
        }

        if rhs.precision < max_precision {
            let diff = max_precision - rhs.precision;
            let ten: U256 = 10u64.into();
            let multiplier = ten.powi(diff)?;
            rhs.value = rhs.value.checked_mul(&multiplier)?;
            rhs.precision = max_precision;
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
    pub fn checked_add(self, rhs: Decimal) -> Result<Decimal, InterLiquidSdkError> {
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
    pub fn checked_sub(self, rhs: Decimal) -> Result<Decimal, InterLiquidSdkError> {
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
            return Ok(Self::new(new_value, 0));
        }

        return Ok(Self::new(new_value, new_precision as u8));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_decimal() {
        let decimal = Decimal::new(1000u64.into(), 2);
        assert_eq!(decimal.value, 1000u64.into());
        assert_eq!(decimal.precision, 2);
    }

    #[test]
    fn test_is_zero() {
        let zero_decimal = Decimal::new(0u64.into(), 2);
        assert!(zero_decimal.is_zero());

        let non_zero_decimal = Decimal::new(100u64.into(), 2);
        assert!(!non_zero_decimal.is_zero());
    }

    #[test]
    fn test_align_precision_no_change() {
        let lhs = Decimal::new(100u64.into(), 2);
        let rhs = Decimal::new(200u64.into(), 2);
        
        let (aligned_lhs, aligned_rhs) = Decimal::align_precision(lhs, rhs).unwrap();
        
        assert_eq!(aligned_lhs.value, 100u64.into());
        assert_eq!(aligned_rhs.value, 200u64.into());
        assert_eq!(aligned_lhs.precision, 2);
        assert_eq!(aligned_rhs.precision, 2);
    }

    #[test]
    fn test_align_precision_different() {
        let lhs = Decimal::new(100u64.into(), 2); // 1.00
        let rhs = Decimal::new(50u64.into(), 4);  // 0.0050
        
        let (aligned_lhs, aligned_rhs) = Decimal::align_precision(lhs, rhs).unwrap();
        
        assert_eq!(aligned_lhs.value, 10000u64.into()); // 100 * 100 = 10000
        assert_eq!(aligned_rhs.value, 50u64.into());
        assert_eq!(aligned_lhs.precision, 4);
        assert_eq!(aligned_rhs.precision, 4);
    }

    #[test]
    fn test_checked_add_same_precision() {
        let a = Decimal::new(100u64.into(), 2); // 1.00
        let b = Decimal::new(200u64.into(), 2); // 2.00
        
        let result = a.checked_add(b).unwrap();
        assert_eq!(result.value, 300u64.into());
        assert_eq!(result.precision, 2);
    }

    #[test]
    fn test_checked_add_different_precision() {
        let a = Decimal::new(100u64.into(), 2); // 1.00
        let b = Decimal::new(50u64.into(), 4);  // 0.0050
        
        let result = a.checked_add(b).unwrap();
        assert_eq!(result.value, 10050u64.into()); // 10000 + 50
        assert_eq!(result.precision, 4);
    }

    #[test]
    fn test_checked_sub_same_precision() {
        let a = Decimal::new(300u64.into(), 2); // 3.00
        let b = Decimal::new(100u64.into(), 2); // 1.00
        
        let result = a.checked_sub(b).unwrap();
        assert_eq!(result.value, 200u64.into());
        assert_eq!(result.precision, 2);
    }

    #[test]
    fn test_checked_sub_underflow() {
        let a = Decimal::new(100u64.into(), 2); // 1.00
        let b = Decimal::new(200u64.into(), 2); // 2.00
        
        let result = a.checked_sub(b);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Underflow) => (),
            _ => panic!("Expected Underflow error"),
        }
    }

    #[test]
    fn test_checked_mul() {
        let a = Decimal::new(100u64.into(), 2); // 1.00
        let b = Decimal::new(200u64.into(), 2); // 2.00
        
        let result = a.checked_mul(&b).unwrap();
        assert_eq!(result.value, 20000u64.into()); // 100 * 200
        assert_eq!(result.precision, 4); // 2 + 2
    }

    #[test]
    fn test_checked_mul_zero() {
        let a = Decimal::new(100u64.into(), 2); // 1.00
        let b = Decimal::new(0u64.into(), 2);   // 0.00
        
        let result = a.checked_mul(&b).unwrap();
        assert_eq!(result.value, 0u64.into());
        assert_eq!(result.precision, 4);
    }

    #[test]
    fn test_display() {
        // Skip Display test since Display trait is not implemented
        // The Decimal type uses Debug for formatting instead
    }

    #[test]
    fn test_debug() {
        let decimal = Decimal::new(12345u64.into(), 3);
        let debug_str = format!("{:?}", decimal);
        assert!(debug_str.contains("Decimal"));
        // Debug output will contain the U256 value representation, not the plain number
        assert!(debug_str.contains("value"));
        assert!(debug_str.contains("precision"));
    }

    #[test]
    fn test_align_precision_overflow() {
        // Create a large value that would overflow when multiplied
        // Create a large value that would overflow when multiplied
        let bytes = vec![0xFF; 32];
        let max_inner = crate::crypto_bigint::U256::from_be_slice(&bytes);
        let max_val = U256::new(max_inner);
        let lhs = Decimal::new(max_val, 0);
        let rhs = Decimal::new(1u64.into(), 100); // Very high precision difference
        
        let result = Decimal::align_precision(lhs, rhs);
        assert!(result.is_err());
    }


    #[test]
    fn test_checked_div() {
        let d1 = Decimal::new(1000u64.into(), 3); // 1.000
        let d2 = Decimal::new(250u64.into(), 2);  // 2.50
        
        let result = d1.checked_div(&d2).unwrap();
        
        assert_eq!(result.value, 4u64.into()); // 1.000 / 2.50 = 0.4
        assert_eq!(result.precision, 1); // 3 - 2
    }

    #[test]
    fn test_checked_div_by_zero() {
        let d1 = Decimal::new(100u64.into(), 2);
        let d2 = Decimal::new(0u64.into(), 2);
        
        let result = d1.checked_div(&d2);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_checked_div_negative_precision() {
        let d1 = Decimal::new(100u64.into(), 2);  // 1.00
        let d2 = Decimal::new(10u64.into(), 4);   // 0.0010
        
        let result = d1.checked_div(&d2).unwrap();
        
        // 1.00 / 0.0010 = 1000, but precision is 2-4=-2
        // 100 / 10 = 10, then multiply by 10^2 = 100, giving us 1000
        assert_eq!(result.value, 1000u64.into());
        assert_eq!(result.precision, 0);
    }

    #[test]
    fn test_decimal_equality() {
        let d1 = Decimal::new(100u64.into(), 2);
        let d2 = Decimal::new(100u64.into(), 2);
        let d3 = Decimal::new(100u64.into(), 3);
        
        assert_eq!(d1, d2);
        assert_ne!(d1, d3); // Different precision means different decimal
    }
}
