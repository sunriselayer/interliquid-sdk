use crate::state::StateManager;

use super::savedata::TxExecutionSnapshot;

pub struct RunnerState<S: StateManager + 'static> {
    pub(crate) chain_id: String,
    pub(crate) block_height: u64,
    pub(crate) block_time_unix_secs: u64,
    pub(crate) state_manager: S,
    pub(crate) tx_snapshots: Vec<TxExecutionSnapshot>,
}

impl<S: StateManager> RunnerState<S> {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time_unix_secs: u64,
        state_manager: S,
        tx_snapshots: Vec<TxExecutionSnapshot>,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time_unix_secs,
            state_manager,
            tx_snapshots,
        }
    }
}
