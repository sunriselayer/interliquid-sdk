use crate::{state::StateManager, types::InterLiquidSdkError};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    serve, Router,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast::Sender, RwLock};

use super::message::{MessageTxReceived, RunnerMessage};

/// Internal state container for the Server.
/// Holds a reference to the state manager for handling queries.
pub struct ServerState<S: StateManager> {
    state_manager: Arc<RwLock<S>>,
}

impl<S: StateManager> ServerState<S> {
    /// Creates a new ServerState instance.
    /// 
    /// # Arguments
    /// * `state_manager` - The state manager for handling blockchain state queries
    pub fn new(state_manager: Arc<RwLock<S>>) -> Self {
        Self { state_manager }
    }
}

impl<S: StateManager> Clone for ServerState<S> {
    fn clone(&self) -> Self {
        Self {
            state_manager: self.state_manager.clone(),
        }
    }
}

/// HTTP server component that exposes REST APIs for transaction submission and state queries.
/// 
/// # Type Parameters
/// * `S` - State manager type that implements the StateManager trait
pub struct Server<S: StateManager> {
    state: ServerState<S>,
    sender: Sender<RunnerMessage>,
}

impl<S: StateManager> Server<S> {
    /// Creates a new Server instance.
    /// 
    /// # Arguments
    /// * `state` - The server state containing the state manager
    /// * `sender` - Channel sender for broadcasting messages to other components
    pub fn new(state: ServerState<S>, sender: Sender<RunnerMessage>) -> Self {
        Self { state, sender }
    }

    /// Runs the HTTP server on port 3000.
    /// 
    /// Exposes the following endpoints:
    /// - POST /tx - Submit a transaction
    /// - GET /query/get/{key} - Query a single value by key
    /// - GET /query/iter/{key_prefix} - Query multiple values by key prefix
    /// 
    /// # Returns
    /// * `Ok(())` - If the server runs successfully
    /// * `Err(InterLiquidSdkError)` - If an error occurs during startup or operation
    pub async fn run(&self) -> Result<(), InterLiquidSdkError> {
        let server_app = Router::new()
            .route("/tx", post(handle_tx))
            .route("/query/get/{key}", get(handle_query_get))
            .route("/query/iter/{key_prefix}", get(handle_query_iter))
            .with_state((self.state.clone(), self.sender.clone()));

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        serve(listener, server_app)
            .await
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))
    }
}

/// Request body structure for transaction submission.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TxRequest {
    pub tx_base64: String,
}

/// Handles transaction submission requests.
/// 
/// # Arguments
/// * `state` - Server state and message sender
/// * `req` - Request containing base64-encoded transaction data
/// 
/// # Returns
/// * `Ok(StatusCode::OK)` - If the transaction is accepted
/// * `Err(Response)` - If the request is invalid or processing fails
async fn handle_tx<S: StateManager>(
    State((_state, sender)): State<(ServerState<S>, Sender<RunnerMessage>)>,
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

/// Handles single value query requests.
/// 
/// # Arguments
/// * `state` - Server state and message sender
/// * `key_base64` - Base64-encoded key to query
/// 
/// # Returns
/// * `Ok((StatusCode::OK, value))` - If the key exists, returns base64-encoded value
/// * `Ok(StatusCode::NOT_FOUND)` - If the key doesn't exist
/// * `Err(Response)` - If the request is invalid or query fails
async fn handle_query_get<S: StateManager>(
    State((state, _sender)): State<(ServerState<S>, Sender<RunnerMessage>)>,
    Path(key_base64): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let key = BASE64_STANDARD
        .decode(key_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)).into_response())?;

    let value = state.state_manager.read().await.get(&key).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get value: {}", e),
        )
            .into_response()
    })?;

    match value {
        Some(value) => Ok((StatusCode::OK, BASE64_STANDARD.encode(value)).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

/// Handles key prefix iteration query requests.
/// 
/// # Arguments
/// * `state` - Server state and message sender
/// * `key_prefix_base64` - Base64-encoded key prefix to iterate over
/// 
/// # Returns
/// * `Ok((StatusCode::OK, data))` - Returns base64-encoded serialized vector of key-value pairs
/// * `Err(Response)` - If the request is invalid or iteration fails
async fn handle_query_iter<S: StateManager>(
    State((state, _sender)): State<(ServerState<S>, Sender<RunnerMessage>)>,
    Path(key_prefix_base64): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let key_prefix = BASE64_STANDARD
        .decode(key_prefix_base64)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)).into_response())?;

    let vec = state
        .state_manager
        .read()
        .await
        .iter(key_prefix)
        .collect::<Result<Vec<(Vec<u8>, Vec<u8>)>, InterLiquidSdkError>>()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to iterate over key prefix: {}", e),
            )
                .into_response()
        })?;

    let mut buf = Vec::new();
    BorshSerialize::serialize(&vec, &mut buf).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize vector: {}", e),
        )
            .into_response()
    })?;

    Ok((StatusCode::OK, BASE64_STANDARD.encode(buf)).into_response())
}
