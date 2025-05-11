use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

use crate::merkle::{bitmap::OctRadBitmap, consts::HASH_BYTES};

use super::OctRadPatriciaTrieError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum OctRadPatriciaTrieNode {
    Leaf(OctRadPatriciaTrieNodeLeaf),
    Branch(OctRadPatriciaTrieNodeBranch),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaTrieNodeLeaf {
    pub key_fragment: Vec<u8>,
}

impl OctRadPatriciaTrieNodeLeaf {
    pub fn new(key_fragment: Vec<u8>) -> Self {
        Self { key_fragment }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key_fragment);
        hasher.finalize().into()
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaTrieNodeBranch {
    pub key_fragment: Vec<u8>,
    pub child_bitmap: OctRadBitmap,
    pub children: Vec<OctRadPatriciaTrieNode>,
}

impl OctRadPatriciaTrieNodeBranch {
    pub fn new(
        key_fragment: Vec<u8>,
        child_bitmap: OctRadBitmap,
        children: Vec<OctRadPatriciaTrieNode>,
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

impl OctRadPatriciaTrieNode {
    pub fn key_fragment(&self) -> &Vec<u8> {
        match self {
            OctRadPatriciaTrieNode::Leaf(leaf) => &leaf.key_fragment,
            OctRadPatriciaTrieNode::Branch(branch) => &branch.key_fragment,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        match self {
            OctRadPatriciaTrieNode::Leaf(leaf) => leaf.hash(),
            OctRadPatriciaTrieNode::Branch(branch) => branch.hash(),
        }
    }

    pub fn from_child_suffixes(
        key_fragment: &[u8],
        child_key_suffixes: &BTreeSet<Vec<u8>>,
    ) -> Result<Self, OctRadPatriciaTrieError> {
        if child_key_suffixes.is_empty() {
            return Err(OctRadPatriciaTrieError::EmptyKeySet);
        }

        // Stack to store pending nodes to process
        #[derive(Debug)]
        struct StackItem {
            key_fragment: Vec<u8>,
            suffixes: BTreeSet<Vec<u8>>,
            parent_byte: Option<u8>,
        }

        let mut stack = vec![StackItem {
            key_fragment: key_fragment.to_vec(),
            suffixes: child_key_suffixes.clone(),
            parent_byte: None,
        }];
        let mut node_stack = Vec::new();

        while let Some(item) = stack.pop() {
            // Group key suffixes by their first byte
            let mut children_map: std::collections::BTreeMap<u8, BTreeSet<Vec<u8>>> =
                std::collections::BTreeMap::new();
            for suffix in item.suffixes {
                if suffix.is_empty() {
                    return Err(OctRadPatriciaTrieError::EmptyKeySuffix);
                }
                let first_byte = suffix[0];
                let rest = suffix[1..].to_vec();
                children_map.entry(first_byte).or_default().insert(rest);
            }

            // If there's only one child and it has no remaining suffix, create a leaf node
            if children_map.len() == 1 {
                let (byte, suffixes) = children_map.iter().next().unwrap();
                if suffixes.len() == 1 && suffixes.iter().next().unwrap().is_empty() {
                    let mut full_key = item.key_fragment.clone();
                    full_key.push(*byte);
                    node_stack.push((
                        item.parent_byte,
                        OctRadPatriciaTrieNode::Leaf(OctRadPatriciaTrieNodeLeaf::new(full_key)),
                    ));
                    continue;
                }
            }

            // Create child nodes
            let mut children = Vec::new();
            let mut child_bitmap = OctRadBitmap::default();

            // Push children to stack in reverse order to maintain correct order
            for (byte, suffixes) in children_map.into_iter().rev() {
                child_bitmap.set(byte, true);
                let mut child_key = item.key_fragment.clone();
                child_key.push(byte);
                stack.push(StackItem {
                    key_fragment: child_key,
                    suffixes,
                    parent_byte: Some(byte),
                });
            }

            // Wait for all children to be processed
            while node_stack
                .last()
                .map_or(false, |(b, _)| *b == item.parent_byte)
            {
                let (_, child) = node_stack.pop().unwrap();
                children.push(child);
            }

            // Create branch node
            let branch = OctRadPatriciaTrieNode::Branch(OctRadPatriciaTrieNodeBranch::new(
                item.key_fragment,
                child_bitmap,
                children,
            ));
            node_stack.push((item.parent_byte, branch));
        }

        // The root node should be the only remaining node
        Ok(node_stack.pop().unwrap().1)
    }
}
