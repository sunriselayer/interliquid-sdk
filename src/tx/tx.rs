use borsh::BorshDeserialize;

use crate::types::SerializableAny;

pub trait Tx: BorshDeserialize {
    fn msgs(&self) -> Vec<SerializableAny>;
}
