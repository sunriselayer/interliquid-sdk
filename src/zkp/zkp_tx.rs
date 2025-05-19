use std::collections::BTreeMap;

use crate::{
    sha2::{Digest, Sha256},
    state::AccumulatedLogs,
    types::Environment,
};
use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{App, SdkContext, Tx},
    state::{RelatedState, TransactionalStateManager},
    types::InterLiquidSdkError,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct PublicInputTx {
    pub tx_hash: [u8; 32],
    pub env_hash: [u8; 32],
    pub accum_logs_hash_prev: [u8; 32],
    pub accum_logs_hash_next: [u8; 32],
    pub entire_root: [u8; 32],
}

impl PublicInputTx {
    pub fn new(
        tx_hash: [u8; 32],
        env_hash: [u8; 32],
        accum_logs_hash_prev: [u8; 32],
        accum_logs_hash_next: [u8; 32],
        entire_root: [u8; 32],
    ) -> Self {
        Self {
            tx_hash,
            env_hash,
            accum_logs_hash_prev,
            accum_logs_hash_next,
            entire_root,
        }
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WitnessTx {
    pub tx: Vec<u8>,
    pub env: Environment,
    pub state_root: [u8; 32],
    pub keys_root: [u8; 32],
    pub state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
    pub accum_logs_prev: AccumulatedLogs,
}

impl WitnessTx {
    pub fn new(
        tx: Vec<u8>,
        env: Environment,
        state_root: [u8; 32],
        keys_root: [u8; 32],
        state_for_access: BTreeMap<Vec<u8>, Vec<u8>>,
        accum_logs_prev: AccumulatedLogs,
    ) -> Self {
        Self {
            tx,
            env,
            state_root,
            keys_root,
            state_for_access,
            accum_logs_prev,
        }
    }
}

pub fn circuit_tx<TX: Tx>(
    witness: WitnessTx,
    app: &App<TX>,
) -> Result<PublicInputTx, InterLiquidSdkError> {
    let mut accum_logs_bytes_prev = Vec::new();
    witness
        .accum_logs_prev
        .serialize(&mut accum_logs_bytes_prev)?;
    let accum_logs_hash_prev = Sha256::digest(&accum_logs_bytes_prev).into();

    let related_state = RelatedState::new(witness.state_for_access);
    let mut transactional =
        TransactionalStateManager::from_accum_logs_prev(&related_state, witness.accum_logs_prev);

    let mut env_bytes = Vec::new();
    witness.env.serialize(&mut env_bytes)?;
    let env_hash = Sha256::digest(&env_bytes).into();

    let mut ctx = SdkContext::new(witness.env, &mut transactional);

    app.execute_tx(&mut ctx, &witness.tx)?;

    let TransactionalStateManager {
        accum_logs_next, ..
    } = transactional;

    let mut tx_bytes = Vec::new();
    witness.tx.serialize(&mut tx_bytes)?;

    let tx_hash = Sha256::digest(&tx_bytes).into();

    let mut accum_logs_bytes_next = Vec::new();
    accum_logs_next.serialize(&mut accum_logs_bytes_next)?;
    let accum_logs_hash_next = Sha256::digest(&accum_logs_bytes_next).into();

    let mut hasher = Sha256::new();
    hasher.update(witness.state_root);
    hasher.update(witness.keys_root);
    let entire_root = hasher.finalize().into();

    let public = PublicInputTx::new(
        tx_hash,
        env_hash,
        accum_logs_hash_prev,
        accum_logs_hash_next,
        entire_root,
    );

    Ok(public)
}
