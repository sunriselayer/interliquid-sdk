use crate::trie::{
    nibbles_from_bytes, search_near_leaf_parent_key, Nibble, NibblePatriciaTrieError,
    NibblePatriciaTrieNodeLeaf, NibblePatriciaTrieRootPath,
};

pub fn leaf_key_fragment_from_path(
    path: &NibblePatriciaTrieRootPath,
    leaf_key: &[Nibble],
) -> Result<Vec<Nibble>, NibblePatriciaTrieError> {
    let parent_key =
        search_near_leaf_parent_key(leaf_key, |key| Ok(path.nodes_branch.get(key).cloned()))?;

    let key_fragment = leaf_key[parent_key.len()..].to_vec();

    Ok(key_fragment)
}

pub fn node_for_inclusion_proof(
    path: &NibblePatriciaTrieRootPath,
    leaf_key: &[u8],
    leaf_value: Vec<u8>,
) -> Result<(Vec<Nibble>, NibblePatriciaTrieNodeLeaf), NibblePatriciaTrieError> {
    let leaf_key = nibbles_from_bytes(leaf_key);
    let leaf_key_fragment = leaf_key_fragment_from_path(path, &leaf_key)?;

    Ok((
        leaf_key,
        NibblePatriciaTrieNodeLeaf::new(leaf_key_fragment, leaf_value),
    ))
}
