use std::collections::BTreeMap;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::OctRadPatriciaTrieError;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaTriePath(BTreeMap<Vec<u8>, [u8; 32]>);

impl OctRadPatriciaTriePath {
    pub fn new(path: BTreeMap<Vec<u8>, [u8; 32]>) -> Self {
        Self(path)
    }

    pub fn assign_node_hashes<'a>(
        &mut self,
        node_hashes: impl Iterator<Item = (&'a Vec<u8>, &'a [u8; 32])>,
    ) {
        for (key, hash) in node_hashes {
            self.0.insert(key.clone(), *hash);
        }
    }

    pub fn verify_root(&self, root: &[u8; 32]) -> Result<(), OctRadPatriciaTrieError> {
        // Calculate the root hash from the path
        let calculated_root = self.root();

        // Verify that the calculated root matches the given root
        if !calculated_root.eq(root) {
            return Err(OctRadPatriciaTrieError::InvalidProof(anyhow::anyhow!(
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
            let branch = crate::merkle::OctRadPatriciaTrieNodeBranch::new(
                item.key_prefix.clone(),
                child_hashes,
            );
            node_stack.push((item.parent_byte, branch.hash()));
        }

        // The root node should be the only remaining node
        node_stack.pop().unwrap().1
    }
}
