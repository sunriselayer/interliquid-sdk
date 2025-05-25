use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use sha2::{Digest, Sha256};
use tokio::sync::{
    broadcast::{Receiver, Sender},
    Mutex, RwLock,
};

use super::{
    message::{MessageTxProofReady, RunnerMessage},
    savedata::{SaveData, TxExecutionSnapshot},
};
use crate::{
    core::{App, SdkContext, Tx},
    state::{StateManager, TransactionalStateManager},
    types::{Environment, InterLiquidSdkError},
    zkp::WitnessTx,
};

/// Internal state container for the Sequencer.
/// Holds references to the application logic, persistent storage, and state manager.
pub struct SequencerState<TX: Tx, S: StateManager> {
    app: Arc<App<TX>>,
    savedata: Arc<Mutex<SaveData>>,
    state_manager: Arc<RwLock<S>>,
}

impl<TX: Tx, S: StateManager> SequencerState<TX, S> {
    /// Creates a new SequencerState instance.
    ///
    /// # Arguments
    /// * `app` - The application instance containing business logic
    /// * `savedata` - Persistent storage for blockchain data
    /// * `state_manager` - The state manager for handling blockchain state
    pub fn new(
        app: Arc<App<TX>>,
        savedata: Arc<Mutex<SaveData>>,
        state_manager: Arc<RwLock<S>>,
    ) -> Self {
        Self {
            app,
            savedata,
            state_manager,
        }
    }
}

impl<TX: Tx, S: StateManager> Clone for SequencerState<TX, S> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
            savedata: self.savedata.clone(),
            state_manager: self.state_manager.clone(),
        }
    }
}

/// The Sequencer component responsible for processing transactions.
/// It executes transactions, updates state, and prepares witness data for proof generation.
///
/// # Type Parameters
/// * `TX` - Transaction type that implements the Tx trait
/// * `S` - State manager type that implements the StateManager trait
pub struct Sequencer<TX: Tx, S: StateManager> {
    state: SequencerState<TX, S>,
    sender: Sender<RunnerMessage>,
    receiver: Receiver<RunnerMessage>,
}

impl<TX: Tx, S: StateManager> Sequencer<TX, S> {
    /// Creates a new Sequencer instance.
    ///
    /// # Arguments
    /// * `state` - The sequencer state containing app, storage, and state manager
    /// * `sender` - Channel sender for broadcasting messages to other components
    /// * `receiver` - Channel receiver for receiving messages from other components
    pub fn new(
        state: SequencerState<TX, S>,
        sender: Sender<RunnerMessage>,
        receiver: Receiver<RunnerMessage>,
    ) -> Self {
        Self {
            state,
            sender,
            receiver,
        }
    }

    /// Runs the sequencer's main event loop.
    ///
    /// Listens for incoming messages and processes transactions when received.
    ///
    /// # Returns
    /// * `Ok(())` - If the sequencer runs successfully
    /// * `Err(InterLiquidSdkError)` - If an error occurs during processing
    pub async fn run(&mut self) -> Result<(), InterLiquidSdkError> {
        while let Ok(msg) = self.receiver.recv().await {
            match msg {
                RunnerMessage::TxReceived(msg) => {
                    if let Err(e) = self.handle_tx_received(msg.tx).await {
                        eprintln!("Failed to handle tx: {}", e);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Handles a received transaction by executing it and generating witness data.
    ///
    /// This method:
    /// 1. Executes the transaction against the current state
    /// 2. Collects state changes and logs
    /// 3. Generates witness data for proof generation
    /// 4. Updates the savedata with the execution snapshot
    /// 5. Sends a message that the transaction is ready for proving
    ///
    /// # Arguments
    /// * `tx` - The serialized transaction data to process
    ///
    /// # Returns
    /// * `Ok(())` - If the transaction is processed successfully
    /// * `Err(InterLiquidSdkError)` - If an error occurs during execution
    async fn handle_tx_received(&self, tx: Vec<u8>) -> Result<(), InterLiquidSdkError> {
        let app = self.state.app.clone();

        let mut savedata_lock = self.state.savedata.lock().await;
        let savedata = savedata_lock.deref_mut();

        let state_manager_lock = self.state.state_manager.read().await;
        let state_manager = state_manager_lock.deref();

        let accum_logs = savedata
            .tx_snapshots
            .last()
            .and_then(|snapshot| Some(snapshot.accum_logs.clone()))
            .unwrap_or_default();

        let accum_logs_prev = accum_logs;

        let mut transactional =
            TransactionalStateManager::from_accum_logs_prev(state_manager, accum_logs_prev);

        let env = Environment::new(
            savedata.chain_id.clone(),
            savedata.block_height,
            savedata.block_time,
        );

        let mut ctx = SdkContext::new(env, &mut transactional);

        app.execute_tx(&mut ctx, &tx)?;
        let SdkContext { env, .. } = ctx;

        let state_for_access = transactional.state_for_access_from_log()?;

        let TransactionalStateManager {
            logs,
            accum_logs_prev,
            accum_logs_next,
            ..
        } = transactional;

        let mut hasher = Sha256::new();
        hasher.update(&savedata.state_sparse_tree_root);
        hasher.update(&savedata.keys_patricia_trie_root);
        let entire_root = hasher.finalize().into();

        let witness = WitnessTx::new(tx, env, entire_root, state_for_access, accum_logs_prev);

        let snapshot = TxExecutionSnapshot::new(logs, accum_logs_next);

        self.sender
            .send(RunnerMessage::TxProofReady(MessageTxProofReady::new(
                savedata.chain_id.clone(),
                savedata.block_height,
                savedata.block_time_unix_secs,
                savedata.tx_snapshots.len() - 1,
                witness,
            )))
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        savedata.tx_snapshots.push(snapshot);

        Ok(())
    }
}
