use crate::state::StateManager;

pub struct SdkContext<'a> {
    chain_id: String,
    block_height: u64,
    block_time_unix_secs: u64,
    state_manager: Box<dyn StateManager + 'a>,
}

impl<'a> SdkContext<'a> {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time_unix_seconds: u64,
        state_manager: Box<dyn StateManager + 'a>,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time_unix_secs: block_time_unix_seconds,
            state_manager,
        }
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn block_height(&self) -> u64 {
        self.block_height
    }

    pub fn block_time_unix_secs(&self) -> u64 {
        self.block_time_unix_secs
    }

    pub fn state_manager(&self) -> &dyn StateManager {
        self.state_manager.as_ref()
    }

    pub fn state_manager_mut(&mut self) -> &mut dyn StateManager {
        self.state_manager.as_mut()
    }
}
