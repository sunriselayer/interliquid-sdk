use super::{msg_registry::MsgRegistry, type_registry::TypeRegistry};

pub trait Module {
    fn register_types(&self, type_registry: &mut TypeRegistry);
    fn register_msgs(&'static self, msg_registry: &mut MsgRegistry);
}
