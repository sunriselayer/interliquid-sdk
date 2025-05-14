use tokio::sync::broadcast::{Receiver, Sender};

use crate::{runner::RunnerMessage, types::InterLiquidSdkError};

use super::ProverInstance;

pub struct ProverOrchestrator {
    instances: Vec<Box<dyn ProverInstance>>,
    next_instance: usize,
    sender: Sender<RunnerMessage>,
    receiver: Receiver<RunnerMessage>,
}

impl ProverOrchestrator {
    pub fn new(
        instances: Vec<Box<dyn ProverInstance>>,
        sender: Sender<RunnerMessage>,
        receiver: Receiver<RunnerMessage>,
    ) -> Self {
        Self {
            instances,
            next_instance: 0,
            sender,
            receiver,
        }
    }

    pub async fn run(&mut self) -> Result<(), InterLiquidSdkError> {
        while let Ok(msg) = self.receiver.recv().await {
            match msg {
                RunnerMessage::TxProofReady(msg) => {}
                RunnerMessage::TxProofAggregationReady(msg) => {}
                RunnerMessage::CommitStateProofReady(msg) => {}
                RunnerMessage::CommitKeysProofReady(msg) => {}
                _ => {}
            }
        }

        Ok(())
    }
}
