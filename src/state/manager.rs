use crate::types::InterLiquidSdkError;

pub trait StateManager {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    // If the key is not found, it must be a no-op
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;

    fn iter(
        &mut self,
        prefix: &[u8],
    ) -> Result<impl Iterator<Item = (Vec<u8>, Vec<u8>)>, InterLiquidSdkError>;
}
