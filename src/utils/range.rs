use std::marker::PhantomData;

use borsh::BorshDeserialize;

use crate::types::InterLiquidSdkError;

use super::KeyDeclaration;

pub trait KeyPrefix: Clone + Sized + Send {
    type KeyToExtract: KeyDeclaration;

    fn to_prefix_bytes(&self) -> Vec<u8>;

    fn extract<'a>(key: &mut [u8]) -> Result<Self::KeyToExtract, InterLiquidSdkError> {
        Ok(<Self::KeyToExtract as BorshDeserialize>::try_from_slice(
            &key,
        )?)
    }
}

#[derive(Clone)]
pub struct KeyPrefixTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    prefix: <T1 as KeyDeclaration>::KeyReference<'a>,
    phantom: PhantomData<(T1, T2)>,
}

impl<'a, T1, T2> KeyPrefixTupleOne<'a, T1, T2>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new(prefix: T1::KeyReference<'a>) -> KeyPrefixTupleOne<'a, T1, T2> {
        KeyPrefixTupleOne {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2> KeyPrefix for KeyPrefixTupleOne<'a, T1, T2>
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
pub struct KeyPrefixTripleOne<'a, T1, T2, T3>
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

impl<'a, T1, T2, T3> KeyPrefixTripleOne<'a, T1, T2, T3>
where
    T1: KeyDeclaration + 'a,
    T2: KeyDeclaration + 'a,
    T3: KeyDeclaration + 'a,
    <T1 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T2 as KeyDeclaration>::KeyReference<'a>: 'a,
    <T3 as KeyDeclaration>::KeyReference<'a>: 'a,
{
    pub fn new(prefix: T1::KeyReference<'a>) -> KeyPrefixTripleOne<'a, T1, T2, T3> {
        KeyPrefixTripleOne {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, T3> KeyPrefix for KeyPrefixTripleOne<'a, T1, T2, T3>
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
pub struct KeyPrefixTripleTwo<'a, T1, T2, T3>
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

impl<'a, T1, T2, T3> KeyPrefixTripleTwo<'a, T1, T2, T3>
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
    ) -> KeyPrefixTripleTwo<'a, T1, T2, T3> {
        KeyPrefixTripleTwo {
            prefix,
            phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, T3> KeyPrefix for KeyPrefixTripleTwo<'a, T1, T2, T3>
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
