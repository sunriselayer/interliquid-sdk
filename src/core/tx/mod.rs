//! Transaction processing module for the InterLiquid SDK.
//!
//! This module provides the core infrastructure for handling transactions and messages
//! in the InterLiquid SDK. It includes traits and registries for defining, registering,
//! and executing messages within transactions.
//!
//! # Key Components
//!
//! - **Transaction (`Tx`)**: The atomic unit of execution containing one or more messages
//! - **Message (`Msg`)**: Individual state transition operations within a transaction
//! - **Handlers**: Pre and post-processing logic for transactions
//! - **Registries**: Type registration and dynamic dispatch for messages

mod handler;
mod msg;
mod msg_handler;
mod msg_registry;
mod tx;

pub use handler::*;
pub use msg::*;
pub use msg_handler::*;
pub use msg_registry::*;
pub use tx::*;
