# InterLiquid SDK Examples

This directory contains example applications built using the InterLiquid SDK. These examples demonstrate various features and use cases of the SDK.

## Available Examples

### Basic Usage
The `basic_usage.rs` example demonstrates:
- Setting up a basic SDK application
- Creating and submitting transactions
- State management
- Message handling
- Module implementation

### Bank Module
The `bank` example shows:
- Token transfer functionality
- Account management
- Balance tracking
- Transaction validation

### State Management
The `state_management` example demonstrates:
- Custom state manager implementation
- State transitions
- State iteration
- State persistence

## Running Examples

1. Navigate to the example directory:
```bash
cd examples/basic_usage
```

2. Run the example:
```bash
cargo run
```

## Example Structure

Each example follows a similar structure:
- `main.rs` - Application entry point
- `state.rs` - State management implementation
- `module.rs` - Module implementation
- `msg.rs` - Message type definitions
- `handler.rs` - Message handlers

## Key Concepts Demonstrated

1. **State Management**
   - Key-value storage
   - State transitions
   - State iteration
   - State persistence

2. **Transaction Processing**
   - Message-based transactions
   - Transaction validation
   - State updates
   - Error handling

3. **Module System**
   - Module registration
   - Message handling
   - State access
   - Context management

4. **ZK Proof Generation**
   - Proof generation
   - Proof verification
   - State commitment
   - Transaction validation

## Next Steps

After exploring the examples:
1. Read the [main documentation](../../docs/README.md)
2. Check out the [whitepaper](https://interliquid.sunriselayer.io/whitepaper/)
3. Start building your own application 