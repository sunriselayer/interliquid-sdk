use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

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
        Ok(self.map.get(key).cloned())
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.map.insert(key.to_vec(), value.to_vec());

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.map.remove(key);

        Ok(())
    }

    fn iter(
        &mut self,
        prefix: &[u8],
    ) -> Result<impl Iterator<Item = (Vec<u8>, Vec<u8>)>, InterLiquidSdkError> {
    }
}

fn increment_prefix(prefix: &[u8]) -> Vec<u8> {}
