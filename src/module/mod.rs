use crate::{
    state::StateManager,
    types::{Any, InterLiquidSdkError},
};

pub trait Keeper<S: StateManager> {
    fn handle_msg(&self, state: &mut S, msg: Any) -> Result<(), InterLiquidSdkError>;
}
