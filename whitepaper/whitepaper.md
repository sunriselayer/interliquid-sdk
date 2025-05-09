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
Only if it exists, on chain logics can be very flexible as well as NoSQL like Firebase Firestore can do.

However, it is not possible in almost all public blockchains.
It is one of the most painful problems for Developer Experience.

Of course Indexing services should be utilized for proper purposes like search and analytics.
However, enforcing developers to pay for subscription of Indexing services only for succinct iteration on frontend side,
while serving the query function which is available only if the user could calculate the slot id of the state like EVM,
is very absurd.

For succinct purposes, key prefix based iteration should be supported by the blockchain itself, moreover on-chain.

### Ethereum

Ethereum's state is managed in a Patricia Merkle Trie (PMT) respectively with each address including smart contract address and EOA address, and further internal state of each smart contract is stored in a Patricia Merkle Trie inside the address state.
Because PMT hashes each key, it disallow developers to iterate state in a key prefix based way.

### Solana

Solana's state is stored respectively with each account.
Thanks to its design, Solana succeeded to parallelize state transition for each account.
However, it is not possible to iterate state in a key prefix based way.
By making each account like B-tree node, developers can realize the structure of B-tree artificially, but it costs rents of Solana account and the Developer Experience is terrible.

### Cosmos SDK

Cosmos SDK's state is managed with IAVL tree.
It allows developers to iterate state in a key prefix based way because the key is not hashed.
However, IAVL has a mechanism of self-rebalancing tree, and it is not proper to prove with Zero Knowledge Proofs.
If we try to remove the mechanism of self-rebalancing (it means it is simple binary tree), it causes an attack vector to make the inclusion proof of a certain key too large because the depth of the node in the tree can be operated by an attacker.

## The challenge of InterLiquid SDK

The challenge of InterLiquid SDK is to make key prefix based iteration and ZK friendliness coexisting.
The architecture to achieve this is **Twin Nibble Trees**.

Before explaining Twin Nibble Trees, let's see how to prove the validity of state transition with ZKP.

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
&\{\text{StateRootNext}, \text{StateNext}^{\text{set, del}}\} \\
&= \hat{f}({\text{StateRootPrev} , \text{StatePrev}^{\text{get, iter}}}, \text{StateNodeHashes}^{\text{NoAccess}}, \text{Txs})
\end{aligned}
$$

Because zkVM cannot access to the storage, we need to give the state to access $$ \text{State}^{\text{get, iter}} $$ beforehand.
It is also enough to output only the written state $$ \text{State}^{\text{set, del}} $$ without entire state.
To calculate the $$ \text{StateRootNext} $$, it is also needed to give the state node hashes $$ \text{StateNodeHashes}^{\text{NoAccess}} $$ to allow zkVM to calculate the state root.

By committing these three values:

- $$\text{StateRootPrev}$$
- $$\text{StateRootNext}$$
- $$\text{TxRoot}$$

as the public input of the ZKP, it is possible to generate the verifiable validity proof of the state transition.

$$
\begin{aligned}
\text{PublicInputsStf} &= [\text{StateRootPrev}, \text{StateRootNext}, \text{TxRoot}] \\
\text{PrivateInputsStf} &= [\text{StatePrev}^{\text{get, iter}}, \\
    &\qquad \text{StateNext}^{\text{set, del}}, \\
    &\qquad \text{StateNodeHashes}^{\text{NoAccess}}, \\
    &\qquad \text{Txs}] \\
\text{ProofStf} &= \text{ZKP}(\text{PublicInputsStf}, \text{PrivateInputsStf})
\end{aligned}
$$

### Security assumptions

Here, it is said that we give the state to zkVM beforehand.
If we don't prove that the given state is correct, it is possible to make a false proof.
To prevent this, we also need to prove that the given state is correct.

Proving it only for get access (only for one designated key) is very easy.
Merkle inclusion proof with the given state root is enough.

However, proving it for iter access (all keys which match the designated key prefix) is not so easy.
*Twin Nibble Trees* enables it while keeping the ZK friendliness.

### Twin Nibble Trees

Twin Nibble Trees combines two tree components:

- 4-bit-Radix Sparse Merkle Tree for state inclusion proof
  - The same architecture as [Jellyfish Merkle Tree](https://developers.diem.com/docs/technical-papers/jellyfish-merkle-tree-paper/) made by Diem (ex-Libra)
- 4-bit-Radix Patricia Trie for key indexing to enable key prefix based iteration

The state root is calculated by the following equation where $$h$$ is the hash function:

$$
\text{StateRoot} = h(\text{StateSmtRoot} || \text{KeyPatriciaRoot})
$$

### 4-bit-Radix Sparse Merkle Tree

This tree works for the state inclusion proof.

It can be used for proving get access validity in the state transition, and also for state inclusion proof of light client based interoperability protocol like IBC.

The leaf index is determined by the key hash, and the leaf value is the state hash.

```rust
pub struct State4RadixSmtInclusionProof {
  pub leaf_hash: [u8; 32],
  pub sparse_bitmap: u64,
  pub path: Vec<State4RadixSmtPath>,
}

pub struct State4RadixSmtPath {
  pub index: u8,
  pub sibling_bitmap: u16,
  pub sibling_hashes: Vec<[u8; 32]>,
}
```

Thanks to the property of the sparseness, if the sibling hashes are not present, the proof size is reduced.
Thanks to the property of the hash function, the attack vector of increasing the inclusion proof size of the specific key is also reduced.

By making it 4-bit Radix, the depth of the tree is reduced from 256 to 64, and the proof size is also reduced.

To prove the validity of get access, it is needed to prove the inclusion of the key in the tree for $$ \{ \text{Key}_i \}_{j=1}^{k} $$.

$$
\begin{aligned}
\text{KeysHash} &= h(\text{Key}_1 || \text{Key}_2 || \dots || \text{Key}_k) \\
\text{PublicInputsGet} &= [\text{StateSmtRootPrev}, \text{KeysHash}] \\
\text{PrivateInputsGet} &= [\{\text{Key}_j, \text{StateSmtInclusionProof}_j\}_{j=1}^{k}] \\
\text{ProofGet} &= \text{ZKP}(\text{PublicInputsGet}, \text{PrivateInputsGet})
\end{aligned}
$$

### 4-bit-Radix Patricia Trie

This trie works for the key indexing.

It can be used for proving iter access validity in the state transition.

```rust
pub struct Key4RadixPatriciaNode {
  pub key_fragment: Vec<u8>,
  pub nibble_front: bool,
  pub nibble_back: bool,
  pub child_bitmap: u16,
  pub children: Vec<Key4RadixPatriciaNode>,
}
```

The node hash is calculated by the following equation where $$h$$ is the hash function:

$$
\text{KeyPatriciaNodeHash} = h(\text{KeyFragment} || \text{ChildNodeHash}_1 || ... || \text{ChildNodeHash}_{16})
$$

To prove the validity of iter access, it is needed to re-construct the $$\text{KeyPatriciaNodeHash}$$ of the designated key prefix, with all iterated keys.

$$
\begin{aligned}
\text{KeyPrefixesHash} &= h(\text{KeyPrefix}_1 || \dots || \text{KeyPrefix}_k) \\
\text{KeyPatriciaNodes} &= \{\text{KeyPatriciaNode}_p\}_{p=1}^{q} \\
\text{PublicInputsIter} &= [\text{KeyPatriciaRootPrev}, \text{KeyPrefixesHash}] \\
\text{PrivateInputsIter} &= [\{\text{KeyPrefix}_j, \text{KeyPatriciaNodes}_j\}_{j=1}^{k}] \\
\text{ProofIter} &= \text{ZKP}(\text{PublicInputsIter}, \text{PrivateInputsIter})
\end{aligned}
$$

It is straightforward to think that this proof is mathematically heavy, but there is a room for parallelization.

### Parallelization of ZKP generation

Generally speaking, the list of transactions is a part of the block.

$$
  \text{Txs} \in \text{Block}
$$

In the InterLiquid SDK, to get the accessed state which is needed to give to zkVM, it is needed to execute the transactions once outside of the zkVM.

Here, we can get the interim result of the state transition function of entire block by assuming the chunk of the transactions $$\{\text{TxsChunk}_i\}_{i=1}^{n}$$, with emitting the accessed keys $$\{\text{Key}_{ij}\}_{j=1}^{k}$$ and $$\{\text{KeyPrefix}_{ij}\}_{j=1}^{k}$$:

$$
\begin{aligned}
&\{ \text{StateRootNext}_i, \text{StateNext}_i^{\text{set, del}}, \{\text{Key}_{ij}, \text{KeyPrefix}_{ij}\}_{j=1}^{k} \} \\
&= g({\text{StateRootPrev} , \text{StatePrev}_i^{\text{get, iter}}}, \text{StateNodeHashes}_i^{\text{NoAccess}}, \text{TxsChunk}_i)
\end{aligned}
$$

$$
\begin{aligned}
\text{TxChunkHash}_i &= h(\text{TxInChunk}_1 || \dots || \text{TxInChunk}_{c(i)}) \\
\text{PublicInputsChunkStf}_i &= [\text{StateRootPrev}, \text{StateRootNext}_i, \text{TxChunkHash}_i] \\
\text{PrivateInputsChunkStf}_i &= [\text{StatePrev}_i^{\text{get, iter}}, \\
    &\qquad \text{StateNext}_i^{\text{set, del}}, \\
    &\qquad \text{StateNodeHashes}_i^{\text{NoAccess}}, \\
    &\qquad \text{TxsChunk}_i]
\end{aligned}
$$

Then we can generate the proof in parallel for each chunk with a combined circuit among $$\text{ProofChunkStf}_i$$, $$\text{ProofChunkGet}_i$$, and $$\text{ProofChunkIter}_i$$:

$$
\begin{aligned}
\forall i \in \{1:n\} \text{ in parallel:} \\
\text{PublicInputsChunk}_i &= [\text{PublicInputsChunkStf}_i] \\
\text{PrivateInputsChunk}_i &= [\text{PrivateInputsChunkStf}_i, \\
    &\qquad \text{PrivateInputsChunkGet}_i, \\
    &\qquad \text{PrivateInputsChunkIter}_i] \\
\text{ProofChunk}_i &= \text{ZKP}(\text{PublicInputsChunk}_i, \text{PrivateInputsChunk}_i)
\end{aligned}
$$

Not only for the parallelization but also the fact that the proof of ZK-STARK requires quasi-linear time $$\mathcal{O}(n \log{n})$$ in proportion to the number of traces, it is meaningfull to separate txs into chunks.

By combining these three circuits, we can omit $$\text{KeysHash}$$ and $$\text{KeyPrefixesHash}$$ in the public inputs of the ZKP because fundamentally STF $$g$$ can verify the validity of $$\{\text{Key}\}_{j=1}^k$$ and $$\{\text{KeyPrefix}\}_{j=1}^k$$ by itself.

Needless to say, $$\text{StateSmtRootPrev}$$ and $$\text{KeyPatriciaRootPrev}$$ which need to be verified, also can be verified by using $$\text{PublicInputsChunk}_i$$ in the circuit.

Finally, we can aggregate all proofs with recursive ZKP:

$$
\begin{aligned}
\text{PublicInputsAgg} &= [\text{StateRootPrev}_1, \text{StateRootNext}_n, \text{TxRoot}] \\
\text{PrivateInputsAgg} &= [\{\text{StateRootPrev}_i\}_{i=2}^{n}, \\
    &\qquad \{\text{StateRootNext}_i\}_{i=1}^{n-1}, \\
    &\qquad \{\text{TxsChunk}_i, \text{ProofChunk}_i\}_{i=1}^{n}] \\
\text{ProofAgg} &= \text{ZKP}(\text{PublicInputsAgg}, \text{PrivateInputsAgg})
\end{aligned}
$$

In this zkVM program, each $$\text{TxChunkHash}_i$$ is calculated internally and used for the public input of the internal ZKP verifications because $$\text{TxRoot}$$ should be not series hash but merkle root of all txs to support the tx inclusion proof.

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

InterLiquid SDK has great theoretical background and has a practical vision to accelerate the fusion of Web2 and Web3.
