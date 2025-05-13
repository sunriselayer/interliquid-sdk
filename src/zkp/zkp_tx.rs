use std::collections::BTreeMap;

use anyhow::anyhow;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    merkle::{OctRadPatriciaTriePath, OctRadSparseTreePath},
    state::{CompressedDiffs, StateLog, StateManager},
    types::InterLiquidSdkError,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTx {
    pub tx_hash: [u8; 32],
    pub accum_diffs_hash_prev: [u8; 32],
    pub accum_diffs_hash_next: [u8; 32],
    pub entire_state_root: [u8; 32],
}

impl PublicInputTx {
    pub fn new(
        tx_hash: [u8; 32],
        accum_diffs_hash_prev: [u8; 32],
        accum_diffs_hash_next: [u8; 32],
        entire_state_root: [u8; 32],
    ) -> Self {
        Self {
            tx_hash,
            accum_diffs_hash_prev,
            accum_diffs_hash_next,
            entire_state_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessTx {
    pub tx: Vec<u8>,
    pub state_sparse_tree_root: [u8; 32],
    pub keys_patricia_trie_root: [u8; 32],
    pub state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
    pub accum_diffs_prev: CompressedDiffs,
    pub read_proof_path: OctRadSparseTreePath,
    pub iter_proof_path: OctRadPatriciaTriePath,
}

impl WitnessTx {
    pub fn new(
        tx: Vec<u8>,
        state_sparse_tree_root: [u8; 32],
        keys_patricia_trie_root: [u8; 32],
        state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
        accum_diffs_prev: CompressedDiffs,
        read_proof_path: OctRadSparseTreePath,
        iter_proof_path: OctRadPatriciaTriePath,
    ) -> Self {
        Self {
            tx,
            state_sparse_tree_root,
            keys_patricia_trie_root,
            state_for_access,
            accum_diffs_prev,
            read_proof_path,
            iter_proof_path,
        }
    }

    pub fn from<S: StateManager>(
        tx: Vec<u8>,
        state_sparse_tree_root: [u8; 32],
        keys_patricia_trie_root: [u8; 32],
        logs: &[StateLog],
        accum_diffs_prev: CompressedDiffs,
        state_manager: &S,
    ) -> Result<Self, InterLiquidSdkError> {
        let mut state_for_access = BTreeMap::new();
        let mut read_proof_path = OctRadSparseTreePath::new(BTreeMap::new());
        let mut iter_proof_path = OctRadPatriciaTriePath::new(BTreeMap::new());

        for (i, log) in logs.iter().enumerate() {
            match log {
                StateLog::Read(read) => {
                    let latest_diff = (0..i)
                        .rev()
                        .map(|j| &logs[j])
                        .filter_map(|log| {
                            if let StateLog::Diff(diff) = log {
                                if diff.key == read.key {
                                    return Some(diff);
                                }
                            }

                            None
                        })
                        .next();

                    if read.found {
                        if let Some(diff) = latest_diff {
                            match diff.diff.after {
                                Some(_) => {
                                    continue;
                                }
                                None => {
                                    return Err(InterLiquidSdkError::Other(anyhow!(
                                        "Inconsistent logs: read.found == true after diff.after == None"
                                    )));
                                }
                            }
                        }

                        let value = state_manager.get(&read.key)?;
                        match value {
                            Some(value) => {
                                state_for_access.insert(read.key.clone(), value);
                            }
                            None => {
                                return Err(InterLiquidSdkError::Other(anyhow!(
                                    "Inconsistent logs and state"
                                )));
                            }
                        }

                        // TODO: add read proof path
                    } else {
                        if let Some(diff) = latest_diff {
                            match diff.diff.after {
                                Some(_) => {
                                    return Err(InterLiquidSdkError::Other(anyhow!(
                                        "Inconsistent logs: read.found == false after diff.after == Some"
                                    )));
                                }
                                None => {
                                    continue;
                                }
                            }
                        }

                        // TODO: add read proof path
                    }
                }
                StateLog::Iter(iter) => {
                    // TODO: add iter proof path
                }
                StateLog::Diff(_diff) => {}
            }
        }

        Ok(Self::new(
            tx,
            state_sparse_tree_root,
            keys_patricia_trie_root,
            state_for_access,
            accum_diffs_prev,
            read_proof_path,
            iter_proof_path,
        ))
    }
}
