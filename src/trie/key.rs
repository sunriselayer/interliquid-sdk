use anyhow::anyhow;

use super::{
    Nibble, NibblePatriciaTrieError, NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch,
    NibblePatriciaTrieNodeLeaf,
};

pub fn search_near_leaf_parent_key(
    leaf_key: &[Nibble],
    get_node: impl Fn(&[Nibble]) -> Result<NibblePatriciaTrieNodeBranch, NibblePatriciaTrieError>,
) -> Result<Vec<Nibble>, NibblePatriciaTrieError> {
    let key_len = leaf_key.len();
    for i in (0..key_len).rev() {
        let key_path = &leaf_key[..i];

        let node = get_node(key_path)?;

        let index = Nibble::from(leaf_key[i]);
        let child_key_fragment = node.child_key_fragments.get(&index);
        // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
        if child_key_fragment.is_none() || !child_key_fragment.unwrap().eq(&leaf_key[i..]) {
            return Ok(leaf_key[..i].to_vec());
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

    if let NibblePatriciaTrieNode::Leaf(leaf) = leaf_node {
        let parent_key = leaf_key[..leaf_key.len() - leaf.key_fragment.len()].to_vec();
        Ok((parent_key, leaf))
    } else {
        Err(NibblePatriciaTrieError::InvalidNode)
    }
}
