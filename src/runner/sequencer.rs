use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

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
    types::InterLiquidSdkError,
    zkp::PrivateInputTx,
};

pub struct SequencerState<TX: Tx, S: StateManager> {
    app: Arc<App<TX>>,
    savedata: Arc<Mutex<SaveData>>,
    state_manager: Arc<RwLock<S>>,
}

impl<TX: Tx, S: StateManager> SequencerState<TX, S> {
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

pub struct Sequencer<TX: Tx, S: StateManager> {
    state: SequencerState<TX, S>,
    sender: Sender<RunnerMessage>,
    receiver: Receiver<RunnerMessage>,
}

impl<TX: Tx, S: StateManager> Sequencer<TX, S> {
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

    async fn handle_tx_received(&self, tx: Vec<u8>) -> Result<(), InterLiquidSdkError> {
        let app = self.state.app.clone();

        let mut savedata_lock = self.state.savedata.lock().await;
        let savedata = savedata_lock.deref_mut();

        let state_manager_lock = self.state.state_manager.read().await;
        let state_manager = state_manager_lock.deref();

        let accum_diffs = savedata
            .tx_snapshots
            .last()
            .and_then(|snapshot| Some(snapshot.accum_diffs.clone()))
            .unwrap_or_default();

        let accum_diffs_prev = accum_diffs.clone();

        let mut transactional = TransactionalStateManager::from_diffs(state_manager, accum_diffs);

        let mut ctx = SdkContext::new(
            savedata.chain_id.clone(),
            savedata.block_height,
            savedata.block_time_unix_secs,
            &mut transactional,
        );

        app.execute_tx(&mut ctx, &tx)?;

        let TransactionalStateManager {
            state_manager,
            logs,
            diffs,
        } = transactional;

        let input = PrivateInputTx::from(
            tx,
            savedata.state_sparse_tree_root,
            savedata.keys_patricia_trie_root,
            &logs,
            accum_diffs_prev,
            state_manager,
        );

        let snapshot = TxExecutionSnapshot::new(logs, diffs);

        self.sender
            .send(RunnerMessage::TxProofReady(MessageTxProofReady::new(
                savedata.chain_id.clone(),
                savedata.block_height,
                savedata.block_time_unix_secs,
                savedata.tx_snapshots.len() - 1,
                input,
            )))
            .map_err(|e| InterLiquidSdkError::Other(anyhow::anyhow!(e)))?;

        savedata.tx_snapshots.push(snapshot);

        Ok(())
    }
}
