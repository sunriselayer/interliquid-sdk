use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    core::{App, SdkContext},
    state::StateManager,
    tx::Tx,
    types::InterLiquidSdkError,
};
pub struct Runner<TX: Tx> {
    pub(super) app: Arc<App<TX>>,
    pub(super) ctx: Arc<Mutex<SdkContext>>,
}

impl<TX: Tx> Runner<TX> {
    pub fn new(app: App<TX>, state_manager: impl StateManager) -> Self {
        Self {
            app: Arc::new(app),
            ctx: Arc::new(Mutex::new(SdkContext::new(
                "".to_owned(),
                0,
                0,
                Box::new(state_manager),
            ))),
        }
    }

    pub async fn run(&self) -> Result<(), InterLiquidSdkError> {
        let _ = tokio::try_join!(self.run_server(), self.run_prover());

        Ok(())
    }
}
