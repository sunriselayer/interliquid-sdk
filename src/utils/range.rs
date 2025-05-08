use std::marker::PhantomData;
use std::ops::RangeBounds as StdRangeBounds;

use borsh::BorshDeserialize;

use crate::state::RangeBounds;

use super::KeyDeclaration;

pub trait PrefixBound {
    type KeyToExtract: KeyDeclaration;

    fn prefix_bytes(&self) -> &Vec<u8>;

    fn extract<'a>(key: &mut [u8]) -> Self::KeyToExtract {
        <Self::KeyToExtract as BorshDeserialize>::deserialize(&mut &key[..]).unwrap()
    }
}

#[derive(Clone)]
pub struct PrefixBoundTupleOne<T1: KeyDeclaration, T2: KeyDeclaration> {
    prefix: Vec<u8>,
    phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2> PrefixBoundTupleOne<T1, T2>
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    for<'a> <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new<'a>(prefix: T1::KeyReference<'a>) -> PrefixBoundTupleOne<T1, T2> {
        PrefixBoundTupleOne {
            prefix: T1::to_key_bytes(prefix),
            phantom: PhantomData,
        }
    }
}

impl<T1, T2> PrefixBound for PrefixBoundTupleOne<T1, T2>
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    for<'a> <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2);

    fn prefix_bytes(&self) -> &Vec<u8> {
        &self.prefix
    }
}

#[derive(Clone)]
pub struct PrefixBoundTripleOne<T1: KeyDeclaration, T2: KeyDeclaration, T3: KeyDeclaration> {
    prefix: Vec<u8>,
    phantom: PhantomData<(T1, T2, T3)>,
}

impl<T1, T2, T3> PrefixBoundTripleOne<T1, T2, T3>
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    T3: KeyDeclaration,
    for<'a> <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new<'a>(prefix: T1::KeyReference<'a>) -> PrefixBoundTripleOne<T1, T2, T3> {
        PrefixBoundTripleOne {
            prefix: T1::to_key_bytes(prefix),
            phantom: PhantomData,
        }
    }
}

impl<T1, T2, T3> PrefixBound for PrefixBoundTripleOne<T1, T2, T3>
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    T3: KeyDeclaration,
    for<'a> <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2, T3);

    fn prefix_bytes(&self) -> &Vec<u8> {
        &self.prefix
    }
}

#[derive(Clone)]
pub struct PrefixBoundTripleTwo<T1: KeyDeclaration, T2: KeyDeclaration, T3: KeyDeclaration> {
    prefix: Vec<u8>,
    phantom: PhantomData<(T1, T2, T3)>,
}

impl<T1, T2, T3> PrefixBoundTripleTwo<T1, T2, T3>
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    T3: KeyDeclaration,
    for<'a> <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new<'a>(
        prefix: (T1::KeyReference<'a>, T2::KeyReference<'a>),
    ) -> PrefixBoundTripleTwo<T1, T2, T3> {
        PrefixBoundTripleTwo {
            prefix: <(T1, T2) as KeyDeclaration>::to_key_bytes(prefix),
            phantom: PhantomData,
        }
    }
}

impl<T1, T2, T3> PrefixBound for PrefixBoundTripleTwo<T1, T2, T3>
where
    T1: KeyDeclaration,
    T2: KeyDeclaration,
    T3: KeyDeclaration,
    for<'a> <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    for<'a> <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2, T3);

    fn prefix_bytes(&self) -> &Vec<u8> {
        &self.prefix
    }
}

impl<'a, T: PrefixBound> From<&'a RangeBounds<'a, T>> for RangeBounds<'a, Vec<u8>> {
    fn from(range: &'a RangeBounds<T>) -> Self {
        RangeBounds::new(
            range.start_bound().map(|x| x.prefix_bytes()),
            range.end_bound().map(|x| x.prefix_bytes()),
        )
    }
}
