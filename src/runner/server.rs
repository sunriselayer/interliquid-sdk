use crate::types::InterLiquidSdkError;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve, Json, Router,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;

use super::message::{MessageTxReceived, RunnerMessage};

pub struct Server {
    sender: Sender<RunnerMessage>,
}

impl Server {
    pub fn new(sender: Sender<RunnerMessage>) -> Self {
        Self { sender }
    }

    pub async fn run(&self) -> Result<(), InterLiquidSdkError> {
        let server_app = Router::new()
            .route("/tx", post(handle_tx))
            .with_state(self.sender.clone());

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
    pub tx_base64: String,
}

async fn handle_tx(
    State(sender): State<Sender<RunnerMessage>>,
    Json(req): Json<TxRequest>,
) -> Result<impl IntoResponse, Response> {
    let tx = BASE64_STANDARD
        .decode(req.tx_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)).into_response())?;

    sender
        .send(RunnerMessage::TxReceived(MessageTxReceived::new(tx)))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to send message: {}", e),
            )
                .into_response()
        })?;

    Ok(StatusCode::OK)
}
