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

#[cfg(feature = "abi")]
use borsh::{schema::BorshSchemaContainer, BorshSchema};
#[cfg(feature = "abi")]
use schemars::{schema::{RootSchema, Schema}, schema_for, JsonSchema};

#[cfg(feature = "abi")]
pub trait SerializationFormat {
    type SchemaObject;
}

#[cfg(feature = "abi")]
pub struct JsonSerializationFormat;
#[cfg(feature = "abi")]
impl SerializationFormat for JsonSerializationFormat {
    type SchemaObject = Schema;
}

#[cfg(feature = "abi")]
pub struct BorshSerializationFormat;
#[cfg(feature = "abi")]
impl SerializationFormat for BorshSerializationFormat {
    type SchemaObject = BorshSchemaContainer;
}

#[cfg(feature = "abi")]
trait SerializableWith<S: SerializationFormat> {
    fn schema() -> S::SchemaObject;
}

#[cfg(feature = "abi")]
impl<T: JsonSchema> SerializableWith<JsonSerializationFormat> for T {
    fn schema() -> Schema {
        schemars::gen::SchemaGenerator::default().subschema_for::<T>()
    }
}

#[cfg(feature = "abi")]
impl<T: BorshSchema> SerializableWith<BorshSerializationFormat> for T {
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

#[cfg(feature = "abi")]
impl<S: SerializationFormat, T: SerializableWith<S>> ContractReturnSchema<S, BaseError> for PhantomData<T> {
    type Input = T;
    type Okay = T;

    fn schema(self, _serialization_format: S) -> S::SchemaObject {
        T::schema()
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
