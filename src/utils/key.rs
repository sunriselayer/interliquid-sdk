pub fn key<'a, P: IntoIterator<Item = &'a [u8]>>(prefix: P, key: &[u8]) -> Vec<u8> {
    let mut k = prefix.into_iter().fold(Vec::new(), |mut acc, p| {
        acc.extend_from_slice(p);
        acc.push(b'/');
        acc
    });
    k.extend(key);
    k
}

pub trait KeySerializable {
    fn key(&self) -> Vec<u8>;
}

impl KeySerializable for u8 {
    fn key(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl KeySerializable for u16 {
    fn key(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl KeySerializable for u32 {
    fn key(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl KeySerializable for u64 {
    fn key(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl KeySerializable for str {
    fn key(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl KeySerializable for String {
    fn key(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}
