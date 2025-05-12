use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{core::App, state::StateManager, tx::Tx, types::InterLiquidSdkError};

use super::{savedata::SaveData, state::RunnerState};
pub struct Runner<S: StateManager + 'static, TX: Tx> {
    pub(super) app: Arc<App<TX>>,
    pub(super) state: Arc<Mutex<RunnerState<S>>>,
}

impl<S: StateManager, TX: Tx> Runner<S, TX> {
    pub fn new(app: App<TX>, savedata: SaveData, state_manager: S) -> Self {
        Self {
            app: Arc::new(app),
            state: Arc::new(Mutex::new(RunnerState::new(
                savedata.chain_id,
                savedata.block_height,
                savedata.block_time_unix_secs,
                state_manager,
                savedata.tx_snapshots,
            ))),
        }
    }

    pub async fn run(&self) -> Result<(), InterLiquidSdkError> {
        let _ = tokio::try_join!(self.run_server(), self.run_prover());

        Ok(())
    }
}
