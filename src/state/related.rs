use std::collections::BTreeMap;

use crate::types::InterLiquidSdkError;

use super::manager::StateManager;

pub struct RelatedState {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl RelatedState {
    pub fn new(map: BTreeMap<Vec<u8>, Vec<u8>>) -> Self {
        Self { map }
    }
}

impl StateManager for RelatedState {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
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
        &'a self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = bytes_prefix_range(&self.map, key_prefix);

        Box::new(iter.map(|(k, v)| Ok((k, v))))
    }
}

pub fn bytes_prefix_range<'a, T: Clone>(
    map: &'a BTreeMap<Vec<u8>, T>,
    key_prefix: Vec<u8>,
) -> Box<dyn Iterator<Item = (Vec<u8>, T)> + 'a> {
    if key_prefix.len() == 0 {
        Box::new(map.iter().map(|(k, v)| (k.clone(), v.clone())))
    } else if key_prefix.iter().all(|&b| b == 0xFF) {
        Box::new(map.range(key_prefix..).map(|(k, v)| (k.clone(), v.clone())))
    } else {
        let mut key_prefix_next = key_prefix.clone();
        *key_prefix_next.last_mut().unwrap() += 1; // len > 0

        Box::new(
            map.range(key_prefix..key_prefix_next)
                .map(|(k, v)| (k.clone(), v.clone())),
        )
    }
}
