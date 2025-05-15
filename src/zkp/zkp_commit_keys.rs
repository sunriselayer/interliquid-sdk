use crate::{
    sha2::{Digest, Sha256},
    trie::NibblePatriciaTrieRootPath,
};
use anyhow::anyhow;
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{state::CompressedDiffs, types::InterLiquidSdkError};

use super::trie::node_for_inclusion_proof;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputCommitKeys {
    pub keys_root_prev: [u8; 32],
    pub keys_root_next: [u8; 32],
    pub accum_diffs_hash_final: [u8; 32],
}

impl PublicInputCommitKeys {
    pub fn new(
        keys_root_prev: [u8; 32],
        keys_root_next: [u8; 32],
        accum_diffs_hash_final: [u8; 32],
    ) -> Self {
        Self {
            keys_root_prev,
            keys_root_next,
            accum_diffs_hash_final,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessCommitKeys {
    pub keys_root_prev: [u8; 32],
    pub accum_diffs_final: CompressedDiffs,
    pub keys_commit_path: NibblePatriciaTrieRootPath,
}

impl WitnessCommitKeys {
    pub fn new(
        keys_root_prev: [u8; 32],
        accum_diffs_final: CompressedDiffs,
        keys_commit_path: NibblePatriciaTrieRootPath,
    ) -> Self {
        Self {
            keys_root_prev,
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

    let nodes_for_inclusion_proof_prev = witness
        .accum_diffs_final
        .diffs
        .iter()
        .filter_map(|(key, diff)| {
            if diff.before.is_some() {
                Some((key, vec![]))
            } else {
                None
            }
        })
        .map(|(k, v)| node_for_inclusion_proof(&witness.keys_commit_path, &k, v))
        .collect::<Result<_, _>>()?;
    let keys_root_prev = witness
        .keys_commit_path
        .clone()
        .root(nodes_for_inclusion_proof_prev, None)?;

    if keys_root_prev != witness.keys_root_prev {
        return Err(InterLiquidSdkError::Other(anyhow!(
            "Inconsistent keys_root_prev and keys_commit_path"
        )));
    }

    let nodes_for_inclusion_proof_next = witness
        .accum_diffs_final
        .diffs
        .iter()
        .filter_map(|(key, diff)| {
            if diff.after.is_some() {
                Some((key, vec![]))
            } else {
                None
            }
        })
        .map(|(k, v)| node_for_inclusion_proof(&witness.keys_commit_path, &k, v))
        .collect::<Result<_, _>>()?;
    let keys_root_next = witness
        .keys_commit_path
        .root(nodes_for_inclusion_proof_next, None)?;

    let accum_diffs_hash_final = Sha256::digest(&accum_diffs_bytes_final).into();

    let input = PublicInputCommitKeys::new(
        witness.keys_root_prev,
        keys_root_next,
        accum_diffs_hash_final,
    );

    Ok(input)
}
