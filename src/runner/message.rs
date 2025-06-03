use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    types::Timestamp, 
    zkp::{
        WitnessTx, WitnessTxAgg, WitnessCommitState, WitnessCommitKeys,
        PublicInputTx, PublicInputTxAgg, PublicInputCommitState, PublicInputCommitKeys
    }
};

/// Represents all possible message types that can be sent between components in the runner system.
/// These messages coordinate the transaction processing and proof generation pipeline.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum RunnerMessage {
    TxReceived(MessageTxReceived),
    TxProofReady(MessageTxProofReady),
    TxProofAggregationReady(MessageTxProofAggregationReady),
    CommitStateProofReady(MessageCommitStateProofReady),
    CommitKeysProofReady(MessageCommitKeysProofReady),
    BlockCommitted(MessageBlockCommitted),
    TxProved(MessageTxProved),
    TxProofAggregated(MessageTxProofAggregated),
    CommitStateProved(MessageCommitStateProved),
    CommitKeysProved(MessageCommitKeysProved),
    BlockProved(MessageBlockProved),
}

/// Message indicating that a new transaction has been received and needs to be processed.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxReceived {
    pub tx: Vec<u8>,
}

impl MessageTxReceived {
    /// Creates a new MessageTxReceived instance.
    ///
    /// # Arguments
    /// * `tx` - The serialized transaction data
    pub fn new(tx: Vec<u8>) -> Self {
        Self { tx }
    }
}

/// Message indicating that a transaction is ready to be proved.
/// Contains all witness data needed for proof generation.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProofReady {
    pub chain_id: String,
    pub block_height: u64,
    pub block_time: Timestamp,
    pub tx_index: usize,
    pub witness: WitnessTx,
}

impl MessageTxProofReady {
    /// Creates a new MessageTxProofReady instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block containing the transaction
    /// * `block_time` - The Unix timestamp of the block
    /// * `tx_index` - The index of the transaction within the block
    /// * `witness` - The witness data needed for proving
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_time: Timestamp,
        tx_index: usize,
        witness: WitnessTx,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_time,
            tx_index,
            witness,
        }
    }
}

/// Message indicating that transaction proofs are ready to be aggregated.
/// Contains the range of transaction indices to aggregate and witness data.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProofAggregationReady {
    pub chain_id: String,
    pub block_height: u64,
    pub tx_index: (usize, usize),
    pub witness: WitnessTxAgg,
}

/// Message indicating that the state commitment is ready to be proved.
/// Contains the state root hash and witness data needed for proof generation.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitStateProofReady {
    pub chain_id: String,
    pub block_height: u64,
    pub state_root: [u8; 32],
    pub witness: WitnessCommitState,
}

impl MessageCommitStateProofReady {
    /// Creates a new MessageCommitStateProofReady instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `state_root` - The 32-byte state root hash
    /// * `witness` - The witness data for state commitment proof
    pub fn new(chain_id: String, block_height: u64, state_root: [u8; 32], witness: WitnessCommitState) -> Self {
        Self {
            chain_id,
            block_height,
            state_root,
            witness,
        }
    }
}

/// Message indicating that the keys commitment is ready to be proved.
/// Contains the keys root hash and witness data needed for proof generation.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitKeysProofReady {
    pub chain_id: String,
    pub block_height: u64,
    pub keys_root: [u8; 32],
    pub witness: WitnessCommitKeys,
}

impl MessageCommitKeysProofReady {
    /// Creates a new MessageCommitKeysProofReady instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `keys_root` - The 32-byte keys root hash
    /// * `witness` - The witness data for keys commitment proof
    pub fn new(chain_id: String, block_height: u64, keys_root: [u8; 32], witness: WitnessCommitKeys) -> Self {
        Self {
            chain_id,
            block_height,
            keys_root,
            witness,
        }
    }
}

/// Message indicating that a block has been committed to the blockchain.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageBlockCommitted {
    pub chain_id: String,
    pub block_height: u64,
}

impl MessageBlockCommitted {
    /// Creates a new MessageBlockCommitted instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the committed block
    pub fn new(chain_id: String, block_height: u64) -> Self {
        Self {
            chain_id,
            block_height,
        }
    }
}

/// Message indicating that a transaction proof has been generated.
/// Contains the proof data and public inputs for the transaction.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProved {
    pub chain_id: String,
    pub block_height: u64,
    pub tx_index: usize,
    pub proof: Vec<u8>,
    pub public_input: PublicInputTx,
}

impl MessageTxProved {
    /// Creates a new MessageTxProved instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block containing the transaction
    /// * `tx_index` - The index of the transaction within the block
    /// * `proof` - The generated proof data
    /// * `public_input` - The public inputs from the proof
    pub fn new(chain_id: String, block_height: u64, tx_index: usize, proof: Vec<u8>, public_input: PublicInputTx) -> Self {
        Self {
            chain_id,
            block_height,
            tx_index,
            proof,
            public_input,
        }
    }
}

/// Message indicating that transaction proofs have been aggregated.
/// Contains the aggregated proof and public inputs for a range of transactions.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageTxProofAggregated {
    pub chain_id: String,
    pub block_height: u64,
    pub tx_index: (usize, usize),
    pub proof: Vec<u8>,
    pub public_input: PublicInputTxAgg,
}

impl MessageTxProofAggregated {
    /// Creates a new MessageTxProofAggregated instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `tx_index` - Tuple representing the range of transaction indices (start, end)
    /// * `proof` - The aggregated proof data
    /// * `public_input` - The public inputs from the aggregated proof
    pub fn new(
        chain_id: String,
        block_height: u64,
        tx_index: (usize, usize),
        proof: Vec<u8>,
        public_input: PublicInputTxAgg,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            tx_index,
            proof,
            public_input,
        }
    }
}

/// Message indicating that a state commitment proof has been generated.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitStateProved {
    pub chain_id: String,
    pub block_height: u64,
    pub state_root: [u8; 32],
    pub proof: Vec<u8>,
    pub public_input: PublicInputCommitState,
}

impl MessageCommitStateProved {
    /// Creates a new MessageCommitStateProved instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `state_root` - The 32-byte state root hash that was proved
    /// * `proof` - The generated proof data
    /// * `public_input` - The public inputs from the proof
    pub fn new(chain_id: String, block_height: u64, state_root: [u8; 32], proof: Vec<u8>, public_input: PublicInputCommitState) -> Self {
        Self {
            chain_id,
            block_height,
            state_root,
            proof,
            public_input,
        }
    }
}

/// Message indicating that a keys commitment proof has been generated.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageCommitKeysProved {
    pub chain_id: String,
    pub block_height: u64,
    pub keys_root: [u8; 32],
    pub proof: Vec<u8>,
    pub public_input: PublicInputCommitKeys,
}

impl MessageCommitKeysProved {
    /// Creates a new MessageCommitKeysProved instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `keys_root` - The 32-byte keys root hash that was proved
    /// * `proof` - The generated proof data
    /// * `public_input` - The public inputs from the proof
    pub fn new(chain_id: String, block_height: u64, keys_root: [u8; 32], proof: Vec<u8>, public_input: PublicInputCommitKeys) -> Self {
        Self {
            chain_id,
            block_height,
            keys_root,
            proof,
            public_input,
        }
    }
}

/// Message indicating that an entire block proof has been generated.
/// Contains the final proof covering all transactions and commitments in the block.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MessageBlockProved {
    pub chain_id: String,
    pub block_height: u64,
    pub entire_root: [u8; 32],
}

impl MessageBlockProved {
    /// Creates a new MessageBlockProved instance.
    ///
    /// # Arguments
    /// * `chain_id` - The identifier of the blockchain
    /// * `block_height` - The height of the block
    /// * `entire_root` - The 32-byte root hash covering the entire block
    pub fn new(chain_id: String, block_height: u64, entire_root: [u8; 32]) -> Self {
        Self {
            chain_id,
            block_height,
            entire_root,
        }
    }
}
