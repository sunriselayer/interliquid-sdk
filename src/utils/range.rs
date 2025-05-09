use std::{marker::PhantomData, ops::RangeInclusive};

use borsh::BorshDeserialize;

use crate::state::ObjectSafeRangeBounds;

use super::KeyDeclaration;

pub trait PrefixBound: Clone + Sized {
    type KeyToExtract: KeyDeclaration;

    fn to_prefix_bytes(&self) -> Vec<u8>;

    fn extract<'a>(key: &mut [u8]) -> Self::KeyToExtract {
        <Self::KeyToExtract as BorshDeserialize>::deserialize(&mut &key[..]).unwrap()
    }

    fn exact(&self) -> RangeInclusive<Self> {
        self.clone()..=self.clone()
    }
}

#[derive(Clone)]
pub struct PrefixBoundTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: <T1 as KeyDeclaration>::KeyReference<'a>,
    phantom: PhantomData<(T1, T2)>,
}

impl<'a, T1, T2> PrefixBoundTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new(prefix: T1::KeyReference<'a>) -> PrefixBoundTupleOne<'a, T1, T2> {
        PrefixBoundTupleOne {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2> PrefixBound for PrefixBoundTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2);

    fn to_prefix_bytes(&self) -> Vec<u8> {
        <T1 as KeyDeclaration>::to_key_bytes(self.prefix)
    }
}

#[derive(Clone)]
pub struct PrefixBoundTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: <T1 as KeyDeclaration>::KeyReference<'a>,
    phantom: PhantomData<(T1, T2, T3)>,
}

impl<'a, T1, T2, T3> PrefixBoundTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new(prefix: T1::KeyReference<'a>) -> PrefixBoundTripleOne<'a, T1, T2, T3> {
        PrefixBoundTripleOne {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, T3> PrefixBound for PrefixBoundTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2, T3);

    fn to_prefix_bytes(&self) -> Vec<u8> {
        <T1 as KeyDeclaration>::to_key_bytes(self.prefix)
    }
}

#[derive(Clone)]
pub struct PrefixBoundTripleTwo<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: (T1::KeyReference<'a>, T2::KeyReference<'a>),
    phantom: PhantomData<(T1, T2, T3)>,
}

impl<'a, T1, T2, T3> PrefixBoundTripleTwo<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new(
        prefix: (T1::KeyReference<'a>, T2::KeyReference<'a>),
    ) -> PrefixBoundTripleTwo<'a, T1, T2, T3> {
        PrefixBoundTripleTwo {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, T3> PrefixBound for PrefixBoundTripleTwo<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    type KeyToExtract = (T1, T2, T3);

    fn to_prefix_bytes(&self) -> Vec<u8> {
        <(T1, T2) as KeyDeclaration>::to_key_bytes(self.prefix)
    }
}

pub trait IntoObjectSafeRangeBounds<T>: std::ops::RangeBounds<T> {
    fn into_object_safe_range_bounds(self) -> ObjectSafeRangeBounds<Vec<u8>>;
}

impl<B: std::ops::RangeBounds<T>, T: PrefixBound> IntoObjectSafeRangeBounds<T> for B {
    fn into_object_safe_range_bounds(self) -> ObjectSafeRangeBounds<Vec<u8>> {
        ObjectSafeRangeBounds::new(
            self.start_bound().map(|x| x.to_prefix_bytes()),
            self.end_bound().map(|x| x.to_prefix_bytes()),
        )
    }
}
