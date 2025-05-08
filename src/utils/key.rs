use borsh::{BorshDeserialize, BorshSerialize};

pub fn join_keys<'a, P: IntoIterator<Item = &'a [u8]>>(parts: P) -> Vec<u8> {
    parts.into_iter().fold(Vec::new(), |mut acc, p| {
        acc.extend_from_slice(p);
        acc
    })
}

pub trait KeyDeclaration: BorshDeserialize + Clone {
    type KeyReference<'a>: BorshSerialize + Clone + Copy + 'a;

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

impl KeyDeclaration for String {
    type KeyReference<'a> = &'a str;

    fn to_key_bytes<'a>(key: Self::KeyReference<'a>) -> Vec<u8> {
        let mut buf = Vec::new();
        key.serialize(&mut buf).unwrap();
        buf
    }
}

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
