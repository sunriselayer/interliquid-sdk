use borsh_derive::{BorshDeserialize, BorshSerialize};

use super::consts::BRANCH_BITMAP_BYTES;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct OctRadPatriciaBitmap([u8; BRANCH_BITMAP_BYTES]);

impl OctRadPatriciaBitmap {
    pub fn new(bitmap: [u8; BRANCH_BITMAP_BYTES]) -> Self {
        Self(bitmap)
    }
}

impl From<[u8; BRANCH_BITMAP_BYTES]> for OctRadPatriciaBitmap {
    fn from(bitmap: [u8; BRANCH_BITMAP_BYTES]) -> Self {
        Self(bitmap)
    }
}

impl From<OctRadPatriciaBitmap> for [u8; BRANCH_BITMAP_BYTES] {
    fn from(bitmap: OctRadPatriciaBitmap) -> Self {
        bitmap.0
    }
}

impl AsRef<[u8]> for OctRadPatriciaBitmap {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl OctRadPatriciaBitmap {
    pub fn get(&self, index: u8) -> bool {
        let segment = index as usize / 8;
        let bit = index % 8;
        self.0[segment] & 1 << bit != 0
    }
}
