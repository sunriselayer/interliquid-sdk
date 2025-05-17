use tokio::sync::broadcast::{Receiver, Sender};

use crate::{runner::RunnerMessage, types::InterLiquidSdkError};

use super::ProverInstance;

pub struct ProverOrchestrator {
    instances: Vec<Box<dyn ProverInstance>>,
    next_instance: Option<usize>,
    sender: Sender<RunnerMessage>,
    receiver: Receiver<RunnerMessage>,
}

impl ProverOrchestrator {
    /// `instances` can be emtpy, in which case the prover should be no-op, for example optimistic sovereign rollup.
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
