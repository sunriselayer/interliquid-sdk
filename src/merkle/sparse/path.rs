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

    /// Assigns the node hashes to the path.
    /// It can be used for inclusion proof.
    pub fn assign_node_hashes<'a>(
        &mut self,
        node_hashes: impl Iterator<Item = (&'a Vec<u8>, &'a [u8; 32])>,
    ) {
        for (key, hash) in node_hashes {
            self.0.insert(key.clone(), *hash);
        }
    }

    pub fn verify_non_inclusion<'a>(
        &self,
        key: &[u8; 32],
        dead_end_node_depth: u8,
        dead_end_node_child_hashes: &BTreeMap<u8, [u8; 32]>,
    ) -> Result<(), OctRadSparseTreeError> {
        if dead_end_node_depth == 31 {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "the node at depth 31 must be a leaf node"
            )));
        }
        if dead_end_node_depth > 31 {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "node depth is greater than 31"
            )));
        }

        let dead_end_node_key = key[..dead_end_node_depth as usize].to_vec();
        let non_inclusion_index = key[dead_end_node_depth as usize];

        if dead_end_node_child_hashes.contains_key(&non_inclusion_index) {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "the keys is included"
            )));
        }

        let node_hash = OctRadSparseTreeNodeBranch::hash_from_child_hashes_iter(
            *dead_end_node_key.last().unwrap(),
            dead_end_node_child_hashes,
        );

        if !self
            .0
            .get(&dead_end_node_key)
            .map_or(false, |h| h.eq(&node_hash))
        {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "the dead end node is not included"
            )));
        }

        Ok(())
    }

    pub fn verify_root(&self, root: &[u8; 32]) -> Result<(), OctRadSparseTreeError> {
        // Calculate the root hash from the path
        let calculated_root = self.root();

        // Verify that the calculated root matches the given root
        if !calculated_root.eq(root) {
            return Err(OctRadSparseTreeError::InvalidProof(anyhow::anyhow!(
                "calculated root does not match given root"
            )));
        }

        Ok(())
    }

    pub fn root(&self) -> [u8; 32] {
        // Stack to store pending nodes to process
        #[derive(Debug)]
        struct StackItem {
            key_prefix: Vec<u8>,
            hashes: BTreeMap<Vec<u8>, [u8; 32]>,
            parent_byte: Option<u8>,
        }

        let mut stack = vec![StackItem {
            key_prefix: Vec::new(),
            hashes: self.0.clone(),
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
}
