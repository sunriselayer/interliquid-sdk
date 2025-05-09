use crate::x::{auth::AuthKeeper, crypto::keeper::CryptoKeeper};

pub struct AddrVerifyAnteHandler {
    auth_keeper: AuthKeeper,
    crypto_keeper: CryptoKeeper,
}

impl AddrVerifyAnteHandler {
    pub fn new(auth_keeper: AuthKeeper, crypto_keeper: CryptoKeeper) -> Self {
        Self {
            auth_keeper,
            crypto_keeper,
        }
    }
}
