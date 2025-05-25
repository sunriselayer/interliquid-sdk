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

/// Local SP1 prover implementation that generates proofs using the SP1 proving system.
/// This prover runs locally and uses ELF (Executable and Linkable Format) files for different proof types.
pub struct ProverLocal {
    elf_tx: &'static [u8],
    elf_tx_agg: &'static [u8],
    elf_commit_state: &'static [u8],
    elf_commit_keys: &'static [u8],
    elf_block: &'static [u8],
}

impl ProverLocal {
    /// Creates a new ProverLocal instance with the specified ELF files.
    /// 
    /// # Arguments
    /// * `elf_tx` - ELF file for single transaction proofs
    /// * `elf_tx_agg` - ELF file for aggregated transaction proofs
    /// * `elf_commit_state` - ELF file for state commitment proofs
    /// * `elf_commit_keys` - ELF file for keys commitment proofs
    /// * `elf_block` - ELF file for block proofs
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

    /// Generic proof generation method used internally by all proof types.
    /// 
    /// # Type Parameters
    /// * `W` - Witness type that can be serialized with Borsh
    /// * `PI` - Public input type that can be deserialized from Borsh
    /// 
    /// # Arguments
    /// * `elf` - The ELF file containing the proof circuit
    /// * `witness` - The witness data to prove
    /// 
    /// # Returns
    /// * `Ok((proof, public_input))` - The generated proof bytes and deserialized public inputs
    /// * `Err(InterLiquidSdkError)` - If proof generation fails
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
    /// Generates a proof for a single transaction using the SP1 proving system.
    fn prove_tx(
        &self,
        witness: WitnessTx,
    ) -> Result<(Vec<u8>, PublicInputTx), InterLiquidSdkError> {
        self.prove::<WitnessTx, PublicInputTx>(self.elf_tx, witness)
    }

    /// Generates an aggregated proof for multiple transactions using the SP1 proving system.
    fn prove_aggregated_tx(
        &self,
        witness: WitnessTxAgg,
    ) -> Result<(Vec<u8>, PublicInputTxAgg), InterLiquidSdkError> {
        self.prove::<WitnessTxAgg, PublicInputTxAgg>(self.elf_tx_agg, witness)
    }

    /// Generates a proof for state commitment using the SP1 proving system.
    fn prove_commit_state(
        &self,
        witness: WitnessCommitState,
    ) -> Result<(Vec<u8>, PublicInputCommitState), InterLiquidSdkError> {
        self.prove::<WitnessCommitState, PublicInputCommitState>(self.elf_commit_state, witness)
    }

    /// Generates a proof for keys commitment using the SP1 proving system.
    fn prove_commit_keys(
        &self,
        witness: WitnessCommitKeys,
    ) -> Result<(Vec<u8>, PublicInputCommitKeys), InterLiquidSdkError> {
        self.prove::<WitnessCommitKeys, PublicInputCommitKeys>(self.elf_commit_keys, witness)
    }

    /// Generates a proof for an entire block using the SP1 proving system.
    fn prove_block(
        &self,
        witness: WitnessBlock,
    ) -> Result<(Vec<u8>, PublicInputBlock), InterLiquidSdkError> {
        self.prove::<WitnessBlock, PublicInputBlock>(self.elf_block, witness)
    }
}
