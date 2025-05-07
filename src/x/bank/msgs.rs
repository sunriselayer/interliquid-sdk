use crate::{
    module::Keeper,
    state::StateManager,
    types::{Any, InterLiquidSdkError},
};

use super::BankKeeper;

impl<S: StateManager> Keeper<S> for BankKeeper<S> {
    fn handle_msg(&self, state: &mut S, msg: Any) -> Result<(), InterLiquidSdkError> {
        Ok(())
    }
}
