use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

use crate::merkle::{bitmap::OctRadBitmap, consts::HASH_BYTES};

use super::OctRadPatriciaTrieError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum OctRadPatriciaNode {
    Leaf(OctRadPatriciaNodeLeaf),
    Branch(OctRadPatriciaNodeBranch),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaNodeLeaf {
    pub key_fragment: Vec<u8>,
    pub value: Vec<u8>,
}

impl OctRadPatriciaNodeLeaf {
    pub fn new(key_fragment: Vec<u8>, value: Vec<u8>) -> Self {
        Self {
            key_fragment,
            value,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key_fragment);
        hasher.update(&self.value);
        hasher.finalize().into()
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaNodeBranch {
    pub key_fragment: Vec<u8>,
    pub child_bitmap: OctRadBitmap,
    pub children: Vec<OctRadPatriciaNode>,
}

impl OctRadPatriciaNodeBranch {
    pub fn new(
        key_fragment: Vec<u8>,
        child_bitmap: OctRadBitmap,
        children: Vec<OctRadPatriciaNode>,
    ) -> Self {
        Self {
            key_fragment,
            child_bitmap,
            children,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();

        hasher.update(&self.key_fragment);
        hasher.update(&self.child_bitmap);

        for child in self.children.iter() {
            hasher.update(&child.hash());
        }

        hasher.finalize().into()
    }

    pub fn hash_from_child_hashes<'a>(
        key_fragment: &[u8],
        child_bitmap: &OctRadBitmap,
        child_hashes: impl Iterator<Item = &'a [u8; HASH_BYTES]>,
    ) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();

        hasher.update(key_fragment);
        hasher.update(child_bitmap);

        for child_hash in child_hashes {
            hasher.update(child_hash);
        }

        hasher.finalize().into()
    }
}

impl OctRadPatriciaNode {
    pub fn key_fragment(&self) -> &Vec<u8> {
        match self {
            OctRadPatriciaNode::Leaf(leaf) => &leaf.key_fragment,
            OctRadPatriciaNode::Branch(branch) => &branch.key_fragment,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        match self {
            OctRadPatriciaNode::Leaf(leaf) => leaf.hash(),
            OctRadPatriciaNode::Branch(branch) => branch.hash(),
        }
    }

    pub fn from_map(
        key_fragment: &[u8],
        key_suffixes: &BTreeSet<Vec<u8>>,
        value: impl Fn(&Vec<u8>) -> Vec<u8>,
    ) -> Result<Self, OctRadPatriciaTrieError> {
        todo!()
    }
}
