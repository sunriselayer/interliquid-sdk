use tokio::sync::broadcast::{Receiver, Sender};

use crate::types::InterLiquidSdkError;

use super::message::RunnerMessage;

pub struct Prover {
    sender: Sender<RunnerMessage>,
    receiver: Receiver<RunnerMessage>,
}

impl Prover {
    pub fn new(sender: Sender<RunnerMessage>, receiver: Receiver<RunnerMessage>) -> Self {
        Self { sender, receiver }
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
