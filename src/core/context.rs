use crate::{state::TracableStateManager, types::Environment};

/// Context bundles the info for tx executions.
pub trait Context: Send + Sync {
    /// Returns a reference to the execution environment.
    fn env(&self) -> &Environment;
    
    /// Returns a reference to the state manager for read operations.
    fn state_manager(&self) -> &dyn TracableStateManager;
    
    /// Returns a mutable reference to the state manager for write operations.
    fn state_manager_mut(&mut self) -> &mut dyn TracableStateManager;
}

/// Default implementation of Context that holds an environment and state manager.
pub struct SdkContext<'a, S: TracableStateManager> {
    pub(crate) env: Environment,
    pub(crate) state_manager: &'a mut S,
}

impl<'a, S: TracableStateManager> SdkContext<'a, S> {
    /// Creates a new SdkContext with the given environment and state manager.
    ///
    /// # Arguments
    /// * `env` - The execution environment
    /// * `state_manager` - Mutable reference to the state manager
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
