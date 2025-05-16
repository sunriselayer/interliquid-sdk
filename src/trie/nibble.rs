use borsh_derive::{BorshDeserialize, BorshSerialize};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct Nibble(u8);

impl Nibble {
    const MASK: u8 = 0xF;
    pub const MAX: u8 = 0xF;

    pub fn new(value: u8) -> Self {
        Self(value & Self::MASK)
    }

    pub fn split(value: u8) -> (Self, Self) {
        (Self::new(value >> 4), Self::new(value & Self::MASK))
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn as_slice(slice: &[Nibble]) -> &[u8] {
        unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) }
    }
}

impl From<u8> for Nibble {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Nibble> for u8 {
    fn from(value: Nibble) -> Self {
        value.0
    }
}

pub fn nibbles_from_bytes(bytes: &[u8]) -> Vec<Nibble> {
    bytes
        .iter()
        .map(|b| Nibble::split(*b))
        .flat_map(|(b1, b2)| [b1, b2])
        .collect()
}
