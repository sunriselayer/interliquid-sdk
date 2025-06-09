# Transaction Processing Flow

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

## Explanation

1. **Transaction Submission**
   - Client submits transactions to the SDK
   - SDK retrieves current state from StateManager

2. **Transaction Processing**
   - Transactions are split into chunks for parallel processing
   - Each chunk is processed independently
   - State updates are tracked per chunk

3. **ZK Proof Generation**
   - For each chunk, three types of proofs are generated:
     - State Transition Proof
     - Get Access Proof
     - Iter Access Proof
   - Proofs are generated in parallel for each chunk

4. **Proof Aggregation**
   - All chunk proofs are aggregated into a single proof
   - The aggregated proof is submitted to the rollup

5. **Block Finality**
   - Rollup verifies the proofs
   - Once verified, block is finalized
   - Client receives confirmation

This parallel processing approach significantly improves performance while maintaining security through zero-knowledge proofs.
