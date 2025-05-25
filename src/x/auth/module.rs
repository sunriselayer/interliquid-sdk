use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::AuthKeeper;

/// The AuthModule provides authentication functionality for the blockchain.
/// It manages user accounts and their cryptographic verification keys.
pub struct AuthModule {
    keeper: Arc<AuthKeeper>,
}

impl AuthModule {
    /// Creates a new AuthModule instance.
    /// 
    /// # Arguments
    /// * `keeper` - The auth keeper that handles account and key management
    pub fn new(keeper: Arc<AuthKeeper>) -> Self {
        Self { keeper }
    }

    /// Returns a reference to the auth keeper.
    pub fn keeper(&self) -> &AuthKeeper {
        &self.keeper
    }
}

impl Module for AuthModule {
    /// Registers message types and handlers for the auth module.
    /// Currently, the auth module does not register any messages directly.
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
