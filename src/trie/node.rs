use crate::sha2::{Digest, Sha256};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;

use super::Nibble;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum NibblePatriciaTrieNode {
    Leaf(NibblePatriciaTrieNodeLeaf),
    Branch(NibblePatriciaTrieNodeBranch),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct NibblePatriciaTrieNodeLeaf {
    pub key_fragment: Vec<Nibble>,
    pub value: Vec<u8>,
}

impl NibblePatriciaTrieNodeLeaf {
    pub fn new(key_fragment: Vec<Nibble>, value: Vec<u8>) -> Self {
        Self {
            key_fragment,
            value,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(Nibble::as_slice(&self.key_fragment));
        hasher.update(&self.value);
        hasher.finalize().into()
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct NibblePatriciaTrieNodeBranch {
    pub key_fragment: Vec<Nibble>,
    pub child_key_fragments: BTreeMap<Nibble, Vec<Nibble>>,
}

impl NibblePatriciaTrieNodeBranch {
    pub fn new(
        key_fragment: Vec<Nibble>,
        child_key_fragments: BTreeMap<Nibble, Vec<Nibble>>,
    ) -> Self {
        Self {
            key_fragment,
            child_key_fragments,
        }
    }

    pub fn hash(&self, child_hash: impl Fn(&Nibble) -> [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(Nibble::as_slice(&self.key_fragment));
        for (index, _child_key_fragment) in self.child_key_fragments.iter() {
            hasher.update([index.as_u8()]);
            hasher.update(child_hash(index));
        }
        hasher.finalize().into()
    }
}
