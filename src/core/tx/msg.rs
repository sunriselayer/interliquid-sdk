use std::collections::BTreeSet;

use crate::types::Address;

/// Single Tx contains multiple Msgs.
/// Each Msg is the unit for state transition.
/// Msgs are defined by each module.
pub trait Msg {
    fn signer_addresses(&self) -> BTreeSet<Address>;
}
