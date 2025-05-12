use std::collections::BTreeSet;

use crate::types::Address;

pub trait Msg {
    fn signer_addresses(&self) -> BTreeSet<Address>;
}
