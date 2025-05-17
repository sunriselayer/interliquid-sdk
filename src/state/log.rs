use std::collections::{BTreeMap, BTreeSet};

use anyhow::anyhow;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

use super::bytes_prefix_range;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum StateLog {
    Read(StateLogRead),
    Iter(StateLogIter),
    Diff(StateLogDiff),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateLogRead {
    pub key: Vec<u8>,
    pub found: bool,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateLogIter {
    pub key_prefix: Vec<u8>,
    pub keys: BTreeSet<Vec<u8>>,
}

impl StateLogIter {
    pub fn new(key_prefix: Vec<u8>) -> Self {
        Self {
            key_prefix,
            keys: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateLogDiff {
    pub key: Vec<u8>,
    pub diff: ValueDiff,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct ValueDiff {
    pub before: Option<Vec<u8>>,
    pub after: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct AccumulatedLogs {
    pub read: BTreeMap<Vec<u8>, bool>,
    pub iter: BTreeMap<Vec<u8>, BTreeSet<Vec<u8>>>,
    pub diff: BTreeMap<Vec<u8>, ValueDiff>,
}

impl AccumulatedLogs {
    pub fn new() -> Self {
        Self {
            read: BTreeMap::new(),
            iter: BTreeMap::new(),
            diff: BTreeMap::new(),
        }
    }

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

    pub fn read(&self) -> &BTreeMap<Vec<u8>, bool> {
        &self.read
    }

    pub fn iter(&self) -> &BTreeMap<Vec<u8>, BTreeSet<Vec<u8>>> {
        &self.iter
    }

    pub fn diff(&self) -> &BTreeMap<Vec<u8>, ValueDiff> {
        &self.diff
    }
}
