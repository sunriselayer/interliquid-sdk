use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{App, SdkContext},
    state::{CompressedDiffs, StateLog},
    tx::Tx,
    types::InterLiquidSdkError,
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct TxExecutionSnapshot {
    pub logs: Vec<StateLog>,
}

impl TxExecutionSnapshot {
    pub fn new(logs: Vec<StateLog>) -> Self {
        Self { logs }
    }

    pub fn from_app<TX: Tx>(
        app: &mut App<TX>,
        ctx: &mut SdkContext,
        txs: &[TX],
    ) -> Result<Vec<Self>, InterLiquidSdkError> {
        let mut logs = Vec::new();
        let mut accumulated_diffs = CompressedDiffs::from_logs(logs.iter())?;

        let snapshots = txs
            .iter()
            .map(|tx| {
                app.execute_tx(ctx, tx)?;
                let mut logs = Vec::new();
                let accumulated_diffs = CompressedDiffs::from_logs(logs.iter())?;

                Ok(Self::new(logs))
            })
            .collect::<Result<Vec<_>, InterLiquidSdkError>>()?;

        Ok(snapshots)
    }
}
