use crate::{
    types::InterLiquidSdkError,
    zkp::{
        PublicInputBlock, PublicInputCommitKeys, PublicInputCommitState, PublicInputTx,
        PublicInputTxAgg, WitnessBlock, WitnessCommitKeys, WitnessCommitState, WitnessTx,
        WitnessTxAgg,
    },
};

use super::ProverInstance;

pub struct ProverLocal {}

impl ProverInstance for ProverLocal {
    fn prove_tx(
        &self,
        witness: WitnessTx,
    ) -> Result<(Vec<u8>, PublicInputTx), InterLiquidSdkError> {
        todo!()
    }

    fn prove_aggregated_tx(
        &self,
        witness: WitnessTxAgg,
    ) -> Result<(Vec<u8>, PublicInputTxAgg), InterLiquidSdkError> {
        todo!()
    }

    fn prove_commit_state(
        &self,
        witness: WitnessCommitState,
    ) -> Result<(Vec<u8>, PublicInputCommitState), InterLiquidSdkError> {
        todo!()
    }

    fn prove_commit_keys(
        &self,
        witness: WitnessCommitKeys,
    ) -> Result<(Vec<u8>, PublicInputCommitKeys), InterLiquidSdkError> {
        todo!()
    }

    fn prove_block(
        &self,
        witness: WitnessBlock,
    ) -> Result<(Vec<u8>, PublicInputBlock), InterLiquidSdkError> {
        todo!()
    }
}
