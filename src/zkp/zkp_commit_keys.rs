use std::collections::{BTreeMap, BTreeSet};

use crate::{
    sha2::{Digest, Sha256},
    trie::{nibbles_from_bytes, Nibble, NibblePatriciaTrieError, NibblePatriciaTrieRootPath},
};
use anyhow::anyhow;
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{state::AccumulatedLogs, types::InterLiquidSdkError};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputCommitKeys {
    pub keys_root_prev: [u8; 32],
    pub keys_root_next: [u8; 32],
    pub state_for_access_hash: [u8; 32],
    pub accum_logs_hash_final: [u8; 32],
}

impl PublicInputCommitKeys {
    pub fn new(
        keys_root_prev: [u8; 32],
        keys_root_next: [u8; 32],
        state_for_access_hash: [u8; 32],
        accum_logs_hash_final: [u8; 32],
    ) -> Self {
        Self {
            keys_root_prev,
            keys_root_next,
            state_for_access_hash,
            accum_logs_hash_final,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessCommitKeys {
    pub keys_root_prev: [u8; 32],
    pub accum_logs_final: AccumulatedLogs,
    pub state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
    pub keys_commit_path: NibblePatriciaTrieRootPath,
}

impl WitnessCommitKeys {
    pub fn new(
        keys_root_prev: [u8; 32],
        accum_logs_final: AccumulatedLogs,
        state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
        keys_commit_path: NibblePatriciaTrieRootPath,
    ) -> Self {
        Self {
            keys_root_prev,
            accum_logs_final,
            state_for_access,
            keys_commit_path,
        }
    }
}

pub fn circuit_commit_keys(
    witness: WitnessCommitKeys,
) -> Result<PublicInputCommitKeys, InterLiquidSdkError> {
    let mut accum_logs_bytes_final = Vec::new();
    witness
        .accum_logs_final
        .serialize(&mut accum_logs_bytes_final)?;

    // Verify completeness for all iteration logs
    for (key_prefix, keys) in witness.accum_logs_final.iter() {
        let key_prefix_nibbles = nibbles_from_bytes(key_prefix);
        let iterated_keys_nibbles: BTreeSet<Vec<Nibble>> = keys
            .iter()
            .map(|k| nibbles_from_bytes(k))
            .collect();
        
        witness
            .keys_commit_path
            .verify_iter_completeness(&key_prefix_nibbles, &iterated_keys_nibbles)?;
    }

    let nodes_for_inclusion_proof_prev = witness
        .state_for_access
        .iter()
        .map(|(k, _v)| (k, vec![]))
        .chain(
            witness
                .accum_logs_final
                .diff()
                .iter()
                .filter_map(|(key, diff)| {
                    if diff.before.is_some() {
                        Some((key, vec![]))
                    } else {
                        None
                    }
                }),
        )
        .map(|(k, v)| {
            let leaf_key = nibbles_from_bytes(&k);
            let leaf_node = witness
                .keys_commit_path
                .node_for_inclusion_proof(&leaf_key, v)?;
            Ok((leaf_key, leaf_node))
        })
        .collect::<Result<_, NibblePatriciaTrieError>>()?;
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
        .state_for_access
        .iter()
        .map(|(k, _v)| (k, vec![]))
        .chain(
            witness
                .accum_logs_final
                .diff()
                .iter()
                .filter_map(|(key, diff)| {
                    if diff.after.is_some() {
                        Some((key, vec![]))
                    } else {
                        None
                    }
                }),
        )
        .map(|(k, v)| {
            let leaf_key = nibbles_from_bytes(&k);
            let leaf_node = witness
                .keys_commit_path
                .node_for_inclusion_proof(&leaf_key, v)?;
            Ok((leaf_key, leaf_node))
        })
        .collect::<Result<_, NibblePatriciaTrieError>>()?;
    let keys_root_next = witness
        .keys_commit_path
        .root(nodes_for_inclusion_proof_next, None)?;

    let mut state_for_access_bytes = Vec::new();
    witness
        .state_for_access
        .serialize(&mut state_for_access_bytes)?;
    let state_for_access_hash = Sha256::digest(&state_for_access_bytes).into();

    let accum_logs_hash_final = Sha256::digest(&accum_logs_bytes_final).into();

    let input = PublicInputCommitKeys::new(
        witness.keys_root_prev,
        keys_root_next,
        state_for_access_hash,
        accum_logs_hash_final,
    );

    Ok(input)
}
