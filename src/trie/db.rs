use super::Nibble;

pub trait NibblePatriciaTrieDb {
    fn get(&self, key: &[Nibble]) -> Option<Vec<u8>>;
    fn set(&mut self, key: &[Nibble], value: &[u8]);
    fn del(&mut self, key: &[Nibble]);
}
