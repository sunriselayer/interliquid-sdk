use borsh_derive::{BorshDeserialize, BorshSerialize};

/// Represents the blockchain environment context for transaction execution.
/// This struct contains information about the current chain state and block.
///
/// `block_time` is the time of the block in seconds since Unix epoch
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Environment {
    /// The unique identifier of the blockchain
    pub chain_id: String,
    /// The current block height (block number)
    pub block_height: u64,
    /// The timestamp of the current block in seconds since Unix epoch
    pub block_time: u64,
}

impl Environment {
    /// Creates a new Environment instance.
    ///
    /// # Arguments
    /// * `chain_id` - The unique identifier of the blockchain
    /// * `block_height` - The current block height
    /// * `block_time` - The timestamp of the block in seconds since Unix epoch
    pub fn new(chain_id: String, block_height: u64, block_time: u64) -> Self {
        Self {
            chain_id,
            block_height,
            block_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_environment() {
        let env = Environment::new("testnet-1".to_string(), 12345, 1640000000);
        
        assert_eq!(env.chain_id, "testnet-1");
        assert_eq!(env.block_height, 12345);
        assert_eq!(env.block_time, 1640000000);
    }

    #[test]
    fn test_environment_clone() {
        let env1 = Environment::new("mainnet".to_string(), 1000, 1234567890);
        let env2 = env1.clone();
        
        assert_eq!(env1, env2);
        assert_eq!(env2.chain_id, "mainnet");
        assert_eq!(env2.block_height, 1000);
        assert_eq!(env2.block_time, 1234567890);
    }

    #[test]
    fn test_environment_debug() {
        let env = Environment::new("test".to_string(), 1, 1);
        let debug_str = format!("{:?}", env);
        
        assert!(debug_str.contains("Environment"));
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("1"));
    }

    #[test]
    fn test_environment_equality() {
        let env1 = Environment::new("chain1".to_string(), 100, 1000);
        let env2 = Environment::new("chain1".to_string(), 100, 1000);
        let env3 = Environment::new("chain2".to_string(), 100, 1000);
        let env4 = Environment::new("chain1".to_string(), 101, 1000);
        let env5 = Environment::new("chain1".to_string(), 100, 1001);
        
        assert_eq!(env1, env2);
        assert_ne!(env1, env3); // Different chain_id
        assert_ne!(env1, env4); // Different block_height
        assert_ne!(env1, env5); // Different block_time
    }

    #[test]
    fn test_environment_edge_cases() {
        // Test with empty chain_id
        let env1 = Environment::new("".to_string(), 0, 0);
        assert_eq!(env1.chain_id, "");
        assert_eq!(env1.block_height, 0);
        assert_eq!(env1.block_time, 0);
        
        // Test with max values
        let env2 = Environment::new("max".to_string(), u64::MAX, u64::MAX);
        assert_eq!(env2.block_height, u64::MAX);
        assert_eq!(env2.block_time, u64::MAX);
    }

    #[test]
    fn test_environment_serialization() {
        let env = Environment::new("test-chain".to_string(), 42, 1234567890);
        
        // Test Borsh serialization roundtrip
        let serialized = borsh::to_vec(&env).unwrap();
        let deserialized: Environment = borsh::from_slice(&serialized).unwrap();
        
        assert_eq!(env, deserialized);
    }
}
