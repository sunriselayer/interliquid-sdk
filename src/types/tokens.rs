use std::collections::BTreeMap;

use crate::types::{InterLiquidSdkError, Token};

use super::U256;

/// A collection of tokens stored as a map from denomination to amount.
/// Uses BTreeMap to maintain sorted order of denominations.
pub type Tokens = BTreeMap<String, U256>;

/// Trait defining operations for token collections.
pub trait TokensI: Sized {
    /// Validates all tokens in the collection.
    fn validate(&self) -> Result<(), InterLiquidSdkError>;
    
    /// Performs checked addition of two token collections.
    fn checked_add(self, rhs: &Self) -> Result<Self, InterLiquidSdkError>;
    
    /// Performs checked subtraction of two token collections.
    fn checked_sub(self, rhs: &Self) -> Result<Self, InterLiquidSdkError>;
}

impl TokensI for Tokens {
    /// Validates all tokens in the collection.
    /// Checks that all denominations are non-empty and all amounts are non-zero.
    ///
    /// # Errors
    /// Returns an error if any token has an invalid denomination or zero amount
    fn validate(&self) -> Result<(), InterLiquidSdkError> {
        for (denom, amount) in self.iter() {
            Token::validate_denom(denom)?;
            Token::validate_amount(amount)?;
        }

        Ok(())
    }

    /// Performs checked addition of two token collections.
    /// Tokens with the same denomination are added together.
    /// Tokens that exist in only one collection are included in the result.
    ///
    /// # Arguments
    /// * `rhs` - The token collection to add
    ///
    /// # Returns
    /// A new token collection with the combined tokens
    ///
    /// # Errors
    /// Returns an error if validation fails or if addition overflows
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

    /// Performs checked subtraction of two token collections.
    /// Subtracts tokens in `rhs` from the corresponding tokens in `self`.
    ///
    /// # Arguments
    /// * `rhs` - The token collection to subtract
    ///
    /// # Returns
    /// A new token collection with the subtracted amounts
    ///
    /// # Errors
    /// Returns `Underflow` if a token in `rhs` doesn't exist in `self` or if subtraction underflows
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_validate_empty() {
        let tokens = Tokens::new();
        assert!(tokens.validate().is_ok());
    }

    #[test]
    fn test_tokens_validate_valid() {
        let mut tokens = Tokens::new();
        tokens.insert("uatom".to_string(), 100u64.into());
        tokens.insert("usdc".to_string(), 200u64.into());
        
        assert!(tokens.validate().is_ok());
    }

    #[test]
    fn test_tokens_validate_invalid_denom() {
        let mut tokens = Tokens::new();
        tokens.insert("".to_string(), 100u64.into());
        
        let result = tokens.validate();
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::InvalidDenom) => (),
            _ => panic!("Expected InvalidDenom error"),
        }
    }

    #[test]
    fn test_tokens_validate_zero_amount() {
        let mut tokens = Tokens::new();
        tokens.insert("uatom".to_string(), 0u64.into());
        
        let result = tokens.validate();
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::ZeroAmount) => (),
            _ => panic!("Expected ZeroAmount error"),
        }
    }

    #[test]
    fn test_tokens_checked_add_both_empty() {
        let tokens1 = Tokens::new();
        let tokens2 = Tokens::new();
        
        let result = tokens1.checked_add(&tokens2).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_tokens_checked_add_one_empty() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        
        let tokens2 = Tokens::new();
        
        let result = tokens1.clone().checked_add(&tokens2).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("uatom"), Some(&100u64.into()));
        
        // Test reverse
        let result2 = tokens2.checked_add(&tokens1).unwrap();
        assert_eq!(result2.len(), 1);
        assert_eq!(result2.get("uatom"), Some(&100u64.into()));
    }

    #[test]
    fn test_tokens_checked_add_same_denoms() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        tokens1.insert("usdc".to_string(), 200u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("uatom".to_string(), 50u64.into());
        tokens2.insert("usdc".to_string(), 150u64.into());
        
        let result = tokens1.checked_add(&tokens2).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("uatom"), Some(&150u64.into()));
        assert_eq!(result.get("usdc"), Some(&350u64.into()));
    }

    #[test]
    fn test_tokens_checked_add_different_denoms() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("usdc".to_string(), 200u64.into());
        
        let result = tokens1.checked_add(&tokens2).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("uatom"), Some(&100u64.into()));
        assert_eq!(result.get("usdc"), Some(&200u64.into()));
    }

    #[test]
    fn test_tokens_checked_add_mixed_denoms() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        tokens1.insert("usdc".to_string(), 200u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("usdc".to_string(), 50u64.into());
        tokens2.insert("ueth".to_string(), 300u64.into());
        
        let result = tokens1.checked_add(&tokens2).unwrap();
        
        assert_eq!(result.len(), 3);
        assert_eq!(result.get("uatom"), Some(&100u64.into()));
        assert_eq!(result.get("usdc"), Some(&250u64.into()));
        assert_eq!(result.get("ueth"), Some(&300u64.into()));
    }

    #[test]
    fn test_tokens_checked_sub_same_amounts() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("uatom".to_string(), 100u64.into());
        
        let result = tokens1.checked_sub(&tokens2).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("uatom"), Some(&0u64.into()));
    }

    #[test]
    fn test_tokens_checked_sub_partial() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        tokens1.insert("usdc".to_string(), 200u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("uatom".to_string(), 50u64.into());
        tokens2.insert("usdc".to_string(), 150u64.into());
        
        let result = tokens1.checked_sub(&tokens2).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("uatom"), Some(&50u64.into()));
        assert_eq!(result.get("usdc"), Some(&50u64.into()));
    }

    #[test]
    fn test_tokens_checked_sub_underflow() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 50u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("uatom".to_string(), 100u64.into());
        
        let result = tokens1.checked_sub(&tokens2);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Underflow) => (),
            _ => panic!("Expected Underflow error"),
        }
    }

    #[test]
    fn test_tokens_checked_sub_missing_denom() {
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), 100u64.into());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("usdc".to_string(), 50u64.into());
        
        let result = tokens1.checked_sub(&tokens2);
        assert!(result.is_err());
        match result {
            Err(InterLiquidSdkError::Underflow) => (),
            _ => panic!("Expected Underflow error"),
        }
    }

    #[test]
    fn test_tokens_btree_order() {
        let mut tokens = Tokens::new();
        tokens.insert("usdc".to_string(), 100u64.into());
        tokens.insert("uatom".to_string(), 200u64.into());
        tokens.insert("ueth".to_string(), 300u64.into());
        
        // BTreeMap maintains sorted order
        let keys: Vec<String> = tokens.keys().cloned().collect();
        assert_eq!(keys, vec!["uatom", "ueth", "usdc"]);
    }

    #[test]
    fn test_tokens_edge_cases() {
        // Test with max values
        let bytes = vec![0xFF; 32];
        let max_inner = crate::crypto_bigint::U256::from_be_slice(&bytes);
        let max_value = U256::new(max_inner);
        
        let mut tokens1 = Tokens::new();
        tokens1.insert("uatom".to_string(), max_value.clone());
        
        let mut tokens2 = Tokens::new();
        tokens2.insert("uatom".to_string(), max_value);
        
        // This should overflow
        let result = tokens1.checked_add(&tokens2);
        assert!(result.is_err());
    }
}
