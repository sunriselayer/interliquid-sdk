use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::keeper::CryptoKeeper;

/// The crypto module that provides cryptographic functionality.
/// 
/// This module manages cryptographic operations through its keeper,
/// which handles verifying key registration and unpacking.
pub struct CryptoModule {
    /// The crypto keeper instance used by this module
    keeper: Arc<CryptoKeeper>,
}

impl CryptoModule {
    /// Creates a new `CryptoModule` instance.
    /// 
    /// # Arguments
    /// 
    /// * `keeper` - A shared reference to the crypto keeper
    /// 
    /// # Returns
    /// 
    /// Returns a new `CryptoModule` instance with the provided keeper.
    pub fn new(keeper: Arc<CryptoKeeper>) -> Self {
        Self { keeper }
    }

    /// Returns a reference to the module's crypto keeper.
    /// 
    /// # Returns
    /// 
    /// Returns a reference to the `CryptoKeeper` instance.
    pub fn keeper(&self) -> &CryptoKeeper {
        &self.keeper
    }
}

impl Module for CryptoModule {
    /// Registers message types and handlers for the crypto module.
    /// 
    /// Currently, this module does not register any messages.
    /// 
    /// # Arguments
    /// 
    /// * `_msg_registry` - The message registry (unused)
    /// * `_msg_handler_registry` - The message handler registry (unused)
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
