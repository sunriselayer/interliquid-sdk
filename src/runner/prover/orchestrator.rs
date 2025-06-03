use tokio::sync::broadcast::{Receiver, Sender};

use crate::{
    runner::{
        MessageCommitKeysProofReady, MessageCommitKeysProved, MessageCommitStateProofReady,
        MessageCommitStateProved, MessageTxProofAggregated, MessageTxProofAggregationReady,
        MessageTxProofReady, MessageTxProved, RunnerMessage,
    },
    types::InterLiquidSdkError,
};

use super::ProverInstance;

/// Orchestrates proof generation across multiple prover instances.
/// Manages the distribution of proof tasks and coordination between different proof types.
pub struct ProverOrchestrator {
    instances: Vec<Box<dyn ProverInstance>>,
    next_instance: Option<usize>,
    sender: Sender<RunnerMessage>,
    receiver: Receiver<RunnerMessage>,
}

impl ProverOrchestrator {
    /// Creates a new ProverOrchestrator instance.
    ///
    /// # Arguments
    /// * `instances` - List of prover instances to manage. Can be empty for no-op operation (e.g., optimistic rollups)
    /// * `sender` - Channel sender for broadcasting messages to other components
    /// * `receiver` - Channel receiver for receiving proof requests
    ///
    /// # Note
    /// If `instances` is empty, the orchestrator operates in no-op mode, useful for optimistic sovereign rollups.
    pub fn new(
        instances: Vec<Box<dyn ProverInstance>>,
        sender: Sender<RunnerMessage>,
        receiver: Receiver<RunnerMessage>,
    ) -> Self {
        let next_instance = if instances.is_empty() { None } else { Some(0) };

        Self {
            instances,
            next_instance,
            sender,
            receiver,
        }
    }

    /// Runs the prover orchestrator's main event loop.
    ///
    /// Listens for proof requests and distributes them to available prover instances.
    /// Currently implements a round-robin scheduling strategy.
    ///
    /// # Returns
    /// * `Ok(())` - If the orchestrator runs successfully
    /// * `Err(InterLiquidSdkError)` - If an error occurs during processing
    pub async fn run(&mut self) -> Result<(), InterLiquidSdkError> {
        while let Ok(msg) = self.receiver.recv().await {
            if let Some(current_instance) = self.next_instance {
                match msg {
                    RunnerMessage::TxProofReady(msg) => {
                        if let Err(e) = self.handle_tx_proof_ready(msg, current_instance).await {
                            eprintln!("Failed to handle tx proof ready: {}", e);
                        }
                    }
                    RunnerMessage::TxProofAggregationReady(msg) => {
                        if let Err(e) = self
                            .handle_tx_proof_aggregation_ready(msg, current_instance)
                            .await
                        {
                            eprintln!("Failed to handle tx proof aggregation ready: {}", e);
                        }
                    }
                    RunnerMessage::CommitStateProofReady(msg) => {
                        if let Err(e) = self
                            .handle_commit_state_proof_ready(msg, current_instance)
                            .await
                        {
                            eprintln!("Failed to handle commit state proof ready: {}", e);
                        }
                    }
                    RunnerMessage::CommitKeysProofReady(msg) => {
                        if let Err(e) = self
                            .handle_commit_keys_proof_ready(msg, current_instance)
                            .await
                        {
                            eprintln!("Failed to handle commit keys proof ready: {}", e);
                        }
                    }
                    _ => {}
                }

                // Update next instance for round-robin scheduling
                self.update_next_instance();
            }
        }

        Ok(())
    }

    /// Handles a transaction proof request by delegating to the specified prover instance.
    async fn handle_tx_proof_ready(
        &self,
        msg: MessageTxProofReady,
        instance_idx: usize,
    ) -> Result<(), InterLiquidSdkError> {
        let prover = &self.instances[instance_idx];

        // Generate proof
        let (proof, public_input) = prover.prove_tx(msg.witness)?;

        // Send proof completion message
        self.sender
            .send(RunnerMessage::TxProved(MessageTxProved::new(
                msg.chain_id,
                msg.block_height,
                msg.tx_index,
                proof,
                public_input,
            )))
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        Ok(())
    }

    /// Handles a transaction proof aggregation request by delegating to the specified prover instance.
    async fn handle_tx_proof_aggregation_ready(
        &self,
        msg: MessageTxProofAggregationReady,
        instance_idx: usize,
    ) -> Result<(), InterLiquidSdkError> {
        let prover = &self.instances[instance_idx];

        // Generate aggregated proof using the witness
        let (proof, public_input) = prover.prove_tx_agg(msg.witness)?;

        // Send proof completion message
        self.sender
            .send(RunnerMessage::TxProofAggregated(
                MessageTxProofAggregated::new(msg.chain_id, msg.block_height, msg.tx_index, proof, public_input),
            ))
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        Ok(())
    }

    /// Handles a state commitment proof request by delegating to the specified prover instance.
    async fn handle_commit_state_proof_ready(
        &self,
        msg: MessageCommitStateProofReady,
        instance_idx: usize,
    ) -> Result<(), InterLiquidSdkError> {
        let prover = &self.instances[instance_idx];

        // Generate state commitment proof using the witness from the message
        let (proof, public_input) = prover.prove_commit_state(msg.witness)?;

        // Send proof completion message
        self.sender
            .send(RunnerMessage::CommitStateProved(
                MessageCommitStateProved::new(
                    msg.chain_id,
                    msg.block_height,
                    msg.state_root,
                    proof,
                    public_input,
                ),
            ))
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        Ok(())
    }

    /// Handles a keys commitment proof request by delegating to the specified prover instance.
    async fn handle_commit_keys_proof_ready(
        &self,
        msg: MessageCommitKeysProofReady,
        instance_idx: usize,
    ) -> Result<(), InterLiquidSdkError> {
        let prover = &self.instances[instance_idx];

        // Generate keys commitment proof using the witness from the message
        let (proof, public_input) = prover.prove_commit_keys(msg.witness)?;

        // Send proof completion message
        self.sender
            .send(RunnerMessage::CommitKeysProved(
                MessageCommitKeysProved::new(msg.chain_id, msg.block_height, msg.keys_root, proof, public_input),
            ))
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        Ok(())
    }

    /// Updates the next prover instance index for round-robin scheduling.
    fn update_next_instance(&mut self) {
        if let Some(current) = self.next_instance {
            if self.instances.len() > 1 {
                self.next_instance = Some((current + 1) % self.instances.len());
            }
        }
    }
}
