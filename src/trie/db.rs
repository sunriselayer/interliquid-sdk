use std::collections::BTreeMap;

use super::Nibble;

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
