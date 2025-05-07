use crate::{state::StateManager, types::InterLiquidSdkError};

pub trait Message<S: StateManager> {
    fn apply(&self, state: &mut S) -> Result<(), InterLiquidSdkError>;
}
