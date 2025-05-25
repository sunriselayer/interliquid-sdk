use crate::{state::TracableStateManager, types::Environment};

/// Context bundles the info for tx executions.
pub trait Context: Send + Sync {
    fn env(&self) -> &Environment;
    fn state_manager(&self) -> &dyn TracableStateManager;
    fn state_manager_mut(&mut self) -> &mut dyn TracableStateManager;
}

pub struct SdkContext<'a, S: TracableStateManager> {
    pub(crate) env: Environment,
    pub(crate) state_manager: &'a mut S,
}

impl<'a, S: TracableStateManager> SdkContext<'a, S> {
    pub fn new(env: Environment, state_manager: &'a mut S) -> Self {
        Self { env, state_manager }
    }
}

impl<'a, S: TracableStateManager> Context for SdkContext<'a, S> {
    fn env(&self) -> &Environment {
        &self.env
    }

    fn state_manager(&self) -> &dyn TracableStateManager {
        self.state_manager
    }

    fn state_manager_mut(&mut self) -> &mut dyn TracableStateManager {
        self.state_manager
    }
}
