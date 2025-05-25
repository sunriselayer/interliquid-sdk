use std::sync::Arc;

use crate::{
    core::Context,
    types::{Address, InterLiquidSdkError, SerializableAny},
    utils::Map,
    x::crypto::keeper::CryptoKeeperI,
};

use super::{
    key::{ACCOUNTS, AUTH, VERIFYING_KEYS, VERIFYING_KEY_COUNTER},
    types::Account,
};

/// The AuthKeeper interface defines the core authentication functionality.
/// It manages user accounts and their associated verification keys.
pub trait AuthKeeperI {
    /// Retrieves an account by its address.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `address` - The address of the account to retrieve
    /// 
    /// # Returns
    /// * `Some(Account)` if the account exists
    /// * `None` if the account does not exist
    fn get_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
    ) -> Result<Option<Account>, InterLiquidSdkError>;

    /// Stores or updates an account in the state.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `address` - The address of the account
    /// * `account` - The account data to store
    fn set_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        account: &Account,
    ) -> Result<(), InterLiquidSdkError>;

    /// Retrieves a specific verifying key for an address.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `address` - The address associated with the key
    /// * `key_index` - The index of the verifying key
    /// 
    /// # Returns
    /// * `Some(SerializableAny)` containing the verifying key if it exists
    /// * `None` if the key does not exist
    fn get_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_index: u64,
    ) -> Result<Option<SerializableAny>, InterLiquidSdkError>;

    /// Adds a new verifying key for an address.
    /// The key is validated before storage and assigned the next available index.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `address` - The address to associate the key with
    /// * `verifying_key` - The verifying key to add
    fn add_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        verifying_key: &SerializableAny,
    ) -> Result<(), InterLiquidSdkError>;

    /// Deletes a verifying key for an address.
    /// 
    /// # Arguments
    /// * `ctx` - The execution context
    /// * `address` - The address associated with the key
    /// * `key_index` - The index of the key to delete
    fn del_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_index: u64,
    ) -> Result<(), InterLiquidSdkError>;
}

/// The concrete implementation of the AuthKeeper.
/// Manages account data and verification keys in the blockchain state.
pub struct AuthKeeper {
    crypto_keeper: Arc<dyn CryptoKeeperI>,

    accounts: Map<Address, Account>,
    verifying_keys: Map<(Address, u64), SerializableAny>,
    verifying_key_counter: Map<Address, u64>,
}

impl AuthKeeper {
    /// Creates a new AuthKeeper instance.
    /// 
    /// # Arguments
    /// * `crypto_keeper` - The crypto keeper for verifying key operations
    pub fn new(crypto_keeper: Arc<dyn CryptoKeeperI>) -> Self {
        Self {
            crypto_keeper,
            accounts: Map::new([AUTH, ACCOUNTS]),
            verifying_keys: Map::new([AUTH, VERIFYING_KEYS]),
            verifying_key_counter: Map::new([AUTH, VERIFYING_KEY_COUNTER]),
        }
    }
}

impl AuthKeeperI for AuthKeeper {
    fn get_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
    ) -> Result<Option<Account>, InterLiquidSdkError> {
        let account = self.accounts.get(ctx.state_manager_mut(), address)?;
        Ok(account)
    }

    fn set_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        account: &Account,
    ) -> Result<(), InterLiquidSdkError> {
        self.accounts
            .set(ctx.state_manager_mut(), address, account)?;
        Ok(())
    }

    fn get_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_id: u64,
    ) -> Result<Option<SerializableAny>, InterLiquidSdkError> {
        let verifying_key = self
            .verifying_keys
            .get(ctx.state_manager_mut(), (address, key_id))?;

        Ok(verifying_key)
    }

    fn add_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        verifying_key: &SerializableAny,
    ) -> Result<(), InterLiquidSdkError> {
        let _ = self.crypto_keeper.unpack_verifying_key(verifying_key)?;

        let key_id = self
            .verifying_key_counter
            .get(ctx.state_manager_mut(), address)?
            .unwrap_or_default();
        self.verifying_keys
            .set(ctx.state_manager_mut(), (address, key_id), verifying_key)?;
        self.verifying_key_counter
            .set(ctx.state_manager_mut(), address, &(key_id + 1))?;

        Ok(())
    }

    fn del_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_index: u64,
    ) -> Result<(), InterLiquidSdkError> {
        self.verifying_keys
            .del(ctx.state_manager_mut(), (address, key_index))?;

        Ok(())
    }
}
