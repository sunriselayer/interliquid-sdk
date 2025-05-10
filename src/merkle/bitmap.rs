use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::consts::BRANCH_BITMAP_BYTES;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadBitmap([u8; BRANCH_BITMAP_BYTES]);

impl OctRadBitmap {
    pub fn new(bitmap: [u8; BRANCH_BITMAP_BYTES]) -> Self {
        Self(bitmap)
    }
}

impl From<[u8; BRANCH_BITMAP_BYTES]> for OctRadBitmap {
    fn from(bitmap: [u8; BRANCH_BITMAP_BYTES]) -> Self {
        Self(bitmap)
    }
}

impl From<OctRadBitmap> for [u8; BRANCH_BITMAP_BYTES] {
    fn from(bitmap: OctRadBitmap) -> Self {
        bitmap.0
    }
}

impl AsRef<[u8]> for OctRadBitmap {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl OctRadBitmap {
    pub fn get(&self, index: u8) -> bool {
        let segment = index as usize / 8;
        let bit = index % 8;
        self.0[segment] & 1 << bit != 0
    }
}
