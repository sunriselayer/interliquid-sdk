use std::any::Any;

use crate::{core::Context, types::InterLiquidSdkError};

pub trait TxAnteHandler {
    fn handle(&self, ctx: &mut dyn Context, tx: &Box<dyn Any>) -> Result<(), InterLiquidSdkError>;
}

pub trait TxPostHandler {
    fn handle(&self, ctx: &mut dyn Context, tx: &Box<dyn Any>) -> Result<(), InterLiquidSdkError>;
}
