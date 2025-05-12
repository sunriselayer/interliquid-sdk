use crate::types::InterLiquidSdkError;

pub trait StateManager: Send + Sync + 'static {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    // If the key is not found, it must be a no-op
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;

    fn iter<'a>(
        &'a self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>;
}

pub trait TracableStateManager: Send + Sync {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    // If the key is not found, it must be a no-op
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;

    fn iter<'a>(
        &'a mut self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>;
}

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
