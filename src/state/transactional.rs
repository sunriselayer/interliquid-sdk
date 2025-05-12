use std::collections::BTreeSet;

use crate::{state::TracableStateManager, types::InterLiquidSdkError};

use super::{
    log::{StateLog, StateLogIter, StateLogRead},
    CompressedDiffs, StateLogDiff, StateManager, ValueDiff,
};

pub struct TransactionalStateManager<'s, S: StateManager> {
    pub state_manager: &'s S,
    pub logs: Vec<StateLog>,
    pub diffs: CompressedDiffs,
}

impl<'s, S: StateManager> TransactionalStateManager<'s, S> {
    pub fn new(state_manager: &'s S) -> Self {
        Self {
            state_manager,
            logs: Vec::new(),
            diffs: CompressedDiffs::default(),
        }
    }

    pub fn from_diffs(state_manager: &'s S, diffs: CompressedDiffs) -> Self {
        Self {
            state_manager,
            logs: Vec::new(),
            diffs,
        }
    }

    fn get_without_logging(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = if let Some(diff) = self.diffs.map().get(key) {
            diff.after.clone()
        } else {
            self.state_manager.get(key)?
        };

        Ok(val)
    }

    pub fn commit(&self, state_manager: &mut S) -> Result<(), InterLiquidSdkError> {
        for (key, diff) in self.diffs.map() {
            match &diff.after {
                Some(value) => state_manager.set(key, value)?,
                None => state_manager.del(key)?,
            }
        }

        Ok(())
    }
}

impl<'s, S: StateManager> TracableStateManager for TransactionalStateManager<'s, S> {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = self.get_without_logging(key)?;

        self.logs.push(StateLog::Read(StateLogRead {
            key: key.to_vec(),
            found: val.is_some(),
        }));

        Ok(val)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        let before = self.get_without_logging(key)?;
        let log = StateLog::Diff(StateLogDiff {
            key: key.to_vec(),
            diff: ValueDiff {
                before,
                after: Some(value.to_vec()),
            },
        });

        self.diffs.apply_logs([&log].into_iter())?;
        self.logs.push(log);

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        let before = self.get_without_logging(key)?;
        let log = StateLog::Diff(StateLogDiff {
            key: key.to_vec(),
            diff: ValueDiff {
                before,
                after: None,
            },
        });

        self.diffs.apply_logs([&log].into_iter())?;
        self.logs.push(log);

        Ok(())
    }

    fn iter<'a>(
        &'a mut self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = self.state_manager.iter(key_prefix).filter_map(|result| {
            let (key, value) = match result {
                Ok((key, value)) => (key, value),
                Err(e) => return Some(Err(e)),
            };

            if let Some(diff) = self.diffs.map().get(&key) {
                match &diff.after {
                    Some(value) => Some(Ok((key, value.clone()))),
                    None => None,
                }
            } else {
                Some(Ok((key, value)))
            }
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

// This is needed to enforce that all keys are recorded.
// This makes the proof of range completeness proof easier.
impl<'a> Drop for TransactionalStateIterator<'a> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}
