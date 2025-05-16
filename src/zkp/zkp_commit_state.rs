use crate::{
    sha2::{Digest, Sha256},
    trie::{nibbles_from_bytes, NibblePatriciaTrieError, NibblePatriciaTrieRootPath},
};
use anyhow::anyhow;
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{state::CompressedDiffs, types::InterLiquidSdkError};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputCommitState {
    pub state_root_prev: [u8; 32],
    pub state_root_next: [u8; 32],
    pub accum_diffs_hash_final: [u8; 32],
}

impl PublicInputCommitState {
    pub fn new(
        state_root_prev: [u8; 32],
        state_root_next: [u8; 32],
        accum_diffs_hash_final: [u8; 32],
    ) -> Self {
        Self {
            state_root_prev,
            state_root_next,
            accum_diffs_hash_final,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessCommitState {
    pub state_root_prev: [u8; 32],
    pub accum_diffs_final: CompressedDiffs,
    pub state_commit_path: NibblePatriciaTrieRootPath,
}

impl WitnessCommitState {
    pub fn new(
        state_root_prev: [u8; 32],
        accum_diffs_final: CompressedDiffs,
        state_commit_path: NibblePatriciaTrieRootPath,
    ) -> Self {
        Self {
            state_root_prev,
            accum_diffs_final,
            state_commit_path,
        }
    }
}

pub fn circuit_commit_state(
    witness: WitnessCommitState,
) -> Result<PublicInputCommitState, InterLiquidSdkError> {
    let mut accum_diffs_bytes_final = Vec::new();
    witness
        .accum_diffs_final
        .serialize(&mut accum_diffs_bytes_final)?;

    let nodes_for_inclusion_proof_prev = witness
        .accum_diffs_final
        .diffs
        .iter()
        .filter_map(|(key, diff)| {
            if let Some(before) = &diff.before {
                let key_hash: [u8; 32] = Sha256::digest(key).into();
                Some((key_hash, before))
            } else {
                None
            }
        })
        .map(|(k, v)| {
            let leaf_key = nibbles_from_bytes(&k);
            let leaf_node = witness
                .state_commit_path
                .node_for_inclusion_proof(&leaf_key, v.clone())?;
            Ok((leaf_key, leaf_node))
        })
        .collect::<Result<_, NibblePatriciaTrieError>>()?;
    let state_root_prev = witness
        .state_commit_path
        .clone()
        .root(nodes_for_inclusion_proof_prev, None)?;

    if state_root_prev != witness.state_root_prev {
        return Err(InterLiquidSdkError::Other(anyhow!(
            "Inconsistent state_sparse_tree_root_prev and state_commit_path"
        )));
    }

    let nodes_for_inclusion_proof_next = witness
        .accum_diffs_final
        .diffs
        .iter()
        .filter_map(|(key, diff)| {
            if let Some(after) = &diff.after {
                let key_hash: [u8; 32] = Sha256::digest(key).into();
                Some((key_hash, after))
            } else {
                None
            }
        })
        .map(|(k, v)| {
            let leaf_key = nibbles_from_bytes(&k);
            let leaf_node = witness
                .state_commit_path
                .node_for_inclusion_proof(&leaf_key, v.clone())?;
            Ok((leaf_key, leaf_node))
        })
        .collect::<Result<_, NibblePatriciaTrieError>>()?;
    let state_root_next = witness
        .state_commit_path
        .root(nodes_for_inclusion_proof_next, None)?;

    let accum_diffs_hash_final = Sha256::digest(&accum_diffs_bytes_final).into();

    let input = PublicInputCommitState::new(
        witness.state_root_prev,
        state_root_next,
        accum_diffs_hash_final,
    );

    Ok(input)
}
