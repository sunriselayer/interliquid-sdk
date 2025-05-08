pub fn join_keys<'a, P: IntoIterator<Item = &'a [u8]>>(parts: P) -> Vec<u8> {
    parts.into_iter().fold(Vec::new(), |mut acc, p| {
        acc.extend_from_slice(p);
        acc
    })
}

pub trait KeyDeclaration {
    type KeyReference<'a>: ?Sized;

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8>;
}

impl KeyDeclaration for u8 {
    type KeyReference<'a> = u8;

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        vec![*key]
    }
}

impl KeyDeclaration for u16 {
    type KeyReference<'a> = u16;

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        key.to_be_bytes().to_vec()
    }
}

impl KeyDeclaration for u32 {
    type KeyReference<'a> = u32;

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        key.to_be_bytes().to_vec()
    }
}

impl KeyDeclaration for u64 {
    type KeyReference<'a> = u64;

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        key.to_be_bytes().to_vec()
    }
}

impl KeyDeclaration for String {
    type KeyReference<'a> = str;

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        key.as_bytes().to_vec()
    }
}

impl<T1, T2> KeyDeclaration for (T1, T2)
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    for<'a> T1::KeyReference<'a>: 'a,
    for<'a> T2::KeyReference<'a>: 'a,
{
    type KeyReference<'a> = (&'a T1::KeyReference<'a>, &'a T2::KeyReference<'a>);

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        [T1::to_key_bytes(key.0), T2::to_key_bytes(key.1)].concat()
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
        &'a T1::KeyReference<'a>,
        &'a T2::KeyReference<'a>,
        &'a T3::KeyReference<'a>,
    );

    fn to_key_bytes<'a>(key: &Self::KeyReference<'a>) -> Vec<u8> {
        [
            T1::to_key_bytes(key.0),
            T2::to_key_bytes(key.1),
            T3::to_key_bytes(key.2),
        ]
        .concat()
    }
}
