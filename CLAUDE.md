# InterLiquid SDK Maintenance Guide

This document contains important information for maintaining and developing the InterLiquid SDK project.

## Project Overview

InterLiquid SDK is a Zero Knowledge (ZK) Sovereign Rollup SDK that provides Web2-like UX/DX for decentralized applications. The project uses Rust and implements a Twin Radix Trees architecture for efficient state management and ZK proof generation.

## Build and Test Commands

```bash
# Build the project
cargo build

# Build with all features
cargo build --all-features

# Run tests
cargo test

# Run tests with specific features
cargo test --features "runner,runner_sp1"

# Build examples
cargo build --example basic_usage --features full

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

### Core Components

- `src/core/` - Core functionality and interfaces
  - `app.rs` - Application logic
  - `block/` - Block processing
  - `context.rs` - Execution context
  - `module.rs` - Module system
  - `tx/` - Transaction handling

- `src/state/` - State management implementation
  - `manager.rs` - State manager
  - `transactional.rs` - Transactional state updates
  - `log.rs` - State logging
  - `related.rs` - Related state handling

- `src/trie/` - Patricia Trie implementation (Twin Radix Trees)
  - `node.rs` - Trie node structure
  - `proof.rs` - Merkle proof generation
  - `db.rs` - Database interface
  - `key.rs` - Key handling
  - `nibble.rs` - Nibble operations for 4-bit radix

- `src/zkp/` - Zero Knowledge Proof components
  - `zkp_block.rs` - Block proof generation
  - `zkp_tx.rs` - Transaction proof generation
  - `zkp_tx_agg.rs` - Proof aggregation
  - `zkp_commit_keys.rs` - Key commitment proofs
  - `zkp_commit_state.rs` - State commitment proofs

- `src/runner/` - Runtime components
  - `server.rs` - HTTP server for transaction submission
  - `sequencer.rs` - Transaction sequencing
  - `prover/` - Proof generation orchestration
    - `local_sp1.rs` - SP1 prover integration
    - `orchestrator.rs` - Parallel proof generation

- `src/x/` - Extended modules (Cosmos SDK style)
  - `auth/` - Authentication and account management
  - `bank/` - Token transfer functionality
  - `crypto/` - Cryptographic operations (P256)
  - `nft/` - NFT module

### Key Technologies

1. **SP1 (Succinct Proofs 1)** - ZK proof system used for generating proofs
2. **Borsh** - Binary serialization format
3. **Axum** - Web framework for the runner server
4. **Tokio** - Async runtime for the runner

## Important Design Patterns

### 1. Twin Radix Trees Architecture

- 4-bit-Radix State Patricia Trie for state inclusion proofs
- 4-bit-Radix Keys Patricia Trie for key indexing and prefix-based iteration

### 2. Module System

All modules implement the `Module` trait:

```rust
pub trait Module {
    fn init(&mut self, ctx: &mut Context) -> Result<()>;
    fn register_msgs(&self, registry: &mut MsgRegistry);
}
```

### 3. Message Handling

Messages follow Cosmos SDK style:

- Implement serialization with Borsh
- Register handlers in modules
- Use `SerializableAny` for type erasure

### 4. State Management

- Uses transactional state updates
- Supports get/set/delete/iterate operations
- Maintains state logs for proof generation

## Feature Flags

- `no_std` - Enable no_std support with standard crypto libraries
- `no_std_sp1` - Enable no_std support with SP1-patched crypto libraries
- `runner` - Enable the runner server components
- `runner_sp1` - Enable SP1 prover integration

## Common Development Tasks

### Adding a New Module

1. Create module directory in `src/x/`
2. Implement `Module` trait
3. Define message types with Borsh serialization
4. Register message handlers
5. Add module to `src/x/mod.rs`

### Adding a New Message Type

1. Define struct with `#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]`
2. Add `TYPE_NAME` constant
3. Implement handler function
4. Register in module's `register_msgs`

### Updating State Structure

1. Modify state keys in relevant keeper
2. Update any iteration logic
3. Ensure compatibility with proof generation
4. Test state transitions thoroughly

## Testing Guidelines

1. Unit tests go in module files with `#[cfg(test)]`
2. Integration tests should test full transaction flow
3. Test both std and no_std configurations
4. Verify proof generation for state changes

## Performance Considerations

1. **Parallel Proof Generation** - Transactions are chunked for parallel processing
2. **Proof Aggregation** - Uses divide-and-conquer approach
3. **State Access** - Minimize state reads/writes in hot paths
4. **Serialization** - Borsh is used for efficiency

## Debugging Tips

1. Enable debug logging in runner with `RUST_LOG=debug`
2. Use `savedata.rs` for persisting state between runs
3. Check proof generation logs for verification failures
4. Monitor transaction processing in sequencer logs

## Security Considerations

1. All state modifications must be proven
2. Verify signature validation in auth module
3. Check bounds on numeric operations
4. Validate all user inputs
5. Ensure deterministic execution for ZK proofs

## Dependencies to Monitor

- `sp1-sdk` - Check for updates to proof system
- `p256` - Security updates for elliptic curve crypto
- `sha2`/`sha3` - Hash function implementations
- `axum`/`tokio` - Runner server dependencies

## Release Checklist

1. Run full test suite with all features
2. Verify example builds and runs
3. Check no_std compatibility
4. Test proof generation and verification
5. Update version in Cargo.toml
6. Tag release with semantic versioning

## Troubleshooting

### Common Issues

1. **Proof generation fails**
   - Check state transitions are deterministic
   - Verify all inputs are properly serialized
   - Ensure SP1 patches are correctly applied

2. **State inconsistency**
   - Check transactional boundaries
   - Verify state logs are complete
   - Ensure proper state root updates

3. **Build failures with no_std**
   - Verify feature flags are consistent
   - Check SP1-patched dependencies
   - Ensure no std imports in core code

## Contact and Resources

- Documentation: https://interliquid.sunriselayer.io/whitepaper/
- Repository: https://github.com/sunriselayer/interliquid-sdk
- Sequence diagram: docs/sequence-diagram.md
