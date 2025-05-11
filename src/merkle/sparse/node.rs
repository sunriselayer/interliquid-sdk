use std::collections::{BTreeMap, BTreeSet};

use borsh_derive::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

use crate::merkle::{bitmap::OctRadBitmap, consts::HASH_BYTES};

use super::OctRadSparseTreeError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum OctRadSparseTreeNode {
    Leaf(OctRadSparseTreeNodeLeaf),
    Branch(OctRadSparseTreeNodeBranch),
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreeNodeLeaf {
    pub key_hash_fragment: Vec<u8>,
}

impl OctRadSparseTreeNodeLeaf {
    pub fn new(key_hash_fragment: Vec<u8>) -> Self {
        Self { key_hash_fragment }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key_hash_fragment);
        hasher.finalize().into()
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreeNodeBranch {
    pub key_hash_fragment: Vec<u8>,
    pub child_hashes: BTreeMap<u8, [u8; HASH_BYTES]>,
}

impl OctRadSparseTreeNodeBranch {
    pub fn new(
        key_hash_fragment: Vec<u8>,
        child_hashes: BTreeMap<u8, [u8; HASH_BYTES]>,
    ) -> Self {
        Self {
            key_hash_fragment,
            child_hashes,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key_hash_fragment);
        
        for (_, hash) in self.child_hashes.iter() {
            hasher.update(hash);
        }

        hasher.finalize().into()
    }

    pub fn hash_from_child_hashes<'a>(
        key_hash_fragment: &[u8],
        child_hashes: impl Iterator<Item = &'a [u8; HASH_BYTES]>,
    ) -> [u8; HASH_BYTES] {
        let mut hasher = Sha256::new();
        hasher.update(key_hash_fragment);

        for child_hash in child_hashes {
            hasher.update(child_hash);
        }

        hasher.finalize().into()
    }
}

impl OctRadSparseTreeNode {
    pub fn key_hash_fragment(&self) -> &Vec<u8> {
        match self {
            OctRadSparseTreeNode::Leaf(leaf) => &leaf.key_hash_fragment,
            OctRadSparseTreeNode::Branch(branch) => &branch.key_hash_fragment,
        }
    }

    pub fn hash(&self) -> [u8; HASH_BYTES] {
        match self {
            OctRadSparseTreeNode::Leaf(leaf) => leaf.hash(),
            OctRadSparseTreeNode::Branch(branch) => branch.hash(),
        }
    }

    pub fn from_child_suffixes(
        key_hash_fragment: &[u8],
        child_key_hash_suffixes: &BTreeSet<Vec<u8>>,
    ) -> Result<Self, OctRadSparseTreeError> {
        if child_key_hash_suffixes.is_empty() {
            return Err(OctRadSparseTreeError::EmptyKeySet);
        }

        // Stack to store pending nodes to process
        #[derive(Debug)]
        struct StackItem {
            key_hash_fragment: Vec<u8>,
            suffixes: BTreeSet<Vec<u8>>,
            parent_byte: Option<u8>,
        }

        let mut stack = vec![StackItem {
            key_hash_fragment: key_hash_fragment.to_vec(),
            suffixes: child_key_hash_suffixes.clone(),
            parent_byte: None,
        }];
        let mut node_stack = Vec::new();

        while let Some(item) = stack.pop() {
            // Group key hash suffixes by their first byte
            let mut children_map: std::collections::BTreeMap<u8, BTreeSet<Vec<u8>>> =
                std::collections::BTreeMap::new();
            for suffix in item.suffixes {
                if suffix.is_empty() {
                    return Err(OctRadSparseTreeError::EmptyKeySuffix);
                }
                let first_byte = suffix[0];
                let rest = suffix[1..].to_vec();
                children_map.entry(first_byte).or_default().insert(rest);
            }

            // If there's only one child and it has no remaining suffix, create a leaf node
            if children_map.len() == 1 {
                let (byte, suffixes) = children_map.iter().next().unwrap();
                if suffixes.len() == 1 && suffixes.iter().next().unwrap().is_empty() {
                    let mut full_key = item.key_hash_fragment.clone();
                    full_key.push(*byte);
                    node_stack.push((
                        item.parent_byte,
                        OctRadSparseTreeNode::Leaf(OctRadSparseTreeNodeLeaf::new(full_key)),
                    ));
                    continue;
                }
            }

            // Create child nodes
            let mut child_hashes = BTreeMap::new();

            // Push children to stack in reverse order to maintain correct order
            for (byte, suffixes) in children_map.into_iter().rev() {
                let mut child_key = item.key_hash_fragment.clone();
                child_key.push(byte);
                stack.push(StackItem {
                    key_hash_fragment: child_key,
                    suffixes,
                    parent_byte: Some(byte),
                });
            }

            // Wait for all children to be processed
            while node_stack
                .last()
                .map_or(false, |(b, _)| *b == item.parent_byte)
            {
                let (byte, child) = node_stack.pop().unwrap();
                child_hashes.insert(byte.unwrap(), child.hash());
            }

            // Create branch node
            let branch = OctRadSparseTreeNode::Branch(OctRadSparseTreeNodeBranch::new(
                item.key_hash_fragment,
                child_hashes,
            ));
            node_stack.push((item.parent_byte, branch));
        }

        // The root node should be the only remaining node
        Ok(node_stack.pop().unwrap().1)
    }
}
