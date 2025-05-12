use crate::{
    core::{App, Tx},
    state::StateManager,
    types::InterLiquidSdkError,
};

use super::{savedata::SaveData, state::RunnerState};
pub struct Runner<TX: Tx, S: StateManager> {
    pub(super) state: RunnerState<TX, S>,
}

impl<TX: Tx, S: StateManager> Runner<TX, S> {
    pub fn new(app: App<TX>, savedata: SaveData, state_manager: S) -> Self {
        Self {
            state: RunnerState::new(app, savedata, state_manager),
        }
    }

    pub async fn run(&self) -> Result<(), InterLiquidSdkError> {
        let _ = tokio::try_join!(self.run_server(), self.run_prover());

        Ok(())
    }
}
