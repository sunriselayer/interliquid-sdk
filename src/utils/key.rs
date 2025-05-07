pub fn join_keys<'a, P: IntoIterator<Item = &'a [u8]>>(parts: P) -> Vec<u8> {
    parts.into_iter().fold(Vec::new(), |mut acc, p| {
        acc.extend_from_slice(p);
        acc
    })
}

pub trait KeySerializable {
    fn to_key_bytes(&self) -> Vec<u8>;
}

impl KeySerializable for u8 {
    fn to_key_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl KeySerializable for u16 {
    fn to_key_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl KeySerializable for u32 {
    fn to_key_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl KeySerializable for u64 {
    fn to_key_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl KeySerializable for str {
    fn to_key_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl KeySerializable for String {
    fn to_key_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<T1: KeySerializable, T2: KeySerializable> KeySerializable for (T1, T2) {
    fn to_key_bytes(&self) -> Vec<u8> {
        [self.0.to_key_bytes(), self.1.to_key_bytes()].concat()
    }
}

impl<T1: KeySerializable, T2: KeySerializable, T3: KeySerializable> KeySerializable
    for (T1, T2, T3)
{
    fn to_key_bytes(&self) -> Vec<u8> {
        [
            self.0.to_key_bytes(),
            self.1.to_key_bytes(),
            self.2.to_key_bytes(),
        ]
        .concat()
    }
}
