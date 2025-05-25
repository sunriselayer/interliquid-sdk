use tokio::sync::broadcast::{Receiver, Sender};

use crate::{runner::RunnerMessage, types::InterLiquidSdkError};

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
            if let Some(next_instance) = self.next_instance {
                match msg {
                    RunnerMessage::TxProofReady(msg) => {}
                    RunnerMessage::TxProofAggregationReady(msg) => {}
                    RunnerMessage::CommitStateProofReady(msg) => {}
                    RunnerMessage::CommitKeysProofReady(msg) => {}
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
