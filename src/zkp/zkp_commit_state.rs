use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{merkle::OctRadSparseTreePath, state::CompressedDiffs};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputSparseTree {
    pub state_sparse_tree_root_prev: [u8; 32],
    pub state_sparse_tree_root_next: [u8; 32],
    pub accum_diffs_final_hash: [u8; 32],
}

impl PublicInputSparseTree {
    pub fn new(
        state_sparse_tree_root_prev: [u8; 32],
        state_sparse_tree_root_next: [u8; 32],
        accum_diffs_final_hash: [u8; 32],
    ) -> Self {
        Self {
            state_sparse_tree_root_prev,
            state_sparse_tree_root_next,
            accum_diffs_final_hash,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputSparseTree {
    pub accum_diffs_final: CompressedDiffs,
    pub state_commit_path: OctRadSparseTreePath,
}

impl PrivateInputSparseTree {
    pub fn new(
        accum_diffs_final: CompressedDiffs,
        state_commit_path: OctRadSparseTreePath,
    ) -> Self {
        Self {
            accum_diffs_final,
            state_commit_path,
        }
    }
}
