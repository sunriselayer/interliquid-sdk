use interliquid_sdk::{
    core::{App, SdkContext, Tx},
    state::StateManager,
    types::{Address, InterLiquidSdkError, Tokens, U256, SerializableAny, NamedSerializableType},
    x::bank::{BankKeeper, BankModule, BankKeeperI, MsgSend},
    runner::{MonolithicRunner as Runner, SaveData},
};
use borsh::{BorshSerialize, BorshDeserialize};
use borsh_derive::{BorshSerialize, BorshDeserialize};
use crypto_bigint::U256 as U256Lib;
use std::collections::BTreeMap;
use std::sync::Arc;
use reqwest;
use base64::{prelude::BASE64_STANDARD, Engine};
use anyhow::anyhow;
use tokio::sync::mpsc;

// Define a simple transaction struct that implements Tx
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SimpleTx {
    pub msgs: Vec<SerializableAny>,
}

impl Tx for SimpleTx {
    fn msgs(&self) -> Vec<SerializableAny> {
        self.msgs.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), InterLiquidSdkError> {
    // Minimal in-memory StateManager for demonstration
    #[derive(Clone)]
    pub struct MemoryStateManager {
        pub map: BTreeMap<Vec<u8>, Vec<u8>>,
    }

    impl MemoryStateManager {
        pub fn new() -> Self {
            Self { map: BTreeMap::new() }
        }
    }

    impl StateManager for MemoryStateManager {
        fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
            Ok(self.map.get(key).cloned())
        }

        fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError> {
            self.map.insert(key.to_vec(), value.to_vec());
            Ok(())
        }

        fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError> {
            self.map.remove(key);
            Ok(())
        }

        fn iter<'a>(
            &'a self,
            key_prefix: Vec<u8>,
        ) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
            Box::new(self.map.iter().filter_map(move |(k, v)| {
                if k.starts_with(&key_prefix) {
                    Some(Ok((k.clone(), v.clone())))
                } else {
                    None
                }
            }))
        }
    }

    // Create addresses
    let alice = Address::from([1; 32]);
    let bob = Address::from([2; 32]);

    // Set up initial state
    let mut init_state_manager = MemoryStateManager::new();
    let mut key_buf = Vec::new();
    (alice, "usdc".to_string()).serialize(&mut key_buf).unwrap();
    let alice_balance_key = b"bank/balances/".iter().chain(key_buf.iter()).cloned().collect::<Vec<u8>>();
    let alice_initial_balance = U256::new(U256Lib::from(1000u64));
    let mut buf = vec![];
    alice_initial_balance.serialize(&mut buf).unwrap();
    init_state_manager.set(&alice_balance_key, &buf)?;

    // Create state manager for context using the initialized state
    let mut ctx_state_manager = init_state_manager;
    let mut ctx = SdkContext::new(
        "test-chain".to_string(),
        1,
        0,
        &mut ctx_state_manager,
    );

    // Create and register the bank module
    let bank_keeper = BankKeeper::new();
    let bank_module = Arc::new(BankModule::new(bank_keeper));
    let mut app: App<SimpleTx> = App::new(vec![bank_module], vec![], vec![]);

    // Create the runner with proper initialization
    let savedata = SaveData {
        chain_id: "test-chain".to_string(),
        block_height: 1,
        block_time_unix_secs: 0,
        state_sparse_tree_root: [0; 32],
        keys_patricia_trie_root: [0; 32],
        tx_snapshots: vec![],
    };
    
    // Create a separate state manager for the runner
    let runner_state_manager = MemoryStateManager::new();
    let mut runner = Runner::new(app, savedata, runner_state_manager);

    // Create a channel for signaling when to stop the server
    let (tx, mut rx) = mpsc::channel(1);

    // Spawn a task to send the transaction
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        // Wait for server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Create a MsgSend transaction
        let mut tokens = Tokens::new();
        tokens.insert("usdc".to_string(), U256::new(U256Lib::from(100u64)));
        let msg = MsgSend {
            from_address: alice,
            to_address: bob,
            tokens,
        };
        let mut msg_bytes = Vec::new();
        msg.serialize(&mut msg_bytes).unwrap();
        let msg_any = SerializableAny::new(MsgSend::type_name().to_owned(), msg_bytes);

        // Wrap the message in a SimpleTx
        let tx = SimpleTx {
            msgs: vec![msg_any],
        };
        let mut tx_bytes = Vec::new();
        tx.serialize(&mut tx_bytes).unwrap();

        // Send transaction via HTTP
        let client = reqwest::Client::new();
        let tx_base64 = BASE64_STANDARD.encode(&tx_bytes);
        let response = client
            .post("http://localhost:3000/tx")
            .json(&serde_json::json!({
                "tx_base64": tx_base64
            }))
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            let error = response.text().await.unwrap();
            eprintln!("Transaction failed: {}", error);
        } else {
            println!("Transaction sent successfully!");
        }

        // Signal to stop the server
        let _ = tx_clone.send(()).await;
    });

    // Run the server in the main task
    println!("Starting server...");
    tokio::select! {
        _ = runner.run() => {
            println!("Server stopped");
        }
        _ = rx.recv() => {
            println!("Received stop signal");
        }
    }

    // Check balances (for demonstration)
    let bank_keeper = BankKeeper::new();
    let alice_balance = bank_keeper.get_balance(&mut ctx, &alice, "usdc")?;
    let bob_balance = bank_keeper.get_balance(&mut ctx, &bob, "usdc")?;
    println!("Alice's balance: {:?}", alice_balance);
    println!("Bob's balance: {:?}", bob_balance);

    Ok(())
} 