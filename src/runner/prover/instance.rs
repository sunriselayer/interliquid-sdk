use crate::{
    types::InterLiquidSdkError,
    zkp::{
        PublicInputBlock, PublicInputCommitKeys, PublicInputCommitState, PublicInputTx,
        PublicInputTxAgg, WitnessBlock, WitnessCommitKeys, WitnessCommitState, WitnessTx,
        WitnessTxAgg,
    },
};

pub trait ProverInstance {
    fn prove_tx(&self, witness: WitnessTx)
        -> Result<(Vec<u8>, PublicInputTx), InterLiquidSdkError>;

    fn prove_aggregated_tx(
        &self,
        witness: WitnessTxAgg,
    ) -> Result<(Vec<u8>, PublicInputTxAgg), InterLiquidSdkError>;

    fn prove_commit_state(
        &self,
        witness: WitnessCommitState,
    ) -> Result<(Vec<u8>, PublicInputCommitState), InterLiquidSdkError>;

    fn prove_commit_keys(
        &self,
        witness: WitnessCommitKeys,
    ) -> Result<(Vec<u8>, PublicInputCommitKeys), InterLiquidSdkError>;

    fn prove_block(
        &self,
        witness: WitnessBlock,
    ) -> Result<(Vec<u8>, PublicInputBlock), InterLiquidSdkError>;
}
