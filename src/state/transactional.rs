use std::collections::{BTreeMap, BTreeSet};

use crate::{state::TracableStateManager, types::InterLiquidSdkError};

use super::{
    log::{StateLog, StateLogIter, StateLogRead},
    AccumulatedLogs, StateLogDiff, StateManager, ValueDiff,
};

pub struct TransactionalStateManager<'s, S: StateManager> {
    pub state_manager: &'s S,
    pub logs: Vec<StateLog>,
    pub accum_logs_prev: AccumulatedLogs,
    pub accum_logs_next: AccumulatedLogs,
}

impl<'s, S: StateManager> TransactionalStateManager<'s, S> {
    pub fn new(state_manager: &'s S) -> Self {
        Self {
            state_manager,
            logs: Vec::new(),
            accum_logs_prev: AccumulatedLogs::default(),
            accum_logs_next: AccumulatedLogs::default(),
        }
    }

    pub fn from_accum_logs_prev(state_manager: &'s S, accum_logs_prev: AccumulatedLogs) -> Self {
        let accum_logs_next = accum_logs_prev.clone();

        Self {
            state_manager,
            logs: Vec::new(),
            accum_logs_prev,
            accum_logs_next,
        }
    }

    fn get_without_logging_from_prev(
        &self,
        key: &[u8],
    ) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = if let Some(diff) = self.accum_logs_prev.diff().get(key) {
            diff.after.clone()
        } else {
            self.state_manager.get(key)?
        };

        Ok(val)
    }

    fn get_without_logging_from_next(
        &self,
        key: &[u8],
    ) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = if let Some(diff) = self.accum_logs_next.diff().get(key) {
            diff.after.clone()
        } else {
            self.state_manager.get(key)?
        };

        Ok(val)
    }

    pub fn commit(&self, state_manager: &mut S) -> Result<(), InterLiquidSdkError> {
        for (key, diff) in self.accum_logs_next.diff() {
            match &diff.after {
                Some(value) => state_manager.set(key, value)?,
                None => state_manager.del(key)?,
            }
        }

        Ok(())
    }

    pub fn state_for_access_from_log(
        &self,
    ) -> Result<BTreeMap<Vec<u8>, Vec<u8>>, InterLiquidSdkError> {
        let mut map = BTreeMap::new();
        for log in &self.logs {
            match log {
                StateLog::Read(read) => {
                    if let Some(value) = self.get_without_logging_from_prev(&read.key)? {
                        map.entry(read.key.clone()).or_insert(value);
                    }
                }
                StateLog::Iter(iter) => {
                    for key in &iter.keys {
                        if let Some(value) = self.get_without_logging_from_prev(&key)? {
                            map.entry(key.clone()).or_insert(value);
                        }
                    }
                }
                StateLog::Diff(_diff) => {}
            }
        }

        Ok(map)
    }
}

impl<'s, S: StateManager> TracableStateManager for TransactionalStateManager<'s, S> {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = self.get_without_logging_from_next(key)?;

        self.logs.push(StateLog::Read(StateLogRead {
            key: key.to_vec(),
            found: val.is_some(),
        }));

        Ok(val)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
        let before = self.get_without_logging_from_next(key)?;
        let log = StateLog::Diff(StateLogDiff {
            key: key.to_vec(),
            diff: ValueDiff {
                before,
                after: Some(value.to_vec()),
            },
        });

        self.accum_logs_next.apply_logs([log.clone()].into_iter())?;
        self.logs.push(log);

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
        let before = self.get_without_logging_from_next(key)?;
        let log = StateLog::Diff(StateLogDiff {
            key: key.to_vec(),
            diff: ValueDiff {
                before,
                after: None,
            },
        });

        self.accum_logs_next.apply_logs([log.clone()].into_iter())?;
        self.logs.push(log);

        Ok(())
    }

    fn iter<'a>(
        &'a mut self,
        key_prefix: Vec<u8>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        let iter = self
            .state_manager
            .iter(key_prefix.clone())
            .filter_map(|result| {
                let (key, value) = match result {
                    Ok((key, value)) => (key, value),
                    Err(e) => return Some(Err(e)),
                };

                if let Some(diff) = self.accum_logs_next.diff().get(&key) {
                    match &diff.after {
                        Some(value) => Some(Ok((key, value.clone()))),
                        None => None,
                    }
                } else {
                    Some(Ok((key, value)))
                }
            });

        let record_index = self.logs.len();
        self.logs
            .push(StateLog::Iter(StateLogIter::new(key_prefix)));

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
