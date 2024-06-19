pub trait ContractErrorTrait {
    fn error_type(&self) -> &'static str;
    fn wrap(&self) -> serde_json::Value;
}

pub fn check_contract_error_trait<T: ContractErrorTrait>(_: &T) {}

#[crate::contract_error(inside_nearsdk)]
pub struct BaseError {
    #[serde(flatten)]
    pub error: serde_json::Value,
}

impl From<BaseError> for String {
    fn from(value: BaseError) -> Self {
        value.error.to_string()
    }
}

pub fn wrap_error<T: ContractErrorTrait>(error: T) -> serde_json::Value {
    error.wrap()
}

use std::marker::PhantomData;

use crate::borsh::{schema::BorshSchemaContainer, BorshSchema};
use schemars::{schema::{RootSchema, Schema}, schema_for, JsonSchema};

#[cfg(feature = "abi")]
pub trait SerializationFormat {
    type SchemaObject;
}

#[cfg(feature = "abi")]
pub struct Json;
#[cfg(feature = "abi")]
impl SerializationFormat for Json {
    type SchemaObject = Schema;
}

#[cfg(feature = "abi")]
pub struct Borsh;
#[cfg(feature = "abi")]
impl SerializationFormat for Borsh {
    type SchemaObject = BorshSchemaContainer;
}

#[cfg(feature = "abi")]
trait SerializableWith<S: SerializationFormat> {
    fn schema() -> S::SchemaObject;
}

#[cfg(feature = "abi")]
impl<T: JsonSchema> SerializableWith<Json> for T {
    fn schema() -> Schema {
        schemars::gen::SchemaGenerator::default().subschema_for::<T>()
    }
}

#[cfg(feature = "abi")]
impl<T: BorshSchema> SerializableWith<Borsh> for T {
    fn schema() -> BorshSchemaContainer {
        crate::borsh::schema_container_of::<T>()
    }
}

#[cfg(feature = "abi")]
pub trait ContractReturnSchema<S: SerializationFormat, Error> {
    // The method return type as specified by the user of the framework.
    type Input;
    // The `Ok` type in the normalized `Result<Ok, _>`.
    type Okay;

    // This should be treated as an associated function. The only reason
    // the `self` receiver is present is for us to be able to abuse method
    // resolution to emulate specialization.
    //
    // The only reason the `_serialization_format` parameter is here is
    // so that we can disambiguate the `S` type parameter in scenarios
    // where we abuse deref coercion.
    fn schema(self, _serialization_format: S) -> S::SchemaObject;
}

pub trait ContractReturnNormalize<Error> {
    // The method return type as specified by the user of the framework.
    type Input;
    // The `Ok` type in the normalized `Result<Ok, _>`.
    type Okay;

    // The `self` receiver is only here as an anchor - we abuse method resolution
    // (deref coercion) to emulate specialization. The real receiver is the `ret`
    // parameter.
    //
    // The only reason the `_serialization_format` parameter is here is
    // so that we can disambiguate the `S` type parameter in scenarios
    // where we abuse deref coercion.
    fn normalize_return(
        self,
        ret: Self::Input,
    ) -> Result<Self::Okay, Error>;
}

#[cfg(feature = "abi")]
impl<S: SerializationFormat, T: SerializableWith<S>> ContractReturnSchema<S, BaseError> for PhantomData<T> {
    type Input = T;
    type Okay = T;

    fn schema(self, _serialization_format: S) -> S::SchemaObject {
        T::schema()
    }
}

impl<T> ContractReturnNormalize<BaseError> for PhantomData<T> {
    type Input = T;
    type Okay = T;

    fn normalize_return(
        self,
        ret: Self::Input,
    ) -> Result<Self::Okay, BaseError> {
        Ok(ret)
    }
}

#[cfg(feature = "abi")]
impl<S: SerializationFormat, T: SerializableWith<S>, Error> ContractReturnSchema<S, Error>
    for &PhantomData<Result<T, Error>>
{
    type Input = Result<T, Error>;
    type Okay = T;

    fn schema(self, _serialization_format: S) -> S::SchemaObject {
        T::schema()
    }
}

impl<T, Error> ContractReturnNormalize<Error>
    for &PhantomData<Result<T, Error>>
{
    type Input = Result<T, Error>;
    type Okay = T;

    fn normalize_return(
        self,
        ret: Self::Input,
    ) -> Result<Self::Okay, Error> {
        ret
    }
}