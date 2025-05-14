use interliquid_sdk::{
    core::{App, Tx},
    state::StateManager,
    types::{Address, InterLiquidSdkError, Tokens, U256, SerializableAny, NamedSerializableType},
    x::bank::{BankKeeper, BankModule, MsgSend},
    runner::{MonolithicRunner as Runner, SaveData},
};
use borsh::{BorshSerialize, BorshDeserialize};
use borsh_derive::{BorshSerialize, BorshDeserialize};
use crypto_bigint::U256 as U256Lib;
use std::collections::BTreeMap;
use std::sync::Arc;
use base64::{prelude::BASE64_STANDARD, Engine};
use tokio::sync::mpsc;
use std::io::Write;
use std::net::TcpStream;

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct SimpleTx {
    pub msgs: Vec<SerializableAny>,
}

impl Tx for SimpleTx {
    fn msgs(&self) -> Vec<SerializableAny> {
        self.msgs.clone()
    }
}

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

#[tokio::main]
async fn main() -> Result<(), InterLiquidSdkError> {
    // Create addresses
    let alice = Address::from([1; 32]);
    let bob = Address::from([2; 32]);

    // Set up initial state
    let mut state_manager = MemoryStateManager::new();
    let mut key_buf = Vec::new();
    (alice, "usdc".to_string()).serialize(&mut key_buf).unwrap();
    let alice_balance_key = b"bank/balances/".iter().chain(key_buf.iter()).cloned().collect::<Vec<u8>>();
    let alice_initial_balance = U256::new(U256Lib::from(1000u64));
    let mut buf = vec![];
    alice_initial_balance.serialize(&mut buf).unwrap();
    state_manager.set(&alice_balance_key, &buf)?;

    // Create and register the bank module
    let bank_keeper = BankKeeper::new();
    let bank_module = Arc::new(BankModule::new(bank_keeper));
    let app: App<SimpleTx> = App::new(vec![bank_module], vec![], vec![]);

    // Create the runner with proper initialization
    let savedata = SaveData {
        chain_id: "test-chain".to_string(),
        block_height: 1,
        block_time_unix_secs: 0,
        state_sparse_tree_root: [0; 32],
        keys_patricia_trie_root: [0; 32],
        tx_snapshots: vec![],
    };
    
    let mut runner = Runner::new(app, savedata, state_manager);

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

        // Send transaction via TCP
        let tx_base64 = BASE64_STANDARD.encode(&tx_bytes);
        let request = format!(
            "POST /tx HTTP/1.1\r\n\
             Host: localhost:3000\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             \r\n\
             {{\"tx_base64\":\"{}\"}}",
            tx_base64.len() + 15,
            tx_base64
        );

        if let Ok(mut stream) = TcpStream::connect("localhost:3000") {
            if stream.write_all(request.as_bytes()).is_ok() {
                println!("Transaction sent successfully!");
            } else {
                eprintln!("Failed to send transaction");
            }
        } else {
            eprintln!("Failed to connect to server");
        }

        // Signal to stop the server
        let _ = tx_clone.send(()).await;
    });

    // Run the server
    println!("Starting server on http://localhost:3000");
    runner.run().await?;

    // Wait for the signal to stop
    let _ = rx.recv().await;

    Ok(())
} 