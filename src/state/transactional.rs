use std::collections::{BTreeMap, BTreeSet};

use crate::{state::StateManager, types::InterLiquidSdkError};

use super::{
    log::{StateLog, StateLogIter, StateLogRead},
    range::ObjectSafeRangeBounds,
};

pub struct TransactionalState<S: StateManager> {
    pub state: S,
    pub set: BTreeMap<Vec<u8>, Vec<u8>>,
    pub del: BTreeSet<Vec<u8>>,
    pub logs: Vec<StateLog>,
}

impl<S: StateManager> TransactionalState<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            set: BTreeMap::new(),
            del: BTreeSet::new(),
            logs: Vec::new(),
        }
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

impl<S: StateManager> StateManager for TransactionalState<S> {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = if self.del.contains(key) {
            None
        } else if let Some(value) = self.set.get(key) {
            Some(value.clone())
        } else {
            self.state.get(key)?
        };

        self.logs.push(StateLog::Read(StateLogRead {
            key: key.to_vec(),
            found: val.is_some(),
        }));

        Ok(val)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.del.remove(key);
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
        range: ObjectSafeRangeBounds<Vec<u8>>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = self.state.iter(range).filter_map(|result| {
            let (key, value) = match result {
                Ok((key, value)) => (key, value),
                Err(e) => return Some(Err(e)),
            };

            if self.del.contains(&key) {
                return None;
            }

            if self.set.contains_key(&key) {
                let value = self.set.get(&key).unwrap().clone();

                return Some(Ok((key, value)));
            }

            Some(Ok((key, value)))
        });

        let record_index = self.logs.len();
        self.logs.push(StateLog::Iter(StateLogIter::new()));

        if let StateLog::Iter(recorder) = &mut self.logs[record_index] {
            Box::new(TransactionalStateIterator::new(recorder, Box::new(iter)))
        } else {
            unreachable!()
        }
    }
}
pub struct IterRecord {
    pub keys: BTreeSet<Vec<u8>>,
}

impl IterRecord {
    pub fn new() -> Self {
        Self {
            keys: BTreeSet::new(),
        }
    }
}
pub struct TransactionalStateIterator<'a> {
    recorder: &'a mut StateLogIter,
    iterator: Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>,
}

impl<'a> TransactionalStateIterator<'a> {
    pub fn new(
        recorder: &'a mut StateLogIter,
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
        }

        item
    }
}
