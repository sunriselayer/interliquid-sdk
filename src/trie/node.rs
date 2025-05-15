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

    pub fn hash(&self, child_hash: impl Fn(&Nibble) -> Option<[u8; 32]>) -> Option<[u8; 32]> {
        let mut hasher = Sha256::new();
        hasher.update(Nibble::as_slice(&self.key_fragment));
        for (index, _child_key_fragment) in self.child_key_fragments.iter() {
            hasher.update([index.as_u8()]);
            hasher.update(child_hash(index)?);
        }
        Some(hasher.finalize().into())
    }

    pub fn build_trie(
        entries: BTreeMap<Vec<Nibble>, Vec<u8>>,
    ) -> (Vec<Nibble>, BTreeMap<Vec<Nibble>, NibblePatriciaTrieNode>) {
        // Each item in the queue is (prefix, key range in sorted_keys)
        struct QueueItem {
            prefix: Vec<Nibble>,
            start: usize,
            end: usize,
        }

        let sorted_keys: Vec<_> = entries.keys().cloned().collect();
        let mut node_map: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNode> = BTreeMap::new();
        let mut queue: Vec<QueueItem> = vec![];

        // Start from the root (empty prefix, all keys)
        queue.push(QueueItem {
            prefix: vec![],
            start: 0,
            end: sorted_keys.len(),
        });

        let mut root_key = vec![];

        while let Some(QueueItem { prefix, start, end }) = queue.pop() {
            // If only one key in this range, create a leaf node
            if end - start == 1 {
                let key = &sorted_keys[start];
                let value = entries.get(key).unwrap().clone();
                let key_fragment = key[prefix.len()..].to_vec();
                let leaf = NibblePatriciaTrieNodeLeaf::new(key_fragment, value);
                node_map.insert(prefix.clone(), NibblePatriciaTrieNode::Leaf(leaf));
                if prefix.is_empty() {
                    root_key = prefix.clone();
                }
                continue;
            }

            // Find the common prefix for this key range
            let mut common_prefix = prefix.clone();
            let first_key = &sorted_keys[start];
            let last_key = &sorted_keys[end - 1];
            let mut i = prefix.len();
            while i < first_key.len() && i < last_key.len() && first_key[i] == last_key[i] {
                common_prefix.push(first_key[i].clone());
                i += 1;
            }

            // Group by the next nibble after the common prefix
            let mut child_key_fragments: BTreeMap<Nibble, Vec<Nibble>> = BTreeMap::new();
            let mut child_ranges: BTreeMap<Nibble, (usize, usize)> = BTreeMap::new();
            let mut prev_nibble: Option<Nibble> = None;
            let mut group_start = start;
            for idx in start..end {
                let key = &sorted_keys[idx];
                if key.len() <= common_prefix.len() {
                    // This should not happen in a valid trie
                    continue;
                }
                let nib = key[common_prefix.len()].clone();
                if prev_nibble.as_ref() != Some(&nib) {
                    // If this is not the first group, set the end of the previous group
                    if let Some(prev) = prev_nibble {
                        child_ranges.insert(prev, (group_start, idx));
                    }
                    // New group starts here
                    child_key_fragments
                        .insert(nib.clone(), key[..common_prefix.len() + 1].to_vec());
                    group_start = idx;
                    prev_nibble = Some(nib);
                }
            }
            // Set the end for the last group
            if let Some(prev) = prev_nibble {
                child_ranges.insert(prev, (group_start, end));
            }

            // Create branch node
            let key_fragment = common_prefix[prefix.len()..].to_vec();
            let branch = NibblePatriciaTrieNodeBranch::new(key_fragment, child_key_fragments);
            node_map.insert(prefix.clone(), NibblePatriciaTrieNode::Branch(branch));
            if prefix.is_empty() {
                root_key = prefix.clone();
            }

            // Push children to the queue
            for (nib, (child_start, child_end)) in child_ranges {
                let mut child_prefix = common_prefix.clone();
                child_prefix.push(nib);
                queue.push(QueueItem {
                    prefix: child_prefix,
                    start: child_start,
                    end: child_end,
                });
            }
        }

        (root_key, node_map)
    }
}
