use std::{
    any::Any,
    collections::{BTreeMap, BTreeSet},
};

use anyhow::anyhow;

use crate::types::{InterLiquidSdkError, NamedSerializableType, SerializableAny};

pub struct TypeRegistry {
    unpack_any: BTreeMap<
        &'static str,
        Box<dyn Fn(&SerializableAny) -> Result<Box<dyn Any>, InterLiquidSdkError> + Send + Sync>,
    >,
    trait_impls: BTreeMap<&'static str, BTreeSet<&'static str>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            unpack_any: BTreeMap::new(),
            trait_impls: BTreeMap::new(),
        }
    }

    pub fn register<T: NamedSerializableType>(&mut self) {
        self.unpack_any.insert(
            T::type_name(),
            Box::new(|any| T::unpack_any(any).map(Box::new).map(|b| b as Box<dyn Any>)),
        );
    }

    pub fn register_trait(
        &mut self,
        sample: &dyn IdentifiableTrait,
    ) -> Result<(), InterLiquidSdkError> {
        if self.trait_impls.contains_key(sample.identifier()) {
            return Err(InterLiquidSdkError::AlreadyExists(anyhow!(
                "trait already registered"
            )));
        }

        self.trait_impls
            .entry(sample.identifier())
            .or_insert_with(BTreeSet::new);

        Ok(())
    }

    pub fn register_impl<T: NamedSerializableType>(
        &mut self,
        sample: &dyn IdentifiableTrait,
    ) -> Result<(), InterLiquidSdkError> {
        let identifier = sample.identifier();
        let impls = self
            .trait_impls
            .get_mut(identifier)
            .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                "trait not registered"
            )))?;

        impls.insert(T::type_name());

        Ok(())
    }

    pub fn unpack_trait(
        &self,
        sample: &dyn IdentifiableTrait,
        any: &SerializableAny,
    ) -> Result<Box<dyn Any>, InterLiquidSdkError> {
        let identifier = sample.identifier();
        let impls = self
            .trait_impls
            .get(identifier)
            .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                "trait not registered"
            )))?;

        if !impls.contains(any.type_.as_str()) {
            return Err(InterLiquidSdkError::NotFound(anyhow!(
                "type not registered"
            )));
        }

        let unpack_any =
            self.unpack_any
                .get(any.type_.as_str())
                .ok_or(InterLiquidSdkError::NotFound(anyhow!(
                    "type not registered"
                )))?;

        let instance = unpack_any(any)?;

        Ok(instance)
    }
}

pub trait IdentifiableTrait {
    fn identifier(&self) -> &'static str;
}
