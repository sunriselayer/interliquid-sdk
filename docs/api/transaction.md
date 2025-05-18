# Transaction API

The Transaction API provides the core functionality for handling transactions in the InterLiquid SDK.

## Core Types

### Tx Trait

The base trait for all transaction types:

```rust
pub trait Tx: Send + Sync {
    fn msgs(&self) -> &[Msg];
}
```

### Message Types

```rust
pub struct Msg {
    pub type_: String,
    pub value: Vec<u8>,
}
```

## Transaction Handlers

### Ante Handlers

Handlers that run before transaction execution:

```rust
pub trait TxAnteHandler<TX: Tx>: Send + Sync {
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}
```

### Post Handlers

Handlers that run after transaction execution:

```rust
pub trait TxPostHandler<TX: Tx>: Send + Sync {
    fn handle(
        &self,
        ctx: &mut dyn Context,
        msg_registry: &MsgRegistry,
        tx: &TX,
    ) -> Result<(), InterLiquidSdkError>;
}
```

## Message Handling

### Message Registry

```rust
pub struct MsgRegistry {
    // Internal registry of message types
}
```

### Message Handler Registry

```rust
pub struct MsgHandlerRegistry {
    handlers: BTreeMap<
        &'static str,
        Box<dyn Fn(&mut dyn Context, &dyn Any) -> Result<(), InterLiquidSdkError> + Send + Sync>,
    >,
}
```

## Application

### App

The main application type that handles transaction execution:

```rust
pub struct App<TX: Tx> {
    tx_ante_handlers: Vec<Box<dyn TxAnteHandler<TX>>>,
    tx_post_handlers: Vec<Box<dyn TxPostHandler<TX>>>,
    msg_registry: MsgRegistry,
    msg_handler_registry: MsgHandlerRegistry,
    phantom: PhantomData<TX>,
}
```

## Usage Examples

### Creating a Transaction

```rust
use auth::StdTx;

// Create a transaction
let tx = StdTx {
    msgs: vec![
        Msg {
            type_: "send".to_string(),
            value: msg_send.try_to_vec()?,
        }
    ],
};
```

### Registering Message Handlers

```rust
let mut msg_registry = MsgRegistry::new();
let mut msg_handler_registry = MsgHandlerRegistry::new();

// Register a message type
msg_registry.register::<MsgSend>();

// Register a message handler
msg_handler_registry.register::<MsgSend>(Box::new(|ctx, msg| {
    // Handle the message
    Ok(())
}));
```

### Creating an Application

```rust
let app = App::new(
    vec![Arc::new(module)],
    vec![Box::new(ante_handler)],
    vec![Box::new(post_handler)],
);
```

### Executing a Transaction

```rust
let mut ctx = SdkContext::new(
    chain_id,
    block_height,
    block_time,
    &mut state_manager,
);

app.execute_tx(&mut ctx, &tx_bytes)?;
```

## Transaction Flow

1. Transaction bytes are received
2. Ante handlers are executed
3. Messages are processed in sequence
4. Post handlers are executed
5. State changes are committed
6. ZK proof is generated