---
layout: paper
title: "InterLiquid SDK Whitepaper"
permalink: /whitepaper/
---

# InterLiquid SDK

Author: KIMURA Yu ([Sunrise](https://sunriselayer.io))

## Introduction

InterLiquid SDK is a software development kit for building ZK Sovereign Rollups.
It aims to realize Web2-like User Experience and Web2-like Developer Experience for dApps.
In other words, it aims to serve a capability for Web2 apps to interact with public DeFi ecosystem.

It is suitable for building on Sunrise, but it is not limited to it.

Also if you think that the evolution of hardware acceleration of ZK proof generation is not enough, it is possible to use it for Optimistic Sovereign Rollups.

To clarify the word of Sovereign Rollup, in ZK Sovereign Rollup, validity proof of state transition are submitted to rollup itself, and in Optimistic Sovereign Rollup, fraud proof is submitted to rollup itself.

## Why Iteration Matters

Key prefix based iteration is a common pattern in Web2 development.
Only if it exists can on-chain logic be as flexible as that of NoSQL systems like Firebase Firestore.

However, it is not possible in almost all public blockchains.
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

### Cosmos SDK

Cosmos SDK's state is managed with IAVL tree.
It allows developers to iterate state in a key prefix based way because the key is not hashed.
However, IAVL has a mechanism of self-rebalancing tree, and it is not proper to prove with Zero Knowledge Proofs.
If we try to remove the mechanism of self-rebalancing (it means it is simple binary tree), it causes an attack vector to make the inclusion proof of a certain key too large because the depth of the node in the tree can be operated by an attacker.

## The challenge of InterLiquid SDK

The challenge of InterLiquid SDK is to make key prefix based iteration and ZK friendliness coexisting.
The architecture to achieve this is **Twin Radix Trees**.

Before explaining Twin Radix Trees, let's see how to prove the validity of state transition with ZKP.

### ZKP of State transition

Generally speaking, state transition function is described as follows:

$$
\text{StateNext} = f(\text{StatePrev}, \text{Txs})
$$

We need to prove the validity of the above equation with ZKP.
We assume to use zkVM.
To prove this, the state transition function is adjusted as follows:

$$
\begin{aligned}
  &\{\text{StateRootNext}, \text{Diffs}\} \\
  &= \hat{f}({\text{StateRootPrev} , \text{StateForAccess}}, \text{StateCommitPath}, \text{Txs})
\end{aligned}
$$

Because zkVM cannot access to the storage, we need to give the state to access $$ \text{StateForAccess} $$ beforehand.
It is also enough to output only the diffs $$ \text{Diffs} $$ without entire state.
To calculate the $$ \text{StateRootNext} $$, it is also needed to give the state commit path $$ \text{StateCommitPath} $$ to allow zkVM to calculate the state root.

By committing these three values $$\text{StateRootPrev}$$, $$\text{StateRootNext}$$ and $$\text{TxRoot}$$:

as the public input of the ZKP, it is possible to generate the verifiable validity proof of the state transition.

$$
\begin{aligned}
  \text{PublicInputsStf} &= \{\text{StateRootPrev}, \text{StateRootNext}, \text{TxRoot}\} \\
  \text{PrivateInputsStf} &= \left\{ \begin{aligned}
    & \text{StateForAccess} \\
    & \text{Diffs} \\
    & \text{StateCommitPath} \\
    & \text{Txs}
  \end{aligned} \right\} \\
  \text{ProofStf} &= \text{CircuitStf}(\text{PublicInputsStf}, \text{PrivateInputsStf})
\end{aligned}
$$

### Security assumptions

Here, it is said that we give the state to zkVM beforehand.
If we don't prove that the given state is correct, it is possible to make a false proof.
To prevent this, we also need to prove that the given state is correct.

Proving it only for get access (only for one designated key) is straightforward.
Merkle inclusion proof with the given state root is enough.

However, proving it for iter access (all keys which match the designated key prefix) requires a smart design.
*Twin Radix Trees* enables it while keeping the ZK friendliness.

### Twin Radix Trees

Twin Radix Trees combines two tree components:

- 8-bit-Radix Sparse Merkle Tree for state inclusion proof
- 8-bit-Radix Patricia Trie for key indexing to enable key prefix based iteration

The state root is calculated by the following equation where $$h$$ is the hash function:

$$
\text{EntireStateRoot} = h(\text{StateSparseTreeRoot} || \text{KeysPatriciaTrieRoot})
$$

### 8-bit-Radix Sparse Merkle Tree

This tree works for the state inclusion proof.

It can be used for proving get access validity in the state transition, and also for state inclusion proof of light client based interoperability protocol like IBC.

The leaf index is determined by the key hash, and the leaf value is the state hash.

```rust
pub enum OctRadSparseTreeNode {
    Leaf(OctRadSparseTreeNodeLeaf),
    Branch(OctRadSparseTreeNodeBranch),
}

pub struct OctRadSparseTreeNodeLeaf {
    pub key_hash_fragment: u8,
    pub value: Vec<u8>,
}

pub struct OctRadSparseTreeNodeBranch {
    pub key_hash_fragment: u8,
    pub child_bitmap: OctRadBitmap,
    pub children: Vec<OctRadSparseTreeNode>,
}

pub struct OctRadSparseTreePath(BTreeMap<Vec<u8>, [u8; 32]>);
```

Thanks to the property of the hash function, the attack vector of increasing the inclusion proof size of the specific key is also reduced.

Using an 8-bit radix reduces the maximum tree depth from 256 to 32.

To prove the validity of get access, it is needed to prove the inclusion of the key in the tree for $$ \{ \text{Key}_i \}_{j=1}^{k} $$.

$$
\begin{aligned}
  \text{KeysHash} &= h(\text{Key}_1 || \dots || \text{Key}_k) \\
  \text{PublicInputsGet} &= [\text{StateSparseTreeRootPrev}, \text{KeysHash}] \\
  \text{PrivateInputsGet} &= [\{\text{Key}_j\}_{j=1}^{k}, \text{ReadProofPath}] \\
  \text{ProofGet} &= \text{CircuitGet}(\text{PublicInputsGet}, \text{PrivateInputsGet})
\end{aligned}
$$

It is also needed to prove the non-inclusion of the key which was tried to be be accessed in the STF but not found. To do this, it is enough to prove the inclusion of dead end node in the tree.

### 8-bit-Radix Patricia Trie

This trie works for the key indexing.

It can be used for proving iter access validity in the state transition.

```rust
pub enum OctRadPatriciaTrieNode {
    Leaf(OctRadPatriciaTrieNodeLeaf),
    Branch(OctRadPatriciaTrieNodeBranch),
}

pub struct OctRadPatriciaTrieNodeLeaf {
    pub key_fragment: Vec<u8>,
}

pub struct OctRadPatriciaTrieNodeBranch {
    pub key_fragment: Vec<u8>,
    pub child_bitmap: OctRadBitmap,
    pub children: Vec<OctRadPatriciaTrieNode>,
}

pub struct OctRadPatriciaTriePath(BTreeMap<Vec<u8>, [u8; 32]>);
```

The node hash is calculated by the following equation where $$h$$ is the hash function:

$$
\begin{aligned}
  &\text{KeyPatriciaNodeHash} \\
  &= \begin{cases}
    \begin{aligned}
      h(&\text{KeyFragment} \\
        &|| \text{ChildNodeHash}_1 || ... || \text{ChildNodeHash}_{256})
    \end{aligned} & \text{if } \text{ChildBitmap} \neq 0\\
    \text{EmptyByte} & \text{if } \text{ChildBitmap} = 0
  \end{cases}
\end{aligned}
$$

To prove the validity of iter access, it is needed to re-construct the node hash of the designated key prefix with all iterated keys, and prove its inclusion in the tree.

$$
\begin{aligned}
  \text{KeyPrefixesHash} &= h(\text{KeyPrefix}_1 || \dots || \text{KeyPrefix}_k) \\
  \text{PublicInputsIter} &= [\text{KeysPatriciaTrieRootPrev}, \text{KeyPrefixesHash}] \\
  \text{PrivateInputsIter} &= [\{\text{KeyPrefix}_j\}_{j=1}^{k}, \text{IterProofPath}] \\
  \text{ProofIter} &= \text{CircuitIter}(\text{PublicInputsIter}, \text{PrivateInputsIter})
\end{aligned}
$$

It is straightforward to think that this proof is mathematically heavy, but there is a room for parallelization.

### Parallelization of ZKP generation

Generally speaking, the list of transactions is a part of the block.

$$
  \text{Txs} \in \text{Block}
$$

In the InterLiquid SDK, to get the accessed state which is needed to give to zkVM, it is needed to execute the transactions once outside of the zkVM.

Here, we can get the interim result of the state transition function for each transaction $$\{\text{Tx}_i\}_{i=1}^{n}$$, with emitting the accessed keys $$\{\text{Key}_{ij}\}_{j=1}^{k}$$ and $$\{\text{KeyPrefix}_{ij}\}_{j=1}^{k}$$:

$$
\begin{aligned}
  &\left\{ \text{AccumDiffs}_{1:i}, \{\text{Key}_{ij}, \text{KeyPrefix}_{ij}\}_{j=1}^{k} \right\} \\
  &= g\left(\text{StateForAccess}_i, \text{AccumDiffs}_{1:i-1}, \text{Tx}_i\right)
\end{aligned}
$$

Then we can generate the proof in parallel for each transaction with one circuit which can be regarded as a combination of $$\text{ProofStf}_i$$, $$\text{ProofGet}_i$$, and $$\text{ProofIter}_i$$:

$$
\begin{aligned}
  \text{PublicInputsTx}_i &= \{\text{TxHash}_i, \text{EntireStateRoot}, \text{AccumDiffsHashPrev}_i, \text{AccumDiffsHashNext}_i \} \\
  \text{PrivateInputsTx}_i &= \left\{ \begin{aligned}
    & \text{StateSparseTreeRoot} \\
    & \text{KeysPatriciaTrieRoot} \\
    & \text{StateForAccess}_i \\
    & \text{AccumDiffs}_{1:i} \\
    & \text{ReadProofPath}_i \\
    & \text{IterProofPath}_i \\
    & \text{Tx}_i
  \end{aligned} \right\} \\
  \text{ProofTx}_i &= \text{CircuitTx}(\text{PublicInputsTx}_i, \text{PrivateInputsTx}_i)
\end{aligned}
$$

Not only for the parallelization but also the fact that the proof of ZK-STARK requires quasi-linear time $$\mathcal{O}(n \log{n})$$ in proportion to the number of traces, it is meaningful to process transactions in parallel.

By combining these three circuits, we can omit $$\text{KeysHash}$$ and $$\text{KeyPrefixesHash}$$ in the public inputs of the ZKP because fundamentally STF $$g$$ can generate $$\{\text{Key}\}_{j=1}^k$$ and $$\{\text{KeyPrefix}\}_{j=1}^k$$ by itself.

Needless to say, $$\text{StateSparseTreeRootPrev}$$ and $$\text{KeysPatriciaTrieRootPrev}$$ which need to be verified, also can be verified by using $$\text{PublicInputsTx}_i$$ in the circuit.

Finally, we can aggregate all proofs with recursive ZKP:

$$
\begin{aligned}
  \text{PublicInputsBlock} &= \{\text{EntireStateRootPrev}, \text{EntireStateRootNext}, \text{TxRoot}\} \\
  \text{PrivateInputsBlock} &= \left\{ \begin{aligned}
    & \{\text{TxHash}_i, \text{ProofTx}_i\}_{i=1}^{n} \\
    & \text{StateSparseTreeRootPrev} \\
    & \text{KeysPatriciaTrieRootPrev} \\
    & \{\text{AccumDiffsHash}_i\}_{i=1}^{n-1} \\
    & \text{AccumDiffsFinal} \\
    & \text{StateNextCommitPath} \\
    & \text{KeysNextCommitPath}
  \end{aligned} \right\} \\
  \text{ProofBlock} &= \text{CircuitBlock}(\text{PublicInputsBlock}, \text{PrivateInputsBlock})
\end{aligned}
$$

In this zkVM program, each $$\text{TxHash}_i$$ is calculated internally and used for the public input of the internal ZKP verifications because $$\text{TxRoot}$$ should be not series hash but merkle root of all txs to support the tx inclusion proof.

The recursive ZKP structure has an systematic anchoring mechanism for the accumulated diffs:

- Each transaction proof uses $$\text{AccumDiffsHashPrev}_i$$ and $$\text{AccumDiffsHashNext}_i$$ as public inputs
- These hashes are provided from the block's private input $$\{\text{AccumDiffsHash}_i\}_{i=1}^{n-1}$$
- The final accumulated diffs $$\text{AccumDiffsFinal}$$ is used to verify the last $$\text{AccumDiffsHash}_n$$
- This creates a chain of verification: $$\text{AccumDiffsHashNext}_i = \text{AccumDiffsHashPrev}_{i+1}$$ for all $$i$$
- The chain is anchored by $$\text{AccumDiffsFinal}$$, which is used in the state commitment

This anchoring mechanism allows us to:

1. Keep the block's public inputs minimal (no need to include $$\text{AccumDiffsHash}$$)
1. Verify the correctness of the accumulated diffs chain
1. Use the final accumulated diffs for state commitment

## Another topics

### Interoperability

The reason why InterLiquid SDK is suitable for building on Sunrise is that Sunrise can support IBC connection with apps made with InterLiquid SDK by using Sunrise's ZKP based light client.
The name of InterLiquid SDK is derived from here.
Any lightweight rollups which can serve Web2 like UX and DX can access to the public DeFi liquidity through Sunrise.

### Serialization

InterLiquid SDK uses [Borsh](https://github.com/near/borsh) made by NEAR for serializing data into binary format.
[Protocol Buffers](https://github.com/protocolbuffers/protobuf) made by Google was not a bad choice for Cosmos SDK to enhance the reusability of the types and to have deterministic property of serialization,
but it is not suitable for ZKP and lightweight rollups.

### Parallelization of tx execution

By adding the accessed keys into the tx, we can realize **Semi-Optimistic Parallel Execution**.

In the conventional Optimistic Parallel Execution, if a tx state access conflicts with another tx, it is reverted and rearranged into the series execution.
Here, if tx conflicts increase, the total performance gets worse.

However, in the Semi-Optimistic Parallel Execution, we can reduce the risk of the state access conflicts by adding the accessed keys into the tx beforehand. The sequencer can plan the parallelization pipeline to minimize the risk of the state access conflicts.

Even if the state access conflicts happen, it is reverted and rearranged into the series execution, so the tx will not fail.

### Customizable tx authentication flow

To realize great User Experience, InterLiquid SDK thinks that Passkey is a key factor.
Enabling P256 ECDSA signature is one factor to make it possible to sign transactions with Passkey.

However, it is not the only factor.
For example, rotating the linked passkey public key for the certain account is more convenient to manage the account.
InterLiquid SDK allows developers to customize the tx authentication flow.

## Conclusion

InterLiquid SDK has great theoretical background and has a practical vision to realize the interoperable web with Web2 like UX and DX, to allow apps to interact with public DeFi ecosystem with the verifiable properties.
