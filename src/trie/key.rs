use std::collections::BTreeMap;

use anyhow::anyhow;

use super::{
    Nibble, NibblePatriciaTrieError, NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch,
    NibblePatriciaTrieNodeLeaf,
};

pub fn search_near_leaf_parent_key(
    leaf_key: &[Nibble],
    get_node: impl Fn(
        &[Nibble],
    ) -> Result<Option<NibblePatriciaTrieNodeBranch>, NibblePatriciaTrieError>,
) -> Result<Vec<Nibble>, NibblePatriciaTrieError> {
    let key_len = leaf_key.len();
    for i in (0..key_len).rev() {
        let key_path = &leaf_key[..i];

        let node = get_node(key_path)?;

        if node.is_some() {
            return Ok(key_path.to_owned());
        }
    }

    Err(NibblePatriciaTrieError::Other(anyhow!(
        "Root node with empty key fragment must catch the key path in the loop"
    )))
}

pub fn leaf_parent_key(
    leaf_key: &[Nibble],
    get_node: impl Fn(&[Nibble]) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError>,
) -> Result<(Vec<Nibble>, NibblePatriciaTrieNodeLeaf), NibblePatriciaTrieError> {
    let leaf_node = get_node(leaf_key)?;

    println!("leaf_node: {:?}", leaf_node);

    if let NibblePatriciaTrieNode::Leaf(leaf) = leaf_node {
        let parent_key = leaf_key[..leaf_key.len() - leaf.key_fragment.len()].to_vec();
        Ok((parent_key, leaf))
    } else {
        Err(NibblePatriciaTrieError::InvalidNode)
    }
}

pub fn nibble_prefix_range<'a, T: Clone>(
    map: &'a BTreeMap<Vec<Nibble>, T>,
    key_prefix: Vec<Nibble>,
) -> Box<dyn Iterator<Item = (Vec<Nibble>, T)> + 'a> {
    if key_prefix.len() == 0 {
        Box::new(map.iter().map(|(k, v)| (k.clone(), v.clone())))
    } else if key_prefix.iter().all(|&b| b == Nibble::from(Nibble::MAX)) {
        Box::new(map.range(key_prefix..).map(|(k, v)| (k.clone(), v.clone())))
    } else {
        let mut key_prefix_next = key_prefix.clone();
        *key_prefix_next.last_mut().unwrap() =
            Nibble::from(key_prefix_next.last().unwrap().as_u8() + 1); // len > 0

        Box::new(
            map.range(key_prefix..key_prefix_next)
                .map(|(k, v)| (k.clone(), v.clone())),
        )
    }
}
