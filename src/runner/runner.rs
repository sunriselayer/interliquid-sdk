use crate::{
    core::{App, Context},
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
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub struct Runner<C: Context, TX: Tx> {
    app: App<C, TX>,
    ctx: C,
}

impl<C: Context, TX: Tx> Runner<C, TX> {
    pub fn new(app: App<C, TX>, ctx: C) -> Self {
        Self { app, ctx }
    }
}

pub struct ServerState<C: Context, TX: Tx> {
    app: Arc<App<C, TX>>,
    ctx: Arc<Mutex<C>>,
}

impl<C: Context, TX: Tx> Clone for ServerState<C, TX> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
            ctx: self.ctx.clone(),
        }
    }
}
impl<C: Context, TX: Tx> Runner<C, TX> {
    pub async fn serve(self) -> Result<(), InterLiquidSdkError> {
        let server_app = Router::new()
            .route("/tx", post(handle_tx::<C, TX>))
            .with_state(ServerState {
                app: Arc::new(self.app),
                ctx: Arc::new(Mutex::new(self.ctx)),
            });

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

async fn handle_tx<C: Context, TX: Tx>(
    State(state): State<ServerState<C, TX>>,
    Json(req): Json<TxRequest>,
) -> Result<impl IntoResponse, Response> {
    let tx_bytes = BASE64_STANDARD
        .decode(req.tx_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)).into_response())?;

    let tx = TX::try_from_slice(&tx_bytes).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to deserialize tx: {}", e),
        )
            .into_response()
    })?;

    let app = state.app;
    let mut ctx = state.ctx.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to lock context: {}", e),
        )
            .into_response()
    })?;

    // TODO: Create context and execute tx
    app.execute_tx(&mut ctx, &tx).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to execute tx: {}", e),
        )
            .into_response()
    })?;

    Ok(StatusCode::OK)
}
