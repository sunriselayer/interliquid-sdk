use crate::types::InterLiquidSdkError;
use std::ops::RangeBounds;

pub trait StateManager {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    // If the key is not found, it must be a no-op
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;

    fn iter<'a>(
        &'a mut self,
        range: impl RangeBounds<Vec<u8>>,
    ) -> impl Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a;
}
