use crate::state::StateManager;

use super::MsgRegistry;

pub trait Context: Send + Sync + 'static {
    fn chain_id(&self) -> &str;
    fn block_height(&self) -> u64;
    fn block_time_seconds(&self) -> u64;
    fn state_manager(&mut self) -> &mut dyn StateManager;
    fn msg_registry(&self) -> &MsgRegistry;
}

pub struct SdkContext {
    chain_id: String,
    block_height: u64,
    block_time_seconds: u64,
    state_manager: Box<dyn StateManager>,
    msg_registry: MsgRegistry,
}

impl SdkContext {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time_seconds: u64,
        state_manager: Box<dyn StateManager>,
        msg_registry: MsgRegistry,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time_seconds,
            state_manager,
            msg_registry,
        }
    }
}

impl Context for SdkContext {
    fn chain_id(&self) -> &str {
        &self.chain_id
    }

    fn block_height(&self) -> u64 {
        self.block_height
    }

    fn block_time_seconds(&self) -> u64 {
        self.block_time_seconds
    }

    fn state_manager(&mut self) -> &mut dyn StateManager {
        self.state_manager.as_mut()
    }

    fn msg_registry(&self) -> &MsgRegistry {
        &self.msg_registry
    }
}
