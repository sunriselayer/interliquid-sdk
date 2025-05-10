use std::collections::{BTreeMap, BTreeSet};

use crate::{state::StateManager, types::InterLiquidSdkError};

use super::range::ObjectSafeRangeBounds;

pub struct TransactionalState<S: StateManager> {
    pub state: S,
    pub get: BTreeMap<Vec<u8>, bool>,
    pub set: BTreeMap<Vec<u8>, Vec<u8>>,
    pub del: BTreeMap<Vec<u8>, Vec<u8>>,
    pub iter: Vec<IterRecord>,
}

impl<S: StateManager> TransactionalState<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            get: BTreeMap::new(),
            set: BTreeMap::new(),
            del: BTreeMap::new(),
            iter: Vec::new(),
        }
    }

    pub fn get_accessed(&self) -> &BTreeMap<Vec<u8>, bool> {
        &self.get
    }

    pub fn commit(&mut self) -> Result<(), InterLiquidSdkError> {
        for key in self.set.keys() {
            self.state.set(key, self.set.get(key).unwrap())?;
        }

        for key in self.del.keys() {
            self.state.del(key)?;
        }

        Ok(())
    }
}

impl<S: StateManager> StateManager for TransactionalState<S> {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = if self.del.contains_key(key) {
            None
        } else if let Some(value) = self.set.get(key) {
            Some(value.clone())
        } else {
            self.state.get(key)?
        };

        if !self.get.contains_key(key) {
            self.get.insert(key.to_vec(), val.is_some());
        }

        Ok(val)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.del.remove(key);
        self.set.insert(key.to_vec(), value.to_vec());

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        // do not use self.get to not update the get map
        let deleted = self.state.get(key)?;

        self.set.remove(key);

        if let Some(deleted) = deleted {
            self.del.insert(key.to_vec(), deleted);
        }

        Ok(())
    }

    fn iter<'a>(
        &'a mut self,
        range: ObjectSafeRangeBounds<Vec<u8>>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = self.state.iter(range).filter_map(|result| {
            let (key, value) = match result {
                Ok((key, value)) => (key, value),
                Err(e) => return Some(Err(e)),
            };

            if self.del.contains_key(&key) {
                return None;
            }

            if self.set.contains_key(&key) {
                let value = self.set.get(&key).unwrap().clone();

                return Some(Ok((key, value)));
            }

            Some(Ok((key, value)))
        });

        let record_index = self.iter.len();
        self.iter.push(IterRecord::new());

        Box::new(TransactionalStateIterator::new(
            &mut self.iter[record_index],
            Box::new(iter),
        ))
    }
}
pub struct IterRecord {
    pub keys: BTreeSet<Vec<u8>>,
    pub finished: bool,
}

impl IterRecord {
    pub fn new() -> Self {
        Self {
            keys: BTreeSet::new(),
            finished: false,
        }
    }
}
pub struct TransactionalStateIterator<'a> {
    recorder: &'a mut IterRecord,
    iterator: Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>,
}

impl<'a> TransactionalStateIterator<'a> {
    pub fn new(
        recorder: &'a mut IterRecord,
        iterator: Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>,
    ) -> Self {
        Self { recorder, iterator }
    }
}

impl<'a> Iterator for TransactionalStateIterator<'a> {
    type Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next();

        if let Some(item) = &item {
            if let Ok((key, _)) = item {
                self.recorder.keys.insert(key.to_owned());
            }
        } else {
            self.recorder.finished = true;
        }

        item
    }
}
