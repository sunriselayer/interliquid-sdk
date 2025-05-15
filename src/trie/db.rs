use std::collections::BTreeMap;

use borsh::BorshDeserialize;

use super::{Nibble, NibblePatriciaTrieError, NibblePatriciaTrieNode};

pub trait NibblePatriciaTrieDb {
    fn get(&self, key: &[Nibble]) -> Option<Vec<u8>>;
    fn set(&mut self, key: &[Nibble], value: &[u8]);
    fn del(&mut self, key: &[Nibble]);
}

pub struct NibblePatriciaTrieMemoryDb {
    db: BTreeMap<Vec<Nibble>, Vec<u8>>,
}

impl NibblePatriciaTrieMemoryDb {
    pub fn new() -> Self {
        Self {
            db: BTreeMap::new(),
        }
    }
}

impl NibblePatriciaTrieDb for NibblePatriciaTrieMemoryDb {
    fn get(&self, key: &[Nibble]) -> Option<Vec<u8>> {
        self.db.get(key).cloned()
    }

    fn set(&mut self, key: &[Nibble], value: &[u8]) {
        self.db.insert(key.to_vec(), value.to_vec());
    }

    fn del(&mut self, key: &[Nibble]) {
        self.db.remove(key);
    }
}

pub(super) fn get_node<Db: NibblePatriciaTrieDb>(
    key: &[Nibble],
    node_db: &Db,
) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError> {
    let node_bytes = node_db.get(key).ok_or(NibblePatriciaTrieError::NotFound)?;
    let node = NibblePatriciaTrieNode::try_from_slice(&node_bytes)?;
    Ok(node)
}

pub(super) fn get_node_hash<Db: NibblePatriciaTrieDb>(
    key: &[Nibble],
    hash_db: &Db,
) -> Result<[u8; 32], NibblePatriciaTrieError> {
    let hash_bytes = hash_db.get(key).ok_or(NibblePatriciaTrieError::NotFound)?;
    let hash: [u8; 32] = hash_bytes
        .try_into()
        .map_err(|_| NibblePatriciaTrieError::InvalidHash)?;
    Ok(hash)
}
