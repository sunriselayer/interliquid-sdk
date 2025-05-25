use std::collections::{BTreeMap, BTreeSet};

use anyhow::anyhow;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

use super::bytes_prefix_range;

/// Represents different types of state operations that can be logged.
/// Used to track state changes and access patterns during transaction execution.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum StateLog {
    Read(StateLogRead),
    Iter(StateLogIter),
    Diff(StateLogDiff),
}

/// Represents a read operation on the state.
/// Tracks which keys were read and whether they were found.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateLogRead {
    /// The key that was read from the state
    pub key: Vec<u8>,
    /// Whether the key existed in the state
    pub found: bool,
}

/// Represents an iteration operation over the state.
/// Tracks which keys were accessed during iteration with a specific prefix.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateLogIter {
    /// The prefix used to filter keys during iteration
    pub key_prefix: Vec<u8>,
    /// Set of all keys that were accessed during the iteration
    pub keys: BTreeSet<Vec<u8>>,
}

impl StateLogIter {
    /// Creates a new iterator log with the given key prefix.
    /// 
    /// # Arguments
    /// 
    /// * `key_prefix` - The prefix to filter keys during iteration
    pub fn new(key_prefix: Vec<u8>) -> Self {
        Self {
            key_prefix,
            keys: BTreeSet::new(),
        }
    }
}

/// Represents a state modification (write or delete) operation.
/// Contains the key and the difference between before and after values.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateLogDiff {
    /// The key that was modified
    pub key: Vec<u8>,
    /// The difference between the old and new values
    pub diff: ValueDiff,
}

/// Represents the difference between state values before and after a modification.
/// Used to track state changes for rollback and verification purposes.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ValueDiff {
    /// The value before the modification (None if key didn't exist)
    pub before: Option<Vec<u8>>,
    /// The value after the modification (None if key was deleted)
    pub after: Option<Vec<u8>>,
}

/// Accumulates and manages state logs from multiple operations.
/// Maintains separate collections for reads, iterations, and modifications.
#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct AccumulatedLogs {
    /// Map of keys to whether they were found during read operations
    pub read: BTreeMap<Vec<u8>, bool>,
    /// Map of key prefixes to sets of keys accessed during iteration
    pub iter: BTreeMap<Vec<u8>, BTreeSet<Vec<u8>>>,
    /// Map of keys to their value differences (modifications)
    pub diff: BTreeMap<Vec<u8>, ValueDiff>,
}

impl AccumulatedLogs {
    /// Creates a new empty AccumulatedLogs instance.
    pub fn new() -> Self {
        Self {
            read: BTreeMap::new(),
            iter: BTreeMap::new(),
            diff: BTreeMap::new(),
        }
    }

    /// Applies a sequence of state logs to the accumulated state.
    /// Validates consistency between reads and modifications.
    /// 
    /// # Arguments
    /// 
    /// * `logs` - Iterator of state logs to apply
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if all logs were applied successfully
    /// * `Err` if inconsistencies are detected (e.g., reading from a deleted key)
    pub fn apply_logs(
        &mut self,
        logs: impl Iterator<Item = StateLog>,
    ) -> Result<(), InterLiquidSdkError> {
        for log in logs {
            match log {
                StateLog::Read(read) => {
                    if let Some(diff) = self.diff.get(&read.key) {
                        if read.found {
                            if diff.after.is_none() {
                                return Err(InterLiquidSdkError::Other(anyhow!(
                                    "inconsistent state log: read found after deletion diff"
                                )));
                            }

                            if diff.before.is_none() {
                                continue;
                            }
                        } else {
                            if diff.after.is_some() {
                                return Err(InterLiquidSdkError::Other(anyhow!(
                                    "inconsistent state log: read not found after insertion diff"
                                )));
                            }
                            if diff.before.is_some() {
                                continue;
                            }
                        }
                    }

                    // do not overwrite
                    self.read.entry(read.key).or_insert(read.found);
                }
                StateLog::Iter(mut iter) => {
                    let diffs_under_prefix =
                        bytes_prefix_range(&self.diff, iter.key_prefix.clone());

                    for (key, diff) in diffs_under_prefix {
                        if diff.before.is_none() && diff.after.is_some() {
                            iter.keys.remove(&key);
                        } else if diff.before.is_some() && diff.after.is_none() {
                            iter.keys.insert(key);
                        }
                    }

                    // do not overwrite
                    self.iter.entry(iter.key_prefix).or_insert(iter.keys);
                }
                StateLog::Diff(diff) => {
                    self.diff
                        .entry(diff.key.clone())
                        .and_modify(|v: &mut ValueDiff| {
                            v.after = diff.diff.after.clone();
                        })
                        .or_insert(diff.diff.clone());
                }
            }
        }

        Ok(())
    }

    /// Returns a reference to the accumulated read operations.
    /// Maps keys to whether they were found during the read.
    pub fn read(&self) -> &BTreeMap<Vec<u8>, bool> {
        &self.read
    }

    /// Returns a reference to the accumulated iteration operations.
    /// Maps key prefixes to sets of keys accessed during iteration.
    pub fn iter(&self) -> &BTreeMap<Vec<u8>, BTreeSet<Vec<u8>>> {
        &self.iter
    }

    /// Returns a reference to the accumulated state modifications.
    /// Maps keys to their value differences (before and after).
    pub fn diff(&self) -> &BTreeMap<Vec<u8>, ValueDiff> {
        &self.diff
    }
}
