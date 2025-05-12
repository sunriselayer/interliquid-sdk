use crate::state::StateManager;

use super::savedata::SaveData;

pub struct RunnerState<S: StateManager + 'static> {
    pub(crate) savedata: SaveData,
    pub(crate) state_manager: S,
}

impl<S: StateManager> RunnerState<S> {
    pub fn new(savedata: SaveData, state_manager: S) -> Self {
        Self {
            savedata,
            state_manager,
        }
    }
}
