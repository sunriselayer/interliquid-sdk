

use crate::{
    types::InterLiquidSdkError,
    zkp::{PrivateInputPatriciaTrie, PrivateInputSparseTree, PrivateInputTx, PrivateInputTxAgg},
};

pub trait ProverInstance {
    fn working(&self) -> bool;
    fn prove_tx(&self, witness: PrivateInputTx) -> Result<(), InterLiquidSdkError>;
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
