use std::collections::BTreeMap;

use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::Tx,
    types::{Address, InterLiquidSdkError, SerializableAny},
};

/// The body of a transaction containing the messages and metadata.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct TxBody {
    /// The list of messages to execute in this transaction.
    pub msgs: Vec<SerializableAny>,
    /// Unix timestamp in seconds when this transaction expires.
    pub timeout_seconds: u64,
    /// Optional transaction parameters.
    pub options: Vec<SerializableAny>,
}

/// Authentication information for a single signer in a transaction.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AuthInfo {
    /// The address of the signer.
    pub address: Address,
    /// The current nonce for replay protection.
    pub nonce: u64,
    /// The index of the verifying key to use.
    pub key_index: u64,
    /// The verifying key for signature verification.
    pub verifying_key: SerializableAny,
}

/// A standard transaction format that includes messages, authentication info, and signatures.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct StdTx {
    /// The transaction body containing messages and metadata.
    pub body: TxBody,
    /// Authentication information for each signer.
    pub auth_info: BTreeMap<Address, AuthInfo>,
    /// Signatures from each signer.
    pub signature: BTreeMap<Address, Vec<u8>>,
}

impl Tx for StdTx {
    fn msgs(&self) -> Vec<SerializableAny> {
        self.body.msgs.clone()
    }
}

/// A document that is signed to create transaction signatures.
/// Contains all the data that needs to be signed for authentication.
#[derive(Debug, Clone, BorshSerialize)]
pub struct SignDoc<'a> {
    /// Reference to the transaction body.
    pub body: &'a TxBody,
    /// Reference to the authentication info.
    pub auth_info: &'a BTreeMap<Address, AuthInfo>,
    /// The chain ID to prevent cross-chain replay attacks.
    pub chain_id: &'a str,
}

impl<'a> SignDoc<'a> {
    /// Creates a new SignDoc for signing.
    /// 
    /// # Arguments
    /// * `body` - The transaction body
    /// * `auth_info` - The authentication information
    /// * `chain_id` - The chain identifier
    pub fn new(
        body: &'a TxBody,
        auth_info: &'a BTreeMap<Address, AuthInfo>,
        chain_id: &'a str,
    ) -> Self {
        Self {
            body,
            auth_info,
            chain_id,
        }
    }

    /// Serializes the SignDoc to bytes for signing.
    /// 
    /// # Returns
    /// The serialized bytes of the document
    pub fn to_bytes(&self) -> Result<Vec<u8>, InterLiquidSdkError> {
        let mut bytes = vec![];
        self.serialize(&mut bytes)?;
        Ok(bytes)
    }
}
