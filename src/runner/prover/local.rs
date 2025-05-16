use borsh::{BorshDeserialize, BorshSerialize};
use sp1_sdk::{ProverClient, SP1Stdin};

use crate::{
    types::InterLiquidSdkError,
    zkp::{
        PublicInputBlock, PublicInputCommitKeys, PublicInputCommitState, PublicInputTx,
        PublicInputTxAgg, WitnessBlock, WitnessCommitKeys, WitnessCommitState, WitnessTx,
        WitnessTxAgg,
    },
};

use super::ProverInstance;

pub struct ProverLocal {
    elf_tx: &'static [u8],
    elf_tx_agg: &'static [u8],
    elf_commit_state: &'static [u8],
    elf_commit_keys: &'static [u8],
    elf_block: &'static [u8],
}

impl ProverLocal {
    pub fn new(
        elf_tx: &'static [u8],
        elf_tx_agg: &'static [u8],
        elf_commit_state: &'static [u8],
        elf_commit_keys: &'static [u8],
        elf_block: &'static [u8],
    ) -> Self {
        Self {
            elf_tx,
            elf_tx_agg,
            elf_commit_state,
            elf_commit_keys,
            elf_block,
        }
    }

    fn prove<W: BorshSerialize, PI: BorshDeserialize>(
        &self,
        elf: &'static [u8],
        witness: W,
    ) -> Result<(Vec<u8>, PI), InterLiquidSdkError> {
        let client = ProverClient::from_env();
        let mut witness_bytes = Vec::new();
        witness.serialize(&mut witness_bytes).unwrap();

        let mut stdin = SP1Stdin::new();
        stdin.write_vec(witness_bytes);

        let (pk, _vk) = client.setup(elf);
        let proof = client.prove(&pk, &stdin).run()?;

        let public_input_bytes = proof.public_values.to_vec();
        let public_input = PI::try_from_slice(&public_input_bytes)?;

        Ok((proof.bytes(), public_input))
    }
}

impl ProverInstance for ProverLocal {
    fn prove_tx(
        &self,
        witness: WitnessTx,
    ) -> Result<(Vec<u8>, PublicInputTx), InterLiquidSdkError> {
        self.prove::<WitnessTx, PublicInputTx>(self.elf_tx, witness)
    }

    fn prove_aggregated_tx(
        &self,
        witness: WitnessTxAgg,
    ) -> Result<(Vec<u8>, PublicInputTxAgg), InterLiquidSdkError> {
        self.prove::<WitnessTxAgg, PublicInputTxAgg>(self.elf_tx_agg, witness)
    }

    fn prove_commit_state(
        &self,
        witness: WitnessCommitState,
    ) -> Result<(Vec<u8>, PublicInputCommitState), InterLiquidSdkError> {
        self.prove::<WitnessCommitState, PublicInputCommitState>(self.elf_commit_state, witness)
    }

    fn prove_commit_keys(
        &self,
        witness: WitnessCommitKeys,
    ) -> Result<(Vec<u8>, PublicInputCommitKeys), InterLiquidSdkError> {
        self.prove::<WitnessCommitKeys, PublicInputCommitKeys>(self.elf_commit_keys, witness)
    }

    fn prove_block(
        &self,
        witness: WitnessBlock,
    ) -> Result<(Vec<u8>, PublicInputBlock), InterLiquidSdkError> {
        self.prove::<WitnessBlock, PublicInputBlock>(self.elf_block, witness)
    }
}
