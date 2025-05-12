use crate::{
    core::{SdkContext, Tx},
    state::{StateManager, TransactionalStateManager},
    types::InterLiquidSdkError,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve, Json, Router,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use super::{savedata::TxExecutionSnapshot, state::RunnerState, Runner};

impl<TX: Tx, S: StateManager> Runner<TX, S> {
    pub(super) async fn run_server(&self) -> Result<(), InterLiquidSdkError> {
        let server_app = Router::new()
            .route("/tx", post(handle_tx::<TX, S>))
            .with_state(self.state.clone());

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        serve(listener, server_app)
            .await
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TxRequest {
    tx_base64: String,
}

async fn handle_tx<TX: Tx, S: StateManager>(
    State(runner_state): State<RunnerState<TX, S>>,
    Json(req): Json<TxRequest>,
) -> Result<impl IntoResponse, Response> {
    let tx = BASE64_STANDARD
        .decode(req.tx_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)).into_response())?;

    let app = runner_state.app.clone();

    let mut savedata_lock = runner_state.savedata.lock().await;
    let savedata = savedata_lock.deref_mut();

    let state_manager_lock = runner_state.state_manager.read().await;
    let state_manager = state_manager_lock.deref();

    let accum_diffs = savedata
        .tx_snapshots
        .last()
        .and_then(|snapshot| Some(snapshot.accum_diffs.clone()))
        .unwrap_or_default();

    let mut transactional = TransactionalStateManager::from_diffs(state_manager, accum_diffs);

    let mut ctx = SdkContext::new(
        savedata.chain_id.clone(),
        savedata.block_height,
        savedata.block_time_unix_secs,
        &mut transactional,
    );

    app.execute_tx(&mut ctx, &tx).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to execute tx: {}", e),
        )
            .into_response()
    })?;

    let snapshot = TxExecutionSnapshot::new(tx, transactional.logs, transactional.diffs);
    savedata.tx_snapshots.push(snapshot);

    Ok(StatusCode::OK)
}
