use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::state::CompressedDiffs;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputSparseTree {
    pub state_root_prev: [u8; 32],
    pub state_root_next: [u8; 32],
    pub accum_diffs_final_hash: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputSparseTree {
    pub state_prev_for_commit: BTreeMap<Vec<u8>, Vec<u8>>,
    pub accum_diffs_final: CompressedDiffs,
}
