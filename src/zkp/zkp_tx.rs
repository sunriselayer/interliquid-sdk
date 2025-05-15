use std::collections::BTreeMap;

use crate::{
    sha2::{Digest, Sha256},
    trie::NibblePatriciaTrieRootPath,
};
use anyhow::anyhow;
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{App, SdkContext, Tx},
    state::{CompressedDiffs, RelatedState, StateLog, StateManager, TransactionalStateManager},
    types::InterLiquidSdkError,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTx {
    pub tx_hash: [u8; 32],
    pub accum_diffs_hash_prev: [u8; 32],
    pub accum_diffs_hash_next: [u8; 32],
    pub entire_root: [u8; 32],
}

impl PublicInputTx {
    pub fn new(
        tx_hash: [u8; 32],
        accum_diffs_hash_prev: [u8; 32],
        accum_diffs_hash_next: [u8; 32],
        entire_root: [u8; 32],
    ) -> Self {
        Self {
            tx_hash,
            accum_diffs_hash_prev,
            accum_diffs_hash_next,
            entire_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessTx {
    pub tx: Vec<u8>,
    pub state_root: [u8; 32],
    pub keys_root: [u8; 32],
    pub state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
    pub accum_diffs_prev: CompressedDiffs,
    pub read_proof_path: NibblePatriciaTrieRootPath,
    pub iter_proof_path: NibblePatriciaTrieRootPath,
}

impl WitnessTx {
    pub fn new(
        tx: Vec<u8>,
        state_root: [u8; 32],
        keys_root: [u8; 32],
        state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
        accum_diffs_prev: CompressedDiffs,
        read_proof_path: NibblePatriciaTrieRootPath,
        iter_proof_path: NibblePatriciaTrieRootPath,
    ) -> Self {
        Self {
            tx,
            state_root,
            keys_root,
            state_for_access,
            accum_diffs_prev,
            read_proof_path,
            iter_proof_path,
        }
    }

    pub fn from<S: StateManager>(
        tx: Vec<u8>,
        state_root: [u8; 32],
        keys_root: [u8; 32],
        logs: &[StateLog],
        accum_diffs_prev: CompressedDiffs,
        state_manager: &S,
    ) -> Result<Self, InterLiquidSdkError> {
        let mut state_for_access = BTreeMap::new();

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

        let mut read_proof_path = NibblePatriciaTrieRootPath::new(BTreeMap::new(), BTreeMap::new());
        let mut iter_proof_path = NibblePatriciaTrieRootPath::new(BTreeMap::new(), BTreeMap::new());

        Ok(Self::new(
            tx,
            state_root,
            keys_root,
            state_for_access,
            accum_diffs_prev,
            read_proof_path,
            iter_proof_path,
        ))
    }
}

pub fn circuit_tx<TX: Tx>(
    witness: WitnessTx,
    app: &App<TX>,
) -> Result<PublicInputTx, InterLiquidSdkError> {
    let mut accum_diffs_bytes_prev = Vec::new();
    witness
        .accum_diffs_prev
        .serialize(&mut accum_diffs_bytes_prev)?;
    let accum_diffs_hash_prev = Sha256::digest(&accum_diffs_bytes_prev).into();

    let related_state = RelatedState::new(witness.state_for_access);
    let mut transactional =
        TransactionalStateManager::from_diffs(&related_state, witness.accum_diffs_prev);
    let mut ctx = SdkContext::new("".to_owned(), 0, 0, &mut transactional);

    app.execute_tx(&mut ctx, &witness.tx)?;

    let TransactionalStateManager {
        logs,
        diffs: accum_diffs_next,
        ..
    } = transactional;

    let RelatedState {
        map: state_for_access,
    } = related_state;

    // TODO: verify read_proof

    // TODO: verify iter_proof

    let mut tx_bytes = Vec::new();
    witness.tx.serialize(&mut tx_bytes)?;

    let tx_hash = Sha256::digest(&tx_bytes).into();

    let mut accum_diffs_bytes_next = Vec::new();
    accum_diffs_next.serialize(&mut accum_diffs_bytes_next)?;
    let accum_diffs_hash_next = Sha256::digest(&accum_diffs_bytes_next).into();

    let mut hasher = Sha256::new();
    hasher.update(witness.state_root);
    hasher.update(witness.keys_root);
    let entire_root = hasher.finalize().into();

    let public = PublicInputTx::new(
        tx_hash,
        accum_diffs_hash_prev,
        accum_diffs_hash_next,
        entire_root,
    );

    Ok(public)
}
