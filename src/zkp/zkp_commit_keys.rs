use std::collections::BTreeMap;

use anyhow::anyhow;
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use crate::sha2::{Digest, Sha256};

use crate::{merkle::OctRadPatriciaTriePath, state::CompressedDiffs, types::InterLiquidSdkError};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputCommitKeys {
    pub keys_patricia_trie_root_prev: [u8; 32],
    pub keys_patricia_trie_root_next: [u8; 32],
    pub accum_diffs_hash_final: [u8; 32],
}

impl PublicInputCommitKeys {
    pub fn new(
        keys_patricia_trie_root_prev: [u8; 32],
        keys_patricia_trie_root_next: [u8; 32],
        accum_diffs_hash_final: [u8; 32],
    ) -> Self {
        Self {
            keys_patricia_trie_root_prev,
            keys_patricia_trie_root_next,
            accum_diffs_hash_final,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessCommitKeys {
    pub keys_patricia_trie_root_prev: [u8; 32],
    pub accum_diffs_final: CompressedDiffs,
    pub keys_commit_path: OctRadPatriciaTriePath,
}

impl WitnessCommitKeys {
    pub fn new(
        keys_patricia_trie_root_prev: [u8; 32],
        accum_diffs_final: CompressedDiffs,
        keys_commit_path: OctRadPatriciaTriePath,
    ) -> Self {
        Self {
            keys_patricia_trie_root_prev,
            accum_diffs_final,
            keys_commit_path,
        }
    }
}

pub fn circuit_commit_keys(
    witness: WitnessCommitKeys,
) -> Result<PublicInputCommitKeys, InterLiquidSdkError> {
    let mut accum_diffs_bytes_final = Vec::new();
    witness
        .accum_diffs_final
        .serialize(&mut accum_diffs_bytes_final)?;

    let mut keys_commit_path_for_prev = witness.keys_commit_path.clone();
    keys_commit_path_for_prev.assign_node_hashes(
        witness
            .accum_diffs_final
            .diffs
            .iter()
            .filter_map(|(key, diff)| {
                if let Some(before) = &diff.before {
                    let hash: [u8; 32] = Sha256::digest(before).into();
                    Some((key, hash))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<&Vec<u8>, [u8; 32]>>()
            .iter()
            .map(|(k, h)| (*k, h)),
    );
    let keys_patricia_trie_root_prev = keys_commit_path_for_prev.root();

    if keys_patricia_trie_root_prev != witness.keys_patricia_trie_root_prev {
        return Err(InterLiquidSdkError::Other(anyhow!(
            "Inconsistent keys_patricia_trie_root_prev and keys_commit_path"
        )));
    }

    let mut keys_commit_path_for_next = witness.keys_commit_path;
    keys_commit_path_for_next.assign_node_hashes(
        witness
            .accum_diffs_final
            .diffs
            .iter()
            .filter_map(|(key, diff)| {
                if let Some(after) = &diff.after {
                    let hash: [u8; 32] = Sha256::digest(after).into();
                    Some((key, hash))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<&Vec<u8>, [u8; 32]>>()
            .iter()
            .map(|(k, h)| (*k, h)),
    );
    let keys_patricia_trie_root_next = keys_commit_path_for_next.root();

    let accum_diffs_hash_final = Sha256::digest(&accum_diffs_bytes_final).into();

    let input = PublicInputCommitKeys::new(
        witness.keys_patricia_trie_root_prev,
        keys_patricia_trie_root_next,
        accum_diffs_hash_final,
    );

    Ok(input)
}
