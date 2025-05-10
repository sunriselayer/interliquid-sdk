use std::collections::{BTreeMap, BTreeSet};

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

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
    pub keys: BTreeSet<Vec<u8>>,
}

impl StateLogIter {
    pub fn new() -> Self {
        Self {
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
pub struct CompressedDiffs {
    pub diffs: BTreeMap<Vec<u8>, ValueDiff>,
}

impl CompressedDiffs {
    pub fn new(diffs: BTreeMap<Vec<u8>, ValueDiff>) -> Self {
        Self { diffs }
    }

    pub fn from_logs(logs: &[StateLog]) -> Result<Self, InterLiquidSdkError> {
        let mut diffs = Self::default();
        diffs.apply_logs(logs)?;

        Ok(diffs)
    }

    pub fn apply_logs(&mut self, logs: &[StateLog]) -> Result<(), InterLiquidSdkError> {
        for log in logs {
            match log {
                StateLog::Diff(diff) => {
                    self.diffs
                        .entry(diff.key.clone())
                        .and_modify(|v: &mut ValueDiff| {
                            v.after = diff.diff.after.clone();
                        })
                        .or_insert(diff.diff.clone());
                }
                _ => {}
            }
        }

        Ok(())
    }
}
