use borsh::BorshDeserialize;

use crate::types::SerializableAny;

/// Represents a transaction in the InterLiquid SDK.
///
/// A transaction is the atomic unit of execution that contains one or more messages.
/// It serves as the authentication boundary, typically containing signatures and
/// other metadata required for transaction validation.
///
/// Single Tx contains multiple Msgs.
/// Tx is the unit for authentication, so Tx would contain signature and so on.
pub trait Tx: BorshDeserialize + Send + Sync + 'static {
    /// Returns the list of messages contained in this transaction.
    ///
    /// Messages are returned as `SerializableAny` to allow for dynamic
    /// message types that can be deserialized at runtime using the message registry.
    ///
    /// # Returns
    /// A vector of `SerializableAny` containing all messages in this transaction.
    fn msgs(&self) -> Vec<SerializableAny>;
}
