use crate::state::StateManager;

pub trait Context: Send + Sync {
    fn chain_id(&self) -> &str;
    fn block_height(&self) -> u64;
    fn block_time_unix_secs(&self) -> u64;
    fn state_manager(&self) -> &dyn StateManager;
    fn state_manager_mut(&mut self) -> &mut dyn StateManager;
}

pub struct SdkContext<'a, S: StateManager> {
    chain_id: String,
    block_height: u64,
    block_time_unix_secs: u64,
    state_manager: &'a mut S,
}

impl<'a, S: StateManager> SdkContext<'a, S> {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time_unix_secs: u64,
        state_manager: &'a mut S,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time_unix_secs,
            state_manager,
        }
    }
}

impl<'a, S: StateManager> Context for SdkContext<'a, S> {
    fn chain_id(&self) -> &str {
        &self.chain_id
    }

    fn block_height(&self) -> u64 {
        self.block_height
    }

    fn block_time_unix_secs(&self) -> u64 {
        self.block_time_unix_secs
    }

    fn state_manager(&self) -> &dyn StateManager {
        self.state_manager
    }

    fn state_manager_mut(&mut self) -> &mut dyn StateManager {
        self.state_manager
    }
}
