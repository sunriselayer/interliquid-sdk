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

fn main() -> Result<(), InterLiquidSdkError> {
    // Create a new state manager
    let state_manager = Box::new(MemoryStateManager::new());
    
    // Create a new context
    let mut ctx = SdkContext::new(
        "test-chain".to_string(),
        1,
        0,
        state_manager,
        MsgRegistry::new(),
    );

    // Initialize your application
    let mut app = MyApp::new();

    // Create addresses
    let alice = Address::from([1; 32]);
    let bob = Address::from([2; 32]);

    // Set initial balance for Alice
    let mut key_buf = Vec::new();
    (alice, "usdc".to_string()).serialize(&mut key_buf).unwrap();
    let alice_balance_key = join_keys(vec![
        &b"bank/"[..],
        &b"balances/"[..],
        key_buf.as_slice(),
    ]);
    let alice_initial_balance = U256::new(U256Lib::from(1000u64));
    let mut buf = vec![];
    alice_initial_balance.serialize(&mut buf).unwrap();
    ctx.state_manager().set(&alice_balance_key, &buf)?;

    // Create and send a message
    let mut tokens = Tokens::new();
    tokens.insert("usdc".to_string(), U256::new(U256Lib::from(100u64)));
    let msg = MsgSend {
        from: alice,
        to: bob,
        amount: tokens,
    };
    let mut msg_bytes = Vec::new();
    msg.serialize(&mut msg_bytes)?;
    app.handle_msg(&mut ctx, &msg_bytes)?;

    // Verify the balances
    let alice_balance = app.bank_keeper.get_balance(&mut ctx, &alice, "usdc")?;
    let bob_balance = app.bank_keeper.get_balance(&mut ctx, &bob, "usdc")?;

    println!("Alice's balance: {:?}", alice_balance);
    println!("Bob's balance: {:?}", bob_balance);

    Ok(())
} 