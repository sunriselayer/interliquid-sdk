use std::sync::Arc;

use super::{MsgHandlerRegistry, MsgRegistry};

/// Single module can define multiple Msgs.
pub trait Module: Send + Sync {
    /// Registers the module's message types and handlers.
    ///
    /// # Arguments
    /// * `msg_registry` - Registry for message type definitions
    /// * `msg_handler_registry` - Registry for message handler functions
    fn register_msgs(
        self: Arc<Self>,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    );
}
