use crate::{state::StateManager, types::InterLiquidSdkError};

pub trait TxHandler<S: StateManager> {
    fn ante(&self, state: &mut S) -> Result<(), InterLiquidSdkError>;
    fn post(&self, state: &mut S) -> Result<(), InterLiquidSdkError>;
}
