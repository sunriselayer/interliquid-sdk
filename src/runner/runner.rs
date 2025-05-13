use std::sync::Arc;

use tokio::sync::{broadcast::channel, Mutex, RwLock};

use crate::{
    core::{App, Tx},
    state::StateManager,
    types::InterLiquidSdkError,
};

use super::{
    prover_orchestrator::ProverOrchestrator,
    savedata::SaveData,
    sequencer::{Sequencer, SequencerState},
    server::Server,
};

pub struct MonolithicRunner<TX: Tx, S: StateManager> {
    pub(super) server: Server,
    pub(super) sequencer: Sequencer<TX, S>,
    pub(super) prover: ProverOrchestrator,
}

impl<TX: Tx, S: StateManager> MonolithicRunner<TX, S> {
    pub fn new(app: App<TX>, savedata: SaveData, state_manager: S) -> Self {
        let (sender, receiver1) = channel(16);
        let receiver2 = sender.subscribe();

        Self {
            server: Server::new(sender.clone()),
            sequencer: Sequencer::new(
                SequencerState::new(
                    Arc::new(app),
                    Arc::new(Mutex::new(savedata)),
                    Arc::new(RwLock::new(state_manager)),
                ),
                sender.clone(),
                receiver1,
            ),
            prover: ProverOrchestrator::new(vec![], sender.clone(), receiver2),
        }
    }

    pub async fn run(&mut self) -> Result<(), InterLiquidSdkError> {
        tokio::try_join!(self.server.run(), self.sequencer.run(), self.prover.run())?;

        Ok(())
    }
}
