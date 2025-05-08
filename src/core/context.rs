use crate::state::StateManager;

pub trait Context {
    fn state_manager(&mut self) -> &mut dyn StateManager;
}

pub struct SdkContext {
    state_manager: Box<dyn StateManager>,
}

impl SdkContext {
    pub fn new<S: StateManager>(state_manager: S) -> Self {
        Self {
            state_manager: Box::new(state_manager),
        }
    }
}

impl Context for SdkContext {
    fn state_manager(&mut self) -> &mut dyn StateManager {
        self.state_manager.as_mut()
    }
}
