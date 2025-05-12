use crate::{tx::Tx, types::InterLiquidSdkError};

use super::Runner;

impl<TX: Tx> Runner<TX> {
    pub(super) async fn run_prover(&self) -> Result<(), InterLiquidSdkError> {
        todo!()
    }
}
