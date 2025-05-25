use crate::{
    types::InterLiquidSdkError,
    zkp::{
        PublicInputBlock, PublicInputCommitKeys, PublicInputCommitState, PublicInputTx,
        PublicInputTxAgg, WitnessBlock, WitnessCommitKeys, WitnessCommitState, WitnessTx,
        WitnessTxAgg,
    },
};

/// Trait defining the interface for zero-knowledge proof generation.
/// Implementations of this trait handle the actual proof computation for different types of operations.
pub trait ProverInstance {
    /// Generates a proof for a single transaction execution.
    /// 
    /// # Arguments
    /// * `witness` - The witness data containing transaction details and state changes
    /// 
    /// # Returns
    /// * `Ok((proof, public_input))` - The generated proof bytes and public inputs
    /// * `Err(InterLiquidSdkError)` - If proof generation fails
    fn prove_tx(&self, witness: WitnessTx)
        -> Result<(Vec<u8>, PublicInputTx), InterLiquidSdkError>;

    /// Generates an aggregated proof for multiple transactions.
    /// 
    /// # Arguments
    /// * `witness` - The witness data containing aggregated transaction information
    /// 
    /// # Returns
    /// * `Ok((proof, public_input))` - The generated aggregated proof and public inputs
    /// * `Err(InterLiquidSdkError)` - If proof generation fails
    fn prove_aggregated_tx(
        &self,
        witness: WitnessTxAgg,
    ) -> Result<(Vec<u8>, PublicInputTxAgg), InterLiquidSdkError>;

    /// Generates a proof for state commitment.
    /// 
    /// # Arguments
    /// * `witness` - The witness data containing state commitment information
    /// 
    /// # Returns
    /// * `Ok((proof, public_input))` - The generated proof and public inputs for state commitment
    /// * `Err(InterLiquidSdkError)` - If proof generation fails
    fn prove_commit_state(
        &self,
        witness: WitnessCommitState,
    ) -> Result<(Vec<u8>, PublicInputCommitState), InterLiquidSdkError>;

    /// Generates a proof for keys commitment.
    /// 
    /// # Arguments
    /// * `witness` - The witness data containing keys commitment information
    /// 
    /// # Returns
    /// * `Ok((proof, public_input))` - The generated proof and public inputs for keys commitment
    /// * `Err(InterLiquidSdkError)` - If proof generation fails
    fn prove_commit_keys(
        &self,
        witness: WitnessCommitKeys,
    ) -> Result<(Vec<u8>, PublicInputCommitKeys), InterLiquidSdkError>;

    /// Generates a proof for an entire block.
    /// 
    /// # Arguments
    /// * `witness` - The witness data containing complete block information
    /// 
    /// # Returns
    /// * `Ok((proof, public_input))` - The generated proof and public inputs for the block
    /// * `Err(InterLiquidSdkError)` - If proof generation fails
    fn prove_block(
        &self,
        witness: WitnessBlock,
    ) -> Result<(Vec<u8>, PublicInputBlock), InterLiquidSdkError>;
}
