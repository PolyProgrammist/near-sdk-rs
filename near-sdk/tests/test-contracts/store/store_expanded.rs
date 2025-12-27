#![feature(prelude_import)]
#![allow(deprecated)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{near, store, PanicOnDefault};
use Collection::*;
#[borsh(crate = "near_sdk::borsh")]
pub struct Insertable {
    pub index: u32,
    pub data: String,
    pub is_valid: bool,
}
impl near_sdk::borsh::ser::BorshSerialize for Insertable {
    fn serialize<__W: near_sdk::borsh::io::Write>(
        &self,
        writer: &mut __W,
    ) -> ::core::result::Result<(), near_sdk::borsh::io::Error> {
        near_sdk::borsh::BorshSerialize::serialize(&self.index, writer)?;
        near_sdk::borsh::BorshSerialize::serialize(&self.data, writer)?;
        near_sdk::borsh::BorshSerialize::serialize(&self.is_valid, writer)?;
        Ok(())
    }
}
impl near_sdk::borsh::de::BorshDeserialize for Insertable {
    fn deserialize_reader<__R: near_sdk::borsh::io::Read>(
        reader: &mut __R,
    ) -> ::core::result::Result<Self, near_sdk::borsh::io::Error> {
        Ok(Self {
            index: near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
            data: near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
            is_valid: near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Insertable {
    #[inline]
    fn cmp(&self, other: &Insertable) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.index, &other.index) {
            ::core::cmp::Ordering::Equal => {
                match ::core::cmp::Ord::cmp(&self.data, &other.data) {
                    ::core::cmp::Ordering::Equal => {
                        ::core::cmp::Ord::cmp(&self.is_valid, &other.is_valid)
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Insertable {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Insertable,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.index, &other.index) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(&self.data, &other.data) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &self.is_valid,
                            &other.is_valid,
                        )
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Insertable {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<String>;
        let _: ::core::cmp::AssertParamIsEq<bool>;
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Insertable {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Insertable {
    #[inline]
    fn eq(&self, other: &Insertable) -> bool {
        self.index == other.index && self.data == other.data
            && self.is_valid == other.is_valid
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Insertable {
    #[inline]
    fn clone(&self) -> Insertable {
        Insertable {
            index: ::core::clone::Clone::clone(&self.index),
            data: ::core::clone::Clone::clone(&self.data),
            is_valid: ::core::clone::Clone::clone(&self.is_valid),
        }
    }
}
#[borsh(crate = ":: near_sdk :: borsh")]
pub struct StoreContract {
    pub iterable_set: store::IterableSet<Insertable>,
    pub iterable_map: store::IterableMap<u32, Insertable>,
    pub unordered_set: store::UnorderedSet<Insertable>,
    pub unordered_map: store::UnorderedMap<u32, Insertable>,
    pub tree_map: store::TreeMap<u32, Insertable>,
    pub lookup_map: store::LookupMap<u32, Insertable>,
    pub lookup_set: store::LookupSet<Insertable>,
    pub vec: store::Vector<Insertable>,
}
const _: () = {
    #[automatically_derived]
    impl ::std::default::Default for StoreContract {
        fn default() -> Self {
            ::near_sdk::env::panic_err(
                ::near_sdk::errors::ContractNotInitialized {
                }
                    .into(),
            );
        }
    }
};
impl ::near_sdk::borsh::ser::BorshSerialize for StoreContract {
    fn serialize<__W: ::near_sdk::borsh::io::Write>(
        &self,
        writer: &mut __W,
    ) -> ::core::result::Result<(), ::near_sdk::borsh::io::Error> {
        ::near_sdk::borsh::BorshSerialize::serialize(&self.iterable_set, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.iterable_map, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.unordered_set, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.unordered_map, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.tree_map, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.lookup_map, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.lookup_set, writer)?;
        ::near_sdk::borsh::BorshSerialize::serialize(&self.vec, writer)?;
        Ok(())
    }
}
impl ::near_sdk::borsh::de::BorshDeserialize for StoreContract {
    fn deserialize_reader<__R: ::near_sdk::borsh::io::Read>(
        reader: &mut __R,
    ) -> ::core::result::Result<Self, ::near_sdk::borsh::io::Error> {
        Ok(Self {
            iterable_set: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(
                reader,
            )?,
            iterable_map: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(
                reader,
            )?,
            unordered_set: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(
                reader,
            )?,
            unordered_map: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(
                reader,
            )?,
            tree_map: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
            lookup_map: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
            lookup_set: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
            vec: ::near_sdk::borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[must_use]
pub struct StoreContractExt {
    pub(crate) promise_or_create_on: ::near_sdk::PromiseOrValue<::near_sdk::AccountId>,
    pub(crate) deposit: ::near_sdk::NearToken,
    pub(crate) static_gas: ::near_sdk::Gas,
    pub(crate) gas_weight: ::near_sdk::GasWeight,
}
impl StoreContractExt {
    pub fn with_attached_deposit(mut self, amount: ::near_sdk::NearToken) -> Self {
        self.deposit = amount;
        self
    }
    pub fn with_static_gas(mut self, static_gas: ::near_sdk::Gas) -> Self {
        self.static_gas = static_gas;
        self
    }
    pub fn with_unused_gas_weight(mut self, gas_weight: u64) -> Self {
        self.gas_weight = ::near_sdk::GasWeight(gas_weight);
        self
    }
}
impl StoreContract {
    /// API for calling this contract's functions in a subsequent execution.
    pub fn ext(account_id: ::near_sdk::AccountId) -> StoreContractExt {
        StoreContractExt {
            promise_or_create_on: ::near_sdk::PromiseOrValue::Value(account_id),
            deposit: ::near_sdk::NearToken::from_near(0),
            static_gas: ::near_sdk::Gas::from_gas(0),
            gas_weight: ::near_sdk::GasWeight::default(),
        }
    }
    pub fn ext_on(promise: ::near_sdk::Promise) -> StoreContractExt {
        StoreContractExt {
            promise_or_create_on: ::near_sdk::PromiseOrValue::Promise(promise),
            deposit: ::near_sdk::NearToken::from_near(0),
            static_gas: ::near_sdk::Gas::from_gas(0),
            gas_weight: ::near_sdk::GasWeight::default(),
        }
    }
}
const CONTRACT_SOURCE_METADATA: &'static str = "{\"version\":\"0.1.0\",\"link\":null,\"standards\":[{\"standard\":\"nep330\",\"version\":\"1.3.0\"}],\"build_info\":null}";
impl StoreContractExt {
    pub fn contract_source_metadata(self) -> ::near_sdk::Promise {
        let __args = ::alloc::vec::Vec::new();
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("contract_source_metadata"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
}
impl StoreContract {
    pub fn contract_source_metadata() {
        near_sdk::env::value_return(CONTRACT_SOURCE_METADATA.as_bytes())
    }
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn contract_source_metadata() {
    ::near_sdk::env::setup_panic_hook();
    StoreContract::contract_source_metadata();
}
impl StoreContract {}
const _: () = {
    impl ::near_sdk::state::ContractState for StoreContract {}
};
#[serde(crate = ":: near_sdk :: serde")]
pub enum Collection {
    IterableSet,
    IterableMap,
    UnorderedSet,
    UnorderedMap,
    LookupMap,
    LookupSet,
    TreeMap,
    Vector,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    use ::near_sdk::serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Collection {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private228::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                Collection::IterableSet => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        0u32,
                        "IterableSet",
                    )
                }
                Collection::IterableMap => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        1u32,
                        "IterableMap",
                    )
                }
                Collection::UnorderedSet => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        2u32,
                        "UnorderedSet",
                    )
                }
                Collection::UnorderedMap => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        3u32,
                        "UnorderedMap",
                    )
                }
                Collection::LookupMap => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        4u32,
                        "LookupMap",
                    )
                }
                Collection::LookupSet => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        5u32,
                        "LookupSet",
                    )
                }
                Collection::TreeMap => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        6u32,
                        "TreeMap",
                    )
                }
                Collection::Vector => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Collection",
                        7u32,
                        "Vector",
                    )
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    use ::near_sdk::serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Collection {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        4u64 => _serde::__private228::Ok(__Field::__field4),
                        5u64 => _serde::__private228::Ok(__Field::__field5),
                        6u64 => _serde::__private228::Ok(__Field::__field6),
                        7u64 => _serde::__private228::Ok(__Field::__field7),
                        _ => {
                            _serde::__private228::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 8",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "IterableSet" => _serde::__private228::Ok(__Field::__field0),
                        "IterableMap" => _serde::__private228::Ok(__Field::__field1),
                        "UnorderedSet" => _serde::__private228::Ok(__Field::__field2),
                        "UnorderedMap" => _serde::__private228::Ok(__Field::__field3),
                        "LookupMap" => _serde::__private228::Ok(__Field::__field4),
                        "LookupSet" => _serde::__private228::Ok(__Field::__field5),
                        "TreeMap" => _serde::__private228::Ok(__Field::__field6),
                        "Vector" => _serde::__private228::Ok(__Field::__field7),
                        _ => {
                            _serde::__private228::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"IterableSet" => _serde::__private228::Ok(__Field::__field0),
                        b"IterableMap" => _serde::__private228::Ok(__Field::__field1),
                        b"UnorderedSet" => _serde::__private228::Ok(__Field::__field2),
                        b"UnorderedMap" => _serde::__private228::Ok(__Field::__field3),
                        b"LookupMap" => _serde::__private228::Ok(__Field::__field4),
                        b"LookupSet" => _serde::__private228::Ok(__Field::__field5),
                        b"TreeMap" => _serde::__private228::Ok(__Field::__field6),
                        b"Vector" => _serde::__private228::Ok(__Field::__field7),
                        _ => {
                            let __value = &_serde::__private228::from_utf8_lossy(
                                __value,
                            );
                            _serde::__private228::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<Collection>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Collection;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "enum Collection",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::IterableSet)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::IterableMap)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::UnorderedSet)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::UnorderedMap)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::LookupMap)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::LookupSet)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::TreeMap)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private228::Ok(Collection::Vector)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "IterableSet",
                "IterableMap",
                "UnorderedSet",
                "UnorderedMap",
                "LookupMap",
                "LookupSet",
                "TreeMap",
                "Vector",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Collection",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Collection>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl StoreContractExt {
    pub fn new(self) -> ::near_sdk::Promise {
        let __args = ::alloc::vec::Vec::new();
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("new"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
    pub fn insert(
        self,
        col: Collection,
        index_offset: usize,
        iterations: usize,
    ) -> ::near_sdk::Promise {
        let __args = {
            #[serde(crate = "::near_sdk::serde")]
            struct Input<'nearinput> {
                col: &'nearinput Collection,
                index_offset: &'nearinput usize,
                iterations: &'nearinput usize,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                use ::near_sdk::serde as _serde;
                #[automatically_derived]
                impl<'nearinput> _serde::Serialize for Input<'nearinput> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Input",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "col",
                            &self.col,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "index_offset",
                            &self.index_offset,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "iterations",
                            &self.iterations,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            let __args = Input {
                col: &col,
                index_offset: &index_offset,
                iterations: &iterations,
            };
            match near_sdk::serde_json::to_vec(&__args) {
                Ok(serialized) => serialized,
                Err(_) => {
                    ::near_sdk::env::panic_str(
                        "Failed to serialize the cross contract args using JSON.",
                    )
                }
            }
        };
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("insert"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
    pub fn remove(self, col: Collection, iterations: usize) -> ::near_sdk::Promise {
        let __args = {
            #[serde(crate = "::near_sdk::serde")]
            struct Input<'nearinput> {
                col: &'nearinput Collection,
                iterations: &'nearinput usize,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                use ::near_sdk::serde as _serde;
                #[automatically_derived]
                impl<'nearinput> _serde::Serialize for Input<'nearinput> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Input",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "col",
                            &self.col,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "iterations",
                            &self.iterations,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            let __args = Input {
                col: &col,
                iterations: &iterations,
            };
            match near_sdk::serde_json::to_vec(&__args) {
                Ok(serialized) => serialized,
                Err(_) => {
                    ::near_sdk::env::panic_str(
                        "Failed to serialize the cross contract args using JSON.",
                    )
                }
            }
        };
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("remove"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
    pub fn contains(
        self,
        col: Collection,
        repeat: usize,
        iterations: usize,
    ) -> ::near_sdk::Promise {
        let __args = {
            #[serde(crate = "::near_sdk::serde")]
            struct Input<'nearinput> {
                col: &'nearinput Collection,
                repeat: &'nearinput usize,
                iterations: &'nearinput usize,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                use ::near_sdk::serde as _serde;
                #[automatically_derived]
                impl<'nearinput> _serde::Serialize for Input<'nearinput> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Input",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "col",
                            &self.col,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "repeat",
                            &self.repeat,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "iterations",
                            &self.iterations,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            let __args = Input {
                col: &col,
                repeat: &repeat,
                iterations: &iterations,
            };
            match near_sdk::serde_json::to_vec(&__args) {
                Ok(serialized) => serialized,
                Err(_) => {
                    ::near_sdk::env::panic_str(
                        "Failed to serialize the cross contract args using JSON.",
                    )
                }
            }
        };
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("contains"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
    pub fn iter(
        self,
        col: Collection,
        repeat: usize,
        iterations: usize,
    ) -> ::near_sdk::Promise {
        let __args = {
            #[serde(crate = "::near_sdk::serde")]
            struct Input<'nearinput> {
                col: &'nearinput Collection,
                repeat: &'nearinput usize,
                iterations: &'nearinput usize,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                use ::near_sdk::serde as _serde;
                #[automatically_derived]
                impl<'nearinput> _serde::Serialize for Input<'nearinput> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Input",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "col",
                            &self.col,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "repeat",
                            &self.repeat,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "iterations",
                            &self.iterations,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            let __args = Input {
                col: &col,
                repeat: &repeat,
                iterations: &iterations,
            };
            match near_sdk::serde_json::to_vec(&__args) {
                Ok(serialized) => serialized,
                Err(_) => {
                    ::near_sdk::env::panic_str(
                        "Failed to serialize the cross contract args using JSON.",
                    )
                }
            }
        };
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("iter"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
    pub fn nth(
        self,
        col: Collection,
        repeat: usize,
        iterations: usize,
    ) -> ::near_sdk::Promise {
        let __args = {
            #[serde(crate = "::near_sdk::serde")]
            struct Input<'nearinput> {
                col: &'nearinput Collection,
                repeat: &'nearinput usize,
                iterations: &'nearinput usize,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                use ::near_sdk::serde as _serde;
                #[automatically_derived]
                impl<'nearinput> _serde::Serialize for Input<'nearinput> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Input",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "col",
                            &self.col,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "repeat",
                            &self.repeat,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "iterations",
                            &self.iterations,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            let __args = Input {
                col: &col,
                repeat: &repeat,
                iterations: &iterations,
            };
            match near_sdk::serde_json::to_vec(&__args) {
                Ok(serialized) => serialized,
                Err(_) => {
                    ::near_sdk::env::panic_str(
                        "Failed to serialize the cross contract args using JSON.",
                    )
                }
            }
        };
        match self.promise_or_create_on {
            ::near_sdk::PromiseOrValue::Promise(p) => p,
            ::near_sdk::PromiseOrValue::Value(account_id) => {
                ::near_sdk::Promise::new(account_id)
            }
        }
            .function_call_weight(
                ::std::string::String::from("nth"),
                __args,
                self.deposit,
                self.static_gas,
                self.gas_weight,
            )
    }
}
impl StoreContract {
    pub fn new() -> StoreContract {
        let vec = store::Vector::new(b"1");
        let iterable_set = store::IterableSet::new(b"2");
        let iterable_map = store::IterableMap::new(b"3");
        let unordered_set = store::UnorderedSet::new(b"4");
        let unordered_map = store::UnorderedMap::new(b"5");
        let tree_map = store::TreeMap::new(b"6");
        let lookup_map = store::LookupMap::new(b"7");
        let lookup_set = store::LookupSet::new(b"8");
        Self {
            vec,
            iterable_set,
            iterable_map,
            unordered_set,
            unordered_map,
            tree_map,
            lookup_map,
            lookup_set,
        }
    }
    fn insertable(&self) -> Insertable {
        Insertable {
            index: 0,
            data: "scatter cinnamon wheel useless please rough situate iron eager noise try evolve runway neglect onion"
                .to_string(),
            is_valid: true,
        }
    }
    pub fn insert(&mut self, col: Collection, index_offset: usize, iterations: usize) {
        let mut insertable = self.insertable();
        for iter in 0..=iterations {
            insertable.index = iter as u32;
            insertable.index += index_offset as u32;
            self.insert_op(&col, insertable.clone())
        }
    }
    pub fn remove(&mut self, col: Collection, iterations: usize) {
        let mut insertable = self.insertable();
        for iter in 0..=iterations {
            insertable.index = iter as u32;
            self.remove_op(&col, &insertable)
        }
    }
    pub fn contains(&mut self, col: Collection, repeat: usize, iterations: usize) {
        let mut insertable = self.insertable();
        for iter in 0..=iterations {
            insertable.index = iter as u32;
            for _ in 0..repeat {
                self.contains_op(&col, &insertable)
            }
        }
    }
    pub fn iter(&mut self, col: Collection, repeat: usize, iterations: usize) {
        for _ in 0..=iterations {
            for _ in 0..repeat {
                self.iter_op(&col, iterations)
            }
        }
    }
    pub fn nth(&mut self, col: Collection, repeat: usize, iterations: usize) {
        for iter in 0..=iterations {
            for _ in 0..repeat {
                self.nth_op(&col, iter)
            }
        }
    }
    fn insert_op(&mut self, col: &Collection, val: Insertable) {
        match col {
            IterableSet => {
                self.iterable_set.insert(val);
            }
            IterableMap => {
                self.iterable_map.insert(val.index, val);
            }
            UnorderedMap => {
                self.unordered_map.insert(val.index, val);
            }
            UnorderedSet => {
                self.unordered_set.insert(val);
            }
            LookupMap => {
                self.lookup_map.insert(val.index, val);
            }
            LookupSet => {
                self.lookup_set.insert(val);
            }
            TreeMap => {
                self.tree_map.insert(val.index, val);
            }
            Vector => {
                self.vec.push(val);
            }
        };
    }
    fn remove_op(&mut self, col: &Collection, val: &Insertable) {
        match col {
            IterableSet => {
                self.iterable_set.remove(&val);
            }
            IterableMap => {
                self.iterable_map.remove(&val.index);
            }
            UnorderedMap => {
                self.unordered_map.remove(&val.index);
            }
            UnorderedSet => {
                self.unordered_set.remove(&val);
            }
            LookupMap => {
                self.lookup_map.remove(&val.index);
            }
            LookupSet => {
                self.lookup_set.remove(&val);
            }
            TreeMap => {
                self.tree_map.remove(&val.index);
            }
            Vector => {
                if self.vec.is_empty() {
                    return;
                }
                self.vec.swap_remove(self.vec.len() - 1);
            }
        };
    }
    fn contains_op(&mut self, col: &Collection, val: &Insertable) {
        match col {
            IterableSet => self.iterable_set.contains(val),
            IterableMap => self.iterable_map.contains_key(&val.index),
            UnorderedMap => self.unordered_map.contains_key(&val.index),
            UnorderedSet => self.unordered_set.contains(val),
            LookupMap => self.lookup_map.contains_key(&val.index),
            LookupSet => self.lookup_set.contains(val),
            TreeMap => self.tree_map.contains_key(&val.index),
            Vector => ::core::panicking::panic("not implemented"),
        };
    }
    fn iter_op(&mut self, col: &Collection, take: usize) {
        match col {
            IterableSet => {
                let mut iter = self.iterable_set.iter();
                for _ in 0..take {
                    iter.next();
                }
            }
            IterableMap => {
                let mut iter = self.iterable_map.iter();
                for _ in 0..take {
                    iter.next();
                }
            }
            UnorderedMap => {
                let mut iter = self.unordered_map.iter();
                for _ in 0..take {
                    iter.next();
                }
            }
            UnorderedSet => {
                let mut iter = self.unordered_set.iter();
                for _ in 0..take {
                    iter.next();
                }
            }
            TreeMap => {
                let mut iter = self.tree_map.iter();
                for _ in 0..take {
                    iter.next();
                }
            }
            Vector => {
                let mut iter = self.vec.iter();
                for _ in 0..take {
                    iter.next();
                }
            }
            LookupMap => ::core::panicking::panic("not implemented"),
            LookupSet => ::core::panicking::panic("not implemented"),
        };
    }
    fn nth_op(&mut self, col: &Collection, element_idx: usize) {
        match col {
            IterableSet => {
                self.iterable_set.iter().nth(element_idx);
            }
            IterableMap => {
                self.iterable_map.iter().nth(element_idx);
            }
            UnorderedMap => {
                self.unordered_map.iter().nth(element_idx);
            }
            UnorderedSet => {
                self.unordered_set.iter().nth(element_idx);
            }
            TreeMap => {
                self.tree_map.iter().nth(element_idx);
            }
            Vector => {
                self.vec.iter().nth(element_idx);
            }
            LookupMap => ::core::panicking::panic("not implemented"),
            LookupSet => ::core::panicking::panic("not implemented"),
        };
    }
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn new() {
    ::near_sdk::env::setup_panic_hook();
    if ::near_sdk::env::attached_deposit().as_yoctonear() != 0 {
        ::near_sdk::env::panic_err(
            ::near_sdk::errors::DepositNotAccepted::new("new").into(),
        );
    }
    if <StoreContract as ::near_sdk::state::ContractState>::state_exists() {
        ::near_sdk::env::panic_err(
            ::near_sdk::errors::ContractAlreadyInitialized {
            }
                .into(),
        );
    }
    let contract = StoreContract::new();
    <StoreContract as ::near_sdk::state::ContractState>::state_write(&contract);
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn insert() {
    ::near_sdk::env::setup_panic_hook();
    #[serde(crate = "::near_sdk::serde")]
    struct Input {
        col: Collection,
        index_offset: usize,
        iterations: usize,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use ::near_sdk::serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Input {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "col" => _serde::__private228::Ok(__Field::__field0),
                            "index_offset" => _serde::__private228::Ok(__Field::__field1),
                            "iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"col" => _serde::__private228::Ok(__Field::__field0),
                            b"index_offset" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Input>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Input;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Input",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Collection,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            index_offset: __field1,
                            iterations: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Collection> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<usize> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<usize> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("col"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Collection>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "index_offset",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("col")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("index_offset")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("iterations")?
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            index_offset: __field1,
                            iterations: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "col",
                    "index_offset",
                    "iterations",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Input",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Input>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    let Input { col, index_offset, iterations }: Input = match ::near_sdk::env::input() {
        Some(input) => {
            match ::near_sdk::serde_json::from_slice(&input) {
                Ok(deserialized) => deserialized,
                Err(e) => {
                    ::near_sdk::env::panic_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to deserialize input from JSON. Error: `{0}`",
                                    e,
                                ),
                            );
                            res
                        }),
                    );
                }
            }
        }
        None => ::near_sdk::env::panic_str("Expected input since method has arguments."),
    };
    let mut contract = <StoreContract as ::near_sdk::state::ContractState>::state_read()
        .unwrap_or_default();
    StoreContract::insert(&mut contract, col, index_offset, iterations);
    <StoreContract as ::near_sdk::state::ContractState>::state_write(&contract);
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn remove() {
    ::near_sdk::env::setup_panic_hook();
    #[serde(crate = "::near_sdk::serde")]
    struct Input {
        col: Collection,
        iterations: usize,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use ::near_sdk::serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Input {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "col" => _serde::__private228::Ok(__Field::__field0),
                            "iterations" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"col" => _serde::__private228::Ok(__Field::__field0),
                            b"iterations" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Input>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Input;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Input",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Collection,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Input with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Input with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            iterations: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Collection> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<usize> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("col"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Collection>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterations",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("col")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("iterations")?
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            iterations: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["col", "iterations"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Input",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Input>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    let Input { col, iterations }: Input = match ::near_sdk::env::input() {
        Some(input) => {
            match ::near_sdk::serde_json::from_slice(&input) {
                Ok(deserialized) => deserialized,
                Err(e) => {
                    ::near_sdk::env::panic_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to deserialize input from JSON. Error: `{0}`",
                                    e,
                                ),
                            );
                            res
                        }),
                    );
                }
            }
        }
        None => ::near_sdk::env::panic_str("Expected input since method has arguments."),
    };
    let mut contract = <StoreContract as ::near_sdk::state::ContractState>::state_read()
        .unwrap_or_default();
    StoreContract::remove(&mut contract, col, iterations);
    <StoreContract as ::near_sdk::state::ContractState>::state_write(&contract);
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn contains() {
    ::near_sdk::env::setup_panic_hook();
    #[serde(crate = "::near_sdk::serde")]
    struct Input {
        col: Collection,
        repeat: usize,
        iterations: usize,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use ::near_sdk::serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Input {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "col" => _serde::__private228::Ok(__Field::__field0),
                            "repeat" => _serde::__private228::Ok(__Field::__field1),
                            "iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"col" => _serde::__private228::Ok(__Field::__field0),
                            b"repeat" => _serde::__private228::Ok(__Field::__field1),
                            b"iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Input>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Input;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Input",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Collection,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            repeat: __field1,
                            iterations: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Collection> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<usize> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<usize> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("col"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Collection>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("repeat"),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("col")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("repeat")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("iterations")?
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            repeat: __field1,
                            iterations: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["col", "repeat", "iterations"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Input",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Input>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    let Input { col, repeat, iterations }: Input = match ::near_sdk::env::input() {
        Some(input) => {
            match ::near_sdk::serde_json::from_slice(&input) {
                Ok(deserialized) => deserialized,
                Err(e) => {
                    ::near_sdk::env::panic_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to deserialize input from JSON. Error: `{0}`",
                                    e,
                                ),
                            );
                            res
                        }),
                    );
                }
            }
        }
        None => ::near_sdk::env::panic_str("Expected input since method has arguments."),
    };
    let mut contract = <StoreContract as ::near_sdk::state::ContractState>::state_read()
        .unwrap_or_default();
    StoreContract::contains(&mut contract, col, repeat, iterations);
    <StoreContract as ::near_sdk::state::ContractState>::state_write(&contract);
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn iter() {
    ::near_sdk::env::setup_panic_hook();
    #[serde(crate = "::near_sdk::serde")]
    struct Input {
        col: Collection,
        repeat: usize,
        iterations: usize,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use ::near_sdk::serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Input {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "col" => _serde::__private228::Ok(__Field::__field0),
                            "repeat" => _serde::__private228::Ok(__Field::__field1),
                            "iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"col" => _serde::__private228::Ok(__Field::__field0),
                            b"repeat" => _serde::__private228::Ok(__Field::__field1),
                            b"iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Input>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Input;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Input",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Collection,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            repeat: __field1,
                            iterations: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Collection> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<usize> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<usize> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("col"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Collection>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("repeat"),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("col")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("repeat")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("iterations")?
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            repeat: __field1,
                            iterations: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["col", "repeat", "iterations"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Input",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Input>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    let Input { col, repeat, iterations }: Input = match ::near_sdk::env::input() {
        Some(input) => {
            match ::near_sdk::serde_json::from_slice(&input) {
                Ok(deserialized) => deserialized,
                Err(e) => {
                    ::near_sdk::env::panic_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to deserialize input from JSON. Error: `{0}`",
                                    e,
                                ),
                            );
                            res
                        }),
                    );
                }
            }
        }
        None => ::near_sdk::env::panic_str("Expected input since method has arguments."),
    };
    let mut contract = <StoreContract as ::near_sdk::state::ContractState>::state_read()
        .unwrap_or_default();
    StoreContract::iter(&mut contract, col, repeat, iterations);
    <StoreContract as ::near_sdk::state::ContractState>::state_write(&contract);
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn nth() {
    ::near_sdk::env::setup_panic_hook();
    #[serde(crate = "::near_sdk::serde")]
    struct Input {
        col: Collection,
        repeat: usize,
        iterations: usize,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        use ::near_sdk::serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Input {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "col" => _serde::__private228::Ok(__Field::__field0),
                            "repeat" => _serde::__private228::Ok(__Field::__field1),
                            "iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"col" => _serde::__private228::Ok(__Field::__field0),
                            b"repeat" => _serde::__private228::Ok(__Field::__field1),
                            b"iterations" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Input>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Input;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Input",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Collection,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Input with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            repeat: __field1,
                            iterations: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Collection> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<usize> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<usize> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("col"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<Collection>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("repeat"),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "iterations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("col")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("repeat")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("iterations")?
                            }
                        };
                        _serde::__private228::Ok(Input {
                            col: __field0,
                            repeat: __field1,
                            iterations: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["col", "repeat", "iterations"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Input",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Input>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    let Input { col, repeat, iterations }: Input = match ::near_sdk::env::input() {
        Some(input) => {
            match ::near_sdk::serde_json::from_slice(&input) {
                Ok(deserialized) => deserialized,
                Err(e) => {
                    ::near_sdk::env::panic_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to deserialize input from JSON. Error: `{0}`",
                                    e,
                                ),
                            );
                            res
                        }),
                    );
                }
            }
        }
        None => ::near_sdk::env::panic_str("Expected input since method has arguments."),
    };
    let mut contract = <StoreContract as ::near_sdk::state::ContractState>::state_read()
        .unwrap_or_default();
    StoreContract::nth(&mut contract, col, repeat, iterations);
    <StoreContract as ::near_sdk::state::ContractState>::state_write(&contract);
}
impl StoreContract {}
