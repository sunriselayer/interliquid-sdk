use super::{msg_registry::MsgRegistry, Context, MsgHandlerRegistry};

pub trait Module<C: Context>: Send + Sync {
    fn register_msgs(
        &'static self,
        msg_registry: &mut MsgRegistry,
        msg_handler_registry: &mut MsgHandlerRegistry<C>,
    );
}
