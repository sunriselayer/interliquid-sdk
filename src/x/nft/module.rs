use std::sync::Arc;

use crate::core::{Module, MsgHandlerRegistry, MsgRegistry};

use super::NftKeeper;

/// NFT module for managing non-fungible tokens
/// 
/// This module provides the functionality for creating, transferring,
/// and managing NFTs within the system.
pub struct NftModule {
    keeper: Arc<NftKeeper>,
}

impl NftModule {
    /// Creates a new NFT module instance
    /// 
    /// # Parameters
    /// 
    /// * `keeper` - The NFT keeper that manages NFT storage and operations
    /// 
    /// # Returns
    /// 
    /// A new `NftModule` instance
    pub fn new(keeper: Arc<NftKeeper>) -> Self {
        Self { keeper }
    }

    /// Returns a reference to the NFT keeper
    /// 
    /// # Returns
    /// 
    /// A reference to the `NftKeeper` instance
    pub fn keeper(&self) -> &NftKeeper {
        &self.keeper
    }
}

impl Module for NftModule {
    /// Registers NFT-related messages and their handlers
    /// 
    /// This method is called during module initialization to register
    /// all NFT-specific message types and their corresponding handlers.
    /// 
    /// # Parameters
    /// 
    /// * `_msg_registry` - Registry for message types
    /// * `_msg_handler_registry` - Registry for message handlers
    fn register_msgs(
        self: Arc<Self>,
        _msg_registry: &mut MsgRegistry,
        _msg_handler_registry: &mut MsgHandlerRegistry,
    ) {
    }
}
