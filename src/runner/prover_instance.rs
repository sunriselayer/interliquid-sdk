use crate::{
    types::InterLiquidSdkError,
    zkp::{PrivateInputPatriciaTrie, PrivateInputSparseTree, WitnessTx, WitnessTxAgg},
};

pub trait ProverInstance {
    fn working(&self) -> bool;
    fn prove_tx(&self, witness: WitnessTx) -> Result<(), InterLiquidSdkError>;
    fn prove_aggregated_tx(&self, witness: WitnessTxAgg) -> Result<(), InterLiquidSdkError>;
    fn prove_commit_state(
        &self,
        witness: PrivateInputSparseTree,
    ) -> Result<(), InterLiquidSdkError>;
    fn prove_commit_keys(
        &self,
        witness: PrivateInputPatriciaTrie,
    ) -> Result<(), InterLiquidSdkError>;
}
