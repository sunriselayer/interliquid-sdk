use std::collections::BTreeMap;

use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::{Nibble, NibblePatriciaTrieError, NibblePatriciaTrieNode};

pub trait NibblePatriciaTrieDb {
    fn get(&self, key: &[Nibble]) -> Option<Vec<u8>>;
    fn set(&mut self, key: &[Nibble], value: &[u8]);
    fn del(&mut self, key: &[Nibble]);

    fn iter<'a>(
        &'a self,
        key_prefix: Vec<Nibble>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<Nibble>, Vec<u8>), NibblePatriciaTrieError>> + 'a>;
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
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

    fn iter<'a>(
        &'a self,
        key_prefix: Vec<Nibble>,
    ) -> Box<dyn Iterator<Item = Result<(Vec<Nibble>, Vec<u8>), NibblePatriciaTrieError>> + 'a>
    {
        if key_prefix.len() == 0 {
            Box::new(self.db.iter().map(|(k, v)| Ok((k.clone(), v.clone()))))
        } else if key_prefix.iter().all(|&b| b == Nibble::from(Nibble::MAX)) {
            Box::new(
                self.db
                    .range(key_prefix..)
                    .map(|(k, v)| Ok((k.clone(), v.clone()))),
            )
        } else {
            let mut key_prefix_next = key_prefix.clone();
            *key_prefix_next.last_mut().unwrap() =
                Nibble::from(key_prefix_next.last().unwrap().as_u8() + 1); // len > 0

            Box::new(
                self.db
                    .range(key_prefix..key_prefix_next)
                    .map(|(k, v)| Ok((k.clone(), v.clone()))),
            )
        }
    }
}

pub fn get_node_from_db<Db: NibblePatriciaTrieDb>(
    key: &[Nibble],
    node_db: &Db,
) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError> {
    let node_bytes = node_db.get(key).ok_or(NibblePatriciaTrieError::NotFound)?;
    let node = NibblePatriciaTrieNode::try_from_slice(&node_bytes)?;
    Ok(node)
}

pub fn get_child_node_fragment_and_hash_from_db<Db: NibblePatriciaTrieDb>(
    key: &[Nibble],
    index: Nibble,
    hash_db: &Db,
) -> Result<(Vec<Nibble>, [u8; 32]), NibblePatriciaTrieError> {
    let child_key_prefix = key.iter().copied().chain([index]).collect::<Vec<_>>();
    let (child_node_key, child_node_hash) = hash_db
        .iter(child_key_prefix)
        .next()
        .ok_or(NibblePatriciaTrieError::NotFound)??;

    let child_node_fragment = child_node_key[key.len()..].to_vec();

    Ok((
        child_node_fragment,
        child_node_hash
            .try_into()
            .map_err(|_| NibblePatriciaTrieError::InvalidHash)?,
    ))
}
