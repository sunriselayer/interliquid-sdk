pub trait NftKeeperI {}

pub struct NftKeeper {}

impl NftKeeper {
    pub fn new() -> Self {
        Self {}
    }
}

impl NftKeeperI for NftKeeper {}
