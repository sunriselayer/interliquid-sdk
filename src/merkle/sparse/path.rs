use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::merkle::OctRadSparseTreeNodeBranch;

use super::OctRadSparseTreeError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadSparseTreePath(BTreeMap<Vec<u8>, [u8; 32]>);

impl OctRadSparseTreePath {
    pub fn new(path: BTreeMap<Vec<u8>, [u8; 32]>) -> Self {
        Self(path)
    }

    /// If there is a duplicated path, `remainder_node_hashes` is prioritized.
    pub fn root(&self, remainder_node_hashes: &BTreeMap<Vec<u8>, [u8; 32]>) -> [u8; 32] {
        // Create a new map that combines self.0 and remainder_node_hashes
        // If there are duplicate keys, remainder_node_hashes takes precedence
        let mut combined_hashes = self.0.clone();
        for (key, hash) in remainder_node_hashes {
            combined_hashes.insert(key.clone(), *hash);
        }

        // Stack to store pending nodes to process
        #[derive(Debug)]
        struct StackItem {
            key_prefix: Vec<u8>,
            hashes: BTreeMap<Vec<u8>, [u8; 32]>,
            parent_byte: Option<u8>,
        }

        let mut stack = vec![StackItem {
            key_prefix: Vec::new(),
            hashes: combined_hashes,
            parent_byte: None,
        }];
        let mut node_stack = Vec::new();

        while let Some(item) = stack.pop() {
            // Group hashes by their next byte
            let mut next_byte_groups: BTreeMap<u8, BTreeMap<Vec<u8>, [u8; 32]>> = BTreeMap::new();
            for (key, hash) in item.hashes {
                if key.is_empty() {
                    // If key is empty, this is a leaf node
                    node_stack.push((item.parent_byte, hash));
                    continue;
                }
                let next_byte = key[0];
                let rest_key = key[1..].to_vec();
                next_byte_groups
                    .entry(next_byte)
                    .or_default()
                    .insert(rest_key, hash);
            }

            // Push children to stack in reverse order to maintain correct order
            for (byte, hashes) in next_byte_groups.into_iter().rev() {
                let mut child_key_prefix = item.key_prefix.clone();
                child_key_prefix.push(byte);
                stack.push(StackItem {
                    key_prefix: child_key_prefix,
                    hashes,
                    parent_byte: Some(byte),
                });
            }

            // Wait for all children to be processed
            let mut child_hashes = BTreeMap::new();
            while node_stack
                .last()
                .map_or(false, |(b, _)| *b == item.parent_byte)
            {
                let (_, child_hash) = node_stack.pop().unwrap();
                if let Some(byte) = item.key_prefix.last() {
                    child_hashes.insert(*byte, child_hash);
                }
            }

            // Create branch node
            let branch = OctRadSparseTreeNodeBranch::new(
                *item.key_prefix.last().unwrap_or(&0),
                child_hashes,
            );
            node_stack.push((item.parent_byte, branch.hash()));
        }

        // The root node should be the only remaining node
        node_stack.pop().unwrap().1
    }

    pub fn prove_inclusion(
        &self,
        node_hashes_to_prove: &BTreeMap<Vec<u8>, [u8; 32]>,
        root: &[u8; 32],
    ) -> Result<(), OctRadSparseTreeError> {
        // For each node hash to prove, verify that it exists in the path
        for (key, _hash) in node_hashes_to_prove {
            if !self.0.contains_key(key) {
                return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                    "node hash not found in path for key {:?}",
                    key
                )));
            }
        }

        // Calculate the root hash from the path
        let calculated_root = self.root(node_hashes_to_prove);

        // Verify that the calculated root matches the given root
        if !calculated_root.eq(root) {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "calculated root does not match given root"
            )));
        }

        Ok(())
    }
}
