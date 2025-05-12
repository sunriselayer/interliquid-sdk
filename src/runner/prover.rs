use crate::{core::Tx, state::StateManager, types::InterLiquidSdkError};

use super::Runner;

impl<TX: Tx, S: StateManager> Runner<TX, S> {
    pub(super) async fn run_prover(&self) -> Result<(), InterLiquidSdkError> {
        todo!()
    }
}
