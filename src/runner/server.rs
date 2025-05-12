use crate::{
    core::{App, SdkContext},
    state::{StateManager, TransactionalStateManager},
    tx::Tx,
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
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

use super::{savedata::TxExecutionSnapshot, state::RunnerState, Runner};

type ServerState<S, TX> = (Arc<App<TX>>, Arc<Mutex<RunnerState<S>>>);

impl<TX: Tx, S: StateManager + 'static> Runner<TX, S> {
    pub(super) async fn run_server(&self) -> Result<(), InterLiquidSdkError> {
        let server_app = Router::new()
            .route("/tx", post(handle_tx::<S, TX>))
            .with_state((self.app.clone(), self.state.clone()));

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

async fn handle_tx<S: StateManager + 'static, TX: Tx>(
    State((app, runner_state)): State<ServerState<S, TX>>,
    Json(req): Json<TxRequest>,
) -> Result<impl IntoResponse, Response> {
    let tx = BASE64_STANDARD
        .decode(req.tx_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)).into_response())?;

    let app = app;

    with_locked(&runner_state, |runner_state| {
        let accum_diffs = runner_state
            .savedata
            .tx_snapshots
            .last()
            .and_then(|snapshot| Some(snapshot.accum_diffs.clone()))
            .unwrap_or_default();

        let mut transactional =
            TransactionalStateManager::from_diffs(&mut runner_state.state_manager, accum_diffs);

        let mut ctx = SdkContext::new(
            runner_state.savedata.chain_id.clone(),
            runner_state.savedata.block_height,
            runner_state.savedata.block_time_unix_secs,
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
        runner_state.savedata.tx_snapshots.push(snapshot);

        Ok(())
    })
    .await?;

    Ok(StatusCode::OK)
}

async fn with_locked<T, R, F: FnOnce(&mut T) -> R>(m: &Mutex<T>, f: F) -> R {
    let mut guard = m.lock().await;
    f(&mut *guard)
}
