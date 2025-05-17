use std::sync::Arc;

use tokio::sync::{broadcast::channel, Mutex, RwLock};

use crate::{
    core::{App, Tx},
    state::StateManager,
    types::InterLiquidSdkError,
};

use super::{
    savedata::SaveData,
    sequencer::{Sequencer, SequencerState},
    server::{Server, ServerState},
    ProverInstance, ProverOrchestrator,
};

pub struct MonolithicRunner<TX: Tx, S: StateManager> {
    pub(super) server: Server<S>,
    pub(super) sequencer: Sequencer<TX, S>,
    pub(super) prover: ProverOrchestrator,
}

impl<TX: Tx, S: StateManager> MonolithicRunner<TX, S> {
    pub fn new(
        app: App<TX>,
        state_manager: S,
        savedata: SaveData,
        prover_instances: Vec<Box<dyn ProverInstance>>,
    ) -> Self {
        let state_manager = Arc::new(RwLock::new(state_manager));
        let (sender, receiver1) = channel(16);
        let receiver2 = sender.subscribe();

        Self {
            server: Server::new(ServerState::new(state_manager.clone()), sender.clone()),
            sequencer: Sequencer::new(
                SequencerState::new(
                    Arc::new(app),
                    Arc::new(Mutex::new(savedata)),
                    state_manager.clone(),
                ),
                sender.clone(),
                receiver1,
            ),
            prover: ProverOrchestrator::new(prover_instances, sender.clone(), receiver2),
        }
    }

    pub async fn run(&mut self) -> Result<(), InterLiquidSdkError> {
        tokio::try_join!(self.server.run(), self.sequencer.run(), self.prover.run())?;

        Ok(())
    }
}
