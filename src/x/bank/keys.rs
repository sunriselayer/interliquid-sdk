/// Key prefix for the bank module's state storage.
/// This prefix is used to namespace all bank-related data in the state store.
pub const BANK: &[u8] = b"bank/";
/// Key prefix for storing account balances.
/// Used in combination with BANK prefix to store balance data.
pub const BALANCES: &[u8] = b"balances/";
