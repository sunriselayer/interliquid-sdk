use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputBlock {
    pub tx_root: [u8; 32],
    pub entire_root_prev: [u8; 32],
    pub entire_root_next: [u8; 32],
}

impl PublicInputBlock {
    pub fn new(tx_root: [u8; 32], entire_root_prev: [u8; 32], entire_root_next: [u8; 32]) -> Self {
        Self {
            tx_root,
            entire_root_prev,
            entire_root_next,
        }
    }
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

impl PrivateInputBlock {
    pub fn new(
        state_sparse_tree_root_prev: [u8; 32],
        state_sparse_tree_root_next: [u8; 32],
        keys_patricia_trie_root_prev: [u8; 32],
        keys_patricia_trie_root_next: [u8; 32],
        accum_diffs_hash: [u8; 32],
        proof_tx_agg: Vec<u8>,
        proof_commit_state: Vec<u8>,
        proof_commit_keys: Vec<u8>,
    ) -> Self {
        Self {
            state_sparse_tree_root_prev,
            state_sparse_tree_root_next,
            keys_patricia_trie_root_prev,
            keys_patricia_trie_root_next,
            accum_diffs_hash,
            proof_tx_agg,
            proof_commit_state,
            proof_commit_keys,
        }
    }
}
