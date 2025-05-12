use interliquid_sdk::{
    core::{App, MsgRegistry, SdkContext, Context, Module},
    state::StateManager,
    types::{Address, InterLiquidSdkError, Tokens, U256, SerializableAny},
    x::bank::{BankKeeper, BankModule, BankKeeperI},
    x::bank::msg_send::MsgSend,
    runner::{Runner, savedata::SaveData},
    tx::Tx,
};
use borsh::{BorshSerialize, BorshDeserialize};
use borsh_derive::{BorshSerialize, BorshDeserialize};
use crypto_bigint::U256 as U256Lib;
use std::collections::BTreeMap;
use std::sync::Arc;
use reqwest;
use base64::{prelude::BASE64_STANDARD, Engine};

// Define a simple transaction struct that implements Tx
#[derive(BorshSerialize, BorshDeserialize)]
struct SimpleTx {
    msgs: Vec<SerializableAny>,
}

impl Tx for SimpleTx {
    fn msgs(&self) -> Vec<SerializableAny> {
        self.msgs.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), InterLiquidSdkError> {
    // Minimal in-memory StateManager for demonstration
    struct MemoryStateManager {
        map: BTreeMap<Vec<u8>, Vec<u8>>,
    }
    impl MemoryStateManager {
        fn new() -> Self {
            Self { map: BTreeMap::new() }
        }
    }
    impl StateManager for MemoryStateManager {
        fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> {
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
            &'a mut self,
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

    // Create state manager, context, and app
    let mut state_manager = MemoryStateManager::new();
    let mut ctx = SdkContext::new(
        "test-chain".to_string(),
        1,
        0,
        &mut state_manager,
    );

    // Create and register the bank module
    let bank_keeper = BankKeeper::new();
    let bank_module = Arc::new(BankModule::new(bank_keeper));
    let mut app = App::new(vec![bank_module], vec![], vec![]);

    // Create the runner
    let savedata = SaveData::default();
    let runner = Arc::new(Runner::new(app, savedata, MemoryStateManager::new()));
    
    // Start the runner in a separate task
    let runner_clone = Arc::clone(&runner);
    tokio::spawn(async move {
        if let Err(e) = runner_clone.run().await {
            eprintln!("Runner error: {}", e);
        }
    });

    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Create addresses
    let alice = Address::from([1; 32]);
    let bob = Address::from([2; 32]);

    // Set initial balance for Alice (simulate genesis)
    let mut key_buf = Vec::new();
    (alice, "usdc".to_string()).serialize(&mut key_buf).unwrap();
    let alice_balance_key = b"bank/balances/".iter().chain(key_buf.iter()).cloned().collect::<Vec<u8>>();
    let alice_initial_balance = U256::new(U256Lib::from(1000u64));
    let mut buf = vec![];
    alice_initial_balance.serialize(&mut buf).unwrap();
    state_manager.set(&alice_balance_key, &buf)?;

    // Create a MsgSend transaction
    let mut tokens = Tokens::new();
    tokens.insert("usdc".to_string(), U256::new(U256Lib::from(100u64)));
    let msg = MsgSend {
        from_address: alice,
        to_address: bob,
        tokens,
    };
    let mut msg_bytes = Vec::new();
    msg.serialize(&mut msg_bytes)?;
    let msg_any = SerializableAny::new(MsgSend::type_name().to_owned(), msg_bytes);

    // Wrap the message in a SimpleTx
    let tx = SimpleTx {
        msgs: vec![msg_any],
    };
    let mut tx_bytes = Vec::new();
    tx.serialize(&mut tx_bytes)?;

    // Send transaction via HTTP
    let client = reqwest::Client::new();
    let tx_base64 = BASE64_STANDARD.encode(&tx_bytes);
    let response = client
        .post("http://localhost:3000/tx")
        .json(&serde_json::json!({
            "tx_base64": tx_base64
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await?;
        return Err(InterLiquidSdkError::Other(anyhow::anyhow!("Transaction failed: {}", error)));
    }

    println!("Transaction sent successfully!");

    // Check balances (for demonstration)
    let bank_keeper = BankKeeper::new();
    let alice_balance = bank_keeper.get_balance(&mut ctx, &alice, "usdc")?;
    let bob_balance = bank_keeper.get_balance(&mut ctx, &bob, "usdc")?;
    println!("Alice's balance: {:?}", alice_balance);
    println!("Bob's balance: {:?}", bob_balance);

    Ok(())
} 