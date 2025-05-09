use crate::{
    core::Context,
    types::{Address, InterLiquidSdkError, SerializableAny},
    utils::Map,
};

use super::{
    key::{ACCOUNTS, AUTH, VERIFYING_KEYS, VERIFYING_KEY_COUNTER},
    types::Account,
};

pub trait AuthKeeperI {
    fn get_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
    ) -> Result<Option<Account>, InterLiquidSdkError>;

    fn set_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        account: &Account,
    ) -> Result<(), InterLiquidSdkError>;

    fn get_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_index: u64,
    ) -> Result<Option<SerializableAny>, InterLiquidSdkError>;

    fn add_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        verifying_key: &SerializableAny,
    ) -> Result<(), InterLiquidSdkError>;

    fn del_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_index: u64,
    ) -> Result<(), InterLiquidSdkError>;
}

pub struct AuthKeeper {
    accounts: Map<Address, Account>,
    verifying_keys: Map<(Address, u64), SerializableAny>,
    verifying_key_counter: Map<Address, u64>,
}

impl AuthKeeper {
    pub fn new() -> Self {
        Self {
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
        let account = self.accounts.get(ctx.state_manager(), address)?;
        Ok(account)
    }

    fn set_account(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        account: &Account,
    ) -> Result<(), InterLiquidSdkError> {
        self.accounts.set(ctx.state_manager(), address, account)?;
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
            .get(ctx.state_manager(), (address, key_id))?;
        Ok(verifying_key)
    }

    fn add_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        verifying_key: &SerializableAny,
    ) -> Result<(), InterLiquidSdkError> {
        let key_id = self
            .verifying_key_counter
            .get(ctx.state_manager(), address)?
            .unwrap_or_default();
        self.verifying_keys
            .set(ctx.state_manager(), (address, key_id), verifying_key)?;
        self.verifying_key_counter
            .set(ctx.state_manager(), address, &(key_id + 1))?;

        Ok(())
    }

    fn del_verifying_key(
        &self,
        ctx: &mut dyn Context,
        address: &Address,
        key_index: u64,
    ) -> Result<(), InterLiquidSdkError> {
        self.verifying_keys
            .del(ctx.state_manager(), (address, key_index))?;

        Ok(())
    }
}
