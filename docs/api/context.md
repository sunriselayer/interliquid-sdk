# Context API

The Context API provides access to the execution environment and state management for transactions and messages.

## Core Types

### Context Trait

The base trait for all context types:

```rust
pub trait Context: Send + Sync {
    fn chain_id(&self) -> &str;
    fn block_height(&self) -> u64;
    fn block_time_unix_secs(&self) -> u64;
    fn state_manager(&self) -> &dyn TracableStateManager;
    fn state_manager_mut(&mut self) -> &mut dyn TracableStateManager;
}
```

### SdkContext

The default context implementation:

```rust
pub struct SdkContext<'a, S: TracableStateManager> {
    chain_id: String,
    block_height: u64,
    block_time_unix_secs: u64,
    state_manager: &'a mut S,
}
```

## Usage Examples

### Creating a Context

```rust
let mut state_manager = MemoryStateManager::new();
let mut ctx = SdkContext::new(
    "chain-1".to_string(),
    100,
    1234567890,
    &mut state_manager,
);
```

### Accessing Context Information

```rust
// Get chain ID
let chain_id = ctx.chain_id();

// Get block height
let height = ctx.block_height();

// Get block time
let time = ctx.block_time_unix_secs();
```

### State Management

```rust
// Get immutable state manager
let state_manager = ctx.state_manager();

// Get mutable state manager
let state_manager = ctx.state_manager_mut();

// Perform state operations
state_manager.set(b"key", b"value")?;
let value = state_manager.get(b"key")?;
```

### Using Context in Modules

```rust
impl MyKeeper {
    pub fn handle_msg(
        &self,
        ctx: &mut dyn Context,
        msg: &MyMsg,
    ) -> Result<(), InterLiquidSdkError> {
        // Access chain information
        let chain_id = ctx.chain_id();
        let height = ctx.block_height();

        // Access state
        let state_manager = ctx.state_manager_mut();
        self.items.set(state_manager, &msg.address, &msg.data)?;

        Ok(())
    }
}
```

### Transactional Context

```rust
let state_manager = MemoryStateManager::new();
let mut transactional = TransactionalStateManager::new(&state_manager);

let mut ctx = SdkContext::new(
    chain_id,
    block_height,
    block_time,
    &mut transactional,
);

// Perform operations
app.execute_tx(&mut ctx, &tx_bytes)?;

// Commit changes
transactional.commit(&mut state_manager)?;
```

## Best Practices

1. **State Access**
   - Use `state_manager_mut()` when you need to modify state
   - Use `state_manager()` for read-only access
   - Handle state errors properly

2. **Context Usage**
   - Pass context as `&mut dyn Context` in module methods
   - Don't store context references
   - Use context methods to access state

3. **Error Handling**
   - Handle all potential errors from state operations
   - Use proper error types
   - Propagate errors appropriately

4. **Testing**
   - Create mock contexts for testing
   - Test state operations
   - Test error cases
   - Test context information access 