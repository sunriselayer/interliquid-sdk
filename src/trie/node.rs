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
    /// key is the first nibble of the child key fragment which works as the index of the child node
    /// value is the key fragment of the child node (including first nibble)
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
        let child_hashes = self
            .child_key_fragments
            .keys()
            .map(|index| {
                let child_hash = child_hash(index);
                (index, child_hash)
            })
            .collect::<BTreeMap<_, _>>();

        if child_hashes.iter().all(|(_, hash)| hash.is_none()) {
            return None;
        }

        let mut hasher = Sha256::new();
        hasher.update(Nibble::as_slice(&self.key_fragment));
        for (index, child_hash) in child_hashes.iter() {
            if let Some(child_hash) = child_hash {
                hasher.update([index.as_u8()]);
                hasher.update(child_hash);
            }
        }
        Some(hasher.finalize().into())
    }

    pub fn build_trie(
        entries: BTreeMap<Vec<Nibble>, Vec<u8>>,
    ) -> (Vec<Nibble>, BTreeMap<Vec<Nibble>, NibblePatriciaTrieNode>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::nibbles_from_bytes;

    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_build_trie_simple() {
        let mut entries = BTreeMap::new();
        entries.insert(vec![Nibble::from(1), Nibble::from(2)], b"a".to_vec());
        entries.insert(vec![Nibble::from(1), Nibble::from(3)], b"b".to_vec());
        entries.insert(vec![Nibble::from(2), Nibble::from(2)], b"c".to_vec());

        // Manually construct nodes
        // leaf [1,2]
        let leaf_12 = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2)], b"a".to_vec());
        let leaf_13 = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(3)], b"b".to_vec());
        let leaf_22 =
            NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2), Nibble::from(2)], b"c".to_vec());

        // branch [1]
        let mut branch_1_children = BTreeMap::new();
        branch_1_children.insert(Nibble::from(2), vec![Nibble::from(2)]);
        branch_1_children.insert(Nibble::from(3), vec![Nibble::from(3)]);
        let branch_1 = NibblePatriciaTrieNodeBranch::new(vec![Nibble::from(1)], branch_1_children);

        // root
        let mut root_children = BTreeMap::new();
        root_children.insert(Nibble::from(1), vec![Nibble::from(1)]); // [1] branch
        root_children.insert(Nibble::from(2), vec![Nibble::from(2), Nibble::from(2)]); // [2,2] leaf
        let root = NibblePatriciaTrieNodeBranch::new(vec![], root_children);

        let (root_key, node_map) = NibblePatriciaTrieNodeBranch::build_trie(entries.clone());
    }
}
