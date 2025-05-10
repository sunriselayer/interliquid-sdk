use std::collections::BTreeMap;

use borsh::BorshSerialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    tx::Tx,
    types::{Address, InterLiquidSdkError, SerializableAny},
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct TxBody {
    pub msgs: Vec<SerializableAny>,
    // Unix timestamp in seconds
    pub timeout_seconds: u64,
    pub options: Vec<SerializableAny>,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AuthInfo {
    pub address: Address,
    pub nonce: u64,
    pub key_index: u64,
    pub verifying_key: SerializableAny,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct StdTx {
    pub body: TxBody,
    pub auth_info: BTreeMap<Address, AuthInfo>,
    pub signature: BTreeMap<Address, Vec<u8>>,
}

impl Tx for StdTx {
    fn msgs(&self) -> Vec<SerializableAny> {
        self.body.msgs.clone()
    }
}

#[derive(Debug, Clone, BorshSerialize)]
pub struct SignDoc<'a> {
    pub body: &'a TxBody,
    pub auth_info: &'a BTreeMap<Address, AuthInfo>,
    pub chain_id: &'a str,
}

impl<'a> SignDoc<'a> {
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

    pub fn to_bytes(&self) -> Result<Vec<u8>, InterLiquidSdkError> {
        let mut bytes = vec![];
        self.serialize(&mut bytes)?;
        Ok(bytes)
    }
}
