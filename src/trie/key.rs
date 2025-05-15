use anyhow::anyhow;
use borsh::BorshDeserialize;

use super::{
    Nibble, NibblePatriciaTrieDb, NibblePatriciaTrieError, NibblePatriciaTrieNode,
    NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
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

        if let Some(branch) = node {
            let index = Nibble::from(leaf_key[i]);
            let child_key_fragment = branch.child_key_fragments.get(&index);
            // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
            if child_key_fragment.is_none() || !child_key_fragment.unwrap().eq(&leaf_key[i..]) {
                return Ok(leaf_key[..i].to_vec());
            }
        }
    }

    Err(NibblePatriciaTrieError::Other(anyhow!(
        "Root node with empty key fragment must catch the key path in the loop"
    )))
}

pub fn leaf_parent_key<Db: NibblePatriciaTrieDb>(
    leaf_key: &[Nibble],
    node_db: &Db,
) -> Result<(Vec<Nibble>, Option<NibblePatriciaTrieNodeLeaf>), NibblePatriciaTrieError> {
    let leaf_node_bytes = node_db.get(leaf_key);

    match leaf_node_bytes {
        Some(leaf_node_bytes) => {
            let leaf_node = NibblePatriciaTrieNode::try_from_slice(&leaf_node_bytes)?;
            if let NibblePatriciaTrieNode::Leaf(leaf) = leaf_node {
                let parent_key = leaf_key[..leaf_key.len() - leaf.key_fragment.len()].to_vec();
                Ok((parent_key, Some(leaf)))
            } else {
                Err(NibblePatriciaTrieError::InvalidNode)
            }
        }
        None => {
            let parent_key = search_near_leaf_parent_key(leaf_key, |key| {
                let node_bytes = node_db.get(key);
                if let Some(node_bytes) = node_bytes {
                    let node = NibblePatriciaTrieNode::try_from_slice(&node_bytes)?;
                    if let NibblePatriciaTrieNode::Branch(branch) = node {
                        Ok(Some(branch))
                    } else {
                        Err(NibblePatriciaTrieError::InvalidNode)
                    }
                } else {
                    Ok(None)
                }
            })?;
            Ok((parent_key, None))
        }
    }
}
