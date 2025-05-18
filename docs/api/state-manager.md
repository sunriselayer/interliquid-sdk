# State Manager API

The State Manager is a core component of the InterLiquid SDK that provides key-value storage functionality with transaction support.

## Core Traits

### StateManager

The base trait for state management:

```rust
pub trait StateManager: Send + Sync + 'static {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;
    fn iter<'a>(&'a self, key_prefix: Vec<u8>) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>;
}
```

### TracableStateManager

A trait for state managers that support transaction tracing:

```rust
pub trait TracableStateManager: Send + Sync {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, InterLiquidSdkError>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), InterLiquidSdkError>;
    fn del(&mut self, key: &[u8]) -> Result<(), InterLiquidSdkError>;
    fn iter<'a>(&'a mut self, key_prefix: Vec<u8>) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), InterLiquidSdkError>> + 'a>;
}
```

## Key Components

### TransactionalStateManager

A wrapper that provides transaction support for any StateManager implementation:

```rust
pub struct TransactionalStateManager<'s, S: StateManager> {
    pub state_manager: &'s S,
    pub logs: Vec<StateLog>,
    pub accum_logs_prev: AccumulatedLogs,
    pub accum_logs_next: AccumulatedLogs,
}
```

### RelatedState

A state manager that only allows access to a specific set of keys:

```rust
pub struct RelatedState {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
}
```

## Utility Types

### Map

A type-safe wrapper for key-value storage:

```rust
pub struct Map<K: KeyDeclaration, V: Value> {
    prefix: Vec<u8>,
    phantom: PhantomData<(K, V)>,
}
```

### Item

A type-safe wrapper for single value storage:

```rust
pub struct Item<V: Value> {
    key: Vec<u8>,
    phantom: PhantomData<V>,
}
```

## Usage Examples

### Basic State Management

```rust
let mut state_manager = MemoryStateManager::new();

// Set a value
state_manager.set(b"key", b"value")?;

// Get a value
let value = state_manager.get(b"key")?;

// Delete a value
state_manager.del(b"key")?;

// Iterate over values with a prefix
for result in state_manager.iter(b"prefix") {
    let (key, value) = result?;
    // Process key-value pair
}
```

### Transactional State Management

```rust
let state_manager = MemoryStateManager::new();
let mut transactional = TransactionalStateManager::new(&state_manager);

// Perform operations within a transaction
transactional.set(b"key", b"value")?;
transactional.get(b"key")?;

// Commit the transaction
transactional.commit(&mut state_manager)?;
```

### Type-Safe Storage

```rust
// Using Map
let balances = Map::new([BANK, BALANCES]);
balances.set(state_manager, &address, &amount)?;
let balance = balances.get(state_manager, &address)?;

// Using Item
let total_supply = Item::new([BANK, TOTAL_SUPPLY]);
total_supply.set(state_manager, &amount)?;
let supply = total_supply.get(state_manager)?;
``` 