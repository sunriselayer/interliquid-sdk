use std::collections::BTreeMap;

use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

use crate::{merkle::OctRadSparseTreePath, state::CompressedDiffs, types::InterLiquidSdkError};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputCommitState {
    pub state_sparse_tree_root_prev: [u8; 32],
    pub state_sparse_tree_root_next: [u8; 32],
    pub accum_diffs_hash_final: [u8; 32],
}

impl PublicInputCommitState {
    pub fn new(
        state_sparse_tree_root_prev: [u8; 32],
        state_sparse_tree_root_next: [u8; 32],
        accum_diffs_hash_final: [u8; 32],
    ) -> Self {
        Self {
            state_sparse_tree_root_prev,
            state_sparse_tree_root_next,
            accum_diffs_hash_final,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessCommitState {
    pub state_sparse_tree_root_prev: [u8; 32],
    pub accum_diffs_final: CompressedDiffs,
    pub state_commit_path: OctRadSparseTreePath,
}

impl WitnessCommitState {
    pub fn new(
        state_sparse_tree_root_prev: [u8; 32],
        accum_diffs_final: CompressedDiffs,
        state_commit_path: OctRadSparseTreePath,
    ) -> Self {
        Self {
            state_sparse_tree_root_prev,
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

    // TODO
    let remainder_nodes = BTreeMap::new();

    let state_sparse_tree_root_next = witness.state_commit_path.root(&remainder_nodes);

    let accum_diffs_hash_final = Sha256::digest(&accum_diffs_bytes_final).into();

    let input = PublicInputCommitState::new(
        witness.state_sparse_tree_root_prev,
        state_sparse_tree_root_next,
        accum_diffs_hash_final,
    );

    Ok(input)
}
