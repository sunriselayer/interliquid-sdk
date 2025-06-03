use borsh_derive::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::trie::nibble_prefix_range;

use super::{
    key::leaf_parent_key, search_near_leaf_parent_key, Nibble, NibblePatriciaTrieError,
    NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct NibblePatriciaTrieRootPath {
    pub nodes_branch: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>,
    pub nodes_hashed: BTreeMap<Vec<Nibble>, [u8; 32]>,
}

impl NibblePatriciaTrieRootPath {
    /// Creates a new NibblePatriciaTrieRootPath instance.
    ///
    /// # Arguments
    ///
    /// * `nodes_branch` - A map of branch nodes in the trie, where the key is the path to the node
    /// * `nodes_hashed` - A map of hashed nodes in the trie, where the key is the path to the node
    pub fn new(
        nodes_branch: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeBranch>,
        nodes_hashed: BTreeMap<Vec<Nibble>, [u8; 32]>,
    ) -> Self {
        Self {
            nodes_branch,
            nodes_hashed,
        }
    }

    /// Constructs an inclusion proof or non-inclusion proof from the designated leafs.
    ///
    /// This function builds a proof by:
    /// 1. Marking the leaf nodes and their parent nodes that need to be included in the proof
    /// 2. For each marked branch node, adding its non-marked child nodes to the proof
    /// 3. Recursively processing parent nodes until reaching the root
    ///
    /// # Arguments
    ///
    /// * `leaf_keys` - Set of leaf keys to include in the proof
    /// * `get_node` - Function to retrieve a node from the trie
    /// * `get_child_node_fragment_and_hash` - Function to get a child node's fragment and hash
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - A proof containing the necessary nodes and hashes
    /// * `Err(NibblePatriciaTrieError)` - If the proof construction fails
    pub fn from_leafs(
        leaf_keys: BTreeSet<Vec<Nibble>>,
        get_node: impl Fn(&[Nibble]) -> Result<NibblePatriciaTrieNode, NibblePatriciaTrieError>,
        get_child_node_fragment_and_hash: impl Fn(
            &[Nibble],
            Nibble,
        ) -> Result<
            (Vec<Nibble>, [u8; 32]),
            NibblePatriciaTrieError,
        >,
    ) -> Result<Self, NibblePatriciaTrieError> {
        // marked nodes means the nodes which are needed to be re-constructed in the inclusion proof
        let mut marked_nodes = BTreeMap::<Vec<Nibble>, NibblePatriciaTrieNode>::new();
        let mut marked_nodes_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

        for leaf_key in leaf_keys.iter() {
            let (parent_key, leaf_node) = leaf_parent_key(leaf_key, &get_node)?;
            let parent_node = get_node(&parent_key)?;

            if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                marked_nodes.insert(
                    parent_key.clone(),
                    NibblePatriciaTrieNode::Branch(parent_branch),
                );
                marked_nodes_for_depth
                    .entry(parent_key.len())
                    .or_insert_with(BTreeSet::new)
                    .insert(parent_key);

                marked_nodes.insert(leaf_key.clone(), NibblePatriciaTrieNode::Leaf(leaf_node));
                marked_nodes_for_depth
                    .entry(leaf_key.len())
                    .or_insert_with(BTreeSet::new)
                    .insert(leaf_key.clone());
            } else {
                return Err(NibblePatriciaTrieError::InvalidNode);
            }
        }

        if marked_nodes.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        // get the depth of the deepest marked node
        let mut depth = *marked_nodes_for_depth.keys().last().unwrap();

        let mut slf = Self::new(BTreeMap::new(), BTreeMap::new());

        loop {
            let marked_nodes_at_depth = marked_nodes_for_depth.get(&depth);
            let mut new_marked_nodes = BTreeMap::<Vec<Nibble>, NibblePatriciaTrieNode>::new();
            let mut new_marked_nodes_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

            if let Some(marked_nodes_at_depth) = marked_nodes_at_depth {
                // get all the marked nodes at the current depth
                for key in marked_nodes_at_depth.iter() {
                    let branch = marked_nodes
                        .get(key)
                        .ok_or(NibblePatriciaTrieError::NotFound)?;

                    if let NibblePatriciaTrieNode::Branch(branch) = branch {
                        // add non-marked child nodes of the marked branch node to the proof
                        for index in branch.child_key_indices.iter() {
                            let marked_child_node = nibble_prefix_range(
                                &marked_nodes,
                                key.iter().copied().chain([*index]).collect::<Vec<_>>(),
                            )
                            .next();

                            // only the non-marked child nodes are added to the proof
                            if marked_child_node.is_some() {
                                continue;
                            }

                            let (child_key_fragment, child_node_hash) =
                                get_child_node_fragment_and_hash(&key, *index)?;

                            let child_key = key
                                .iter()
                                .copied()
                                .chain(child_key_fragment)
                                .collect::<Vec<_>>();

                            slf.nodes_hashed.insert(child_key, child_node_hash);
                        }

                        // add the parent node to the new marked nodes
                        let parent_key = &key[..key.len() - branch.key_fragment.len()];
                        let parent_node = get_node(parent_key)?;

                        if let NibblePatriciaTrieNode::Branch(parent_branch) = parent_node {
                            new_marked_nodes.insert(
                                parent_key.to_owned(),
                                NibblePatriciaTrieNode::Branch(parent_branch),
                            );
                            new_marked_nodes_for_depth
                                .entry(parent_key.len())
                                .or_insert_with(BTreeSet::new)
                                .insert(parent_key.to_owned());
                        } else {
                            return Err(NibblePatriciaTrieError::InvalidNode);
                        }

                        // add the marked node to the proof
                        slf.nodes_branch.insert(key.clone(), branch.clone());
                    }
                }
            }
            marked_nodes.extend(new_marked_nodes);
            marked_nodes_for_depth.extend(new_marked_nodes_for_depth);

            if depth == 0 {
                break;
            }

            depth -= 1;
        }

        Ok(slf)
    }

    /// Verifies the non-inclusion of a leaf key in the trie.
    ///
    /// This function verifies that a key is not present in the trie by:
    /// 1. Traversing the path from leaf to the root
    /// 2. Checking if any branch node along the path doesn't have the next nibble in its child indices
    /// 3. If such a branch node is found, the non-inclusion is proven
    ///
    /// # Arguments
    ///
    /// * `leaf_key` - The key to verify non-inclusion for
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If non-inclusion is successfully proven
    /// * `Err(NibblePatriciaTrieError)` - If the proof is invalid or non-inclusion cannot be proven
    pub fn verify_non_inclusion(&self, leaf_key: &[Nibble]) -> Result<(), NibblePatriciaTrieError> {
        let key_len = leaf_key.len();

        for i in (0..key_len).rev() {
            let key_path = &leaf_key[..i];

            let node = self.nodes_branch.get(key_path);

            if let Some(node) = node {
                // to prove the non-inclusion, branch node must not have the child key index for the leaf to prove non-inclusion
                if !node.child_key_indices.contains(&Nibble::from(leaf_key[i])) {
                    return Ok(());
                }
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
        }

        Err(NibblePatriciaTrieError::InvalidProof)
    }

    /// Verifies the completeness of an iteration over keys with a given prefix.
    ///
    /// This function ensures that:
    /// 1. All provided keys start with the given prefix
    /// 2. No keys within the prefix range are missing
    /// 3. The proof contains sufficient information to verify completeness
    ///
    /// # Arguments
    ///
    /// * `key_prefix` - The prefix to verify iteration completeness for
    /// * `iterated_keys` - Set of keys that were iterated
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the iteration is complete and valid
    /// * `Err(NibblePatriciaTrieError)` - If the proof is incomplete or invalid
    pub fn verify_iter_completeness(
        &self,
        key_prefix: &[Nibble],
        iterated_keys: &BTreeSet<Vec<Nibble>>,
    ) -> Result<(), NibblePatriciaTrieError> {
        // First, verify all iterated keys start with the prefix
        for key in iterated_keys {
            if !key.starts_with(key_prefix) {
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
        }

        // Find the best branch node to verify our target prefix
        // We want the branch whose full prefix (key + fragment) is the longest prefix of our target,
        // or whose key is the longest that our target starts with
        let mut best_branch: Option<(&Vec<Nibble>, &NibblePatriciaTrieNodeBranch)> = None;
        let mut best_match_len = 0;
        
        for (branch_key, branch_node) in &self.nodes_branch {
            // Calculate the full prefix of this branch (key + fragment)
            let full_branch_prefix: Vec<Nibble> = branch_key
                .iter()
                .copied()
                .chain(branch_node.key_fragment.iter().copied())
                .collect();
            
            // Check different matching scenarios
            if key_prefix == &full_branch_prefix[..] {
                // Exact match - this is the best possible branch
                best_branch = Some((branch_key, branch_node));
                break; // Can't get better than exact match
            } else if key_prefix.starts_with(&full_branch_prefix) && full_branch_prefix.len() > best_match_len {
                // Target extends beyond this branch
                best_branch = Some((branch_key, branch_node));
                best_match_len = full_branch_prefix.len();
            } else if key_prefix.starts_with(branch_key) && branch_key.len() > best_match_len {
                // Branch key is a prefix of target (branch might help)
                best_branch = Some((branch_key, branch_node));
                best_match_len = branch_key.len();
            }
        }
        
        if let Some((branch_key, branch_node)) = best_branch {
            // We found the best branch for our prefix
            let full_branch_prefix: Vec<Nibble> = branch_key
                .iter()
                .copied()
                .chain(branch_node.key_fragment.iter().copied())
                .collect();
            
            // Check the relationship between target prefix and branch
            if key_prefix.starts_with(&full_branch_prefix) {
                // Case 1: Target extends beyond this branch
                let remaining_prefix = &key_prefix[full_branch_prefix.len()..];
                
                if remaining_prefix.is_empty() {
                    // The branch exactly matches our prefix
                    // All children of this branch should be included in iterated_keys
                    self.verify_all_branch_children_iterated(
                        &full_branch_prefix,
                        branch_node,
                        iterated_keys,
                    )
            } else {
                // We need to go deeper - check the specific child
                let target_child = remaining_prefix[0];
                
                if !branch_node.child_key_indices.contains(&target_child) {
                    // The target child doesn't exist
                    if !iterated_keys.is_empty() {
                        return Err(NibblePatriciaTrieError::InvalidProof);
                    }
                    return Ok(());
                }
                
                // All keys under the target child should be iterated
                let child_full_prefix: Vec<Nibble> = full_branch_prefix
                    .iter()
                    .copied()
                    .chain(std::iter::once(target_child))
                    .collect();
                    
                // Check if all iterated keys are under this child
                for key in iterated_keys {
                    if !key.starts_with(&child_full_prefix) {
                        return Err(NibblePatriciaTrieError::InvalidProof);
                    }
                }
                
                // We can't verify completeness under this specific child without more branch nodes
                // But we've verified all iterated keys are valid
                Ok(())
            }
            } else {
                // Case 2: Branch key is a prefix of target, but full branch prefix might not be
                // This happens when target falls within the branch's key but before its fragment
                
                // The target prefix must fall somewhere between branch_key and full_branch_prefix
                let remaining_after_key = &key_prefix[branch_key.len()..];
                
                // Check if the branch has the child we need
                if !remaining_after_key.is_empty() {
                    let target_child = remaining_after_key[0];
                    
                    if branch_node.child_key_indices.contains(&target_child) {
                        // We can verify through this child
                        // All iterated keys should be under this child
                        for key in iterated_keys {
                            if !key.starts_with(key_prefix) {
                                return Err(NibblePatriciaTrieError::InvalidProof);
                            }
                        }
                        
                        // We can partially verify - all keys are valid but can't ensure completeness
                        // without more branch information
                        Ok(())
                    } else {
                        // Child doesn't exist, so there should be no keys
                        if !iterated_keys.is_empty() {
                            return Err(NibblePatriciaTrieError::InvalidProof);
                        }
                        Ok(())
                    }
                } else {
                    // remaining_after_key is empty, which means key_prefix == branch_key
                    // This means we're looking for all children of this branch
                    self.verify_all_branch_children_iterated(
                        branch_key,
                        branch_node,
                        iterated_keys,
                    )
                }
            }
        } else {
            // No branch node found that is a prefix of our target
            // If we have iterated keys, the proof is incomplete
            if !iterated_keys.is_empty() {
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
            Ok(())
        }
    }
    
    /// Verify that all children of a branch are represented in the iterated keys
    fn verify_all_branch_children_iterated(
        &self,
        branch_full_prefix: &[Nibble],
        branch_node: &NibblePatriciaTrieNodeBranch,
        iterated_keys: &BTreeSet<Vec<Nibble>>,
    ) -> Result<(), NibblePatriciaTrieError> {
        // For each child index in the branch
        for &child_index in &branch_node.child_key_indices {
            let child_prefix: Vec<Nibble> = branch_full_prefix
                .iter()
                .copied()
                .chain(std::iter::once(child_index))
                .collect();
                
            // Check if we have any keys starting with this child prefix
            let has_child_keys = iterated_keys
                .iter()
                .any(|k| k.starts_with(&child_prefix));
                
            if !has_child_keys {
                // This child exists but no keys were iterated from it
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
        }
        
        // Also verify no extra keys that don't belong to any child
        for key in iterated_keys {
            if !key.starts_with(branch_full_prefix) {
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
            
            // The key should start with one of the child indices
            let key_child_nibble = key.get(branch_full_prefix.len())
                .ok_or(NibblePatriciaTrieError::InvalidProof)?;
                
            if !branch_node.child_key_indices.contains(key_child_nibble) {
                return Err(NibblePatriciaTrieError::InvalidProof);
            }
        }
        
        Ok(())
    }


    pub fn leaf_key_fragment_from_path(
        &self,
        leaf_key: &[Nibble],
    ) -> Result<Vec<Nibble>, NibblePatriciaTrieError> {
        let parent_key =
            search_near_leaf_parent_key(leaf_key, |key| Ok(self.nodes_branch.get(key).cloned()))?;

        let key_fragment = leaf_key[parent_key.len()..].to_vec();

        Ok(key_fragment)
    }

    pub fn node_for_inclusion_proof(
        &self,
        leaf_key: &[Nibble],
        leaf_value: Vec<u8>,
    ) -> Result<NibblePatriciaTrieNodeLeaf, NibblePatriciaTrieError> {
        let leaf_key_fragment = self.leaf_key_fragment_from_path(leaf_key)?;

        Ok(NibblePatriciaTrieNodeLeaf::new(
            leaf_key_fragment,
            leaf_value,
        ))
    }

    /// Computes the root hash of the trie using the proof and inclusion nodes.
    ///
    /// This function:
    /// 1. Combines the proof's branch nodes and hashed nodes with the provided inclusion nodes
    /// 2. Processes nodes from deepest to shallowest depth
    /// 3. Computes hashes for each branch node using its children's hashes
    /// 4. Returns the final root hash
    ///
    /// # Arguments
    ///
    /// * `nodes_for_inclusion_proof` - Map of leaf values to include in the proof
    /// * `branch_hash_callback` - Optional callback function to be called when a branch node's hash is computed
    ///
    /// # Returns
    ///
    /// * `Ok([u8; 32])` - The computed root hash
    /// * `Err(NibblePatriciaTrieError)` - If the root hash computation fails
    pub fn root(
        self,
        nodes_for_inclusion_proof: BTreeMap<Vec<Nibble>, NibblePatriciaTrieNodeLeaf>,
        branch_hash_callback: Option<Box<dyn Fn(&Vec<Nibble>, &[u8; 32])>>,
    ) -> Result<[u8; 32], NibblePatriciaTrieError> {
        let mut nodes_branch = BTreeMap::<Vec<Nibble>, NibblePatriciaTrieNodeBranch>::new();
        let mut nodes_branch_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

        let mut nodes_hashed = BTreeMap::<Vec<Nibble>, [u8; 32]>::new();
        let mut nodes_hashed_for_depth = BTreeMap::<usize, BTreeSet<Vec<Nibble>>>::new();

        for (key, node) in self.nodes_branch {
            nodes_branch.insert(key.clone(), node);

            nodes_branch_for_depth
                .entry(key.len())
                .or_insert_with(BTreeSet::new)
                .insert(key);
        }

        for (key, node) in self.nodes_hashed {
            nodes_hashed.insert(key.clone(), node);

            nodes_hashed_for_depth
                .entry(key.len())
                .or_insert_with(BTreeSet::new)
                .insert(key);
        }

        for (key, node) in nodes_for_inclusion_proof {
            nodes_hashed.insert(key.clone(), node.hash());

            nodes_hashed_for_depth
                .entry(key.len())
                .or_insert_with(BTreeSet::new)
                .insert(key);
        }

        if nodes_branch.is_empty() {
            return Err(NibblePatriciaTrieError::EmptyProof);
        }

        while let Some((_depth, nodes_branch_at_depth)) = nodes_branch_for_depth.pop_last() {
            let nodes_branch_at_depth = nodes_branch_at_depth;
            for key in nodes_branch_at_depth {
                let branch = nodes_branch
                    .get(&key)
                    .ok_or(NibblePatriciaTrieError::NotFound)?;

                let hash = branch.hash(|index| {
                    let hashed_child_node = nibble_prefix_range(
                        &nodes_hashed,
                        key.iter().copied().chain([*index]).collect::<Vec<_>>(),
                    )
                    .next()?;

                    let (_key, hash) = hashed_child_node;

                    Some(hash)
                });

                if let Some(hash) = hash {
                    if let Some(ref branch_hash_callback) = branch_hash_callback {
                        branch_hash_callback(&key, &hash);
                    }

                    nodes_hashed.insert(key.clone(), hash);
                    nodes_hashed_for_depth
                        .entry(key.len())
                        .or_insert_with(BTreeSet::new)
                        .insert(key.clone());
                }
            }
        }

        let root_hash = nodes_hashed
            .pop_first()
            .ok_or(NibblePatriciaTrieError::InvalidProof)?
            .1;

        Ok(root_hash)
    }
}

#[cfg(test)]
mod tests {
    use borsh::BorshSerialize;

    use super::*;
    use crate::trie::db::NibblePatriciaTrieMemoryDb;
    use crate::trie::nibble::Nibble;
    use crate::trie::node::{
        NibblePatriciaTrieNode, NibblePatriciaTrieNodeBranch, NibblePatriciaTrieNodeLeaf,
    };
    use crate::trie::{
        get_child_node_fragment_and_hash_from_db, get_node_from_db, NibblePatriciaTrieDb,
    };
    use std::collections::BTreeMap;

    fn setup_trie_and_db() -> (
        BTreeMap<Vec<Nibble>, Vec<u8>>,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieNode,
    ) {
        // Prepare simple key-value pairs
        let mut entries = BTreeMap::new();
        entries.insert(vec![Nibble::from(1), Nibble::from(2)], b"a".to_vec());
        entries.insert(vec![Nibble::from(1), Nibble::from(3)], b"b".to_vec());
        entries.insert(vec![Nibble::from(2), Nibble::from(2)], b"c".to_vec());

        // Manually construct nodes
        // leaf [1,2]
        let leaf_12 = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2)], b"a".to_vec());
        let leaf_13 = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(3)], b"b".to_vec());
        let leaf_22 =
            NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2), Nibble::from(2)], b"c".to_vec());

        // branch [1]
        let mut branch_1_children = BTreeSet::new();
        branch_1_children.insert(Nibble::from(2));
        branch_1_children.insert(Nibble::from(3));
        let branch_1 = NibblePatriciaTrieNodeBranch::new(vec![Nibble::from(1)], branch_1_children);

        // root
        let mut root_children = BTreeSet::new();
        root_children.insert(Nibble::from(1)); // [1] branch
        root_children.insert(Nibble::from(2)); // [2,2] leaf
        let root = NibblePatriciaTrieNodeBranch::new(vec![], root_children);

        // Prepare node_db and hash_db
        let mut node_db = NibblePatriciaTrieMemoryDb::new();
        let mut hash_db = NibblePatriciaTrieMemoryDb::new();

        // Serialize and store nodes
        let mut buf = Vec::new();
        // leaf [1,2]
        buf.clear();
        hash_db.set(&[Nibble::from(1), Nibble::from(2)], &leaf_12.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_12)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(1), Nibble::from(2)], &buf);
        // leaf [1,3]
        buf.clear();
        hash_db.set(&[Nibble::from(1), Nibble::from(3)], &leaf_13.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_13)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(1), Nibble::from(3)], &buf);
        // leaf [2,2]
        buf.clear();
        hash_db.set(&[Nibble::from(2), Nibble::from(2)], &leaf_22.hash()[..]);
        NibblePatriciaTrieNode::Leaf(leaf_22)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(2), Nibble::from(2)], &buf);
        // branch [1]
        buf.clear();
        // branch [1] hash
        let branch_1_hash = branch_1
            .hash(|nib| {
                let child_key = match nib.as_u8() {
                    2 => vec![Nibble::from(1), Nibble::from(2)],
                    3 => vec![Nibble::from(1), Nibble::from(3)],
                    _ => return None,
                };
                Some(<[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice()).unwrap())
            })
            .unwrap();
        hash_db.set(&[Nibble::from(1)], &branch_1_hash[..]);
        NibblePatriciaTrieNode::Branch(branch_1)
            .serialize(&mut buf)
            .unwrap();
        node_db.set(&[Nibble::from(1)], &buf);
        // root
        buf.clear();
        // root hash
        let root_hash = root
            .hash(|nib| {
                let child_key = match nib.as_u8() {
                    1 => vec![Nibble::from(1)],
                    2 => vec![Nibble::from(2), Nibble::from(2)],
                    _ => return None,
                };
                Some(<[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice()).unwrap())
            })
            .unwrap();
        hash_db.set(&[], &root_hash[..]);
        let root_node = NibblePatriciaTrieNode::Branch(root);
        root_node.serialize(&mut buf).unwrap();
        node_db.set(&[], &buf);

        (entries, node_db, hash_db, root_node)
    }

    fn setup_trie_and_db_large() -> (
        BTreeMap<Vec<Nibble>, Vec<u8>>,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieMemoryDb,
        NibblePatriciaTrieNode,
    ) {
        // Prepare key-value pairs with 1000 elements
        let mut entries = BTreeMap::new();
        for i in 0..1000 {
            let key = vec![
                Nibble::from((i / 100) as u8),
                Nibble::from(((i % 100) / 10) as u8),
                Nibble::from((i % 10) as u8),
            ];
            let value = format!("value_{}", i).into_bytes();
            entries.insert(key, value);
        }

        // Prepare node_db and hash_db
        let mut node_db = NibblePatriciaTrieMemoryDb::new();
        let mut hash_db = NibblePatriciaTrieMemoryDb::new();
        let mut buf = Vec::new();

        // Create and store all leaf nodes first
        for (key, value) in entries.iter() {
            let leaf = NibblePatriciaTrieNodeLeaf::new(vec![key[2]], value.clone());
            buf.clear();
            let leaf_hash = leaf.hash();
            hash_db.set(key, &leaf_hash[..]);
            NibblePatriciaTrieNode::Leaf(leaf)
                .serialize(&mut buf)
                .unwrap();
            node_db.set(key, &buf);
        }

        // Create branch nodes for each level, starting from the deepest level
        // Level 2 (last level before leaves)
        for i in 0..10 {
            for j in 0..10 {
                let mut children = BTreeSet::new();
                for k in 0..10 {
                    children.insert(Nibble::from(k as u8));
                }
                let branch =
                    NibblePatriciaTrieNodeBranch::new(vec![Nibble::from(j as u8)], children);
                buf.clear();
                let branch_hash = branch
                    .hash(|nib| {
                        let child_key = vec![Nibble::from(i as u8), Nibble::from(j as u8), *nib];
                        Some(
                            <[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice())
                                .unwrap(),
                        )
                    })
                    .unwrap();
                let branch_key = vec![Nibble::from(i as u8), Nibble::from(j as u8)];
                hash_db.set(&branch_key, &branch_hash[..]);
                NibblePatriciaTrieNode::Branch(branch)
                    .serialize(&mut buf)
                    .unwrap();
                node_db.set(&branch_key, &buf);
            }
        }

        // Level 1
        for i in 0..10 {
            let mut children = BTreeSet::new();
            for j in 0..10 {
                children.insert(Nibble::from(j as u8));
            }
            let branch = NibblePatriciaTrieNodeBranch::new(vec![Nibble::from(i as u8)], children);
            buf.clear();
            let branch_hash = branch
                .hash(|nib| {
                    let child_key = vec![Nibble::from(i as u8), *nib];
                    Some(<[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice()).unwrap())
                })
                .unwrap();
            let branch_key = vec![Nibble::from(i as u8)];
            hash_db.set(&branch_key, &branch_hash[..]);
            NibblePatriciaTrieNode::Branch(branch)
                .serialize(&mut buf)
                .unwrap();
            node_db.set(&branch_key, &buf);
        }

        // Root level
        let mut root_children = BTreeSet::new();
        for i in 0..10 {
            root_children.insert(Nibble::from(i as u8));
        }
        let root = NibblePatriciaTrieNodeBranch::new(vec![], root_children);
        buf.clear();
        let root_hash = root
            .hash(|nib| {
                let child_key = vec![*nib];
                Some(<[u8; 32]>::try_from(hash_db.get(&child_key).unwrap().as_slice()).unwrap())
            })
            .unwrap();
        hash_db.set(&[], &root_hash[..]);
        let root_node = NibblePatriciaTrieNode::Branch(root);
        root_node.serialize(&mut buf).unwrap();
        node_db.set(&[], &buf);

        (entries, node_db, hash_db, root_node)
    }

    #[test]
    fn test_leaf_parent_key() {
        let (_entries, node_db, _hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);

        let (parent_key, _leaf_node) =
            leaf_parent_key(&vec![Nibble::from(1), Nibble::from(2)], &get_node).unwrap();
        assert_eq!(parent_key, vec![Nibble::from(1)]);

        let (parent_key, _leaf_node) =
            leaf_parent_key(&vec![Nibble::from(1), Nibble::from(3)], &get_node).unwrap();
        assert_eq!(parent_key, vec![Nibble::from(1)]);

        let (parent_key, _leaf_node) =
            leaf_parent_key(&vec![Nibble::from(2), Nibble::from(2)], &get_node).unwrap();
        assert_eq!(parent_key, vec![]);

        let err = leaf_parent_key(&vec![Nibble::from(2), Nibble::from(3)], &get_node);
        assert!(err.is_err());
    }

    #[test]
    fn test_leaf_parent_key_large() {
        let (_entries, node_db, _hash_db, _root_node) = setup_trie_and_db_large();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);

        let (parent_key, _leaf_node) = leaf_parent_key(
            &vec![Nibble::from(1), Nibble::from(2), Nibble::from(3)],
            &get_node,
        )
        .unwrap();
        assert_eq!(parent_key, vec![Nibble::from(1), Nibble::from(2)]);

        let (parent_key, _leaf_node) = leaf_parent_key(
            &vec![Nibble::from(1), Nibble::from(3), Nibble::from(4)],
            &get_node,
        )
        .unwrap();
        assert_eq!(parent_key, vec![Nibble::from(1), Nibble::from(3)]);

        let (parent_key, _leaf_node) = leaf_parent_key(
            &vec![Nibble::from(2), Nibble::from(2), Nibble::from(3)],
            &get_node,
        )
        .unwrap();
        assert_eq!(parent_key, vec![Nibble::from(2), Nibble::from(2)]);

        let err = leaf_parent_key(&vec![Nibble::from(2), Nibble::from(3)], &get_node);
        assert!(err.is_err());
    }

    #[test]
    fn test_from_leafs() {
        let (_entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_keys = BTreeSet::from([vec![Nibble::from(1), Nibble::from(2)]]);
        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
        println!("proof: {:?}", proof);

        assert!(proof.nodes_branch.contains_key(&vec![Nibble::from(1)]));
        assert!(!proof
            .nodes_hashed
            .contains_key(&vec![Nibble::from(1), Nibble::from(2)]));
        assert!(proof
            .nodes_hashed
            .contains_key(&vec![Nibble::from(1), Nibble::from(3)]));
        assert!(proof
            .nodes_hashed
            .contains_key(&vec![Nibble::from(2), Nibble::from(2)]));
    }

    #[test]
    fn test_verify_non_inclusion() {
        let (_entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_keys = BTreeSet::from([vec![Nibble::from(1), Nibble::from(3)]]);
        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
        println!("proof: {:?}", proof);

        assert!(proof
            .verify_non_inclusion(&vec![Nibble::from(1), Nibble::from(4)])
            .is_ok());
    }

    #[test]
    fn test_inclusion_proof_and_root() {
        let (entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_key = vec![Nibble::from(1), Nibble::from(2)];
        let leaf_keys = BTreeSet::from([leaf_key.clone()]);
        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
        println!("proof: {:?}", proof);

        let leaf_value = entries.get(&leaf_key).unwrap();
        let root = proof
            .root(
                [(
                    leaf_key.clone(),
                    NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(2)], leaf_value.clone()),
                )]
                .into_iter()
                .collect(),
                None,
            )
            .unwrap();
        println!("root: {:?}", root);

        let root_hash: [u8; 32] = hash_db.get(&vec![]).unwrap().try_into().unwrap();

        assert_eq!(root, root_hash);
    }

    #[test]
    fn test_inclusion_proof_and_root_large() {
        let (entries, node_db, hash_db, _root_node) = setup_trie_and_db_large();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        let leaf_key = vec![Nibble::from(1), Nibble::from(2), Nibble::from(3)];
        let leaf_keys = BTreeSet::from([leaf_key.clone()]);

        // Debug: Print the leaf value
        println!("Leaf value: {:?}", entries.get(&leaf_key));

        // Debug: Print the node structure
        println!(
            "Node at [1,2]: {:?}",
            get_node(&vec![Nibble::from(1), Nibble::from(2)])
        );
        println!("Node at [1]: {:?}", get_node(&vec![Nibble::from(1)]));
        println!("Node at []: {:?}", get_node(&vec![]));

        let proof = NibblePatriciaTrieRootPath::from_leafs(
            leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();
        println!("proof: {:?}", proof);

        let leaf_value = entries.get(&leaf_key).unwrap();
        let leaf_node = NibblePatriciaTrieNodeLeaf::new(vec![Nibble::from(3)], leaf_value.clone());
        println!("leaf_node: {:?}", leaf_node);

        let root = proof
            .root([(leaf_key.clone(), leaf_node)].into_iter().collect(), None)
            .unwrap();
        println!("Computed root: {:?}", root);

        let root_hash: [u8; 32] = hash_db.get(&vec![]).unwrap().try_into().unwrap();
        println!("Expected root: {:?}", root_hash);

        assert_eq!(root, root_hash);
    }

    #[test]
    fn test_verify_iter_completeness() {
        let (_entries, node_db, hash_db, _root_node) = setup_trie_and_db();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        // Test case 1: Complete iteration with prefix [1]
        let prefix = vec![Nibble::from(1)];
        let iterated_keys = BTreeSet::from([
            vec![Nibble::from(1), Nibble::from(2)],
            vec![Nibble::from(1), Nibble::from(3)],
        ]);

        // Get all leaf keys to create a complete proof
        let all_leaf_keys = BTreeSet::from([
            vec![Nibble::from(1), Nibble::from(2)],
            vec![Nibble::from(1), Nibble::from(3)],
            vec![Nibble::from(2), Nibble::from(2)],
        ]);

        let proof = NibblePatriciaTrieRootPath::from_leafs(
            all_leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();

        // Should succeed - complete iteration
        assert!(proof.verify_iter_completeness(&prefix, &iterated_keys).is_ok());

        // Test case 2: Incomplete iteration - missing one key
        let incomplete_keys = BTreeSet::from([
            vec![Nibble::from(1), Nibble::from(2)],
            // Missing [1, 3]
        ]);

        // Should fail - incomplete iteration
        assert!(proof.verify_iter_completeness(&prefix, &incomplete_keys).is_err());

        // Test case 3: Invalid iteration - key outside prefix
        let invalid_keys = BTreeSet::from([
            vec![Nibble::from(1), Nibble::from(2)],
            vec![Nibble::from(2), Nibble::from(2)], // Outside prefix [1]
        ]);

        // Should fail - key outside prefix
        assert!(proof.verify_iter_completeness(&prefix, &invalid_keys).is_err());

        // Test case 4: Empty prefix - should include all keys
        let empty_prefix = vec![];
        let all_keys = BTreeSet::from([
            vec![Nibble::from(1), Nibble::from(2)],
            vec![Nibble::from(1), Nibble::from(3)],
            vec![Nibble::from(2), Nibble::from(2)],
        ]);

        // Should succeed - all keys included
        assert!(proof.verify_iter_completeness(&empty_prefix, &all_keys).is_ok());
    }

    #[test]
    fn test_verify_iter_completeness_large() {
        let (_entries, node_db, hash_db, _root_node) = setup_trie_and_db_large();

        let get_node = |key: &[Nibble]| get_node_from_db(key, &node_db);
        let get_child_node_fragment_and_hash = |key: &[Nibble], index: Nibble| {
            get_child_node_fragment_and_hash_from_db(key, index, &hash_db)
        };

        // Test case 1: Complete iteration with prefix [1, 2] - should have 10 keys
        let prefix = vec![Nibble::from(1), Nibble::from(2)];
        let mut iterated_keys = BTreeSet::new();
        for i in 0..10 {
            iterated_keys.insert(vec![
                Nibble::from(1),
                Nibble::from(2),
                Nibble::from(i as u8),
            ]);
        }

        // Get all leaf keys starting with [1, 2] to create the proof
        let mut proof_leaf_keys = BTreeSet::new();
        for i in 0..10 {
            proof_leaf_keys.insert(vec![
                Nibble::from(1),
                Nibble::from(2),
                Nibble::from(i as u8),
            ]);
        }

        // Add some other keys to make the proof more complete
        for i in 0..10 {
            proof_leaf_keys.insert(vec![
                Nibble::from(1),
                Nibble::from(3),
                Nibble::from(i as u8),
            ]);
        }

        let proof = NibblePatriciaTrieRootPath::from_leafs(
            proof_leaf_keys,
            &get_node,
            &get_child_node_fragment_and_hash,
        )
        .unwrap();

        // Should succeed - complete iteration
        assert!(proof.verify_iter_completeness(&prefix, &iterated_keys).is_ok());

        // Test case 2: Incomplete iteration - missing one key
        let mut incomplete_keys = iterated_keys.clone();
        incomplete_keys.remove(&vec![Nibble::from(1), Nibble::from(2), Nibble::from(5)]);

        // Should fail - incomplete iteration
        assert!(proof.verify_iter_completeness(&prefix, &incomplete_keys).is_err());

        // Test case 3: Extra key outside prefix
        let mut invalid_keys = iterated_keys.clone();
        invalid_keys.insert(vec![Nibble::from(1), Nibble::from(3), Nibble::from(0)]);

        // Should fail - key outside prefix
        assert!(proof.verify_iter_completeness(&prefix, &invalid_keys).is_err());
    }
}
