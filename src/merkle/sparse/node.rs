use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

use crate::merkle::{bitmap::OctRadBitmap, consts::HASH_BYTES};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum OctRadSparseTreeNode {
    Leaf(OctRadSparseTreeNodeLeaf),
    Branch(OctRadSparseTreeNodeBranch),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreeNodeLeaf {
    pub key_hash_fragment: u8,
    pub value: Vec<u8>,
}

impl OctRadSparseTreeNodeLeaf {
    pub fn new(key_hash_fragment: u8, value: Vec<u8>) -> Self {
        Self {
            key_hash_fragment,
            value,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();
        hasher.update(&[self.key_hash_fragment]);
        hasher.update(&self.value);
        hasher.finalize().into()
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreeNodeBranch {
    pub key_hash_fragment: u8,
    pub child_hashes: BTreeMap<u8, [u8; HASH_BYTES]>,
}

impl OctRadSparseTreeNodeBranch {
    pub fn new(key_hash_fragment: u8, child_hashes: BTreeMap<u8, [u8; HASH_BYTES]>) -> Self {
        Self {
            key_hash_fragment,
            child_hashes,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let child_bitmap = OctRadBitmap::from_index_set(self.child_hashes.keys().copied());
        let mut hasher = Sha256::new();

        hasher.update(&[self.key_hash_fragment]);
        hasher.update(&child_bitmap);

        for child_hash in self.child_hashes.values() {
            hasher.update(child_hash);
        }

        hasher.finalize().into()
    }

    pub fn hash_from_child_hashes_iter<'a>(
        key_hash_fragment: u8,
        child_bitmap: &OctRadBitmap,
        child_hashes: impl Iterator<Item = &'a [u8; HASH_BYTES]>,
    ) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();

        hasher.update(&[key_hash_fragment]);
        hasher.update(child_bitmap);

        for child_hash in child_hashes {
            hasher.update(child_hash);
        }

        hasher.finalize().into()
    }
}

impl OctRadSparseTreeNode {
    pub fn key_hash_fragment(&self) -> u8 {
        match self {
            OctRadSparseTreeNode::Leaf(leaf) => leaf.key_hash_fragment,
            OctRadSparseTreeNode::Branch(branch) => branch.key_hash_fragment,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        match self {
            OctRadSparseTreeNode::Leaf(leaf) => leaf.hash(),
            OctRadSparseTreeNode::Branch(branch) => branch.hash(),
        }
    }
}
