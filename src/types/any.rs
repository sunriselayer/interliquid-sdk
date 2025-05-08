use std::any::Any;

use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SerializableAny {
    pub type_: String,
    pub value: Vec<u8>,
}

impl SerializableAny {
    pub fn new(type_: String, value: Vec<u8>) -> Self {
        Self { type_, value }
    }
}

pub trait NamedSerializableType: Any + BorshSerialize + BorshDeserialize {
    fn type_name() -> &'static str;

    fn to_any(&self) -> Result<SerializableAny, InterLiquidSdkError> {
        let mut buf = vec![];
        self.serialize(&mut buf)?;

        let any = SerializableAny::new(Self::type_name().to_owned(), buf);

        Ok(any)
    }

    fn from_any(any: SerializableAny) -> Result<Self, InterLiquidSdkError> {
        let value = Self::deserialize(&mut &any.value[..])?;

        Ok(value)
    }
}
