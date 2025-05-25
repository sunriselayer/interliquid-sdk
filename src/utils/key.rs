use borsh::{BorshDeserialize, BorshSerialize};

/// Join multiple key parts into one key
/// 
/// This function concatenates multiple byte slices into a single key.
/// It's commonly used to build hierarchical keys from prefixes and suffixes.
/// 
/// # Parameters
/// - `parts`: An iterator of byte slices to concatenate
/// 
/// # Returns
/// A `Vec<u8>` containing all the parts joined together
/// 
/// # Example
/// ```ignore
/// let key = join_keys([b"users", b"alice"]);
/// // key = b"usersalice"
/// ```
pub fn join_keys<'a, P: IntoIterator<Item = &'a [u8]>>(parts: P) -> Vec<u8> {
    parts.into_iter().fold(Vec::new(), |mut acc, p| {
        acc.extend_from_slice(p);
        acc
    })
}

/// Trait for types that can be used as keys in state storage.
/// 
/// This trait defines how types can be used as keys in maps and other storage structures.
/// It provides a way to convert between owned key types and their references, and to
/// serialize keys to bytes for storage.
/// 
/// # Associated Types
/// - `KeyReference`: The reference type used when passing keys to storage operations.
///   This allows for efficient key passing without cloning.
/// 
/// # Required Methods
/// - `to_key_bytes`: Converts a key reference to its byte representation
pub trait KeyDeclaration: BorshSerialize + BorshDeserialize + Clone + Send + Sync {
    /// The reference type for this key
    type KeyReference<'a>: BorshSerialize + Clone + Copy + Send + 'a;

    /// Converts a key reference to its byte representation for storage
    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8>;
}

impl KeyDeclaration for u8 {
    type KeyReference<'a> = u8;

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        vec![key]
    }
}

impl KeyDeclaration for u16 {
    type KeyReference<'a> = u16;

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}

impl KeyDeclaration for u32 {
    type KeyReference<'a> = u32;

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}

impl KeyDeclaration for u64 {
    type KeyReference<'a> = u64;

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}

impl<const N: usize> KeyDeclaration for [u8; N] {
    type KeyReference<'a> = &'a [u8; N];

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        key.to_vec()
    }
}

impl KeyDeclaration for String {
    type KeyReference<'a> = &'a str;

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}

// Tuple implementation
impl<T1, T2> KeyDeclaration for (T1, T2)
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    for<'a> T1::KeyReference<'a>: 'a,
    for<'a> T2::KeyReference<'a>: 'a,
{
    type KeyReference<'a> = (T1::KeyReference<'a>, T2::KeyReference<'a>);

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}

// Triple implementation
impl<T1, T2, T3> KeyDeclaration for (T1, T2, T3)
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    T3: KeyDeclaration,
    for<'a> T1::KeyReference<'a>: 'a,
    for<'a> T2::KeyReference<'a>: 'a,
    for<'a> T3::KeyReference<'a>: 'a,
{
    type KeyReference<'a> = (
        T1::KeyReference<'a>,
        T2::KeyReference<'a>,
        T3::KeyReference<'a>,
    );

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}
