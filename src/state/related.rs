use std::collections::BTreeMap;
use std::ops::RangeBounds;

use crate::types::InterLiquidSdkError;

use super::manager::StateManager;

pub struct RelatedStates {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl RelatedStates {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl StateManager for RelatedStates {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        if let Some(value) = self.map.get(key) {
            Ok(Some(value.clone()))
        } else {
            Err(InterLiquidSdkError::UnrelatedState)
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.map.insert(key.to_vec(), value.to_vec());

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.map.remove(key);

        Ok(())
    }

    fn iter<'a>(
        &'a mut self,
        range: impl RangeBounds<Vec<u8>>,
    ) -> impl Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a {
        let iter = self.map.range(range);

        iter.map(|result| {
            let (key, value) = result;

            Ok((key.clone(), value.clone()))
        })
    }
}
