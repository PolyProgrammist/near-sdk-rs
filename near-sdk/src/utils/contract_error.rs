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

pub trait ContractReturn<S, Error> {
    type Input;
    type Okay;

    fn normalize_return(
        self,
        _serialization_format: S,
        ret: Self::Input,
    ) -> Result<Self::Okay, Error>;
}

impl<S, T> ContractReturn<S, BaseError> for PhantomData<T> {
    type Input = T;
    type Okay = T;

    fn normalize_return(
        self,
        _serialization_format: S,
        ret: Self::Input,
    ) -> Result<Self::Okay, BaseError> {
        Ok(ret)
    }
}

impl<S, T, Error> ContractReturn<S, Error>
    for &PhantomData<Result<T, Error>>
{
    type Input = Result<T, Error>;
    type Okay = T;

    fn normalize_return(
        self,
        _serialization_format: S,
        ret: Self::Input,
    ) -> Result<Self::Okay, Error> {
        ret
    }
}