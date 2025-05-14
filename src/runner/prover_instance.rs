use crate::{
    types::InterLiquidSdkError,
    zkp::{WitnessCommitKeys, WitnessCommitState, WitnessTx, WitnessTxAgg},
};

pub trait ProverInstance {
    fn working(&self) -> bool;
    fn prove_tx(&self, witness: WitnessTx) -> Result<(), InterLiquidSdkError>;
    fn prove_aggregated_tx(&self, witness: WitnessTxAgg) -> Result<(), InterLiquidSdkError>;
    fn prove_commit_state(&self, witness: WitnessCommitState) -> Result<(), InterLiquidSdkError>;
    fn prove_commit_keys(&self, witness: WitnessCommitKeys) -> Result<(), InterLiquidSdkError>;
}
