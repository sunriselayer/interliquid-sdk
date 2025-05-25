use std::collections::BTreeSet;

use crate::types::Address;

/// Represents a message that can be executed within a transaction.
///
/// Messages are the atomic units of state transition in the InterLiquid SDK.
/// Each message type is typically defined within its respective module and
/// implements specific business logic for state changes.
///
/// Single Tx contains multiple Msgs. Each Msg is the unit for state transition.
/// The `Msg` trait represents the fundamental unit of state change in the system.
/// Each message encapsulates a specific action that can be performed on the blockchain
/// state, such as transfers, account creation, or other module-specific operations.
pub trait Msg {
    /// Returns the set of addresses that must sign this message.
    ///
    /// This method is used to determine which accounts need to authorize
    /// the execution of this message. The transaction will fail if any
    /// of these addresses haven't provided a valid signature.
    ///
    /// # Returns
    /// A `BTreeSet` containing all addresses that must sign this message.
    /// Using a set ensures uniqueness and deterministic ordering.
    fn signer_addresses(&self) -> BTreeSet<Address>;
}
