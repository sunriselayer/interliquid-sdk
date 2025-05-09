use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::{Address, InterLiquidSdkError, SerializableAny};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct TxBody {
    pub msgs: Vec<SerializableAny>,
    // Unix timestamp in seconds
    pub timeout_seconds: u64,
    pub options: Vec<SerializableAny>,
}

impl TxBody {
    pub fn new(
        msgs: Vec<SerializableAny>,
        timeout_seconds: u64,
        options: Vec<SerializableAny>,
    ) -> Self {
        Self {
            msgs,
            timeout_seconds,
            options,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AuthInfo {
    pub address: Address,
    pub nonce: u64,
    pub key_index: u64,
    pub verifying_key: SerializableAny,
}

impl AuthInfo {
    pub fn new(
        address: Address,
        nonce: u64,
        key_index: u64,
        verifying_key: SerializableAny,
    ) -> Self {
        Self {
            address,
            nonce,
            key_index,
            verifying_key,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct StdTx {
    pub body: TxBody,
    pub auth_info: AuthInfo,
    pub signature: Vec<u8>,
}

impl StdTx {
    pub fn new(body: TxBody, auth_info: AuthInfo, signature: Vec<u8>) -> Self {
        Self {
            body,
            auth_info,
            signature,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize)]
pub struct SignDoc<'a> {
    pub body: &'a TxBody,
    pub auth_info: &'a AuthInfo,
    pub chain_id: &'a str,
}

impl<'a> SignDoc<'a> {
    pub fn new(body: &'a TxBody, auth_info: &'a AuthInfo, chain_id: &'a str) -> Self {
        Self {
            body,
            auth_info,
            chain_id,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, InterLiquidSdkError> {
        let mut bytes = vec![];
        self.serialize(&mut bytes)?;
        Ok(bytes)
    }
}
