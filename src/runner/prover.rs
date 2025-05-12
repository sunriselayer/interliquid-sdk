use crate::{state::StateManager, tx::Tx, types::InterLiquidSdkError};

use super::Runner;

impl<S: StateManager, TX: Tx> Runner<S, TX> {
    pub(super) async fn run_prover(&self) -> Result<(), InterLiquidSdkError> {
        todo!()
    }
}
