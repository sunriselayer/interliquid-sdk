/// Base prefix for auth module state storage.
pub const AUTH: &[u8] = b"auth";
/// Storage key for account data.
pub const ACCOUNTS: &[u8] = b"accounts";
/// Storage key for verifying keys associated with accounts.
pub const VERIFYING_KEYS: &[u8] = b"verifying_keys";
/// Storage key for tracking the next available key index for each account.
pub const VERIFYING_KEY_COUNTER: &[u8] = b"verifying_key_counter";
