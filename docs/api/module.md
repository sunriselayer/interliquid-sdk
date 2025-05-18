# Module API

The Module API provides a way to extend the InterLiquid SDK with custom functionality through a modular architecture.

## Core Types

### Module Trait

The base trait for all modules:

```rust
pub trait Module: Send + Sync {
    fn register_msgs(
        self: Arc<Self>,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    );
}
```

## Built-in Modules

### Bank Module

The bank module provides token transfer functionality:

```rust
pub struct BankModule {
    keeper: Arc<BankKeeper>,
}

impl BankModule {
    pub fn new(keeper: Arc<BankKeeper>) -> Self {
        Self { keeper }
    }

    pub fn keeper(&self) -> &BankKeeper {
        &self.keeper
    }
}
```

### Bank Keeper

The bank keeper handles token operations:

```rust
pub trait BankKeeperI: Send {
    fn get_balance(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        denom: &str,
    ) -> Result<Option<U256>, InterLiquidSdkError>;

    fn get_all_balances(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
    ) -> Result<Tokens, InterLiquidSdkError>;

    fn send(
        &self,
        ctx: &mut dyn Context,
        from: &Address,
        to: &Address,
        tokens: &Tokens,
    ) -> Result<(), InterLiquidSdkError>;
}
```

## Creating Custom Modules

### Example Module

```rust
pub struct MyModule {
    keeper: Arc<MyKeeper>,
}

impl MyModule {
    pub fn new(keeper: Arc<MyKeeper>) -> Self {
        Self { keeper }
    }
}

impl Module for MyModule {
    fn register_msgs(
        self: Arc<Self>,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
        // Register message types
        msg_registry.register::<MyMsg>();

        // Register message handlers
        let module = self.clone();
        msg_handler_registry.register::<MyMsg>(Box::new(move |ctx, msg| {
            module.keeper.handle_msg(ctx, msg)
        }));
    }
}
```

### Example Keeper

```rust
pub struct MyKeeper {
    // State storage
    items: Map<Address, MyData>,
}

impl MyKeeper {
    pub fn new() -> Self {
        Self {
            items: Map::new([MY_MODULE, ITEMS]),
        }
    }

    pub fn handle_msg(
        &self,
        ctx: &mut dyn Context,
        msg: &MyMsg,
    ) -> Result<(), InterLiquidSdkError> {
        match msg {
            MyMsg::Create { data } => self.create(ctx, data),
            MyMsg::Update { data } => self.update(ctx, data),
            MyMsg::Delete { address } => self.delete(ctx, address),
        }
    }

    fn create(
        &self,
        ctx: &mut dyn Context,
        data: &MyData,
    ) -> Result<(), InterLiquidSdkError> {
        // Implementation
        Ok(())
    }

    // Other methods...
}
```

## Module Integration

### Adding Modules to Application

```rust
let bank_keeper = Arc::new(BankKeeper::new());
let bank_module = Arc::new(BankModule::new(bank_keeper));

let my_keeper = Arc::new(MyKeeper::new());
let my_module = Arc::new(MyModule::new(my_keeper));

let app = App::new(
    vec![bank_module, my_module],
    vec![Box::new(ante_handler)],
    vec![Box::new(post_handler)],
);
```

## Best Practices

1. **State Management**
   - Use `Map` and `Item` for type-safe state access
   - Implement proper key prefixes
   - Handle state transitions carefully

2. **Message Handling**
   - Define clear message types
   - Implement proper validation
   - Handle errors gracefully

3. **Module Design**
   - Keep modules focused and single-purpose
   - Use keepers for business logic
   - Implement proper access control

4. **Testing**
   - Write unit tests for keepers
   - Test message handling
   - Test state transitions
   - Test error cases 