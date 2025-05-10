use std::collections::{BTreeMap, HashMap};

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::state::CompressedDiffs;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputBlock {
    pub tx_root: [u8; 32],
    pub state_root_prev: [u8; 32],
    pub state_root_next: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputTx {
    pub tx_hashes: Vec<[u8; 32]>,
    pub tx_proofs: Vec<Vec<u8>>,
    pub accum_diffs_hashes: Vec<[u8; 32]>,
    pub accum_diffs_final: CompressedDiffs,
    pub state_for_commit: BTreeMap<Vec<u8>, Vec<u8>>,
}
