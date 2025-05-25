use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::{msg_send::MsgSend, BankKeeper};

/// The bank module handles token transfers and balance management.
/// It provides functionality for sending tokens between accounts and querying balances.
pub struct BankModule {
    /// Shared reference to the bank keeper for state management
    keeper: Arc<BankKeeper>,
}

impl BankModule {
    /// Creates a new instance of the bank module.
    ///
    /// # Arguments
    /// * `keeper` - Shared reference to the bank keeper
    ///
    /// # Returns
    /// A new BankModule instance
    pub fn new(keeper: Arc<BankKeeper>) -> Self {
        Self { keeper }
    }

    /// Returns a reference to the bank keeper.
    ///
    /// # Returns
    /// Reference to the bank keeper for accessing bank functionality
    pub fn keeper(&self) -> &BankKeeper {
        &self.keeper
    }
}

impl Module for BankModule {
    /// Registers bank module messages and their handlers.
    /// Currently registers MsgSend for token transfers.
    ///
    /// # Arguments
    /// * `msg_registry` - Registry for message types
    /// * `msg_handler_registry` - Registry for message handlers
    fn register_msgs(
        self: Arc<Self>,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
        msg_registry.register::<MsgSend>();

        let module = self.clone();
        msg_handler_registry
            .register::<MsgSend>(Box::new(move |ctx, msg| module.keeper.msg_send(ctx, msg)));
    }
}
