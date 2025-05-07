use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeBounds,
};

use crate::{state::StateManager, types::InterLiquidSdkError};

pub struct StateContext<S: StateManager> {
    pub state: S,
    pub get: BTreeSet<Vec<u8>>,
    pub set: BTreeMap<Vec<u8>, Vec<u8>>,
    pub del: BTreeSet<Vec<u8>>,
}

impl<S: StateManager> StateContext<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            get: BTreeSet::new(),
            set: BTreeMap::new(),
            del: BTreeSet::new(),
        }
    }

    pub fn accessed_keys(&self) -> &BTreeSet<Vec<u8>> {
        &self.get
    }

    pub fn commit(&mut self) -> Result<(), InterLiquidSdkError> {
        for key in self.set.keys() {
            self.state.set(key, self.set.get(key).unwrap())?;
        }

        for key in self.del.iter() {
            self.state.del(key)?;
        }

        Ok(())
    }
}

impl<S: StateManager> StateManager for StateContext<S> {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        self.get.insert(key.to_vec());

        if let Some(value) = self.set.get(key) {
            Ok(Some(value.clone()))
        } else {
            self.state.get(key)
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.set.insert(key.to_vec(), value.to_vec());

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.set.remove(key);
        self.del.insert(key.to_vec());

        Ok(())
    }

    fn iter<'a>(
        &'a mut self,
        range: impl RangeBounds<Vec<u8>>,
    ) -> impl Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a {
        self.state.iter(range).map(|result| {
            let (key, value) = result?;

            if self.set.contains_key(&key) {
                let value = self.set.get(&key).unwrap().clone();

                return Ok((key, value));
            }
            Ok((key, value))
        })
    }
}
