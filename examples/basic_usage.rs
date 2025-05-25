use base64::{prelude::BASE64_STANDARD, Engine};
use borsh::BorshSerialize;
use crypto_bigint::U256 as U256Lib;
use interliquid_sdk::{
    core::App,
    runner::{MonolithicRunner as Runner, SaveData},
    state::StateManager,
    types::{Address, InterLiquidSdkError, NamedSerializableType, SerializableAny, Tokens, U256},
    x::{
        auth::{
            ante::{AddrVerifyAnteHandler, SigVerifyAnteHandler, StdTx, TxBody},
            AuthKeeper, AuthModule,
        },
        bank::{BankKeeper, BankModule, MsgSend},
        crypto::{keeper::CryptoKeeper, module::CryptoModule, p256::VerifyingKeyP256},
    },
};
use std::collections::BTreeMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct MemoryStateManager {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl MemoryStateManager {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
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
    let alice_balance_key = b"bank/balances/"
        .iter()
        .chain(key_buf.iter())
        .cloned()
        .collect::<Vec<u8>>();
    let alice_initial_balance = U256::new(U256Lib::from(1000u64));
    let mut buf = vec![];
    alice_initial_balance.serialize(&mut buf).unwrap();
    state_manager.set(&alice_balance_key, &buf)?;

    // Create and register the bank module
    let mut crypto_keeper = CryptoKeeper::new();
    crypto_keeper
        .register_verifying_key::<VerifyingKeyP256>()
        .unwrap();
    let crypto_keeper = Arc::new(crypto_keeper);
    let auth_keeper = Arc::new(AuthKeeper::new(crypto_keeper.clone()));
    let bank_keeper = Arc::new(BankKeeper::new());

    let auth_module = Arc::new(AuthModule::new(auth_keeper.clone()));
    let bank_module = Arc::new(BankModule::new(bank_keeper));
    let crypto_module = Arc::new(CryptoModule::new(crypto_keeper.clone()));

    let app: App<StdTx> = App::new(
        vec![auth_module, bank_module, crypto_module],
        vec![
            Box::new(AddrVerifyAnteHandler::new()),
            Box::new(SigVerifyAnteHandler::new(
                auth_keeper.clone(),
                crypto_keeper.clone(),
            )),
        ],
        vec![],
    );

    // Create the runner with proper initialization
    let savedata = SaveData {
        chain_id: "test-chain".to_string(),
        block_height: 1,
        block_time: 0,
        state_sparse_tree_root: [0; 32],
        keys_patricia_trie_root: [0; 32],
        tx_snapshots: vec![],
    };

    let mut runner = Runner::new(app, state_manager, savedata, vec![]);

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
        let tx = StdTx {
            body: TxBody {
                msgs: vec![msg_any],
                timeout_seconds: 0,
                options: vec![],
            },
            auth_info: BTreeMap::new(),
            signature: BTreeMap::new(),
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
