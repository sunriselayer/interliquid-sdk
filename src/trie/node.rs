use crate::sha2::{Digest, Sha256};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;

use super::{Nibble, NibblePatriciaTrieError};

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

    pub fn build_branch_nodes(
        entries: BTreeMap<Vec<Nibble>, Vec<u8>>,
    ) -> Result<BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>, NibblePatriciaTrieError> {
        struct StackItem<'a> {
            key_fragments: Vec<Nibble>,
            remaining_key_values: BTreeMap<&'a [Nibble], &'a Vec<u8>>,
        }

        let mut stack = vec![StackItem {
            key_fragments: vec![],
            remaining_key_values: entries
                .iter()
                .map(|(key, value)| (&key[..], value))
                .collect(),
        }];
        let mut result = BTreeMap::new();

        while let Some(stack_item) = stack.pop() {
            let items = (0..Nibble::MAX)
                .into_iter()
                .filter_map(|i| {
                    let index = Nibble::from(i);

                    let remaining_key_values = stack_item
                        .remaining_key_values
                        .iter()
                        .filter(|(&key, _)| key.starts_with(&[index]))
                        .map(|(&key, &value)| (&key[1..], value))
                        .collect::<BTreeMap<_, _>>();

                    let len = remaining_key_values.len();

                    if len == 0 {
                        return None;
                    }

                    let item = if len == 1 {
                        StackItem {
                            key_fragments: stack_item
                                .key_fragments
                                .iter()
                                .cloned()
                                .chain([index])
                                .collect(),
                            remaining_key_values: remaining_key_values,
                        }
                    } else {
                        StackItem {
                            key_fragments: vec![index],
                            remaining_key_values: remaining_key_values,
                        }
                    };

                    Some((index, item))
                })
                .collect::<BTreeMap<_, _>>();

            let node = NibblePatriciaTrieNodeBranch::new(stack_item.key_fragments, BTreeMap::new());
        }

        Ok(result)
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

        let node_map = NibblePatriciaTrieNodeBranch::build_branch_nodes(entries.clone()).unwrap();

        assert_eq!(node_map.len(), 2);
        assert_eq!(
            node_map
                .get(&vec![Nibble::from(1)])
                .unwrap()
                .child_key_fragments
                .len(),
            2
        );
        assert_eq!(
            node_map
                .get(&vec![Nibble::from(2)])
                .unwrap()
                .child_key_fragments
                .len(),
            1
        );
    }
}
