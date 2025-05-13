use borsh::BorshDeserialize;

use crate::types::SerializableAny;

pub trait Tx: BorshDeserialize + Send + Sync + 'static {
    fn msgs(&self) -> Vec<SerializableAny>;
}
