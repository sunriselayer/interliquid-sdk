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
            let mut val = self.checked_mul(self)?;

            for _ in 2..exponent {
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
