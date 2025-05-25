use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{InterLiquidSdkError, U256};

/// Represents a token with a denomination and amount.
/// Used for handling different types of tokens/currencies in the system.
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Token {
    /// The denomination or identifier of the token (e.g., "uatom", "usdc")
    pub denom: String,
    /// The amount of the token
    pub amount: U256,
}

impl Token {
    /// Creates a new Token instance.
    ///
    /// # Arguments
    /// * `denom` - The denomination of the token
    /// * `amount` - The amount of the token
    pub fn new(denom: String, amount: U256) -> Self {
        Self { denom, amount }
    }

    /// Validates that a denomination is valid (non-empty).
    ///
    /// # Arguments
    /// * `denom` - The denomination to validate
    ///
    /// # Errors
    /// Returns `InvalidDenom` if the denomination is empty
    pub(crate) fn validate_denom(denom: &str) -> Result<(), InterLiquidSdkError> {
        if denom.is_empty() {
            return Err(InterLiquidSdkError::InvalidDenom);
        }
        Ok(())
    }

    /// Validates that an amount is valid (non-zero).
    ///
    /// # Arguments
    /// * `amount` - The amount to validate
    ///
    /// # Errors
    /// Returns `ZeroAmount` if the amount is zero
    pub(crate) fn validate_amount(amount: &U256) -> Result<(), InterLiquidSdkError> {
        if amount.is_zero() {
            return Err(InterLiquidSdkError::ZeroAmount);
        }
        Ok(())
    }

    /// Validates the token by checking both denomination and amount.
    ///
    /// # Errors
    /// Returns an error if either the denomination is empty or the amount is zero
    pub fn validate(&self) -> Result<(), InterLiquidSdkError> {
        Self::validate_denom(&self.denom)?;
        Self::validate_amount(&self.amount)
    }

    /// Performs checked addition of two tokens.
    ///
    /// # Arguments
    /// * `rhs` - The token to add
    ///
    /// # Returns
    /// A new token with the sum of the amounts
    ///
    /// # Errors
    /// Returns `DenomMismatch` if the denominations don't match, or `Overflow` if the addition overflows
    pub fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError> {
        if self.denom != rhs.denom {
            return Err(InterLiquidSdkError::DenomMismatch);
        }

        Ok(Self {
            denom: self.denom,
            amount: self.amount.checked_add(&rhs.amount)?,
        })
    }

    /// Performs checked subtraction of two tokens.
    ///
    /// # Arguments
    /// * `lhs` - The token to subtract
    ///
    /// # Returns
    /// A new token with the difference of the amounts
    ///
    /// # Errors
    /// Returns `DenomMismatch` if the denominations don't match, or `Underflow` if the subtraction underflows
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_token() {
        let token = Token::new("uatom".to_string(), 1000u64.into());
        
        assert_eq!(token.denom, "uatom");
        assert_eq!(token.amount, 1000u64.into());
    }

    #[test]
    fn test_validate_denom_valid() {
        assert!(Token::validate_denom("uatom").is_ok());
        assert!(Token::validate_denom("usdc").is_ok());
        assert!(Token::validate_denom("ibc/ABC123").is_ok());
    }

    #[test]
    fn test_validate_denom_invalid() {
        let result = Token::validate_denom("");
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::InvalidDenom) => (),
            _ => panic!("Expected InvalidDenom error"),
        }
    }

    #[test]
    fn test_validate_amount_valid() {
        assert!(Token::validate_amount(&100u64.into()).is_ok());
        assert!(Token::validate_amount(&1u64.into()).is_ok());
        assert!(Token::validate_amount(&U256::from(u64::MAX)).is_ok());
    }

    #[test]
    fn test_validate_amount_invalid() {
        let result = Token::validate_amount(&0u64.into());
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::ZeroAmount) => (),
            _ => panic!("Expected ZeroAmount error"),
        }
    }

    #[test]
    fn test_validate_valid_token() {
        let token = Token::new("uatom".to_string(), 1000u64.into());
        assert!(token.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_denom() {
        let token = Token::new("".to_string(), 1000u64.into());
        let result = token.validate();
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::InvalidDenom) => (),
            _ => panic!("Expected InvalidDenom error"),
        }
    }

    #[test]
    fn test_validate_zero_amount() {
        let token = Token::new("uatom".to_string(), 0u64.into());
        let result = token.validate();
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::ZeroAmount) => (),
            _ => panic!("Expected ZeroAmount error"),
        }
    }

    #[test]
    fn test_checked_add_same_denom() {
        let token1 = Token::new("uatom".to_string(), 100u64.into());
        let token2 = Token::new("uatom".to_string(), 200u64.into());
        
        let result = token1.checked_add(&token2).unwrap();
        
        assert_eq!(result.denom, "uatom");
        assert_eq!(result.amount, 300u64.into());
    }

    #[test]
    fn test_checked_add_different_denom() {
        let token1 = Token::new("uatom".to_string(), 100u64.into());
        let token2 = Token::new("usdc".to_string(), 200u64.into());
        
        let result = token1.checked_add(&token2);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::DenomMismatch) => (),
            _ => panic!("Expected DenomMismatch error"),
        }
    }

    #[test]
    fn test_checked_add_overflow() {
        // Create a very large value that will overflow when added to itself
        let bytes = vec![0xFF; 32];
        let max_inner = crate::crypto_bigint::U256::from_be_slice(&bytes);
        let max_value = U256::new(max_inner);
        
        let token1 = Token::new("uatom".to_string(), max_value.clone());
        let token2 = Token::new("uatom".to_string(), max_value);
        
        let result = token1.checked_add(&token2);
        assert!(result.is_err()); // Should overflow
    }

    #[test]
    fn test_checked_sub_same_denom() {
        let token1 = Token::new("uatom".to_string(), 300u64.into());
        let token2 = Token::new("uatom".to_string(), 100u64.into());
        
        let result = token1.checked_sub(&token2).unwrap();
        
        assert_eq!(result.denom, "uatom");
        assert_eq!(result.amount, 200u64.into());
    }

    #[test]
    fn test_checked_sub_different_denom() {
        let token1 = Token::new("uatom".to_string(), 300u64.into());
        let token2 = Token::new("usdc".to_string(), 100u64.into());
        
        let result = token1.checked_sub(&token2);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::DenomMismatch) => (),
            _ => panic!("Expected DenomMismatch error"),
        }
    }

    #[test]
    fn test_checked_sub_underflow() {
        let token1 = Token::new("uatom".to_string(), 100u64.into());
        let token2 = Token::new("uatom".to_string(), 200u64.into());
        
        let result = token1.checked_sub(&token2);
        assert!(result.is_err()); // Should underflow
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new("uatom".to_string(), 100u64.into());
        let token2 = Token::new("uatom".to_string(), 100u64.into());
        let token3 = Token::new("usdc".to_string(), 100u64.into());
        let token4 = Token::new("uatom".to_string(), 200u64.into());
        
        assert_eq!(token1, token2);
        assert_ne!(token1, token3); // Different denom
        assert_ne!(token1, token4); // Different amount
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new("uatom".to_string(), 100u64.into());
        let token2 = token1.clone();
        
        assert_eq!(token1, token2);
        assert_eq!(token2.denom, "uatom");
        assert_eq!(token2.amount, 100u64.into());
    }

    #[test]
    fn test_token_debug() {
        let token = Token::new("uatom".to_string(), 100u64.into());
        let debug_str = format!("{:?}", token);
        
        assert!(debug_str.contains("Token"));
        assert!(debug_str.contains("uatom"));
        // The amount is displayed as U256 struct, not a plain number
    }
}
