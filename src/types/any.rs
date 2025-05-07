use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::InterLiquidSdkError;

pub struct Any {
    pub type_: String,
    pub value: Vec<u8>,
}

impl Any {
    pub fn new(type_: String, value: Vec<u8>) -> Self {
        Self { type_, value }
    }
}

pub trait NamedSerializableType: BorshSerialize + BorshDeserialize {
    fn type_name() -> String;

    fn to_any(&self) -> Result<Any, InterLiquidSdkError> {
        let mut buf = vec![];
        self.serialize(&mut buf)?;

        let any = Any::new(Self::type_name(), buf);

        Ok(any)
    }

    fn from_any(any: Any) -> Result<Self, InterLiquidSdkError> {
        let value = Self::deserialize(&mut &any.value[..])?;

        Ok(value)
    }
}
