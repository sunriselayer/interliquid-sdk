use interliquid_sdk::{
    core::{MsgRegistry, SdkContext, Context, msg_handler},
    types::{Address, InterLiquidSdkError, Tokens, U256},
    x::bank::{BankKeeper, msg_send::MsgSend},
    state::StateManager,
    utils::join_keys,
};
use crypto_bigint::U256 as U256Lib;
use std::collections::BTreeMap;
use borsh::{BorshSerialize, BorshDeserialize};
use borsh_derive::{BorshSerialize, BorshDeserialize};

// Minimal in-memory StateManager
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

// Define your message type
#[derive(BorshSerialize, BorshDeserialize)]
struct MsgSend {
    from: Address,
    to: Address,
    amount: Tokens,
}

// Define your application
struct MyApp<C: Context> {
    bank_keeper: BankKeeper<C>,
}

impl<C: Context> MyApp<C> {
    fn new() -> Self {
        Self {
            bank_keeper: BankKeeper::new(),
        }
    }
}

// Implement message handler for your app
impl<C: Context> msg_handler::MsgHandler for MyApp<C> {
    fn handle_msg(&mut self, ctx: &mut C, msg: &[u8]) -> Result<(), InterLiquidSdkError> {
        // Deserialize and handle the message
        let msg_send = MsgSend::try_from_slice(msg)?;
        self.bank_keeper.msg_send(ctx, &msg_send)
    }
}

// Runner struct to handle API server and app lifecycle
struct Runner<C: Context> {
    app: MyApp<C>,
    ctx: C,
}

impl<C: Context> Runner<C> {
    fn new(app: MyApp<C>, ctx: C) -> Self {
        Self { app, ctx }
    }

    // Start the API server
    fn start(&mut self) -> Result<(), InterLiquidSdkError> {
        // TODO: Implement API server startup
        // This would typically:
        // 1. Start an HTTP/gRPC server
        // 2. Register message handlers
        // 3. Handle incoming transactions
        // 4. Route messages to the app
        println!("API server started");
        Ok(())
    }

    // Handle incoming transaction
    fn handle_tx(&mut self, tx: &[u8]) -> Result<(), InterLiquidSdkError> {
        self.app.handle_msg(&mut self.ctx, tx)
    }
}

fn main() -> Result<(), InterLiquidSdkError> {
    // Create a new state manager
    let state_manager = Box::new(MemoryStateManager::new());
    
    // Create a new context
    let ctx = SdkContext::new(
        "test-chain".to_string(),
        1,
        0,
        state_manager,
        MsgRegistry::new(),
    );

    // Initialize your application
    let app = MyApp::new();

    // Create and start the runner
    let mut runner = Runner::new(app, ctx);
    runner.start()?;

    // Example: Create addresses
    let alice = Address::from([1; 32]);
    let bob = Address::from([2; 32]);

    // Example: Create and send a message
    let mut tokens = Tokens::new();
    tokens.insert("usdc".to_string(), U256::new(U256Lib::from(100u64)));
    let msg = MsgSend {
        from: alice,
        to: bob,
        amount: tokens,
    };
    let mut msg_bytes = Vec::new();
    msg.serialize(&mut msg_bytes)?;
    
    // Handle the transaction through the runner
    runner.handle_tx(&msg_bytes)?;

    Ok(())
} 