use std::sync::Arc;

use tokio::sync::{broadcast::Sender, Mutex};

use crate::{core::App, state::StateManager, tx::Tx, types::InterLiquidSdkError};

use super::{message::RunnerMessage, savedata::SaveData, state::RunnerState};
pub struct Runner<TX: Tx, S: StateManager + 'static> {
    pub(super) app: Arc<App<TX>>,
    pub(super) state: Arc<Mutex<RunnerState<S>>>,
    pub(super) message: Sender<RunnerMessage>,
}

impl<TX: Tx, S: StateManager + 'static> Runner<TX, S> {
    pub fn new(app: App<TX>, savedata: SaveData, state_manager: S) -> Self {
        Self {
            app: Arc::new(app),
            state: Arc::new(Mutex::new(RunnerState::new(savedata, state_manager))),
            message: Sender::new(16),
        }
    }

    pub async fn run(&self) -> Result<(), InterLiquidSdkError> {
        let _ = tokio::try_join!(self.run_server(), self.run_prover());

        Ok(())
    }
}
