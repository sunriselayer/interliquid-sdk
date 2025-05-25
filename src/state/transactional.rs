use std::collections::{BTreeMap, BTreeSet};

use crate::{state::TracableStateManager, types::InterLiquidSdkError};

use super::{
    log::{StateLog, StateLogIter, StateLogRead},
    AccumulatedLogs, StateLogDiff, StateManager, ValueDiff,
};

/// A state manager wrapper that tracks all state operations in a transaction.
/// Maintains logs of reads, writes, and iterations for verification and rollback.
pub struct TransactionalStateManager<'s, S: StateManager> {
    /// The underlying state manager being wrapped
    pub state_manager: &'s S,
    /// Sequential log of all state operations performed in this transaction
    pub logs: Vec<StateLog>,
    /// Accumulated logs from previous transactions (starting state)
    pub accum_logs_prev: AccumulatedLogs,
    /// Accumulated logs including current transaction (ending state)
    pub accum_logs_next: AccumulatedLogs,
}

impl<'s, S: StateManager> TransactionalStateManager<'s, S> {
    /// Creates a new transactional state manager wrapping the given state manager.
    /// 
    /// # Arguments
    /// 
    /// * `state_manager` - The underlying state manager to wrap
    pub fn new(state_manager: &'s S) -> Self {
        Self {
            state_manager,
            logs: Vec::new(),
            accum_logs_prev: AccumulatedLogs::default(),
            accum_logs_next: AccumulatedLogs::default(),
        }
    }

    /// Creates a new transactional state manager with previous accumulated logs.
    /// Used for nested transactions or continuing from a previous state.
    /// 
    /// # Arguments
    /// 
    /// * `state_manager` - The underlying state manager to wrap
    /// * `accum_logs_prev` - Previously accumulated logs to start from
    pub fn from_accum_logs_prev(state_manager: &'s S, accum_logs_prev: AccumulatedLogs) -> Self {
        let accum_logs_next = accum_logs_prev.clone();

        Self {
            state_manager,
            logs: Vec::new(),
            accum_logs_prev,
            accum_logs_next,
        }
    }

    /// Gets a value using the previous accumulated state without logging the read.
    /// Used internally to check state before modifications.
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

    /// Gets a value using the current accumulated state without logging the read.
    /// Reflects all modifications made in the current transaction.
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

    /// Commits all accumulated changes to the given state manager.
    /// Applies all modifications (sets and deletes) from the transaction.
    /// 
    /// # Arguments
    /// 
    /// * `state_manager` - The state manager to commit changes to
    pub fn commit(&self, state_manager: &mut S) -> Result<(), InterLiquidSdkError> {
        for (key, diff) in self.accum_logs_next.diff() {
            match &diff.after {
                Some(value) => state_manager.set(key, value)?,
                None => state_manager.del(key)?,
            }
        }

        Ok(())
    }

    /// Constructs a map of all state that was accessed during the transaction.
    /// Used for proving state access patterns and dependencies.
    /// 
    /// # Returns
    /// 
    /// A map containing all key-value pairs that were read or iterated over
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
    /// Gets a value from the state and logs the read operation.
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
        let val = self.get_without_logging_from_next(key)?;

        self.logs.push(StateLog::Read(StateLogRead {
            key: key.to_vec(),
            found: val.is_some(),
        }));

        Ok(val)
    }

    /// Sets a value in the state and logs the modification.
    /// Records the before and after values for the key.
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

    /// Deletes a key from the state and logs the modification.
    /// Records the before value and marks the key as deleted.
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

    /// Creates an iterator over key-value pairs and logs all accessed keys.
    /// The iterator tracks which keys were accessed during iteration.
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

/// Records keys accessed during iteration operations.
/// Used for tracking state dependencies.
pub struct IterRecord {
    /// Set of keys that were accessed during iteration
    pub keys: BTreeSet<Vec<u8>>,
}

impl IterRecord {
    /// Creates a new empty iteration record.
    pub fn new() -> Self {
        Self {
            keys: BTreeSet::new(),
        }
    }
}
/// Iterator wrapper that records all accessed keys during iteration.
/// Ensures complete tracking of state access for verification.
pub struct TransactionalStateIterator<'a> {
    /// The state log where accessed keys are recorded
    recorder: &'a mut StateLogIter,
    /// The underlying iterator being wrapped
    iterator: Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>,
}

impl<'a> TransactionalStateIterator<'a> {
    /// Creates a new transactional iterator that records accessed keys.
    /// 
    /// # Arguments
    /// 
    /// * `recorder` - The state log to record accessed keys to
    /// * `iterator` - The underlying iterator to wrap
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

/// Drop implementation ensures all keys are recorded even if iteration stops early.
/// This is needed to enforce that all keys are recorded.
/// This makes the proof of range completeness proof easier.
impl<'a> Drop for TransactionalStateIterator<'a> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}
