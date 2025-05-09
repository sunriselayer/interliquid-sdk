use crate::core::type_registry::TypeRegistry;
use crate::state::StateManager;

pub trait Context {
    fn chain_id(&self) -> &str;
    fn state_manager(&mut self) -> &mut dyn StateManager;
    fn type_registry(&self) -> &TypeRegistry;
}

pub struct SdkContext {
    chain_id: String,
    state_manager: Box<dyn StateManager>,
    type_registry: TypeRegistry,
}

impl SdkContext {
    pub fn new<S: StateManager>(
        chain_id: String,
        state_manager: S,
        type_registry: TypeRegistry,
    ) -> Self {
        Self {
            chain_id,
            state_manager: Box::new(state_manager),
            type_registry,
        }
    }
}

impl Context for SdkContext {
    fn chain_id(&self) -> &str {
        &self.chain_id
    }

    fn state_manager(&mut self) -> &mut dyn StateManager {
        self.state_manager.as_mut()
    }

    fn type_registry(&self) -> &TypeRegistry {
        &self.type_registry
    }
}
