use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputBlock {
    pub tx_root: [u8; 32],
    pub entire_root_prev: [u8; 32],
    pub entire_root_next: [u8; 32],
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PrivateInputBlock {
    pub state_sparse_tree_root_prev: [u8; 32],
    pub state_sparse_tree_root_next: [u8; 32],
    pub keys_patricia_trie_root_prev: [u8; 32],
    pub keys_patricia_trie_root_next: [u8; 32],
    pub accum_diffs_hash: [u8; 32],
    pub proof_tx_agg: Vec<u8>,
    pub proof_commit_state: Vec<u8>,
    pub proof_commit_keys: Vec<u8>,
}
