use std::sync::Arc;

use super::{MsgHandlerRegistry, MsgRegistry};

pub trait Module: Send + Sync {
    fn register_msgs(
        self: Arc<Self>,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry,
    );
}
