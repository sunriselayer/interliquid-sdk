use core::clone::Clone;

use crate::crypto_bigint::prelude::*;
use crate::crypto_bigint::{Encoding, U256 as U256Lib};
use borsh::{BorshDeserialize, BorshSerialize};

use super::InterLiquidSdkError;

/// A 256-bit unsigned integer type.
/// This is a wrapper around the crypto_bigint U256 type that provides
/// checked arithmetic operations and serialization support.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct U256(U256Lib);

impl U256 {
    /// Creates a new U256 from a crypto_bigint U256 value.
    ///
    /// # Arguments
    /// * `value` - The underlying U256 value from crypto_bigint
    pub fn new(value: U256Lib) -> Self {
        Self(value)
    }

    /// Returns a reference to the underlying crypto_bigint U256 value.
    pub fn inner_value(&self) -> &U256Lib {
        &self.0
    }

    /// Checks if the value is zero.
    ///
    /// # Returns
    /// true if the value is zero, false otherwise
    pub fn is_zero(&self) -> bool {
        self.0.is_zero().into()
    }

    /// Performs checked addition.
    ///
    /// # Arguments
    /// * `rhs` - The value to add
    ///
    /// # Returns
    /// The sum of self and rhs
    ///
    /// # Errors
    /// Returns `Overflow` if the addition would overflow
    pub fn checked_add(&self, rhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_add(&rhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::Overflow);
        }
        Ok(U256(option.unwrap()))
    }

    /// Performs checked subtraction.
    ///
    /// # Arguments
    /// * `lhs` - The value to subtract
    ///
    /// # Returns
    /// The difference of self and lhs
    ///
    /// # Errors
    /// Returns `Underflow` if the subtraction would underflow
    pub fn checked_sub(&self, lhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_sub(&lhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::Underflow);
        }
        Ok(U256(option.unwrap()))
    }

    /// Performs checked multiplication.
    ///
    /// # Arguments
    /// * `rhs` - The value to multiply by
    ///
    /// # Returns
    /// The product of self and rhs
    ///
    /// # Errors
    /// Returns `Overflow` if the multiplication would overflow
    pub fn checked_mul(&self, rhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_mul(&rhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::Overflow);
        }
        Ok(U256(option.unwrap()))
    }

    /// Performs checked division.
    ///
    /// # Arguments
    /// * `rhs` - The value to divide by
    ///
    /// # Returns
    /// The quotient of self divided by rhs
    ///
    /// # Errors
    /// Returns `DivisionByZero` if rhs is zero
    pub fn checked_div(&self, rhs: &U256) -> Result<U256, InterLiquidSdkError> {
        let option = self.0.checked_div(&rhs.0);
        if option.is_none().into() {
            return Err(InterLiquidSdkError::DivisionByZero);
        }
        Ok(U256(option.unwrap()))
    }

    /// Raises the value to the power of the given exponent.
    ///
    /// # Arguments
    /// * `exponent` - The exponent to raise to (limited to u8 for safety)
    ///
    /// # Returns
    /// self raised to the power of exponent
    ///
    /// # Errors
    /// Returns `Overflow` if the operation would overflow
    pub fn powi(&self, exponent: u8) -> Result<U256, InterLiquidSdkError> {
        if exponent == 0 {
            return Ok(1.into());
        } else if exponent == 1 {
            return Ok(self.clone());
        } else {
            let mut val = self.clone();

            for _ in 1..exponent {
                val = val.checked_mul(self)?;
            }

            return Ok(val);
        }
    }
}

/// Implements Borsh serialization for U256.
/// Serializes the value as little-endian bytes.
impl BorshSerialize for U256 {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.to_le_bytes().serialize(writer)
    }
}

/// Implements Borsh deserialization for U256.
/// Deserializes from little-endian bytes.
impl BorshDeserialize for U256 {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let amount_bytes = <U256Lib as Encoding>::Repr::deserialize_reader(reader)?;
        let amount = U256Lib::from_le_bytes(amount_bytes);

        Ok(U256(amount))
    }
}

/// Converts from crypto_bigint U256 to our U256 wrapper.
impl From<U256Lib> for U256 {
    fn from(value: U256Lib) -> Self {
        U256(value)
    }
}

/// Converts from our U256 wrapper to crypto_bigint U256.
impl From<U256> for U256Lib {
    fn from(value: U256) -> Self {
        value.0
    }
}

/// Converts from u64 to U256.
impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        U256(U256Lib::from_u64(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_u256() {
        let value = U256Lib::from_u64(12345);
        let u256 = U256::new(value);
        assert_eq!(*u256.inner_value(), value);
    }

    #[test]
    fn test_is_zero() {
        let zero = U256::from(0u64);
        assert!(zero.is_zero());

        let non_zero = U256::from(1u64);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_checked_add_success() {
        let a = U256::from(100u64);
        let b = U256::from(200u64);
        
        let result = a.checked_add(&b).unwrap();
        assert_eq!(result, U256::from(300u64));
    }

    #[test]
    fn test_checked_add_overflow() {
        // Create a value close to max
        let max = U256::new(U256Lib::MAX);
        let one = U256::from(1u64);
        
        let result = max.checked_add(&one);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Overflow) => (),
            _ => panic!("Expected Overflow error"),
        }
    }

    #[test]
    fn test_checked_sub_success() {
        let a = U256::from(300u64);
        let b = U256::from(100u64);
        
        let result = a.checked_sub(&b).unwrap();
        assert_eq!(result, U256::from(200u64));
    }

    #[test]
    fn test_checked_sub_underflow() {
        let a = U256::from(100u64);
        let b = U256::from(200u64);
        
        let result = a.checked_sub(&b);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Underflow) => (),
            _ => panic!("Expected Underflow error"),
        }
    }

    #[test]
    fn test_checked_mul_success() {
        let a = U256::from(50u64);
        let b = U256::from(100u64);
        
        let result = a.checked_mul(&b).unwrap();
        assert_eq!(result, U256::from(5000u64));
    }

    #[test]
    fn test_checked_mul_overflow() {
        // Create large values that will overflow when multiplied
        // Use a very large value that will overflow when multiplied by 3
        let large_inner = U256Lib::from_be_slice(&[0xFF; 32]);
        let large = U256::new(large_inner);
        let three = U256::from(3u64);
        
        let result = large.checked_mul(&three);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Overflow) => (),
            _ => panic!("Expected Overflow error"),
        }
    }

    #[test]
    fn test_checked_div_success() {
        let a = U256::from(1000u64);
        let b = U256::from(10u64);
        
        let result = a.checked_div(&b).unwrap();
        assert_eq!(result, U256::from(100u64));
    }

    #[test]
    fn test_checked_div_by_zero() {
        let a = U256::from(100u64);
        let zero = U256::from(0u64);
        
        let result = a.checked_div(&zero);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::DivisionByZero) => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_powi_zero_exponent() {
        let base = U256::from(123u64);
        let result = base.powi(0).unwrap();
        assert_eq!(result, U256::from(1u64));
    }

    #[test]
    fn test_powi_one_exponent() {
        let base = U256::from(123u64);
        let result = base.powi(1).unwrap();
        assert_eq!(result, U256::from(123u64));
    }

    #[test]
    fn test_powi_small_exponents() {
        let base = U256::from(2u64);
        
        let result2 = base.powi(2).unwrap();
        assert_eq!(result2, U256::from(4u64));
        
        let result3 = base.powi(3).unwrap();
        assert_eq!(result3, U256::from(8u64));
        
        let result4 = base.powi(4).unwrap();
        assert_eq!(result4, U256::from(16u64));
        
        let result8 = base.powi(8).unwrap();
        assert_eq!(result8, U256::from(256u64));
    }

    #[test]
    fn test_powi_base_10() {
        let base = U256::from(10u64);
        
        let result2 = base.powi(2).unwrap();
        assert_eq!(result2, U256::from(100u64));
        
        let result3 = base.powi(3).unwrap();
        assert_eq!(result3, U256::from(1000u64));
        
        let result6 = base.powi(6).unwrap();
        assert_eq!(result6, U256::from(1_000_000u64));
    }

    #[test]
    fn test_powi_overflow() {
        // Create a large base that will overflow when raised to a high power
        let base = U256::from(u64::MAX);
        let result = base.powi(100); // This should definitely overflow
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Overflow) => (),
            _ => panic!("Expected Overflow error"),
        }
    }

    #[test]
    fn test_from_u256lib() {
        let lib_value = U256Lib::from_u64(999);
        let u256: U256 = lib_value.into();
        assert_eq!(*u256.inner_value(), lib_value);
    }

    #[test]
    fn test_into_u256lib() {
        let u256 = U256::from(888u64);
        let lib_value: U256Lib = u256.into();
        assert_eq!(lib_value, U256Lib::from_u64(888));
    }

    #[test]
    fn test_from_u64() {
        let u256 = U256::from(12345u64);
        assert_eq!(*u256.inner_value(), U256Lib::from_u64(12345));
        
        // Test edge cases
        let zero = U256::from(0u64);
        assert!(zero.is_zero());
        
        let max = U256::from(u64::MAX);
        assert_eq!(*max.inner_value(), U256Lib::from_u64(u64::MAX));
    }

    #[test]
    fn test_clone() {
        let original = U256::from(100u64);
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(*original.inner_value(), *cloned.inner_value());
    }

    #[test]
    fn test_debug() {
        let u256 = U256::from(123u64);
        let debug_str = format!("{:?}", u256);
        
        // The debug format will show U256 struct with the inner U256Lib value
        assert!(debug_str.contains("U256"));
    }

    #[test]
    fn test_equality() {
        let a = U256::from(100u64);
        let b = U256::from(100u64);
        let c = U256::from(200u64);
        
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_borsh_serialization() {
        let original = U256::from(123456789u64);
        
        // Serialize
        let serialized = borsh::to_vec(&original).unwrap();
        
        // Deserialize
        let deserialized: U256 = borsh::from_slice(&serialized).unwrap();
        
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_edge_case_arithmetic() {
        // Test with zero
        let zero = U256::from(0u64);
        let one = U256::from(1u64);
        
        assert_eq!(zero.checked_add(&one).unwrap(), one);
        assert_eq!(one.checked_sub(&one).unwrap(), zero);
        assert_eq!(zero.checked_mul(&one).unwrap(), zero);
        assert_eq!(zero.checked_div(&one).unwrap(), zero);
        
        // Test identity operations
        let value = U256::from(42u64);
        assert_eq!(value.checked_add(&zero).unwrap(), value);
        assert_eq!(value.checked_sub(&zero).unwrap(), value);
        assert_eq!(value.checked_mul(&one).unwrap(), value);
        assert_eq!(value.checked_div(&one).unwrap(), value);
    }
}
