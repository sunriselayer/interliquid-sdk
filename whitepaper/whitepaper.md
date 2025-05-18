---
layout: paper
title: "InterLiquid SDK Whitepaper"
permalink: /whitepaper/
---

# InterLiquid SDK

Author: KIMURA Yu ([Sunrise](https://sunriselayer.io))

## Table of Contents
- [Introduction](#introduction)
- [Why Iteration Matters](#why-iteration-matters)
  - [Ethereum](#ethereum)
  - [Solana](#solana)
  - [Cosmos SDK](#cosmos-sdk)
  - [Polkadot SDK (Substrate)](#polkadot-sdk-substrate)
- [Technical: The Challenge of InterLiquid SDK](#technical-the-challenge-of-interliquid-sdk)
  - [ZKP of State Transition](#zkp-of-state-transition)
  - [Security Assumptions](#security-assumptions)
  - [Twin Radix Trees](#twin-radix-trees)
    - [4-bit-Radix State Patricia Trie](#4-bit-radix-state-patricia-trie)
    - [4-bit-Radix Keys Patricia Trie](#4-bit-radix-keys-patricia-trie)
  - [Parallelization of ZKP Generation](#parallelization-of-zkp-generation)
- [Glossary](#glossary)
- [Further Reading](#further-reading)

## Introduction

> **Key Concept**: InterLiquid SDK enables Web2-like experiences for dApps while maintaining blockchain security.

InterLiquid SDK is a software development kit for building ZK Sovereign Rollups.
It aims to realize Web2-like User Experience and Web2-like Developer Experience for dApps.
In other words, it aims to serve a capability for non-Web3 applications to interact with public DeFi ecosystem.
The term "apps" encompasses not only web applications but also application logic of financial systems.

It is suitable for building on Sunrise, but it is not limited to it.

Also if you think that the evolution of hardware acceleration of ZK proof generation is not enough, it is possible to use it for Optimistic Sovereign Rollups.

To clarify the word of Sovereign Rollup, in ZK Sovereign Rollup, validity proof of state transition are submitted to rollup itself, and in Optimistic Sovereign Rollup, fraud proof is submitted to rollup itself.

**Key Takeaways**:
- Enables Web2-like experiences for dApps
- Supports both ZK and Optimistic Sovereign Rollups
- Suitable for Sunrise but not limited to it
- Bridges Web2 applications with DeFi ecosystem

## Why Iteration Matters

Key prefix based iteration is a common pattern in Web2 development.
Only if it exists can on-chain logic be as flexible as that of NoSQL systems like Firebase Firestore.

For example, imagine iterating all vote info in a governance contract.
The key should be like `gov/votes/{proposal_id}/{voter_address}`.
Here, it is very useful to iterate all vote info in a certain proposal by designating a key prefix like `gov/votes/{proposal_id}`.

However, it is not possible in almost all public blockchains.
Even fundamental features like governance vote iteration are not supported.
It is one of the most painful problems for Developer Experience.

Of course Indexing services should be utilized for proper purposes like search and analytics.
However, enforcing developers to pay for subscription of Indexing services only for succinct iteration on frontend side,
while serving the query function which is available only if the user could calculate the slot id of the state like EVM,
is very absurd.

For succinct purposes, key prefix based iteration should be supported by the blockchain itself, moreover on-chain.

### Ethereum

Ethereum's state is managed in a Patricia Merkle Trie (PMT) for each address including smart contract address and EOA address, and further internal state of each smart contract is stored in a Patricia Merkle Trie inside the address state.
Because PMT hashes each key, it disallow developers from iterating state in a key prefix based way.

### Solana

Solana's state is stored respectively with each account.
Thanks to its design, Solana succeeded to parallelize state transition for each account.
However, it is not possible to iterate state in a key prefix based way.
By making each account like B-tree node, developers can realize the structure of B-tree artificially, but it requires paying Solana account rent and the Developer Experience is terrible.

Not only Solana, but also other chains like scalable monolithic blockchains (e.g. Sui) have the same structure to make it scalable.
Given that next-generation financial infrastructure demands key prefix based iterable state, the cluster of interoperable rollups provide better solutions than monolithic chains.

### Cosmos SDK

Cosmos SDK's state is managed with IAVL tree.
It allows developers to iterate state in a key prefix based way because the key is not hashed.
However, IAVL has a mechanism of self-rebalancing tree, and it is not proper to prove with Zero Knowledge Proofs.
If we try to remove the mechanism of self-rebalancing (it means it is simple binary tree), it causes an attack vector to make the inclusion proof of a certain key too large because the depth of the node in the tree can be operated by an attacker.

### Polkadot SDK (Substrate)

Polkadot SDK's state is managed with Patricia Merkle Trie.
It also supports key prefix based iteration with the unique way.
In the Polkadot SDK, the key is separated into the prefix parts, and each prefix part is hashed.

For example, if the application logic want to access the state with the key  `gov/votes/{proposal_id}/{voter_address}`, it is hashed with each prefix part like `gov/votes/{hash(proposal_id)}/{proposal_id}/{hash(voter_address)}/{voter_address}`.

This design allows the application logic to iterate the state in a key prefix based way.
However, there are two limitations.

Firstly, the key is hashed, so it is not able to iterate with the sorted order.
If we iterate all the state and sort them, the meaning of the iteration is lost because it can be achieved by storing the entire array in one key.

Secondly, the key is hashed, so the proof of the inclusion of the iterated key is not ZK friendly.
It is not possible to prove the completeness of the iterated key as we do in InterLiquid SDK.

## Techincal: The challenge of InterLiquid SDK

> **Key Concept**: Twin Radix Trees enable key prefix based iteration while maintaining ZK friendliness.

The challenge of InterLiquid SDK is to make key prefix based iteration and ZK friendliness coexisting.
The architecture to achieve this is **Twin Radix Trees**.

Before explaining Twin Radix Trees, let's see how to prove the validity of state transition with ZKP.

### ZKP of State transition

```mermaid
graph LR
    A[StatePrev] --> C[State Transition Function]
    B[Transactions] --> C
    C --> D[StateNext]
```

Generally speaking, state transition function is described as follows:

$$
\text{StateNext} = f(\text{Txs}, \text{StatePrev})
$$

We need to prove the validity of the above equation with ZKP.
We assume to use zkVM.
To prove this, the state transition function is adjusted as follows:

$$
\begin{aligned}
  &\{\text{StateRootNext}, \text{Diffs}\} \\
  &= \hat{f}(\text{Txs}, \text{StateRootPrev} , \text{StateForAccess}, \text{StateCommitPath})
\end{aligned}
$$

> **Note**: The state transition proof requires three key components:
> 1. State to access (StateForAccess)
> 2. State diffs (Diffs)
> 3. State commit path (StateCommitPath)

Because zkVMs cannot access the storage directly, we need to give the state to access $$ \text{StateForAccess} $$ beforehand.
It is also enough to output only the diffs $$ \text{Diffs} $$ without entire state.
To calculate the $$ \text{StateRootNext} $$, it is also needed to give the state commit path $$ \text{StateCommitPath} $$ to allow zkVM to calculate the state root.

By committing these three values $$\text{StateRootPrev}$$, $$\text{StateRootNext}$$ and $$\text{TxsRoot}$$ as the public input of the ZKP, it is possible to generate the verifiable validity proof of the state transition.

$$
\begin{aligned}
  \text{WitnessStf} &= \left\{ \begin{aligned}
    & \text{Txs} \\
    & \text{StateForAccess} \\
    & \text{Diffs} \\
    & \text{StateCommitPath}
  \end{aligned} \right\} \\
  \text{PubInputsStf} &= \left\{ \begin{aligned}
    & \text{TxsRoot}(\text{Txs}) \\
    & \text{StateRootPrev} \\
    & (\text{StateForAccess}, \text{StateCommitPath}) \\
    & \text{StateRootNext} \\
    & (\text{StateForAccess}, \text{Diffs}, \text{StateCommitPath})
  \end{aligned} \right\} \\
  \text{ProofStf} &= \text{CircuitStf}(\text{WitnessStf}, \text{PubInputsStf})
\end{aligned}
$$

Hereafter the relation between $$\text{ProofXXX}$$, $$\text{WitnessXXX}$$ and $$\text{PubInputsXXX}$$ is omitted.

### Security assumptions

Here, it is said that we give the state to zkVM beforehand.
If we don't prove that the given state is correct, it is possible to make a false proof.
To prevent this, we also need to prove that the given state is correct.

Proving inclusion for get-access (i.e., a single designated key) is straightforward.
Merkle inclusion proof with the given state root is enough.

However, proving it for iter access (all keys which match the designated key prefix) requires a smart design.
*Twin Radix Trees* enables it while keeping the ZK friendliness.

### Twin Radix Trees

```mermaid
graph TD
    A[EntireRoot] --> B[StateRoot]
    A --> C[KeysRoot]
    B --> D[4-bit-Radix State Patricia Trie]
    C --> E[4-bit-Radix Keys Patricia Trie]
    D --> F[State Inclusion Proof]
    E --> G[Key Indexing]
```

Twin Radix Trees combines two tree components:

1. **4-bit-Radix State Patricia Trie**
   - Purpose: State inclusion proof
   - Use case: Get access validity in state transition
   - Feature: Light client based interoperability protocol support

2. **4-bit-Radix Keys Patricia Trie**
   - Purpose: Key indexing
   - Use case: Iter access validity in state transition
   - Feature: Key prefix based iteration

The state root is calculated by the following equation where $$h$$ is the hash function:

$$
\text{EntireRoot} = h(\text{StateRoot} || \text{KeysRoot})
$$

**Key Takeaways**:
- Twin Radix Trees enable both state inclusion proof and key indexing
- Maintains ZK friendliness while supporting key prefix iteration
- Enables efficient state transition validation
- Supports light client interoperability

### 4-bit-Radix State Patricia Trie

```mermaid
graph TD
    A[State Patricia Trie] --> B[Leaf Index]
    A --> C[Leaf Value]
    B --> D[Key Hash]
    C --> E[State Hash]
    F[Inclusion Proof] --> G[Get Access Validity]
    F --> H[Light Client Interoperability]
```

This tree works for the state inclusion proof.

It can be used for proving get access validity in the state transition, and also for state inclusion proof of light client based interoperability protocol like IBC.

The leaf index is determined by the key hash, and the leaf value is the state hash.

Thanks to the property of the hash function, the attack vector of increasing the inclusion proof size of the specific key is also reduced.

> **Security Note**: The hash function property helps mitigate attacks that could increase inclusion proof size.

To prove the validity of get access, it is needed to prove the inclusion of the key in the tree for $$ \text{ReadKVPairs} $$.

$$
\begin{aligned}
  \text{WitnessRead} &= \left\{ \begin{aligned}
    & \text{StateForAccess} \\
    & \text{ReadKVPairs} \\
    & \text{ReadProofPath}
  \end{aligned} \right\} \\
  \text{PubInputsRead} &= \left\{ \begin{aligned}
    & \text{StateRootPrev} \\
    & (\text{StateForAccess}, \text{ReadKVPairs}, \text{ReadProofPath}) \\
    & \text{ReadKVPairsHash}(\text{ReadKVPairs})
  \end{aligned} \right\}
\end{aligned}
$$

**Key Takeaways**:
- Uses key hash for leaf index determination
- Provides state inclusion proof
- Supports get access validity verification
- Enables light client interoperability

### 4-bit-Radix Keys Patricia Trie

```mermaid
graph TD
    A[Keys Patricia Trie] --> B[Node Hash Calculation]
    B --> C[Key Fragment]
    B --> D[Child Node Hashes]
    E[Iteration] --> F[Key Prefix Matching]
    F --> G[State Transition Validity]
```

This trie works for the key indexing.

It can be used for proving iter access validity in the state transition.

The node hash is calculated by the following equation where $$h$$ is the hash function:

$$
\begin{aligned}
  &\text{KeysNodeHash} \\
  &= \begin{cases}
    \begin{aligned}
      h(&\text{KeyFragment} \\
        &|| \text{ChildNodeHash}_1 || ... || \text{ChildNodeHash}_{256})
    \end{aligned} & \text{if } \text{ChildBitmap} \neq 0\\
    \text{EmptyByte} & \text{if } \text{ChildBitmap} = 0
  \end{cases}
\end{aligned}
$$

> **Implementation Note**: The node hash calculation supports efficient key prefix iteration while maintaining ZK friendliness.

To prove the validity of iter access, it is needed to re-construct the node hash of the designated key prefix with all iterated keys, and prove its inclusion in the tree.

$$
\begin{aligned}
  \text{WitnessIter} &= \left\{ \begin{aligned}
    & \text{StateForAccess} \\
    & \text{IterKVPairs} \\
    & \text{IterProofPath}
  \end{aligned} \right\} \\
  \text{PubInputsIter} &= \left\{ \begin{aligned}
    & \text{KeysRootPrev} \\
    & (\text{StateForAccess}, \text{IterKVPairs}, \text{IterProofPath}) \\
    & \text{IterKVPairsHash}(\text{IterKVPairs})
  \end{aligned} \right\}
\end{aligned}
$$

**Key Takeaways**:
- Enables key prefix based iteration
- Supports efficient node hash calculation
- Maintains ZK friendliness
- Provides iter access validity proof

### Parallelization of ZKP Generation

```mermaid
graph TD
    A[Block] --> B[Transactions]
    B --> C[State Transition]
    C --> D[Interim Results]
    D --> E[ReadKVPairs]
    D --> F[IterKVPairs]
    E --> G[Parallel ZKP Generation]
    F --> G
```

Generally speaking, the list of transactions is a part of the block.

$$
  \text{Txs} \in \text{Block}
$$

In the InterLiquid SDK, to get the accessed state which is needed to give to zkVM, it is needed to execute the transactions once outside of the zkVM.

> **Performance Note**: Parallelization of ZKP generation significantly improves processing efficiency.

Here, we can get the interim result of the state transition function for each transaction $$\{\text{Tx}_i\}_{i=1}^{n}$$, with emitting the accessed key value pairs $$\text{ReadKVPairs}_i$$ and $$\text{IterKVPairs}_i$$:

$$
\begin{aligned}
  &\left\{ \text{AccumDiffsNext}_i, \text{ReadKVPairs}_i, \text{IterKVPairs}_i \right\} \\
  &= \hat{f}_i(\text{Tx}_i, \text{AccumDiffsPrev}_i, \text{StateForAccess}_i, \text{StateCommitPath}_i)
\end{aligned}
$$

**Key Takeaways**:
- Supports parallel ZKP generation
- Enables efficient transaction processing
- Maintains state consistency
- Optimizes performance through parallelization

## Performances of experimental implementation

Because InterLiquid SDK makes the pipeline of proof generation, the proving time is dominated by these proof generation times:

- n-th transaction proof generation and recursive aggregation for $$\log_2{n}$$ times
- state commitment proof generation
- keys commitment proof generation

Here, we prepared the experimental implementation of state commitment proof generation which is one of the most heavy part in these processes, with our own implementation of 4-bit-Radix Patricia Merkle Trie.

For the experimental implementation, we assumed a Patricia Merkle Trie with 1000 elements condensed within the top 3 levels from the root. This is reasonable because branch nodes closer to the root tend to have denser children, and the performance of computing the Merkle root is dominated by the number of elements near the root.

This is the table of the number of SP1 zkVM program cycles for the number of keys to commit state change.

|Keys to commit state change|Elements in the trie|SP1 zkVM program cycles|
|---|---|---|
|1|1,000|5,367,109|
|2|1,000|5,554,284|
|3|1,000|5,753,669|
|4|1,000|5,787,398|
|5|1,000|5,791,943|
|6|1,000|5,767,738|
|7|1,000|5,759,194|
|8|1,000|5,787,806|
|9|1,000|5,783,981|
|96|1,000|7,122,345|
|192|1,000|8,589,647|
|384|1,000|9,445,028|

The interesting thing is that the number of program cycle is decreasing in 6 commit keys in comparison with 5 commit keys. It can be attributed to the efficient implementation of our trie which reduces the number of zkVM program cycle if the many hashes can be shared.

In typical tx with transferring one token from one account to another, it is needed to commit 3 keys.

- The nonce of the account for the signature
- The balance of the sender account
- The balance of the receiver account

From the data above, the program cycle of the block with 32 txs is about 7-8M, 64 txs is about 8-9M, and 128txs is about 9-10M.

In the circuit of proving the state root transition, we need to calculate two merkle roots of previous and next state. It means that at least two times of program cycles are needed.
Then the total number of the entire program cycles would be under 20,000,000.

The table below shows the proving time experiment by SP1 team. Note that the data is of before SP1 Turbo.

|Programs|SP1 zkVM program cycles|Proving time|
|---|---|---|
|Tendermint Light Client|29,348,142|270 secs|
|zkEVM block with Reth|199,644,261|1417 secs|

SP1 zkVM showed that due to its architecture, the proving time is almost linear in proportion to the number of cycles. Here, we can estimate the proving time of the block of InterLiquid SDK with 32 txs to be under 3 minutes.
It can be further improved by SP1 Turbo.

By SP1 team and Polygon team, the proving time for the same block of Ethereum is published.

|zkEVM implementation|Block number|Transactions|Gas|Proving time|Proving cost per tx|
|---|---|---|---|---|---|
|SP1 Reth (before SP1 Turbo)|17106222|105|10,781,405|41.8 mins|$0.015|
|Polygon type 1|17106222|105|10,781,405|44.2 mins|$0.002|
|SP1 Reth (SP1 Turbo)|20600000|NaN|about 15M|23.6 secs|NaN|

The proving cost per transaction of Polygon type 1 is better than SP1 Reth before SP1 Turbo, which can be attributed to Polygon type 1's use of a circuit DSL for zk-SNARKs.

InterLiquid SDK's proving time per transaction will be significantly better than both Polygon type 1 and SP1 Reth, as it enables parallel proof generation through optimized zkVM program cycles.

|Block proof implementation|ZKP type|Customizability|Proof generation pipeline|
|---|---|---|---|
|InterLiquid SDK|mainly SP1 zk-STARKs|✅|✅|
|SP1 Reth|SP1 zk-STARKs|✅|❌|
|Polygon type 1|Plonky zk-SNARKs|❌|❌|

The comparison can be summarized in the table above. Polygon type 1 is the best in terms of proving cost per tx, but its DSL based circuit is not suitable for customizing the blockchain in the native level.

## Another topics

### Interoperability

The reason why InterLiquid SDK is suitable for building on Sunrise is that Sunrise can support IBC connection with apps made with InterLiquid SDK by using Sunrise's ZKP based light client.
The name of InterLiquid SDK is derived from here.
Any lightweight rollups which can serve Web2 like UX and DX can access to the public DeFi liquidity through Sunrise.

### Serialization

InterLiquid SDK uses [Borsh](https://github.com/near/borsh) made by NEAR for serializing data into binary format.
[Protocol Buffers](https://github.com/protocolbuffers/protobuf) made by Google was not a bad choice for Cosmos SDK to enhance the reusability of the types and to have deterministic property of serialization,
but it is not suitable for ZKP and lightweight rollups.

### Customizable tx authentication flow

To realize great User Experience, InterLiquid SDK thinks that Passkey is a key factor.
Enabling P256 ECDSA signature is one factor to make it possible to sign transactions with Passkey.

However, it is not the only factor.
For example, rotating the linked passkey public key for the certain account is more convenient to manage the account.
InterLiquid SDK allows developers to customize the tx authentication flow.

## Conclusion

The innovative architecture Twin Radix Trees enables key prefix based iteration while maintaining ZK friendliness, which is a significant advancement in blockchain state management. The parallel processing capabilities and divide-and-conquer approach for proof aggregation ensure efficient performance even with complex state transitions.

With its customizable transaction authentication flow and seamless integration with Sunrise, InterLiquid SDK provides a robust foundation for building next-generation financial applications that combine the best of Web2 and Web3 technologies.

InterLiquid SDK has great theoretical background and has a practical vision to realize the interoperable financial system with Web2 like UX and DX, to allow apps to interact with public DeFi ecosystem with the financial enterprise grade verifiability.

## References

- <https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/>
- <https://blog.succinct.xyz/sp1-testnet/>
- <https://www.succinct.xyz/blog-articles/introducing-sp1-reth-a-performant-type-1-zkevm-built-with-sp1>
- <https://blog.succinct.xyz/sp1-turbo/>
- <https://docs.polygon.technology/cdk/architecture/type-1-prover/testing-and-proving-costs/#proving-costs>
- <https://borsh.io/>
- <https://protobuf.dev/>

## Glossary

- **ZK Sovereign Rollup**: A type of rollup where validity proofs of state transitions are submitted to the rollup itself
- **Optimistic Sovereign Rollup**: A type of rollup where fraud proofs are submitted to the rollup itself
- **Patricia Merkle Trie (PMT)**: A data structure used for storing and verifying state in blockchains
- **IAVL Tree**: A self-balancing binary search tree used in Cosmos SDK
- **Twin Radix Trees**: A combination of two tree components for state inclusion proof and key indexing
- **zkVM**: Zero Knowledge Virtual Machine

## Further Reading

1. [Sunrise Layer Documentation](https://sunriselayer.io/docs)
2. [Zero Knowledge Proofs in Blockchain](https://ethereum.org/en/zero-knowledge-proofs/)
3. [Understanding Sovereign Rollups](https://ethereum.org/en/rollups/)
4. [Patricia Merkle Trie Explained](https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/)
