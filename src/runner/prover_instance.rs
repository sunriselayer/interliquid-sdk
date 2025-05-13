use crate::{
    types::InterLiquidSdkError,
    zkp::{PrivateInputPatriciaTrie, PrivateInputSparseTree, PrivateInputTxAgg, WitnessTx},
};

pub trait ProverInstance {
    fn working(&self) -> bool;
    fn prove_tx(&self, witness: WitnessTx) -> Result<(), InterLiquidSdkError>;
    fn prove_aggregated_tx(&self, witness: PrivateInputTxAgg) -> Result<(), InterLiquidSdkError>;
    fn prove_commit_state(
        &self,
        witness: PrivateInputSparseTree,
    ) -> Result<(), InterLiquidSdkError>;
    fn prove_commit_keys(
        &self,
        witness: PrivateInputPatriciaTrie,
    ) -> Result<(), InterLiquidSdkError>;
}
