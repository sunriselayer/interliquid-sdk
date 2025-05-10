use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{AppI, Context},
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

    pub fn from_app<C: Context, TX: Tx>(
        app: &mut impl AppI<C, TX>,
        ctx: &mut C,
        txs: &[TX],
    ) -> Result<Vec<Self>, InterLiquidSdkError> {
        let mut logs = Vec::new();
        let mut accumulated_diffs = CompressedDiffs::from_logs(&logs);

        let snapshots = txs
            .iter()
            .map(|tx| {
                app.execute_tx(ctx, tx)?;
                let mut logs = Vec::new();
                let accumulated_diffs = CompressedDiffs::from_logs(&logs)?;

                Ok(Self::new(logs))
            })
            .collect::<Result<Vec<_>, InterLiquidSdkError>>()?;

        Ok(snapshots)
    }
}
