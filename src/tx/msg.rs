use crate::types::{Address, NamedSerializableType};

pub trait Msg: NamedSerializableType {
    fn signer_address(&self) -> Address;
}
