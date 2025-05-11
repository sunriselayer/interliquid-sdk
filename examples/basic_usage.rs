use interliquid_sdk::core::{App, Context, MsgRegistry};
use interliquid_sdk::state::StateManager;
use interliquid_sdk::tx::Tx;
use interliquid_sdk::types::InterLiquidSdkError;
use borsh_derive::BorshDeserialize;

// Dummy context implementation
struct DummyContext {
    state_manager: DummyStateManager,
    msg_registry: MsgRegistry,
}

impl DummyContext {
    fn new() -> Self {
        Self {
            state_manager: DummyStateManager,
            msg_registry: MsgRegistry::new(),
        }
    }
}

impl Context for DummyContext {
    fn chain_id(&self) -> &str {
        "dummy-chain"
    }

    fn block_height(&self) -> u64 {
        1
    }

    fn block_time_seconds(&self) -> u64 {
        1234567890
    }

    fn state_manager(&mut self) -> &mut (dyn StateManager + 'static) {
        &mut self.state_manager
    }

    fn msg_registry(&self) -> &MsgRegistry {
        &self.msg_registry
    }
}

// Dummy state manager implementation
struct DummyStateManager;
impl StateManager for DummyStateManager {
    fn get(&mut self, _key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError> { Ok(None) }
    fn set(&mut self, _key: &[u8], _value: &[u8]) -> Result<(), InterLiquidSdkError> { Ok(()) }
    fn del(&mut self, _key: &[u8]) -> Result<(), InterLiquidSdkError> { Ok(()) }
    fn iter<'a>(&'a mut self, _key_prefix: Vec<u8>) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a> {
        Box::new(std::iter::empty())
    }
}

#[derive(BorshDeserialize)]
struct DummyTx;
impl Tx for DummyTx {
    fn msgs(&self) -> Vec<interliquid_sdk::types::SerializableAny> { vec![] }
}

fn main() {
    let mut app = App::<DummyContext, DummyTx>::new();
    let mut ctx = DummyContext::new();
    let tx = DummyTx;
    let result = app.execute_tx(&mut ctx, &tx);
    println!("Result: {:?}", result);
} 