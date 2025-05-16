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
