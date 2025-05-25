use crate::types::InterLiquidSdkError;

/// Trait for managing state storage operations.
/// Provides basic key-value store functionality with iteration support.
/// Implementations must be thread-safe and have a static lifetime.
pub trait StateManager: Send + Sync + 'static {
    /// Retrieves a value by its key from the state.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to look up
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(value))` if the key exists
    /// * `Ok(None)` if the key doesn't exist
    /// * `Err` if an error occurred during retrieval
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    
    /// Sets a key-value pair in the state.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to set
    /// * `value` - The value to associate with the key
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if the operation succeeded
    /// * `Err` if an error occurred during storage
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    
    /// Deletes a key from the state.
    /// If the key is not found, it must be a no-op.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to delete
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if the operation succeeded (including when key doesn't exist)
    /// * `Err` if an error occurred during deletion
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;

    /// Creates an iterator over key-value pairs with a specific key prefix.
    /// 
    /// # Arguments
    /// 
    /// * `key_prefix` - The prefix to filter keys by
    /// 
    /// # Returns
    /// 
    /// An iterator that yields `Result<(key, value)>` pairs
    fn iter<'a>(
        &'a self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>;
}

/// Trait for state managers that support operation tracing.
/// Similar to StateManager but with mutable access for tracking operations.
/// This trait is automatically implemented for all StateManager implementations.
pub trait TracableStateManager: Send + Sync {
    /// Retrieves a value by its key from the state (with tracing support).
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to look up
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(value))` if the key exists
    /// * `Ok(None)` if the key doesn't exist
    /// * `Err` if an error occurred during retrieval
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    
    /// Sets a key-value pair in the state (with tracing support).
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to set
    /// * `value` - The value to associate with the key
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if the operation succeeded
    /// * `Err` if an error occurred during storage
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    
    /// Deletes a key from the state (with tracing support).
    /// If the key is not found, it must be a no-op.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to delete
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if the operation succeeded (including when key doesn't exist)
    /// * `Err` if an error occurred during deletion
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;

    /// Creates an iterator over key-value pairs with a specific key prefix (with tracing support).
    /// 
    /// # Arguments
    /// 
    /// * `key_prefix` - The prefix to filter keys by
    /// 
    /// # Returns
    /// 
    /// An iterator that yields `Result<(key, value)>` pairs
    fn iter<'a>(
        &'a mut self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>;
}

/// Blanket implementation of TracableStateManager for all StateManager types.
/// This allows any StateManager to be used where TracableStateManager is required.
impl<S: StateManager> TracableStateManager for S {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        <S as StateManager>::get(self, key)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        <S as StateManager>::set(self, key, value)
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        <S as StateManager>::del(self, key)
    }

    fn iter<'a>(
        &'a mut self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        <S as StateManager>::iter(self, key_prefix)
    }
}
