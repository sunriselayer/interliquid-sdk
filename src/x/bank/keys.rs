use crate::utils::{key, KeySerializable};

pub const BANK: &[u8] = b"bank";
pub const BALANCES: &[u8] = b"balances";
pub const DENOMS: &[u8] = b"denoms";

pub fn account_balances_key(address: &str) -> Vec<u8> {
    key([BANK, BALANCES], &address.key())
}
