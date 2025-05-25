use std::collections::BTreeSet;

use borsh_derive::{BorshDeserialize, BorshSerialize};

use crate::{
    core::{Context, Msg},
    types::{Address, InterLiquidSdkError, NamedSerializableType, Tokens},
};

use super::{BankKeeper, BankKeeperI};

/// Message for sending tokens from one account to another.
/// This message enables token transfers between addresses in the bank module.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct MsgSend {
    /// The sender's address (must be a signer of the transaction)
    pub from_address: Address,
    /// The recipient's address
    pub to_address: Address,
    /// The collection of tokens to transfer (multiple denominations supported)
    pub tokens: Tokens,
}

impl NamedSerializableType for MsgSend {
    /// Returns the type name for serialization purposes.
    ///
    /// # Returns
    /// The canonical type name "Bank/MsgSend"
    const TYPE_NAME: &'static str = "Bank/MsgSend";
}

impl Msg for MsgSend {
    /// Returns the addresses that must sign this message.
    /// For MsgSend, only the sender needs to sign.
    ///
    /// # Returns
    /// A set containing the from_address as the only required signer
    fn signer_addresses(&self) -> BTreeSet<Address> {
        BTreeSet::from([self.from_address])
    }
}

impl BankKeeper {
    /// Handles the MsgSend message by executing a token transfer.
    ///
    /// # Arguments
    /// * `ctx` - The context for state access
    /// * `msg` - The MsgSend message containing transfer details
    ///
    /// # Returns
    /// * `Ok(())` - If the transfer succeeds
    /// * `Err` - If insufficient balance or validation fails
    pub fn msg_send(
        &self,
        ctx: &mut dyn Context,
        msg: &MsgSend,
    ) -> Result<(), InterLiquidSdkError> {
        self.send(ctx, &msg.from_address, &msg.to_address, &msg.tokens)
    }
}
