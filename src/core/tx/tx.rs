use borsh::BorshDeserialize;

use crate::types::SerializableAny;

/// Single Tx contains multiple Msgs.
/// Tx is the unit for authentication, so Tx would contain signature and so on.
pub trait Tx: BorshDeserialize + Send + Sync + 'static {
    fn msgs(&self) -> Vec<SerializableAny>;
}
