# InterLiquid SDK



InterLiquid SDK is a software development kit for building ZK Sovereign Rollups, designed to bridge Web2 and Web3 development experiences. It enables seamless integration of Web2 applications with public DeFi ecosystems while maintaining blockchain security.

## 🌟 Features

- **ZK Sovereign Rollup Support**: Build applications with zero-knowledge proof verification
- **Web2-like Developer Experience**: Familiar development patterns and tools
- **Key Prefix Based Iteration**: Efficient state management and querying
- **Parallelized ZK Proof Generation**: Optimized performance through chunked processing
- **Twin Nibble Trees Architecture**: Innovative state management system
- **Cross-Platform Compatibility**: Works with Sunrise and other platforms

## 📚 Documentation

For detailed technical documentation, please refer to our [Whitepaper](whitepaper/whitepaper.md).

## Technical Architecture

### Core Components

1. **Twin Nibble Trees**
   - 4-bit-Radix Sparse Merkle Tree for state inclusion proof
   - 4-bit-Radix Patricia Trie for key indexing
   - State root calculation: `StateRoot = h(StateSmtRoot || KeyPatriciaRoot)`

2. **State Management**
   - Efficient key-value storage with prefix-based iteration
   - ZK-friendly state transitions
   - State transition function:
     ```
     StateNext = f(StatePrev, Txs)
     ```

3. **Transaction Processing**
   - Chunked transaction processing for parallel execution
   - Parallel ZK proof generation
   - Recursive proof aggregation

### Implementation Details

#### State Transition Proof
```rust
pub struct State4RadixSmtInclusionProof {
    pub path: [Option<State4RadixSmtPath>; 63]
    pub leaf_hash: [u8; 32],
}

pub struct State4RadixSmtPath {
    pub child_index: u8,
    pub sibling_hashes: [Option<[u8; 32]>; 15],
}
```

#### Key Indexing
```rust
pub struct Key4RadixPatriciaNode {
    pub key_fragment: Vec<u8>,
    pub nibble_front: bool,
    pub nibble_back: bool,
    pub children: [Option<Key4RadixPatriciaNode>; 16],
}
```

### Transaction Processing Flow

```mermaid
sequenceDiagram
    participant Client
    participant SDK
    participant StateManager
    participant ZKProver
    participant Rollup

    Client->>SDK: Submit Transaction(s)
    SDK->>StateManager: Get Current State
    StateManager-->>SDK: Return State
    
    par Transaction Processing
        SDK->>SDK: Split into Chunks
        loop For Each Chunk
            SDK->>StateManager: Process Chunk
            StateManager->>StateManager: Update State
            StateManager-->>SDK: Return Updated State
        end
    end

    par ZK Proof Generation
        loop For Each Chunk
            SDK->>ZKProver: Generate Proof
            ZKProver->>ZKProver: Create State Transition Proof
            ZKProver->>ZKProver: Create Get Access Proof
            ZKProver->>ZKProver: Create Iter Access Proof
            ZKProver-->>SDK: Return Combined Proof
        end
    end

    SDK->>SDK: Aggregate Proofs
    SDK->>Rollup: Submit Block with Proofs
    Rollup->>Rollup: Verify Proofs
    Rollup-->>Client: Confirm Block Finality
```

### Code Structure

```
src/
├── core/         # Core functionality and interfaces
│   ├── state.rs  # State management
│   └── tx.rs     # Transaction processing
├── state/        # State implementation
│   ├── smt.rs    # Sparse Merkle Tree
│   └── patricia.rs # Patricia Trie
├── tx/           # Transaction processing
│   ├── chunk.rs  # Chunk management
│   └── proof.rs  # Proof generation
├── types/        # Common types
├── utils/        # Utility functions
└── x/            # Extended functionality
```

### ZK Proof Generation

1. **State Transition Proof**
   ```rust
   PublicInputsStf = [StateRootPrev, StateRootNext, TxRoot]
   PrivateInputsStf = [
       StatePrev^{get, iter},
       StateNext^{set, del},
       StateNodeHashes^{NoAccess},
       Txs
   ]
   ```

2. **Get Access Proof**
   ```rust
   PublicInputsGet = [StateSmtRootPrev, KeysHash]
   PrivateInputsGet = [{Key_j, StateSmtInclusionProof_j}_{j=1}^k]
   ```

3. **Iter Access Proof**
   ```rust
   PublicInputsIter = [KeyPatriciaRootPrev, KeyPrefixesHash]
   PrivateInputsIter = [{KeyPrefix_j, KeyPatriciaNodes_j}_{j=1}^k]
   ```

### Parallel Processing

The SDK implements parallel processing through chunking:

```rust
// Chunk processing
{StateRootNext_i, StateNext_i^{set, del}, {Key_ij, KeyPrefix_ij}_{j=1}^k} 
= g({StateRootPrev, StatePrev_i^{get, iter}}, StateNodeHashes_i^{NoAccess}, TxsChunk_i)

// Proof aggregation
PublicInputsAgg = [StateRootPrev_1, StateRootNext_n, TxRoot]
PrivateInputsAgg = [{StateRootPrev_i}_{i=2}^n, {StateRootNext_i}_{i=1}^{n-1}, {ProofChunk_i}_{i=1}^n]
```

### Security Measures

1. **Zero-Knowledge Proofs**
   - State transition verification
   - Get access validation
   - Iter access validation

2. **State Management**
   - Merkle tree-based state validation
   - Patricia trie for key indexing
   - 4-bit radix optimization

3. **Parallel Processing**
   - Chunked transaction processing
   - Parallel proof generation
   - Recursive proof aggregation

### Usage Example

Here's a simple example showing how to use the SDK for token transfers:

```rust
// Add this import for derive macros:
use borsh_derive::{BorshSerialize, BorshDeserialize};

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

// Dispatch the transaction through the app (Cosmos SDK style)
app.execute_tx(&mut ctx, &tx_bytes)?;

// Note: MsgSend and SaveData are private modules in the SDK and will cause errors unless made public.
```

### Implementation Details

- **SimpleTx**: A struct that implements the `Tx` trait, allowing you to wrap messages and process them.
- **App::execute_tx**: Used to dispatch transactions through the app, similar to the Cosmos SDK style.
- **MsgSend and SaveData**: These are private and must be used via public APIs. If you see errors, check if the SDK has made them public.

For a complete working example, see [examples/basic_usage.rs](examples/basic_usage.rs).

For more technical details, refer to the [Whitepaper](whitepaper/whitepaper.md).
