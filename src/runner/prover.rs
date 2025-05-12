use crate::{state::StateManager, tx::Tx, types::InterLiquidSdkError};

use super::Runner;

impl<TX: Tx, S: StateManager + 'static> Runner<TX, S> {
    pub(super) async fn run_prover(&self) -> Result<(), InterLiquidSdkError> {
        todo!()
    }
}
