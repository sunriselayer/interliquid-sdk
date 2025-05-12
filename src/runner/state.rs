use std::sync::Arc;

use tokio::sync::{broadcast::Sender, Mutex, RwLock};

use crate::{
    core::{App, Tx},
    state::StateManager,
};

use super::{message::RunnerMessage, savedata::SaveData};

pub struct RunnerState<TX: Tx, S: StateManager> {
    pub(super) app: Arc<App<TX>>,
    pub(super) savedata: Arc<Mutex<SaveData>>,
    pub(super) state_manager: Arc<RwLock<S>>,
    pub(super) message: Sender<RunnerMessage>,
}

impl<TX: Tx, S: StateManager> RunnerState<TX, S> {
    pub fn new(app: App<TX>, savedata: SaveData, state_manager: S) -> Self {
        Self {
            app: Arc::new(app),
            savedata: Arc::new(Mutex::new(savedata)),
            state_manager: Arc::new(RwLock::new(state_manager)),
            message: Sender::new(16),
        }
    }
}

impl<TX: Tx, S: StateManager> Clone for RunnerState<TX, S> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
            savedata: self.savedata.clone(),
            state_manager: self.state_manager.clone(),
            message: self.message.clone(),
        }
    }
}
